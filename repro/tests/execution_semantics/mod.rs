//! Phase 8.1 — deterministic execution trace semantics (integration tests).

mod event_determinism;
mod event_invariants;
mod event_order;
mod replay_contract;

use repro::core::artifact::ExecutionArtifact;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

static SCRATCH_SEQ: AtomicU64 = AtomicU64::new(0);

fn repro_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_repro"))
}

/// Isolated cwd under the build tree (no wall clock / randomness).
fn scratch_dir(label: &str) -> PathBuf {
    let n = SCRATCH_SEQ.fetch_add(1, Ordering::SeqCst);
    let base = std::env::var_os("CARGO_TARGET_TMPDIR").map(PathBuf::from).unwrap_or_else(|| {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target")
    });
    let p = base.join("repro_exec_semantics").join(format!("{label}_{n}"));
    std::fs::create_dir_all(&p).unwrap();
    p
}

fn run_repro(cwd: &Path, args: &[&str]) -> std::process::Output {
    Command::new(repro_bin())
        .current_dir(cwd)
        .args(args)
        .output()
        .expect("spawn repro")
}

fn run_repro_env(cwd: &Path, extra_env: &[(&str, &str)], args: &[&str]) -> std::process::Output {
    let mut cmd = Command::new(repro_bin());
    cmd.current_dir(cwd);
    for (k, v) in extra_env {
        cmd.env(k, v);
    }
    cmd.args(args);
    cmd.output().expect("spawn repro")
}

fn assert_repro_ok(output: &std::process::Output, ctx: &str) {
    let code = output.status.code().unwrap_or(-1);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(code, 0, "{ctx}: stderr:\n{stderr}");
}

fn load_latest_artifact(cwd: &Path) -> ExecutionArtifact {
    let index_path = cwd.join("repro_runs").join("INDEX");
    let index = std::fs::read_to_string(&index_path).expect("read INDEX");
    let id = index
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .last()
        .expect("INDEX non-empty");
    let json_path = cwd.join("repro_runs").join(format!("{id}.json"));
    let json = std::fs::read_to_string(&json_path).expect("read artifact");
    serde_json::from_str(&json).expect("deserialize ExecutionArtifact")
}
