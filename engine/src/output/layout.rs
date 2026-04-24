//! Deterministic output directory layout and archive generation.
//!
//! Root resolution order:
//! 1. `AION_OUTPUT_BASE`
//! 2. `./aion.config.toml` (`[output].base`)
//! 3. `~/.aion/config.toml` (`[output].base`)
//! 4. `<cwd>/aion_output`
//!
//! Run ID resolution order:
//! 1. `AION_OUTPUT_ID`
//! 2. `./aion.config.toml` (`[output].id`)
//! 3. `~/.aion/config.toml` (`[output].id`)
//! 4. next deterministic incremental `run_0001`, `run_0002`, ...

use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

const PRODUCT_VERSION: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../VERSION"));

#[derive(Debug, Clone, Default, Deserialize)]
struct OutputConfig {
    #[serde(default)]
    output: OutputConfigOutput,
}

#[derive(Debug, Clone, Default, Deserialize)]
struct OutputConfigOutput {
    base: Option<String>,
    id: Option<String>,
}

/// Per-run metadata written as `meta.json` (self-contained).
#[derive(Debug, Clone, Serialize)]
pub struct OutputRunMeta {
    pub aion_version: String,
    pub git_commit: Option<String>,
    pub command: String,
    pub cli_args: String,
    pub utc_start: String,
    pub utc_end: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_profile: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub determinism_profile: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct ZipManifest {
    version: String,
    command: String,
    run_id: String,
    files: Vec<String>,
}

fn read_config_file(path: &Path) -> Option<OutputConfig> {
    let s = fs::read_to_string(path).ok()?;
    toml::from_str::<OutputConfig>(&s).ok()
}

fn merged_output_config() -> OutputConfigOutput {
    let mut out = OutputConfigOutput::default();
    if let Some(home) = std::env::var_os("USERPROFILE").or_else(|| std::env::var_os("HOME")) {
        let p = PathBuf::from(home).join(".aion").join("config.toml");
        if let Some(cfg) = read_config_file(&p) {
            out.base = cfg.output.base;
            out.id = cfg.output.id;
        }
    }
    let local = std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("aion.config.toml");
    if let Some(cfg) = read_config_file(&local) {
        if cfg.output.base.is_some() {
            out.base = cfg.output.base;
        }
        if cfg.output.id.is_some() {
            out.id = cfg.output.id;
        }
    }
    out
}

fn resolve_base_from(input: &str) -> PathBuf {
    let p = PathBuf::from(input);
    if p.is_absolute() {
        p
    } else {
        std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join(p)
    }
}

/// Resolve artefact root base.
pub fn output_base_dir() -> PathBuf {
    if let Some(p) = std::env::var_os("AION_OUTPUT_BASE") {
        if !p.is_empty() {
            return resolve_base_from(&p.to_string_lossy());
        }
    }
    let cfg = merged_output_config();
    if let Some(base) = cfg.base {
        return resolve_base_from(&base);
    }
    std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("aion_output")
}

fn next_run_id(command_dir: &Path) -> String {
    let mut max_seen = 0usize;
    if let Ok(rd) = fs::read_dir(command_dir) {
        for ent in rd.flatten() {
            let name = ent.file_name();
            let name = name.to_string_lossy();
            if let Some(rest) = name.strip_prefix("run_") {
                if let Ok(n) = rest.parse::<usize>() {
                    if n > max_seen {
                        max_seen = n;
                    }
                }
            }
        }
    }
    format!("run_{:04}", max_seen + 1)
}

fn chosen_run_id(command_dir: &Path) -> String {
    if let Ok(v) = std::env::var("AION_OUTPUT_ID") {
        let s = v.trim();
        if !s.is_empty() {
            return sanitize_command(s);
        }
    }
    let cfg = merged_output_config();
    if let Some(id) = cfg.id {
        let s = id.trim();
        if !s.is_empty() {
            return sanitize_command(s);
        }
    }
    next_run_id(command_dir)
}

fn write_meta_json(root: &Path, command: &str, cli_args: &str) -> Result<(), String> {
    let now = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    let meta = OutputRunMeta {
        aion_version: PRODUCT_VERSION.trim().to_string(),
        git_commit: std::env::var("AION_GIT_COMMIT")
            .ok()
            .filter(|s| !s.is_empty()),
        command: command.to_string(),
        cli_args: cli_args.to_string(),
        utc_start: now.clone(),
        utc_end: now,
        policy_profile: None,
        determinism_profile: None,
    };
    write_json(root, "meta", &meta)?;
    Ok(())
}

fn update_latest_link(command_dir: &Path, root: &Path) -> Result<(), String> {
    let latest = command_dir.join("latest");
    if latest.exists() {
        let _ = fs::remove_file(&latest);
        let _ = fs::remove_dir(&latest);
    }
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(root, &latest).map_err(|e| format!("latest symlink: {e}"))?;
    }
    #[cfg(windows)]
    {
        std::os::windows::fs::symlink_dir(root, &latest)
            .map_err(|e| format!("latest symlink: {e}"))?;
    }
    Ok(())
}

