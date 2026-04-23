//! Deterministic execution envelope capture (pre-AI freeze hooks).

use crate::env::filtered_env_for_child;
use crate::random::DeterministicRng;
use aion_core::{DeterminismProfile, ExecutionEnvelope};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::sync::OnceLock;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

static MONO_ANCHOR: OnceLock<Instant> = OnceLock::new();

/// Stable, non-reversible digest of host identity fields (cross-machine replay metadata).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MachineFingerprint {
    pub cpu_features: String,
    pub os_version: String,
    pub kernel_version: String,
    pub hostname_hash: String,
    pub binary_hash: String,
}

fn sha256_short(s: &str) -> String {
    let h = Sha256::digest(s.as_bytes());
    format!("{:x}", h)[..32].to_string()
}

/// Capture a deterministic snapshot of machine-level context (best-effort; no network I/O).
pub fn capture_machine_fingerprint() -> MachineFingerprint {
    let arch = std::env::consts::ARCH;
    let family = std::env::consts::FAMILY;
    let os = std::env::consts::OS;
    let cpu_features = format!("{arch} {family}");
    let hostname_raw = std::env::var("COMPUTERNAME")
        .or_else(|_| std::env::var("HOSTNAME"))
        .or_else(|_| std::env::var("HOST"))
        .unwrap_or_default();
    let hostname_hash = sha256_short(&hostname_raw);
    let exe_display = std::env::current_exe()
        .map(|p| p.display().to_string())
        .unwrap_or_default();
    let binary_hash = sha256_short(&exe_display);
    MachineFingerprint {
        cpu_features,
        os_version: os.into(),
        kernel_version: kernel_version_best_effort(),
        hostname_hash,
        binary_hash,
    }
}

fn kernel_version_best_effort() -> String {
    #[cfg(windows)]
    {
        std::env::var("OS").unwrap_or_else(|_| "windows".into())
    }
    #[cfg(not(windows))]
    {
        std::fs::read_to_string("/proc/version")
            .map(|s| s.lines().next().unwrap_or("").trim().to_string())
            .unwrap_or_default()
    }
}

/// Wall-clock ms since UNIX epoch plus monotonic elapsed (packed sum; capture-time only).
pub fn freeze_time_ms() -> u64 {
    let wall = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0);
    let anchor = MONO_ANCHOR.get_or_init(Instant::now);
    let mono = anchor.elapsed().as_millis() as u64;
    wall.wrapping_add(mono)
}

pub fn freeze_env() -> BTreeMap<String, String> {
    filtered_env_for_child()
}

pub fn freeze_cwd() -> String {
    std::env::current_dir()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| ".".into())
}

/// One-shot xorshift64* draw from run seed mixed with profile RNG metadata.
pub fn freeze_random(run_seed: u64, profile: &DeterminismProfile) -> u64 {
    DeterministicRng::new(run_seed ^ profile.random_seed).next_u64()
}

/// Snapshot all envelope fields immediately before AI execution.
pub fn capture_execution_envelope(profile: &DeterminismProfile, run_seed: u64) -> ExecutionEnvelope {
    let mut snap = profile.clone();
    snap.freeze_time |= snap.time_frozen;
    snap.freeze_random |= snap.syscall_intercept;
    ExecutionEnvelope {
        frozen_time_ms: freeze_time_ms(),
        frozen_env: freeze_env(),
        frozen_cwd: freeze_cwd(),
        frozen_random_seed: freeze_random(run_seed, profile),
        determinism_profile: snap,
    }
}
