// `repro why <run_id|last>` — causal query view (causes / effects / graph divergence).

use crate::core::causal_graph::{build_causal_graph, CausalGraph, CausalNode};
use crate::core::causal_query::{first_divergent_causal_node, query_causes, query_effects};
use crate::core::storage;
use crate::core::why_engine;

fn focal_node_id(graph: &CausalGraph) -> Option<String> {
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

fn format_list(nodes: &[CausalNode]) -> String {
    if nodes.is_empty() {
        return "  (none)\n".to_string();
    }
    let mut s = String::new();
    for n in nodes {
        s.push_str(&format!("- {} ({})\n", n.id, n.event_type));
    }
    s
}

pub fn handle(run_id: &str, compare_to: Option<&str>) -> Result<(), String> {
    if let Some(other) = compare_to {
        let id_a = storage::resolve_alias(run_id).map_err(|e| format!("why: {e}"))?;
        let id_b = storage::resolve_alias(other).map_err(|e| format!("why: {e}"))?;
        let a = storage::load_run(&id_a).map_err(|e| format!("why: {e}"))?;
        let b = storage::load_run(&id_b).map_err(|e| format!("why: {e}"))?;
        print!("{}", why_engine::explain_pair_why(&a, &b));
        return Ok(());
    }

    let id = storage::resolve_alias(run_id).map_err(|e| format!("why: {e}"))?;
    let current = storage::load_run(&id).map_err(|e| format!("why: {e}"))?;
    let graph = build_causal_graph(&current.trace);

    let focal = focal_node_id(&graph).ok_or_else(|| "why: empty causal graph".to_string())?;
    let focal_node = graph
        .nodes
        .iter()
        .find(|n| n.id == focal)
        .ok_or_else(|| "why: focal node missing".to_string())?;

    let causes = query_causes(&graph, &focal);
    let effects = query_effects(&graph, &focal);

    let mut out = String::new();
    out.push_str("CAUSE ANALYSIS\n");
    out.push_str(&format!(
        "Node: {} ({})\n\n",
        focal_node.id, focal_node.event_type
    ));
    out.push_str("Causes:\n");
    out.push_str(&format_list(&causes));
    out.push_str("\nEffects:\n");
    out.push_str(&format_list(&effects));
    out.push_str("\nROOT CAUSE:\n");

    let runs = storage::list_runs().map_err(|e| format!("why: {e}"))?;
    let idx = runs
        .iter()
        .position(|r| r == &id)
        .ok_or_else(|| "why: run not in index".to_string())?;

    if idx == 0 {
        out.push_str("(no previous run for graph comparison)\n");
    } else {
        let prev_id = runs[idx - 1].clone();
        let previous = storage::load_run(&prev_id).map_err(|e| format!("why: {e}"))?;
        let gp = build_causal_graph(&previous.trace);
        let gc = build_causal_graph(&current.trace);
        match first_divergent_causal_node(&gp, &gc) {
            Some(n) => {
                out.push_str(&format!(
                    "Graph divergence at node {} ({})\n",
                    n.id, n.event_type
                ));
            }
            None => out.push_str("no graph divergence from previous run\n"),
        }
    }

    print!("{out}");
    Ok(())
}
