use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ObservabilityViolation {
    pub code: String,
    pub origin: String,
    pub context: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ObservabilityResult {
    pub status: String,
    pub violations: Vec<ObservabilityViolation>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ObservabilityContract {
    pub log_schema_version: String,
    pub metrics_schema_version: String,
    pub trace_schema_version: String,
    pub result: ObservabilityResult,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ObservabilityInput {
    pub logs_deterministic: bool,
    pub metrics_deterministic: bool,
    pub traces_deterministic: bool,
}

pub fn evaluate_observability_contract(input: ObservabilityInput) -> ObservabilityContract {
    let mut violations = Vec::new();
    if !input.logs_deterministic {
        violations.push(ObservabilityViolation {
            code: "observability:log_nondeterministic".to_string(),
            origin: "observability".to_string(),
            context: "observability.logs".to_string(),
            cause: Some("log_order_unstable".to_string()),
        });
    }
    if !input.metrics_deterministic {
        violations.push(ObservabilityViolation {
            code: "observability:metric_nondeterministic".to_string(),
            origin: "observability".to_string(),
            context: "observability.metrics".to_string(),
            cause: Some("metric_sampling_unstable".to_string()),
        });
    }
    if !input.traces_deterministic {
        violations.push(ObservabilityViolation {
            code: "observability:trace_nondeterministic".to_string(),
            origin: "observability".to_string(),
            context: "observability.traces".to_string(),
            cause: Some("trace_id_or_order_unstable".to_string()),
        });
    }
    ObservabilityContract {
        log_schema_version: "v1".to_string(),
        metrics_schema_version: "v1".to_string(),
        trace_schema_version: "v1".to_string(),
        result: ObservabilityResult {
            status: if violations.is_empty() { "ok".into() } else { "error".into() },
            violations,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{evaluate_observability_contract, ObservabilityInput};

    #[test]
    fn all_ok() {
        let c = evaluate_observability_contract(ObservabilityInput {
            logs_deterministic: true,
            metrics_deterministic: true,
            traces_deterministic: true,
        });
        assert_eq!(c.result.status, "ok");
    }

    #[test]
    fn deterministic_violations() {
        let c = evaluate_observability_contract(ObservabilityInput {
            logs_deterministic: false,
            metrics_deterministic: false,
            traces_deterministic: false,
        });
        let codes: Vec<&str> = c.result.violations.iter().map(|v| v.code.as_str()).collect();
        assert_eq!(
            codes,
            vec![
                "observability:log_nondeterministic",
                "observability:metric_nondeterministic",
                "observability:trace_nondeterministic"
            ]
        );
    }
}

