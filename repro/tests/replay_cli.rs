//! Phase 8.3 — `repro replay` reconstructs stdout from the event stream.

use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

static SEQ: AtomicU64 = AtomicU64::new(0);

fn repro_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_repro"))
}

fn scratch() -> std::path::PathBuf {
    let n = SEQ.fetch_add(1, Ordering::SeqCst);
    let base = std::env::var_os("CARGO_TARGET_TMPDIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target"));
    let p = base.join("repro_replay_cli_tests").join(format!("t_{n}"));
    std::fs::create_dir_all(&p).unwrap();
    p
}

fn parse_run_id(stdout: &str) -> String {
    for line in stdout.lines() {
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
    panic!("no run_id in:\n{stdout}");
}

#[test]
fn replay_prints_exact_stdout_from_event_stream() {
    let cwd = scratch();
    let run_out = Command::new(repro_bin())
        .current_dir(&cwd)
        .args(["run", "--", "echo", "hello"])
        .output()
        .expect("spawn repro");
    assert_eq!(
        run_out.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&run_out.stderr)
    );
    let id = parse_run_id(&String::from_utf8_lossy(&run_out.stdout));

    let r1 = Command::new(repro_bin())
        .current_dir(&cwd)
        .args(["replay", &id])
        .output()
        .expect("replay");
    let r2 = Command::new(repro_bin())
        .current_dir(&cwd)
        .args(["replay", &id])
        .output()
        .expect("replay 2");

    assert_eq!(r1.status.code(), Some(0));
    assert_eq!(r2.status.code(), Some(0));
    let s1 = String::from_utf8_lossy(&r1.stdout);
    let s2 = String::from_utf8_lossy(&r2.stdout);
    assert_eq!(s1, "hello\n");
    assert_eq!(s1, s2, "replay must be deterministic");
}
