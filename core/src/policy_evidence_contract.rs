use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PolicyDecisionRecord {
    pub input_ref: String,
    pub policy_ref: String,
    pub result: String,
    pub timestamp: u64,
    pub actor: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PolicyEvidenceChain {
    pub records: Vec<PolicyDecisionRecord>,
    pub hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PolicyAuditTrail {
    pub entries: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PolicyEvidence {
    pub chain: PolicyEvidenceChain,
    pub audit_trail: PolicyAuditTrail,
    pub status: String,
}

pub fn evaluate_policy_evidence(mut evidence: PolicyEvidence) -> PolicyEvidence {
    evidence
        .chain
        .records
        .sort_by(|a, b| a.timestamp.cmp(&b.timestamp).then(a.actor.cmp(&b.actor)));
    evidence.audit_trail.entries.sort();
    evidence.status = if evidence.chain.records.is_empty() {
        "incomplete".into()
    } else if evidence.chain.hash.is_empty() {
        "tampered".into()
    } else {
        "complete".into()
    };
    evidence
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn evidence_incomplete_negative() {
        let e = evaluate_policy_evidence(PolicyEvidence {
            chain: PolicyEvidenceChain {
                records: vec![],
                hash: "h".into(),
            },
            audit_trail: PolicyAuditTrail { entries: vec![] },
            status: String::new(),
        });
        assert_eq!(e.status, "incomplete");
    }
}
