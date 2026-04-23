use crate::evidence_engine::record::EvidenceTimelineRow;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

pub fn timeline_dir(process_id: impl AsRef<str>) -> PathBuf {
    PathBuf::from("evidence").join(process_id.as_ref())
}

pub fn timeline_path(process_id: impl AsRef<str>) -> PathBuf {
    timeline_dir(process_id).join("timeline.json")
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EvidenceTimeline {
    pub process_id: String,
    pub records: Vec<EvidenceTimelineRow>,
}

impl EvidenceTimeline {
    pub fn load(process_id: impl AsRef<str>) -> Result<Self> {
        let pid = process_id.as_ref().to_string();
        let path = timeline_path(&pid);
        if path.is_file() {
            let raw = fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
            let mut t: EvidenceTimeline = serde_json::from_str(&raw)
                .with_context(|| format!("parse {}", path.display()))?;
            t.process_id = pid;
            Ok(t)
        } else {
            Ok(Self {
                process_id: pid,
                records: Vec::new(),
            })
        }
    }

    pub fn save(&self) -> Result<()> {
        let dir = timeline_dir(&self.process_id);
        fs::create_dir_all(&dir).with_context(|| format!("create_dir_all {}", dir.display()))?;
        let path = timeline_path(&self.process_id);
        let tmp = path.with_extension("json.tmp");
        let body = serde_json::to_string_pretty(self).context("serialize EvidenceTimeline")?;
        fs::write(&tmp, body).with_context(|| format!("write {}", tmp.display()))?;
        fs::rename(&tmp, &path).with_context(|| format!("rename {} -> {}", tmp.display(), path.display()))?;
        Ok(())
    }

    pub fn append(&mut self, record: EvidenceTimelineRow) {
        self.records.push(record);
    }
}
