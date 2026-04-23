//! Why engine 2.0: drift + correlation + ranked hypotheses.

use aion_core::{RunResult, WhyReport};

mod core;
mod correlate;
mod diff;
mod rank;

pub use core::explain_pair;

/// Back-compat alias.
pub fn why_pair(left_json: &str, right_json: &str) -> Result<WhyReport, String> {
    explain_pair(left_json, right_json)
}

/// Structured entrypoint for callers that already hold [`RunResult`] values.
pub fn why_run_pair(left: &RunResult, right: &RunResult) -> Result<WhyReport, String> {
    let l = serde_json::to_string(left).map_err(|e| e.to_string())?;
    let r = serde_json::to_string(right).map_err(|e| e.to_string())?;
    why_pair(&l, &r)
}
pub use correlate::env_stdout_linked;
pub use diff::{drift_event_projection, drift_runs, run_pair_from_json};
pub use rank::{rank, Hypothesis};
