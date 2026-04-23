use aion_core::error::{canonical_error_json, code, io_cause, line};
use aion_engine::ai::AICapsuleV1;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

fn py_err(s: impl AsRef<str>, origin: &str) -> PyErr {
    PyRuntimeError::new_err(canonical_error_json(s.as_ref(), origin))
}

#[pyclass]
struct RunResult {
    #[pyo3(get)]
    stdout: String,
    #[pyo3(get)]
    stderr: String,
    #[pyo3(get)]
    exit_code: i32,
    #[pyo3(get)]
    duration_ms: u64,
    #[pyo3(get)]
    capsule_id: String,
    /// Set when this row comes from [`replay_capsule`].
    #[pyo3(get)]
    replay_symmetry_ok: Option<bool>,
    #[pyo3(get)]
    deterministic_hash_hex: Option<String>,
}

#[pyclass]
struct AICapsule {
    #[pyo3(get)]
    model: String,
    #[pyo3(get)]
    prompt: String,
    #[pyo3(get)]
    seed: u64,
    #[pyo3(get)]
    determinism_profile: String,
    #[pyo3(get)]
    token_trace: String,
    #[pyo3(get)]
    events: String,
    #[pyo3(get)]
    graph: String,
    #[pyo3(get)]
    why_report: String,
    #[pyo3(get)]
    drift_report: String,
    #[pyo3(get)]
    evidence_path: String,
}

#[pyclass]
struct ReplayComparison {
    #[pyo3(get)]
    tokens_equal: bool,
    #[pyo3(get)]
    trace_equal: bool,
    #[pyo3(get)]
    events_equal: bool,
    #[pyo3(get)]
    graph_equal: bool,
    #[pyo3(get)]
    why_equal: bool,
    #[pyo3(get)]
    drift_equal: bool,
    #[pyo3(get)]
    capsule_equal: bool,
    #[pyo3(get)]
    evidence_equal: bool,
    #[pyo3(get)]
    differences: Vec<String>,
}

#[pyclass]
struct DriftReport {
    #[pyo3(get)]
    changed: bool,
    #[pyo3(get)]
    fields: Vec<String>,
}

#[pyclass]
struct GovernanceReport {
    #[pyo3(get)]
    policy_ok: bool,
    #[pyo3(get)]
    determinism_ok: bool,
    #[pyo3(get)]
    integrity_ok: bool,
    #[pyo3(get)]
    violations: String,
}

fn capsule_to_py(c: &AICapsuleV1, evidence_path: impl Into<String>) -> AICapsule {
    AICapsule {
        model: c.model.clone(),
        prompt: c.prompt.clone(),
        seed: c.seed,
        determinism_profile: serde_json::to_string(&c.determinism).unwrap_or_default(),
        token_trace: serde_json::to_string(&c.token_trace).unwrap_or_default(),
        events: serde_json::to_string(&c.event_stream).unwrap_or_default(),
        graph: serde_json::to_string(&c.graph).unwrap_or_default(),
        why_report: serde_json::to_string(&c.why).unwrap_or_default(),
        drift_report: serde_json::to_string(&c.drift).unwrap_or_default(),
        evidence_path: evidence_path.into(),
    }
}

#[pyfunction]
fn run(cmd: String, args: Vec<String>, _env: Vec<(String, String)>) -> PyResult<RunResult> {
    let mut argv = vec![cmd.clone()];
    argv.extend(args);
    let rr = aion_engine::capture::capture(&argv, &aion_core::DeterminismProfile::default())
        .map_err(|e| py_err(e, "bindings"))?;
    Ok(RunResult {
        stdout: rr.stdout,
        stderr: rr.stderr,
        exit_code: rr.exit_code,
        duration_ms: rr.duration_ms,
        capsule_id: rr.run_id,
        replay_symmetry_ok: None,
        deterministic_hash_hex: None,
    })
}

#[pyfunction]
#[pyo3(signature = (model, prompt, seed, backend=None))]
fn execute_ai(
    model: String,
    prompt: String,
    seed: u64,
    backend: Option<String>,
) -> PyResult<AICapsule> {
    let b = backend.unwrap_or_else(|| "dummy".into());
    let c = aion_engine::ai::build_ai_capsule_v1_with_backend(model, prompt, seed, &b);
    Ok(capsule_to_py(&c, ""))
}

#[pyfunction]
fn capsule_deterministic_hash(path: String) -> PyResult<String> {
    let c = aion_engine::sdk::load_capsule(std::path::Path::new(&path)).map_err(|e| py_err(e, "bindings"))?;
    let h = aion_engine::capsule::deterministic_capsule_hash(&c);
    Ok(hex::encode(h))
}

