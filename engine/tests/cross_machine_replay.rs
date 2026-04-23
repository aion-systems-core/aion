//! Cross-machine replay validation (runtime strict, machine tolerant).

use aion_engine::ai::{build_ai_capsule_v1, reassemble_capsule_with_backend};
use aion_engine::replay::validate_cross_machine_replay;

#[test]
fn cross_machine_replay_equivalent() {
    let c = build_ai_capsule_v1("m".into(), "probe".into(), 19);
    let r = reassemble_capsule_with_backend(&c.model, &c.prompt, c.seed, &c.backend_name);
    validate_cross_machine_replay(&c, &r).expect("same-machine replay equivalent");
}

#[test]
fn cross_machine_replay_detects_env_change() {
    let c = build_ai_capsule_v1("m".into(), "probe".into(), 19);
    let mut r = reassemble_capsule_with_backend(&c.model, &c.prompt, c.seed, &c.backend_name);
    if let Some(ref mut env) = r.execution_environment {
        env.runtime_fingerprint.aion_version = "tampered-for-test".into();
    }
    assert!(validate_cross_machine_replay(&c, &r).is_err());
}
