// `repro graph <run_id|last>`

use crate::core::causal_graph::{build_causal_graph, format_causal_graph_text};
use crate::core::storage;

pub fn handle(run_id: &str) -> Result<(), String> {
    let id = storage::resolve_alias(run_id).map_err(|e| format!("graph: {e}"))?;
    let artifact = storage::load_run(&id).map_err(|e| format!("graph: {e}"))?;
    let g = build_causal_graph(&artifact.trace);
    print!("{}", format_causal_graph_text(&g));
    Ok(())
}
