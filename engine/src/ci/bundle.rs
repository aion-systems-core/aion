//! Serializable CI command payloads for product output.

use super::meta::CiRunMeta;
use aion_core::{DriftReport, RunResult};
use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct CiRunBundle {
    pub baseline: String,
    pub run: RunResult,
    pub meta: CiRunMeta,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct CiDriftBundle {
    pub drift: DriftReport,
    pub actual: RunResult,
}
