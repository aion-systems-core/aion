//! Evidence Layer v2 — single record shape (data only).

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EvidenceRecordV2 {
    pub index: u64,
    pub step_name: String,
    pub input: String,
    pub output: String,
    pub determinism: String,
    pub forbidden_ops: Vec<String>,
    pub previous_hash: String,
    pub current_hash: String,
}
