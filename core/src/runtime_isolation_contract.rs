use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeIsolationViolation {
    pub code: String,
    pub origin: String,
    pub context: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeIsolationResult {
    pub status: String,
    pub violations: Vec<RuntimeIsolationViolation>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeIsolationContract {
    pub io_boundary_mode: String,
    pub side_effect_matrix_version: String,
    pub result: RuntimeIsolationResult,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeIsolationInput {
    pub io_boundary_enforced: bool,
    pub side_effects_bounded: bool,
    pub network_isolated: bool,
}

pub fn evaluate_runtime_isolation_contract(input: RuntimeIsolationInput) -> RuntimeIsolationContract {
    let mut violations = Vec::new();
    if !input.io_boundary_enforced {
        violations.push(RuntimeIsolationViolation {
            code: "runtime_isolation:io_boundary_violation".to_string(),
            origin: "runtime_isolation".to_string(),
            context: "runtime_isolation.io_boundary".to_string(),
            cause: Some("io_boundary_not_enforced".to_string()),
        });
    }
    if !input.side_effects_bounded {
        violations.push(RuntimeIsolationViolation {
            code: "runtime_isolation:side_effect_violation".to_string(),
            origin: "runtime_isolation".to_string(),
            context: "runtime_isolation.side_effect_matrix".to_string(),
            cause: Some("side_effect_matrix_broken".to_string()),
        });
    }
    if !input.network_isolated {
        violations.push(RuntimeIsolationViolation {
            code: "runtime_isolation:network_violation".to_string(),
            origin: "runtime_isolation".to_string(),
            context: "runtime_isolation.network".to_string(),
            cause: Some("network_not_isolated".to_string()),
        });
    }
    RuntimeIsolationContract {
        io_boundary_mode: "strict".to_string(),
        side_effect_matrix_version: "v1".to_string(),
        result: RuntimeIsolationResult {
            status: if violations.is_empty() { "ok".into() } else { "error".into() },
            violations,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{evaluate_runtime_isolation_contract, RuntimeIsolationInput};

    #[test]
    fn deterministic_serialization_and_ok() {
        let c = evaluate_runtime_isolation_contract(RuntimeIsolationInput {
            io_boundary_enforced: true,
            side_effects_bounded: true,
            network_isolated: true,
        });
        assert_eq!(c.result.status, "ok");
        let a = serde_json::to_string(&c).expect("a");
        let b = serde_json::to_string(&c).expect("b");
        assert_eq!(a, b);
    }

    #[test]
    fn deterministic_violation_order() {
        let c = evaluate_runtime_isolation_contract(RuntimeIsolationInput {
            io_boundary_enforced: false,
            side_effects_bounded: false,
            network_isolated: false,
        });
        let codes: Vec<&str> = c.result.violations.iter().map(|v| v.code.as_str()).collect();
        assert_eq!(
            codes,
            vec![
                "runtime_isolation:io_boundary_violation",
                "runtime_isolation:side_effect_violation",
                "runtime_isolation:network_violation"
            ]
        );
    }
}

