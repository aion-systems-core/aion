//! Governance DeterminismProfile v1 — rules checked against capsule runtime determinism metadata.

use crate::ai::AICapsuleV1;
use aion_core::error::{code, io_cause, line};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeterminismProfile {
    pub freeze_time: bool,
    pub freeze_random: bool,
    pub freeze_env: bool,
    pub freeze_io: bool,
    pub freeze_network: bool,
    pub freeze_parallelism: bool,
}

impl Default for DeterminismProfile {
    fn default() -> Self {
        Self {
            freeze_time: false,
            freeze_random: false,
            freeze_env: false,
            freeze_io: false,
            freeze_network: false,
            freeze_parallelism: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeterminismViolation {
    pub ok: bool,
    pub messages: Vec<String>,
}

impl DeterminismViolation {
    pub fn pass() -> Self {
        Self {
            ok: true,
            messages: vec![],
        }
    }
}

/// Reserved hook for applying governance determinism in a runtime / kernel (no-op in-engine).
pub fn apply_determinism_profile(profile: &DeterminismProfile) {
    std::env::set_var(
        "AION_FREEZE_TIME",
        if profile.freeze_time { "1" } else { "0" },
    );
    std::env::set_var(
        "AION_FREEZE_RANDOM",
        if profile.freeze_random { "1" } else { "0" },
    );
    std::env::set_var(
        "AION_FREEZE_ENV",
        if profile.freeze_env { "1" } else { "0" },
    );
    std::env::set_var("AION_FREEZE_IO", if profile.freeze_io { "1" } else { "0" });
    std::env::set_var(
        "AION_FREEZE_NETWORK",
        if profile.freeze_network { "1" } else { "0" },
    );
    std::env::set_var(
        "AION_FREEZE_PARALLELISM",
        if profile.freeze_parallelism { "1" } else { "0" },
    );
}

pub fn load_determinism(path: &Path) -> Result<DeterminismProfile, String> {
    let s = fs::read_to_string(path)
        .map_err(|e| line(code::GOVERNANCE_IO, "load_determinism", &io_cause(&e)))?;
    serde_json::from_str(&s)
        .map_err(|_| line(code::GOVERNANCE_JSON, "load_determinism", "invalid_json"))
}

/// Validates `capsule.determinism` ([`aion_core::DeterminismProfile`]) against governance rules.
pub fn validate_determinism(
    capsule: &AICapsuleV1,
    profile: &DeterminismProfile,
) -> DeterminismViolation {
    let mut messages = Vec::new();
    let d = &capsule.determinism;

    if profile.freeze_time && !d.time_frozen {
        messages.push("freeze_time: capsule determinism.time_frozen is false".into());
    }

    if profile.freeze_random && !(d.freeze_random || d.syscall_intercept) {
        messages.push(
            "freeze_random: capsule determinism.freeze_random and syscall_intercept are false"
                .into(),
        );
    }

    // freeze_env / freeze_io / freeze_network / freeze_parallelism: reserved for future capsule
    // fields or kernel attestations (v1 does not fail on these flags alone).

    messages.sort();
    DeterminismViolation {
        ok: messages.is_empty(),
        messages,
    }
}
