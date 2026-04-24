//! Optional heavy checks for examples (`SEALRUN_PRODUCT_TESTS=1` or legacy `AION_PRODUCT_TESTS=1`).

use std::path::PathBuf;
use std::process::Command;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[test]
fn test_examples_run_ci_gated() {
    let enabled = std::env::var("SEALRUN_PRODUCT_TESTS")
        .map(|v| v == "1")
        .unwrap_or(false)
        || std::env::var("AION_PRODUCT_TESTS")
            .map(|v| v == "1")
            .unwrap_or(false);
    if !enabled {
        return;
    }
    let root = repo_root();
    let st = Command::new("cargo")
        .current_dir(&root)
        .args(["check", "-p", "aion-cli", "--examples"])
        .status()
        .expect("cargo");
    assert!(st.success(), "cargo check --examples");

    for ex in [
        "sdk_capsule_build",
        "sdk_replay",
        "sdk_drift",
        "sdk_governance",
    ] {
        let st = Command::new("cargo")
            .current_dir(&root)
            .args(["run", "-q", "-p", "aion-cli", "--example", ex])
            .status()
            .expect("cargo run example");
        assert!(st.success(), "example {ex}");
    }
}
