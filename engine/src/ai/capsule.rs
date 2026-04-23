//! Build, serialize, and present AI Capsule v1.

use super::explain::explain_capsule;
use super::backend::backend_by_name;
use super::graph::CausalGraphV2;
use super::graph_render::render_causal_graph_svg;
use super::model::{AICapsuleV1, ExecutionEnvironment};
use super::runtime::AiRunResult;
use super::why::WhyReportV2;
use super::why_render::render_why_report_html;
use aion_core::{
    error::{code, io_cause, line},
    seal_run, DeterminismProfile, DriftReport, ExecutionEnvelope, PolicyProfile, RunResult,
    EXECUTION_ARTIFACT_SCHEMA_VERSION,
};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;

fn esc(s: &str) -> String {
    let mut o = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => o.push_str("&amp;"),
            '<' => o.push_str("&lt;"),
            '>' => o.push_str("&gt;"),
            '"' => o.push_str("&quot;"),
            _ => o.push(c),
        }
    }
    o
}

pub fn ai_run_id(model: &str, prompt: &str, seed: u64, stdout: &str) -> String {
    let src = format!("ai\x1f{model}\x1f{prompt}\x1f{seed}\x1f{stdout}");
    let full = format!("{:x}", Sha256::digest(src.as_bytes()));
    full.chars().take(16).collect()
}

pub fn synth_run_result(
    model: &str,
    prompt: &str,
    seed: u64,
    tokens: &[String],
    det: &DeterminismProfile,
    cwd_override: Option<&str>,
) -> RunResult {
    let stdout = tokens.join(" ");
    RunResult {
        schema_version: EXECUTION_ARTIFACT_SCHEMA_VERSION,
        run_id: ai_run_id(model, prompt, seed, &stdout),
        command: format!("ai {model}"),
        cwd: cwd_override.unwrap_or(".").to_string(),
        timestamp: det.time_epoch_secs,
        stdout,
        stderr: String::new(),
        exit_code: 0,
        duration_ms: 0,
        env_fingerprint: format!("{:016x}", seed),
    }
}

fn template_determinism(seed: u64) -> DeterminismProfile {
    DeterminismProfile {
        random_seed: seed,
        time_frozen: true,
        ..Default::default()
    }
}

/// Merge frozen envelope metadata into a runtime [`DeterminismProfile`] (exported for tests / tooling).
pub fn merge_det_from_envelope(run_seed: u64, envelope: &ExecutionEnvelope) -> DeterminismProfile {
    let mut d = envelope.determinism_profile;
    d.random_seed = run_seed;
    d.time_frozen = d.freeze_time || d.time_frozen;
    if d.time_frozen || d.freeze_time {
        d.time_epoch_secs = (envelope.frozen_time_ms / 1000).max(1);
    }
    d
}

/// Construct a full capsule from model / prompt / seed (deterministic).
pub fn build_ai_capsule_v1(model: String, prompt: String, seed: u64) -> AICapsuleV1 {
    build_ai_capsule_v1_with_backend(model, prompt, seed, "dummy")
}

pub fn build_ai_capsule_v1_with_backend(
    model: String,
    prompt: String,
    seed: u64,
    backend: &str,
) -> AICapsuleV1 {
    validate_capsule_inputs(&model, &prompt, seed)
        .unwrap_or_else(|e| panic!("invalid ai capsule input: {e}"));
    let template = template_determinism(seed);
    let envelope = aion_kernel::capture_execution_envelope(&template, seed);
    let det = merge_det_from_envelope(seed, &envelope);
    let b = backend_by_name(backend, &model);
    let out = b
        .generate(&prompt, seed, &det)
        .unwrap_or_else(|e| panic!("backend generate failed: {e}"));
    assemble_capsule(
        model,
        prompt,
        seed,
        out.run_result,
        b.name(),
        Some(envelope),
    )
}

/// Re-run the deterministic pipeline and assemble a fresh capsule (for replay verification).
pub fn reassemble_capsule(model: &str, prompt: &str, seed: u64) -> AICapsuleV1 {
    reassemble_capsule_with_backend(model, prompt, seed, "dummy")
}

pub fn reassemble_capsule_with_backend(
    model: &str,
    prompt: &str,
    seed: u64,
    backend: &str,
) -> AICapsuleV1 {
    validate_capsule_inputs(model, prompt, seed)
        .unwrap_or_else(|e| panic!("invalid ai capsule input: {e}"));
    let template = template_determinism(seed);
    let envelope = aion_kernel::capture_execution_envelope(&template, seed);
    let det = merge_det_from_envelope(seed, &envelope);
    let b = backend_by_name(backend, model);
    let out = b
        .generate(prompt, seed, &det)
        .unwrap_or_else(|e| panic!("backend generate failed: {e}"));
    assemble_capsule(
        model.into(),
        prompt.into(),
        seed,
        out.run_result,
        b.name(),
        Some(envelope),
    )
}

