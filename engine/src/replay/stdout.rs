//! Deterministic stdout replay from serialized [`RunResult`].

use aion_core::RunResult;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReplayReport {
    pub stdout: String,
}

pub fn replay_stdout(run_json: &str) -> Result<String, String> {
    Ok(replay_report(run_json)?.stdout)
}

pub fn replay_report(run_json: &str) -> Result<ReplayReport, String> {
    let r: RunResult = serde_json::from_str(run_json).map_err(|e| format!("replay: {e}"))?;
    Ok(ReplayReport { stdout: r.stdout })
}
