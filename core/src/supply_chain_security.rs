use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum VulnerabilitySeverity {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VulnerabilityReport {
    pub id: String,
    pub severity: VulnerabilitySeverity,
    pub age_hours: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VulnerabilitySla {
    pub severity: VulnerabilitySeverity,
    pub max_hours: u64,
}

pub fn evaluate_vulnerability_sla(report: &VulnerabilityReport) -> Result<VulnerabilitySla, String> {
    let max = match report.severity {
        VulnerabilitySeverity::Critical => 72,
        VulnerabilitySeverity::High => 24 * 7,
        VulnerabilitySeverity::Medium => 24 * 30,
        VulnerabilitySeverity::Low => 24 * 90,
    };
    if report.age_hours > max {
        return Err("supply_chain:sla_violation".to_string());
    }
    Ok(VulnerabilitySla {
        severity: report.severity.clone(),
        max_hours: max,
    })
}

#[cfg(test)]
mod tests {
    use super::{evaluate_vulnerability_sla, VulnerabilityReport, VulnerabilitySeverity};

    #[test]
    fn sla_rules_apply() {
        let r = VulnerabilityReport {
            id: "CVE-1".to_string(),
            severity: VulnerabilitySeverity::Critical,
            age_hours: 10,
        };
        assert!(evaluate_vulnerability_sla(&r).is_ok());
    }

    #[test]
    fn sla_violation_negative() {
        let r = VulnerabilityReport {
            id: "CVE-2".to_string(),
            severity: VulnerabilitySeverity::Critical,
            age_hours: 100,
        };
        assert!(evaluate_vulnerability_sla(&r).is_err());
    }
}

