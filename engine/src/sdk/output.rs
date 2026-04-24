//! Deterministic output bundle writes (no timestamps, no logging).

use serde::Serialize;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize)]
pub struct SdkMeta {
    pub sdk_version: String,
    pub output_base: String,
    pub file_count: usize,
}

pub fn sdk_version() -> String {
    std::env::var("AION_SDK_VERSION")
        .ok()
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| {
            include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../VERSION"))
                .trim()
                .to_string()
        })
}

pub fn sdk_output_dir(requested: &Path) -> PathBuf {
    if requested.is_absolute() {
        return requested.to_path_buf();
    }
    match std::env::var_os("AION_SDK_OUTPUT_BASE") {
        Some(v) if !v.is_empty() => PathBuf::from(v).join(requested),
        _ => requested.to_path_buf(),
    }
}

/// Write named files under `output_dir` in **sorted** name order (deterministic).
pub fn write_output_bundle(output_dir: &Path, files: &[(&str, Vec<u8>)]) -> Result<(), String> {
    let output_dir = sdk_output_dir(output_dir);
    fs::create_dir_all(&output_dir).map_err(|e| format!("create_dir_all: {e}"))?;
    let mut ordered: Vec<(&str, &[u8])> = files.iter().map(|(n, b)| (*n, b.as_slice())).collect();
    ordered.sort_by(|a, b| a.0.cmp(b.0));
    for (name, bytes) in ordered {
        let path = output_dir.join(name);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("create_dir_all: {e}"))?;
        }
        let mut f =
            fs::File::create(&path).map_err(|e| format!("create {}: {e}", path.display()))?;
        f.write_all(bytes)
            .map_err(|e| format!("write {}: {e}", path.display()))?;
    }
    let meta = SdkMeta {
        sdk_version: sdk_version(),
        output_base: output_dir.to_string_lossy().to_string(),
        file_count: files.len(),
    };
    let meta_body = serde_json::to_vec_pretty(&meta).map_err(|e| e.to_string())?;
    fs::write(output_dir.join("sdk_meta.json"), meta_body)
        .map_err(|e| format!("sdk_meta.json: {e}"))?;
    Ok(())
}

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

/// Minimal HTML wrapper for SDK JSON payloads (no product chrome).
pub fn render_sdk_html(title: &str, json_body: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en"><head><meta charset="utf-8"><title>{}</title>
<style>body{{font-family:system-ui,sans-serif;margin:1.5rem}} pre{{background:#f6f6f6;padding:1rem;overflow:auto}}</style>
</head><body>
<h1>{}</h1>
<pre>{}</pre>
</body></html>"#,
        esc(title),
        esc(title),
        esc(json_body),
    )
}

/// Single status node for SDK runs.
pub fn render_sdk_svg(success: bool) -> String {
    let fill = if success {
        "rgb(210,240,220)"
    } else {
        "rgb(255,220,220)"
    };
    let label = if success { "ok" } else { "fail" };
    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="200" height="80" viewBox="0 0 200 80">
<rect width="100%" height="100%" fill="rgb(252,252,252)"/>
<rect x="20" y="20" width="160" height="40" rx="6" fill="{fill}" stroke="rgb(80,80,80)"/>
<text x="100" y="46" font-size="14" font-family="sans-serif" text-anchor="middle">{label}</text>
</svg>"#,
        fill = fill,
        label = label,
    )
}

pub fn json_pretty(value: &impl Serialize) -> Result<String, String> {
    serde_json::to_string_pretty(value).map_err(|e| e.to_string())
}
