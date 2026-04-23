//! Why v2 + causal graph explanation bundle.

use crate::ai::explain_capsule as engine_explain;
use crate::ai::why_diff as engine_why_diff;
use crate::ai::{AICapsuleV1, ExplanationBundle, WhyDiff, WhyReportV2};

pub fn explain_capsule(capsule: &AICapsuleV1) -> ExplanationBundle {
    engine_explain(capsule)
}

pub fn why_diff(a: &WhyReportV2, b: &WhyReportV2) -> WhyDiff {
    engine_why_diff(a, b)
}
