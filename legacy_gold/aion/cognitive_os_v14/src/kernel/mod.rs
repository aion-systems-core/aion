//! COS Kernel v1 — orchestrates persistence, enforcement, and evidence chain.
//!
//! Canonical kernel **DTOs** live in `cos_core` — re-exported here for a single import path
//! (`crate::kernel::…`) without duplicating struct definitions.

/// When `true`, the Truth Layer is frozen from Phase 4 onward; do not change kernel semantics or on-disk contracts without COS architecture sign-off and a deliberate version bump.
pub const KERNEL_LOCKED: bool = true;

pub use cos_core::audit::records::chain::AuditChain;
pub use cos_core::audit::records::record::AuditRecord;
pub use cos_core::evidence::record::EvidenceRecordV2;
pub use cos_core::replay::record::ReplayRecord;

pub mod audit_append;
mod enforcement;
mod envelope;
mod evidence_chain;
mod execution;
mod snapshot;
mod system_truth;
mod transaction;

pub use enforcement::{EnforcementEngine, EnforcementMode};
pub use envelope::ExecutionEnvelope;
pub use evidence_chain::{verify_evidence_chain, EvidenceChain};
pub use execution::{ExecutionResult, KernelExecutionEngine};
pub use snapshot::{snapshots_directory, KernelSnapshot, SystemSnapshot};
pub use system_truth::{SystemTruth, SystemTruthSnapshot};
pub use transaction::KernelTransaction;
