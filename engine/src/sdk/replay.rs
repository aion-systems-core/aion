//! Replay and capsule-vs-capsule comparison.

use crate::ai::{compare_ai_capsules, replay_ai_capsule, AICapsuleV1, ReplayComparison, ReplayReport};

pub fn replay_capsule(capsule: &AICapsuleV1) -> ReplayReport {
    replay_ai_capsule(capsule)
}

pub fn compare_capsules(a: &AICapsuleV1, b: &AICapsuleV1) -> ReplayComparison {
    compare_ai_capsules(a, b)
}
