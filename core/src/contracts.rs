//! Canonical JSON-serializable contracts for kernel, engine, and CLI.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::env;
use crate::error::code;

/// Capsule ZIP layout version.
pub const CAPSULE_SCHEMA_VERSION: u32 = 1;

/// Execution artifact JSON schema (aligned with engine output).
pub const EXECUTION_ARTIFACT_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RunResult {
    pub schema_version: u32,
    pub run_id: String,
    pub command: String,
    pub cwd: String,
    pub timestamp: u64,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
    pub duration_ms: u64,
    pub env_fingerprint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CapsuleManifest {
    pub capsule_schema_version: u32,
    pub execution_artifact_schema_version: u32,
    pub run_id: String,
    pub command: String,
    pub policy: PolicyProfile,
    pub determinism: DeterminismProfile,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PolicyProfile {
    pub name: String,
    pub no_network: bool,
    pub deterministic_time: bool,
    pub deterministic_random: bool,
    pub max_duration_ms: Option<u64>,
}

impl Default for PolicyProfile {
    fn default() -> Self {
        Self::dev()
    }
}

impl PolicyProfile {
    pub fn dev() -> Self {
        Self {
            name: "dev".into(),
            no_network: false,
            deterministic_time: true,
            deterministic_random: true,
            max_duration_ms: None,
        }
    }

    pub fn stage() -> Self {
        Self {
            name: "stage".into(),
            no_network: true,
            deterministic_time: true,
            deterministic_random: true,
            max_duration_ms: Some(300_000),
        }
    }

    pub fn prod() -> Self {
        Self {
            name: "prod".into(),
            no_network: true,
            deterministic_time: true,
            deterministic_random: true,
            max_duration_ms: Some(120_000),
        }
    }
}

/// Frozen execution context captured before an AI run (deterministic replay metadata).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExecutionEnvelope {
    /// Wall-clock milliseconds since UNIX epoch, mixed with a monotonic snapshot at capture.
    pub frozen_time_ms: u64,
    pub frozen_env: BTreeMap<String, String>,
    pub frozen_cwd: String,
    pub frozen_random_seed: u64,
    pub determinism_profile: DeterminismProfile,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeterminismProfile {
    pub time_frozen: bool,
    pub time_epoch_secs: u64,
    pub random_seed: u64,
    pub syscall_intercept: bool,
    /// Determinism profile v2 — explicit freeze toggles (additive; serde defaults preserve old capsules).
    #[serde(default = "default_true")]
    pub freeze_time: bool,
    #[serde(default = "default_true")]
    pub freeze_env: bool,
    #[serde(default = "default_true")]
    pub freeze_random: bool,
    #[serde(default = "default_true")]
    pub freeze_cwd: bool,
    #[serde(default)]
    pub strict_replay: bool,
    #[serde(default)]
    pub io_policy: DeterministicIOPolicy,
}

fn default_true() -> bool {
    true
}

/// Deterministic syscall / IO surface for replay (additive on [`DeterminismProfile`]).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum DeterministicIOPolicy {
    /// Only the syscall whitelist is permitted (deterministic capture / replay path).
    Strict,
    /// Observe and record all syscalls; do not block on policy alone.
    #[default]
    Audit,
    /// Block any syscall outside the whitelist.
    Deny,
}

impl Default for DeterminismProfile {
    fn default() -> Self {
        Self {
            time_frozen: true,
            time_epoch_secs: 0,
            random_seed: 0xdeadbeefdeadbeef,
            syscall_intercept: false,
            freeze_time: true,
            freeze_env: true,
            freeze_random: true,
            freeze_cwd: true,
            strict_replay: false,
            io_policy: DeterministicIOPolicy::default(),
        }
    }
}

impl ExecutionEnvelope {
    /// Fully synthetic envelope for unit tests (no wall clock, no filesystem).
    pub fn synthetic_deterministic(run_seed: u64) -> Self {
        Self {
            frozen_time_ms: run_seed.wrapping_mul(1_000),
            frozen_env: BTreeMap::new(),
            frozen_cwd: "/synthetic/cwd".into(),
            frozen_random_seed: run_seed.rotate_left(17),
            determinism_profile: DeterminismProfile {
                time_frozen: true,
                time_epoch_secs: (run_seed % 4_000_000_000).max(1),
                random_seed: run_seed,
                syscall_intercept: false,
                ..Default::default()
            },
        }
    }
}

impl DeterminismProfile {
    /// Apply governance-style process hints for active freeze flags (best-effort; no-op where unsupported).
    pub fn apply_profile(&self) {
        env::set_var(
            "AION_FREEZE_TIME",
            if self.freeze_time || self.time_frozen {
                "1"
            } else {
                "0"
            },
        );
        env::set_var(
            "AION_FREEZE_RANDOM",
            if self.freeze_random || self.syscall_intercept {
                "1"
            } else {
                "0"
            },
        );
        env::set_var(
            "AION_FREEZE_ENV",
            if self.freeze_env { "1" } else { "0" },
        );
        env::set_var(
            "AION_FREEZE_CWD",
            if self.freeze_cwd { "1" } else { "0" },
        );
    }

    /// When `strict_replay` is set on the original profile, replayed runs must carry an identical profile.
    pub fn validate_replay_profile(&self, replayed: &DeterminismProfile) -> Result<(), String> {
        if self.strict_replay && self != replayed {
            return Err("determinism_profile: strict_replay requires byte-identical DeterminismProfile on replay".into());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct DriftToleranceProfile {
    pub max_token_delta: usize,
    pub max_event_delta: usize,
    pub max_timing_delta_ms: u64,
    pub max_labels: usize,
}

impl DriftToleranceProfile {
    pub fn deterministic_default() -> Self {
        Self {
            max_token_delta: 0,
            max_event_delta: 0,
            max_timing_delta_ms: 0,
            max_labels: 64,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct DriftToleranceViolation {
    pub label: String,
    pub actual: u64,
    pub limit: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct DriftReport {
    pub changed: bool,
    #[serde(default)]
    pub categories: Vec<String>,
    #[serde(default)]
    pub labels: Vec<String>,
    pub fields: Vec<String>,
    pub details: Vec<String>,
    #[serde(default = "default_drift_tolerance_profile")]
    pub tolerance_profile: DriftToleranceProfile,
    #[serde(default)]
    pub tolerance_violations: Vec<DriftToleranceViolation>,
    #[serde(default)]
    pub overflow: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

fn default_drift_tolerance_profile() -> DriftToleranceProfile {
    DriftToleranceProfile::deterministic_default()
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WhyReport {
    pub summary: String,
    pub first_divergent_field: Option<String>,
    pub suggestion: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct FsSnapshot {
    pub roots: Vec<String>,
    pub entries: Vec<(String, u64)>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct FsDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub modified: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct NetLog {
    pub events: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NetPolicy {
    pub deny_outbound: bool,
    pub allow_loopback: bool,
}

impl Default for NetPolicy {
    fn default() -> Self {
        Self {
            deny_outbound: true,
            allow_loopback: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConsistencyFinality {
    pub status: String,
    pub code: String,
    pub context: String,
    pub origin: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GlobalConsistencyContract {
    pub run_finality: ConsistencyFinality,
    pub capsule_finality: ConsistencyFinality,
    pub evidence_finality: ConsistencyFinality,
    pub replay_finality: ConsistencyFinality,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GlobalConsistencySignals {
    pub replay_invariant_ok: bool,
    pub replay_symmetry_ok: bool,
    pub replay_cross_machine_ok: bool,
    pub drift_ok: bool,
    pub policy_ok: bool,
    pub evidence_verified: bool,
    pub evidence_open_anchors: bool,
    pub capsule_complete: bool,
    pub capsule_referencable: bool,
    pub capsule_signature_required: bool,
    pub capsule_signature_present: bool,
}

fn finality_ok(context: &str, origin: &str) -> ConsistencyFinality {
    ConsistencyFinality {
        status: "ok".to_string(),
        code: "AION_OK".to_string(),
        context: context.to_string(),
        origin: origin.to_string(),
        cause: None,
    }
}

fn finality_error(code: &str, context: &str, origin: &str, cause: &str) -> ConsistencyFinality {
    ConsistencyFinality {
        status: "error".to_string(),
        code: code.to_string(),
        context: context.to_string(),
        origin: origin.to_string(),
        cause: Some(cause.to_string()),
    }
}

pub fn evaluate_global_consistency_contract(s: &GlobalConsistencySignals) -> GlobalConsistencyContract {
    let replay_finality = if !s.replay_invariant_ok {
        finality_error(code::REPLAY_MISMATCH, "global_consistency.replay_finality", "replay", "replay:invariant_failed")
    } else if !s.replay_symmetry_ok {
        finality_error(code::REPLAY_SYMMETRY, "global_consistency.replay_finality", "replay", "replay:symmetry_failed")
    } else if !s.replay_cross_machine_ok {
        finality_error(code::REPLAY_MISMATCH, "global_consistency.replay_finality", "replay", "replay:cross_machine_failed")
    } else {
        finality_ok("global_consistency.replay_finality", "replay")
    };

    let evidence_finality = if !s.evidence_verified {
        finality_error(code::EVIDENCE_HASH, "global_consistency.evidence_finality", "evidence", "evidence:verify_failed")
    } else if s.evidence_open_anchors {
        finality_error(code::EVIDENCE_ANCHOR, "global_consistency.evidence_finality", "evidence", "evidence:open_anchors")
    } else {
        finality_ok("global_consistency.evidence_finality", "evidence")
    };

    let run_finality = if replay_finality.status != "ok" {
        finality_error(&replay_finality.code, "global_consistency.run_finality", "doctor", "run:replay_not_final")
    } else if !s.drift_ok {
        finality_error(code::DRIFT_TOLERANCE, "global_consistency.run_finality", "doctor", "run:drift_not_final")
    } else if !s.policy_ok {
        finality_error(code::GOVERNANCE_JSON, "global_consistency.run_finality", "doctor", "run:policy_not_final")
    } else if evidence_finality.status != "ok" {
        finality_error(&evidence_finality.code, "global_consistency.run_finality", "doctor", "run:evidence_not_final")
    } else {
        finality_ok("global_consistency.run_finality", "doctor")
    };

    let capsule_finality = if !s.capsule_complete {
        finality_error(code::CAPSULE_VALIDATE, "global_consistency.capsule_finality", "capsule", "capsule:incomplete")
    } else if !s.capsule_referencable {
        finality_error(code::CAPSULE_VALIDATE, "global_consistency.capsule_finality", "capsule", "capsule:not_referencable")
    } else if s.capsule_signature_required && !s.capsule_signature_present {
        finality_error(code::EVIDENCE_HASH, "global_consistency.capsule_finality", "capsule", "capsule:signature_missing")
    } else {
        finality_ok("global_consistency.capsule_finality", "capsule")
    };

    GlobalConsistencyContract {
        run_finality,
        capsule_finality,
        evidence_finality,
        replay_finality,
    }
}

#[cfg(test)]
mod tests {
    use super::{evaluate_global_consistency_contract, GlobalConsistencySignals};

    fn all_ok_signals() -> GlobalConsistencySignals {
        GlobalConsistencySignals {
            replay_invariant_ok: true,
            replay_symmetry_ok: true,
            replay_cross_machine_ok: true,
            drift_ok: true,
            policy_ok: true,
            evidence_verified: true,
            evidence_open_anchors: false,
            capsule_complete: true,
            capsule_referencable: true,
            capsule_signature_required: false,
            capsule_signature_present: false,
        }
    }

    #[test]
    fn run_not_final_when_replay_fails() {
        let mut s = all_ok_signals();
        s.replay_symmetry_ok = false;
        let c = evaluate_global_consistency_contract(&s);
        assert_eq!(c.run_finality.status, "error");
        assert_eq!(c.run_finality.cause.as_deref(), Some("run:replay_not_final"));
    }

    #[test]
    fn run_not_final_when_evidence_fails() {
        let mut s = all_ok_signals();
        s.evidence_verified = false;
        let c = evaluate_global_consistency_contract(&s);
        assert_eq!(c.run_finality.status, "error");
        assert_eq!(c.run_finality.cause.as_deref(), Some("run:evidence_not_final"));
    }

    #[test]
    fn run_not_final_when_policy_fails() {
        let mut s = all_ok_signals();
        s.policy_ok = false;
        let c = evaluate_global_consistency_contract(&s);
        assert_eq!(c.run_finality.status, "error");
        assert_eq!(c.run_finality.cause.as_deref(), Some("run:policy_not_final"));
    }

    #[test]
    fn run_final_when_all_core_paths_ok() {
        let s = all_ok_signals();
        let c = evaluate_global_consistency_contract(&s);
        assert_eq!(c.run_finality.status, "ok");
        assert_eq!(c.replay_finality.status, "ok");
        assert_eq!(c.evidence_finality.status, "ok");
        assert_eq!(c.capsule_finality.status, "ok");
    }
}
