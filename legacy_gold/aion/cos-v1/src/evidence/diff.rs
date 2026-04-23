//! Cross-run evidence comparison: step sets and I/O hashes only (no timestamps).

use serde::{Deserialize, Serialize};

use super::EvidenceIndex;

/// Step names only; vectors are sorted by construction (walk sorted map keys).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub changed: Vec<String>,
}

/// Compare two indexes: `added` / `removed` by step set; `changed` when `input_hash` or `output_hash` differ.
pub fn diff_evidence(a: &EvidenceIndex, b: &EvidenceIndex) -> EvidenceDiff {
    let mut added = Vec::new();
    for step in b.by_step.keys() {
        if !a.by_step.contains_key(step) {
            added.push(step.clone());
        }
    }
    let mut removed = Vec::new();
    for step in a.by_step.keys() {
        if !b.by_step.contains_key(step) {
            removed.push(step.clone());
        }
    }
    let mut changed = Vec::new();
    for step in a.by_step.keys() {
        if let (Some(pa), Some(pb)) = (a.by_step.get(step), b.by_step.get(step)) {
            if pa.input_hash != pb.input_hash || pa.output_hash != pb.output_hash {
                changed.push(step.clone());
            }
        }
    }
    EvidenceDiff {
        added,
        removed,
        changed,
    }
}
