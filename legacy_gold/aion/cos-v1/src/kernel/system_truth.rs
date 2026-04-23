use super::transaction::KernelMutation;

pub trait SystemTruth {
    fn apply_mutation(&mut self, mutation: KernelMutation) -> Result<(), String>;
    fn persist(&self) -> Result<(), String>;
}
