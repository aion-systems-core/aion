// `repro root-cause <run_id>`
//
// Drives `root_cause::build_root_cause_report`, which returns a full
// `ExecutionReport`, and renders its `root_cause` section. The CLI
// layer stays trivial: no classification, no scoring, no formatting
// decisions live here.

use crate::core::{output, root_cause};

pub fn handle(run_id: &str) -> Result<(), String> {
    let report =
        root_cause::build_root_cause_report(run_id).map_err(|e| format!("root-cause: {e}"))?;

    let rc = report.root_cause.as_ref().ok_or_else(|| {
        "root-cause: report missing root_cause section (internal invariant broken)".to_string()
    })?;

    print!("{}", output::format_root_cause_summary(rc));
    Ok(())
}
