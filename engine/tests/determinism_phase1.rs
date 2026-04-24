//! Phase 1 determinism tests — no wall clock, no filesystem, no OS RNG.

use aion_core::{DeterminismProfile, ExecutionEnvelope};
use aion_engine::ai::{
    build_ai_capsule_v1, drift_between_runs, merge_det_from_envelope, sort_events_deterministic,
    Event,
};
use aion_engine::capsule::deterministic_capsule_hash;
use aion_engine::replay::assert_replay_symmetry;

#[test]
fn determinism_freeze_test() {
    let env = ExecutionEnvelope::synthetic_deterministic(0xC0FFEE);
    let d = merge_det_from_envelope(0xC0FFEE, &env);
    assert_eq!(d.random_seed, 0xC0FFEE);
    assert_eq!(d.time_epoch_secs, env.frozen_time_ms / 1000);

    let strict = DeterminismProfile {
        strict_replay: true,
        ..Default::default()
    };
    assert!(strict.validate_replay_profile(&strict).is_ok());
    let other = DeterminismProfile {
        freeze_cwd: false,
        ..strict
    };
    assert!(strict.validate_replay_profile(&other).is_err());
}

#[test]
fn replay_symmetry_test() {
    let c = build_ai_capsule_v1("m".into(), "alpha beta".into(), 11);
    assert!(assert_replay_symmetry(&c, &c).is_ok());
    let mut bad = c.clone();
    assert!(!bad.token_trace.is_empty());
    bad.token_trace[0].token = "tampered".into();
    assert!(assert_replay_symmetry(&c, &bad).is_err());
}

#[test]
fn capsule_hash_stability_test() {
    let c = build_ai_capsule_v1("m".into(), "gamma".into(), 3);
    let a = deterministic_capsule_hash(&c);
    let b = deterministic_capsule_hash(&c);
    assert_eq!(a, b);
}

#[test]
fn event_ordering_test() {
    let v = vec![
        Event::RunComplete { token_count: 2 },
        Event::RunStart { model: "m".into() },
        Event::TokenGenerated {
            index: 0,
            token: "a".into(),
        },
        Event::PromptIngested { chars: 1 },
        Event::TokenGenerated {
            index: 1,
            token: "b".into(),
        },
    ];
    let sorted = sort_events_deterministic(&v);
    assert_eq!(sorted.first(), Some(&Event::RunStart { model: "m".into() }));
    assert!(matches!(sorted.last(), Some(Event::RunComplete { .. })));
}

#[test]
fn drift_noise_test() {
    let base = build_ai_capsule_v1("m".into(), "z".into(), 5);
    let mut noisy = base.clone();
    noisy.model = "totally-different-model".into();
    noisy.aion_version = "noise-version".into();
    let d = drift_between_runs(&base, &noisy);
    assert!(
        !d.changed,
        "semantic fields identical; model/version are noise: {:?}",
        d.fields
    );
}
