//! Token trace and timeline events for AI Capsule v1.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// One emitted token with a monotonic logical clock (not wall time).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct AiTokenEvent {
    pub index: u32,
    pub token: String,
    #[serde(default)]
    pub token_id: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logits: Option<Vec<(String, String)>>,
    /// Monotonic tick (0, 1, 2, …) for deterministic ordering.
    pub timestamp: u64,
}

/// High-level stream events for audit (deterministic variant order).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Event {
    RunStart { model: String },
    PromptIngested { chars: usize },
    TokenGenerated { index: u32, token: String },
    RunComplete { token_count: usize },
    /// Deterministic syscall capture (whitelist policy).
    SyscallCaptured {
        id: u64,
        name: String,
        args: Value,
        result: Value,
        #[serde(default = "syscall_deterministic_default")]
        deterministic: bool,
    },
    /// Governance-style policy violation on syscall / IO surface.
    PolicyViolation {
        syscall: String,
        reason: String,
        severity: String,
    },
}

fn syscall_deterministic_default() -> bool {
    true
}
