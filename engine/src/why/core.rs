//! Assemble [`WhyReport`](aion_core::WhyReport) from drift + correlation.

use super::correlate::env_stdout_linked;
use super::diff::drift_runs;
use super::rank::{rank, Hypothesis};
use aion_core::WhyReport;

pub fn explain_pair(left_json: &str, right_json: &str) -> Result<WhyReport, String> {
    let drift = drift_runs(left_json, right_json)?;
    let first = drift.fields.first().cloned();
    let mut hy = Vec::new();
    if !drift.changed {
        return Ok(WhyReport {
            summary: "No divergence: artifacts match on compared fields.".into(),
            first_divergent_field: None,
            suggestion: None,
        });
    }
    if env_stdout_linked(&drift) {
        hy.push(Hypothesis {
            score: 10,
            text: "Environment and stdout moved together; treat as env-sensitive.".into(),
        });
    }
    if drift.fields.iter().any(|f| f == "stdout") {
        hy.push(Hypothesis {
            score: 8,
            text: "Stdout diverged; inspect inputs and nondeterminism sources.".into(),
        });
    }
    if drift.fields.iter().any(|f| f == "exit_code") {
        hy.push(Hypothesis {
            score: 6,
            text: "Exit code diverged; compare stderr and duration.".into(),
        });
    }
    hy.push(Hypothesis {
        score: 1,
        text: "Runs diverged on one or more compared fields.".into(),
    });
    let ranked = rank(hy);
    let summary = ranked
        .first()
        .map(|h| h.text.clone())
        .unwrap_or_else(|| "Divergence detected.".into());
    let suggestion = if env_stdout_linked(&drift) {
        Some("Pin environment or capture richer env metadata.".into())
    } else if drift.fields.iter().any(|f| f == "stdout") {
        Some("Inspect command inputs and nondeterministic sources.".into())
    } else {
        None
    };
    Ok(WhyReport {
        summary,
        first_divergent_field: first,
        suggestion,
    })
}
