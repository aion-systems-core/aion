use crate::ai;
use crate::governance::{self, DeterminismProfile, IntegrityProfile};
use aion_core::error::{canonical_error_json, code, io_cause, line};
use libc::{c_char, size_t};
use std::ffi::{CStr, CString};
use std::path::Path;
use std::ptr;
use std::sync::{Mutex, OnceLock};

pub const AION_OK: i32 = 0;
pub const AION_ERR_GENERIC: i32 = 1;
pub const AION_ERR_CAPSULE_NOT_FOUND: i32 = 2;
pub const AION_ERR_CAPSULE_CORRUPT: i32 = 3;
pub const AION_ERR_INVALID_POLICY: i32 = 4;
pub const AION_ERR_DETERMINISM_FAILURE: i32 = 5;
pub const AION_ERR_INTEGRITY_FAILURE: i32 = 6;
pub const AION_ERR_EVIDENCE_INVALID: i32 = 7;
pub const AION_ERR_IO: i32 = 8;
pub const AION_ERR_OUT_OF_MEMORY: i32 = 9;
pub const AION_ERR_UNSUPPORTED_VERSION: i32 = 10;

#[repr(C)]
pub struct AionRunResult {
    pub stdout_data: *mut c_char,
    pub stdout_len: size_t,
    pub stderr_data: *mut c_char,
    pub stderr_len: size_t,
    pub exit_code: i32,
    pub duration_ms: u64,
    pub capsule_id: *mut c_char,
}

#[repr(C)]
pub struct AionCapsule {
    pub model: *mut c_char,
    pub prompt: *mut c_char,
    pub seed: u64,
    pub determinism_profile_json: *mut c_char,
    pub token_trace_json: *mut c_char,
    pub events_json: *mut c_char,
    pub graph_json: *mut c_char,
    pub why_report_json: *mut c_char,
    pub drift_report_json: *mut c_char,
    pub evidence_path: *mut c_char,
}

#[repr(C)]
pub struct AionReplayComparison {
    pub tokens_equal: u8,
    pub trace_equal: u8,
    pub events_equal: u8,
    pub graph_equal: u8,
    pub why_equal: u8,
    pub drift_equal: u8,
    pub capsule_equal: u8,
    pub evidence_equal: u8,
    pub differences: *mut *mut c_char,
    pub differences_count: size_t,
}

#[repr(C)]
pub struct AionDriftReport {
    pub changed: u8,
    pub fields_json: *mut c_char,
}

#[repr(C)]
pub struct AionGovernanceReport {
    pub policy_ok: u8,
    pub determinism_ok: u8,
    pub integrity_ok: u8,
    pub violations_json: *mut c_char,
}

const FFI_IDLE_JSON: &str = r#"{"schema_version":1,"code":"AION_FFI_IDLE","message":"ffi_idle","context":"aion_last_error","origin":"ffi"}"#;

fn last_error_store() -> &'static Mutex<CString> {
    static LAST: OnceLock<Mutex<CString>> = OnceLock::new();
    LAST.get_or_init(|| Mutex::new(CString::new(FFI_IDLE_JSON).expect("cstr")))
}

fn set_last_error(msg: impl AsRef<str>) -> i32 {
    let j = canonical_error_json(msg.as_ref(), "ffi");
    let clean = j.replace('\0', " ");
    if let Ok(mut g) = last_error_store().lock() {
        *g = CString::new(clean).unwrap_or_else(|_| CString::new(FFI_IDLE_JSON).expect("cstr"));
    }
    AION_ERR_GENERIC
}

fn cstr_to_string(ptr: *const c_char, name: &str) -> Result<String, i32> {
    if ptr.is_null() {
        return Err(set_last_error(line(
            code::FFI_NULL_ARG,
            "cstr_to_string",
            name,
        )));
    }
    let s = unsafe { CStr::from_ptr(ptr) }
        .to_str()
        .map_err(|_| set_last_error(line(code::FFI_UTF8, "cstr_to_string", name)))?;
    Ok(s.to_string())
}

fn to_c_owned(s: String) -> Result<*mut c_char, i32> {
    CString::new(s)
        .map(|x| x.into_raw())
        .map_err(|_| {
            set_last_error(line(
                code::FFI_CSTRING_INTERIOR_NULL,
                "to_c_owned",
                "interior_null",
            ))
        })
}

