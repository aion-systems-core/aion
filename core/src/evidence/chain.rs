//! Linear evidence chain anchored to a run.

use super::hashing::sha256_hex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EvidenceRecord {
    pub seq: u32,
    pub kind: String,
    /// Hash of the raw step bytes.
    pub leaf_digest: String,
    /// Rolling chain digest including parent.
    pub payload_digest: String,
    pub parent_digest: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EvidenceChain {
    pub run_id: String,
    pub records: Vec<EvidenceRecord>,
    /// Formal replay invariant check (original vs replay); `None` when not evaluated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub formal_replay_invariant_ok: Option<bool>,
    /// Cross-machine replay validation (strict runtime, tolerant machine).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cross_machine_replay_ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct EvidenceReplayAnchors {
    pub formal_replay_invariant_ok: Option<bool>,
    pub cross_machine_replay_ok: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct EvidenceContract {
    pub run_id: String,
    pub root_anchor: String,
    pub rolling_hashes: Vec<String>,
    pub replay_anchors: EvidenceReplayAnchors,
    pub records: Vec<EvidenceRecord>,
}

impl Default for EvidenceChain {
    fn default() -> Self {
        Self {
            run_id: String::new(),
            records: Vec::new(),
            formal_replay_invariant_ok: None,
            cross_machine_replay_ok: None,
        }
    }
}

impl EvidenceChain {
    pub fn root_digest(&self) -> String {
        self.records
            .last()
            .map(|r| r.payload_digest.clone())
            .unwrap_or_else(|| sha256_hex(b"empty"))
    }

    pub fn contract_view(&self) -> EvidenceContract {
        EvidenceContract {
            run_id: self.run_id.clone(),
            root_anchor: self.root_digest(),
            rolling_hashes: self
                .records
                .iter()
                .map(|r| r.payload_digest.clone())
                .collect(),
            replay_anchors: EvidenceReplayAnchors {
                formal_replay_invariant_ok: self.formal_replay_invariant_ok,
                cross_machine_replay_ok: self.cross_machine_replay_ok,
            },
            records: self.records.clone(),
        }
    }
}
