//! Deterministic syscall whitelist and IO policy enforcement (additive layer).

use aion_core::DeterministicIOPolicy;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// Syscall surface permitted under deterministic IO policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SyscallName {
    Read,
    Write,
    Open,
    Stat,
    ClockGettime,
}

impl SyscallName {
    pub fn as_str(self) -> &'static str {
        match self {
            SyscallName::Read => "read",
            SyscallName::Write => "write",
            SyscallName::Open => "open",
            SyscallName::Stat => "stat",
            SyscallName::ClockGettime => "clock_gettime",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "read" => Some(SyscallName::Read),
            "write" => Some(SyscallName::Write),
            "open" => Some(SyscallName::Open),
            "stat" | "fstat" | "lstat" => Some(SyscallName::Stat),
            "clock_gettime" => Some(SyscallName::ClockGettime),
            _ => None,
        }
    }
}

/// Canonical filesystem path for stable stat / open replay.
pub fn canonicalize_fs_path(path: &str) -> String {
    let mut s = path.replace('\\', "/");
    while s.contains("//") {
        s = s.replace("//", "/");
    }
    s
}

/// `open` is allowed only when args indicate read-only access.
pub fn open_is_read_only(args: &Value) -> bool {
    let flags = args
        .get("flags")
        .and_then(|v| v.as_str())
        .unwrap_or("O_RDONLY");
    let f = flags.to_uppercase();
    !f.contains("O_WRONLY") && !f.contains("O_RDWR") && !f.contains("O_CREAT")
}

/// Returns `Ok(())` when the syscall is permitted; `Err(reason)` when it must be denied under policy.
pub fn evaluate_syscall(
    policy: DeterministicIOPolicy,
    name: SyscallName,
    args: &Value,
) -> Result<(), String> {
    let allowed = matches!(
        name,
        SyscallName::Read | SyscallName::Write | SyscallName::Stat | SyscallName::ClockGettime
    ) || (name == SyscallName::Open && open_is_read_only(args));

    if !allowed {
        return Err(format!(
            "{} is not permitted on this deterministic IO surface",
            name.as_str()
        ));
    }

    match policy {
        DeterministicIOPolicy::Audit => Ok(()),
        DeterministicIOPolicy::Strict | DeterministicIOPolicy::Deny => Ok(()),
    }
}

/// Decide whether a syscall that failed `evaluate_syscall` should hard-fail the run.
pub fn should_block(policy: DeterministicIOPolicy) -> bool {
    matches!(
        policy,
        DeterministicIOPolicy::Strict | DeterministicIOPolicy::Deny
    )
}

/// JSON payload for governance / capsule event stream.
pub fn policy_violation_value(syscall: &str, reason: &str) -> Value {
    json!({
        "syscall": syscall,
        "reason": reason,
        "severity": "error",
    })
}
