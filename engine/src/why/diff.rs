//! Diff helpers for why-engine (artifact + optional event projection).

use crate::diff::diff_runs;
use crate::events::{EventReader, EventStore};
use aion_core::{DriftReport, RunResult};

pub fn drift_runs(left_json: &str, right_json: &str) -> Result<DriftReport, String> {
    diff_runs(left_json, right_json)
}

pub fn drift_event_projection(left: &EventStore, right: &EventStore) -> DriftReport {
    EventReader::diff_summaries(&EventReader::new(left), &EventReader::new(right))
}

pub fn run_pair_from_json(left: &str, right: &str) -> Result<(RunResult, RunResult), String> {
    let a: RunResult = serde_json::from_str(left).map_err(|e| e.to_string())?;
    let b: RunResult = serde_json::from_str(right).map_err(|e| e.to_string())?;
    Ok((a, b))
}
