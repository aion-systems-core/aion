use super::{ExecutionEnvelope, SystemState};
use crate::spec::WorkflowSpec;

pub type KernelResult = Result<KernelExecutionOutcome, KernelError>;

#[derive(Debug)]
pub struct KernelExecutionOutcome {
    pub final_state: SystemState,
}

#[derive(Debug, thiserror::Error)]
pub enum KernelError {
    #[error("kernel execution failed: {0}")]
    ExecutionFailed(String),
}

pub struct ExecutionContext {
    pub workflow: WorkflowSpec,
    pub envelope: ExecutionEnvelope,
}

pub trait KernelExecutionEngine {
    fn execute(&mut self, ctx: ExecutionContext) -> KernelResult;
}
