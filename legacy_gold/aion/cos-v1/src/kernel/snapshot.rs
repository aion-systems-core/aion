use super::SystemState;

#[derive(Debug, Clone)]
pub struct KernelSnapshot {
    pub state: SystemState,
}

pub trait SnapshotEngine {
    fn snapshot(&self, state: &SystemState) -> KernelSnapshot;
}
