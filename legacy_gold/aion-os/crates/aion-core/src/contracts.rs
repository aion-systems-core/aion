use serde::{Deserialize, Serialize};

pub const CAPSULE_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunResult {
    pub command: String,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
    pub duration_ms: u64,
    pub timestamp: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapsuleManifest {
    pub capsule_schema_version: u32,
    pub command: String,
    pub policy: PolicyProfile,
    pub artifact_schema_version: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyProfile {
    pub name: String,
    pub no_network: bool,
    pub deterministic_time: bool,
    pub deterministic_random: bool,
    pub max_duration_ms: Option<u64>,
}

impl PolicyProfile {
    pub fn dev() -> Self {
        Self {
            name: "dev".to_string(),
            no_network: false,
            deterministic_time: true,
            deterministic_random: true,
            max_duration_ms: None,
        }
    }

    pub fn stage() -> Self {
        Self {
            name: "stage".to_string(),
            no_network: true,
            deterministic_time: true,
            deterministic_random: true,
            max_duration_ms: Some(300_000),
        }
    }

    pub fn prod() -> Self {
        Self {
            name: "prod".to_string(),
            no_network: true,
            deterministic_time: true,
            deterministic_random: true,
            max_duration_ms: Some(120_000),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriftReport {
    pub changed: bool,
    pub fields: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhyReport {
    pub root_cause: String,
    pub suggestion: Option<String>,
}
