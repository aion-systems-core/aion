//! CI human + JSON output contract (deterministic, no timing noise).

use repro::ci::ci_orchestrator::{ci_run_json_value, format_ci_result_text, CiResult, CI_RULE};
use repro::core::report::CauseCategory;
use repro::core::root_cause::generate_summary;

#[test]
fn failure_human_output_matches_layout() {
    let r = CiResult {
        success: false,
        run_id: "run_test_01".into(),
        baseline_run_id: Some("base_00".into()),
        category: Some("RUNTIME".into()),
        node: Some("n2".into()),
        summary: Some(generate_summary(CauseCategory::RuntimeChange)),
        details: vec![
            "command differs".into(),
            "stdout differs".into(),
            "exit code 0 → 1".into(),
        ],
    };
    let text = format_ci_result_text(&r);
    let expected = format!(
        "{rule}CI RUN FAILED\n\
Run:        run_test_01\n\
Compared:   base_00\n\
\n\
Root Cause: RUNTIME\n\
Node:       n2\n\
\n\
Summary:\n\
execution failed → process returned non-zero exit code\n\
\n\
Details:\n\
- command differs\n\
- stdout differs\n\
- exit code 0 → 1\n\
\n\
Hint:\n\
repro ci why run_test_01\n\
{rule}",
        rule = CI_RULE
    );
    assert_eq!(text, expected);
}

#[test]
fn success_human_output_is_minimal() {
    let r = CiResult {
        success: true,
        run_id: "ok_run".into(),
        baseline_run_id: None,
        category: None,
        node: None,
        summary: None,
        details: vec![],
    };
    let text = format_ci_result_text(&r);
    let expected = format!(
        "{rule}CI RUN SUCCESS\n\
Run:        ok_run\n\
{rule}",
        rule = CI_RULE
    );
    assert_eq!(text, expected);
}

#[test]
fn failure_json_fixed_schema_and_key_order() {
    let r = CiResult {
        success: false,
        run_id: "j1".into(),
        baseline_run_id: None,
        category: Some("INPUT".into()),
        node: Some("n1".into()),
        summary: Some(generate_summary(CauseCategory::InputChange)),
        details: vec!["command differs".into()],
    };
    let v = ci_run_json_value(&r);
    let keys: Vec<String> = v.as_object().expect("object").keys().cloned().collect();
    assert_eq!(
        keys,
        vec![
            "status".to_string(),
            "run_id".to_string(),
            "baseline_run_id".to_string(),
            "root_cause".to_string(),
        ]
    );
    assert_eq!(v["status"], "failed");
    assert_eq!(v["run_id"], "j1");
    assert!(v["baseline_run_id"].is_null());
    let rc = &v["root_cause"];
    let rc_keys: Vec<_> = rc.as_object().unwrap().keys().cloned().collect();
    assert_eq!(
        rc_keys,
        vec!["category", "node", "summary", "details"]
            .into_iter()
            .map(String::from)
            .collect::<Vec<_>>()
    );
    assert_eq!(rc["category"], "INPUT");
    assert_eq!(rc["node"], "n1");
    let s = rc["summary"].as_str().unwrap();
    assert!(!s.contains('\n'));
    assert_eq!(
        s,
        "input changed → program behavior differs → output diverged"
    );
}

#[test]
fn summary_text_is_fixed_per_category() {
    assert_eq!(
        generate_summary(CauseCategory::EnvironmentChange),
        "environment changed → execution context differs → output diverged"
    );
    assert_eq!(
        generate_summary(CauseCategory::SystemNoise),
        "non-deterministic system effect detected"
    );
}
