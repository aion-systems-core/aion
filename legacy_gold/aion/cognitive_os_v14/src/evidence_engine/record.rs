use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// One row in the **runtime** evidence timeline (werk process ledger).
///
/// Renamed in Phase 6 so the identifier `EvidenceRecord` is reserved for
/// [`cos_core::evidence::EvidenceRecordV2`] only across the workspace.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvidenceTimelineRow {
    pub timestamp: DateTime<Utc>,
    pub process_id: String,
    pub action: String,
    pub input: Option<Value>,
    pub output: Option<Value>,
    /// Hash of the previous record in this process timeline, or empty for the genesis record.
    #[serde(default)]
    pub prev_hash: String,
    /// SHA256 hex of canonical JSON payload (fields excluding `hash`).
    #[serde(default)]
    pub hash: String,
}

impl EvidenceTimelineRow {
    pub fn new(
        process_id: impl Into<String>,
        action: impl Into<String>,
        input: Option<Value>,
        output: Option<Value>,
    ) -> Self {
        Self {
            timestamp: Utc::now(),
            process_id: process_id.into(),
            action: action.into(),
            input,
            output,
            prev_hash: String::new(),
            hash: String::new(),
        }
    }
}
