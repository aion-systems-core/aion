use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CapsuleAbiViolation {
    pub code: String,
    pub origin: String,
    pub context: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CapsuleAbiResult {
    pub status: String,
    pub abi_version: String,
    pub violations: Vec<CapsuleAbiViolation>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CapsuleAbiContract {
    pub current_kernel_version: String,
    pub supported_abi_versions: Vec<String>,
    pub result: CapsuleAbiResult,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CapsuleAbiInput {
    pub abi_version: String,
    pub abi_layout_stable: bool,
    pub fields_compatible: bool,
    pub serialization_compatible: bool,
}

pub fn evaluate_capsule_abi_contract(
    current_kernel_version: &str,
    input: CapsuleAbiInput,
) -> CapsuleAbiContract {
    let mut violations = Vec::new();
    if !input.abi_layout_stable {
        violations.push(CapsuleAbiViolation {
            code: "capsule_abi:layout_incompatible".to_string(),
            origin: "capsule_abi".to_string(),
            context: "capsule_abi.layout".to_string(),
            cause: Some("abi_layout_changed".to_string()),
        });
    }
    if !input.fields_compatible {
        violations.push(CapsuleAbiViolation {
            code: "capsule_abi:fields_incompatible".to_string(),
            origin: "capsule_abi".to_string(),
            context: "capsule_abi.fields".to_string(),
            cause: Some("required_fields_changed".to_string()),
        });
    }
    if !input.serialization_compatible {
        violations.push(CapsuleAbiViolation {
            code: "capsule_abi:serialization_incompatible".to_string(),
            origin: "capsule_abi".to_string(),
            context: "capsule_abi.serialization".to_string(),
            cause: Some("canonical_serialization_changed".to_string()),
        });
    }
    CapsuleAbiContract {
        current_kernel_version: current_kernel_version.to_string(),
        supported_abi_versions: vec!["v1".to_string()],
        result: CapsuleAbiResult {
            status: if violations.is_empty() {
                "ok".to_string()
            } else {
                "error".to_string()
            },
            abi_version: input.abi_version,
            violations,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{evaluate_capsule_abi_contract, CapsuleAbiInput};

    fn ok_input() -> CapsuleAbiInput {
        CapsuleAbiInput {
            abi_version: "v1".to_string(),
            abi_layout_stable: true,
            fields_compatible: true,
            serialization_compatible: true,
        }
    }

    #[test]
    fn deterministic_serialization() {
        let c = evaluate_capsule_abi_contract("0.2.0", ok_input());
        let a = serde_json::to_string(&c).expect("json a");
        let b = serde_json::to_string(&c).expect("json b");
        assert_eq!(a, b);
    }

    #[test]
    fn all_ok_status() {
        let c = evaluate_capsule_abi_contract("0.2.0", ok_input());
        assert_eq!(c.result.status, "ok");
    }

    #[test]
    fn violations_are_deterministic() {
        let mut i = ok_input();
        i.abi_layout_stable = false;
        i.fields_compatible = false;
        i.serialization_compatible = false;
        let c = evaluate_capsule_abi_contract("0.2.0", i);
        let codes: Vec<&str> = c
            .result
            .violations
            .iter()
            .map(|v| v.code.as_str())
            .collect();
        assert_eq!(
            codes,
            vec![
                "capsule_abi:layout_incompatible",
                "capsule_abi:fields_incompatible",
                "capsule_abi:serialization_incompatible"
            ]
        );
    }
}
