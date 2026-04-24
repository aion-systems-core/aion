use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LegalDeterminismViolation {
    pub code: String,
    pub origin: String,
    pub context: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LegalDeterminismResult {
    pub status: String,
    pub violations: Vec<LegalDeterminismViolation>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LegalDeterminismContract {
    pub license_contract_version: String,
    pub sla_contract_version: String,
    pub result: LegalDeterminismResult,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LegalDeterminismInput {
    pub license_stable: bool,
    pub sla_stable: bool,
    pub terms_machine_readable: bool,
}

pub fn evaluate_legal_determinism_contract(
    input: LegalDeterminismInput,
) -> LegalDeterminismContract {
    let mut violations = Vec::new();
    if !input.license_stable {
        violations.push(LegalDeterminismViolation {
            code: "legal_determinism:license_unstable".to_string(),
            origin: "legal_determinism".to_string(),
            context: "legal_determinism.license".to_string(),
            cause: Some("license_terms_changed".to_string()),
        });
    }
    if !input.sla_stable {
        violations.push(LegalDeterminismViolation {
            code: "legal_determinism:sla_unstable".to_string(),
            origin: "legal_determinism".to_string(),
            context: "legal_determinism.sla".to_string(),
            cause: Some("sla_terms_changed".to_string()),
        });
    }
    if !input.terms_machine_readable {
        violations.push(LegalDeterminismViolation {
            code: "legal_determinism:terms_not_machine_readable".to_string(),
            origin: "legal_determinism".to_string(),
            context: "legal_determinism.machine_readable".to_string(),
            cause: Some("contract_terms_not_canonical".to_string()),
        });
    }
    LegalDeterminismContract {
        license_contract_version: "v1".to_string(),
        sla_contract_version: "v1".to_string(),
        result: LegalDeterminismResult {
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
    use super::{evaluate_legal_determinism_contract, LegalDeterminismInput};

    #[test]
    fn all_ok() {
        let c = evaluate_legal_determinism_contract(LegalDeterminismInput {
            license_stable: true,
            sla_stable: true,
            terms_machine_readable: true,
        });
        assert_eq!(c.result.status, "ok");
    }

    #[test]
    fn deterministic_errors() {
        let c = evaluate_legal_determinism_contract(LegalDeterminismInput {
            license_stable: false,
            sla_stable: false,
            terms_machine_readable: false,
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
                "legal_determinism:license_unstable",
                "legal_determinism:sla_unstable",
                "legal_determinism:terms_not_machine_readable"
            ]
        );
    }
}
