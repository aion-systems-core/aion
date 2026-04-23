// Semantic root cause.
//
// Divergences map to four `CauseCategory` buckets with fixed priority:
// environment → input → runtime → system noise.

use crate::core::artifact::ExecutionArtifact;
use crate::core::diff::{diff_runs, DiffReport};
use crate::core::execution_trace::{ExecutionEvent, ExecutionTrace};
use crate::core::identity::ExecutionIdentity;
use crate::core::report::{
    CauseCategory, DiffSummary, ExecutionReport, RootCauseSummary, SemanticCause, Severity,
    TraceSummary,
};
use crate::core::storage;
use std::io;

/// Result of `classify_root_cause`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassifiedRootCause {
    pub primary: CauseCategory,
    pub explanation: String,
    pub affected_fields: Vec<String>,
}

/// Event-timeline hint (complements field-based [`classify_root_cause`]).
pub fn find_trace_root_cause(trace: &ExecutionTrace) -> Option<String> {
    for event in &trace.events {
        match event {
            ExecutionEvent::EnvResolved { .. } => {
                return Some("environment_change".to_string());
            }
            ExecutionEvent::Spawn { .. } => continue,
            ExecutionEvent::Stdout { .. } => continue,
            ExecutionEvent::Stderr { .. } => continue,
            ExecutionEvent::Exit { code } if *code != 0 => {
                return Some("runtime_failure".to_string());
            }
            _ => {}
        }
    }
    None
}

/// Classify the diff into a single primary `CauseCategory` plus fields.
pub fn classify_root_cause(diff: &DiffReport) -> ClassifiedRootCause {
    let mut env_f: Vec<String> = Vec::new();
    let mut inp_f: Vec<String> = Vec::new();
    let mut run_f: Vec<String> = Vec::new();
    let mut noise_f: Vec<String> = Vec::new();

    for d in &diff.differences {
        match d.field.as_str() {
            "environment_hash" | "cwd" => env_f.push(d.field.clone()),
            "command" => inp_f.push(d.field.clone()),
            "exit_code" | "stdout" | "stderr" => run_f.push(d.field.clone()),
            "duration_ms" | "timestamp" => noise_f.push(d.field.clone()),
            other => run_f.push(other.to_string()),
        }
    }

    if !env_f.is_empty() {
        return ClassifiedRootCause {
            primary: CauseCategory::EnvironmentChange,
            explanation: "Environment or working directory changed between runs; align \
                          the 15% env slice (PATH, HOME, CI, SHELL, LANG) and cwd."
                .to_string(),
            affected_fields: env_f,
        };
    }
    if !inp_f.is_empty() {
        return ClassifiedRootCause {
            primary: CauseCategory::InputChange,
            explanation: "The command invocation (intent) changed between runs.".to_string(),
            affected_fields: inp_f,
        };
    }
    if !run_f.is_empty() {
        return ClassifiedRootCause {
            primary: CauseCategory::RuntimeChange,
            explanation: "Subprocess outcome or I/O differed between runs (exit code, \
                          stdout/stderr)."
                .to_string(),
            affected_fields: run_f,
        };
    }
    if !noise_f.is_empty() {
        return ClassifiedRootCause {
            primary: CauseCategory::SystemNoise,
            explanation: "Only timing or capture metadata fields differ.".to_string(),
            affected_fields: noise_f,
        };
    }

    let meta: Vec<String> = diff.differences.iter().map(|d| d.field.clone()).collect();
    ClassifiedRootCause {
        primary: CauseCategory::SystemNoise,
        explanation: "No semantic bucket matched; treating as low-signal.".to_string(),
        affected_fields: meta,
    }
}

/// Uppercase CI / contract label for [`CauseCategory`].
#[must_use]
pub fn cause_category_ci_upper(cat: CauseCategory) -> &'static str {
    match cat {
        CauseCategory::EnvironmentChange => "ENVIRONMENT",
        CauseCategory::InputChange => "INPUT",
        CauseCategory::RuntimeChange => "RUNTIME",
        CauseCategory::SystemNoise => "SYSTEM",
    }
}

