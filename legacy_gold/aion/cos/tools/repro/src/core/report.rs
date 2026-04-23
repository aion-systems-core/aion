// Unified execution report.
//
// Semantic cause vocabulary (fixed four buckets):
//   EnvironmentChange, InputChange, RuntimeChange, SystemNoise

use crate::core::artifact::ExecutionArtifact;
use crate::core::diff::{DiffReport, FieldDiff};
use crate::core::identity::{ExecutionIdentity, IdentityDelta};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExecutionReport {
    pub identity: ExecutionIdentity,
    pub trace: TraceSummary,
    pub diff: Option<DiffSummary>,
    pub root_cause: Option<RootCauseSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TraceSummary {
    pub run_id: String,
    pub command: String,
    pub exit_code: i32,
    pub stdout_bytes: usize,
    pub stderr_bytes: usize,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DiffSummary {
    pub run_a: String,
    pub run_b: String,
    pub identity_delta: IdentityDelta,
    pub differences: Vec<FieldDiff>,
    pub changed_fields: Vec<String>,
    pub unchanged_fields: Vec<String>,
    pub causal_chain: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RootCauseSummary {
    pub previous_run: Option<String>,
    pub primary: Option<SemanticCause>,
    pub causes: Vec<SemanticCause>,
    pub first_diverging_field: Option<String>,
    pub explanation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SemanticCause {
    pub category: CauseCategory,
    pub field: String,
    pub severity: Severity,
    pub previous_value: String,
    pub current_value: String,
    pub explanation: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum CauseCategory {
    /// Whitelisted env slice or cwd drift.
    EnvironmentChange,
    /// Command string / argv intent drift.
    InputChange,
    /// Exit code, stdout, stderr — observable process outcome.
    RuntimeChange,
    /// Timing-only or other low-signal capture metadata.
    SystemNoise,
}

impl CauseCategory {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EnvironmentChange => "environment_change",
            Self::InputChange => "input_change",
            Self::RuntimeChange => "runtime_change",
            Self::SystemNoise => "system_noise",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum Severity {
    High,
    Medium,
    Low,
}

impl Severity {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::High => "high",
            Self::Medium => "medium",
            Self::Low => "low",
        }
    }

    fn rank(self) -> u8 {
        match self {
            Self::High => 2,
            Self::Medium => 1,
            Self::Low => 0,
        }
    }
}

impl TraceSummary {
    pub fn from_artifact(a: &ExecutionArtifact) -> Self {
        Self {
            run_id: a.run_id.clone(),
            command: a.command.clone(),
            exit_code: a.exit_code,
            stdout_bytes: a.stdout.len(),
            stderr_bytes: a.stderr.len(),
            duration_ms: a.duration_ms,
        }
    }
}

impl DiffSummary {
    pub fn from_report(report: &DiffReport, delta: IdentityDelta) -> Self {
        Self {
            run_a: report.run_a.clone(),
            run_b: report.run_b.clone(),
            identity_delta: delta,
            differences: report.differences.clone(),
            changed_fields: report.changed_fields.clone(),
            unchanged_fields: report.unchanged_fields.clone(),
            causal_chain: report.causal_chain.clone(),
        }
    }
}

/// Priority when selecting a primary cause (lower index = higher priority).
const PRIORITY: &[CauseCategory] = &[
    CauseCategory::EnvironmentChange,
    CauseCategory::InputChange,
    CauseCategory::RuntimeChange,
    CauseCategory::SystemNoise,
];

pub fn classify(diff: &FieldDiff) -> SemanticCause {
    let (category, severity, explanation) = match diff.field.as_str() {
        "command" => (
            CauseCategory::InputChange,
            Severity::High,
            "Command invocation changed between runs. The input itself was \
             modified, so output divergence is expected.",
        ),
        "environment_hash" => (
            CauseCategory::EnvironmentChange,
            Severity::High,
            "The 15% whitelisted environment fingerprint shifted between runs \
             while the command string may be unchanged.",
        ),
        "cwd" => (
            CauseCategory::EnvironmentChange,
            Severity::High,
            "Working directory changed between runs. Relative paths and \
             tooling discovery behave differently per cwd.",
        ),
        "exit_code" => (
            CauseCategory::RuntimeChange,
            Severity::High,
            "Exit code changed between runs. The process outcome is not \
             equivalent.",
        ),
        "stderr" => {
            let severity = if diff.a.is_empty() && !diff.b.is_empty() {
                Severity::High
            } else {
                Severity::Medium
            };
            (
                CauseCategory::RuntimeChange,
                severity,
                "Error output emerged or changed between runs. Inspect stderr \
                 before looking elsewhere.",
            )
        }
        "stdout" => (
            CauseCategory::RuntimeChange,
            Severity::Medium,
            "Standard output differs between runs (real subprocess capture).",
        ),
        "duration_ms" => (
            CauseCategory::SystemNoise,
            Severity::Low,
            "Wall-clock duration differed between runs. Rarely a root cause alone.",
        ),
        "timestamp" => (
            CauseCategory::SystemNoise,
            Severity::Low,
            "Only the capture timestamp differs. Not a root cause on its own.",
        ),
        _ => (
            CauseCategory::SystemNoise,
            Severity::Low,
            "Unrecognized field divergence; treated as low-signal.",
        ),
    };

    SemanticCause {
        category,
        field: diff.field.to_string(),
        severity,
        previous_value: diff.a.clone(),
        current_value: diff.b.clone(),
        explanation: explanation.to_string(),
    }
}

pub fn select_primary(causes: &[SemanticCause]) -> Option<SemanticCause> {
    let mut best: Option<(usize, &SemanticCause)> = None;
    for c in causes {
        let prio = PRIORITY
            .iter()
            .position(|p| *p == c.category)
            .unwrap_or(PRIORITY.len());
        match best {
            None => best = Some((prio, c)),
            Some((best_prio, best_c)) => {
                let cur_rank = c.severity.rank();
                let best_rank = best_c.severity.rank();
                if cur_rank > best_rank || (cur_rank == best_rank && prio < best_prio) {
                    best = Some((prio, c));
                }
            }
        }
    }
    best.map(|(_, c)| c.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fd(field: &'static str, a: &str, b: &str) -> FieldDiff {
        FieldDiff {
            field: field.to_string(),
            a: a.to_string(),
            b: b.to_string(),
        }
    }

    #[test]
    fn runtime_change_wins_over_stdout_when_exit_differs() {
        let causes = vec![
            classify(&fd("stdout", "hello", "world")),
            classify(&fd("exit_code", "0", "1")),
        ];
        let p = select_primary(&causes).unwrap();
        assert_eq!(p.category, CauseCategory::RuntimeChange);
        assert_eq!(p.severity, Severity::High);
    }

    #[test]
    fn empty_to_nonempty_stderr_is_high_severity() {
        let c = classify(&fd("stderr", "", "boom"));
        assert_eq!(c.severity, Severity::High);
    }

    #[test]
    fn timestamp_only_is_system_noise() {
        let c = classify(&fd("timestamp", "1", "2"));
        assert_eq!(c.severity, Severity::Low);
        assert_eq!(c.category, CauseCategory::SystemNoise);
    }
}
