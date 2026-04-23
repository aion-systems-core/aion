use aion_engine::ai::{
    build_ai_capsule_v1, compare_ai_capsules, drift_against_original, drift_between_runs,
    replay_ai_capsule, Event,
};
use aion_engine::replay::assert_replay_symmetry;

#[test]
fn test_replay_determinism() {
    let cap = build_ai_capsule_v1("m".into(), "hello".into(), 42);
    let rep = replay_ai_capsule(&cap);
    assert!(rep.success, "replay v2 success: {:?}", rep.comparison.differences);
    assert!(rep.comparison.all_product_flags());
    assert!(rep.comparison.graph_equal);
    assert!(!rep.drift_report.changed);
}

#[test]
fn test_replay_drift() {
    let left = build_ai_capsule_v1("m".into(), "hello".into(), 1);
    let right = build_ai_capsule_v1("m".into(), "hello".into(), 2);
    let d = drift_between_runs(&left, &right);
    assert!(d.changed, "different seeds must drift");
    assert!(d.fields.contains(&"tokens".to_string()));
    let cross = drift_against_original(&left, &right);
    assert!(cross.changed);
}

#[test]
fn test_replay_graph() {
    let cap = build_ai_capsule_v1("model-a".into(), "prompt p".into(), 99);
    let rep = replay_ai_capsule(&cap);
    assert!(rep.comparison.graph_equal);
}

#[test]
fn test_replay_metadata_and_warnings() {
    let mut cap = build_ai_capsule_v1("m".into(), "hello".into(), 1);
    cap.version = "2".into();
    cap.evidence.records.clear();
    let rep = replay_ai_capsule(&cap);
    assert!(rep.replay_timestamp > 0);
    assert!(!rep.replay_aion_version.is_empty());
    assert!(rep.replay_duration_ms < 60_000);
    assert!(
        rep.warnings
            .iter()
            .any(|w| w == "replay:invariant_failed")
    );
    assert!(
        rep.warnings
            .iter()
            .any(|w| w == "replay:invariant_failed")
    );
}

#[test]
fn test_replay_mismatch_diff_grouping_and_order() {
    let left = build_ai_capsule_v1("m".into(), "hello".into(), 1);
    let mut right = left.clone();
    right.seed = 2;
    right.prompt = "hello changed".into();
    right.tokens.push("extra".into());
    right.event_stream.push(Event::RunComplete { token_count: 999 });

    let cmp = compare_ai_capsules(&left, &right);
    assert!(!cmp.all_equal());
    assert_eq!(
        cmp.differences,
        cmp.mismatch_diff.flatten(),
        "flat differences must be deterministic compatibility view"
    );
    assert!(
        !cmp.mismatch_diff.input.is_empty()
            && !cmp.mismatch_diff.output.is_empty()
            && !cmp.mismatch_diff.events.is_empty()
    );

    let expected = cmp
        .mismatch_diff
        .input
        .iter()
        .map(|d| format!("input:{d}"))
        .chain(cmp.mismatch_diff.output.iter().map(|d| format!("output:{d}")))
        .chain(cmp.mismatch_diff.events.iter().map(|d| format!("events:{d}")))
        .chain(
            cmp.mismatch_diff
                .evidence
                .iter()
                .map(|d| format!("evidence:{d}")),
        )
        .collect::<Vec<_>>();
    assert_eq!(
        expected, cmp.differences,
        "flat mismatch list must follow fixed category order"
    );
}

#[test]
fn test_replay_error_contract_for_profile_and_symmetry() {
    let mut cap = build_ai_capsule_v1("m".into(), "hello".into(), 1);
    cap.determinism.time_epoch_secs += 1;
    let rep = replay_ai_capsule(&cap);
    if let Some(e) = rep.replay_profile_error.or(rep.replay_symmetry_error) {
        let v: serde_json::Value = serde_json::from_str(&e).expect("json error contract");
        assert_eq!(v["schema_version"], 1);
        assert!(v["code"].as_str().unwrap_or_default().starts_with("AION_REPLAY_"));
        assert_eq!(v["origin"], "replay");
    }
}

#[test]
fn test_replay_invariant_violation_tokenized() {
    let original = build_ai_capsule_v1("m".into(), "hello".into(), 7);
    let mut replayed = original.clone();
    replayed.version = "2".into();
    let err = assert_replay_symmetry(&original, &replayed).expect_err("must fail");
    assert_eq!(err, "replay:invariant_failed");
}

#[test]
fn test_replay_symmetry_failure_tokenized() {
    let original = build_ai_capsule_v1("m".into(), "hello".into(), 8);
    let mut replayed = original.clone();
    replayed.token_trace.clear();
    let err = assert_replay_symmetry(&original, &replayed).expect_err("must fail");
    assert_eq!(err, "replay:symmetry_failed");
}

#[test]
fn test_replay_why_slice_mismatch_tokenized() {
    let original = build_ai_capsule_v1("m".into(), "hello".into(), 9);
    let mut replayed = original.clone();
    replayed.why.summary = "why_delta".into();
    let err = assert_replay_symmetry(&original, &replayed).expect_err("must fail");
    assert_eq!(err, "replay:why_slice_mismatch");
}

#[test]
fn test_replay_event_stream_mismatch_tokenized() {
    let original = build_ai_capsule_v1("m".into(), "hello".into(), 10);
    let mut replayed = original.clone();
    replayed.event_stream.push(Event::RunComplete { token_count: 777 });
    let err = assert_replay_symmetry(&original, &replayed).expect_err("must fail");
    assert_eq!(err, "replay:event_stream_mismatch");
}

#[test]
fn test_replay_error_output_order_is_deterministic() {
    let mut cap = build_ai_capsule_v1("m".into(), "hello".into(), 11);
    cap.determinism.freeze_time = !cap.determinism.freeze_time;
    let rep = replay_ai_capsule(&cap);
    let err_json = rep
        .replay_profile_error
        .or(rep.replay_symmetry_error)
        .expect("replay error json");
    let code_idx = err_json.find("\"code\"").unwrap_or(usize::MAX);
    let message_idx = err_json.find("\"message\"").unwrap_or(usize::MAX);
    let context_idx = err_json.find("\"context\"").unwrap_or(usize::MAX);
    let origin_idx = err_json.find("\"origin\"").unwrap_or(usize::MAX);
    assert!(code_idx < message_idx);
    assert!(message_idx < context_idx);
    assert!(context_idx < origin_idx);
}
