use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TrustChainViolation {
    pub code: String,
    pub origin: String,
    pub context: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TrustChainResult {
    pub status: String,
    pub violations: Vec<TrustChainViolation>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TrustChainContract {
    pub key_rotation_policy: String,
    pub attestation_required: bool,
    pub result: TrustChainResult,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TrustChainInput {
    pub signatures_valid: bool,
    pub attestation_valid: bool,
    pub key_rotation_valid: bool,
}

pub fn evaluate_trust_chain_contract(input: TrustChainInput) -> TrustChainContract {
    let mut violations = Vec::new();
    if !input.signatures_valid {
        violations.push(TrustChainViolation {
            code: "trust_chain:signature_invalid".to_string(),
            origin: "trust_chain".to_string(),
            context: "trust_chain.signature".to_string(),
            cause: Some("signature_verification_failed".to_string()),
        });
    }
    if !input.attestation_valid {
        violations.push(TrustChainViolation {
            code: "trust_chain:attestation_invalid".to_string(),
            origin: "trust_chain".to_string(),
            context: "trust_chain.attestation".to_string(),
            cause: Some("provenance_attestation_missing".to_string()),
        });
    }
    if !input.key_rotation_valid {
        violations.push(TrustChainViolation {
            code: "trust_chain:key_rotation_invalid".to_string(),
            origin: "trust_chain".to_string(),
            context: "trust_chain.key_rotation".to_string(),
            cause: Some("rotation_policy_violation".to_string()),
        });
    }
    TrustChainContract {
        key_rotation_policy: "deterministic_rotation_v1".to_string(),
        attestation_required: true,
        result: TrustChainResult {
            status: if violations.is_empty() {
                "ok".into()
            } else {
                "error".into()
            },
            violations,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{evaluate_trust_chain_contract, TrustChainInput};

    #[test]
    fn all_ok() {
        let c = evaluate_trust_chain_contract(TrustChainInput {
            signatures_valid: true,
            attestation_valid: true,
            key_rotation_valid: true,
        });
        assert_eq!(c.result.status, "ok");
    }

    #[test]
    fn deterministic_errors() {
        let c = evaluate_trust_chain_contract(TrustChainInput {
            signatures_valid: false,
            attestation_valid: false,
            key_rotation_valid: false,
        });
        let codes: Vec<&str> = c
            .result
            .violations
            .iter()
            .map(|v| v.code.as_str())
            .collect();
        assert_eq!(
            codes,
            vec![
                "trust_chain:signature_invalid",
                "trust_chain:attestation_invalid",
                "trust_chain:key_rotation_invalid"
            ]
        );
    }
}
