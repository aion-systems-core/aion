use aion_engine::governance::{
    builtin_policy_profile, CiBaseline, DeterminismProfile, IntegrityProfile,
};
use aion_engine::sdk::{
    build_capsule, ci_check, ci_record_baseline, compare_capsules, drift_between, explain_capsule,
    load_capsule, replay_capsule, save_capsule, validate_capsule, why_diff, write_output_bundle,
};
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

fn tmp() -> PathBuf {
    let p = std::env::temp_dir().join(format!("aion-sdk-test-{}", std::process::id()));
    let _ = fs::create_dir_all(&p);
    p
}

#[test]
fn test_sdk_capsule_roundtrip() {
    let dir = tmp();
    let path = dir.join("cap.aionai");
    let c0 = build_capsule("m", "hello", 9);
    save_capsule(&path, &c0).unwrap();
    let c1 = load_capsule(&path).unwrap();
    assert_eq!(c0.seed, c1.seed);
    assert_eq!(c0.tokens, c1.tokens);
}

#[test]
fn test_sdk_replay() {
    let cap = build_capsule("m", "z", 1);
    let rep = replay_capsule(&cap);
    assert!(rep.success, "{:?}", rep.comparison.differences);
}

#[test]
fn test_sdk_compare_capsules() {
    let cap = build_capsule("m", "z", 1);
    let cmp = compare_capsules(&cap, &cap);
    assert!(cmp.all_equal());
}

#[test]
fn test_sdk_drift() {
    let a = build_capsule("m", "x", 1);
    let b = build_capsule("m", "x", 2);
    let d = drift_between(&a, &b);
    assert!(d.changed);
}

#[test]
fn test_sdk_explain() {
    let cap = build_capsule("m", "a b", 3);
    let ex = explain_capsule(&cap);
    assert!(!ex.why.nodes.is_empty());
    assert!(!ex.graph.nodes.is_empty());
    let d = why_diff(&ex.why, &ex.why);
    assert!(!d.changed);
}

#[test]
fn test_sdk_governance() {
    let cap = build_capsule("m", "p", 4);
    let pol = builtin_policy_profile("dev");
    let det = DeterminismProfile::default();
    let integ = IntegrityProfile::default();
    let rep = validate_capsule(&cap, &pol, &det, &integ);
    assert!(rep.success, "{:?}", rep);
}

#[test]
fn test_sdk_ci() {
    let cap = build_capsule("m", "hello", 42);
    let bl = ci_record_baseline(
        &cap,
        &builtin_policy_profile("dev"),
        &DeterminismProfile::default(),
        &IntegrityProfile::default(),
    );
    let res = ci_check(&cap, &bl);
    assert!(res.success);

    let s = serde_json::to_string(&bl).unwrap();
    let parsed: CiBaseline = serde_json::from_str(&s).unwrap();
    assert_eq!(parsed.capsule.seed, cap.seed);
}

#[test]
fn test_sdk_output_bundle_ordering() {
    let dir = tmp().join("bundle");
    write_output_bundle(
        &dir,
        &[
            ("z.txt", b"z".to_vec()),
            ("a.txt", b"a".to_vec()),
            ("m.txt", b"m".to_vec()),
        ],
    )
    .unwrap();
    assert!(dir.join("a.txt").exists());
    assert!(dir.join("m.txt").exists());
    assert!(dir.join("z.txt").exists());
}

#[test]
fn test_sdk_versioning_check() {
    let dir = tmp();
    let path = dir.join("cap-version.aionai");
    let mut c = build_capsule("m", "x", 1);
    c.version = "2".into();
    fs::write(&path, serde_json::to_string_pretty(&c).unwrap()).unwrap();
    let err = aion_engine::sdk::capsule::load_capsule_checked(&path).unwrap_err();
    let s = err.to_string();
    let v: serde_json::Value = serde_json::from_str(&s).expect("sdk error json");
    assert_eq!(v["schema_version"], 1);
    assert_eq!(v["code"], "AION_SDK_VERSION");
}

#[test]
fn test_sdk_thread_safety() {
    let cap = Arc::new(build_capsule("m", "thread", 7));
    let mut handles = Vec::new();
    for _ in 0..8 {
        let c = Arc::clone(&cap);
        handles.push(std::thread::spawn(move || {
            let rep = replay_capsule(&c);
            assert!(rep.success);
            let d = drift_between(&c, &c);
            assert!(!d.changed);
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
}

#[test]
fn test_sdk_capsule_overwrite_protection() {
    let dir = tmp();
    let path = dir.join("cap-overwrite.aionai");
    let c = build_capsule("m", "hello", 1);
    save_capsule(&path, &c).unwrap();
    let err = save_capsule(&path, &c).unwrap_err();
    assert!(err.starts_with("AION_CAPSULE_SAVE_EXISTS|"));
}

#[test]
fn test_sdk_corrupt_capsule_error() {
    let dir = tmp();
    let path = dir.join("bad.aionai");
    fs::write(&path, "{not-json").unwrap();
    let err = load_capsule(&path).unwrap_err();
    assert!(err.starts_with("AION_CAPSULE_JSON|"));
}
