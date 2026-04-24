use super::graph::CausalGraphV2;
use super::runtime::{AiRunResult, AiRuntime};
use super::trace::Event;
use super::why::WhyReportV2;
use aion_core::DeterminismProfile;

pub trait LlmBackend: Send + Sync {
    fn generate(
        &self,
        prompt: &str,
        seed: u64,
        determinism_profile: &DeterminismProfile,
    ) -> Result<LlmOutput, String>;
    fn name(&self) -> &str;
    fn supports_deterministic(&self) -> bool;
}

#[derive(Debug, Clone)]
pub struct LlmOutput {
    pub tokens: Vec<String>,
    pub events: Vec<Event>,
    pub graph: CausalGraphV2,
    pub why_report: WhyReportV2,
    pub run_result: AiRunResult,
}

/// Deterministic stub backend; `runtime_model` must match the capsule model id for replay parity.
#[derive(Debug, Clone)]
pub struct DummyBackend {
    pub runtime_model: String,
}

impl DummyBackend {
    pub fn new(runtime_model: impl Into<String>) -> Self {
        Self {
            runtime_model: runtime_model.into(),
        }
    }
}

impl Default for DummyBackend {
    fn default() -> Self {
        Self {
            runtime_model: "dummy".into(),
        }
    }
}

impl LlmBackend for DummyBackend {
    fn generate(
        &self,
        prompt: &str,
        seed: u64,
        determinism_profile: &DeterminismProfile,
    ) -> Result<LlmOutput, String> {
        let rt = AiRuntime::new(&self.runtime_model, seed);
        let rr = rt.run(prompt, determinism_profile);
        Ok(LlmOutput {
            tokens: rr.tokens.clone(),
            events: rr.event_stream.clone(),
            graph: CausalGraphV2 {
                nodes: vec![],
                edges: vec![],
            },
            why_report: WhyReportV2 {
                why_schema_version: "2".into(),
                model_version: self.name().into(),
                seed,
                determinism_profile: "frozen".into(),
                nodes: vec![],
                edges: vec![],
                summary: String::new(),
            },
            run_result: rr,
        })
    }

    fn name(&self) -> &str {
        "dummy"
    }

    fn supports_deterministic(&self) -> bool {
        true
    }
}

pub fn backend_by_name(_name: &str, runtime_model: &str) -> Box<dyn LlmBackend> {
    // v1: only dummy backend; `_name` reserved for future model routing.
    Box::new(DummyBackend::new(runtime_model))
}
