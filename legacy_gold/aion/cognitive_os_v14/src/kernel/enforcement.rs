use crate::hardware::HardwareManager;
use crate::kernel::system_truth::SystemTruth;
use crate::map_engine::MapModel;
use crate::policy::evaluator::PolicyEvaluator;
use crate::process::ProcessModel;
use anyhow::{anyhow, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnforcementMode {
    /// Full rules including evidence and map presence (uses in-memory map and staged evidence when present).
    Standard,
    /// Hardware budget rules only (process registration before evidence files exist on disk).
    HardwareOnly,
}

pub struct EnforcementEngine;

impl EnforcementEngine {
    /// When `map_for_policy` is set, map rules use it instead of [`SystemTruth::map`] so enforcement can run
    /// before mutating in-memory truth (e.g. before a [`KernelTransaction`]).
    pub fn check_or_block(
        truth: &SystemTruth,
        process: &ProcessModel,
        mode: EnforcementMode,
        map_for_policy: Option<&MapModel>,
    ) -> Result<()> {
        let pid = process.meta.id.to_string();
        let hw = HardwareManager::from_profiles(truth.hardware.clone());
        let evidence_ok = truth.evidence_ready_for_policy(&pid);
        let include_evidence_map = matches!(mode, EnforcementMode::Standard);
        let map = map_for_policy.unwrap_or(&truth.map);
        let report = PolicyEvaluator::evaluate_with_context(
            &pid,
            &truth.policies,
            &hw,
            Some(map),
            evidence_ok,
            include_evidence_map,
        );
        if report.has_violations() {
            let msgs: Vec<String> = report
                .violations
                .iter()
                .map(|v| format!("{}: {}", v.rule, v.message))
                .collect();
            return Err(anyhow!("policy violations for {}: {}", pid, msgs.join("; ")));
        }
        Ok(())
    }
}
