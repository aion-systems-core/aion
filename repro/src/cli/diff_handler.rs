// `repro diff <run_a> <run_b>`

use crate::core::{diff, output, storage};

pub fn handle(run_a: &str, run_b: &str) -> Result<(), String> {
    let id_a = storage::resolve_alias(run_a).map_err(|e| format!("diff: run_a: {e}"))?;
    let id_b = storage::resolve_alias(run_b).map_err(|e| format!("diff: run_b: {e}"))?;

    let a = storage::load_run(&id_a).map_err(|e| format!("diff: load {id_a}: {e}"))?;
    let b = storage::load_run(&id_b).map_err(|e| format!("diff: load {id_b}: {e}"))?;

    let index = storage::list_runs().unwrap_or_default();
    let report = diff::diff_runs_with_index(&a, &b, &index);
    print!("{}", output::format_diff(&report));
    Ok(())
}
