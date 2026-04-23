//! Governance layer — policy, constraints, validation (Phase 4.5).
//!
//! **Must not:** spawn execution, mutate live kernel state, or depend on `cos_runtime` / `cos_adapters`.

#![forbid(unsafe_code)]

pub mod interfaces;
pub mod policy;
pub mod validation;

pub use interfaces::{PolicyClock, PolicyInputSource};
pub use policy::{evaluate_audit_append, policy_audit_append_allowed, GovernanceError, PolicyResult};
pub use policy::{
    policy_evidence_v2_append_allowed, policy_replay_frame_packable,
};
pub use validation::{
    validate_audit_record_shape, validate_evidence_record_v2_shape, validate_replay_record_shape,
};
