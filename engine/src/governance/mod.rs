//! SealRun Governance v1 — policy, determinism, integrity, and CI baseline enforcement for AI capsules.

mod audit;
mod ci;
mod determinism;
mod integrity;
mod policy;
pub mod validate;

pub use audit::{append_governance_audit, GovernanceAuditRecord};
pub use ci::{ci_check_against_baseline, ci_record_baseline, CiBaseline, CiResult};
pub use determinism::{
    apply_determinism_profile, load_determinism, validate_determinism, DeterminismProfile,
    DeterminismViolation,
};
pub use integrity::{
    aion_evidence_generate_keypair, aion_evidence_sign, aion_evidence_verify_ed25519,
    load_integrity, validate_integrity, IntegrityProfile, IntegrityViolation,
};
pub use integrity::{sign_integrity, IntegritySignature};
pub use policy::{
    builtin_policy_profile, compose_policies, load_policy, validate_capsule_against_policy,
    PolicyProfile, PolicyViolation,
};
pub use validate::{
    governance_report_with_ci, validate_capsule, validate_capsules_parallel, GovernanceCiAttach,
    GovernanceReport,
};
