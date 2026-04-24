use aion_engine::ai::build_ai_capsule_v1;
use aion_engine::governance::{
    append_governance_audit, builtin_policy_profile, ci_check_against_baseline, ci_record_baseline,
    compose_policies, load_policy, validate_capsule, validate_capsule_against_policy,
    validate_capsules_parallel, validate_determinism, validate_integrity, DeterminismProfile,
    GovernanceAuditRecord, IntegrityProfile, PolicyProfile,
};
use std::fs;
use std::io::Write;
use std::path::PathBuf;

fn tmp_dir() -> PathBuf {
    let p = std::env::temp_dir().join(format!("aion-gov-test-{}", std::process::id()));
    let _ = fs::create_dir_all(&p);
    p
}

#[test]
fn test_policy_validation() {
    let cap = build_ai_capsule_v1("m1".into(), "hello".into(), 7);
    let pol_ok = PolicyProfile {
        policy_version: "1".into(),
        name: "t".into(),
        allowed_models: vec!["m1".into()],
        max_prompt_length: 100,
        allowed_seeds: Some((0, 99)),
        max_drift_tokens: 10_000,
        require_evidence: false,
        require_replay_success: false,
    };
    let v = validate_capsule_against_policy(&cap, &pol_ok, None);
    assert!(v.ok, "{:?}", v.messages);

    let pol_bad = PolicyProfile {
        allowed_models: vec!["other".into()],
        ..pol_ok.clone()
    };
    let v2 = validate_capsule_against_policy(&cap, &pol_bad, None);
    assert!(!v2.ok);
}

#[test]
fn test_policy_composition() {
    let parent = PolicyProfile {
        policy_version: "1".into(),
        name: "parent".into(),
        allowed_models: vec!["m1".into()],
        max_prompt_length: 100,
        allowed_seeds: Some((0, 9)),
        max_drift_tokens: 10,
        require_evidence: false,
        require_replay_success: false,
    };
    let child = PolicyProfile {
        policy_version: "1".into(),
        name: "child".into(),
        allowed_models: vec!["m2".into()],
        max_prompt_length: 0,
        allowed_seeds: None,
        max_drift_tokens: 0,
        require_evidence: true,
        require_replay_success: false,
    };
    let c = compose_policies(&parent, &child);
    assert_eq!(c.name, "child");
    assert_eq!(c.allowed_models, vec!["m2"]);
    assert_eq!(c.max_prompt_length, 100);
    assert!(c.require_evidence);
}

#[test]
fn test_policy_load_roundtrip() {
    let dir = tmp_dir();
    let path = dir.join("policy.json");
    let pol = builtin_policy_profile("dev");
    fs::write(&path, serde_json::to_string_pretty(&pol).unwrap()).unwrap();
    let loaded = load_policy(&path).unwrap();
    assert_eq!(loaded.name, pol.name);
}

#[test]
fn test_determinism_validation() {
    let cap = build_ai_capsule_v1("m".into(), "x".into(), 1);
    let loose = DeterminismProfile::default();
    assert!(validate_determinism(&cap, &loose).ok);

    let strict_time = DeterminismProfile {
        freeze_time: true,
        ..DeterminismProfile::default()
    };
    assert!(validate_determinism(&cap, &strict_time).ok);

    let strict_rng = DeterminismProfile {
        freeze_random: true,
        ..DeterminismProfile::default()
    };
    let d = validate_determinism(&cap, &strict_rng);
    assert!(d.ok, "capsule v2 sets freeze_random=true by default");
}

#[test]
fn test_integrity_validation() {
    let cap = build_ai_capsule_v1("m".into(), "a b".into(), 2);
    let loose = IntegrityProfile::default();
    assert!(validate_integrity(&cap, &loose, None).ok);

    let need_why = IntegrityProfile {
        require_why: true,
        ..Default::default()
    };
    assert!(validate_integrity(&cap, &need_why, None).ok);

    let need_rep = IntegrityProfile {
        require_replay: true,
        ..Default::default()
    };
    assert!(!validate_integrity(&cap, &need_rep, None).ok);
    assert!(validate_integrity(&cap, &need_rep, Some(true)).ok);
}

#[test]
fn test_ci_baseline() {
    let cap = build_ai_capsule_v1("model".into(), "p".into(), 11);
    let pol = builtin_policy_profile("dev");
    let det = DeterminismProfile::default();
    let integ = IntegrityProfile::default();
    let baseline = ci_record_baseline(cap.clone(), pol.clone(), det, integ);
    assert_eq!(baseline.capsule.model, "model");
    let dir = tmp_dir();
    let path = dir.join("baseline.json");
    let mut f = fs::File::create(&path).unwrap();
    f.write_all(serde_json::to_string_pretty(&baseline).unwrap().as_bytes())
        .unwrap();
    let s = fs::read_to_string(&path).unwrap();
    let parsed: aion_engine::governance::CiBaseline = serde_json::from_str(&s).unwrap();
    assert_eq!(parsed.capsule.seed, baseline.capsule.seed);
}

