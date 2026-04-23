//! Single conversion pipeline for kernel-shaped JSON → `cos_core` (Phase 5.3).

use cos_core::audit::records::record::AuditRecord;
use cos_core::replay::record::ReplayRecord;

/// Deserialize a kernel [`AuditRecord`] when JSON matches the `cos_core` schema.
#[inline]
pub fn audit_record_from_json_bytes(bytes: &[u8]) -> serde_json::Result<AuditRecord> {
    serde_json::from_slice(bytes)
}

/// Deserialize a kernel [`ReplayRecord`] when JSON matches the `cos_core` schema.
#[inline]
pub fn replay_record_from_json_bytes(bytes: &[u8]) -> serde_json::Result<ReplayRecord> {
    serde_json::from_slice(bytes)
}
