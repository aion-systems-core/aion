//! Execution trace: ordered spans derived from [`RunResult`](aion_core::RunResult) + events.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TraceSpan {
    pub seq: u64,
    pub op: String,
    pub surface: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Trace {
    pub run_id: String,
    pub spans: Vec<TraceSpan>,
}
