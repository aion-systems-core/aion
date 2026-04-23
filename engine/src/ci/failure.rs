//! Failure classification for CI drift.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum FailureKind {
    None,
    DriftStdout,
    DriftStderr,
    DriftExit,
    DriftEnv,
    DriftOther,
}

impl FailureKind {
    pub fn from_fields(fields: &[String]) -> Self {
        if fields.is_empty() {
            return FailureKind::None;
        }
        if fields.iter().any(|f| f == "stdout") {
            FailureKind::DriftStdout
        } else if fields.iter().any(|f| f == "stderr") {
            FailureKind::DriftStderr
        } else if fields.iter().any(|f| f == "exit_code") {
            FailureKind::DriftExit
        } else if fields.iter().any(|f| f == "env_fingerprint") {
            FailureKind::DriftEnv
        } else {
            FailureKind::DriftOther
        }
    }
}
