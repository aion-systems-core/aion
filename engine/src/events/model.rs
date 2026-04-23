//! Canonical run event model: ordered, JSON-serializable, kernel-agnostic.

use aion_core::{CapsuleManifest, RunResult};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// On-disk / wire schema for the engine event stream.
pub const EVENT_STREAM_SCHEMA_V2: &str = "aion/event_stream/v2";

/// Stable category ordering for deterministic tie-breaks (after `seq`).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EventCategory {
    Capsule,
    Env,
    Exit,
    Fs,
    Network,
    Process,
    Random,
    Syscall,
    Time,
}

/// One logical event in a run timeline.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RunEvent {
    /// Syscall interception or audit hook (no raw pointers).
    SyscallIntercept {
        syscall: String,
        outcome: String,
    },
    /// Filesystem policy or observed path operation.
    FsOperation {
        op: String,
        path: String,
        note: String,
    },
    /// Network policy or observed connect/bind/send (logical labels only).
    NetworkActivity {
        op: String,
        peer: String,
    },
    /// Environment fingerprint or single-key snapshot.
    EnvSnapshot {
        fingerprint: String,
    },
    /// Wall-clock or frozen epoch for the slice.
    TimeSnapshot {
        epoch_secs: u64,
    },
    /// Deterministic RNG / seed state.
    RandomState {
        label: String,
        seed: u64,
    },
    /// Captured stdout segment (may be full run in v2 capture).
    StdoutChunk {
        text: String,
    },
    /// Captured stderr segment.
    StderrChunk {
        text: String,
    },
    /// Process exit.
    ExitStatus {
        code: i32,
    },
    /// Command + cwd anchor.
    RunBoundaries {
        command: String,
        cwd: String,
    },
    /// Capsule manifest linkage (after seal).
    CapsuleAttached {
        capsule_schema_version: u32,
        execution_artifact_schema_version: u32,
        policy_name: String,
        time_frozen: bool,
        time_epoch_secs: u64,
        random_seed: u64,
    },
}

impl RunEvent {
    pub fn category(&self) -> EventCategory {
        match self {
            RunEvent::SyscallIntercept { .. } => EventCategory::Syscall,
            RunEvent::FsOperation { .. } => EventCategory::Fs,
            RunEvent::NetworkActivity { .. } => EventCategory::Network,
            RunEvent::EnvSnapshot { .. } => EventCategory::Env,
            RunEvent::TimeSnapshot { .. } => EventCategory::Time,
            RunEvent::RandomState { .. } => EventCategory::Random,
            RunEvent::StdoutChunk { .. } | RunEvent::StderrChunk { .. } => EventCategory::Process,
            RunEvent::ExitStatus { .. } => EventCategory::Exit,
            RunEvent::RunBoundaries { .. } => EventCategory::Process,
            RunEvent::CapsuleAttached { .. } => EventCategory::Capsule,
        }
    }
}

/// Single append-only record: `seq` is the primary total order key.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EventEnvelope {
    pub seq: u64,
    /// Deterministic extension bag (sorted keys on serialize via `BTreeMap`).
    pub attrs: BTreeMap<String, String>,
    pub event: RunEvent,
}

impl EventEnvelope {
    pub fn new(seq: u64, event: RunEvent) -> Self {
        Self {
            seq,
            attrs: BTreeMap::new(),
            event,
        }
    }

    pub fn with_attr(mut self, k: impl Into<String>, v: impl Into<String>) -> Self {
        self.attrs.insert(k.into(), v.into());
        self
    }
}

/// Wire container written next to artifacts or embedded in sidecars.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EventStreamFile {
    pub schema: String,
    pub run_id: String,
    pub events: Vec<EventEnvelope>,
}

impl EventStreamFile {
    pub fn validate(&self) -> Result<(), String> {
        if self.schema != EVENT_STREAM_SCHEMA_V2 {
            return Err(format!("unsupported event stream schema: {}", self.schema));
        }
        let mut prev: Option<u64> = None;
        for e in &self.events {
            if let Some(p) = prev {
                if e.seq <= p {
                    return Err(format!(
                        "events not strictly ordered by seq: {} then {}",
                        p, e.seq
                    ));
                }
            }
            prev = Some(e.seq);
        }
        Ok(())
    }
}

/// Build the default v2 timeline from a [`RunResult`] and optional [`CapsuleManifest`].
pub fn envelopes_from_run(
    run: &RunResult,
    manifest: Option<&CapsuleManifest>,
) -> Vec<EventEnvelope> {
    let mut seq = 0u64;
    let mut out = Vec::new();
    out.push(EventEnvelope::new(
        seq,
        RunEvent::RunBoundaries {
            command: run.command.clone(),
            cwd: run.cwd.clone(),
        },
    ));
    seq += 1;
    out.push(EventEnvelope::new(
        seq,
        RunEvent::EnvSnapshot {
            fingerprint: run.env_fingerprint.clone(),
        },
    ));
    seq += 1;
    out.push(EventEnvelope::new(
        seq,
        RunEvent::TimeSnapshot {
            epoch_secs: run.timestamp,
        },
    ));
    seq += 1;
    out.push(EventEnvelope::new(
        seq,
        RunEvent::StdoutChunk {
            text: run.stdout.clone(),
        },
    ));
    seq += 1;
    out.push(EventEnvelope::new(
        seq,
        RunEvent::StderrChunk {
            text: run.stderr.clone(),
        },
    ));
    seq += 1;
    out.push(EventEnvelope::new(
        seq,
        RunEvent::ExitStatus {
            code: run.exit_code,
        },
    ));
    if let Some(m) = manifest {
        seq += 1;
        out.push(EventEnvelope::new(
            seq,
            RunEvent::CapsuleAttached {
                capsule_schema_version: m.capsule_schema_version,
                execution_artifact_schema_version: m.execution_artifact_schema_version,
                policy_name: m.policy.name.clone(),
                time_frozen: m.determinism.time_frozen,
                time_epoch_secs: m.determinism.time_epoch_secs,
                random_seed: m.determinism.random_seed,
            },
        ));
    }
    out
}
