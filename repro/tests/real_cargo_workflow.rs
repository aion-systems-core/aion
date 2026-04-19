//! End-to-end scenario: two `cargo build` runs with PATH order permuted;
//! `repro diff` / `repro root-cause` must surface environment drift. Then
//! `repro ci run` uses the same engine; snapshot PATH matches for the same
//! effective PATH string.

use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn repro_exe() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_repro"))
}

fn write_probe_crate(dir: &PathBuf) {
    fs::create_dir_all(dir.join("src")).unwrap();
    fs::write(
        dir.join("Cargo.toml"),
        "[package]\nname = \"repro_flow_probe\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[lib]\npath = \"lib.rs\"\n",
    )
    .unwrap();
    fs::write(
        dir.join("src/lib.rs"),
        "#[cfg(test)]\nmod t {\n    #[test]\n    fn ok() {\n        assert!(true);\n    }\n}\n",
    )
    .unwrap();
}

fn path_sep() -> char {
    if cfg!(windows) {
        ';'
    } else {
        ':'
    }
}

fn permuted_paths() -> Option<(String, String)> {
    let orig = std::env::var("PATH").ok()?;
    let sep = path_sep();
    let parts: Vec<&str> = orig.split(sep).filter(|p| !p.is_empty()).collect();
    if parts.len() < 2 {
        return None;
    }
    let sep_s = sep.to_string();
    let path_a = parts.join(&sep_s);
    let mut rev = parts;
    rev.reverse();
    let path_b = rev.join(&sep_s);
    if path_a == path_b {
        return None;
    }
    Some((path_a, path_b))
}

fn last_local_artifact_json(dir: &PathBuf) -> String {
    let idx = fs::read_to_string(dir.join("repro_runs").join("INDEX")).expect("INDEX");
    let last = idx
        .lines()
        .filter(|l| !l.is_empty())
        .last()
        .expect("run id");
    fs::read_to_string(dir.join("repro_runs").join(format!("{last}.json"))).unwrap()
}

fn last_ci_artifact_json(dir: &PathBuf) -> String {
    let idx_path = dir.join("repro_ci_store").join("INDEX.jsonl");
    let idx = fs::read_to_string(&idx_path).expect("CI INDEX");
    let last_line = idx
        .lines()
        .filter(|l| !l.is_empty())
        .last()
        .expect("index line");
    let v: Value = serde_json::from_str(last_line).unwrap();
    let run_id = v["run_id"].as_str().expect("run_id");
    fs::read_to_string(
        dir.join("repro_ci_store")
            .join(run_id)
            .join("artifact.json"),
    )
    .unwrap()
}

#[test]
fn cargo_build_path_swap_environment_root_cause_and_ci_snapshot() {
    let Some((path_a, path_b)) = permuted_paths() else {
        eprintln!("skip real_cargo_workflow: need PATH with at least two segments");
        return;
    };

    let tmp = std::env::temp_dir().join(format!(
        "repro-cargo-flow-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&tmp).unwrap();
    write_probe_crate(&tmp);

    let run = |path: &str, args: &[&str]| {
        Command::new(repro_exe())
            .current_dir(&tmp)
            .env("PATH", path)
            .args(args)
            .output()
            .unwrap_or_else(|e| panic!("spawn {args:?}: {e}"))
    };

    let b1 = run(&path_a, &["run", "--", "cargo", "build", "-q"]);
    assert!(
        b1.status.success(),
        "cargo build a: stderr={}",
        String::from_utf8_lossy(&b1.stderr)
    );
    let b2 = run(&path_b, &["run", "--", "cargo", "build", "-q"]);
    assert!(
        b2.status.success(),
        "cargo build b: stderr={}",
        String::from_utf8_lossy(&b2.stderr)
    );

    let diff = run(&path_b, &["diff", "last", "prev"]);
    let diff_out = String::from_utf8_lossy(&diff.stdout);
    assert!(
        diff.status.success(),
        "diff failed: stderr={}",
        String::from_utf8_lossy(&diff.stderr)
    );
    assert!(
        diff_out.contains("environment_hash"),
        "expected environment_hash in diff:\n{diff_out}"
    );

    let rc = run(&path_b, &["root-cause", "last"]);
    let rc_out = String::from_utf8_lossy(&rc.stdout);
    assert!(
        rc.status.success(),
        "root-cause failed: stderr={}",
        String::from_utf8_lossy(&rc.stderr)
    );
    assert!(
        rc_out.contains("environment_change") || rc_out.contains("environment_hash"),
        "expected environment-oriented root cause:\n{rc_out}"
    );

    let ci = run(&path_b, &["ci", "run", "--", "cargo", "build", "-q"]);
    assert!(
        ci.status.success(),
        "ci run: stderr={}",
        String::from_utf8_lossy(&ci.stderr)
    );

    let local_v: Value = serde_json::from_str(&last_local_artifact_json(&tmp)).unwrap();
    let ci_v: Value = serde_json::from_str(&last_ci_artifact_json(&tmp)).unwrap();
    assert_eq!(
        local_v["env_snapshot"]["path"], ci_v["env_snapshot"]["path"],
        "CI and local captures should agree on PATH in the 15% snapshot when cwd+PATH match"
    );

    let _ = fs::remove_dir_all(&tmp);
}
