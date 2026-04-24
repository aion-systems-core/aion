//! Causal DAG v2 (structured, deterministic, aligned with Why v2).

use super::model::AICapsuleV1;
use aion_core::DeterminismProfile;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "snake_case")]
pub enum GraphNodeKind {
    Prompt,
    Token,
    Seed,
    Determinism,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct GraphNode {
    pub id: String,
    pub label: String,
    pub kind: GraphNodeKind,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct GraphEdge {
    pub from: String,
    pub to: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CausalGraphV2 {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

fn det_label(model: &str, det: &DeterminismProfile) -> String {
    format!(
        "model={model}; time_frozen={}; epoch={}; random_seed=0x{:x}; syscall_intercept={}",
        det.time_frozen, det.time_epoch_secs, det.random_seed, det.syscall_intercept
    )
}

/// Build DAG from capsule fields (deterministic node / edge ordering).
pub fn build_causal_graph_v2(
    model: &str,
    prompt: &str,
    seed: u64,
    det: &DeterminismProfile,
    tokens: &[String],
) -> CausalGraphV2 {
    let mut nodes = Vec::new();
    let segs: Vec<&str> = prompt.split_whitespace().collect();
    if segs.is_empty() {
        nodes.push(GraphNode {
            id: "prompt_0".into(),
            label: "(empty prompt)".into(),
            kind: GraphNodeKind::Prompt,
        });
    } else {
        for (i, s) in segs.iter().enumerate() {
            nodes.push(GraphNode {
                id: format!("prompt_{i}"),
                label: (*s).to_string(),
                kind: GraphNodeKind::Prompt,
            });
        }
    }
    nodes.push(GraphNode {
        id: "seed".into(),
        label: format!("0x{seed:x}"),
        kind: GraphNodeKind::Seed,
    });
    nodes.push(GraphNode {
        id: "determinism".into(),
        label: det_label(model, det),
        kind: GraphNodeKind::Determinism,
    });
    for (i, t) in tokens.iter().enumerate() {
        nodes.push(GraphNode {
            id: format!("token_{i}"),
            label: t.clone(),
            kind: GraphNodeKind::Token,
        });
    }
    nodes.sort();

    let mut edges = Vec::new();
    let prompt_ids: Vec<String> = if segs.is_empty() {
        vec!["prompt_0".into()]
    } else {
        (0..segs.len()).map(|i| format!("prompt_{i}")).collect()
    };
    if !tokens.is_empty() {
        for pid in &prompt_ids {
            edges.push(GraphEdge {
                from: pid.clone(),
                to: "token_0".into(),
            });
        }
        edges.push(GraphEdge {
            from: "seed".into(),
            to: "token_0".into(),
        });
        edges.push(GraphEdge {
            from: "determinism".into(),
            to: "token_0".into(),
        });
        for i in 0..tokens.len().saturating_sub(1) {
            edges.push(GraphEdge {
                from: format!("token_{i}"),
                to: format!("token_{}", i + 1),
            });
        }
    }
    edges.sort();

    CausalGraphV2 { nodes, edges }
}

/// Token-level causal DAG for a capsule (matches Why v2 topology).
pub fn ai_causal_graph_v2(capsule: &AICapsuleV1) -> CausalGraphV2 {
    build_causal_graph_v2(
        &capsule.model,
        &capsule.prompt,
        capsule.seed,
        &capsule.determinism,
        &capsule.tokens,
    )
}

/// Legacy JSON projection (canonicalizable) for drift / tooling.
pub fn ai_causal_graph_json(capsule: &AICapsuleV1) -> serde_json::Value {
    serde_json::to_value(ai_causal_graph_v2(capsule)).unwrap_or(serde_json::Value::Null)
}
