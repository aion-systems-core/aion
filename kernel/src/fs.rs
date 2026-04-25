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

pub fn enterprise_storage_root() -> Result<std::path::PathBuf, String> {
    let cwd = std::env::current_dir().map_err(|e| format!("cwd: {e}"))?;
    let root = cwd.join("sealrun_enterprise");
    std::fs::create_dir_all(&root).map_err(|e| format!("create enterprise storage root: {e}"))?;
    Ok(root)
}
