//! Filesystem policy + shallow snapshot hook (stub for v2).

use aion_core::FsSnapshot;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FsPolicy {
    ReadCwdOnly,
}

pub fn snapshot_cwd_stub(_policy: FsPolicy) -> FsSnapshot {
    FsSnapshot {
        roots: vec![".".into()],
        entries: Vec::new(),
    }
}
