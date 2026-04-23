//! CI orchestration over the existing capture + causal stack (no new execution model).

use std::path::Path;

use crate::ci::baseline;
use crate::ci::failure_detector::{self, FailureType};
use crate::ci::storage;
use crate::core::artifact::ExecutionArtifact;
use crate::core::causal_graph::build_causal_graph;
use crate::core::causal_query::first_divergent_causal_node;
use crate::core::diff::diff_runs;
use crate::core::report::CauseCategory;
use crate::core::root_cause::{
    cause_category_ci_upper, classify_root_cause, generate_details, generate_summary,
};

/// Horizontal rule for CI human output (50 ASCII hyphens + newline).
pub const CI_RULE: &str = "--------------------------------------------------\n";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CiResult {
    pub success: bool,
    pub run_id: String,
    pub baseline_run_id: Option<String>,
    pub category: Option<String>,
    pub node: Option<String>,
    pub summary: Option<String>,
    pub details: Vec<String>,
}

fn focal_node_id(graph: &crate::core::causal_graph::CausalGraph) -> Option<String> {
    if graph.nodes.is_empty() {
        return None;
    }
    for n in &graph.nodes {
        if n.event_type == "Stdout" {
            return Some(n.id.clone());
        }
    }
    for n in &graph.nodes {
        if n.event_type == "Stderr" {
            return Some(n.id.clone());
        }
    }
    let idx = graph.nodes.len().saturating_sub(2);
    Some(graph.nodes[idx.min(graph.nodes.len() - 1)].id.clone())
}

fn failure_category(ft: FailureType) -> CauseCategory {
    match ft {
        FailureType::EnvironmentFailure => CauseCategory::EnvironmentChange,
        FailureType::RuntimeFailure
        | FailureType::AssertionFailure
        | FailureType::GenericFailure => CauseCategory::RuntimeChange,
    }
}

fn details_without_baseline(artifact: &ExecutionArtifact, ft: FailureType) -> Vec<String> {
    let mut out = Vec::new();
    match ft {
        FailureType::EnvironmentFailure => {
            out.push("environment or path resolution failed".into());
        }
        FailureType::RuntimeFailure => {
            if artifact.stderr.contains("spawn error") {
                out.push("process spawn failed".into());
            } else if artifact.exit_code != 0 {
                out.push(format!("exit code {}", artifact.exit_code));
            }
        }
        FailureType::AssertionFailure => {
            if !artifact.stderr.is_empty() {
                out.push("stderr output present".into());
            } else if artifact.exit_code != 0 {
                out.push(format!("exit code {}", artifact.exit_code));
            }
        }
        FailureType::GenericFailure => {
            if artifact.exit_code != 0 {
                out.push(format!("exit code {}", artifact.exit_code));
            }
        }
    }
    if out.is_empty() && !artifact.stderr.is_empty() {
        out.push("stderr output present".into());
    }
    if out.is_empty() {
        out.push("run did not complete successfully".into());
    }
    out.truncate(3);
    out
}

/// Run classification + optional causal bundle **before** persisting the new row.
pub fn process_ci_run(root: &Path, artifact: &ExecutionArtifact) -> CiResult {
    let run_id = artifact.run_id.clone();
    if !failure_detector::is_failure(artifact) {
        return CiResult {
            success: true,
            run_id,
            baseline_run_id: None,
            category: None,
            node: None,
            summary: None,
            details: Vec::new(),
        };
    }

    let candidates = storage::list_ci_runs_in(root);
    let baseline = baseline::select_baseline_run(artifact, &candidates);
    let baseline_run_id = baseline.as_ref().map(|b| b.run_id.clone());

    let gc = build_causal_graph(&artifact.trace);
    let focal = focal_node_id(&gc).unwrap_or_else(|| "n0".to_string());

    let ft = failure_detector::classify_failure(artifact);

    let (category, summary, details, node) = if let Some(ref b) = baseline {
        let diff = diff_runs(b, artifact);
        let classified = classify_root_cause(&diff);
        let cat = if matches!(classified.primary, CauseCategory::SystemNoise) {
            CauseCategory::RuntimeChange
        } else {
            classified.primary
        };
        let gb = build_causal_graph(&b.trace);
        let node = first_divergent_causal_node(&gb, &gc)
            .map(|n| n.id.clone())
            .unwrap_or_else(|| focal.clone());
        let mut details = generate_details(&diff);
        if details.is_empty() {
            details = details_without_baseline(artifact, ft);
        }
        (
            cause_category_ci_upper(cat).to_string(),
            generate_summary(cat),
            details,
            node,
        )
    } else {
        let cat = failure_category(ft);
        (
            cause_category_ci_upper(cat).to_string(),
            generate_summary(cat),
            details_without_baseline(artifact, ft),
            focal,
        )
    };

    CiResult {
        success: false,
        run_id,
        baseline_run_id,
        category: Some(category),
        node: Some(node),
        summary: Some(summary),
        details,
    }
}

pub fn format_ci_result_text(r: &CiResult) -> String {
    let mut s = String::new();
    s.push_str(CI_RULE);
    if r.success {
        s.push_str("CI RUN SUCCESS\n");
        s.push_str(&format!("{:<12}{}\n", "Run:", r.run_id));
        s.push_str(CI_RULE);
        return s;
    }

    s.push_str("CI RUN FAILED\n");
    let compared = r
        .baseline_run_id
        .as_deref()
        .filter(|x| !x.is_empty())
        .unwrap_or("none");
    s.push_str(&format!("{:<12}{}\n", "Run:", r.run_id));
    s.push_str(&format!("{:<12}{}\n", "Compared:", compared));
    s.push('\n');
    let cat = r.category.as_deref().unwrap_or("RUNTIME");
    s.push_str(&format!("{:<12}{}\n", "Root Cause:", cat));
    let node = r.node.as_deref().unwrap_or("n0");
    s.push_str(&format!("{:<12}{}\n", "Node:", node));
    s.push('\n');
    s.push_str("Summary:\n");
    s.push_str(r.summary.as_deref().unwrap_or(""));
    s.push('\n');
    s.push('\n');
    s.push_str("Details:\n");
    for line in &r.details {
        s.push_str("- ");
        s.push_str(line);
        s.push('\n');
    }
    s.push('\n');
    s.push_str("Hint:\n");
    s.push_str("repro ci why ");
    s.push_str(&r.run_id);
    s.push('\n');
    s.push_str(CI_RULE);
    s
}

/// Stable JSON value for `repro ci run --json` (fixed key order via `serde_json::json!`).
pub fn ci_run_json_value(r: &CiResult) -> serde_json::Value {
    let status = if r.success { "success" } else { "failed" };
    let baseline = r
        .baseline_run_id
        .as_ref()
        .map(|s| serde_json::Value::String(s.clone()))
        .unwrap_or(serde_json::Value::Null);

    if r.success {
        return serde_json::json!({
            "status": status,
            "run_id": r.run_id,
            "baseline_run_id": baseline,
            "root_cause": serde_json::Value::Null,
        });
    }

    let cat = r.category.clone().unwrap_or_else(|| "RUNTIME".into());
    let node = r.node.clone().unwrap_or_else(|| "n0".into());
    let summary = r.summary.clone().unwrap_or_default();
    let details: Vec<serde_json::Value> = r
        .details
        .iter()
        .map(|d| serde_json::Value::String(d.clone()))
        .collect();

    serde_json::json!({
        "status": status,
        "run_id": r.run_id,
        "baseline_run_id": baseline,
        "root_cause": {
            "category": cat,
            "node": node,
            "summary": summary,
            "details": details,
        },
    })
}
