//! Cross-machine replay invariants for deterministic capsule portability.
//!
//! This module separates strict runtime checks from tolerant machine-envelope checks.
//! Invariant: deterministic semantics and runtime fingerprint stay stable across hosts.
//! Host variance is allowed only in approved machine fingerprint dimensions.

use crate::ai::AICapsuleV1;
use crate::capsule::deterministic_capsule_hash;

/// Compare machine fingerprints with explicit tolerant rules.
///
/// Purpose: allow known-safe host variance while preserving replay guarantees.
/// Invariant: CPU features, OS version, and kernel version must match exactly.
/// I/O: `(a, b)` machine fingerprints -> compatibility boolean.
/// Determinism: pure field equality; no environment-dependent branching.
pub fn machine_fingerprint_tolerant_equal(a: &aion_kernel::MachineFingerprint, b: &aion_kernel::MachineFingerprint) -> bool {
    a.cpu_features == b.cpu_features
        && a.os_version == b.os_version
        && a.kernel_version == b.kernel_version
}

/// Validate deterministic replay across different machines.
///
/// Purpose: enforce semantic/hash/runtime invariants and bounded machine compatibility.
/// Invariant: determinism profile/hash/trace/events always match; runtime fingerprint is strict.
/// I/O: `(original, replayed)` capsule pair -> `Ok(())` or stable mismatch token.
/// Determinism: check order and comparison criteria are fixed.
pub fn validate_cross_machine_replay(original: &AICapsuleV1, replayed: &AICapsuleV1) -> Result<(), String> {
    if original.determinism != replayed.determinism {
        return Err("cross_machine: determinism_profile mismatch".into());
    }
    if deterministic_capsule_hash(original) != deterministic_capsule_hash(replayed) {
        return Err("cross_machine: capsule_hash mismatch".into());
    }
    if original.token_trace != replayed.token_trace {
        return Err("cross_machine: token_trace mismatch".into());
    }
    if original.event_stream != replayed.event_stream {
        return Err("cross_machine: event_stream mismatch".into());
    }
    match (&original.execution_environment, &replayed.execution_environment) {
        (Some(o), Some(r)) => {
            if o.runtime_fingerprint != r.runtime_fingerprint {
                return Err("cross_machine: runtime_fingerprint mismatch (strict)".into());
            }
            if !machine_fingerprint_tolerant_equal(&o.machine_fingerprint, &r.machine_fingerprint) {
                return Err("cross_machine: machine_fingerprint incompatible (tolerant CPU/OS check failed)".into());
            }
        }
        (None, None) => {}
        _ => {
            return Err("cross_machine: execution_environment presence mismatch".into());
        }
    }
    Ok(())
}
