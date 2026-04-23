use crate::evidence_engine::record::EvidenceTimelineRow;
use crate::evidence_engine::timeline::EvidenceTimeline;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvidenceDiff {
    pub process_id: String,
    pub from: usize,
    pub to: usize,
    pub changes: Vec<EvidenceTimelineRow>,
}

pub fn diff(timeline: &EvidenceTimeline, from: usize, to: usize) -> EvidenceDiff {
    let len = timeline.records.len();
    let changes = if from <= to && from < len {
        let end = (to + 1).min(len);
        timeline.records[from..end].to_vec()
    } else {
        Vec::new()
    };
    EvidenceDiff {
        process_id: timeline.process_id.clone(),
        from,
        to,
        changes,
    }
}
