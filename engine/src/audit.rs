//! Audit bundle combining integrity with a run identifier.

use aion_kernel::IntegrityReport;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct AuditReport {
    pub audit: String,
    pub run_id: String,
    pub integrity: IntegrityReport,
}
