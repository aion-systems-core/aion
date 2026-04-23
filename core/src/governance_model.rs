use serde::{Deserialize, Serialize};

use crate::{PolicyEvidence, PolicyGate, PolicyPack};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum GovernanceDomain {
    Policy,
    Security,
    Compliance,
    Release,
    Operations,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GovernanceStatus {
    pub domain: GovernanceDomain,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GovernanceGap {
    pub domain: GovernanceDomain,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GovernanceModel {
    pub policy_packs: Vec<PolicyPack>,
    pub policy_gates: Vec<PolicyGate>,
    pub policy_evidence: PolicyEvidence,
    pub domains: Vec<GovernanceStatus>,
    pub gaps: Vec<GovernanceGap>,
    pub status: String,
}

pub fn evaluate_governance_model(
    mut policy_packs: Vec<PolicyPack>,
    mut policy_gates: Vec<PolicyGate>,
    policy_evidence: PolicyEvidence,
    mut domains: Vec<GovernanceStatus>,
) -> GovernanceModel {
    policy_packs.sort_by(|a, b| a.name.cmp(&b.name));
    policy_gates.sort_by(|a, b| a.context.cmp(&b.context));
    domains.sort_by(|a, b| a.domain.cmp(&b.domain));
    let mut gaps = Vec::new();
    let required = vec![
        GovernanceDomain::Policy,
        GovernanceDomain::Security,
        GovernanceDomain::Compliance,
        GovernanceDomain::Release,
        GovernanceDomain::Operations,
    ];
    for d in required {
        if !domains.iter().any(|x| x.domain == d) {
            gaps.push(GovernanceGap {
                domain: d,
                message: "missing_domain".into(),
            });
        }
    }
    let status = if gaps.is_empty()
        && policy_packs.iter().all(|p| p.status == "valid")
        && policy_gates.iter().all(|g| g.status == "ok")
        && policy_evidence.status == "complete"
    {
        "ok"
    } else {
        "error"
    };
    GovernanceModel {
        policy_packs,
        policy_gates,
        policy_evidence,
        domains,
        gaps,
        status: status.into(),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        evaluate_policy_evidence, evaluate_policy_gate, evaluate_policy_pack, PolicyAuditTrail,
        PolicyDecisionRecord, PolicyEvidence, PolicyEvidenceChain, PolicyGate, PolicyGateContext,
        PolicyGateDecision, PolicyPack, PolicyPackEntry, PolicyPackLevel, PolicyPackSignature,
    };

    use super::*;

    #[test]
    fn governance_gaps_negative() {
        let pack = evaluate_policy_pack(PolicyPack {
            name: "baseline".into(),
            version: "1".into(),
            level: PolicyPackLevel::Baseline,
            entries: vec![PolicyPackEntry {
                id: "p1".into(),
                use_case: "internal".into(),
                rule: "rule".into(),
            }],
            signature: Some(PolicyPackSignature {
                signature_id: "s1".into(),
                algorithm: "ed25519".into(),
                valid: true,
            }),
            status: String::new(),
        });
        let gate = evaluate_policy_gate(PolicyGate {
            context: PolicyGateContext::Ci,
            decision: Some(PolicyGateDecision::Allow),
            violations: vec![],
            status: String::new(),
        });
        let evidence = evaluate_policy_evidence(PolicyEvidence {
            chain: PolicyEvidenceChain {
                records: vec![PolicyDecisionRecord {
                    input_ref: "i".into(),
                    policy_ref: "p".into(),
                    result: "allow".into(),
                    timestamp: 1,
                    actor: "ci".into(),
                }],
                hash: "h".into(),
            },
            audit_trail: PolicyAuditTrail {
                entries: vec!["entry".into()],
            },
            status: String::new(),
        });
        let model = evaluate_governance_model(
            vec![pack],
            vec![gate],
            evidence,
            vec![GovernanceStatus {
                domain: GovernanceDomain::Policy,
                status: "ok".into(),
            }],
        );
        assert_eq!(model.status, "error");
    }
}

