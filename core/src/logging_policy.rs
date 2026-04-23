use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogCategory {
    Audit,
    Security,
    Operational,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RetentionRule {
    pub category: LogCategory,
    pub retention_days: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LoggingPolicy {
    pub events: Vec<String>,
    pub retention: Vec<RetentionRule>,
    pub pii_guard: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LoggingComplianceResult {
    pub status: String,
    pub violations: Vec<String>,
}

pub fn evaluate_logging_policy(policy: &LoggingPolicy) -> LoggingComplianceResult {
    let mut violations = Vec::new();
    if !policy.events.iter().any(|e| e == "replay") {
        violations.push("logging_policy:replay_event_missing".to_string());
    }
    if !policy.events.iter().any(|e| e == "policy") {
        violations.push("logging_policy:policy_event_missing".to_string());
    }
    if !policy.events.iter().any(|e| e == "evidence") {
        violations.push("logging_policy:evidence_event_missing".to_string());
    }
    if policy.pii_guard != "enabled" {
        violations.push("logging_policy:pii_guard_invalid".to_string());
    }
    violations.sort();
    LoggingComplianceResult {
        status: if violations.is_empty() { "ok" } else { "error" }.to_string(),
        violations,
    }
}

#[cfg(test)]
mod tests {
    use super::{evaluate_logging_policy, LogCategory, LoggingPolicy, RetentionRule};

    #[test]
    fn positive_baseline() {
        let p = LoggingPolicy {
            events: vec![
                "replay".into(),
                "policy".into(),
                "evidence".into(),
                "errors".into(),
                "security_events".into(),
            ],
            retention: vec![
                RetentionRule { category: LogCategory::Audit, retention_days: 365 },
                RetentionRule { category: LogCategory::Security, retention_days: 365 },
                RetentionRule { category: LogCategory::Operational, retention_days: 90 },
            ],
            pii_guard: "enabled".into(),
        };
        assert_eq!(evaluate_logging_policy(&p).status, "ok");
    }

    #[test]
    fn invalid_logging_configuration_negative() {
        let p = LoggingPolicy {
            events: vec!["errors".into()],
            retention: vec![],
            pii_guard: "disabled".into(),
        };
        let r = evaluate_logging_policy(&p);
        assert_eq!(r.status, "error");
        assert!(r.violations.iter().any(|v| v == "logging_policy:pii_guard_invalid"));
    }
}

