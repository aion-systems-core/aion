//! Deterministic replay — loaded chain context (data only).

use super::record::ReplayRecord;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReplayContext {
    pub records: Vec<ReplayRecord>,
}
