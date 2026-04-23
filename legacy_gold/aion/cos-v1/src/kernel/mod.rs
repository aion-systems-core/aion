pub mod envelope;
pub mod enforcement;
pub mod evidence_chain;
pub mod execution;
pub mod snapshot;
pub mod system_truth;
pub mod transaction;
pub mod types;

/// Canonical truth DTOs from `cos_core` (do not redefine in cos-v1 kernel).
pub use cos_core::audit::records::record::AuditRecord as CosCoreAuditRecord;
pub use cos_core::evidence::record::EvidenceRecordV2 as CosCoreEvidenceRecordV2;
pub use cos_core::replay::record::ReplayRecord as CosCoreReplayRecord;

pub use envelope::ExecutionEnvelope;
pub use enforcement::{EnforcementEngine, EnforcementResult};
pub use evidence_chain::EvidenceWriter;
pub use execution::{ExecutionContext, KernelError, KernelExecutionEngine, KernelExecutionOutcome, KernelResult};
pub use snapshot::{KernelSnapshot, SnapshotEngine};
pub use system_truth::SystemTruth;
pub use transaction::KernelMutation;
pub use types::{Step, SystemState};
