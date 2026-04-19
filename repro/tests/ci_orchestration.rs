//! Phase 5: CI orchestration (failure surface + causal bundle).

use repro::ci::ci_orchestrator::process_ci_run;
use repro::ci::ci_runtime::detect_ci_context;
use repro::ci::meta::CiExecutionContext;
use repro::ci::storage::save_ci_run;
use repro::core::capture::{
    capture_command_real_with_clock, reset_counter_for_tests, FixedClock, CAPTURE_TEST_LOCK,
};

use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

static CI_PHASE5_LOCK: Mutex<()> = Mutex::new(());

fn tmpdir(name: &str) -> PathBuf {
    let base = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("repro-ci-phase5-tests");
    let _ = fs::create_dir_all(&base);
    base.join(format!(
        "{}-{}",
        name,
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ))
}

#[test]
fn success_ci_run_result() {
    let _g = CAPTURE_TEST_LOCK.lock().unwrap();
    let dir = tmpdir("ok");
    reset_counter_for_tests();
    let a = capture_command_real_with_clock("echo", &["z".into()], "echo z", &FixedClock(1));
    let r = process_ci_run(&dir, &a);
    assert!(r.success, "{r:?}");
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn failure_ci_run_surface_summary() {
    let _g = CAPTURE_TEST_LOCK.lock().unwrap();
    let dir = tmpdir("fail");
    reset_counter_for_tests();
    let ok = capture_command_real_with_clock("echo", &["x".into()], "echo x", &FixedClock(10));
    save_ci_run(&dir, &ok, &CiExecutionContext::local_default()).unwrap();
    reset_counter_for_tests();
    let bad = capture_command_real_with_clock("", &[], "", &FixedClock(11));
    let r = process_ci_run(&dir, &bad);
    assert!(!r.success, "{r:?}");
    assert_eq!(r.baseline_run_id.as_deref(), Some(ok.run_id.as_str()));
    assert!(matches!(
        r.category.as_deref(),
        Some("INPUT" | "RUNTIME" | "ENVIRONMENT" | "SYSTEM")
    ));
    let text = repro::ci::ci_orchestrator::format_ci_result_text(&r);
    assert!(text.contains("CI RUN FAILED"));
    assert!(text.contains("Compared:"));
    assert!(text.contains("Root Cause:"));
    assert!(text.contains("Node:"));
    assert!(text.contains("Summary:"));
    assert!(text.contains("Details:"));
    assert!(text.contains("repro ci why"));
    assert!(
        !text.to_lowercase().contains("timestamp"),
        "human CI output must not mention timestamp: {text}"
    );
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn detect_ci_context_sets_ci_flag() {
    let _g = CI_PHASE5_LOCK.lock().unwrap();
    std::env::set_var("CI", "1");
    let c = detect_ci_context();
    assert!(c.ci);
    std::env::remove_var("CI");
}
