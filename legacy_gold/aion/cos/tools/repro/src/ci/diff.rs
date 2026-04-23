// Semantic comparison between two CI runs (normalized logs + buckets).

use crate::core::artifact::ExecutionArtifact;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SemanticClassification {
    EnvironmentChange,
    InputChange,
    RuntimeChange,
    SystemNoise,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FieldChange {
    pub changed: bool,
    pub summary: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CIComparisonReport {
    pub run_a: String,
    pub run_b: String,
    pub env_diff: FieldChange,
    pub stdout_diff: FieldChange,
    pub stderr_diff: FieldChange,
    pub exit_code_diff: FieldChange,
    pub command_diff: FieldChange,
    pub semantic_classification: Vec<SemanticClassification>,
}

pub fn normalize_log_text(s: &str) -> String {
    s.lines()
        .map(collapse_horizontal_ws)
        .collect::<Vec<_>>()
        .join("\n")
}

fn collapse_horizontal_ws(line: &str) -> String {
    let t = line.trim_end();
    let mut out = String::with_capacity(t.len());
    let mut prev_space = false;
    for c in t.trim_start().chars() {
        if c == ' ' || c == '\t' {
            if !prev_space {
                out.push(' ');
                prev_space = true;
            }
        } else {
            prev_space = false;
            out.push(c);
        }
    }
    out
}

pub fn compare_ci_runs(a: &ExecutionArtifact, b: &ExecutionArtifact) -> CIComparisonReport {
    let ha = a.environment_hash();
    let hb = b.environment_hash();
    let env_diff = FieldChange {
        changed: ha != hb,
        summary: if ha == hb {
            "environment_hash identical".into()
        } else {
            format!("environment_hash differs: {} vs {}", ha, hb)
        },
    };

    let na = normalize_log_text(&a.stdout);
    let nb = normalize_log_text(&b.stdout);
    let stdout_diff = FieldChange {
        changed: na != nb,
        summary: if na == nb {
            "stdout identical after normalization".into()
        } else {
            "stdout differs after normalization".into()
        },
    };

    let ea = normalize_log_text(&a.stderr);
    let eb = normalize_log_text(&b.stderr);
    let stderr_diff = FieldChange {
        changed: ea != eb,
        summary: if ea == eb {
            "stderr identical after normalization".into()
        } else {
            "stderr differs after normalization".into()
        },
    };

    let exit_code_diff = FieldChange {
        changed: a.exit_code != b.exit_code,
        summary: format!("exit_code {} vs {}", a.exit_code, b.exit_code),
    };

    let command_diff = FieldChange {
        changed: a.command != b.command,
        summary: if a.command == b.command {
            "command identical".into()
        } else {
            "command invocation differs".into()
        },
    };

    let mut semantic_classification = Vec::new();
    if env_diff.changed {
        semantic_classification.push(SemanticClassification::EnvironmentChange);
    }
    if command_diff.changed {
        semantic_classification.push(SemanticClassification::InputChange);
    }
    let outcome_changed = exit_code_diff.changed || stdout_diff.changed || stderr_diff.changed;
    if outcome_changed {
        semantic_classification.push(SemanticClassification::RuntimeChange);
    }
    if !env_diff.changed
        && !command_diff.changed
        && !exit_code_diff.changed
        && !stdout_diff.changed
        && !stderr_diff.changed
        && (a.duration_ms != b.duration_ms || a.timestamp != b.timestamp)
    {
        semantic_classification.push(SemanticClassification::SystemNoise);
    }

    semantic_classification.sort_by_key(|c| semantic_rank(*c));
    semantic_classification.dedup();

    CIComparisonReport {
        run_a: a.run_id.clone(),
        run_b: b.run_id.clone(),
        env_diff,
        stdout_diff,
        stderr_diff,
        exit_code_diff,
        command_diff,
        semantic_classification,
    }
}

fn semantic_rank(c: SemanticClassification) -> u8 {
    match c {
        SemanticClassification::EnvironmentChange => 0,
        SemanticClassification::InputChange => 1,
        SemanticClassification::RuntimeChange => 2,
        SemanticClassification::SystemNoise => 3,
    }
}
