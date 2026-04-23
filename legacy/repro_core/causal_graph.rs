//! Deterministic causal projection of an [`crate::core::execution_trace::ExecutionTrace`].
//! Derived view only — not persisted.

use crate::core::execution_trace::{ExecutionEvent, ExecutionTrace};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CausalNode {
    pub id: String,
    pub event_type: String,
    /// Stable payload fingerprint for divergence (empty when not applicable).
    pub surface: String,
    pub index: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CausalEdge {
    pub from: String,
    pub to: String,
    pub relation: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CausalGraph {
    pub nodes: Vec<CausalNode>,
    pub edges: Vec<CausalEdge>,
}

fn fnv1a64(bytes: &[u8]) -> u64 {
    const OFFSET: u64 = 0xcbf29ce484222325;
    const PRIME: u64 = 1099511628211;
    let mut h = OFFSET;
    for &b in bytes {
        h ^= b as u64;
        h = h.wrapping_mul(PRIME);
    }
    h
}

fn event_surface(ev: &ExecutionEvent) -> String {
    match ev {
        ExecutionEvent::Spawn { command } => format!("{:016x}", fnv1a64(command.as_bytes())),
        ExecutionEvent::EnvResolved { keys } => {
            let mut s = String::new();
            for k in keys {
                s.push_str(k);
                s.push('\n');
            }
            format!("{:016x}", fnv1a64(s.as_bytes()))
        }
        ExecutionEvent::Stdout { chunk } | ExecutionEvent::Stderr { chunk } => {
            format!("{:016x}", fnv1a64(chunk.as_bytes()))
        }
        ExecutionEvent::Exit { code } => format!("exit:{code}"),
        ExecutionEvent::Timing { duration_ms } => format!("ms:{duration_ms}"),
    }
}

fn event_type_name(ev: &ExecutionEvent) -> &'static str {
    match ev {
        ExecutionEvent::Spawn { .. } => "Spawn",
        ExecutionEvent::EnvResolved { .. } => "EnvResolved",
        ExecutionEvent::Stdout { .. } => "Stdout",
        ExecutionEvent::Stderr { .. } => "Stderr",
        ExecutionEvent::Exit { .. } => "Exit",
        ExecutionEvent::Timing { .. } => "Timing",
    }
}

fn edge_relation(from: &ExecutionEvent, to: &ExecutionEvent) -> &'static str {
    match (from, to) {
        (ExecutionEvent::EnvResolved { .. }, ExecutionEvent::Spawn { .. }) => "enables",
        (ExecutionEvent::Spawn { .. }, ExecutionEvent::Stdout { .. })
        | (ExecutionEvent::Spawn { .. }, ExecutionEvent::Stderr { .. }) => "produces",
        (ExecutionEvent::Exit { .. }, _) => "terminates",
        _ => "next",
    }
}

/// Linear causal projection: one node per trace event, one labeled edge per consecutive pair.
pub fn build_causal_graph(trace: &ExecutionTrace) -> CausalGraph {
    let mut nodes = Vec::with_capacity(trace.events.len());
    for (index, ev) in trace.events.iter().enumerate() {
        nodes.push(CausalNode {
            id: format!("n{index}"),
            event_type: event_type_name(ev).to_string(),
            surface: event_surface(ev),
            index,
        });
    }

    let mut edges = Vec::with_capacity(trace.events.len().saturating_sub(1));
    for i in 0..trace.events.len().saturating_sub(1) {
        let from_ev = &trace.events[i];
        let to_ev = &trace.events[i + 1];
        edges.push(CausalEdge {
            from: format!("n{i}"),
            to: format!("n{}", i + 1),
            relation: edge_relation(from_ev, to_ev).to_string(),
        });
    }

    CausalGraph { nodes, edges }
}

pub(crate) fn outgoing_from_node_index(g: &CausalGraph, node_index: usize) -> Option<&CausalEdge> {
    let id = format!("n{node_index}");
    g.edges.iter().find(|e| e.from == id)
}

/// Stable, human-readable graph text (CLI + tests).
pub fn format_causal_graph_text(g: &CausalGraph) -> String {
    let mut s = String::new();
    s.push_str("CAUSAL GRAPH\n");
    s.push_str(&format!("Nodes: {}\n", g.nodes.len()));
    s.push_str(&format!("Edges: {}\n\n", g.edges.len()));
    for node in &g.nodes {
        s.push_str(&format!("{} ({})\n", node.id, node.event_type));
        if let Some(e) = g.edges.iter().find(|edge| edge.from == node.id) {
            let to_label = g
                .nodes
                .iter()
                .find(|n| n.id == e.to)
                .map(|n| format!("{} ({})", n.id, n.event_type))
                .unwrap_or_else(|| e.to.clone());
            s.push_str(&format!("  -> {} [{}]\n", to_label, e.relation));
        }
    }
    s
}

/// Maturity hook: 0..=3 from node coverage, edge coverage, and rebuild determinism.
pub fn causal_graph_completeness_score() -> u8 {
    let trace = ExecutionTrace {
        run_id: "eval".into(),
        events: vec![
            ExecutionEvent::Spawn {
                command: "x".into(),
            },
            ExecutionEvent::EnvResolved {
                keys: vec!["PATH".into()],
            },
            ExecutionEvent::Stdout {
                chunk: String::new(),
            },
            ExecutionEvent::Stderr {
                chunk: String::new(),
            },
            ExecutionEvent::Exit { code: 0 },
            ExecutionEvent::Timing { duration_ms: 0 },
        ],
    };
    let g1 = build_causal_graph(&trace);
    let g2 = build_causal_graph(&trace);

    let mut score: u8 = 0;
    if g1.nodes.len() == trace.events.len() {
        score = score.saturating_add(1);
    }
    if trace.events.len().saturating_sub(1) == g1.edges.len() {
        score = score.saturating_add(1);
    }
    if g1 == g2 {
        score = score.saturating_add(1);
    }
    score.min(3)
}
