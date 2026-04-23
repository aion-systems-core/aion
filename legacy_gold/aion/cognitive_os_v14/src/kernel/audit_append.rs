//! Kernel-visible audit append path with governance precheck (Phase 4.5b).
//!
//! v14 must not import `cos_governance` directly — all policy goes through `cos_adapters`.

use anyhow::Result;
use cos_core::audit::records::record::AuditRecord;

/// Runs governance precheck, emits observability logs, then persists **only** if allowed.
pub fn persist_kernel_audit_record_with_governance<F>(record: &AuditRecord, persist: F) -> Result<()>
where
    F: FnOnce(&AuditRecord) -> Result<()>,
{
    let decision = cos_adapters::delegate_audit_append_precheck_decision(record);

    log::info!(
        "AUDIT_PRECHECK decision={} reason={:?}",
        decision.allow,
        decision.reason
    );

    if decision.allow {
        persist(record)?;
    } else {
        log::warn!("AUDIT_BLOCKED: {:?}", decision.reason);
        anyhow::bail!(
            "AUDIT_BLOCKED: {:?}",
            decision.reason.unwrap_or_else(|| "unknown".into())
        );
    }
    Ok(())
}
