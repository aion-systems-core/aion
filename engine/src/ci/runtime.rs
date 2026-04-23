//! CI runtime orchestration: capture + diff + failure class.

use super::baseline::check_baseline;
use super::failure::FailureKind;
use super::meta::CiRunMeta;
use aion_core::DriftReport;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CiCheckOutcome {
    pub drift: DriftReport,
    pub failure: FailureKind,
    pub meta: CiRunMeta,
}

pub fn check_with_meta(baseline: &Path, actual_json: &str) -> Result<CiCheckOutcome, String> {
    let drift = check_baseline(baseline, actual_json)?;
    let failure = FailureKind::from_fields(&drift.fields);
    Ok(CiCheckOutcome {
        drift,
        failure,
        meta: CiRunMeta {
            schema_version: super::schema::CI_LEDGER_SCHEMA_VERSION,
            baseline_file_version: super::schema::BASELINE_FILE_VERSION,
            recorded_at_epoch_secs: aion_kernel::now_secs(),
        },
    })
}