#[pyfunction]
fn capsule_save(capsule: &AICapsule, path: String) -> PyResult<()> {
    let cap = aion_engine::ai::build_ai_capsule_v1(
        capsule.model.clone(),
        capsule.prompt.clone(),
        capsule.seed,
    );
    aion_engine::sdk::save_capsule(std::path::Path::new(&path), &cap).map_err(|e| py_err(e, "bindings"))
}

#[pyfunction]
fn capsule_load(path: String) -> PyResult<AICapsule> {
    let c = aion_engine::sdk::load_capsule(std::path::Path::new(&path)).map_err(|e| py_err(e, "bindings"))?;
    Ok(capsule_to_py(&c, path))
}

#[pyfunction]
fn replay_capsule(path: String) -> PyResult<RunResult> {
    let c = aion_engine::sdk::load_capsule(std::path::Path::new(&path)).map_err(|e| py_err(e, "bindings"))?;
    let hash = hex::encode(aion_engine::capsule::deterministic_capsule_hash(&c));
    let rep = aion_engine::sdk::replay_capsule(&c);
    Ok(RunResult {
        stdout: rep.replay_capsule.tokens.join(" "),
        stderr: String::new(),
        exit_code: if rep.success { 0 } else { 1 },
        duration_ms: rep.replay_duration_ms,
        capsule_id: rep.replay_capsule.evidence.run_id,
        replay_symmetry_ok: Some(rep.replay_symmetry_ok),
        deterministic_hash_hex: Some(hash),
    })
}

#[pyfunction]
fn compare_capsules(left: String, right: String) -> PyResult<ReplayComparison> {
    let a = aion_engine::sdk::load_capsule(std::path::Path::new(&left)).map_err(|e| py_err(e, "bindings"))?;
    let b = aion_engine::sdk::load_capsule(std::path::Path::new(&right)).map_err(|e| py_err(e, "bindings"))?;
    let c = aion_engine::sdk::compare_capsules(&a, &b);
    Ok(ReplayComparison {
        tokens_equal: c.tokens_equal,
        trace_equal: c.trace_equal,
        events_equal: c.events_equal,
        graph_equal: c.graph_equal,
        why_equal: c.why_equal,
        drift_equal: c.drift_equal,
        capsule_equal: c.capsule_equal,
        evidence_equal: c.evidence_equal,
        differences: c.differences,
    })
}

#[pyfunction]
fn drift_between(a: String, b: String) -> PyResult<DriftReport> {
    let ca = aion_engine::sdk::load_capsule(std::path::Path::new(&a)).map_err(|e| py_err(e, "bindings"))?;
    let cb = aion_engine::sdk::load_capsule(std::path::Path::new(&b)).map_err(|e| py_err(e, "bindings"))?;
    let d = aion_engine::sdk::drift_between(&ca, &cb);
    Ok(DriftReport {
        changed: d.changed,
        fields: d.fields,
    })
}

#[pyfunction]
fn why_explain(capsule_path: String) -> PyResult<String> {
    let c = aion_engine::sdk::load_capsule(std::path::Path::new(&capsule_path)).map_err(|e| py_err(e, "bindings"))?;
    serde_json::to_string(&aion_engine::sdk::explain_capsule(&c).why).map_err(|_| {
        py_err(
            &line(code::BINDINGS_JSON, "why_explain", "invalid_json"),
            "bindings",
        )
    })
}

#[pyfunction]
fn graph_causal(capsule_path: String) -> PyResult<String> {
    let c = aion_engine::sdk::load_capsule(std::path::Path::new(&capsule_path)).map_err(|e| py_err(e, "bindings"))?;
    serde_json::to_string(&c.graph).map_err(|_| {
        py_err(
            &line(code::BINDINGS_JSON, "graph_causal", "invalid_json"),
            "bindings",
        )
    })
}

#[pyfunction]
fn validate(capsule_path: String, policy_path: String) -> PyResult<GovernanceReport> {
    let c = aion_engine::sdk::load_capsule(std::path::Path::new(&capsule_path)).map_err(|e| py_err(e, "bindings"))?;
    let p = aion_engine::governance::load_policy(std::path::Path::new(&policy_path)).map_err(|e| py_err(e, "bindings"))?;
    let r = aion_engine::sdk::validate_capsule(
        &c,
        &p,
        &aion_engine::governance::DeterminismProfile::default(),
        &aion_engine::governance::IntegrityProfile::default(),
    );
    Ok(GovernanceReport {
        policy_ok: r.policy.ok,
        determinism_ok: r.determinism.ok,
        integrity_ok: r.integrity.ok,
        violations: serde_json::to_string(&r).unwrap_or_default(),
    })
}

#[pyfunction]
fn evidence_verify(evidence_path: String) -> PyResult<bool> {
    let body = std::fs::read_to_string(&evidence_path).map_err(|e| {
        py_err(
            &line(code::BINDINGS_IO, "evidence_verify_read", &io_cause(&e)),
            "bindings",
        )
    })?;
    let chain = serde_json::from_str::<aion_core::EvidenceChain>(&body).map_err(|_| {
        py_err(
            &line(code::BINDINGS_JSON, "evidence_verify_parse", "invalid_json"),
            "bindings",
        )
    })?;
    Ok(aion_core::verify_linear(&chain).is_ok())
}