#[test]
fn test_ci_check() {
    let cap = build_ai_capsule_v1("m".into(), "hello".into(), 42);
    let baseline = ci_record_baseline(
        cap.clone(),
        builtin_policy_profile("dev"),
        DeterminismProfile::default(),
        IntegrityProfile::default(),
    );
    let res = ci_check_against_baseline(&cap, &baseline);
    assert!(res.replay_success, "{:?}", res);
    assert!(res.success, "{:?}", res);

    let rep = validate_capsule(
        &cap,
        &baseline.policy,
        &baseline.determinism,
        &baseline.integrity,
    );
    assert!(rep.success);
}

#[test]
fn test_parallel_validation() {
    let a = build_ai_capsule_v1("m".into(), "a".into(), 1);
    let b = build_ai_capsule_v1("m".into(), "b".into(), 2);
    let pol = builtin_policy_profile("dev");
    let det = DeterminismProfile::default();
    let integ = IntegrityProfile::default();
    let out = validate_capsules_parallel(&[a, b], &pol, &det, &integ);
    assert_eq!(out.len(), 2);
    assert!(out.iter().all(|r| r.success));
}

#[test]
fn test_governance_audit_log() {
    let dir = tmp_dir();
    let path = dir.join("gov.log");
    let rec = GovernanceAuditRecord {
        ts_epoch_secs: 1,
        action: "test".into(),
        subject: "subject".into(),
        ok: true,
        message: "ok".into(),
    };
    let p = append_governance_audit(Some(&path), &rec).unwrap();
    assert_eq!(p, path);
    let s = fs::read_to_string(&p).unwrap();
    assert!(s.contains("\"action\":\"test\""));
}

#[test]
fn test_policy_load_missing_required_field() {
    let dir = tmp_dir();
    let path = dir.join("policy-missing-required.json");
    fs::write(
        &path,
        r#"{
  "policy_version":"1",
  "allowed_models":[],
  "max_prompt_length":0,
  "allowed_seeds":null,
  "max_drift_tokens":0,
  "require_evidence":false,
  "require_replay_success":false
}"#,
    )
    .unwrap();
    let err = load_policy(&path).unwrap_err();
    assert!(err.starts_with("AION_GOVERNANCE_JSON|"));
    assert!(err.contains("field_missing:name"));
}

#[test]
fn test_policy_load_wrong_type() {
    let dir = tmp_dir();
    let path = dir.join("policy-wrong-type.json");
    fs::write(
        &path,
        r#"{
  "policy_version":"1",
  "name":"dev",
  "allowed_models":"*",
  "max_prompt_length":0,
  "allowed_seeds":null,
  "max_drift_tokens":0,
  "require_evidence":false,
  "require_replay_success":false
}"#,
    )
    .unwrap();
    let err = load_policy(&path).unwrap_err();
    assert!(err.starts_with("AION_GOVERNANCE_JSON|"));
    assert!(err.contains("field_type:allowed_models:array_string"));
}

#[test]
fn test_policy_load_invalid_value() {
    let dir = tmp_dir();
    let path = dir.join("policy-invalid-value.json");
    fs::write(
        &path,
        r#"{
  "policy_version":"2",
  "name":"dev",
  "allowed_models":[],
  "max_prompt_length":0,
  "allowed_seeds":null,
  "max_drift_tokens":0,
  "require_evidence":false,
  "require_replay_success":false
}"#,
    )
    .unwrap();
    let err = load_policy(&path).unwrap_err();
    assert!(err.starts_with("AION_GOVERNANCE_JSON|"));
    assert!(err.contains("policy_version_unsupported"));
}

#[test]
fn test_policy_load_cross_field_error() {
    let dir = tmp_dir();
    let path = dir.join("policy-cross-field.json");
    fs::write(
        &path,
        r#"{
  "policy_version":"1",
  "name":"dev",
  "allowed_models":[],
  "max_prompt_length":0,
  "allowed_seeds":null,
  "max_drift_tokens":0,
  "require_evidence":false,
  "require_replay_success":true
}"#,
    )
    .unwrap();
    let err = load_policy(&path).unwrap_err();
    assert!(err.starts_with("AION_GOVERNANCE_JSON|"));
    assert!(err.contains("replay_requires_evidence"));
}

#[test]
fn test_policy_load_json_parse_error() {
    let dir = tmp_dir();
    let path = dir.join("policy-parse-error.json");
    fs::write(&path, "{not-json").unwrap();
    let err = load_policy(&path).unwrap_err();
    assert!(err.starts_with("AION_GOVERNANCE_JSON|"));
    assert!(err.contains("json_parse_invalid"));
}
