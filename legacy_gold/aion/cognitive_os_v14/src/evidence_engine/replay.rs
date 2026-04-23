use crate::evidence_engine::diff::diff;
use crate::evidence_engine::record::EvidenceTimelineRow;
use crate::evidence_engine::timeline::EvidenceTimeline;
use anyhow::Result;

pub fn replay(process_id: impl AsRef<str>, from: usize, to: usize) -> Result<Vec<EvidenceTimelineRow>> {
    let t = EvidenceTimeline::load(process_id.as_ref())?;
    Ok(diff(&t, from, to).changes)
}
