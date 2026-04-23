//! Pure `validate_*` helpers over `cos_core` DTOs — no mutation, no spawning, no runtime imports.

use cos_core::audit::records::record::AuditRecord;
use cos_core::evidence::record::EvidenceRecordV2;
use cos_core::replay::record::ReplayRecord;

/// Structural sanity for an append-only audit row (kernel contract: non-empty actor/action).
pub fn validate_audit_record_shape(rec: &AuditRecord) -> bool {
    !rec.actor.trim().is_empty() && !rec.action.trim().is_empty()
}

/// Replay frame must name a step and carry determinism metadata string.
pub fn validate_replay_record_shape(rec: &ReplayRecord) -> bool {
    !rec.step_name.trim().is_empty() && !rec.determinism.trim().is_empty()
}

/// Evidence V2 row must have coherent index and hash strings present.
pub fn validate_evidence_record_v2_shape(rec: &EvidenceRecordV2) -> bool {
    !rec.current_hash.trim().is_empty()
}