pub(super) fn assemble_capsule(
    model: String,
    prompt: String,
    seed: u64,
    rr: AiRunResult,
    backend_name: &str,
    execution_envelope: Option<ExecutionEnvelope>,
) -> AICapsuleV1 {
    let cwd = execution_envelope
        .as_ref()
        .map(|e| e.frozen_cwd.as_str());
    let run = synth_run_result(&model, &prompt, seed, &rr.tokens, &rr.determinism, cwd);
    let evidence = seal_run(&run, &PolicyProfile::dev(), &rr.determinism);
    let model_hash = format!("{:x}", Sha256::digest(model.as_bytes()));
    let mut c = AICapsuleV1 {
        version: "1".into(),
        aion_version: include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../VERSION"))
            .trim()
            .to_string(),
        created_at: rr.determinism.time_epoch_secs,
        policy_profile: "dev".into(),
        backend_name: backend_name.to_string(),
        prompt,
        model,
        model_hash,
        seed,
        determinism: rr.determinism,
        execution_envelope,
        execution_environment: Some(ExecutionEnvironment {
            machine_fingerprint: aion_kernel::capture_machine_fingerprint(),
            runtime_fingerprint: crate::runtime::RuntimeFingerprint::capture(),
        }),
        tokens: rr.tokens.clone(),
        token_trace: rr.token_events.clone(),
        event_stream: rr.event_stream.clone(),
        why: WhyReportV2 {
            why_schema_version: "2".into(),
            model_version: String::new(),
            seed,
            determinism_profile: String::new(),
            nodes: vec![],
            edges: vec![],
            summary: String::new(),
        },
        graph: CausalGraphV2 {
            nodes: vec![],
            edges: vec![],
        },
        drift: DriftReport::default(),
        evidence,
    };
    let exp = explain_capsule(&c);
    c.why = exp.why;
    c.graph = exp.graph;
    c
}

pub fn read_ai_capsule_v1(path: &Path) -> Result<AICapsuleV1, String> {
    let s = fs::read_to_string(path)
        .map_err(|e| line(code::CAPSULE_IO, "read_ai_capsule_v1", &io_cause(&e)))?;
    let c: AICapsuleV1 = serde_json::from_str(&s)
        .map_err(|_| line(code::CAPSULE_JSON, "read_ai_capsule_v1", "invalid_json"))?;
    validate_loaded_capsule(&c)?;
    Ok(c)
}

pub fn to_json_string(c: &AICapsuleV1) -> Result<String, String> {
    serde_json::to_string_pretty(c)
        .map_err(|_| line(code::CAPSULE_SERIALIZE, "to_json_string", "invalid_json"))
}

pub fn validate_capsule_inputs(model: &str, prompt: &str, _seed: u64) -> Result<(), String> {
    if model.trim().is_empty() {
        return Err(line(
            code::CAPSULE_INPUT,
            "validate_capsule_inputs",
            "model_empty",
        ));
    }
    if prompt.chars().count() > 131_072 {
        return Err(line(
            code::CAPSULE_INPUT,
            "validate_capsule_inputs",
            "prompt_too_long",
        ));
    }
    Ok(())
}

pub fn validate_loaded_capsule(c: &AICapsuleV1) -> Result<(), String> {
    validate_capsule_inputs(&c.model, &c.prompt, c.seed)?;
    if c.version.trim().is_empty() {
        return Err(line(
            code::CAPSULE_VALIDATE,
            "validate_loaded_capsule",
            "version_empty",
        ));
    }
    if c.tokens.len() != c.token_trace.len() {
        return Err(line(
            code::CAPSULE_VALIDATE,
            "validate_loaded_capsule",
            &format!(
                "token_trace_lenMismatch:{}:{}",
                c.tokens.len(),
                c.token_trace.len()
            ),
        ));
    }
    if c.backend_name.trim().is_empty() {
        return Err(line(
            code::CAPSULE_VALIDATE,
            "validate_loaded_capsule",
            "backend_name_empty",
        ));
    }
    Ok(())
}

pub fn render_ai_capsule_html(c: &AICapsuleV1) -> String {
    let mut page = render_why_report_html(&c.why, &c.graph);
    let tok_rows: String = c
        .token_trace
        .iter()
        .map(|e| {
            format!(
                "<tr><td>{}</td><td><code>{}</code></td><td>{}</td></tr>",
                e.index,
                esc(&e.token),
                e.timestamp
            )
        })
        .collect();
    let hdr = format!(
        "<p>model <code>{}</code> · seed <code>{}</code> · {} tokens</p><p>Prompt: {}</p>",
        esc(&c.model),
        c.seed,
        c.tokens.len(),
        esc(&c.prompt)
    );
    if let Some(i) = page.find("<body>") {
        let j = i + "<body>".len();
        page.insert_str(j, &hdr);
    }
    if !tok_rows.is_empty() {
        let trace = format!(
            r#"<h2>Token trace</h2><table><thead><tr><th>#</th><th>token</th><th>tick</th></tr></thead><tbody>{tok_rows}</tbody></table>"#
        );
        if let Some(i) = page.rfind("</body>") {
            page.insert_str(i, &trace);
        }
    }
    page
}

pub fn render_ai_capsule_svg(c: &AICapsuleV1) -> String {
    render_causal_graph_svg(&c.graph)
}
