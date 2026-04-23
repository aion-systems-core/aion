//! Graph queries (ported semantics: sorted neighbors, BFS path).

use super::model::{CausalGraph, CausalNode};
use std::collections::VecDeque;

fn node_by_id<'a>(g: &'a CausalGraph, id: &str) -> Option<&'a CausalNode> {
    g.nodes.iter().find(|n| n.id == id)
}

pub fn causes(g: &CausalGraph, node_id: &str) -> Vec<CausalNode> {
    let mut v: Vec<CausalNode> = g
        .edges
        .iter()
        .filter(|e| e.to == node_id)
        .filter_map(|e| node_by_id(g, &e.from).cloned())
        .collect();
    v.sort_by_key(|n| n.index);
    v
}

pub fn effects(g: &CausalGraph, node_id: &str) -> Vec<CausalNode> {
    let mut v: Vec<CausalNode> = g
        .edges
        .iter()
        .filter(|e| e.from == node_id)
        .filter_map(|e| node_by_id(g, &e.to).cloned())
        .collect();
    v.sort_by_key(|n| n.index);
    v
}

pub fn path(g: &CausalGraph, from: &str, to: &str) -> Vec<CausalNode> {
    if from == to {
        return node_by_id(g, from).cloned().into_iter().collect();
    }
    let mut queue = VecDeque::new();
    queue.push_back(vec![from.to_string()]);
    while let Some(p) = queue.pop_front() {
        let Some(cur) = p.last() else { continue };
        for e in g.edges.iter().filter(|e| e.from == *cur) {
            if p.contains(&e.to) {
                continue;
            }
            let mut nxt = p.clone();
            nxt.push(e.to.clone());
            if e.to == to {
                return nxt
                    .iter()
                    .filter_map(|id| node_by_id(g, id).cloned())
                    .collect();
            }
            queue.push_back(nxt);
        }
    }
    Vec::new()
}

pub fn first_divergent_node(a: &CausalGraph, b: &CausalGraph) -> Option<CausalNode> {
    if a.nodes.len() != b.nodes.len() {
        return if a.nodes.len() < b.nodes.len() {
            b.nodes.get(a.nodes.len()).cloned()
        } else {
            a.nodes.get(b.nodes.len()).cloned()
        };
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
    None
}