/// Single deterministic sentence per category (no timestamps or captured values).
#[must_use]
pub fn generate_summary(category: CauseCategory) -> String {
    match category {
        CauseCategory::EnvironmentChange => {
            "environment changed → execution context differs → output diverged".into()
        }
        CauseCategory::InputChange => {
            "input changed → program behavior differs → output diverged".into()
        }
        CauseCategory::RuntimeChange => {
            "execution failed → process returned non-zero exit code".into()
        }
        CauseCategory::SystemNoise => "non-deterministic system effect detected".into(),
    }
}

/// Up to three deterministic bullets from a semantic diff (priority: env, command, streams, exit).
#[must_use]
pub fn generate_details(diff: &DiffReport) -> Vec<String> {
    let d = |f: &str| diff.differences.iter().find(|e| e.field == f);
    let mut out = Vec::new();

    if d("environment_hash").is_some() || d("cwd").is_some() {
        let mut parts = Vec::new();
        if d("environment_hash").is_some() {
            parts.push("environment fingerprint");
        }
        if d("cwd").is_some() {
            parts.push("cwd");
        }
        if !parts.is_empty() {
            out.push(format!("{} changed", parts.join(" and ")));
        }
    }
    if out.len() < 3 && d("command").is_some() {
        out.push("command differs".into());
    }
    if out.len() < 3 && d("stdout").is_some() {
        out.push("stdout differs".into());
    }
    if out.len() < 3 && d("stderr").is_some() {
        out.push("stderr differs".into());
    }
    if out.len() < 3 {
        if let Some(e) = d("exit_code") {
            out.push(format!("exit code {} → {}", e.a, e.b));
        }
    }

    out.truncate(3);
    out
}

fn classified_to_semantic(
    classified: &ClassifiedRootCause,
    diff: &DiffReport,
) -> Option<SemanticCause> {
    let field = classified.affected_fields.first()?.clone();
    let line = diff.differences.iter().find(|d| d.field == field)?;
    Some(SemanticCause {
        category: classified.primary,
        field: field.clone(),
        severity: Severity::High,
        previous_value: line.a.clone(),
        current_value: line.b.clone(),
        explanation: classified.explanation.clone(),
    })
}

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RootCause {
    NoPrevious,
    Identical {
        previous: String,
    },
    Divergence {
        previous: String,
        field: String,
        previous_value: String,
        current_value: String,
    },
}

pub fn build_root_cause_report(run_id_or_alias: &str) -> io::Result<ExecutionReport> {
    let run_id = storage::resolve_alias(run_id_or_alias)?;
    let runs = storage::list_runs()?;
    let idx = runs
        .iter()
        .position(|r| r == &run_id)
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "run not in index"))?;

    let current = storage::load_run(&run_id)?;
    let current_identity = ExecutionIdentity::from_artifact(&current);
    let trace = TraceSummary::from_artifact(&current);

    if idx == 0 {
        return Ok(ExecutionReport {
            identity: current_identity,
            trace,
            diff: None,
            root_cause: Some(RootCauseSummary {
                previous_run: None,
                primary: None,
                causes: Vec::new(),
                first_diverging_field: None,
                explanation: "No previous run exists. There is nothing to \
                             compare against; record another run to get a \
                             root cause."
                    .to_string(),
            }),
        });
    }

    let prev_id = runs[idx - 1].clone();
    let previous = storage::load_run(&prev_id)?;
    let previous_identity = ExecutionIdentity::from_artifact(&previous);

    let diff_report = diff_runs(&previous, &current);
    let delta = previous_identity.delta(&current_identity);

    let root_cause_summary = summarize(&previous, &current, &diff_report);

    Ok(ExecutionReport {
        identity: current_identity,
        trace,
        diff: Some(DiffSummary::from_report(&diff_report, delta)),
        root_cause: Some(root_cause_summary),
    })
}

