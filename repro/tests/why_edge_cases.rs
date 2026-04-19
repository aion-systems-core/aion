//! Phase 8.4 — pair-why edge cases (deterministic, no wall clock).

use repro::core::artifact::{ExecutionArtifact, ReproRun};
use repro::core::capture::{
    capture_command_real_with_clock, reset_counter_for_tests, FixedClock, CAPTURE_TEST_LOCK,
};
use repro::core::execution_boundary::EnvSnapshot15;
use repro::core::execution_trace::ExecutionTrace;
use repro::core::why_engine::explain_pair_why;
use std::collections::BTreeMap;

fn mk_artifact(run_id: &str, stdout: &str, env: BTreeMap<String, String>) -> ExecutionArtifact {
    let snap = EnvSnapshot15 {
        cwd: "/tmp".into(),
        path: String::new(),
        home: String::new(),
        ci: String::new(),
        shell: String::new(),
        lang: String::new(),
    };
    ExecutionArtifact {
        schema_version: 4,
        run_id: run_id.to_string(),
        repro_run: Some(ReproRun {
            id: run_id.to_string(),
            command: vec!["cmd".into()],
            env: env.clone(),
            stdout: stdout.into(),
        }),
        command: "cmd".into(),
        cwd: "/tmp".into(),
        timestamp: 0,
        env_snapshot: snap,
        stdout: stdout.into(),
        stderr: String::new(),
        exit_code: 0,
        duration_ms: 0,
        trace: ExecutionTrace {
            run_id: run_id.to_string(),
            events: vec![],
        },
    }
}

#[test]
fn identical_run_ids_yield_no_difference() {
    let _g = CAPTURE_TEST_LOCK.lock().unwrap();
    reset_counter_for_tests();
    let a = capture_command_real_with_clock("echo", &["x".into()], "echo x", &FixedClock(1));
    let b = a.clone();
    let s = explain_pair_why(&a, &b);
    assert!(s.contains("NO DIFFERENCE"), "{s}");
}

#[test]
fn no_env_change_stdout_only_triggers_no_root_cause() {
    let _g = CAPTURE_TEST_LOCK.lock().unwrap();
    reset_counter_for_tests();
    let a = capture_command_real_with_clock("echo", &["a".into()], "echo a", &FixedClock(2));
    reset_counter_for_tests();
    let b = capture_command_real_with_clock("echo", &["b".into()], "echo b", &FixedClock(2));
    let s = explain_pair_why(&a, &b);
    assert!(s.contains("NO ROOT CAUSE FOUND"), "{s}");
    assert!(s.contains("stdout changed"), "{s}");
}

#[test]
fn multiple_env_changes_list_multi_cause() {
    let env_a = BTreeMap::from([("A".into(), "1".into()), ("B".into(), "2".into())]);
    let env_b = BTreeMap::from([("A".into(), "9".into()), ("B".into(), "8".into())]);
    let a = mk_artifact("m1", "out\n", env_a);
    let b = mk_artifact("m2", "diff\n", env_b);
    let s = explain_pair_why(&a, &b);
    assert!(s.contains("MULTI-CAUSE"), "{s}");
    assert!(s.contains("A") && s.contains("B"), "{s}");
    assert!(s.contains("CAUSAL CHAIN:"), "{s}");
    assert!(s.contains("stdout"), "{s}");
}

#[test]
fn equivalent_payloads_distinct_run_ids_no_difference() {
    let env = BTreeMap::from([("X".into(), "1".into())]);
    let a = mk_artifact("rid1", "same\n", env.clone());
    let b = mk_artifact("rid2", "same\n", env);
    let s = explain_pair_why(&a, &b);
    assert!(s.contains("NO DIFFERENCE"), "{s}");
}
