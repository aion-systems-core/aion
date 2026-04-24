//! CI Engine 2.0 — governance baseline and capsule checks.

use crate::ai::{drift_between_runs, replay_ai_capsule, AICapsuleV1};
use crate::governance::determinism::{validate_determinism, DeterminismProfile};
use crate::governance::integrity::{validate_integrity, IntegrityProfile};
use crate::governance::policy::{validate_capsule_against_policy, PolicyProfile};
use aion_core::DriftReport;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CiBaseline {
    pub capsule: AICapsuleV1,
    pub policy: PolicyProfile,
    pub determinism: DeterminismProfile,
    pub integrity: IntegrityProfile,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CiResult {
    pub drift: DriftReport,
    pub replay_success: bool,
    pub policy_ok: bool,
    pub determinism_ok: bool,
    pub integrity_ok: bool,
    pub success: bool,
}

pub fn ci_record_baseline(
    capsule: AICapsuleV1,
    policy: PolicyProfile,
    determinism: DeterminismProfile,
    integrity: IntegrityProfile,
) -> CiBaseline {
    CiBaseline {
        capsule,
        policy,
        determinism,
        integrity,
    }
}

pub fn ci_check_against_baseline(capsule: &AICapsuleV1, baseline: &CiBaseline) -> CiResult {
    let drift = drift_between_runs(&baseline.capsule, capsule);
    let replay = replay_ai_capsule(capsule);
    let replay_success = replay.success;

    let policy_v = validate_capsule_against_policy(capsule, &baseline.policy, Some(replay_success));
    let det_v = validate_determinism(capsule, &baseline.determinism);
    let int_v = validate_integrity(capsule, &baseline.integrity, Some(replay_success));

    let policy_ok = policy_v.ok;
    let determinism_ok = det_v.ok;
    let integrity_ok = int_v.ok;

    let success = !drift.changed && replay_success && policy_ok && determinism_ok && integrity_ok;

    CiResult {
        drift,
        replay_success,
        policy_ok,
        determinism_ok,
        integrity_ok,
        success,
    }
}
