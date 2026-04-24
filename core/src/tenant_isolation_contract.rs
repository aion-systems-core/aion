use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TenantIsolationViolation {
    pub code: String,
    pub origin: String,
    pub context: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TenantIsolationResult {
    pub status: String,
    pub violations: Vec<TenantIsolationViolation>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TenantIsolationContract {
    pub guard_matrix_version: String,
    pub result: TenantIsolationResult,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TenantIsolationInput {
    pub tenant_boundary_enforced: bool,
    pub cross_tenant_access_blocked: bool,
    pub token_scope_valid: bool,
}

pub fn evaluate_tenant_isolation_contract(input: TenantIsolationInput) -> TenantIsolationContract {
    let mut violations = Vec::new();
    if !input.tenant_boundary_enforced {
        violations.push(TenantIsolationViolation {
            code: "tenant_isolation:boundary_violation".to_string(),
            origin: "tenant_isolation".to_string(),
            context: "tenant_isolation.boundary".to_string(),
            cause: Some("tenant_boundary_not_enforced".to_string()),
        });
    }
    if !input.cross_tenant_access_blocked {
        violations.push(TenantIsolationViolation {
            code: "tenant_isolation:cross_tenant_violation".to_string(),
            origin: "tenant_isolation".to_string(),
            context: "tenant_isolation.cross_access".to_string(),
            cause: Some("cross_tenant_access_allowed".to_string()),
        });
    }
    if !input.token_scope_valid {
        violations.push(TenantIsolationViolation {
            code: "tenant_isolation:scope_violation".to_string(),
            origin: "tenant_isolation".to_string(),
            context: "tenant_isolation.token_scope".to_string(),
            cause: Some("token_scope_mismatch".to_string()),
        });
    }
    TenantIsolationContract {
        guard_matrix_version: "v1".to_string(),
        result: TenantIsolationResult {
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
    use super::{evaluate_tenant_isolation_contract, TenantIsolationInput};

    #[test]
    fn all_ok() {
        let c = evaluate_tenant_isolation_contract(TenantIsolationInput {
            tenant_boundary_enforced: true,
            cross_tenant_access_blocked: true,
            token_scope_valid: true,
        });
        assert_eq!(c.result.status, "ok");
    }

    #[test]
    fn deterministic_error_order() {
        let c = evaluate_tenant_isolation_contract(TenantIsolationInput {
            tenant_boundary_enforced: false,
            cross_tenant_access_blocked: false,
            token_scope_valid: false,
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
                "tenant_isolation:boundary_violation",
                "tenant_isolation:cross_tenant_violation",
                "tenant_isolation:scope_violation"
            ]
        );
    }
}