#[pyfunction]
fn telemetry_enable() -> PyResult<bool> {
    let home = std::env::var_os("USERPROFILE")
        .or_else(|| std::env::var_os("HOME"))
        .ok_or_else(|| {
            py_err(
                &line(code::BINDINGS_HOME, "telemetry_enable", "home_unset"),
                "bindings",
            )
        })?;
    let p = std::path::PathBuf::from(home).join(".aion").join("telemetry.toml");
    if let Some(parent) = p.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    std::fs::write(&p, "enabled = true\n").map_err(|e| {
        py_err(
            &line(code::BINDINGS_IO, "telemetry_enable_write", &io_cause(&e)),
            "bindings",
        )
    })?;
    Ok(true)
}

#[pyfunction]
fn telemetry_disable() -> PyResult<bool> {
    let home = std::env::var_os("USERPROFILE")
        .or_else(|| std::env::var_os("HOME"))
        .ok_or_else(|| {
            py_err(
                &line(code::BINDINGS_HOME, "telemetry_disable", "home_unset"),
                "bindings",
            )
        })?;
    let p = std::path::PathBuf::from(home).join(".aion").join("telemetry.toml");
    if let Some(parent) = p.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    std::fs::write(&p, "enabled = false\n").map_err(|e| {
        py_err(
            &line(code::BINDINGS_IO, "telemetry_disable_write", &io_cause(&e)),
            "bindings",
        )
    })?;
    Ok(false)
}

#[pyfunction]
fn telemetry_status() -> PyResult<bool> {
    let home = std::env::var_os("USERPROFILE")
        .or_else(|| std::env::var_os("HOME"))
        .ok_or_else(|| {
            py_err(
                &line(code::BINDINGS_HOME, "telemetry_status", "home_unset"),
                "bindings",
            )
        })?;
    let p = std::path::PathBuf::from(home).join(".aion").join("telemetry.toml");
    Ok(std::fs::read_to_string(&p)
        .ok()
        .map(|s| s.contains("true"))
        .unwrap_or(false))
}

#[pyfunction]
fn setup(config_path: String) -> PyResult<()> {
    std::fs::write(config_path, "[output]\nbase = \"aion_output\"\n").map_err(|e| {
        py_err(
            &line(code::BINDINGS_IO, "setup_write", &io_cause(&e)),
            "bindings",
        )
    })
}

#[pyfunction]
fn doctor(py: Python<'_>) -> PyResult<PyObject> {
    let d = serde_json::json!({
        "ok": true,
        "cwd": std::env::current_dir().ok(),
        "version": env!("CARGO_PKG_VERSION")
    });
    let dict = pyo3::types::PyDict::new_bound(py);
    dict.set_item("ok", d["ok"].as_bool().unwrap_or(false))?;
    dict.set_item("cwd", d["cwd"].to_string())?;
    dict.set_item("version", d["version"].to_string())?;
    Ok(dict.into_py(py))
}

#[pymodule]
fn _native(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<RunResult>()?;
    m.add_class::<AICapsule>()?;
    m.add_class::<ReplayComparison>()?;
    m.add_class::<DriftReport>()?;
    m.add_class::<GovernanceReport>()?;
    m.add_function(wrap_pyfunction!(run, m)?)?;
    m.add_function(wrap_pyfunction!(execute_ai, m)?)?;
    m.add_function(wrap_pyfunction!(capsule_deterministic_hash, m)?)?;
    m.add_function(wrap_pyfunction!(capsule_save, m)?)?;
    m.add_function(wrap_pyfunction!(capsule_load, m)?)?;
    m.add_function(wrap_pyfunction!(replay_capsule, m)?)?;
    m.add_function(wrap_pyfunction!(compare_capsules, m)?)?;
    m.add_function(wrap_pyfunction!(drift_between, m)?)?;
    m.add_function(wrap_pyfunction!(why_explain, m)?)?;
    m.add_function(wrap_pyfunction!(graph_causal, m)?)?;
    m.add_function(wrap_pyfunction!(validate, m)?)?;
    m.add_function(wrap_pyfunction!(evidence_verify, m)?)?;
    m.add_function(wrap_pyfunction!(telemetry_enable, m)?)?;
    m.add_function(wrap_pyfunction!(telemetry_disable, m)?)?;
    m.add_function(wrap_pyfunction!(telemetry_status, m)?)?;
    m.add_function(wrap_pyfunction!(setup, m)?)?;
    m.add_function(wrap_pyfunction!(doctor, m)?)?;
    Ok(())
}
