//! Governance CI baseline and check.

use crate::ai::AICapsuleV1;
use crate::governance::{
    ci_check_against_baseline, ci_record_baseline as engine_record_baseline, CiBaseline, CiResult,
    DeterminismProfile, IntegrityProfile, PolicyProfile,
};

pub fn ci_record_baseline(
    capsule: &AICapsuleV1,
    policy: &PolicyProfile,
    determinism: &DeterminismProfile,
    integrity: &IntegrityProfile,
) -> CiBaseline {
    engine_record_baseline(capsule.clone(), policy.clone(), *determinism, *integrity)
}

pub fn ci_check(capsule: &AICapsuleV1, baseline: &CiBaseline) -> CiResult {
    ci_check_against_baseline(capsule, baseline)
}
