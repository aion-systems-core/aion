// CI root cause — fixed priority, deterministic confidence (no ML).
// Inputs: only `compare_ci_runs` fields (15% env hash, command, normalized I/O).

use crate::ci::diff::compare_ci_runs;
use crate::core::artifact::ExecutionArtifact;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PrimaryCause {
    EnvironmentChange,
    InputChange,
    RuntimeChange,
    SystemNoise,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RootCauseReport {
    pub primary_cause: PrimaryCause,
    pub explanation: String,
    pub confidence_score: f64,
}

impl PrimaryCause {
    fn as_str(self) -> &'static str {
        match self {
            Self::EnvironmentChange => "ENVIRONMENT_CHANGE",
            Self::InputChange => "INPUT_CHANGE",
            Self::RuntimeChange => "RUNTIME_CHANGE",
            Self::SystemNoise => "SYSTEM_NOISE",
        }
    }
}

fn confidence_for(cause: PrimaryCause) -> f64 {
    match cause {
        PrimaryCause::EnvironmentChange => 0.91,
        PrimaryCause::InputChange => 0.88,
        PrimaryCause::RuntimeChange => 0.82,
        PrimaryCause::SystemNoise => 0.55,
    }
}

/// Fixed order: environment → input (argv) → runtime (exit/streams) → noise.
pub fn explain_ci_root_cause(a: &ExecutionArtifact, b: &ExecutionArtifact) -> RootCauseReport {
    let cmp = compare_ci_runs(a, b);

    if cmp.env_diff.changed {
        return RootCauseReport {
            primary_cause: PrimaryCause::EnvironmentChange,
            explanation: "The 15% environment fingerprint (cwd + PATH/HOME/CI/SHELL/LANG) \
                            changed between runs. Treat as infra drift before blaming code."
                .to_string(),
            confidence_score: confidence_for(PrimaryCause::EnvironmentChange),
        };
    }

    if cmp.command_diff.changed {
        return RootCauseReport {
            primary_cause: PrimaryCause::InputChange,
            explanation: "The invoked command string changed between runs.".to_string(),
            confidence_score: confidence_for(PrimaryCause::InputChange),
        };
    }

    if cmp.exit_code_diff.changed || cmp.stdout_diff.changed || cmp.stderr_diff.changed {
        return RootCauseReport {
            primary_cause: PrimaryCause::RuntimeChange,
            explanation: "Subprocess exit code or captured stdout/stderr differ while \
                            environment hash and argv string matched."
                .to_string(),
            confidence_score: confidence_for(PrimaryCause::RuntimeChange),
        };
    }

    RootCauseReport {
        primary_cause: PrimaryCause::SystemNoise,
        explanation: "Only timing or metadata-level differences detected.".to_string(),
        confidence_score: confidence_for(PrimaryCause::SystemNoise),
    }
}

pub fn format_root_cause_human(report: &RootCauseReport) -> String {
    let mut s = String::new();
    s.push_str("── CI root cause ──\n");
    s.push_str(&format!(
        "primary_cause: {}\n",
        report.primary_cause.as_str()
    ));
    s.push_str("explanation:\n");
    for line in report.explanation.lines() {
        s.push_str("  ");
        s.push_str(line);
        s.push('\n');
    }
    if !report.explanation.ends_with('\n') && !report.explanation.is_empty() {
        s.push_str("  <no trailing newline>\n");
    }
    s.push_str(&format!("confidence: {:.2}\n", report.confidence_score));
    s
}
