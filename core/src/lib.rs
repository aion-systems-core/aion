//! SealRun **core** (`aion-core`): deterministic contracts, capsule I/O, enterprise contract matrices, and the embedded OS contract spec surface.
//!
//! Consumers include the `aion-engine` crate and tooling that depend on stable, serde-friendly contract types and deterministic error envelopes.

pub mod admin_docs_contract;
pub mod api_stability_contract;
pub mod audit_report_contract;
pub mod capsule;
pub mod capsule_abi_contract;
pub mod chaos_contract;
pub mod cli_stability_contract;
pub mod compatibility_test_contract;
pub mod compliance_contract;
pub mod contract_stability;
pub mod contracts;
pub mod determinism_contract;
pub mod determinism_matrix;
pub mod distribution_contract;
pub mod distribution_model;
pub mod dr_contract;
pub mod error;
pub mod evidence;
pub mod evidence_export_contract;
pub mod fuzz_property_contract;
pub mod golden_path_contract;
pub mod governance_model;
pub mod identity;
pub mod identity_matrix;
pub mod incident_contract;
pub mod installer_trust_chain;
pub mod kpi_contract;
pub mod legal_determinism_contract;
pub mod logging_policy;
pub mod lts_policy;
pub mod measurement_model;
pub mod metrics_contract;
pub mod observability_contract;
pub mod operations_model;
pub mod os_contract;
pub mod policy_evidence_contract;
pub mod policy_gate_contract;
pub mod policy_pack_contract;
pub mod provenance;
pub mod regression_matrix;
pub mod release_governance;
pub mod release_signing;
pub mod reliability_contract;
pub mod replay_invariant_gate;
pub mod runbook_contract;
pub mod runtime_isolation_contract;
pub mod sbom;
pub mod security_model;
pub mod security_scanning;
pub mod slo_contract;
pub mod soak_test_contract;
pub mod supply_chain_security;
pub mod tenant_isolation_contract;
pub mod test_strategy_contract;
pub mod threat_model;
pub mod trust_chain_contract;
pub mod upgrade_migration_contract;
pub mod upgrade_replay_contract;

