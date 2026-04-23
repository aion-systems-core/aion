//! Deterministic replay — single step record (data only).

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReplayRecord {
    pub index: u64,
    pub step_name: String,
    pub input: String,
    pub expected_output: String,
    pub actual_output: String,
    pub determinism: String,
}
