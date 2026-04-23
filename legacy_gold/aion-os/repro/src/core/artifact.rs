//! Single canonical execution record for local runs and CI ledger rows.
//!
//! **Execution truth is this struct** — all diff, identity, replay, and
//! storage paths deserialize into [`ExecutionArtifact`] only. Field order
//! is locked for deterministic `serde_json` (with `preserve_order`).

use crate::core::execution_boundary::{compute_env_hash, EnvSnapshot15};
use crate::core::execution_trace::ExecutionTrace;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

/// Schema version for on-disk JSON (`repro_runs/*.json`, `repro_ci_store/*/artifact.json`).
pub const EXECUTION_ARTIFACT_SCHEMA_VERSION: u32 = 4;

/// Minimal deterministic run slice (full env + argv + stdout) for inspection
/// and future diff/why. `env` uses [`BTreeMap`] so JSON key order is stable.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReproRun {
    pub id: String,
    pub command: Vec<String>,
    pub env: BTreeMap<String, String>,
    pub stdout: String,
}

#[derive(Serialize)]
struct RunIdMaterial<'a> {
    command: &'a [String],
    env: &'a BTreeMap<String, String>,
    stdout: &'a str,
}

/// SHA-256 over compact JSON of `command` + `env` + `stdout` (stable key order via `BTreeMap`).
#[must_use]
pub fn deterministic_run_id(
    command: &[String],
    env: &BTreeMap<String, String>,
    stdout: &str,
) -> String {
    let m = RunIdMaterial {
        command,
        env,
        stdout,
    };
    // `RunIdMaterial` is strings and a `BTreeMap` of strings; `serde_json` cannot fail here.
    let bytes = serde_json::to_vec(&m).unwrap();
    format!("{:x}", Sha256::digest(&bytes))
}

/// Canonical serialized execution payload (v4).
///
/// Field order is part of the public contract — do not reorder without a
/// migration and `EXECUTION_ARTIFACT_SCHEMA_VERSION` bump.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExecutionArtifact {
    pub schema_version: u32,
    pub run_id: String,
    /// Present for v4+ captures; absent on deserialized v3 artifacts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repro_run: Option<ReproRun>,
    pub command: String,
    pub cwd: String,
    pub timestamp: u64,
    pub env_snapshot: EnvSnapshot15,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
    pub duration_ms: u64,
    pub trace: ExecutionTrace,
}

impl ExecutionArtifact {
    #[must_use]
    pub fn environment_hash(&self) -> String {
        compute_env_hash(&self.env_snapshot)
    }
}
