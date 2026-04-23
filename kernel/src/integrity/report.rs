//! Serializable integrity report for CLI / audit.

use super::rules::RuleOutcome;
use aion_core::EvidenceChain;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IntegrityReport {
    pub kernel_build_hash: String,
    pub rule_outcomes: Vec<RuleOutcomeView>,
    pub evidence_root: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuleOutcomeView {
    pub rule_id: String,
    pub passed: bool,
    pub detail: String,
}

pub fn kernel_build_hash() -> String {
    let meta = concat!(env!("CARGO_PKG_NAME"), "@", env!("CARGO_PKG_VERSION"));
    format!("{:x}", Sha256::digest(meta.as_bytes()))
}

pub fn build_report(
    outcomes: &[RuleOutcome],
    evidence: Option<&EvidenceChain>,
) -> IntegrityReport {
    let rule_outcomes = outcomes
        .iter()
        .map(|o| RuleOutcomeView {
            rule_id: o.rule_id.to_string(),
            passed: o.passed,
            detail: o.detail.clone(),
        })
        .collect();
    let evidence_root = evidence
        .map(|c| c.root_digest())
        .unwrap_or_else(|| "none".into());
    IntegrityReport {
        kernel_build_hash: kernel_build_hash(),
        rule_outcomes,
        evidence_root,
    }
}
