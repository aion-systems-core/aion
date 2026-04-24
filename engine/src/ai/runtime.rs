//! Deterministic pseudo-LM runtime (seeded, replay-ready).

use super::events::{deterministic_token_id, sort_events_deterministic};
use super::model::AICapsuleV1;
use super::trace::{AiTokenEvent, Event};
use aion_core::DeterminismProfile;
use aion_kernel::DeterministicRng;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

const VOCAB: &[&str] = &[
    "null", "unit", "trace", "kernel", "graph", "drift", "seed", "epoch", "token", "causal",
    "audit", "capsule", "replay", "policy", "evidence", "stable", "order", "emit", "bind", "seal",
];

/// Output of one deterministic AI run (before capsule assembly).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AiRunResult {
    pub tokens: Vec<String>,
    pub token_events: Vec<AiTokenEvent>,
    pub event_stream: Vec<Event>,
    pub determinism: DeterminismProfile,
}

impl AiRunResult {
    /// Reconstruct from persisted capsule fields (for verification against a fresh runtime run).
    pub fn from_capsule(c: &AICapsuleV1) -> Self {
        Self {
            tokens: c.tokens.clone(),
            token_events: c.token_trace.clone(),
            event_stream: c.event_stream.clone(),
            determinism: c.determinism,
        }
    }
}

pub struct AiRuntime {
    model: String,
    seed: u64,
}

impl AiRuntime {
    pub fn new(model: &str, seed: u64) -> Self {
        Self {
            model: model.to_string(),
            seed,
        }
    }

    /// Deterministic token stream: length and vocabulary picks depend on `prompt`, `model`, `seed`, and `det` only.
    pub fn run(&self, prompt: &str, det: &DeterminismProfile) -> AiRunResult {
        let mut rng = DeterministicRng::new(self.seed ^ hash64(prompt) ^ hash64(&self.model));
        let n = 1 + (rng.next_u64() % 12) as usize;
        let mut tokens = Vec::with_capacity(n);
        let mut token_events = Vec::with_capacity(n);
        let mut events = Vec::new();
        events.push(Event::RunStart {
            model: self.model.clone(),
        });
        events.push(Event::PromptIngested {
            chars: prompt.chars().count(),
        });
        for i in 0..n {
            let pick = blend_pick(prompt, &self.model, i, rng.next_u64());
            let tok = VOCAB[pick % VOCAB.len()].to_string();
            let tick = i as u64;
            token_events.push(AiTokenEvent {
                index: i as u32,
                token: tok.clone(),
                token_id: deterministic_token_id(self.seed, i as u32),
                logits: None,
                timestamp: tick,
            });
            tokens.push(tok.clone());
            events.push(Event::TokenGenerated {
                index: i as u32,
                token: tok,
            });
        }
        events.push(Event::RunComplete {
            token_count: tokens.len(),
        });
        let event_stream = sort_events_deterministic(&events);
        AiRunResult {
            tokens,
            token_events,
            event_stream,
            determinism: *det,
        }
    }
}

fn hash64(s: &str) -> u64 {
    let d = Sha256::digest(s.as_bytes());
    u64::from_be_bytes(d[..8].try_into().unwrap_or([0u8; 8]))
}

fn blend_pick(prompt: &str, model: &str, i: usize, r: u64) -> usize {
    let a = hash64(prompt) as usize;
    let b = hash64(model) as usize;
    (a ^ b ^ i ^ (r as usize)) % VOCAB.len().max(1)
}
