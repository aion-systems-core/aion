//! Graph engine 2.0: causal projection over trace + event ordering.

mod builder;
mod model;
mod query;

pub use builder::from_trace;
pub use model::{CausalEdge, CausalGraph, CausalNode};
pub use query::{causes, effects, first_divergent_node, path};

use aion_core::RunResult;
use serde_json::Value;

/// Back-compat name for graph export.
pub fn graph_json(run_json: &str) -> Result<String, String> {
    graph_json_v2(run_json)
}

/// Build a [`CausalGraph`] suitable for JSON / SVG export.
pub fn causal_graph_from_run_json(run_json: &str) -> Result<CausalGraph, String> {
    if let Ok(r) = serde_json::from_str::<RunResult>(run_json) {
        let t = crate::trace::trace_from_run(&r);
        return Ok(from_trace(&t));
    }
    let v: Value = serde_json::from_str(run_json).map_err(|e| format!("graph: {e}"))?;
    let id = v
        .get("run_id")
        .and_then(Value::as_str)
        .ok_or_else(|| "graph: need RunResult JSON or {\"run_id\":\"…\"}".to_string())?;
    Ok(CausalGraph {
        nodes: vec![CausalNode {
            id: id.to_string(),
            event_type: "run".into(),
            surface: String::new(),
            index: 0,
        }],
        edges: vec![],
    })
}

/// Serialize graph for CLI / tools.
pub fn graph_json_v2(run_json: &str) -> Result<String, String> {
    let g = causal_graph_from_run_json(run_json)?;
    serde_json::to_string(&g).map_err(|e| e.to_string())
}
