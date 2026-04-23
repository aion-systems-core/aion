//! Governance + evidence integration tests (Phase 3).
//!
//! **Guided Link: Governance Tests** — see `docs/guided_tour.md`.

use aion_core::verify_linear;
use aion_engine::ai::{build_ai_capsule_v1, replay_ai_capsule};
use aion_engine::governance::{
    aion_evidence_generate_keypair, aion_evidence_sign, aion_evidence_verify_ed25519,
    sign_integrity, validate_capsule, validate_integrity, DeterminismProfile, IntegrityProfile,
    PolicyProfile,
};

#[test]
fn capsule_sign_verify_roundtrip() {
    let cap = build_ai_capsule_v1("demo".into(), "sign-verify".into(), 901);
    let int_a = sign_integrity(&cap);
    let int_b = sign_integrity(&cap);
    assert_eq!(int_a, int_b, "integrity envelope stable for same capsule");
    assert_eq!(int_a.capsule_hash.len(), 64);
    assert!(!int_a.evidence_root.is_empty());

    let (sk, pk) = aion_evidence_generate_keypair();
    let msg = serde_json::to_vec(&cap.evidence).expect("evidence json");
    let sig = aion_evidence_sign(&msg, &sk).expect("ed25519 sign");
    assert!(aion_evidence_verify_ed25519(&msg, &sig, &pk).unwrap());
    assert!(!aion_evidence_verify_ed25519(b"tampered", &sig, &pk).unwrap());
}

#[test]
fn capsule_replay_then_verify() {
    let cap = build_ai_capsule_v1("demo".into(), "replay-verify".into(), 902);
    verify_linear(&cap.evidence).expect("original evidence chain");

    let rep = replay_ai_capsule(&cap);
    assert!(
        rep.replay_symmetry_ok,
        "replay symmetry: {:?}",
        rep.replay_symmetry_error
    );
    assert!(rep.success, "diffs: {:?}", rep.comparison.differences);
    verify_linear(&rep.replay_capsule.evidence).expect("replay capsule evidence chain");

    let need_replay = IntegrityProfile {
        require_replay: true,
        ..Default::default()
    };
    assert!(
        !validate_integrity(&cap, &need_replay, None).ok,
        "replay-required integrity fails until replay is attested"
    );
    assert!(
        validate_integrity(&cap, &need_replay, Some(rep.success)).ok,
        "after replay, integrity attestation should pass"
    );
}

#[test]
fn policy_violation_fails_validation() {
    let cap = build_ai_capsule_v1("allowed-model".into(), "pilot".into(), 903);
    let bad = PolicyProfile {
        policy_version: "1".into(),
        name: "deny".into(),
        allowed_models: vec!["other-model".into()],
        max_prompt_length: 10_000,
        allowed_seeds: None,
        max_drift_tokens: 1_000_000,
        require_evidence: false,
        require_replay_success: false,
    };
    let det = DeterminismProfile::default();
    let integ = IntegrityProfile::default();
    let r = validate_capsule(&cap, &bad, &det, &integ);
    assert!(!r.success);
    assert!(!r.policy.ok, "{:?}", r.policy.messages);
}

#[test]
fn evidence_chain_continuity_test() {
    let cap = build_ai_capsule_v1("demo".into(), "chain".into(), 904);
    verify_linear(&cap.evidence).expect("valid chain");

    let mut broken = cap.evidence.clone();
    if let Some(r0) = broken.records.first_mut() {
        r0.leaf_digest = "deadbeef".into();
    }
    let err = verify_linear(&broken).expect_err("tampered leaf should break continuity");
    assert!(err.starts_with("AION_EVIDENCE_HASH|"));
    assert!(err.contains("evidence:hash_mismatch"));
}

#[test]
fn evidence_chain_missing_anchor_is_coded() {
    let empty = aion_core::EvidenceChain::default();
    let err = verify_linear(&empty).expect_err("missing anchor must fail");
    assert!(err.starts_with("AION_EVIDENCE_ANCHOR|"));
    assert!(err.contains("evidence:anchor_missing"));
}

#[test]
fn evidence_chain_replay_anchor_mismatch_is_coded() {
    let mut cap = build_ai_capsule_v1("demo".into(), "anchor".into(), 905);
    cap.evidence.formal_replay_invariant_ok = Some(false);
    let err = verify_linear(&cap.evidence).expect_err("replay anchor mismatch must fail");
    assert!(err.starts_with("AION_EVIDENCE_ANCHOR|"));
    assert!(err.contains("evidence:replay_anchor_mismatch"));
}
