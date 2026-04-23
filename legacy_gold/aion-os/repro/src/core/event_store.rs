//! Persistent canonical event stream (`*.events.json`) alongside artifacts.

use crate::core::execution_trace::{ExecutionEvent, ExecutionTrace};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

pub const EVENT_STREAM_SCHEMA_V1: &str = "aion/event_stream/v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EventStreamFile {
    pub schema: String,
    pub run_id: String,
    pub events: Vec<ExecutionEvent>,
}

/// Write `repro_runs/<run_id>.events.json` under the process cwd (same layout as [`crate::core::storage::runs_dir`]).
pub fn save_event_stream(run_id: &str, trace: &ExecutionTrace) -> io::Result<()> {
    save_event_stream_in(
        &PathBuf::from(crate::core::storage::RUNS_DIR),
        run_id,
        trace,
    )
}

pub fn save_event_stream_in(dir: &Path, run_id: &str, trace: &ExecutionTrace) -> io::Result<()> {
    fs::create_dir_all(dir)?;
    let payload = EventStreamFile {
        schema: EVENT_STREAM_SCHEMA_V1.to_string(),
        run_id: run_id.to_string(),
        events: trace.events.clone(),
    };
    let json = serde_json::to_string_pretty(&payload)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    let path = dir.join(format!("{run_id}.events.json"));
    write_events_atomically(&path, run_id, json.as_bytes())
}

pub fn load_event_stream(run_id: &str) -> io::Result<ExecutionTrace> {
    load_event_stream_in(&PathBuf::from(crate::core::storage::RUNS_DIR), run_id)
}

pub fn load_event_stream_in(dir: &Path, run_id: &str) -> io::Result<ExecutionTrace> {
    let path = dir.join(format!("{run_id}.events.json"));
    let mut s = String::new();
    fs::File::open(&path)?.read_to_string(&mut s)?;
    let file: EventStreamFile =
        serde_json::from_str(&s).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    if file.schema != EVENT_STREAM_SCHEMA_V1 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("unsupported event stream schema: {}", file.schema),
        ));
    }
    if file.run_id != run_id {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!(
                "event stream run_id mismatch: file {} vs requested {}",
                file.run_id, run_id
            ),
        ));
    }
    Ok(ExecutionTrace {
        run_id: file.run_id,
        events: file.events,
    })
}

fn write_events_atomically(path: &Path, run_id: &str, bytes: &[u8]) -> io::Result<()> {
    let dir = path.parent().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            "event stream path has no parent",
        )
    })?;
    let tmp = dir.join(format!("{run_id}.events.json.tmp"));
    {
        let mut f = fs::File::create(&tmp)?;
        f.write_all(bytes)?;
        f.sync_all()?;
    }
    fs::rename(&tmp, path)
}
