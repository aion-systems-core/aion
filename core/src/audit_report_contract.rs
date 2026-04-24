use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum AuditScope {
    Security,
    Compliance,
    Release,
    Operations,
    DataProtection,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum AuditFindingSeverity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AuditFinding {
    pub id: String,
    pub scope: AuditScope,
    pub severity: AuditFindingSeverity,
    pub evidence_ref: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AuditReport {
    pub findings: Vec<AuditFinding>,
    pub status: String,
}

pub fn evaluate_audit_report_contract(mut findings: Vec<AuditFinding>) -> AuditReport {
    findings.sort_by(|a, b| a.id.cmp(&b.id));
    let failed = findings.iter().any(|f| f.evidence_ref.is_empty());
    let has_findings = !findings.is_empty();
    let status = if failed {
        "failed"
    } else if has_findings {
        "findings"
    } else {
        "clean"
    };
    AuditReport {
        findings,
        status: status.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn audit_without_evidence_negative() {
        let r = evaluate_audit_report_contract(vec![AuditFinding {
            id: "a1".into(),
            scope: AuditScope::Security,
            severity: AuditFindingSeverity::High,
            evidence_ref: String::new(),
        }]);
        assert_eq!(r.status, "failed");
    }
}
