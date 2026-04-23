//! AION Capsule v1 — ZIP container with stable entry names.

use crate::contracts::{
    CapsuleManifest, DeterminismProfile, PolicyProfile, RunResult, CAPSULE_SCHEMA_VERSION,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Serializable capsule descriptor for product output (`*.aion`).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Capsule {
    pub capsule_schema_version: u32,
    /// File name of the sealed ZIP inside the output directory (e.g. `run_….aion.zip`).
    pub zip_file: String,
    pub run: RunResult,
    pub policy: PolicyProfile,
    pub determinism: DeterminismProfile,
}
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use zip::write::FileOptions;
use zip::ZipArchive;

fn hash_hex(bytes: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(bytes);
    format!("{:x}", h.finalize())
}

/// Write capsule v1 from a serialized [`RunResult`] JSON string.
pub fn write_capsule_v1(
    run_json: &str,
    out_dir: &Path,
    policy: PolicyProfile,
    determinism: DeterminismProfile,
) -> Result<PathBuf, String> {
    let run: RunResult = serde_json::from_str(run_json)
        .map_err(|e| format!("capsule: invalid run JSON: {e}"))?;

    let manifest = CapsuleManifest {
        capsule_schema_version: CAPSULE_SCHEMA_VERSION,
        execution_artifact_schema_version: run.schema_version,
        run_id: run.run_id.clone(),
        command: run.command.clone(),
        policy,
        determinism,
    };
    let manifest_json =
        serde_json::to_string_pretty(&manifest).map_err(|e| format!("manifest: {e}"))?;

    let replay = json!({
        "mode": "deterministic",
        "time_epoch_secs": determinism.time_epoch_secs,
        "seed": format!("0x{:x}", determinism.random_seed),
    });
    let replay_json = serde_json::to_string_pretty(&replay).map_err(|e| format!("replay: {e}"))?;

    let events = json!([{
        "kind": "run_complete",
        "run_id": run.run_id,
        "exit_code": run.exit_code,
        "duration_ms": run.duration_ms,
    }]);
    let events_json = serde_json::to_string_pretty(&events).map_err(|e| format!("events: {e}"))?;

    let audit_line = json!({
        "event": "capsule_sealed",
        "run_id": run.run_id,
    })
    .to_string();

    fs::create_dir_all(out_dir).map_err(|e| format!("out_dir: {e}"))?;
    let stamp = run.run_id.chars().take(16).collect::<String>();
    let path = out_dir.join(format!("run_{stamp}.aion.zip"));
    let file = File::create(&path).map_err(|e| format!("zip create: {e}"))?;
    let mut zip = zip::ZipWriter::new(file);
    let opts = FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    let entries: [(&str, String); 7] = [
        ("manifest.json", manifest_json),
        ("stdout.txt", run.stdout.clone()),
        ("stderr.txt", run.stderr.clone()),
        ("events.json", events_json),
        ("replay.json", replay_json),
        ("audit.jsonl", format!("{audit_line}\n")),
        ("drift_baseline.json", "{}\n".into()),
    ];

    let mut hashes = serde_json::Map::new();
    for (name, body) in entries.iter() {
        zip.start_file(*name, opts)
            .map_err(|e| format!("zip {name}: {e}"))?;
        zip.write_all(body.as_bytes())
            .map_err(|e| format!("zip write {name}: {e}"))?;
        hashes.insert((*name).to_string(), serde_json::Value::String(hash_hex(body.as_bytes())));
    }

    let hashes_json =
        serde_json::to_string_pretty(&serde_json::Value::Object(hashes)).map_err(|e| format!("hashes: {e}"))?;
    zip.start_file("hashes.json", opts)
        .map_err(|e| format!("zip hashes: {e}"))?;
    zip.write_all(hashes_json.as_bytes())
        .map_err(|e| format!("zip write hashes: {e}"))?;
    zip.finish().map_err(|e| format!("zip finish: {e}"))?;

    Ok(path)
}

pub fn read_capsule_manifest(path: &Path) -> Result<CapsuleManifest, String> {
    let file = File::open(path).map_err(|e| format!("open capsule: {e}"))?;
    let mut archive = ZipArchive::new(file).map_err(|e| format!("zip: {e}"))?;
    let mut z = archive.by_name("manifest.json").map_err(|_| "missing manifest.json".to_string())?;
    let mut s = String::new();
    z.read_to_string(&mut s)
        .map_err(|e| format!("read manifest: {e}"))?;
    serde_json::from_str(&s).map_err(|e| format!("manifest json: {e}"))
}
