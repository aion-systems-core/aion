//! Baseline record/load (filesystem).

use super::schema::{BASELINE_FILE_VERSION, CI_LEDGER_SCHEMA_VERSION};
use crate::diff::diff_runs;
use aion_core::DriftReport;
use aion_kernel::now_secs;
use std::fs;
use std::path::Path;

use super::meta::CiRunMeta;

pub fn record_baseline(path: &Path, run_json: &str) -> Result<CiRunMeta, String> {
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    fs::write(path, run_json).map_err(|e| format!("ci record: {e}"))?;
    Ok(CiRunMeta {
        schema_version: CI_LEDGER_SCHEMA_VERSION,
        baseline_file_version: BASELINE_FILE_VERSION,
        recorded_at_epoch_secs: now_secs(),
    })
}

pub fn check_baseline(path: &Path, actual_json: &str) -> Result<DriftReport, String> {
    let base = fs::read_to_string(path).map_err(|e| format!("ci read baseline: {e}"))?;
    diff_runs(&base, actual_json)
}
