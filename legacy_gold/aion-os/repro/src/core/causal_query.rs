//! Deterministic causal queries over a [`crate::core::causal_graph::CausalGraph`].
//! Pure reasoning layer — no I/O, no persistence.

use std::collections::VecDeque;

use crate::core::causal_graph::{CausalGraph, CausalNode};

fn node_by_id<'a>(g: &'a CausalGraph, id: &str) -> Option<&'a CausalNode> {
    g.nodes.iter().find(|n| n.id == id)
}

/// All immediate predecessors of `node_id` (incoming edges), index ascending.
pub fn query_causes(graph: &CausalGraph, node_id: &str) -> Vec<CausalNode> {
    let mut v: Vec<CausalNode> = graph
        .edges
        .iter()
        .filter(|e| e.to == node_id)
        .filter_map(|e| node_by_id(graph, &e.from).cloned())
        .collect();
    v.sort_by_key(|n| n.index);
    v
}

/// All immediate successors of `node_id` (outgoing edges), index ascending.
pub fn query_effects(graph: &CausalGraph, node_id: &str) -> Vec<CausalNode> {
    let mut v: Vec<CausalNode> = graph
        .edges
        .iter()
        .filter(|e| e.from == node_id)
        .filter_map(|e| node_by_id(graph, &e.to).cloned())
        .collect();
    v.sort_by_key(|n| n.index);
    v
}

/// BFS from `from` to `to`; first path found wins. Queue is FIFO over child insertion order.
pub fn trace_path(graph: &CausalGraph, from: &str, to: &str) -> Vec<CausalNode> {
    if from == to {
        return node_by_id(graph, from).cloned().into_iter().collect();
    }
    let mut queue: VecDeque<Vec<String>> = VecDeque::new();
    queue.push_back(vec![from.to_string()]);

    while let Some(path) = queue.pop_front() {
        let Some(cur) = path.last() else {
            continue;
        };
        for edge in graph.edges.iter().filter(|e| e.from == *cur) {
            if path.contains(&edge.to) {
                continue;
            }
            let mut next = path.clone();
            next.push(edge.to.clone());
            if edge.to == to {
                return next
                    .iter()
                    .filter_map(|id| node_by_id(graph, id).cloned())
                    .collect();
            }
            queue.push_back(next);
        }
    }
    Vec::new()
}

/// First structural divergence between two graphs (strict: length → node type → outgoing edge).
pub fn first_divergent_causal_node(a: &CausalGraph, b: &CausalGraph) -> Option<CausalNode> {
    if a.nodes.len() != b.nodes.len() {
        if a.nodes.len() < b.nodes.len() {
            return b.nodes.get(a.nodes.len()).cloned();
        }
        return a.nodes.get(b.nodes.len()).cloned();
    }
    for i in 0..a.nodes.len() {
        if a.nodes[i].event_type != b.nodes[i].event_type {
            return Some(b.nodes[i].clone());
        }
    }
    for i in 0..a.nodes.len() {
        if a.nodes[i].surface != b.nodes[i].surface {
            return Some(b.nodes[i].clone());
        }
    }
    for i in 0..a.nodes.len() {
        match (
            crate::core::causal_graph::outgoing_from_node_index(a, i),
            crate::core::causal_graph::outgoing_from_node_index(b, i),
        ) {
            (Some(ea), Some(eb)) if ea != eb => return Some(b.nodes[i].clone()),
            (Some(_), None) | (None, Some(_)) => return Some(b.nodes[i].clone()),
            _ => {}
        }
    }
    None
}

/// Maturity hook: query_causes / query_effects / trace_path / graph divergence API.
pub fn causal_query_completeness_score() -> u8 {
    use crate::core::causal_graph::build_causal_graph;
    use crate::core::execution_trace::{ExecutionEvent, ExecutionTrace};

    let t = ExecutionTrace {
        run_id: "q".into(),
        events: vec![
            ExecutionEvent::Spawn {
                command: "c".into(),
            },
            ExecutionEvent::Stdout {
                chunk: String::new(),
            },
            ExecutionEvent::Stderr {
                chunk: String::new(),
            },
        ],
    };
    let g = build_causal_graph(&t);
    let mut score: u8 = 0;

    let c1 = query_causes(&g, "n1");
    if c1.iter().any(|n| n.id == "n0") {
        score = score.saturating_add(1);
    }
    let e1 = query_effects(&g, "n1");
    if e1.iter().any(|n| n.id == "n2") {
        score = score.saturating_add(1);
    }
    let p1 = trace_path(&g, "n0", "n2");
    let p2 = trace_path(&g, "n0", "n2");
    if !p1.is_empty() && p1 == p2 {
        score = score.saturating_add(1);
    }

    let ta = t.clone();
    let mut tb = t;
    if let ExecutionEvent::Stdout { ref mut chunk } = tb.events[1] {
        *chunk = "x".into();
    }
    let ga = build_causal_graph(&ta);
    let gb = build_causal_graph(&tb);
    if first_divergent_causal_node(&ga, &gb).is_some() {
        score = score.saturating_add(1);
    }

    score.min(3)
}