fn ensure_file(root: &Path, rel: &str, default_content: &[u8]) -> Result<(), String> {
    let p = root.join(rel);
    if !p.exists() {
        fs::write(&p, default_content).map_err(|e| format!("write {}: {e}", p.display()))?;
    }
    Ok(())
}

fn file_hash_hex(path: &Path) -> Result<String, String> {
    let mut f = fs::File::open(path).map_err(|e| format!("open {}: {e}", path.display()))?;
    let mut h = Sha256::new();
    let mut buf = [0u8; 8192];
    loop {
        let n = f
            .read(&mut buf)
            .map_err(|e| format!("read {}: {e}", path.display()))?;
        if n == 0 {
            break;
        }
        h.update(&buf[..n]);
    }
    Ok(format!("{:x}", h.finalize()))
}

/// Finalize output folder with required files and deterministic `.aion.zip`.
pub fn finalize_output_bundle(root: &Path) -> Result<PathBuf, String> {
    ensure_file(root, "stdout.txt", b"")?;
    ensure_file(root, "stderr.txt", b"")?;
    ensure_file(root, "events.jsonl", b"")?;
    ensure_file(root, "replay.json", b"{}\n")?;
    ensure_file(root, "audit.jsonl", b"")?;
    ensure_file(root, "result.json", b"{\"success\":true,\"exit_code\":0}\n")?;

    let run_id = root
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "run_0000".into());
    let command = root
        .parent()
        .and_then(|p| p.file_name())
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown".into());

    let required = vec![
        "manifest.json".to_string(),
        "stdout.txt".to_string(),
        "stderr.txt".to_string(),
        "events.jsonl".to_string(),
        "replay.json".to_string(),
        "audit.jsonl".to_string(),
        "hashes.json".to_string(),
    ];

    let manifest = ZipManifest {
        version: PRODUCT_VERSION.trim().to_string(),
        command,
        run_id,
        files: required.clone(),
    };
    let manifest_body = canonical_json_from_serialize(&manifest)?;
    fs::write(root.join("manifest.json"), manifest_body)
        .map_err(|e| format!("manifest write: {e}"))?;

    let mut entries: Vec<(String, String)> = Vec::new();
    for f in [
        "manifest.json",
        "stdout.txt",
        "stderr.txt",
        "events.jsonl",
        "replay.json",
        "audit.jsonl",
    ] {
        entries.push((f.to_string(), file_hash_hex(&root.join(f))?));
    }
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    let hashes_body = canonical_json_from_serialize(&entries)?;
    fs::write(root.join("hashes.json"), hashes_body).map_err(|e| format!("hashes write: {e}"))?;

    let zip_path = root.join(".aion.zip");
    let zip_file = fs::File::create(&zip_path).map_err(|e| format!("zip create: {e}"))?;
    let mut zip = zip::ZipWriter::new(zip_file);
    let options = zip::write::FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .last_modified_time(zip::DateTime::default())
        .unix_permissions(0o644);
    for f in required {
        zip.start_file(&f, options)
            .map_err(|e| format!("zip start_file {f}: {e}"))?;
        let body = fs::read(root.join(&f)).map_err(|e| format!("zip read {f}: {e}"))?;
        zip.write_all(&body)
            .map_err(|e| format!("zip write {f}: {e}"))?;
    }
    zip.finish().map_err(|e| format!("zip finish: {e}"))?;
    Ok(zip_path)
}

/// Root directory for one command invocation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputPath {
    pub root: PathBuf,
}

impl OutputPath {
    /// Create `<base>/<command>/<run_id>`.
    pub fn new(command: &str) -> Result<Self, String> {
        Self::new_with_cli_args(command, "")
    }

    /// Same as [`Self::new`], with optional CLI argument summary for `meta.json`.
    pub fn new_with_cli_args(command: &str, cli_args: &str) -> Result<Self, String> {
        let command_dir = output_base_dir().join(sanitize_command(command));
        fs::create_dir_all(&command_dir).map_err(|e| format!("output dir: {e}"))?;
        let run_id = chosen_run_id(&command_dir);
        let root = command_dir.join(run_id);
        fs::create_dir_all(&root).map_err(|e| format!("output dir: {e}"))?;
        write_meta_json(&root, command, cli_args).map_err(|e| format!("meta.json: {e}"))?;
        let _ = update_latest_link(&command_dir, &root);
        Ok(Self { root })
    }

    pub fn join(&self, name: &str) -> PathBuf {
        self.root.join(name)
    }
}