fn extract_primary_code(e: &str) -> String {
    let t = e.trim_start();
    if t.starts_with('{') {
        if let Ok(ae) = serde_json::from_str::<aion_core::error::AionError>(t) {
            return ae.code;
        }
    }
    t.split('|').next().unwrap_or("").to_string()
}

fn map_err_code(e: &str) -> i32 {
    let code = extract_primary_code(e);
    match code.as_str() {
        "AION_CAPSULE_IO" => {
            if e.contains("NotFound") {
                AION_ERR_CAPSULE_NOT_FOUND
            } else {
                AION_ERR_IO
            }
        }
        "AION_CAPSULE_JSON" | "AION_CAPSULE_SERIALIZE" | "AION_CAPSULE_VALIDATE"
        | "AION_CAPSULE_INPUT" => AION_ERR_CAPSULE_CORRUPT,
        "AION_CAPSULE_SAVE_EXISTS" | "AION_CAPSULE_SAVE_IO" | "AION_CAPSULE_SAVE_MKDIR" => {
            AION_ERR_IO
        }
        "AION_OUTPUT_AIONAI_EXISTS" | "AION_OUTPUT_JSON_SERIALIZE" => AION_ERR_IO,
        "AION_SDK_PARSE" | "AION_SDK_VALIDATION" | "AION_SDK_VERSION" => {
            AION_ERR_CAPSULE_CORRUPT
        }
        "AION_SDK_IO" => AION_ERR_IO,
        "AION_CAPTURE_EMPTY" => AION_ERR_GENERIC,
        "AION_GOVERNANCE_IO" | "AION_GOVERNANCE_JSON" => AION_ERR_INVALID_POLICY,
        "AION_KERNEL_SPAWN" => AION_ERR_IO,
        "AION_CLI_IO_READ" | "AION_CLI_JSON_PARSE" | "AION_CLI_JSON_SERIALIZE"
        | "AION_CLI_SPEC_SHAPE" => AION_ERR_GENERIC,
        "AION_FFI_IO" | "AION_EVIDENCE_IO" => AION_ERR_IO,
        "AION_EVIDENCE_HASH" | "AION_EVIDENCE_ANCHOR" => AION_ERR_EVIDENCE_INVALID,
        "AION_BINDINGS_HOME" | "AION_BINDINGS_IO" | "AION_BINDINGS_JSON" => AION_ERR_IO,
        _ => {
            if e.contains("not found") {
                AION_ERR_CAPSULE_NOT_FOUND
            } else if e.contains("corrupt") {
                AION_ERR_CAPSULE_CORRUPT
            } else if e.contains("policy") {
                AION_ERR_INVALID_POLICY
            } else if e.contains("io") || e.contains("read") || e.contains("write") {
                AION_ERR_IO
            } else if e.contains("version") {
                AION_ERR_UNSUPPORTED_VERSION
            } else {
                AION_ERR_GENERIC
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn aion_run(
    cmd: *const c_char,
    args: *const *const c_char,
    args_len: size_t,
    _env: *const *const c_char,
    _env_len: size_t,
    out_result: *mut AionRunResult,
) -> i32 {
    if out_result.is_null() {
        return set_last_error(line(
            code::FFI_NULL_ARG,
            "aion_run",
            "out_result",
        ));
    }
    let cmd_s = match cstr_to_string(cmd, "cmd") {
        Ok(v) => v,
        Err(c) => return c,
    };
    let mut argv = vec![cmd_s];
    for i in 0..args_len {
        let p = unsafe { *args.add(i) };
        if p.is_null() {
            continue;
        }
        let a = match cstr_to_string(p, "arg") {
            Ok(v) => v,
            Err(c) => return c,
        };
        argv.push(a);
    }
    let det = aion_core::DeterminismProfile::default();
    let rr = match crate::capture::capture(&argv, &det) {
        Ok(v) => v,
        Err(e) => return set_last_error(e),
    };
    let out = unsafe { &mut *out_result };
    out.stdout_len = rr.stdout.len();
    out.stderr_len = rr.stderr.len();
    out.exit_code = rr.exit_code;
    out.duration_ms = rr.duration_ms;
    out.stdout_data = match to_c_owned(rr.stdout) {
        Ok(v) => v,
        Err(c) => return c,
    };
    out.stderr_data = match to_c_owned(rr.stderr) {
        Ok(v) => v,
        Err(c) => return c,
    };
    out.capsule_id = match to_c_owned(rr.run_id) {
        Ok(v) => v,
        Err(c) => return c,
    };
    AION_OK
}

#[no_mangle]
pub extern "C" fn aion_capsule_save(capsule: *const AionCapsule, path: *const c_char) -> i32 {
    if capsule.is_null() {
        return set_last_error(line(
            code::FFI_NULL_ARG,
            "aion_capsule_save",
            "capsule",
        ));
    }
    let path_s = match cstr_to_string(path, "path") {
        Ok(v) => v,
        Err(c) => return c,
    };
    let c = unsafe { &*capsule };
    let model = match cstr_to_string(c.model, "capsule.model") {
        Ok(v) => v,
        Err(code) => return code,
    };
    let prompt = match cstr_to_string(c.prompt, "capsule.prompt") {
        Ok(v) => v,
        Err(code) => return code,
    };
    let cap = ai::build_ai_capsule_v1(model, prompt, c.seed);
    match crate::sdk::save_capsule(Path::new(&path_s), &cap) {
        Ok(()) => AION_OK,
        Err(e) => {
            let _ = set_last_error(&e);
            map_err_code(&e)
        }
    }
}

#[no_mangle]
pub extern "C" fn aion_capsule_load(path: *const c_char, out_capsule: *mut AionCapsule) -> i32 {
    if out_capsule.is_null() {
        return set_last_error(line(
            code::FFI_NULL_ARG,
            "aion_capsule_load",
            "out_capsule",
        ));
    }
    let path_s = match cstr_to_string(path, "path") {
        Ok(v) => v,
        Err(c) => return c,
    };
    let cap = match crate::sdk::load_capsule(Path::new(&path_s)) {
        Ok(v) => v,
        Err(e) => {
            let _ = set_last_error(&e);
            return map_err_code(&e);
        }
    };
    let out = unsafe { &mut *out_capsule };
    out.model = match to_c_owned(cap.model) { Ok(v) => v, Err(c) => return c };
    out.prompt = match to_c_owned(cap.prompt) { Ok(v) => v, Err(c) => return c };
    out.seed = cap.seed;
    out.determinism_profile_json = match to_c_owned(serde_json::to_string(&cap.determinism).unwrap_or_default()) { Ok(v) => v, Err(c) => return c };
    out.token_trace_json = match to_c_owned(serde_json::to_string(&cap.token_trace).unwrap_or_default()) { Ok(v) => v, Err(c) => return c };
    out.events_json = match to_c_owned(serde_json::to_string(&cap.event_stream).unwrap_or_default()) { Ok(v) => v, Err(c) => return c };
    out.graph_json = match to_c_owned(serde_json::to_string(&cap.graph).unwrap_or_default()) { Ok(v) => v, Err(c) => return c };
    out.why_report_json = match to_c_owned(serde_json::to_string(&cap.why).unwrap_or_default()) { Ok(v) => v, Err(c) => return c };
    out.drift_report_json = match to_c_owned(serde_json::to_string(&cap.drift).unwrap_or_default()) { Ok(v) => v, Err(c) => return c };
    out.evidence_path = match to_c_owned(path_s) { Ok(v) => v, Err(c) => return c };
    AION_OK
}

#[no_mangle]
pub extern "C" fn aion_replay_capsule(path: *const c_char, out_result: *mut AionRunResult) -> i32 {
    if out_result.is_null() {
        return set_last_error(line(
            code::FFI_NULL_ARG,
            "aion_replay_capsule",
            "out_result",
        ));
    }
    let p = match cstr_to_string(path, "capsule_path") { Ok(v) => v, Err(c) => return c };
    let cap = match crate::sdk::load_capsule(Path::new(&p)) {
        Ok(v) => v,
        Err(e) => { let _ = set_last_error(&e); return map_err_code(&e); }
    };
    let rep = crate::sdk::replay_capsule(&cap);
    let stdout = rep.replay_capsule.tokens.join(" ");
    let out = unsafe { &mut *out_result };
    out.stdout_len = stdout.len();
    out.stderr_len = 0;
    out.exit_code = if rep.success { 0 } else { 1 };
    out.duration_ms = rep.replay_duration_ms;
    out.stdout_data = match to_c_owned(stdout) { Ok(v) => v, Err(c) => return c };
    out.stderr_data = ptr::null_mut();
    out.capsule_id = match to_c_owned(rep.replay_capsule.evidence.run_id) { Ok(v) => v, Err(c) => return c };
    AION_OK
}

#[no_mangle]
pub extern "C" fn aion_compare_capsules(left_path: *const c_char, right_path: *const c_char, out_comparison: *mut AionReplayComparison) -> i32 {
    if out_comparison.is_null() {
        return set_last_error(line(
            code::FFI_NULL_ARG,
            "aion_compare_capsules",
            "out_comparison",
        ));
    }
    let l = match cstr_to_string(left_path, "left_path") { Ok(v) => v, Err(c) => return c };
    let r = match cstr_to_string(right_path, "right_path") { Ok(v) => v, Err(c) => return c };
    let a = match crate::sdk::load_capsule(Path::new(&l)) { Ok(v) => v, Err(e) => { let _ = set_last_error(&e); return map_err_code(&e); } };
    let b = match crate::sdk::load_capsule(Path::new(&r)) { Ok(v) => v, Err(e) => { let _ = set_last_error(&e); return map_err_code(&e); } };
    let c = crate::sdk::compare_capsules(&a, &b);
    let out = unsafe { &mut *out_comparison };
    out.tokens_equal = c.tokens_equal as u8;
    out.trace_equal = c.trace_equal as u8;
    out.events_equal = c.events_equal as u8;
    out.graph_equal = c.graph_equal as u8;
    out.why_equal = c.why_equal as u8;
    out.drift_equal = c.drift_equal as u8;
    out.capsule_equal = c.capsule_equal as u8;
    out.evidence_equal = c.evidence_equal as u8;
    let mut ptrs = Vec::new();
    for d in c.differences {
        if let Ok(cs) = CString::new(d) {
            ptrs.push(cs.into_raw());
        }
    }
    out.differences_count = ptrs.len();
    out.differences = if ptrs.is_empty() {
        ptr::null_mut()
    } else {
        Box::into_raw(ptrs.into_boxed_slice()) as *mut *mut c_char
    };
    AION_OK
}

#[no_mangle]
pub extern "C" fn aion_drift_between_capsules(a_path: *const c_char, b_path: *const c_char, out_report: *mut AionDriftReport) -> i32 {
    if out_report.is_null() {
        return set_last_error(line(
            code::FFI_NULL_ARG,
            "aion_drift_between_capsules",
            "out_report",
        ));
    }
    let a = match cstr_to_string(a_path, "a_path") { Ok(v) => v, Err(c) => return c };
    let b = match cstr_to_string(b_path, "b_path") { Ok(v) => v, Err(c) => return c };
    let ca = match crate::sdk::load_capsule(Path::new(&a)) { Ok(v) => v, Err(e) => { let _ = set_last_error(&e); return map_err_code(&e); } };
    let cb = match crate::sdk::load_capsule(Path::new(&b)) { Ok(v) => v, Err(e) => { let _ = set_last_error(&e); return map_err_code(&e); } };
    let d = crate::sdk::drift_between(&ca, &cb);
    let out = unsafe { &mut *out_report };
    out.changed = d.changed as u8;
    out.fields_json = match to_c_owned(serde_json::to_string(&d.fields).unwrap_or_default()) { Ok(v) => v, Err(c) => return c };
    AION_OK
}

#[no_mangle]
pub extern "C" fn aion_why_explain_capsule(capsule_path: *const c_char, out_why_json: *mut *mut c_char) -> i32 {
    if out_why_json.is_null() {
        return set_last_error(line(
            code::FFI_NULL_ARG,
            "aion_why_explain_capsule",
            "out_why_json",
        ));
    }
    let p = match cstr_to_string(capsule_path, "capsule_path") { Ok(v) => v, Err(c) => return c };
    let cap = match crate::sdk::load_capsule(Path::new(&p)) { Ok(v) => v, Err(e) => { let _ = set_last_error(&e); return map_err_code(&e); } };
    let exp = crate::sdk::explain_capsule(&cap);
    unsafe { *out_why_json = match to_c_owned(serde_json::to_string(&exp.why).unwrap_or_default()) { Ok(v) => v, Err(c) => return c }; }
    AION_OK
}

#[no_mangle]
pub extern "C" fn aion_graph_causal(capsule_path: *const c_char, out_graph_json: *mut *mut c_char) -> i32 {
    if out_graph_json.is_null() {
        return set_last_error(line(
            code::FFI_NULL_ARG,
            "aion_graph_causal",
            "out_graph_json",
        ));
    }
    let p = match cstr_to_string(capsule_path, "capsule_path") { Ok(v) => v, Err(c) => return c };
    let cap = match crate::sdk::load_capsule(Path::new(&p)) { Ok(v) => v, Err(e) => { let _ = set_last_error(&e); return map_err_code(&e); } };
    unsafe { *out_graph_json = match to_c_owned(serde_json::to_string(&cap.graph).unwrap_or_default()) { Ok(v) => v, Err(c) => return c }; }
    AION_OK
}

fn bytes_to_hex_lower(bytes: &[u8; 32]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

#[no_mangle]
pub extern "C" fn aion_capsule_deterministic_hash_hex(
    path: *const c_char,
    out_hex: *mut *mut c_char,
) -> i32 {
    if out_hex.is_null() {
        return set_last_error(line(
            code::FFI_NULL_ARG,
            "aion_capsule_deterministic_hash_hex",
            "out_hex",
        ));
    }
    let p = match cstr_to_string(path, "path") {
        Ok(v) => v,
        Err(c) => return c,
    };
    let cap = match crate::sdk::load_capsule(Path::new(&p)) {
        Ok(v) => v,
        Err(e) => {
            let _ = set_last_error(&e);
            return map_err_code(&e);
        }
    };
    let h = crate::capsule::deterministic_capsule_hash(&cap);
    let hex = bytes_to_hex_lower(&h);
    unsafe {
        *out_hex = match to_c_owned(hex) {
            Ok(v) => v,
            Err(c) => return c,
        };
    }
    AION_OK
}

#[no_mangle]
pub extern "C" fn aion_replay_symmetry_ok(path: *const c_char, out_ok: *mut u8) -> i32 {
    if out_ok.is_null() {
        return set_last_error(line(
            code::FFI_NULL_ARG,
            "aion_replay_symmetry_ok",
            "out_ok",
        ));
    }
    let p = match cstr_to_string(path, "path") {
        Ok(v) => v,
        Err(c) => return c,
    };
    let cap = match crate::sdk::load_capsule(Path::new(&p)) {
        Ok(v) => v,
        Err(e) => {
            let _ = set_last_error(&e);
            return map_err_code(&e);
        }
    };
    let rep = crate::sdk::replay_capsule(&cap);
    unsafe {
        *out_ok = if rep.replay_symmetry_ok { 1 } else { 0 };
    }
    AION_OK
}

#[no_mangle]
pub extern "C" fn aion_validate_capsule(capsule_path: *const c_char, policy_path: *const c_char, out_report: *mut AionGovernanceReport) -> i32 {
    if out_report.is_null() {
        return set_last_error(line(
            code::FFI_NULL_ARG,
            "aion_validate_capsule",
            "out_report",
        ));
    }
    let cp = match cstr_to_string(capsule_path, "capsule_path") { Ok(v) => v, Err(c) => return c };
    let pp = match cstr_to_string(policy_path, "policy_path") { Ok(v) => v, Err(c) => return c };
    let cap = match crate::sdk::load_capsule(Path::new(&cp)) { Ok(v) => v, Err(e) => { let _ = set_last_error(&e); return map_err_code(&e); } };
    let pol = match governance::load_policy(Path::new(&pp)) { Ok(v) => v, Err(e) => { let _ = set_last_error(&e); return AION_ERR_INVALID_POLICY; } };
    let rep = crate::sdk::validate_capsule(&cap, &pol, &DeterminismProfile::default(), &IntegrityProfile::default());
    let out = unsafe { &mut *out_report };
    out.policy_ok = rep.policy.ok as u8;
    out.determinism_ok = rep.determinism.ok as u8;
    out.integrity_ok = rep.integrity.ok as u8;
    out.violations_json = match to_c_owned(serde_json::to_string(&rep).unwrap_or_default()) { Ok(v) => v, Err(c) => return c };
    AION_OK
}

#[no_mangle]
pub extern "C" fn aion_evidence_verify(evidence_path: *const c_char, out_valid: *mut u8) -> i32 {
    if out_valid.is_null() {
        return set_last_error(line(
            code::FFI_NULL_ARG,
            "aion_evidence_verify",
            "out_valid",
        ));
    }
    let p = match cstr_to_string(evidence_path, "evidence_path") { Ok(v) => v, Err(c) => return c };
    let body = match std::fs::read_to_string(&p) {
        Ok(v) => v,
        Err(e) => {
            let _ = set_last_error(line(
                code::EVIDENCE_IO,
                "aion_evidence_verify",
                &io_cause(&e),
            ));
            return AION_ERR_IO;
        }
    };
    let ok = serde_json::from_str::<aion_core::EvidenceChain>(&body)
        .ok()
        .map(|c| aion_core::verify_linear(&c).is_ok())
        .unwrap_or(false);
    unsafe { *out_valid = ok as u8; }
    if ok { AION_OK } else { AION_ERR_EVIDENCE_INVALID }
}

#[no_mangle]
pub extern "C" fn aion_evidence_generate_keypair(
    out_private_key: *mut u8,
    out_private_len: *mut size_t,
    out_public_key: *mut u8,
    out_public_len: *mut size_t,
) -> i32 {
    if out_private_key.is_null() || out_private_len.is_null() || out_public_key.is_null() || out_public_len.is_null() {
        return set_last_error(line(
            code::FFI_NULL_ARG,
            "aion_evidence_generate_keypair",
            "output_pointers",
        ));
    }
    let (sk, pk) = governance::aion_evidence_generate_keypair();
    unsafe {
        ptr::copy_nonoverlapping(sk.as_ptr(), out_private_key, sk.len());
        ptr::copy_nonoverlapping(pk.as_ptr(), out_public_key, pk.len());
        *out_private_len = sk.len();
        *out_public_len = pk.len();
    }
    AION_OK
}

#[no_mangle]
pub extern "C" fn aion_evidence_sign(
    evidence_data: *const u8,
    evidence_len: size_t,
    private_key: *const u8,
    private_key_len: size_t,
    out_signature: *mut u8,
    out_signature_len: *mut size_t,
) -> i32 {
    if evidence_data.is_null() || private_key.is_null() || out_signature.is_null() || out_signature_len.is_null() {
        return set_last_error(line(
            code::FFI_NULL_ARG,
            "aion_evidence_sign",
            "pointers",
        ));
    }
    let data = unsafe { std::slice::from_raw_parts(evidence_data, evidence_len) };
    let sk = unsafe { std::slice::from_raw_parts(private_key, private_key_len) };
    let sig = match governance::aion_evidence_sign(data, sk) {
        Ok(v) => v,
        Err(e) => return set_last_error(e),
    };
    unsafe {
        ptr::copy_nonoverlapping(sig.as_ptr(), out_signature, sig.len());
        *out_signature_len = sig.len();
    }
    AION_OK
}

#[no_mangle]
pub extern "C" fn aion_evidence_verify_with_key(
    evidence_data: *const u8,
    evidence_len: size_t,
    signature: *const u8,
    signature_len: size_t,
    public_key: *const u8,
    public_key_len: size_t,
    out_valid: *mut u8,
) -> i32 {
    if evidence_data.is_null() || signature.is_null() || public_key.is_null() || out_valid.is_null() {
        return set_last_error(line(
            code::FFI_NULL_ARG,
            "aion_evidence_verify_with_key",
            "pointers",
        ));
    }
    let data = unsafe { std::slice::from_raw_parts(evidence_data, evidence_len) };
    let sig = unsafe { std::slice::from_raw_parts(signature, signature_len) };
    let pk = unsafe { std::slice::from_raw_parts(public_key, public_key_len) };
    let ok = match governance::aion_evidence_verify_ed25519(data, sig, pk) {
        Ok(v) => v,
        Err(e) => return set_last_error(e),
    };
    unsafe { *out_valid = ok as u8; }
    if ok { AION_OK } else { AION_ERR_EVIDENCE_INVALID }
}

#[no_mangle]
pub extern "C" fn aion_telemetry_set_enabled(enabled: u8) -> i32 {
    let home = std::env::var_os("USERPROFILE").or_else(|| std::env::var_os("HOME"));
    let Some(home) = home else {
        return set_last_error(line(
            code::BINDINGS_HOME,
            "aion_telemetry_set_enabled",
            "home_unset",
        ));
    };
    let p = std::path::PathBuf::from(home).join(".aion").join("telemetry.toml");
    if let Some(parent) = p.parent() { let _ = std::fs::create_dir_all(parent); }
    match std::fs::write(&p, if enabled == 0 { "enabled = false\n" } else { "enabled = true\n" }) {
        Ok(()) => AION_OK,
        Err(e) => {
            let _ = set_last_error(line(
                code::FFI_IO,
                "aion_telemetry_set_enabled",
                &io_cause(&e),
            ));
            AION_ERR_IO
        }
    }
}

#[no_mangle]
pub extern "C" fn aion_telemetry_get_enabled(out_enabled: *mut u8) -> i32 {
    if out_enabled.is_null() {
        return set_last_error(line(
            code::FFI_NULL_ARG,
            "aion_telemetry_get_enabled",
            "out_enabled",
        ));
    }
    let home = std::env::var_os("USERPROFILE").or_else(|| std::env::var_os("HOME"));
    let Some(home) = home else {
        return set_last_error(line(
            code::BINDINGS_HOME,
            "aion_telemetry_get_enabled",
            "home_unset",
        ));
    };
    let p = std::path::PathBuf::from(home).join(".aion").join("telemetry.toml");
    let enabled = std::fs::read_to_string(&p).ok().map(|s| s.contains("true")).unwrap_or(false);
    unsafe { *out_enabled = enabled as u8; }
    AION_OK
}

#[no_mangle]
pub extern "C" fn aion_setup(config_path: *const c_char) -> i32 {
    let p = match cstr_to_string(config_path, "config_path") { Ok(v) => v, Err(c) => return c };
    let body = "[output]\nbase = \"aion_output\"\n";
    match std::fs::write(&p, body) {
        Ok(()) => AION_OK,
        Err(e) => {
            let _ = set_last_error(line(
                code::FFI_IO,
                "aion_setup",
                &io_cause(&e),
            ));
            AION_ERR_IO
        }
    }
}

#[no_mangle]
pub extern "C" fn aion_doctor(out_diagnostic_json: *mut *mut c_char) -> i32 {
    if out_diagnostic_json.is_null() {
        return set_last_error(line(
            code::FFI_NULL_ARG,
            "aion_doctor",
            "out_diagnostic_json",
        ));
    }
    let doc = serde_json::json!({
        "ok": true,
        "aion_version": env!("CARGO_PKG_VERSION"),
        "cwd": std::env::current_dir().ok(),
    });
    unsafe { *out_diagnostic_json = match to_c_owned(doc.to_string()) { Ok(v) => v, Err(c) => return c }; }
    AION_OK
}

#[no_mangle]
pub extern "C" fn aion_free_string(s: *mut c_char) {
    if s.is_null() {
        return;
    }
    unsafe {
        let _ = CString::from_raw(s);
    }
}

#[no_mangle]
pub extern "C" fn aion_free_run_result(res: *mut AionRunResult) {
    if res.is_null() { return; }
    let r = unsafe { &mut *res };
    if !r.stdout_data.is_null() { unsafe { let _ = CString::from_raw(r.stdout_data); } r.stdout_data = ptr::null_mut(); }
    if !r.stderr_data.is_null() { unsafe { let _ = CString::from_raw(r.stderr_data); } r.stderr_data = ptr::null_mut(); }
    if !r.capsule_id.is_null() { unsafe { let _ = CString::from_raw(r.capsule_id); } r.capsule_id = ptr::null_mut(); }
}

#[no_mangle]
pub extern "C" fn aion_last_error() -> *const c_char {
    if let Ok(g) = last_error_store().lock() {
        g.as_ptr()
    } else {
        ptr::null()
    }
}

#[no_mangle]
pub extern "C" fn aion_capsule_version() -> *const c_char {
    static VERSION: &[u8] = b"1\0";
    VERSION.as_ptr() as *const c_char
}
