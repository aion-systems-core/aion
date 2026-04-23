//! Governance validation (policy / determinism / integrity v1).

use crate::ai::AICapsuleV1;
use crate::governance::{
    validate_capsule as engine_validate,
    DeterminismProfile, GovernanceReport, IntegrityProfile, PolicyProfile,
};

pub fn validate_capsule(
    capsule: &AICapsuleV1,
    policy: &PolicyProfile,
    determinism: &DeterminismProfile,
    integrity: &IntegrityProfile,
) -> GovernanceReport {
    engine_validate(capsule, policy, determinism, integrity)
}
