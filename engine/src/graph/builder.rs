//! Build [`CausalGraph`](super::model::CausalGraph) from a [`Trace`](crate::trace::Trace).

use super::model::{CausalEdge, CausalGraph, CausalNode};
use crate::trace::Trace;

fn edge_relation(prev_op: &str, next_op: &str) -> &'static str {
    match (prev_op, next_op) {
        ("Env", "Process") => "enables",
        ("Process", "Process") => "streams",
        ("Exit", _) => "terminates",
        _ => "next",
    }
}

pub fn from_trace(trace: &Trace) -> CausalGraph {
    let mut nodes = Vec::new();
    let mut edges = Vec::new();
    for (i, s) in trace.spans.iter().enumerate() {
        let id = format!("n{}", s.seq);
        nodes.push(CausalNode {
            id: id.clone(),
            event_type: s.op.clone(),
            surface: s.surface.clone(),
            index: i,
        });
        if i > 0 {
            let prev = &trace.spans[i - 1];
            let rel = edge_relation(&prev.op, &s.op);
            edges.push(CausalEdge {
                from: format!("n{}", prev.seq),
                to: id,
                relation: rel.into(),
            });
        }
    }
    CausalGraph { nodes, edges }
}
