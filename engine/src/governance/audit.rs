//! Governance audit log (JSONL, append-only).

use serde::Serialize;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize)]
pub struct GovernanceAuditRecord {
    pub ts_epoch_secs: u64,
    pub action: String,
    pub subject: String,
    pub ok: bool,
    pub message: String,
}

fn default_audit_log_path() -> PathBuf {
    std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join(".aion")
        .join("governance_audit.log")
}

pub fn append_governance_audit(
    path: Option<&Path>,
    rec: &GovernanceAuditRecord,
) -> Result<PathBuf, String> {
    let p = path.map_or_else(default_audit_log_path, |x| x.to_path_buf());
    if let Some(parent) = p.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("audit mkdir: {e}"))?;
    }
    let mut f = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&p)
        .map_err(|e| format!("audit open: {e}"))?;
    let line = serde_json::to_string(rec).map_err(|e| format!("audit json: {e}"))?;
    writeln!(f, "{line}").map_err(|e| format!("audit write: {e}"))?;
    Ok(p)
}
