//! Unified explanation bundle (Why v2 + Graph v2).

use super::graph::{ai_causal_graph_v2, CausalGraphV2};
use super::model::AICapsuleV1;
use super::why::{why_ai_run_v2, WhyReportV2};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExplanationBundle {
    pub why: WhyReportV2,
    pub graph: CausalGraphV2,
}

pub fn explain_capsule(capsule: &AICapsuleV1) -> ExplanationBundle {
    ExplanationBundle {
        why: why_ai_run_v2(capsule),
        graph: ai_causal_graph_v2(capsule),
    }
}
