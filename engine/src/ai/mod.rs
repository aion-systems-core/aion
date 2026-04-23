//! AION-AI Capsule v1 — deterministic, replay-ready AI execution format.

pub(crate) mod canon;
mod backend;
mod capsule;
mod drift;
pub(crate) mod events;
mod explain;
mod graph;
mod graph_render;
mod model;
mod replay;
mod replay_render;
mod runtime;
mod trace;
mod why;
mod why_render;

pub use capsule::{
    ai_run_id, build_ai_capsule_v1, build_ai_capsule_v1_with_backend, merge_det_from_envelope,
    read_ai_capsule_v1, reassemble_capsule, reassemble_capsule_with_backend, render_ai_capsule_html,
    render_ai_capsule_svg, synth_run_result, to_json_string as ai_capsule_to_json,
    validate_capsule_inputs, validate_loaded_capsule,
};
pub use backend::{backend_by_name, DummyBackend, LlmBackend, LlmOutput};
pub use drift::{
    drift_against_original, drift_between_runs, drift_between_runs_full, evidence_chains_equal_relaxed,
};
pub use events::{
    canonical_events_for_compare, canonical_events_for_hash, canonical_token_trace_for_hash,
    deterministic_event_id, deterministic_token_id, sort_events_deterministic,
    token_events_semantic_equal,
};
pub use explain::{explain_capsule, ExplanationBundle};
pub use graph::{
    ai_causal_graph_json, ai_causal_graph_v2, build_causal_graph_v2, CausalGraphV2, GraphEdge,
    GraphNode, GraphNodeKind,
};
pub use graph_render::render_causal_graph_svg;
pub use model::{AICapsuleV1, ExecutionEnvironment};
pub use replay::{compare_ai_capsules, replay_ai_capsule, ReplayComparison, ReplayReport};
pub use replay_render::{render_replay_graph_svg, render_replay_report_html};
pub use runtime::{AiRunResult, AiRuntime};
pub use trace::{AiTokenEvent, Event};
pub use why::{
    build_why_report_v2, why_ai_run_v2, why_diff, WhyDiff, WhyEdge, WhyNode, WhyNodeKind,
    WhyReportV2,
};
pub use why_render::{
    render_why_diff_html, render_why_diff_svg, render_why_report_html,
};
