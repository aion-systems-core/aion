//! AI Capsule v1 wire types (no dependency on graph/why logic beyond type references).

use super::graph::CausalGraphV2;
use super::trace::{AiTokenEvent, Event};
use super::why::WhyReportV2;
use aion_core::{DeterminismProfile, DriftReport, EvidenceChain, ExecutionEnvelope};
use serde::{Deserialize, Serialize};

/// Machine + runtime fingerprints captured at capsule seal time.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExecutionEnvironment {
    pub machine_fingerprint: aion_kernel::MachineFingerprint,
    pub runtime_fingerprint: crate::runtime::RuntimeFingerprint,
}

/// Deterministic, replay-ready AI execution capsule (JSON / `.aionai`).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AICapsuleV1 {
    pub version: String,
    #[serde(default)]
    pub aion_version: String,
    #[serde(default)]
    pub created_at: u64,
    #[serde(default)]
    pub policy_profile: String,
    #[serde(default)]
    pub backend_name: String,
    pub prompt: String,
    pub model: String,
    #[serde(default)]
    pub model_hash: String,
    pub seed: u64,
    pub determinism: DeterminismProfile,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_envelope: Option<ExecutionEnvelope>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_environment: Option<ExecutionEnvironment>,
    pub tokens: Vec<String>,
    pub token_trace: Vec<AiTokenEvent>,
    pub event_stream: Vec<Event>,
    pub why: WhyReportV2,
    pub graph: CausalGraphV2,
    pub drift: DriftReport,
    pub evidence: EvidenceChain,
}
