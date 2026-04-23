//! Adapter layer — cross-system translation and delegation (Phase 4.5).
//!
//! **Must not:** define alternate kernel structs or embed policy/business rules.
//! Use `from_*` for serde mapping; use `delegate_*` to reach `cos_runtime` without reverse deps.

#![forbid(unsafe_code)]

pub mod from_repro;
pub mod from_v1;
pub mod from_v14;
pub mod kernel_compatible;
pub mod legacy_bridge;
pub mod unified;

pub use kernel_compatible::KernelCompatible;
pub use legacy_bridge::{
    cos_v1_integration_outer_v2, tagged_build_audit_chain, tagged_v1_evidence,
    tagged_v1_kernel_evidence, tagged_v14_evidence, BridgeError, BuildAuditChainBridge,
    V1KernelEvidenceBridge, V14EvidenceBridge,
};

use cos_core::audit::records::record::AuditRecord;

/// Compile-time probe for dependents.
#[derive(Debug, Clone, Copy)]
pub struct AdapterHostMarker;

/// Outcome of the governance-gated audit append precheck (Phase 4.5b observability surface).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditAppendPrecheckDecision {
    pub allow: bool,
    pub reason: Option<String>,
}

/// Delegates append preflight to runtime (which consults governance only).
#[inline]
pub fn delegate_audit_append_precheck(rec: &AuditRecord) -> Result<(), cos_runtime::RuntimeError> {
    delegate_audit_append_precheck_decision(rec).into_result()
}

/// Single adapter entry point: runtime → governance → `cos_core` audit row shape check.
#[inline]
pub fn delegate_audit_append_precheck_decision(rec: &AuditRecord) -> AuditAppendPrecheckDecision {
    // `cos_core::AuditRecord` has no `source` field; `actor` is the logical provenance key here.
    debug_assert!(
        !rec.actor.eq_ignore_ascii_case("corrupted"),
        "Invalid audit source (actor) passed into governance pipeline"
    );
    match cos_runtime::orchestrate_audit_append_precheck(rec) {
        Ok(()) => AuditAppendPrecheckDecision {
            allow: true,
            reason: None,
        },
        Err(e) => AuditAppendPrecheckDecision {
            allow: false,
            reason: Some(format!("{e:?}")),
        },
    }
}

impl AuditAppendPrecheckDecision {
    pub fn into_result(self) -> Result<(), cos_runtime::RuntimeError> {
        if self.allow {
            Ok(())
        } else {
            Err(cos_runtime::RuntimeError::GovernanceRejected)
        }
    }
}