#[allow(dead_code)]
pub fn find_root_cause(run_id_or_alias: &str) -> io::Result<RootCause> {
    let report = build_root_cause_report(run_id_or_alias)?;
    Ok(match report.root_cause {
        None => RootCause::NoPrevious,
        Some(rc) => match (rc.previous_run, rc.first_diverging_field) {
            (None, _) => RootCause::NoPrevious,
            (Some(prev), None) => RootCause::Identical { previous: prev },
            (Some(prev), Some(field)) => {
                let (pv, cv) = rc
                    .causes
                    .iter()
                    .find(|c| c.field == field)
                    .map(|c| (c.previous_value.clone(), c.current_value.clone()))
                    .unwrap_or_default();
                RootCause::Divergence {
                    previous: prev,
                    field,
                    previous_value: pv,
                    current_value: cv,
                }
            }
        },
    })
}

fn summarize(
    previous: &ExecutionArtifact,
    _current: &ExecutionArtifact,
    diff_report: &DiffReport,
) -> RootCauseSummary {
    use crate::core::report::classify;

    let causes: Vec<SemanticCause> = diff_report.differences.iter().map(classify).collect();

    if causes.is_empty() {
        return RootCauseSummary {
            previous_run: Some(previous.run_id.clone()),
            primary: None,
            causes: Vec::new(),
            first_diverging_field: None,
            explanation: format!(
                "No divergence from previous run {}. The two artifacts are \
                 semantically identical.",
                previous.run_id
            ),
        };
    }

    let classified = classify_root_cause(diff_report);
    let primary = classified_to_semantic(&classified, diff_report);
    let first_field = diff_report.differences.first().map(|d| d.field.clone());

    let explanation = format!(
        "{} :: {}",
        classified.primary.as_str(),
        classified.explanation
    );

    RootCauseSummary {
        previous_run: Some(previous.run_id.clone()),
        primary,
        causes: causes.clone(),
        first_diverging_field: first_field,
        explanation,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::capture::{
        capture_command_with_clock, reset_counter_for_tests, FixedClock, CAPTURE_TEST_LOCK,
    };
    use crate::core::diff::diff_runs;
    use crate::core::execution_trace::{ExecutionEvent, ExecutionTrace};

    #[test]
    fn find_trace_root_cause_runtime_failure_on_nonzero_exit() {
        let t = ExecutionTrace {
            run_id: "x".into(),
            events: vec![
                ExecutionEvent::Spawn {
                    command: "false".into(),
                },
                ExecutionEvent::Exit { code: 1 },
            ],
        };
        assert_eq!(
            find_trace_root_cause(&t).as_deref(),
            Some("runtime_failure")
        );
    }

    #[test]
    fn find_trace_root_cause_env_resolved_branch() {
        let t = ExecutionTrace {
            run_id: "x".into(),
            events: vec![
                ExecutionEvent::Spawn {
                    command: "x".into(),
                },
                ExecutionEvent::EnvResolved {
                    keys: vec!["PATH=/z".into()],
                },
                ExecutionEvent::Exit { code: 1 },
            ],
        };
        assert_eq!(
            find_trace_root_cause(&t).as_deref(),
            Some("environment_change")
        );
    }

    #[test]
    fn command_and_stdout_diff_classifies_as_input_first() {
        let _g = CAPTURE_TEST_LOCK.lock().unwrap();
        reset_counter_for_tests();
        let a = capture_command_with_clock("echo a".into(), &FixedClock(1));
        let b = capture_command_with_clock("echo b".into(), &FixedClock(1));
        let diff = diff_runs(&a, &b);
        let c = classify_root_cause(&diff);
        assert_eq!(c.primary, CauseCategory::InputChange);
        assert!(c.affected_fields.contains(&"command".to_string()));
    }
}
