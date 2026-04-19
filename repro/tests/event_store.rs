//! Phase 8.2 — `.events.json` alongside artifact JSON.

use repro::core::event_store::{self, EVENT_STREAM_SCHEMA_V1};
use serde_json::Value;
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

static SEQ: AtomicU64 = AtomicU64::new(0);

fn repro_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_repro"))
}

fn scratch() -> std::path::PathBuf {
    let n = SEQ.fetch_add(1, Ordering::SeqCst);
    let base = std::env::var_os("CARGO_TARGET_TMPDIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target"));
    let p = base.join("repro_event_store_tests").join(format!("t_{n}"));
    std::fs::create_dir_all(&p).unwrap();
    p
}

#[test]
fn events_file_matches_artifact_trace_json() {
    let cwd = scratch();
    let out = Command::new(repro_bin())
        .current_dir(&cwd)
        .args(["run", "--", "echo", "hello"])
        .output()
        .expect("spawn repro");
    assert_eq!(out.status.code(), Some(0), "{}", String::from_utf8_lossy(&out.stderr));

    let index = std::fs::read_to_string(cwd.join("repro_runs").join("INDEX")).unwrap();
    let id = index.lines().map(str::trim).find(|l| !l.is_empty()).unwrap();
    let events_path = cwd.join("repro_runs").join(format!("{id}.events.json"));
    assert!(
        events_path.exists(),
        "expected events file at {:?}",
        events_path
    );

    let loaded = event_store::load_event_stream_in(&cwd.join("repro_runs"), id).unwrap();
    let art_json = std::fs::read_to_string(cwd.join("repro_runs").join(format!("{id}.json")))
        .unwrap();
    let v: Value = serde_json::from_str(&art_json).unwrap();
    let trace_events = &v["trace"]["events"];
    let ev_json: Value = serde_json::to_value(&loaded.events).unwrap();
    assert_eq!(&ev_json, trace_events, "events must match artifact.trace.events");

    let body: Value = serde_json::from_str(&std::fs::read_to_string(&events_path).unwrap()).unwrap();
    assert_eq!(
        body["schema"].as_str().unwrap_or_default(),
        EVENT_STREAM_SCHEMA_V1
    );
    assert_eq!(body["run_id"], id);
}
