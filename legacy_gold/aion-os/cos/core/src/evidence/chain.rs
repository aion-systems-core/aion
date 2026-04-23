//! Evidence Layer v2 — ordered record list (data only).

use super::record::EvidenceRecordV2;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EvidenceChainV2 {
    pub records: Vec<EvidenceRecordV2>,
}
