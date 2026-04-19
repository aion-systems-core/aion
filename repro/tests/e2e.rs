// End-to-end flow test.
//
// Executes the compiled `repro` binary in an isolated temp cwd and walks
// through the full success-criteria sequence:
//
//   1. repro run -- echo hello
//   2. repro run -- echo world       (needed so `prev` resolves)
//   3. repro replay last
//   4. repro diff last prev
//   5. repro root-cause last
//
// We then re-run the same sequence in a second temp directory and assert
// that the per-step stdout is byte-identical between the two runs. That
// is the empirical definition of "deterministic" for this tool.

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn repro_bin() -> PathBuf {
    // Cargo sets CARGO_BIN_EXE_<name> for integration tests.
    PathBuf::from(env!("CARGO_BIN_EXE_repro"))
}

fn tmpdir(tag: &str) -> PathBuf {
    let mut d = env::temp_dir();
    d.push(format!(
        "repro-e2e-{}-{}-{}",
        tag,
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&d).unwrap();
    d
}

fn run(cwd: &PathBuf, args: &[&str]) -> (String, String, i32) {
    let out = Command::new(repro_bin())
        .args(args)
        .current_dir(cwd)
        .output()
        .expect("spawn repro");
    (
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
        out.status.code().unwrap_or(-1),
    )
}

/// Drop the `~ timestamp` field block from `repro diff` output. Two runs
/// captured a second apart differ on `timestamp` even when everything
/// else matches structurally; the e2e harness compares two isolated temp
/// dirs, so wall-clock seconds can diverge. Stripping this block keeps
/// the test aligned with "semantic determinism" rather than clock time.
fn redact_diff_skip_timestamp_field(s: &str) -> String {
    let lines: Vec<&str> = s.lines().collect();
    let mut out = String::with_capacity(s.len());
    let mut i = 0;
    while i < lines.len() {
        let trimmed = lines[i].trim_start();
        if trimmed == "~ timestamp" || trimmed == "~ duration_ms" {
            i += 1;
            while i < lines.len() {
                let line = lines[i];
                if line.is_empty() {
                    i += 1;
                    continue;
                }
                let t = line.trim_start();
                if t.starts_with('~') && t != trimmed {
                    break;
                }
                // Value blocks are indented; `causal chain:` and similar are not — stop here.
                if !matches!(line.chars().next(), Some(' ' | '\t')) {
                    break;
                }
                i += 1;
            }
            continue;
        }
        out.push_str(lines[i]);
        out.push('\n');
        i += 1;
    }
    out
}

/// Redact timestamps and run_ids so two executions separated in wall-clock
/// time can still be compared for structural / content determinism.
fn redact(s: &str) -> String {
    // `output::format_diff` uses U+2500 box-drawing chars, not ASCII `-`.
    let s = if s.contains("\u{2500}\u{2500} diff \u{2500}\u{2500}") || s.contains("── AION DIFF ──") {
        redact_diff_skip_timestamp_field(s)
    } else {
        s.to_string()
    };

    let mut out = String::with_capacity(s.len());
    for line in s.lines() {
        let trimmed = line.trim_start();
        // Duration noise: one run pair may include a `duration_ms` line in
        // `~ diverged fields:` and another may not, depending on wall clock.
        if trimmed.starts_with("- duration_ms") {
            continue;
        }
        if trimmed.starts_with("- timestamp") {
            continue;
        }
        if trimmed.contains("-> duration_ms") {
            continue;
        }
        if trimmed.starts_with("-> timestamp") {
            continue;
        }
        if trimmed.starts_with("timestamp") {
            out.push_str("timestamp        : <redacted>\n");
        } else if trimmed.starts_with("cwd") {
            out.push_str("cwd              : <redacted>\n");
        } else if trimmed.starts_with("environment_hash") {
            out.push_str("environment_hash : <redacted>\n");
        } else if trimmed.starts_with("duration_ms") {
            out.push_str("duration_ms      : <redacted>\n");
        } else if trimmed.starts_with("epoch_a AION run_id:") {
            out.push_str("epoch_a AION run_id: <redacted>\n");
        } else if trimmed.starts_with("epoch_b AION run_id:") {
            out.push_str("epoch_b AION run_id: <redacted>\n");
        } else if trimmed.starts_with("AION EPOCH (older):") {
            out.push_str("AION EPOCH (older): <redacted>\n");
        } else if trimmed.starts_with("AION EPOCH (newer):") {
            out.push_str("AION EPOCH (newer): <redacted>\n");
        } else if trimmed.starts_with("AION run_id") || trimmed.starts_with("run_id") {
            out.push_str("AION run_id      : <redacted>\n");
        } else if trimmed.starts_with("a: ") || trimmed.starts_with("b: ") {
            // Diff header: `a: <id>` / `b: <id>`.
            let tag = &trimmed[..1];
            out.push_str(tag);
            out.push_str(": <redacted>\n");
        } else if trimmed.starts_with("previous run: ") {
            out.push_str("previous run: <redacted>\n");
        } else {
            out.push_str(line);
            out.push('\n');
        }
    }
    out
}

fn full_flow(cwd: &PathBuf) -> Vec<String> {
    let mut results = Vec::new();

    let (stdout, stderr, code) = run(cwd, &["run", "--", "echo", "hello"]);
    assert_eq!(code, 0, "run#1 failed: {stderr}");
    results.push(redact(&stdout));

    let (stdout, stderr, code) = run(cwd, &["run", "--", "echo", "world"]);
    assert_eq!(code, 0, "run#2 failed: {stderr}");
    results.push(redact(&stdout));

    let (stdout, stderr, code) = run(cwd, &["replay", "last"]);
    assert_eq!(code, 0, "replay failed: {stderr}");
    results.push(redact(&stdout));

    let (stdout, stderr, code) = run(cwd, &["diff", "last", "prev"]);
    assert_eq!(code, 0, "diff failed: {stderr}");
    results.push(redact(&stdout));

    let (stdout, stderr, code) = run(cwd, &["root-cause", "last"]);
    assert_eq!(code, 0, "root-cause failed: {stderr}");
    results.push(redact(&stdout));

    results
}

#[test]
fn full_success_criteria_flow_is_deterministic() {
    let dir_a = tmpdir("a");
    let dir_b = tmpdir("b");

    let out_a = full_flow(&dir_a);
    let out_b = full_flow(&dir_b);

    assert_eq!(
        out_a.len(),
        out_b.len(),
        "step count must match across runs"
    );
    for (i, (a, b)) in out_a.iter().zip(out_b.iter()).enumerate() {
        assert_eq!(a, b, "step {i} output diverged between runs");
    }

    // Filesystem persistence: every captured run must exist as a JSON file
    // and the INDEX must list it.
    let runs_dir = dir_a.join("repro_runs");
    assert!(runs_dir.exists(), "repro_runs/ should exist");
    let index = fs::read_to_string(runs_dir.join("INDEX")).expect("INDEX must exist");
    let ids: Vec<&str> = index.lines().filter(|l| !l.is_empty()).collect();
    assert_eq!(ids.len(), 2, "expected 2 captured runs");
    for id in &ids {
        let p = runs_dir.join(format!("{id}.json"));
        assert!(p.exists(), "artifact for {id} should exist at {p:?}");
    }

    // Diff output must highlight the differing field (`stdout`, since the
    // only thing that differs between "echo hello" and "echo world" is the
    // command + its simulated stdout).
    let diff_output = &out_a[3];
    assert!(
        diff_output.contains("~ command") || diff_output.contains("~ stdout"),
        "diff output should mark at least one field as differing:\n{diff_output}"
    );

    // Root cause should surface `command` as the primary semantic field.
    let rc_output = &out_a[4];
    assert!(
        rc_output.contains("primary cause: command"),
        "root-cause should name `command` as primary:\n{rc_output}"
    );

    // Cleanup best-effort.
    let _ = fs::remove_dir_all(&dir_a);
    let _ = fs::remove_dir_all(&dir_b);
}
