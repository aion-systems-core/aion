use super::{ExecutionEnvelope, Step};

#[derive(Debug)]
pub enum EnforcementResult {
    Allowed,
    Denied(String),
}

pub trait EnforcementEngine {
    fn check(&self, step: &Step, ctx: &ExecutionEnvelope) -> EnforcementResult;
}