fn sanitize_command(command: &str) -> String {
    command
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '-'
            }
        })
        .collect()
}

fn with_extension(name: &str, ext: &str) -> String {
    if name.ends_with(ext) {
        name.to_string()
    } else {
        format!("{name}{ext}")
    }
}

fn should_sort_array(key: Option<&str>) -> bool {
    matches!(
        key,
        Some("labels")
            | Some("categories")
            | Some("violations")
            | Some("differences")
            | Some("fields")
            | Some("policies")
            | Some("crates")
    )
}

fn canonicalize_json_value(key: Option<&str>, value: Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut keys: Vec<String> = map.keys().cloned().collect();
            keys.sort();
            let mut out = serde_json::Map::new();
            for k in keys {
                let v = map.get(&k).cloned().unwrap_or(Value::Null);
                out.insert(k.clone(), canonicalize_json_value(Some(&k), v));
            }
            Value::Object(out)
        }
        Value::Array(items) => {
            let mut out: Vec<Value> = items
                .into_iter()
                .map(|v| canonicalize_json_value(None, v))
                .collect();
            if should_sort_array(key) {
                out.sort_by(|a, b| {
                    serde_json::to_string(a)
                        .unwrap_or_default()
                        .cmp(&serde_json::to_string(b).unwrap_or_default())
                });
            }
            Value::Array(out)
        }
        _ => value,
    }
}

pub fn canonical_json_string(value: &Value) -> Result<String, String> {
    let mut body = Vec::new();
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"  ");
    let mut ser = serde_json::Serializer::with_formatter(&mut body, formatter);
    value
        .serialize(&mut ser)
        .map_err(|e| format!("json encode: {e}"))?;
    String::from_utf8(body).map_err(|e| format!("json utf8: {e}"))
}

pub fn canonical_json_from_serialize<T: Serialize + ?Sized>(value: &T) -> Result<String, String> {
    let raw = serde_json::to_value(value).map_err(|e| format!("json encode: {e}"))?;
    let norm = canonicalize_json_value(None, raw);
    canonical_json_string(&norm)
}

/// Write deterministic contract JSON with stable key order.
pub fn write_json<T: Serialize + ?Sized>(
    root: &Path,
    name: &str,
    value: &T,
) -> Result<PathBuf, String> {
    #[derive(Serialize)]
    struct OutputEnvelope {
        status: &'static str,
        data: Value,
        #[serde(skip_serializing_if = "Option::is_none")]
        error: Option<Value>,
    }

    let file = with_extension(name, ".json");
    let path = root.join(&file);
    let raw = serde_json::to_value(value).map_err(|e| format!("json encode: {e}"))?;
    let data = canonicalize_json_value(None, raw);
    let envelope = OutputEnvelope {
        status: "ok",
        data,
        error: None,
    };
    let envelope_value = serde_json::to_value(envelope).map_err(|e| format!("json encode: {e}"))?;
    let body = canonical_json_string(&envelope_value)?;
    fs::write(&path, body).map_err(|e| format!("write {}: {e}", path.display()))?;
    Ok(path)
}

pub fn write_html(root: &Path, name: &str, body: &str) -> Result<PathBuf, String> {
    let file = with_extension(name, ".html");
    let path = root.join(&file);
    fs::write(&path, body).map_err(|e| format!("write {}: {e}", path.display()))?;
    Ok(path)
}

pub fn write_svg(root: &Path, name: &str, body: &str) -> Result<PathBuf, String> {
    let file = with_extension(name, ".svg");
    let path = root.join(&file);
    fs::write(&path, body).map_err(|e| format!("write {}: {e}", path.display()))?;
    Ok(path)
}

pub fn write_capsule(root: &Path, name: &str, body: &str) -> Result<PathBuf, String> {
    let file = if name.ends_with(".aion") {
        name.to_string()
    } else {
        format!("{name}.aion")
    };
    let path = root.join(&file);
    fs::write(&path, body).map_err(|e| format!("write {}: {e}", path.display()))?;
    Ok(path)
}

pub fn write_evidence(root: &Path, name: &str, body: &str) -> Result<PathBuf, String> {
    let file = if name.ends_with(".aionevidence") {
        name.to_string()
    } else {
        format!("{name}.aionevidence")
    };
    let path = root.join(&file);
    fs::write(&path, body).map_err(|e| format!("write {}: {e}", path.display()))?;
    Ok(path)
}

pub fn write_aionai(root: &Path, name: &str, body: &str) -> Result<PathBuf, String> {
    let file = if name.ends_with(".aionai") {
        name.to_string()
    } else {
        format!("{name}.aionai")
    };
    let path = root.join(&file);
    fs::write(&path, body).map_err(|e| format!("write {}: {e}", path.display()))?;
    Ok(path)
}
