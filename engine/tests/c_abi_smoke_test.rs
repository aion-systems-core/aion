#![cfg(feature = "ffi")]

//! Smoke coverage for C ABI entrypoints (link `aion-engine` with `--features ffi`).

use aion_engine::ai::build_ai_capsule_v1;
use aion_engine::ffi::{
    aion_capsule_deterministic_hash_hex, aion_evidence_verify, aion_free_run_result,
    aion_free_string, aion_last_error, aion_replay_capsule, aion_replay_symmetry_ok, aion_run,
    AionRunResult, AION_ERR_IO, AION_OK,
};
use aion_engine::sdk::save_capsule;
use std::ffi::CString;
use std::os::raw::c_char;

fn cstr(s: &str) -> CString {
    CString::new(s).expect("cstr")
}

#[test]
fn c_abi_smoke_run_capture_free() {
    let (cmd, extra): (CString, Vec<CString>) = if cfg!(windows) {
        (
            cstr("cmd"),
            vec![cstr("/C"), cstr("echo"), cstr("aion_c_abi")],
        )
    } else {
        (cstr("echo"), vec![cstr("aion_c_abi")])
    };
    let argv: Vec<*const c_char> = extra.iter().map(|s| s.as_ptr()).collect();
    let mut out: AionRunResult = unsafe { std::mem::zeroed() };
    let rc = unsafe {
        aion_run(
            cmd.as_ptr(),
            argv.as_ptr(),
            argv.len(),
            std::ptr::null(),
            0,
            &mut out as *mut _,
        )
    };
    assert_eq!(rc, AION_OK, "aion_run");
    unsafe { aion_free_run_result(&mut out) };
}

#[test]
fn c_abi_smoke_capsule_hash_symmetry_replay() {
    let cap = build_ai_capsule_v1("demo".into(), "c abi".into(), 77);
    let mut path = std::env::temp_dir();
    path.push("aion_engine_c_abi_smoke.aionai");
    let _ = std::fs::remove_file(&path);
    save_capsule(&path, &cap).expect("save");

    let p = path.to_string_lossy();
    let pc = CString::new(p.as_ref()).unwrap();

    let mut hex: *mut c_char = std::ptr::null_mut();
    let rc = unsafe { aion_capsule_deterministic_hash_hex(pc.as_ptr(), &mut hex) };
    assert_eq!(rc, AION_OK);
    assert!(!hex.is_null());
    let s = unsafe { std::ffi::CStr::from_ptr(hex).to_string_lossy().into_owned() };
    assert_eq!(s.len(), 64);
    unsafe { aion_free_string(hex) };

    let mut sym: u8 = 0;
    let rc = unsafe { aion_replay_symmetry_ok(pc.as_ptr(), &mut sym) };
    assert_eq!(rc, AION_OK);
    assert!(sym <= 1, "symmetry flag must be bool-like");

    let mut rep: AionRunResult = unsafe { std::mem::zeroed() };
    let rc = unsafe { aion_replay_capsule(pc.as_ptr(), &mut rep) };
    assert_eq!(rc, AION_OK);
    unsafe { aion_free_run_result(&mut rep) };

    let _ = std::fs::remove_file(&path);
}

#[test]
fn c_abi_evidence_io_error_is_contract_json() {
    let missing = cstr("C:/definitely/missing/aion-evidence.json");
    let mut ok: u8 = 0;
    let rc = unsafe { aion_evidence_verify(missing.as_ptr(), &mut ok as *mut _) };
    assert_eq!(rc, AION_ERR_IO);
    let err_ptr = aion_last_error();
    assert!(!err_ptr.is_null());
    let err = unsafe {
        std::ffi::CStr::from_ptr(err_ptr)
            .to_string_lossy()
            .into_owned()
    };
    let v: serde_json::Value = serde_json::from_str(&err).expect("json error");
    assert_eq!(v["schema_version"], 1);
    assert_eq!(v["code"], "AION_EVIDENCE_IO");
}
