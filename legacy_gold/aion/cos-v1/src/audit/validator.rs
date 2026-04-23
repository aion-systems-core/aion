use super::audit_chain::AuditChain;

#[derive(Debug)]
pub enum AuditResult {
    Valid,
    Invalid(String),
}

pub trait AuditReader {
    fn read_chain(&self) -> AuditChain;
}

pub trait AuditValidator {
    fn validate(&self, chain: &AuditChain) -> AuditResult;
}
