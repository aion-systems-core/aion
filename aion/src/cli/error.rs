//! Deterministic, structured errors for the CLI shell (no panics on expected paths).

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AionError {
    ToolNotFound {
        requested: String,
        available: String,
    },
    ExecutionFailed(String),
    InvalidArgs(String),
}

impl fmt::Display for AionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AionError::ToolNotFound {
                requested,
                available,
            } => write!(
                f,
                "unknown tool '{requested}'. Available tools: {available}"
            ),
            AionError::ExecutionFailed(msg) => write!(f, "{msg}"),
            AionError::InvalidArgs(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for AionError {}
