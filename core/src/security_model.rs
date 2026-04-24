use crate::{
    evaluate_compliance_contract, evaluate_logging_policy, evaluate_threat_model,
    evaluate_vulnerability_sla, run_security_scans, ComplianceContract, LoggingComplianceResult,
    LoggingPolicy, SecurityScanResult, ThreatModel, VulnerabilityReport, VulnerabilitySeverity,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SecurityGap {
    pub code: String,
    pub context: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SecurityPosture {
    pub status: String,
    pub gaps: Vec<SecurityGap>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SecurityModel {
    pub threat_model: ThreatModel,
    pub compliance_contract: ComplianceContract,
    pub security_scanning: SecurityScanResult,
    pub logging_compliance: LoggingComplianceResult,
    pub vulnerability_sla_status: String,
    pub posture: SecurityPosture,
}

pub fn evaluate_security_model() -> SecurityModel {
    let threat_model = evaluate_threat_model();
    let compliance_contract = evaluate_compliance_contract();
    let security_scanning = run_security_scans();
    let logging_policy = LoggingPolicy {
        events: vec![
            "replay".into(),
            "policy".into(),
            "evidence".into(),
            "errors".into(),
            "security_events".into(),
        ],
        retention: vec![],
        pii_guard: "enabled".into(),
    };
    let logging_compliance = evaluate_logging_policy(&logging_policy);
    let vuln = evaluate_vulnerability_sla(&VulnerabilityReport {
        id: "AION-SEC-BASELINE".to_string(),
        severity: VulnerabilitySeverity::Low,
        age_hours: 1,
    });
    let mut gaps = Vec::new();
    if compliance_contract.status == "error" {
        gaps.push(SecurityGap {
            code: "security_model:compliance_gap".to_string(),
            context: "security_model.compliance".to_string(),
        });
    }
    if security_scanning.status != "ok" {
        gaps.push(SecurityGap {
            code: "security_model:scan_gap".to_string(),
            context: "security_model.scanning".to_string(),
        });
    }
    if logging_compliance.status != "ok" {
        gaps.push(SecurityGap {
            code: "security_model:logging_gap".to_string(),
            context: "security_model.logging".to_string(),
        });
    }
    if vuln.is_err() {
        gaps.push(SecurityGap {
            code: "security_model:vulnerability_gap".to_string(),
            context: "security_model.vulnerability_sla".to_string(),
        });
    }
    SecurityModel {
        threat_model,
        compliance_contract,
        security_scanning,
        logging_compliance,
        vulnerability_sla_status: if vuln.is_ok() { "ok" } else { "error" }.to_string(),
        posture: SecurityPosture {
            status: if gaps.is_empty() { "ok" } else { "error" }.to_string(),
            gaps,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::evaluate_security_model;

    #[test]
    fn baseline_security_posture_ok() {
        let m = evaluate_security_model();
        assert_eq!(m.posture.status, "ok");
    }

    #[test]
    fn deterministic_serialization() {
        let m = evaluate_security_model();
        assert_eq!(
            serde_json::to_string(&m).unwrap(),
            serde_json::to_string(&m).unwrap()
        );
    }
}
