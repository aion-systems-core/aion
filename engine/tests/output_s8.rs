//! Output layout refinements: base override, deterministic IDs, completeness, deterministic ZIP hash.

use aion_engine::output::{output_base_dir, OutputWriter};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

static ENV_LOCK: Mutex<()> = Mutex::new(());

fn tmp_base() -> PathBuf {
    let p = std::env::temp_dir().join(format!(
        "aion-output-s8-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    ));
    fs::create_dir_all(&p).unwrap();
    p
}

#[test]
fn meta_json_written_by_default() {
    let _g = ENV_LOCK.lock().unwrap();
    std::env::remove_var("AION_OUTPUT_BASE");
    let w = OutputWriter::new("meta-default-test").expect("writer");
    let meta_path = w.root().join("meta.json");
    assert!(meta_path.exists(), "meta.json at {}", meta_path.display());
    let s = fs::read_to_string(&meta_path).unwrap();
    assert!(s.contains("meta-default-test"));
    assert!(s.contains("aion_version"));
}

#[test]
fn output_base_respects_aion_output_base() {
    let _g = ENV_LOCK.lock().unwrap();
    let base = tmp_base();
    std::env::set_var("AION_OUTPUT_BASE", &base);
    let resolved = output_base_dir();
    assert_eq!(resolved, base);
    let w = OutputWriter::new("env-base-test").expect("writer");
    assert!(w.root().starts_with(&base));
    std::env::remove_var("AION_OUTPUT_BASE");
}

fn hash_file(path: &std::path::Path) -> String {
    let bytes = fs::read(path).unwrap();
    format!("{:x}", Sha256::digest(bytes))
}

#[test]
fn output_bundle_completeness() {
    let _g = ENV_LOCK.lock().unwrap();
    let base = tmp_base();
    std::env::set_var("AION_OUTPUT_BASE", &base);
    std::env::set_var("AION_OUTPUT_ID", "run_0001");

    let w = OutputWriter::new("bundle-complete-test").expect("writer");
    w.write_json("sample", &serde_json::json!({"ok": true})).unwrap();
    let root = w.into_root();

    for f in [
        "manifest.json",
        "stdout.txt",
        "stderr.txt",
        "events.jsonl",
        "replay.json",
        "audit.jsonl",
        "hashes.json",
        ".aion.zip",
    ] {
        assert!(root.join(f).exists(), "missing {}", f);
    }

    std::env::remove_var("AION_OUTPUT_ID");
    std::env::remove_var("AION_OUTPUT_BASE");
}

#[test]
fn deterministic_zip_hash_for_same_inputs() {
    let _g = ENV_LOCK.lock().unwrap();
    let base = tmp_base();
    std::env::set_var("AION_OUTPUT_BASE", &base);

    std::env::set_var("AION_OUTPUT_ID", "run_0100");
    let w1 = OutputWriter::new("zip-hash-test").expect("writer");
    w1.write_json("sample", &serde_json::json!({"k": "v"})).unwrap();
    let r1 = w1.into_root();
    let h1 = hash_file(&r1.join(".aion.zip"));

    let _ = fs::remove_dir_all(base.join("zip-hash-test").join("run_0100"));

    std::env::set_var("AION_OUTPUT_ID", "run_0100");
    let w2 = OutputWriter::new("zip-hash-test").expect("writer");
    w2.write_json("sample", &serde_json::json!({"k": "v"})).unwrap();
    let r2 = w2.into_root();
    let h2 = hash_file(&r2.join(".aion.zip"));

    assert_eq!(h1, h2, "zip hash must be deterministic");

    std::env::remove_var("AION_OUTPUT_ID");
    std::env::remove_var("AION_OUTPUT_BASE");
}
