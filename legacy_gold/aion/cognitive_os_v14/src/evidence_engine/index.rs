use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

pub fn index_path() -> PathBuf {
    PathBuf::from("evidence").join("index.json")
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceIndex {
    pub processes: Vec<String>,
}

impl EvidenceIndex {
    pub fn load() -> Result<Self> {
        let path = index_path();
        if path.is_file() {
            let raw = fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
            serde_json::from_str(&raw).with_context(|| format!("parse {}", path.display()))
        } else {
            Ok(Self {
                processes: Vec::new(),
            })
        }
    }

    pub fn save(&self) -> Result<()> {
        if let Some(parent) = index_path().parent() {
            fs::create_dir_all(parent).with_context(|| format!("create_dir_all {}", parent.display()))?;
        }
        let path = index_path();
        let tmp = path.with_extension("json.tmp");
        let body = serde_json::to_string_pretty(self).context("serialize EvidenceIndex")?;
        fs::write(&tmp, body).with_context(|| format!("write {}", tmp.display()))?;
        fs::rename(&tmp, &path).with_context(|| format!("rename {} -> {}", tmp.display(), path.display()))?;
        Ok(())
    }

    pub fn add_process(&mut self, process_id: String) {
        if !self.processes.contains(&process_id) {
            self.processes.push(process_id);
        }
    }
}
