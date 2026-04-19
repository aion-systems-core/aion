//! Smoke: `aion repro` forwards to REPRO without duplicating integration tests.

use std::path::PathBuf;
use std::process::Command;

fn aion_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_aion"))
}

#[test]
fn aion_forwards_repro_subcommand_help() {
    // `aion repro --help` is the router's own help; REPRO's parser is reached with a subcommand.
    let out = Command::new(aion_bin())
        .args(["repro", "run", "--help"])
        .output()
        .expect("spawn aion");
    assert!(
        out.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("run") || stdout.contains("Capture") || stdout.contains("command"),
        "expected forwarded `repro run --help` output:\n{stdout}"
    );
}

#[test]
fn unknown_tool_is_deterministic_error() {
    let out = Command::new(aion_bin())
        .args(["not_a_tool"])
        .output()
        .expect("spawn aion");
    assert!(!out.status.success());
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("unknown tool") && stderr.contains("repro"),
        "expected ToolNotFound-style stderr, got:\n{stderr}"
    );
}
