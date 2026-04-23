//! Why engine v2 — structured causal explanations (prompt / seed / determinism / tokens).

use super::model::AICapsuleV1;
use aion_core::DeterminismProfile;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "snake_case")]
pub enum WhyNodeKind {
    Prompt,
    Token,
    Seed,
    Determinism,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct WhyNode {
    pub id: String,
    pub kind: WhyNodeKind,
    pub text: String,
    pub position: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WhyEdge {
    pub from: String,
    pub to: String,
    pub weight: f32,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WhyReportV2 {
    pub why_schema_version: String,
    pub model_version: String,
    pub seed: u64,
    pub determinism_profile: String,
    pub nodes: Vec<WhyNode>,
    pub edges: Vec<WhyEdge>,
    pub summary: String,
}

fn det_text(model: &str, det: &DeterminismProfile) -> String {
    format!(
        "Determinism: model={model}, time_frozen={}, time_epoch_secs={}, random_seed=0x{:x}, syscall_intercept={}",
        det.time_frozen, det.time_epoch_secs, det.random_seed, det.syscall_intercept
    )
}

/// Build structured why graph from capsule fields (deterministic ordering).
pub fn build_why_report_v2(
    model: &str,
    prompt: &str,
    seed: u64,
    det: &DeterminismProfile,
    tokens: &[String],
) -> WhyReportV2 {
    let mut nodes = Vec::new();
    let segs: Vec<&str> = prompt.split_whitespace().collect();
    if segs.is_empty() {
        nodes.push(WhyNode {
            id: "prompt_0".into(),
            kind: WhyNodeKind::Prompt,
            text: "(empty prompt)".into(),
            position: Some(0),
        });
    } else {
        for (i, s) in segs.iter().enumerate() {
            nodes.push(WhyNode {
                id: format!("prompt_{i}"),
                kind: WhyNodeKind::Prompt,
                text: (*s).to_string(),
                position: Some(i),
            });
        }
    }
    nodes.push(WhyNode {
        id: "seed".into(),
        kind: WhyNodeKind::Seed,
        text: format!("Global RNG / sampling seed: 0x{seed:x}"),
        position: None,
    });
    nodes.push(WhyNode {
        id: "determinism".into(),
        kind: WhyNodeKind::Determinism,
        text: det_text(model, det),
        position: None,
    });
    for (i, t) in tokens.iter().enumerate() {
        nodes.push(WhyNode {
            id: format!("token_{i}"),
            kind: WhyNodeKind::Token,
            text: t.clone(),
            position: Some(i),
        });
    }
    nodes.sort();

    let mut edges = Vec::new();
    let prompt_ids: Vec<String> = if segs.is_empty() {
        vec!["prompt_0".into()]
    } else {
        (0..segs.len()).map(|i| format!("prompt_{i}")).collect()
    };
    let mut w = 1.0f32;
    if !tokens.is_empty() {
        for pid in &prompt_ids {
            edges.push(WhyEdge {
                from: pid.clone(),
                to: "token_0".into(),
                weight: w,
                reason: "Prompt segment conditions the first emitted token".into(),
            });
            w -= 0.01;
        }
        edges.push(WhyEdge {
            from: "seed".into(),
            to: "token_0".into(),
            weight: 0.95,
            reason: "Seed selects vocabulary draws for each index".into(),
        });
        edges.push(WhyEdge {
            from: "determinism".into(),
            to: "token_0".into(),
            weight: 0.9,
            reason: "Determinism profile pins time and RNG policy for the run".into(),
        });
        for i in 0..tokens.len().saturating_sub(1) {
            edges.push(WhyEdge {
                from: format!("token_{i}"),
                to: format!("token_{}", i + 1),
                weight: 0.85,
                reason: "Prior token state feeds the next token in sequence".into(),
            });
        }
    }
    edges.sort_by(|a, b| {
        (&a.from, &a.to, &a.reason).cmp(&(&b.from, &b.to, &b.reason))
    });

    let summary = format!(
        "{} influence nodes, {} causal edges, {} emitted tokens; deterministic under the given model, prompt, seed, and determinism profile.",
        nodes.len(),
        edges.len(),
        tokens.len()
    );

    WhyReportV2 {
        why_schema_version: "2".into(),
        model_version: model.to_string(),
        seed,
        determinism_profile: det_text(model, det),
        nodes,
        edges,
        summary,
    }
}

/// Structured explanation for a capsule (matches [`super::graph::ai_causal_graph_v2`] topology).
pub fn why_ai_run_v2(capsule: &AICapsuleV1) -> WhyReportV2 {
    build_why_report_v2(
        &capsule.model,
        &capsule.prompt,
        capsule.seed,
        &capsule.determinism,
        &capsule.tokens,
    )
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WhyDiff {
    pub changed: bool,
    pub node_diffs: Vec<String>,
    pub edge_diffs: Vec<String>,
}

fn edge_sig(e: &WhyEdge) -> String {
    format!("{}|{}|{:e}|{}", e.from, e.to, e.weight, e.reason)
}

/// Deterministic diff between two Why v2 reports (for replay).
pub fn why_diff(original: &WhyReportV2, replay: &WhyReportV2) -> WhyDiff {
    let mut node_diffs = Vec::new();
    let om: BTreeMap<&str, &WhyNode> = original.nodes.iter().map(|n| (n.id.as_str(), n)).collect();
    let rm: BTreeMap<&str, &WhyNode> = replay.nodes.iter().map(|n| (n.id.as_str(), n)).collect();
    let all_ids: BTreeSet<&str> = om.keys().chain(rm.keys()).copied().collect();
    for id in &all_ids {
        match (om.get(id), rm.get(id)) {
            (None, Some(_)) => node_diffs.push(format!("node {id}: only in replay")),
            (Some(_), None) => node_diffs.push(format!("node {id}: only in original")),
            (Some(a), Some(b)) if a != b => {
                node_diffs.push(format!("node {id}: text/kind/position mismatch"))
            }
            _ => {}
        }
    }

    let mut edge_diffs = Vec::new();
    let oe: BTreeSet<String> = original.edges.iter().map(edge_sig).collect();
    let re: BTreeSet<String> = replay.edges.iter().map(edge_sig).collect();
    for e in oe.difference(&re) {
        edge_diffs.push(format!("edge only in original: {e}"));
    }
    for e in re.difference(&oe) {
        edge_diffs.push(format!("edge only in replay: {e}"));
    }

    node_diffs.sort();
    edge_diffs.sort();
    let changed = !node_diffs.is_empty() || !edge_diffs.is_empty();
    WhyDiff {
        changed,
        node_diffs,
        edge_diffs,
    }
}
