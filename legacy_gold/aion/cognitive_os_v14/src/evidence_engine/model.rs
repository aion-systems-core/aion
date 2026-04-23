use crate::evidence_engine::record::EvidenceTimelineRow;
use crate::evidence_engine::timeline::EvidenceTimeline;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct EvidenceModel {
    pub timeline: EvidenceTimeline,
}

impl EvidenceModel {
    pub fn append_record(&mut self, record: EvidenceTimelineRow) -> Result<()> {
        self.timeline.append(record);
        Ok(())
    }
}
