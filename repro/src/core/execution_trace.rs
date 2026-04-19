//! Minimal event timeline for an execution (Phase 2). Complements the
//! field-based [`crate::core::artifact::ExecutionArtifact`] without replacing it.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ExecutionEvent {
    Spawn { command: String },
    EnvResolved { keys: Vec<String> },
    Stdout { chunk: String },
    Stderr { chunk: String },
    Exit { code: i32 },
    Timing { duration_ms: u128 },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExecutionTrace {
    pub run_id: String,
    pub events: Vec<ExecutionEvent>,
}
