//! `evaluate_*` / `policy_*` — decision functions only (no side effects).

use crate::validation;
use cos_core::audit::records::record::AuditRecord;
use cos_core::evidence::record::EvidenceRecordV2;
use cos_core::replay::record::ReplayRecord;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GovernanceError {
    RejectedShape,
}

pub type PolicyResult<T> = Result<T, GovernanceError>;

/// Policy gate: audit row allowed to enter a kernel-visible append path.
pub fn policy_audit_append_allowed(rec: &AuditRecord) -> PolicyResult<()> {
    if validation::validate_audit_record_shape(rec) {
        Ok(())
    } else {
        Err(GovernanceError::RejectedShape)
    }
}

/// Policy gate: replay frame acceptable for packaging.
pub fn policy_replay_frame_packable(rec: &ReplayRecord) -> PolicyResult<()> {
    if validation::validate_replay_record_shape(rec) {
        Ok(())
    } else {
        Err(GovernanceError::RejectedShape)
    }
}

/// Policy gate: evidence V2 row acceptable for chain append (shape only).
pub fn policy_evidence_v2_append_allowed(rec: &EvidenceRecordV2) -> PolicyResult<()> {
    if validation::validate_evidence_record_v2_shape(rec) {
        Ok(())
    } else {
        Err(GovernanceError::RejectedShape)
    }
}

/// Alias for external “evaluate” naming convention.
#[inline]
pub fn evaluate_audit_append(rec: &AuditRecord) -> PolicyResult<()> {
    policy_audit_append_allowed(rec)
}
