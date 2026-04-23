use aion_engine::ai::{build_ai_capsule_v1, drift_between_runs, drift_between_runs_full};

#[test]
fn drift_contract_groups_and_labels_are_deterministic() {
    let a = build_ai_capsule_v1("m".into(), "hello".into(), 1);
    let mut b = a.clone();
    b.prompt = "hello world".into();
    b.tokens.push("extra".into());

    let d = drift_between_runs_full(&a, &b);
    assert!(d.changed);
    assert!(!d.categories.is_empty());
    assert!(!d.labels.is_empty());
    assert_eq!(d.labels, d.details);
    assert_eq!(d.categories, vec!["tokens", "model"]);
    assert!(d.labels.iter().all(|x| x.contains(':')));
}

#[test]
fn drift_contract_reports_tolerance_violation() {
    let a = build_ai_capsule_v1("m".into(), "hello".into(), 1);
    let mut b = a.clone();
    b.tokens.push("extra".into());

    let d = drift_between_runs(&a, &b);
    assert!(d.changed);
    assert!(!d.tolerance_violations.is_empty());
    assert!(d
        .tolerance_violations
        .iter()
        .any(|v| v.label == "tokens:delta_over_limit"));
    let e = d.error.expect("drift tolerance error");
    let v: serde_json::Value = serde_json::from_str(&e).expect("json error");
    assert_eq!(v["schema_version"], 1);
    assert_eq!(v["code"], "AION_DRIFT_TOLERANCE");
    assert_eq!(v["origin"], "drift");
}

#[test]
fn drift_contract_reports_overflow() {
    let mut a = build_ai_capsule_v1("m".into(), "hello".into(), 1);
    let mut b = a.clone();

    a.tokens = (0..120).map(|i| format!("a{i}")).collect();
    b.tokens = (0..120).map(|i| format!("b{i}")).collect();

    let d = drift_between_runs(&a, &b);
    assert!(d.overflow);
    assert!(d.labels.iter().any(|x| x == "other:labels_overflow"));
    let e = d.error.expect("drift overflow error");
    let v: serde_json::Value = serde_json::from_str(&e).expect("json error");
    assert_eq!(v["code"], "AION_DRIFT_OVERFLOW");
}
