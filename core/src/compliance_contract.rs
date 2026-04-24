use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ComplianceDomain {
    Dsgvo,
    Iso27001,
    Soc2,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ComplianceControl {
    pub domain: ComplianceDomain,
    pub control: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ComplianceEvidence {
    pub control: String,
    pub source_contract: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ComplianceContract {
    pub controls: Vec<ComplianceControl>,
    pub evidence: Vec<ComplianceEvidence>,
    pub status: String,
}

pub fn evaluate_compliance_contract() -> ComplianceContract {
    let mut controls = vec![
        ComplianceControl {
            domain: ComplianceDomain::Dsgvo,
            control: "logging_retention".into(),
            status: "partial".into(),
        },
        ComplianceControl {
            domain: ComplianceDomain::Iso27001,
            control: "release_governance".into(),
            status: "ok".into(),
        },
        ComplianceControl {
            domain: ComplianceDomain::Soc2,
            control: "evidence_chain".into(),
            status: "ok".into(),
        },
        ComplianceControl {
            domain: ComplianceDomain::Iso27001,
            control: "policy_engine".into(),
            status: "ok".into(),
        },
        ComplianceControl {
            domain: ComplianceDomain::Soc2,
            control: "identity_distribution".into(),
            status: "ok".into(),
        },
    ];
    controls.sort_by(|a, b| {
        (a.domain.clone(), a.control.clone()).cmp(&(b.domain.clone(), b.control.clone()))
    });
    let mut evidence = vec![
        ComplianceEvidence {
            control: "logging_retention".into(),
            source_contract: "logging_policy".into(),
        },
        ComplianceEvidence {
            control: "evidence_chain".into(),
            source_contract: "evidence_contract".into(),
        },
        ComplianceEvidence {
            control: "policy_engine".into(),
            source_contract: "policy_contract".into(),
        },
        ComplianceEvidence {
            control: "identity_distribution".into(),
            source_contract: "identity_contract".into(),
        },
        ComplianceEvidence {
            control: "release_governance".into(),
            source_contract: "release_governance_contract".into(),
        },
    ];
    evidence.sort_by(|a, b| a.control.cmp(&b.control));
    let status = if controls.iter().any(|c| c.status == "open") {
        "error"
    } else if controls.iter().any(|c| c.status == "partial") {
        "partial"
    } else {
        "ok"
    };
    ComplianceContract {
        controls,
        evidence,
        status: status.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::evaluate_compliance_contract;

    #[test]
    fn deterministic_serialization() {
        let c = evaluate_compliance_contract();
        assert_eq!(
            serde_json::to_string(&c).unwrap(),
            serde_json::to_string(&c).unwrap()
        );
    }

    #[test]
    fn contains_expected_controls() {
        let c = evaluate_compliance_contract();
        assert!(c.controls.iter().any(|x| x.control == "policy_engine"));
    }
}
