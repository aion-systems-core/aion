//! **Execution boundary** — the **only** module that may read `std::env` for
//! capture material. `core::capture` is the only other `core/` module that may
//! call `std::process::Command` (subprocess I/O).
//!
//! Two tiers:
//! - **Budget slice** ([`capture_env_snapshot_15`]): `cwd` plus whitelisted
//!   keys for the legacy 15% exposure accounting and `environment_hash`.
//! - **Full snapshot** ([`capture_process_environment_full`]): entire process
//!   environment for Phase 6.5+ persistence (`ReproRun.env`); must stay in
//!   this file so OS introspection stays auditable.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::env;

/// Hard cap: whitelist + cwd must not represent more than this fraction of the
/// conceptual full-OS surface (see `OS_EXPOSURE_DENOMINATOR`).
pub const OS_EXPOSURE_RATIO: f64 = 0.15;

/// Only these OS-facing fields may ever be surfaced into capture material.
pub const ALLOWED_OS_FIELDS: &[&str] = &["cwd", "PATH", "HOME", "CI", "SHELL", "LANG"];

/// Denominator for the static budget (do not raise without re-auditing exposure).
pub const OS_EXPOSURE_DENOMINATOR: usize = 40;

const _: () = assert!(
    ALLOWED_OS_FIELDS.len() * 100 <= OS_EXPOSURE_DENOMINATOR * 15,
    "OS exposure whitelist must stay within the 15% budget"
);

/// Deterministic 15%-slice environment material embedded in [`crate::core::artifact::ExecutionArtifact`].
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EnvSnapshot15 {
    pub cwd: String,
    pub path: String,
    pub home: String,
    pub ci: String,
    pub shell: String,
    pub lang: String,
}

/// Capture cwd + whitelisted env keys only.
pub fn capture_env_snapshot_15() -> EnvSnapshot15 {
    EnvSnapshot15 {
        cwd: env::current_dir()
            .map(|p| p.to_string_lossy().into_owned())
            .unwrap_or_default(),
        path: env_var("PATH"),
        home: env_var("HOME"),
        ci: env_var("CI"),
        shell: env_var("SHELL"),
        lang: env_var("LANG"),
    }
}

/// Full process environment at capture time (`std::env::vars`), stored in
/// [`crate::core::artifact::ReproRun::env`] as a [`BTreeMap`] for stable JSON key order.
#[must_use]
pub fn capture_process_environment_full() -> BTreeMap<String, String> {
    env::vars().collect()
}

/// Stable key list embedded as [`crate::core::execution_trace::ExecutionEvent::EnvResolved`] on
/// the execution trace (15% slice field names only; order matches `ALLOWED_OS_FIELDS` minus `cwd`).
#[must_use]
pub fn env_resolved_trace_keys() -> Vec<String> {
    ALLOWED_OS_FIELDS
        .iter()
        .filter(|k| **k != "cwd")
        .map(|s| (*s).to_string())
        .collect()
}

fn env_var(key: &str) -> String {
    debug_assert!(
        key != "cwd" && ALLOWED_OS_FIELDS.contains(&key),
        "internal: env_var only for ALLOWED_OS_FIELDS (except cwd)"
    );
    env::var(key).unwrap_or_default()
}

/// Runtime check: enforced at the start of real capture (before any OS work).
pub fn assert_exposure_budget() {
    assert!(
        os_exposure_ratio() <= OS_EXPOSURE_RATIO,
        "OS exposure budget exceeded (15% limit)"
    );
}

pub fn env_snapshot_canonical(snapshot: &EnvSnapshot15) -> String {
    let mut s = String::new();
    s.push_str("cwd:");
    s.push_str(&snapshot.cwd);
    s.push('\n');
    s.push_str("PATH=");
    s.push_str(&snapshot.path);
    s.push('\n');
    s.push_str("HOME=");
    s.push_str(&snapshot.home);
    s.push('\n');
    s.push_str("CI=");
    s.push_str(&snapshot.ci);
    s.push('\n');
    s.push_str("SHELL=");
    s.push_str(&snapshot.shell);
    s.push('\n');
    s.push_str("LANG=");
    s.push_str(&snapshot.lang);
    s.push('\n');
    s
}

pub fn compute_env_hash(snapshot: &EnvSnapshot15) -> String {
    let canonical = env_snapshot_canonical(snapshot);
    format!("envsha:{}", sha256_hex_bytes(canonical.as_bytes()))
}

fn sha256_hex_bytes(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}

#[must_use]
pub fn os_exposure_ratio() -> f64 {
    ALLOWED_OS_FIELDS.len() as f64 / OS_EXPOSURE_DENOMINATOR as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exposure_budget_holds() {
        assert_exposure_budget();
        assert!((os_exposure_ratio() - OS_EXPOSURE_RATIO).abs() < 1e-9);
    }
}
