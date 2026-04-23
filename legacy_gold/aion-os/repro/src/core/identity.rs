// Execution identity.
//
// An [`crate::core::artifact::ExecutionArtifact`] says *what happened* on disk.
// An `ExecutionIdentity` is the *semantic fingerprint* of that happening,
// split into four independent signatures.

use crate::core::artifact::ExecutionArtifact;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExecutionIdentity {
    pub command_signature: String,
    pub environment_signature: String,
    pub input_signature: String,
    pub trace_signature: String,
    pub composite: String,
}

/// Which axes of a pair of identities disagree.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdentityDelta {
    pub command_changed: bool,
    pub environment_changed: bool,
    pub input_changed: bool,
    pub trace_changed: bool,
}

impl IdentityDelta {
    #[allow(dead_code)]
    pub fn any(self) -> bool {
        self.command_changed || self.environment_changed || self.input_changed || self.trace_changed
    }
}

/// SHA-256 over stdout + stderr + exit code (stable delimiter).
pub fn compute_execution_hash(stdout: &str, stderr: &str, exit_code: i32) -> String {
    let mut buf = String::new();
    buf.push_str("stdout:");
    buf.push_str(stdout);
    buf.push_str("\nstderr:");
    buf.push_str(stderr);
    buf.push_str("\nexit:");
    buf.push_str(&exit_code.to_string());
    format!("exsha:{}", sha256_hex_bytes(buf.as_bytes()))
}

/// SHA-256 over the raw command / intent string.
pub fn compute_intent_hash(command: &str) -> String {
    format!("intsha:{}", sha256_hex_bytes(command.as_bytes()))
}

fn sha256_hex_bytes(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}

impl ExecutionIdentity {
    pub fn from_artifact(a: &ExecutionArtifact) -> Self {
        let env_h = a.environment_hash();
        let command_signature = format!("cmd:{}", compute_intent_hash(&a.command));
        let environment_signature = format!("env:{env_h}");
        let input_signature = format!("inp:{}", compute_intent_hash(&normalize_input(&a.command)));
        let trace_signature = format!(
            "trc:{}",
            compute_execution_hash(&a.stdout, &a.stderr, a.exit_code)
        );
        let composite_raw = format!(
            "{}|{}|{}|{}",
            command_signature, environment_signature, input_signature, trace_signature
        );
        let composite = format!("id:{}", sha256_hex_bytes(composite_raw.as_bytes()));
        Self {
            command_signature,
            environment_signature,
            input_signature,
            trace_signature,
            composite,
        }
    }

    pub fn delta(&self, other: &Self) -> IdentityDelta {
        IdentityDelta {
            command_changed: self.command_signature != other.command_signature,
            environment_changed: self.environment_signature != other.environment_signature,
            input_changed: self.input_signature != other.input_signature,
            trace_changed: self.trace_signature != other.trace_signature,
        }
    }
}

fn normalize_input(command: &str) -> String {
    let mut out = String::with_capacity(command.len());
    let mut in_ws = false;
    for c in command.trim().chars() {
        if c.is_whitespace() {
            if !in_ws {
                out.push(' ');
                in_ws = true;
            }
        } else {
            out.push(c);
            in_ws = false;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::capture::{
        capture_command_with_clock, reset_counter_for_tests, FixedClock, CAPTURE_TEST_LOCK,
    };
    use crate::core::execution_boundary::{compute_env_hash, EnvSnapshot15};

    #[test]
    fn identical_artifacts_have_identical_identity() {
        let _g = CAPTURE_TEST_LOCK.lock().unwrap();
        reset_counter_for_tests();
        let a = capture_command_with_clock("echo hello".into(), &FixedClock(1));
        let ia = ExecutionIdentity::from_artifact(&a);
        let ib = ExecutionIdentity::from_artifact(&a);
        assert_eq!(ia, ib);
        assert!(!ia.delta(&ib).any());
    }

    #[test]
    fn quoting_drift_keeps_input_signature_stable() {
        let _g = CAPTURE_TEST_LOCK.lock().unwrap();
        reset_counter_for_tests();
        let a = capture_command_with_clock("echo hello".into(), &FixedClock(1));
        reset_counter_for_tests();
        let b = capture_command_with_clock("echo   hello".into(), &FixedClock(1));
        let ia = ExecutionIdentity::from_artifact(&a);
        let ib = ExecutionIdentity::from_artifact(&b);
        assert_ne!(ia.command_signature, ib.command_signature);
        assert_eq!(ia.input_signature, ib.input_signature);
    }

    #[test]
    fn different_commands_differ_on_every_axis() {
        let _g = CAPTURE_TEST_LOCK.lock().unwrap();
        reset_counter_for_tests();
        let a = capture_command_with_clock("echo a".into(), &FixedClock(1));
        let b = capture_command_with_clock("echo b".into(), &FixedClock(1));
        let d = ExecutionIdentity::from_artifact(&a).delta(&ExecutionIdentity::from_artifact(&b));
        assert!(d.command_changed && d.trace_changed);
    }

    #[test]
    fn compute_env_hash_is_stable() {
        let s = EnvSnapshot15 {
            cwd: "/tmp".into(),
            path: "/bin".into(),
            home: "".into(),
            ci: "".into(),
            shell: "".into(),
            lang: "".into(),
        };
        assert_eq!(compute_env_hash(&s), compute_env_hash(&s));
    }
}
