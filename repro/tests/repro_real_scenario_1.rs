//! Phase 6.5.1 — first real-world repro scenario (test-first).
//!
//! Scenario: a script prints `ENV_VAR`; two runs differ only because the
//! variable changed (`foo` → `bar`). A developer must see *which* variable
//! moved and *from → to* in under ~10s — not only an opaque hash.
//!
//! `repro diff` contracts are enforced in [`real_scenario_env_var_change_is_diagnosed_end_to_end`].
//! Pair-mode `repro why` uses the deterministic why engine (Phase 8.4).

use repro::core::capture::CAPTURE_TEST_LOCK;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const VAR: &str = "ENV_VAR";

fn repro_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_repro"))
}

fn workspace_tmp() -> PathBuf {
    let mut d = env::temp_dir();
    d.push(format!(
        "repro-real-scenario-1-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&d).unwrap();
    d
}

fn write_print_env_rs(dir: &Path) {
    let src = r#"fn main() {
    match std::env::var("ENV_VAR") {
        Ok(v) => println!("{v}"),
        Err(_) => println!(),
    }
}
"#;
    fs::write(dir.join("print_env.rs"), src).expect("write print_env.rs");
}

fn compile_print_env(dir: &Path) -> PathBuf {
    let out_name = if cfg!(windows) {
        "print_env.exe"
    } else {
        "print_env"
    };
    let out = dir.join(out_name);
    let status = Command::new("rustc")
        .args(["print_env.rs", "-o"])
        .arg(out_name)
        .current_dir(dir)
        .status()
        .expect("spawn rustc for print_env.rs");
    assert!(
        status.success(),
        "rustc failed — integration test needs a working rustc on PATH"
    );
    out
}

fn run_repro_in_dir_with_extra_env(
    cwd: &Path,
    extra: &[(&str, &str)],
    args: &[&str],
) -> (String, String, i32) {
    let mut cmd = Command::new(repro_bin());
    cmd.current_dir(cwd).args(args);
    for (k, v) in extra {
        cmd.env(k, v);
    }
    let out = cmd.output().expect("spawn repro");
    (
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
        out.status.code().unwrap_or(-1),
    )
}

fn parse_run_id(artifact_stdout: &str) -> String {
    for line in artifact_stdout.lines() {
        let t = line.trim_start();
        if let Some(rest) = t
            .strip_prefix("AION run_id")
            .or_else(|| t.strip_prefix("run_id"))
        {
            let rest = rest.trim_start();
            if let Some(idx) = rest.find(':') {
                return rest[idx + 1..].trim().to_string();
            }
        }
    }
    panic!("could not parse run_id from repro run stdout:\n{artifact_stdout}");
}

/// Diff must surface an **environment** change in a human-readable, structured
/// way — not only `environment_hash` blobs.
fn assert_diff_names_env_var_with_values(diff: &str) {
    assert!(
        diff.contains("AION"),
        "diff output must be AION-branded.\n─── diff stdout ───\n{diff}"
    );
    assert!(
        diff.contains("── AION DIFF ──"),
        "diff must use the AION diff header.\n─── diff stdout ───\n{diff}"
    );
    assert!(
        diff.contains(VAR),
        "diff must name the changed variable `{VAR}` (opaque hash-only output is insufficient).\n\
         ─── diff stdout ───\n{diff}"
    );
    assert!(
        diff.contains("foo") && diff.contains("bar"),
        "diff must include both captured values `foo` and `bar`.\n\
         ─── diff stdout ───\n{diff}"
    );
    let lower = diff.to_ascii_lowercase();
    assert!(
        lower.contains("previous")
            || lower.contains("was")
            || lower.contains("epoch_a:"),
        "diff must label the epoch_a side (AION epoch labels or legacy previous/was).\n\
         ─── diff stdout ───\n{diff}"
    );
    assert!(
        lower.contains("new:") || lower.contains("now:") || lower.contains("epoch_b:"),
        "diff must label the epoch_b side (AION epoch labels or legacy new/now).\n\
         ─── diff stdout ───\n{diff}"
    );
    assert!(
        diff.contains("── AION CAUSAL LAYER ──") && lower.contains("age shift detected"),
        "diff must emit the AION causal layer and age-shift headline.\n─── diff stdout ───\n{diff}"
    );
    assert!(
        lower.contains("causal shift"),
        "diff must emit the CAUSAL SHIFT interpretation.\n─── diff stdout ───\n{diff}"
    );
}

/// `repro why` must answer “what actually changed?” for this pair — env-first (Phase 8.4).
fn assert_why_structured_env_root_cause(why: &str) {
    assert!(
        why.contains("── AION WHY ──"),
        "why must use the AION WHY header.\n─── why stdout ───\n{why}"
    );
    assert!(
        why.contains(VAR),
        "why output must name `{VAR}`.\n\
         ─── why stdout ───\n{why}"
    );
    assert!(
        why.contains("foo") && why.contains("bar"),
        "why output must show previous value `foo` and new value `bar`.\n\
         ─── why stdout ───\n{why}"
    );
    let lower = why.to_ascii_lowercase();
    assert!(
        lower.contains("root cause"),
        "why output must name a root cause section.\n─── why stdout ───\n{why}"
    );
    assert!(
        lower.contains("changed"),
        "why output must state explicitly that `{VAR}` changed.\n\
         ─── why stdout ───\n{why}"
    );
    assert!(
        why.contains("before:") && why.contains("after:"),
        "why must label values with before/after.\n─── why stdout ───\n{why}"
    );
    assert!(
        lower.contains("causal chain"),
        "why must emit a causal chain.\n─── why stdout ───\n{why}"
    );
    assert!(
        lower.contains("stdout changed"),
        "why must report stdout effect.\n─── why stdout ───\n{why}"
    );
}

fn setup_two_run_scenario() -> (PathBuf, String, String) {
    let tmp = workspace_tmp();
    write_print_env_rs(&tmp);
    let print_env_exe = compile_print_env(&tmp);
    let exe_for_argv = print_env_exe.to_str().expect("utf-8 temp path");

    let (out1, err1, c1) = run_repro_in_dir_with_extra_env(
        &tmp,
        &[(VAR, "foo")],
        &["run", "--", exe_for_argv],
    );
    assert_eq!(c1, 0, "repro run #1 failed: {err1}\n{out1}");
    assert!(
        out1.contains("foo"),
        "first run stdout should echo captured artifact including script output `foo`:\n{out1}"
    );

    let (out2, err2, c2) = run_repro_in_dir_with_extra_env(
        &tmp,
        &[(VAR, "bar")],
        &["run", "--", exe_for_argv],
    );
    assert_eq!(c2, 0, "repro run #2 failed: {err2}\n{out2}");
    assert!(
        out2.contains("bar"),
        "second run stdout should include script output `bar`:\n{out2}"
    );

    let run1 = parse_run_id(&out1);
    let run2 = parse_run_id(&out2);
    (tmp, run1, run2)
}

#[test]
fn real_scenario_env_var_change_is_diagnosed_end_to_end() {
    let _lock = CAPTURE_TEST_LOCK.lock().unwrap();

    let (tmp, run1, run2) = setup_two_run_scenario();

    let (diff_out, diff_err, diff_code) =
        run_repro_in_dir_with_extra_env(&tmp, &[], &["diff", &run1, &run2]);
    assert_eq!(
        diff_code, 0,
        "repro diff must succeed: {diff_err}\n{diff_out}"
    );
    assert_diff_names_env_var_with_values(&diff_out);

    let (why_a, why_err_a, why_code_a) =
        run_repro_in_dir_with_extra_env(&tmp, &[], &["why", &run1, &run2]);
    assert_eq!(
        why_code_a, 0,
        "repro why <run_a> <run_b> must succeed: {why_err_a}\n{why_a}"
    );
    assert_why_structured_env_root_cause(&why_a);

    let (why_b, why_err_b, why_code_b) =
        run_repro_in_dir_with_extra_env(&tmp, &[], &["why", &run1, &run2]);
    assert_eq!(
        why_code_b, 0,
        "second why invocation failed: {why_err_b}\n{why_b}"
    );
    assert_eq!(
        why_a, why_b,
        "why output must be deterministic for identical inputs (no wall-clock noise on stdout).\n\
         first:\n{why_a}\nsecond:\n{why_b}"
    );

    let _ = fs::remove_dir_all(&tmp);
}
