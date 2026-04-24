//! Integration tests for the `sealrun` binary: `execute ai` JSON envelope and human-readable headings.
//!
//! Uses `CARGO_BIN_EXE_sealrun`; does not assert on full artefact trees—only stdout shape.

use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn unique_run_id(prefix: &str) -> String {
    let ns = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("{prefix}_{ns}")
}

#[test]
fn cli_execute_ai_emits_json_summary() {
    let id = unique_run_id("cli_flow_json");
    let out = Command::new(env!("CARGO_BIN_EXE_sealrun"))
        .args([
            "--id",
            id.as_str(),
            "execute",
            "ai",
            "--model",
            "demo",
            "--prompt",
            "cli_flow_test",
            "--seed",
            "9",
            "--json",
        ])
        .output()
        .expect("spawn sealrun");
    assert!(
        out.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    let line = stdout
        .lines()
        .find(|l| l.contains("\"kind\":\"execute_ai\"") || l.contains("\"kind\": \"execute_ai\""));
    assert!(line.is_some(), "expected JSON summary line, got:\n{stdout}");
}

#[test]
fn cli_execute_ai_emits_human_heading() {
    let id = unique_run_id("cli_flow_human");
    let out = Command::new(env!("CARGO_BIN_EXE_sealrun"))
        .args([
            "--id",
            id.as_str(),
            "execute",
            "ai",
            "--model",
            "demo",
            "--prompt",
            "cli_flow_human",
            "--seed",
            "2",
        ])
        .output()
        .expect("spawn sealrun");
    assert!(
        out.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("SealRun | seal your run"),
        "expected banner, got:\n{stdout}"
    );
}