pub use admin_docs_contract::{
    evaluate_admin_docs_contract, AdminDocContract, AdminDocCoverage, AdminDocSection,
};
pub use api_stability_contract::{
    evaluate_api_stability_contract, ApiChangeType, ApiDeprecationNotice, ApiStabilityContract,
    ApiSurface,
};
pub use audit_report_contract::{
    evaluate_audit_report_contract, AuditFinding, AuditFindingSeverity, AuditReport, AuditScope,
};
pub use capsule::{read_capsule_manifest, write_capsule_v1, Capsule};
pub use capsule_abi_contract::{
    evaluate_capsule_abi_contract, CapsuleAbiContract, CapsuleAbiInput, CapsuleAbiResult,
    CapsuleAbiViolation,
};
pub use chaos_contract::{
    run_chaos_experiments, ChaosExperiment, ChaosFault, ChaosResult, ChaosTarget,
};
pub use cli_stability_contract::{
    evaluate_cli_stability_contract, CliChangeType, CliCommandSurface, CliDeprecationWarning,
    CliFlag, CliStabilityContract,
};
pub use compatibility_test_contract::{
    evaluate_compatibility_test_contract, CompatibilityCase, CompatibilityDimension,
    CompatibilityResult, CompatibilityTestContract,
};
pub use compliance_contract::{
    evaluate_compliance_contract, ComplianceContract, ComplianceControl, ComplianceDomain,
    ComplianceEvidence,
};
pub use contract_stability::{
    diff_contract_snapshots, evaluate_contract_stability, write_contract_snapshots,
    ContractBreakingChange, ContractCompatibilityRule, ContractSnapshot, ContractStabilityReport,
};
pub use contracts::*;
pub use determinism_contract::{
    evaluate_determinism_contract, DeterminismContract, DeterminismContractInput,
    DeterminismGuarantee, DeterminismViolation,
};
pub use determinism_matrix::{
    evaluate_determinism_matrix, DeterminismAxis, DeterminismMatrix, DeterminismResult,
    DeterminismTarget,
};
pub use distribution_contract::{
    evaluate_distribution_contract, DistributionArtifact, DistributionChannel,
    DistributionContract, DistributionSupportStatus,
};
pub use distribution_model::{evaluate_distribution_model, DistributionModel};
pub use dr_contract::{
    evaluate_dr_contract, BackupPolicy, DisasterRecoveryContract, DrTestResult, RecoveryObjective,
    RestorePlan,
};
pub use error::{
    aion_error_from_line, canonical_error_json, code, error_to_json, io_cause, is_packed_line,
    line, message_from_code, origin_for_code, sanitize_cause, AionError,
};
pub use evidence::{
    seal_run, sha256_hex, verify_linear, EvidenceChain, EvidenceContract, EvidenceRecord,
    EvidenceReplayAnchors,
};
pub use evidence_export_contract::{
    evaluate_evidence_export_contract, EvidenceExportContract, EvidenceExportFormat,
    EvidenceExportRequest, EvidenceExportResult, EvidenceExportScope,
};
pub use fuzz_property_contract::{
    evaluate_fuzz_property_contract, FuzzFinding, FuzzPropertyContract, FuzzTarget,
    FuzzTestContract, PropertyInvariant, PropertyTarget, PropertyTestContract,
};
pub use golden_path_contract::{
    evaluate_golden_path_contract, GoldenPathContract, GoldenPathResult, GoldenPathScenario,
    GoldenPathStep,
};
pub use governance_model::{
    evaluate_governance_model, GovernanceDomain, GovernanceGap, GovernanceModel, GovernanceStatus,
};
pub use identity::{
    global_consistency_contract_version, os_compatibility_profile, os_identity, os_kernel_version,
    os_kernel_version_from_inputs, OsCompatibilityProfile, OsIdentity, OsInstanceId,
    OsKernelVersion,
};
pub use identity_matrix::{
    evaluate_identity_matrix, IdentityCompatibilityStatus, IdentityDimension, IdentityEntry,
    IdentityMatrix,
};
pub use incident_contract::{
    evaluate_incident_contract, IncidentContract, IncidentResolution, IncidentResponsePlan,
    IncidentSeverity, IncidentTrigger,
};
pub use installer_trust_chain::{
    evaluate_installer_trust_chain, InstallerArtifact, InstallerSignature, InstallerTrustChain,
    InstallerType,
};
pub use kpi_contract::{
    evaluate_kpi_contract, KpiContract, KpiDefinition, KpiDomain, KpiStatus, KpiTarget,
};
pub use legal_determinism_contract::{
    evaluate_legal_determinism_contract, LegalDeterminismContract, LegalDeterminismInput,
    LegalDeterminismResult, LegalDeterminismViolation,
};
pub use logging_policy::{
    evaluate_logging_policy, LogCategory, LoggingComplianceResult, LoggingPolicy, RetentionRule,
};
pub use lts_policy::{evaluate_lts_policy, EolPolicy, LtsChannel, LtsPolicy, SupportWindow};
pub use measurement_model::{
    evaluate_measurement_model, MeasurementGap, MeasurementModel, MeasurementStatus,
};
pub use metrics_contract::{
    evaluate_metrics_contract, MetricDefinition, MetricNamespace, MetricStatus, MetricType,
    MetricsContract,
};
pub use observability_contract::{
    evaluate_observability_contract, ObservabilityContract, ObservabilityInput,
    ObservabilityResult, ObservabilityViolation,
};
pub use operations_model::{evaluate_operations_model, OperationsModel};
pub use os_contract::{
    hash_os_contract_spec_file, hash_os_contract_spec_markdown, os_contract_spec,
    os_contract_spec_version, OsContractSection, OsContractSpec, OsErrorCode, OsFinalityRule,
    OsInvariant,
};
pub use policy_evidence_contract::{
    evaluate_policy_evidence, PolicyAuditTrail, PolicyDecisionRecord, PolicyEvidence,
    PolicyEvidenceChain,
};
pub use policy_gate_contract::{
    evaluate_policy_gate, PolicyGate, PolicyGateContext, PolicyGateDecision, PolicyGateViolation,
};
pub use policy_pack_contract::{
    evaluate_policy_pack, PolicyPack, PolicyPackEntry, PolicyPackLevel, PolicyPackSignature,
};
pub use provenance::{
    generate_provenance, verify_provenance, ProvenancePredicate, ProvenanceStatement,
    ProvenanceSubject,
};
pub use regression_matrix::{
    evaluate_regression_matrix, RegressionArea, RegressionCase, RegressionMatrix, RegressionStatus,
};
pub use release_governance::{
    evaluate_deterministic_build_contract, BuildFingerprint, DeterministicBuildContract,
};
pub use release_signing::{sign_release_artifact, verify_release_signature, ReleaseSignature};
pub use reliability_contract::{
    evaluate_reliability_contract, ErrorBudget, ErrorBudgetStatus, IncidentCriteria,
    ReliabilityContract, ReliabilityEvaluation,
};
pub use replay_invariant_gate::{
    run_replay_invariant_gate, ReplayInvariantCheck, ReplayInvariantGate, ReplayInvariantViolation,
};
pub use runbook_contract::{
    evaluate_runbook_contract, RunbookContract, RunbookResult, RunbookScenario, RunbookStep,
};
pub use runtime_isolation_contract::{
    evaluate_runtime_isolation_contract, RuntimeIsolationContract, RuntimeIsolationInput,
    RuntimeIsolationResult, RuntimeIsolationViolation,
};
pub use sbom::{generate_sbom, verify_sbom, SbomComponent, SbomDocument, SbomHash};
pub use security_model::{evaluate_security_model, SecurityGap, SecurityModel, SecurityPosture};
pub use security_scanning::{
    run_security_scans, SecurityIssue, SecurityScanConfig, SecurityScanResult,
};
pub use slo_contract::{
    evaluate_slo_contract, SloContract, SloEvaluationResult, SloObjective, SloTarget, SloWindow,
};
pub use soak_test_contract::{
    run_soak_test_plan, SoakTestMetric, SoakTestPlan, SoakTestResult, SoakTestTarget,
};
pub use supply_chain_security::{
    evaluate_vulnerability_sla, VulnerabilityReport, VulnerabilitySeverity, VulnerabilitySla,
};
pub use tenant_isolation_contract::{
    evaluate_tenant_isolation_contract, TenantIsolationContract, TenantIsolationInput,
    TenantIsolationResult, TenantIsolationViolation,
};
pub use test_strategy_contract::{
    evaluate_test_strategy_contract, TestCoverageStatus, TestCoverageTarget, TestLayer,
    TestStrategyContract,
};
pub use threat_model::{
    evaluate_threat_model, ThreatCategory, ThreatMitigation, ThreatModel, ThreatSurface,
};
pub use trust_chain_contract::{
    evaluate_trust_chain_contract, TrustChainContract, TrustChainInput, TrustChainResult,
    TrustChainViolation,
};
pub use upgrade_migration_contract::{
    evaluate_upgrade_migration_contract, DowngradePath, MigrationRisk, MigrationStep,
    UpgradeMigrationContract, UpgradePath,
};
pub use upgrade_replay_contract::{
    evaluate_upgrade_replay, UpgradeReplayContract, UpgradeReplayInput, UpgradeReplayResult,
    UpgradeReplayViolation,
};
