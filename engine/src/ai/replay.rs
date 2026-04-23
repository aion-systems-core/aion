//! Replay core for AION deterministic capsule validation.
//!
//! This module replays persisted capsules and emits structured mismatch contracts.
//! Invariant: replay comparisons use canonicalized fields and deterministic ordering.
//! Output structures are audit-ready and stable for machine processing.

use super::canon::{
    canonical_capsule_bytes, canonical_drift_json, canonical_graph_v2_json,
};
use super::capsule::reassemble_capsule_with_backend;
use super::drift::{drift_against_original, evidence_chains_equal_relaxed};
use super::model::AICapsuleV1;
use super::runtime::{AiRunResult, AiRuntime};
use super::why::{why_diff, WhyDiff};
use aion_core::error::{canonical_error_json, code, line, sanitize_cause};
use aion_core::DriftReport;
use serde::{Deserialize, Serialize};

fn default_replay_symmetry_ok() -> bool {
    true
}

fn replay_error_label(raw: &str) -> &'static str {
    if raw.contains("replay:profile_mismatch") {
        "replay:profile_mismatch"
    } else if raw.contains("replay:why_slice_mismatch") {
        "replay:why_slice_mismatch"
    } else if raw.contains("replay:event_stream_mismatch") {
        "replay:event_stream_mismatch"
    } else if raw.contains("replay:symmetry_failed") {
        "replay:symmetry_failed"
    } else {
        "replay:invariant_failed"
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReplayMismatchDiff {
    pub input: Vec<String>,
    pub output: Vec<String>,
    pub events: Vec<String>,
    pub evidence: Vec<String>,
}

impl ReplayMismatchDiff {
    /// Create an empty grouped mismatch container.
    ///
    /// Purpose: initialize deterministic mismatch buckets used by replay/compare paths.
    /// Invariant: all categories exist and start empty.
    /// I/O: no input -> `ReplayMismatchDiff` with empty vectors.
    /// Determinism: no side effects; fixed initialization order.
    pub fn new() -> Self {
        Self {
            input: Vec::new(),
            output: Vec::new(),
            events: Vec::new(),
            evidence: Vec::new(),
        }
    }

    /// Sort and deduplicate all mismatch categories in-place.
    ///
    /// Purpose: enforce canonical order for grouped mismatch output.
    /// Invariant: each category is lexicographically sorted and unique.
    /// I/O: mutable grouped diff -> normalized grouped diff.
    /// Determinism: fixed sort/dedup rules independent of runtime environment.
    fn normalize(&mut self) {
        self.input.sort();
        self.output.sort();
        self.events.sort();
        self.evidence.sort();
        self.input.dedup();
        self.output.dedup();
        self.events.dedup();
        self.evidence.dedup();
    }

    /// Flatten grouped mismatches into stable prefixed labels.
    ///
    /// Purpose: provide backward-compatible flat diff view with explicit category prefix.
    /// Invariant: output order is `input`, `output`, `events`, `evidence`.
    /// I/O: grouped diff -> flat `Vec<String>` labels.
    /// Determinism: category iteration and item order are fixed after normalization.
    pub fn flatten(&self) -> Vec<String> {
        let mut out = Vec::new();
        out.extend(self.input.iter().map(|d| format!("input:{d}")));
        out.extend(self.output.iter().map(|d| format!("output:{d}")));
        out.extend(self.events.iter().map(|d| format!("events:{d}")));
        out.extend(self.evidence.iter().map(|d| format!("evidence:{d}")));
        out
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReplayComparison {
    pub tokens_equal: bool,
    pub trace_equal: bool,
    pub events_equal: bool,
    pub graph_equal: bool,
    pub why_equal: bool,
    pub drift_equal: bool,
    pub capsule_equal: bool,
    pub evidence_equal: bool,
    pub model_equal: bool,
    pub prompt_equal: bool,
    pub seed_equal: bool,
    pub determinism_equal: bool,
    /// Grouped deterministic mismatch taxonomy.
    pub mismatch_diff: ReplayMismatchDiff,
    /// Flat deterministic mismatch list for compatibility.
    pub differences: Vec<String>,
}

impl ReplayComparison {
    pub fn all_product_flags(&self) -> bool {
        self.tokens_equal
            && self.trace_equal
            && self.events_equal
            && self.graph_equal
            && self.why_equal
            && self.drift_equal
            && self.capsule_equal
            && self.evidence_equal
    }

    pub fn all_equal(&self) -> bool {
        self.all_product_flags()
            && self.model_equal
            && self.prompt_equal
            && self.seed_equal
            && self.determinism_equal
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReplayReport {
    pub original_capsule: AICapsuleV1,
    pub replay_capsule: AICapsuleV1,
    pub comparison: ReplayComparison,
    pub drift_report: DriftReport,
    pub why_diff: WhyDiff,
    pub success: bool,
    /// Fresh runtime run (same model / seed / prompt) for audit.
    pub runtime_rerun: AiRunResult,
    /// Capsule-derived reconstruction (must match `runtime_rerun` when intact).
    pub reconstructed_from_capsule: AiRunResult,
    pub replay_timestamp: u64,
    pub replay_aion_version: String,
    pub replay_duration_ms: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_differing_token: Option<usize>,
    #[serde(default)]
    pub warnings: Vec<String>,
    #[serde(default = "default_replay_symmetry_ok")]
    pub replay_symmetry_ok: bool,
    #[serde(default)]
    pub replay_symmetry_error: Option<String>,
    #[serde(default)]
    pub replay_profile_error: Option<String>,
}

/// Compare two persisted capsules without runtime re-execution.
///
/// Purpose: produce deterministic equality flags and grouped mismatch taxonomy.
/// Invariant: all comparisons run on canonicalized representations where required.
/// I/O: `(left, right)` capsule pair -> `ReplayComparison`.
/// Determinism: normalized grouped labels and fixed flattening order.
pub fn compare_ai_capsules(left: &AICapsuleV1, right: &AICapsuleV1) -> ReplayComparison {
    let why_diff_report = why_diff(&left.why, &right.why);
    let mut mismatch = ReplayMismatchDiff::new();

    let model_equal = left.model == right.model;
    if !model_equal {
        mismatch.input.push("model_mismatch".into());
    }
    let prompt_equal = left.prompt == right.prompt;
    if !prompt_equal {
        mismatch.input.push("prompt_mismatch".into());
    }
    let seed_equal = left.seed == right.seed;
    if !seed_equal {
        mismatch
            .input
            .push(format!("seed_mismatch:{}:{}", left.seed, right.seed));
    }
    let determinism_equal = left.determinism == right.determinism;
    if !determinism_equal {
        mismatch.input.push("replay:profile_mismatch".into());
    }

    let tokens_equal = left.tokens == right.tokens;
    if !tokens_equal {
        mismatch.output.push("tokens_mismatch".into());
        push_token_diffs(&left.tokens, &right.tokens, &mut mismatch.output);
    }

    let trace_equal = left.token_trace == right.token_trace;
    if !trace_equal {
        mismatch.events.push("token_trace_mismatch".into());
    }

    let events_equal = left.event_stream == right.event_stream;
    if !events_equal {
        mismatch.events.push("replay:event_stream_mismatch".into());
    }

    let graph_equal = canonical_graph_v2_json(&left.graph) == canonical_graph_v2_json(&right.graph);
    if !graph_equal {
        mismatch.events.push("graph_mismatch".into());
    }

    let why_equal = !why_diff_report.changed;
    if !why_equal {
        mismatch.events.push("replay:why_slice_mismatch".into());
    }

    let drift_equal = canonical_drift_json(&left.drift) == canonical_drift_json(&right.drift);
    if !drift_equal {
        mismatch.events.push("embedded_drift_mismatch".into());
    }

    let capsule_equal = canonical_capsule_bytes(left, true, true)
        == canonical_capsule_bytes(right, true, true);
    if !capsule_equal {
        mismatch.evidence.push("capsule_canonical_mismatch".into());
    }

    let evidence_equal = evidence_chains_equal_relaxed(&left.evidence, &right.evidence);
    if !evidence_equal {
        mismatch.evidence.push("evidence_chain_mismatch".into());
    }

    mismatch.normalize();
    let differences = mismatch.flatten();

    ReplayComparison {
        tokens_equal,
        trace_equal,
        events_equal,
        graph_equal,
        why_equal,
        drift_equal,
        capsule_equal,
        evidence_equal,
        model_equal,
        prompt_equal,
        seed_equal,
        determinism_equal,
        mismatch_diff: mismatch,
        differences,
    }
}

/// Emit token-position mismatch labels for two token sequences.
///
/// Purpose: capture precise output drift positions for replay diagnostics.
/// Invariant: one label per differing index over the max sequence length.
/// I/O: `(orig, replay, out)` token arrays -> appends `token_at:*` labels.
/// Determinism: iterates indices in ascending order with stable placeholder usage.
fn push_token_diffs(orig: &[String], rep: &[String], out: &mut Vec<String>) {
    if orig == rep {
        return;
    }
    let n = orig.len().max(rep.len());
    for i in 0..n {
        let o = orig.get(i).map(String::as_str).unwrap_or("<missing>");
        let r = rep.get(i).map(String::as_str).unwrap_or("<missing>");
        if o != r {
            out.push(format!(
                "token_at:{}:{}:{}",
                i,
                sanitize_cause(o),
                sanitize_cause(r)
            ));
        }
    }
}

/// Execute full replay pipeline and return an audit-ready report.
///
/// Purpose: rerun runtime, reconstruct capsule, compare canonical artifacts, and attach mismatch/error contracts.
/// Invariant: replay symmetry/profile checks are encoded via deterministic `AION_REPLAY_*` error contracts.
/// I/O: original persisted capsule -> `ReplayReport` with flags, grouped diffs, and replay metadata.
/// Determinism: canonical compare paths, normalized labels, and stable warning ordering.
pub fn replay_ai_capsule(original: &AICapsuleV1) -> ReplayReport {
    let replay_start = std::time::Instant::now();
    let runtime_rerun = AiRuntime::new(&original.model, original.seed).run(&original.prompt, &original.determinism);
    let reconstructed_from_capsule = AiRunResult::from_capsule(original);
    let mut replay_capsule = reassemble_capsule_with_backend(
        &original.model,
        &original.prompt,
        original.seed,
        &original.backend_name,
    );
    let formal_ok = crate::replay::assert_formal_replay_invariant(original, &replay_capsule).is_ok();
    let cross_ok = crate::replay::validate_cross_machine_replay(original, &replay_capsule).is_ok();
    replay_capsule.evidence.formal_replay_invariant_ok = Some(formal_ok);
    replay_capsule.evidence.cross_machine_replay_ok = Some(cross_ok);
    let drift_report = drift_against_original(original, &replay_capsule);
    let why_diff_report = why_diff(&original.why, &replay_capsule.why);

    let replay_profile_error = original
        .determinism
        .validate_replay_profile(&replay_capsule.determinism)
        .err();
    let replay_symmetry_error = crate::replay::assert_replay_symmetry(original, &replay_capsule).err();
    let replay_symmetry_ok = replay_profile_error.is_none() && replay_symmetry_error.is_none();

    let mut mismatch = ReplayMismatchDiff::new();
    if let Some(ref e) = replay_profile_error {
        mismatch.events.push(replay_error_label(e).to_string());
    }
    if let Some(ref e) = replay_symmetry_error {
        mismatch.events.push(replay_error_label(e).to_string());
    }
    if !formal_ok {
        mismatch.evidence.push("replay:invariant_failed".into());
    }
    if !cross_ok {
        mismatch.evidence.push("replay:invariant_failed".into());
    }
    let mut warnings: Vec<String> = Vec::new();
    if original.version != "1" {
        warnings.push("replay:invariant_failed".into());
    }
    if original.evidence.records.is_empty() {
        warnings.push("replay:invariant_failed".into());
    }

    let model_equal = original.model == replay_capsule.model;
    if !model_equal {
        mismatch.input.push("model_mismatch".into());
    }
    let prompt_equal = original.prompt == replay_capsule.prompt;
    if !prompt_equal {
        mismatch.input.push("prompt_mismatch".into());
    }
    let seed_equal = original.seed == replay_capsule.seed;
    if !seed_equal {
        mismatch.input.push(format!(
            "seed_mismatch:{}:{}",
            original.seed, replay_capsule.seed
        ));
    }
    let determinism_equal = original.determinism == replay_capsule.determinism;
    if !determinism_equal {
        mismatch.input.push("replay:profile_mismatch".into());
    }

    let tokens_equal = original.tokens == replay_capsule.tokens;
    if !tokens_equal {
        mismatch.output.push("tokens_mismatch".into());
        push_token_diffs(&original.tokens, &replay_capsule.tokens, &mut mismatch.output);
    }

    let trace_equal = original.token_trace == replay_capsule.token_trace;
    if !trace_equal {
        mismatch.events.push("token_trace_mismatch".into());
    }

    let events_equal = original.event_stream == replay_capsule.event_stream;
    if !events_equal {
        mismatch.events.push("replay:event_stream_mismatch".into());
    }

    let graph_equal = canonical_graph_v2_json(&original.graph)
        == canonical_graph_v2_json(&replay_capsule.graph);
    if !graph_equal {
        mismatch.events.push("graph_mismatch".into());
    }

    let why_equal = !why_diff_report.changed;
    if !why_equal {
        mismatch.events.push("replay:why_slice_mismatch".into());
    }

    let drift_equal =
        canonical_drift_json(&original.drift) == canonical_drift_json(&replay_capsule.drift);
    if !drift_equal {
        mismatch.events.push("embedded_drift_mismatch".into());
    }

    let capsule_equal = canonical_capsule_bytes(original, true, true)
        == canonical_capsule_bytes(&replay_capsule, true, true);
    if !capsule_equal {
        mismatch.evidence.push("capsule_canonical_mismatch".into());
    }

    let evidence_equal =
        evidence_chains_equal_relaxed(&original.evidence, &replay_capsule.evidence);
    if !evidence_equal {
        mismatch.evidence.push("evidence_chain_mismatch".into());
    }

    if runtime_rerun.tokens != original.tokens {
        mismatch.output.push("runtime_tokens_mismatch".into());
    }
    if runtime_rerun.token_events != original.token_trace {
        mismatch.events.push("runtime_token_trace_mismatch".into());
    }
    if runtime_rerun.event_stream != original.event_stream {
        mismatch.events.push("replay:event_stream_mismatch".into());
    }

    mismatch.normalize();
    let differences = mismatch.flatten();
    warnings.sort();
    warnings.dedup();

    let first_differing_token = {
        let n = original.tokens.len().max(replay_capsule.tokens.len());
        let mut out = None;
        for i in 0..n {
            if original.tokens.get(i) != replay_capsule.tokens.get(i) {
                out = Some(i);
                break;
            }
        }
        out
    };

    let comparison = ReplayComparison {
        tokens_equal,
        trace_equal,
        events_equal,
        graph_equal,
        why_equal,
        drift_equal,
        capsule_equal,
        evidence_equal,
        model_equal,
        prompt_equal,
        seed_equal,
        determinism_equal,
        mismatch_diff: mismatch.clone(),
        differences: differences.clone(),
    };

    let success = comparison.all_equal() && !drift_report.changed && replay_symmetry_ok;

    ReplayReport {
        original_capsule: original.clone(),
        replay_capsule,
        comparison,
        drift_report,
        why_diff: why_diff_report,
        success,
        runtime_rerun,
        reconstructed_from_capsule,
        replay_timestamp: chrono::Utc::now().timestamp().max(0) as u64,
        replay_aion_version: include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../VERSION"))
            .trim()
            .to_string(),
        replay_duration_ms: replay_start.elapsed().as_millis() as u64,
        first_differing_token,
        warnings,
        replay_symmetry_ok,
        replay_symmetry_error: replay_symmetry_error.as_ref().map(|e| {
            canonical_error_json(
                &line(code::REPLAY_SYMMETRY, "replay_ai_capsule", &sanitize_cause(e)),
                "replay",
            )
        }),
        replay_profile_error: replay_profile_error.as_ref().map(|e| {
            canonical_error_json(
                &line(code::REPLAY_PROFILE, "replay_ai_capsule", &sanitize_cause(e)),
                "replay",
            )
        }),
    }
}
