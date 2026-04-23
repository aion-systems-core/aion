//! Deterministic failure classification for captured artifacts.

use crate::core::artifact::ExecutionArtifact;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FailureType {
    EnvironmentFailure,
    RuntimeFailure,
    AssertionFailure,
    GenericFailure,
}

impl std::fmt::Display for FailureType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FailureType::EnvironmentFailure => write!(f, "EnvironmentFailure"),
            FailureType::RuntimeFailure => write!(f, "RuntimeFailure"),
            FailureType::AssertionFailure => write!(f, "AssertionFailure"),
            FailureType::GenericFailure => write!(f, "GenericFailure"),
        }
    }
}

/// Explicit opt-in for CI harnesses (tests).
pub fn ci_force_failure_flag() -> bool {
    matches!(
        std::env::var("REPRO_CI_FORCE_FAILURE")
            .unwrap_or_default()
            .as_str(),
        "1" | "true" | "TRUE"
    )
}

pub fn is_failure(artifact: &ExecutionArtifact) -> bool {
    if ci_force_failure_flag() {
        return true;
    }
    if artifact.exit_code != 0 {
        return true;
    }
    if !artifact.stderr.is_empty() && artifact.stdout.is_empty() {
        return true;
    }
    false
}

pub fn classify_failure(artifact: &ExecutionArtifact) -> FailureType {
    let e = artifact.stderr.to_lowercase();
    if artifact.stderr.contains("spawn error") {
        return FailureType::RuntimeFailure;
    }
    if artifact.exit_code == 127 || e.contains("not found") || e.contains("no such file") {
        return FailureType::EnvironmentFailure;
    }
    if artifact.exit_code != 0 && artifact.stdout.is_empty() && !artifact.stderr.is_empty() {
        return FailureType::AssertionFailure;
    }
    if artifact.exit_code != 0 {
        return FailureType::GenericFailure;
    }
    if !artifact.stderr.is_empty() && artifact.stdout.is_empty() {
        return FailureType::AssertionFailure;
    }
    FailureType::GenericFailure
}
