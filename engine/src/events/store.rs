//! Append-only event store with strict `seq` monotonicity and JSON persistence.

use super::model::{envelopes_from_run, EventEnvelope, EventStreamFile, EVENT_STREAM_SCHEMA_V2};
use aion_core::{CapsuleManifest, RunResult};
use serde_json::json;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventStore {
    run_id: String,
    events: Vec<EventEnvelope>,
}

impl EventStore {
    pub fn new(run_id: impl Into<String>) -> Self {
        Self {
            run_id: run_id.into(),
            events: Vec::new(),
        }
    }

    pub fn from_run_result(run: &RunResult, manifest: Option<&CapsuleManifest>) -> Self {
        Self {
            run_id: run.run_id.clone(),
            events: envelopes_from_run(run, manifest),
        }
    }

    pub fn run_id(&self) -> &str {
        &self.run_id
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    /// Append one envelope; `seq` must equal `last_seq + 1` (or 0 if empty).
    pub fn append(&mut self, envelope: EventEnvelope) -> Result<(), String> {
        let expected = self
            .events
            .last()
            .map(|e| e.seq.saturating_add(1))
            .unwrap_or(0);
        if envelope.seq != expected {
            return Err(format!(
                "event seq gap: expected {expected}, got {}",
                envelope.seq
            ));
        }
        self.events.push(envelope);
        Ok(())
    }

    /// Merge another store’s events after this one’s last seq (renumbering incoming).
    pub fn merge_sorted(&mut self, mut other: EventStore) -> Result<(), String> {
        let base = self
            .events
            .last()
            .map(|e| e.seq)
            .map(|s| s.saturating_add(1))
            .unwrap_or(0);
        other.renumber_from(base)?;
        for e in other.events {
            self.append(e)?;
        }
        Ok(())
    }

    fn renumber_from(&mut self, start: u64) -> Result<(), String> {
        let mut seq = start;
        for e in &mut self.events {
            e.seq = seq;
            seq = seq.saturating_add(1);
        }
        Ok(())
    }

    pub fn as_slice(&self) -> &[EventEnvelope] {
        &self.events
    }

    pub fn into_file(self) -> EventStreamFile {
        EventStreamFile {
            schema: EVENT_STREAM_SCHEMA_V2.to_string(),
            run_id: self.run_id,
            events: self.events,
        }
    }

    pub fn from_file(file: EventStreamFile) -> Result<Self, String> {
        file.validate()?;
        if file.run_id.is_empty() {
            return Err("event stream: empty run_id".into());
        }
        Ok(Self {
            run_id: file.run_id,
            events: file.events,
        })
    }

    pub fn to_json_string(&self) -> Result<String, String> {
        let f = self.clone().into_file();
        serde_json::to_string_pretty(&f).map_err(|e| e.to_string())
    }

    pub fn from_json_str(s: &str) -> Result<Self, String> {
        let f: EventStreamFile = serde_json::from_str(s).map_err(|e| e.to_string())?;
        Self::from_file(f)
    }

    /// Write `<dir>/<run_id>.events.json` atomically (best-effort).
    pub fn save_to_dir(&self, dir: &Path) -> Result<PathBuf, String> {
        fs::create_dir_all(dir).map_err(|e| e.to_string())?;
        let path = dir.join(format!("{}.events.json", self.run_id));
        let tmp = dir.join(format!("{}.events.json.tmp", self.run_id));
        let json = self.to_json_string()?;
        {
            let mut f = fs::File::create(&tmp).map_err(|e| e.to_string())?;
            f.write_all(json.as_bytes()).map_err(|e| e.to_string())?;
            f.sync_all().map_err(|e| e.to_string())?;
        }
        fs::rename(&tmp, &path).map_err(|e| e.to_string())?;
        Ok(path)
    }

    pub fn load_path(path: &Path) -> Result<Self, String> {
        let s = fs::read_to_string(path).map_err(|e| e.to_string())?;
        Self::from_json_str(&s)
    }

    /// Stable summary for diffing two stores (ordered lines).
    pub fn canonical_summary_json(&self) -> Result<String, String> {
        let lines: Vec<serde_json::Value> = self
            .events
            .iter()
            .map(|e| {
                json!({
                    "seq": e.seq,
                    "category": format!("{:?}", e.event.category()),
                    "event": e.event,
                    "attrs": e.attrs,
                })
            })
            .collect();
        serde_json::to_string(&lines).map_err(|e| e.to_string())
    }
}
