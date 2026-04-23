//! Formal replay invariant checks against live capsules.

use aion_engine::ai::{build_ai_capsule_v1, reassemble_capsule_with_backend};
use aion_engine::replay::assert_formal_replay_invariant;

#[test]
fn formal_replay_invariant_holds() {
    let c = build_ai_capsule_v1("m".into(), "hello".into(), 7);
    let r = reassemble_capsule_with_backend(&c.model, &c.prompt, c.seed, &c.backend_name);
    assert_formal_replay_invariant(&c, &r).expect("formal replay invariant");
}

#[test]
fn formal_replay_invariant_breaks_on_tamper() {
    let c = build_ai_capsule_v1("m".into(), "hello".into(), 7);
    let mut r = reassemble_capsule_with_backend(&c.model, &c.prompt, c.seed, &c.backend_name);
    if !r.token_trace.is_empty() {
        r.token_trace[0].token = "tampered".into();
    }
    assert!(assert_formal_replay_invariant(&c, &r).is_err());
}
