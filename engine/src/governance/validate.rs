//! Orchestrated governance validation for AI capsules.

use crate::ai::AICapsuleV1;
use crate::governance::ci::CiResult;
use crate::governance::determinism::{
    validate_determinism, DeterminismProfile, DeterminismViolation,
};
use crate::governance::integrity::{validate_integrity, IntegrityProfile, IntegrityViolation};
use crate::governance::policy::{validate_capsule_against_policy, PolicyProfile, PolicyViolation};
use aion_core::error::{code, line};
use serde::{Deserialize, Serialize};
use sha2::Digest;
use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GovernanceCiAttach {
    pub baseline_name: String,
    pub drift_changed: bool,
    pub drift_fields: Vec<String>,
    pub replay_success: bool,
    pub ci_policy_ok: bool,
    pub ci_determinism_ok: bool,
    pub ci_integrity_ok: bool,
    pub ci_success: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GovernanceReport {
    pub policy: PolicyViolation,
    pub determinism: DeterminismViolation,
    pub integrity: IntegrityViolation,
    pub success: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ci: Option<GovernanceCiAttach>,
}

fn cache() -> &'static Mutex<HashMap<String, GovernanceReport>> {
    static C: OnceLock<Mutex<HashMap<String, GovernanceReport>>> = OnceLock::new();
    C.get_or_init(|| Mutex::new(HashMap::new()))
}

fn cache_key(
    capsule: &AICapsuleV1,
    policy: &PolicyProfile,
    determinism: &DeterminismProfile,
    integrity: &IntegrityProfile,
) -> String {
    let mut h = sha2::Sha256::new();
    h.update(serde_json::to_vec(capsule).unwrap_or_default());
    h.update(serde_json::to_vec(policy).unwrap_or_default());
    h.update(serde_json::to_vec(determinism).unwrap_or_default());
    h.update(serde_json::to_vec(integrity).unwrap_or_default());
    format!("{:x}", h.finalize())
}

/// Full governance validation (policy / determinism / integrity). Replay-dependent rules use `replay_success: None` (strict policies may fail until CI supplies an outcome).
pub fn validate_capsule(
    capsule: &AICapsuleV1,
    policy: &PolicyProfile,
    determinism: &DeterminismProfile,
    integrity: &IntegrityProfile,
) -> GovernanceReport {
    let key = cache_key(capsule, policy, determinism, integrity);
    if let Ok(guard) = cache().lock() {
        if let Some(r) = guard.get(&key) {
            return r.clone();
        }
    }
    let policy_v = validate_capsule_against_policy(capsule, policy, None);
    let det_v = validate_determinism(capsule, determinism);
    let int_v = validate_integrity(capsule, integrity, None);
    let success = policy_v.ok && det_v.ok && int_v.ok;
    let out = GovernanceReport {
        policy: policy_v,
        determinism: det_v,
        integrity: int_v,
        success,
        ci: None,
    };
    if let Ok(mut guard) = cache().lock() {
        guard.insert(key, out.clone());
    }
    out
}

/// Merge CI check outcome for HTML/JSON (baseline comparison summary).
pub fn governance_report_with_ci(
    capsule: &AICapsuleV1,
    policy: &PolicyProfile,
    determinism: &DeterminismProfile,
    integrity: &IntegrityProfile,
    baseline_name: &str,
    ci: &CiResult,
) -> GovernanceReport {
    let mut r = validate_capsule(capsule, policy, determinism, integrity);
    let policy_ci = validate_capsule_against_policy(capsule, policy, Some(ci.replay_success));
    let int_ci = validate_integrity(capsule, integrity, Some(ci.replay_success));
    r.policy = policy_ci;
    r.integrity = int_ci;
    r.success = r.policy.ok && r.determinism.ok && r.integrity.ok && ci.success;
    r.ci = Some(GovernanceCiAttach {
        baseline_name: baseline_name.to_string(),
        drift_changed: ci.drift.changed,
        drift_fields: ci.drift.fields.clone(),
        replay_success: ci.replay_success,
        ci_policy_ok: ci.policy_ok,
        ci_determinism_ok: ci.determinism_ok,
        ci_integrity_ok: ci.integrity_ok,
        ci_success: ci.success,
    });
    r
}

/// Validate many capsules in parallel (best-effort thread fan-out).
pub fn validate_capsules_parallel(
    capsules: &[AICapsuleV1],
    policy: &PolicyProfile,
    determinism: &DeterminismProfile,
    integrity: &IntegrityProfile,
) -> Vec<GovernanceReport> {
    let mut handles = Vec::with_capacity(capsules.len());
    for c in capsules {
        let c = c.clone();
        let p = policy.clone();
        let d = *determinism;
        let i = *integrity;
        handles.push(std::thread::spawn(move || validate_capsule(&c, &p, &d, &i)));
    }
    handles
        .into_iter()
        .map(|h| {
            h.join().unwrap_or(GovernanceReport {
                policy: PolicyViolation {
                    ok: false,
                    messages: vec![line(
                        code::GOVERNANCE_JSON,
                        "validate_capsules_parallel",
                        "thread_failed",
                    )],
                },
                determinism: DeterminismViolation {
                    ok: false,
                    messages: vec![line(
                        code::GOVERNANCE_JSON,
                        "validate_capsules_parallel",
                        "thread_failed",
                    )],
                },
                integrity: IntegrityViolation {
                    ok: false,
                    messages: vec![line(
                        code::GOVERNANCE_JSON,
                        "validate_capsules_parallel",
                        "thread_failed",
                    )],
                },
                success: false,
                ci: None,
            })
        })
        .collect()
}
