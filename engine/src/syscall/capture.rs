//! Record syscalls as capsule timeline events.

use super::policy::{canonicalize_fs_path, evaluate_syscall, should_block};
use crate::ai::Event;
use aion_core::DeterministicIOPolicy;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// One captured syscall with deterministic replay metadata.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SyscallEvent {
    pub id: u64,
    pub name: String,
    pub args: Value,
    pub result: Value,
    pub deterministic: bool,
}

impl SyscallEvent {
    pub fn to_event(&self) -> Event {
        Event::SyscallCaptured {
            id: self.id,
            name: self.name.clone(),
            args: self.args.clone(),
            result: self.result.clone(),
            deterministic: self.deterministic,
        }
    }
}

/// Capture a syscall under policy; may append a [`Event::PolicyViolation`] to `violations_out`.
pub fn capture_syscall(
    policy: DeterministicIOPolicy,
    id: u64,
    raw_name: &str,
    mut args: Value,
    result: Value,
    violations_out: &mut Vec<Event>,
) -> Result<SyscallEvent, String> {
    let name_lc = raw_name.to_ascii_lowercase();
    if name_lc == "stat" || name_lc == "open" {
        if let Some(p) = args.get("path").and_then(|v| v.as_str()) {
            let c = canonicalize_fs_path(p);
            if let Value::Object(ref mut m) = args {
                m.insert("path".into(), json!(c));
            }
        }
    }

    let parsed = match super::policy::SyscallName::parse(&name_lc) {
        Some(p) => p,
        None => {
            let reason = format!("syscall {raw_name} is outside the deterministic syscall whitelist");
            violations_out.push(Event::PolicyViolation {
                syscall: raw_name.into(),
                reason: reason.clone(),
                severity: "error".into(),
            });
            if should_block(policy) {
                return Err(reason);
            }
            return Ok(SyscallEvent {
                id,
                name: name_lc,
                args,
                result,
                deterministic: false,
            });
        }
    };

    if let Err(reason) = evaluate_syscall(policy, parsed, &args) {
        violations_out.push(Event::PolicyViolation {
            syscall: raw_name.into(),
            reason: reason.clone(),
            severity: "error".into(),
        });
        if should_block(policy) {
            return Err(reason);
        }
    }

    Ok(SyscallEvent {
        id,
        name: name_lc,
        args,
        result,
        deterministic: true,
    })
}
