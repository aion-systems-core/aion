use crate::contracts::{CapsuleManifest, PolicyProfile, CAPSULE_SCHEMA_VERSION};
use chrono::Utc;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use zip::write::FileOptions;

pub fn write_capsule_from_artifact_json(
    artifact_json: &str,
    out_dir: &Path,
    policy: PolicyProfile,
) -> Result<PathBuf, String> {
    let artifact: Value = serde_json::from_str(artifact_json)
        .map_err(|e| format!("invalid artifact JSON for capsule: {e}"))?;

    let command = artifact
        .get("command")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();
    let stdout = artifact
        .get("stdout")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();
    let stderr = artifact
        .get("stderr")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();

    let manifest = CapsuleManifest {
        capsule_schema_version: CAPSULE_SCHEMA_VERSION,
        command,
        policy,
        artifact_schema_version: artifact
            .get("schema_version")
            .and_then(Value::as_u64)
            .map(|v| v as u32),
    };
    let manifest_json =
        serde_json::to_string_pretty(&manifest).map_err(|e| format!("manifest encode error: {e}"))?;

    let events = artifact
        .get("trace")
        .and_then(|v| v.get("events"))
        .cloned()
        .unwrap_or_else(|| json!([]));
    let events_json = serde_json::to_string_pretty(&events)
        .map_err(|e| format!("events encode error: {e}"))?;

    let replay = json!({
        "mode": "deterministic",
        "time_base": artifact.get("timestamp").cloned().unwrap_or(Value::Null),
        "seed": "0xDEADBEEF",
    });
    let replay_json = serde_json::to_string_pretty(&replay)
        .map_err(|e| format!("replay encode error: {e}"))?;

    let audit_jsonl = json!({
        "event": "capsule_created",
        "ts": Utc::now().to_rfc3339(),
        "run_id": artifact.get("run_id").cloned().unwrap_or(Value::Null),
    })
    .to_string();

    fs::create_dir_all(out_dir).map_err(|e| format!("cannot create capsule directory: {e}"))?;
    let stamp = Utc::now().format("%Y-%m-%d_%H-%M-%S");
    let file_name = format!("run_{stamp}.aion.zip");
    let path = out_dir.join(file_name);
    let file = File::create(&path).map_err(|e| format!("cannot create capsule zip: {e}"))?;
    let mut zip = zip::ZipWriter::new(file);
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

    let entries = vec![
        ("manifest.json", manifest_json.clone()),
        ("stdout.txt", stdout.clone()),
        ("stderr.txt", stderr.clone()),
        ("events.json", events_json.clone()),
        ("replay.json", replay_json.clone()),
        ("audit.jsonl", format!("{audit_jsonl}\n")),
        ("drift_baseline.json", "{}\n".to_string()),
    ];

    let mut hashes = serde_json::Map::new();
    for (name, content) in &entries {
        zip.start_file(*name, options)
            .map_err(|e| format!("zip entry error for {name}: {e}"))?;
        zip.write_all(content.as_bytes())
            .map_err(|e| format!("zip write error for {name}: {e}"))?;
        hashes.insert((*name).to_string(), Value::String(hash_hex(content.as_bytes())));
    }

    let hashes_json = serde_json::to_string_pretty(&Value::Object(hashes))
        .map_err(|e| format!("hash encode error: {e}"))?;
    zip.start_file("hashes.json", options)
        .map_err(|e| format!("zip entry error for hashes.json: {e}"))?;
    zip.write_all(hashes_json.as_bytes())
        .map_err(|e| format!("zip write error for hashes.json: {e}"))?;
    zip.finish()
        .map_err(|e| format!("zip finalize error: {e}"))?;

    Ok(path)
}

fn hash_hex(input: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(input);
    format!("{:x}", h.finalize())
}
