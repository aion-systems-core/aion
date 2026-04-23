//! Ordered field diff between two [`RunResult`] values.

use aion_core::error::{canonical_error_json, code, line};
use aion_core::{DriftReport, DriftToleranceProfile, DriftToleranceViolation, RunResult};

pub fn diff_run_snapshots(a: &RunResult, b: &RunResult) -> DriftReport {
    let mut fields = Vec::new();
    let mut labels = Vec::new();
    let profile = DriftToleranceProfile::deterministic_default();
    let mut violations = Vec::new();
    if a.stdout != b.stdout {
        fields.push("stdout".into());
        labels.push("other:stdout_mismatch".into());
    }
    if a.stderr != b.stderr {
        fields.push("stderr".into());
        labels.push("other:stderr_mismatch".into());
    }
    if a.exit_code != b.exit_code {
        fields.push("exit_code".into());
        labels.push("shape:exit_code_mismatch".into());
    }
    if a.command != b.command {
        fields.push("command".into());
        labels.push("shape:command_mismatch".into());
    }
    if a.env_fingerprint != b.env_fingerprint {
        fields.push("env_fingerprint".into());
        labels.push("model:env_fingerprint_mismatch".into());
    }
    if a.cwd != b.cwd {
        fields.push("cwd".into());
        labels.push("shape:cwd_mismatch".into());
    }
    let token_delta = a
        .stdout
        .split_whitespace()
        .count()
        .abs_diff(b.stdout.split_whitespace().count()) as u64;
    if token_delta > profile.max_token_delta as u64 {
        violations.push(DriftToleranceViolation {
            label: "tokens:stdout_delta_over_limit".into(),
            actual: token_delta,
            limit: profile.max_token_delta as u64,
        });
    }
    fields.sort();
    fields.dedup();
    labels.sort();
    labels.dedup();
    let categories = categories_from_labels(&labels);
    let changed = !fields.is_empty() || !violations.is_empty();
    let error = if !violations.is_empty() {
        Some(canonical_error_json(
            &line(code::DRIFT_TOLERANCE, "diff_run_snapshots", "tolerance_violation"),
            "drift",
        ))
    } else {
        None
    };
    DriftReport {
        changed,
        categories,
        labels: labels.clone(),
        fields,
        details: labels,
        tolerance_profile: profile,
        tolerance_violations: violations,
        overflow: false,
        error,
    }
}

pub fn diff_runs(left_json: &str, right_json: &str) -> Result<DriftReport, String> {
    let a: RunResult = serde_json::from_str(left_json)
        .map_err(|_| line(code::DRIFT_JSON, "diff_runs_left", "invalid_json"))?;
    let b: RunResult = serde_json::from_str(right_json)
        .map_err(|_| line(code::DRIFT_JSON, "diff_runs_right", "invalid_json"))?;
    Ok(diff_run_snapshots(&a, &b))
}

fn categories_from_labels(labels: &[String]) -> Vec<String> {
    let ordered = ["shape", "tokens", "timing", "model", "evidence", "other"];
    let mut out = Vec::new();
    for cat in ordered {
        if labels.iter().any(|l| l.starts_with(&format!("{cat}:"))) {
            out.push(cat.to_string());
        }
    }
    out
}
