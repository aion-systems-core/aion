//! Phase 5.5: deterministic CI baseline selection + regression graph divergence.

use repro::ci::baseline::select_baseline_run;
use repro::ci::meta::CiExecutionContext;
use repro::ci::storage::{list_ci_runs_in, save_ci_run};
use repro::core::capture::{
    build_execution_trace, capture_command_real_with_clock, reset_counter_for_tests, FixedClock,
    ProcessResult, CAPTURE_TEST_LOCK,
};
use repro::core::execution_boundary::env_resolved_trace_keys;
use repro::core::causal_graph::build_causal_graph;
use repro::core::causal_query::first_divergent_causal_node;

use std::fs;
use std::path::PathBuf;

fn tmpdir(tag: &str) -> PathBuf {
    let base = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("repro-ci-baseline-tests");
    let _ = fs::create_dir_all(&base);
    base.join(format!(
        "{}-{}",
        tag,
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ))
}

#[test]
fn baseline_selection_prefers_last_success_same_cmd_cwd() {
    let _g = CAPTURE_TEST_LOCK.lock().unwrap();
    reset_counter_for_tests();
    let success =
        capture_command_real_with_clock("echo", &["t".into()], "echo t", &FixedClock(101));
    let mut failing = success.clone();
    failing.run_id = "synthetic-fail".into();
    failing.exit_code = 1;
    failing.stdout.clear();
    failing.stderr = "boom\n".into();
    failing.trace = build_execution_trace(
        "echo",
        &["t".into()],
        &ProcessResult {
            stdout: String::new(),
            stderr: "boom\n".into(),
            exit_code: 1,
        },
        1,
        failing.run_id.clone(),
        env_resolved_trace_keys(),
    );

    let got = select_baseline_run(&failing, &[success.clone()]).expect("baseline");
    assert_eq!(got.run_id, success.run_id);
}

#[test]
fn command_mismatch_uses_index_fallback_last() {
    let _g = CAPTURE_TEST_LOCK.lock().unwrap();
    reset_counter_for_tests();
    let a = capture_command_real_with_clock("echo", &["a".into()], "echo a", &FixedClock(201));
    reset_counter_for_tests();
    let b = capture_command_real_with_clock("echo", &["b".into()], "echo b", &FixedClock(202));
    reset_counter_for_tests();
    let cur = capture_command_real_with_clock("echo", &["z".into()], "echo z", &FixedClock(203));

    let candidates = vec![a.clone(), b.clone()];
    let got = select_baseline_run(&cur, &candidates).expect("baseline");
    assert_eq!(got.run_id, b.run_id);
}

#[test]
fn regression_changed_stdout_yields_divergence() {
    let _g = CAPTURE_TEST_LOCK.lock().unwrap();
    reset_counter_for_tests();
    let template =
        capture_command_real_with_clock("echo", &["p".into()], "echo p", &FixedClock(301));

    let mut baseline = template.clone();
    baseline.run_id = "baseline-reg".into();
    baseline.exit_code = 0;
    baseline.stdout = "alpha\n".into();
    baseline.stderr.clear();
    baseline.trace = build_execution_trace(
        "echo",
        &["p".into()],
        &ProcessResult {
            stdout: "alpha\n".into(),
            stderr: String::new(),
            exit_code: 0,
        },
        1,
        baseline.run_id.clone(),
        env_resolved_trace_keys(),
    );

    let mut current = baseline.clone();
    current.run_id = "current-reg".into();
    current.exit_code = 1;
    current.stdout = "beta\n".into();
    current.trace = build_execution_trace(
        "echo",
        &["p".into()],
        &ProcessResult {
            stdout: "beta\n".into(),
            stderr: String::new(),
            exit_code: 1,
        },
        1,
        current.run_id.clone(),
        env_resolved_trace_keys(),
    );

    let gb = build_causal_graph(&baseline.trace);
    let gc = build_causal_graph(&current.trace);
    assert!(
        first_divergent_causal_node(&gb, &gc).is_some(),
        "expected structural or surface divergence"
    );
}

#[test]
fn baseline_selection_is_deterministic() {
    let _g = CAPTURE_TEST_LOCK.lock().unwrap();
    reset_counter_for_tests();
    let t = capture_command_real_with_clock("echo", &["q".into()], "echo q", &FixedClock(401));

    let mut a = t.clone();
    a.run_id = "r1".into();
    a.exit_code = 1;
    a.command = "echo q".into();
    a.cwd = t.cwd.clone();

    let mut b = t.clone();
    b.run_id = "r2".into();
    b.exit_code = 0;
    b.command = "echo q".into();
    b.cwd = t.cwd.clone();

    let mut c = t.clone();
    c.run_id = "r3".into();
    c.exit_code = 0;
    c.command = "echo q".into();
    c.cwd = t.cwd.clone();

    let mut cur = t.clone();
    cur.run_id = "current".into();
    cur.exit_code = 1;
    cur.command = "echo q".into();
    cur.cwd = t.cwd.clone();

    let candidates = vec![a, b, c];
    for _ in 0..25 {
        let got = select_baseline_run(&cur, &candidates).expect("baseline");
        assert_eq!(got.run_id, "r3");
    }
}

#[test]
fn list_ci_runs_in_matches_index_order() {
    let _g = CAPTURE_TEST_LOCK.lock().unwrap();
    let root = tmpdir("list");
    reset_counter_for_tests();
    let x = capture_command_real_with_clock("echo", &["m".into()], "echo m", &FixedClock(501));
    reset_counter_for_tests();
    let y = capture_command_real_with_clock("echo", &["n".into()], "echo n", &FixedClock(502));
    save_ci_run(&root, &x, &CiExecutionContext::local_default()).unwrap();
    save_ci_run(&root, &y, &CiExecutionContext::local_default()).unwrap();

    let loaded = list_ci_runs_in(&root);
    assert_eq!(loaded.len(), 2);
    assert_eq!(loaded[0].run_id, x.run_id);
    assert_eq!(loaded[1].run_id, y.run_id);
    let _ = fs::remove_dir_all(&root);
}
