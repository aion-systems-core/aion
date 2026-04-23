//! Phase 3.2: structured, queryable evidence (JSONL / outer [`cos_core::evidence::EvidenceRecordV2`]) without touching kernel types.
//!
//! Parsing uses [`crate::common::normalize::normalize_json_bytes`] on both the outer record line and
//! the inner integration payload (stored in [`EvidenceRecordV2::input`]).

mod diff;

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use serde_json::Value;

pub use diff::{diff_evidence, EvidenceDiff};

use crate::common::normalize::{normalize_json_bytes, strip_utf8_bom};
use cos_adapters::cos_v1_integration_outer_v2;
use cos_core::evidence::record::EvidenceRecordV2;

/// Canonical integration evidence fields (from normalized outer `EvidenceRecordV2.input` JSON).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ParsedEvidence {
    pub step: String,
    pub adapter: String,
    pub input_hash: String,
    pub output_hash: String,
    pub status: String,
}

/// Deterministic lookup by step name.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct EvidenceIndex {
    pub by_step: BTreeMap<String, ParsedEvidence>,
}

fn str_field(v: &Value, k: &str) -> Result<String, String> {
    v.get(k)
        .and_then(|x| x.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("evidence missing `{k}`"))
}

/// Parse normalized integration JSON stored in the outer row's `input` field.
pub fn parse_evidence_description(description: &str) -> Result<ParsedEvidence, String> {
    let canon = normalize_json_bytes(description.as_bytes()).map_err(|e| e.to_string())?;
    let v: Value = serde_json::from_slice(&canon).map_err(|e| e.to_string())?;
    Ok(ParsedEvidence {
        step: str_field(&v, "step")?,
        adapter: str_field(&v, "adapter")?,
        input_hash: str_field(&v, "input_hash")?,
        output_hash: str_field(&v, "output_hash")?,
        status: str_field(&v, "status")?,
    })
}

fn outer_evidence_row_to_v2(v: &Value) -> Result<EvidenceRecordV2, String> {
    let obj = v
        .as_object()
        .ok_or_else(|| "evidence outer row must be a JSON object".to_string())?;
    if obj.contains_key("step_name") && obj.contains_key("current_hash") {
        return serde_json::from_value(v.clone()).map_err(|e| e.to_string());
    }
    if obj.len() == 2 && obj.contains_key("id") && obj.contains_key("description") {
        let id = str_field(v, "id")?;
        let description = str_field(v, "description")?;
        return Ok(cos_v1_integration_outer_v2(&id, &description));
    }
    Err(format!(
        "evidence JSONL outer row must be canonical EvidenceRecordV2 or legacy object with only `id` and `description`; got keys: {:?}",
        obj.keys().collect::<Vec<_>>()
    ))
}

/// Parse one JSONL line: canonical [`EvidenceRecordV2`] JSON (outer), or legacy `{id, description}` routed through the v1 kernel evidence adapter, then inner payload from `input`.
pub fn parse_evidence_line(line: &str) -> Result<ParsedEvidence, String> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return Err("empty evidence line".into());
    }
    let canon_line = normalize_json_bytes(trimmed.as_bytes()).map_err(|e| e.to_string())?;
    let v: Value = serde_json::from_slice(&canon_line).map_err(|e| e.to_string())?;
    let rec = outer_evidence_row_to_v2(&v)?;
    parse_evidence_description(&rec.input)
}

/// Parse non-empty lines from embedded JSONL text (e.g. `evidence_chain.jsonl` from `audit.zip`).
pub fn parse_evidence_chain(text: &str) -> Result<Vec<ParsedEvidence>, String> {
    let mut out = Vec::new();
    for line in text.lines() {
        if line.trim().is_empty() {
            continue;
        }
        out.push(parse_evidence_line(line)?);
    }
    Ok(out)
}

/// Load a standalone `.jsonl` file (UTF-8, optional BOM).
pub fn load_evidence_chain(path: &Path) -> Result<Vec<ParsedEvidence>, String> {
    let raw = fs::read(path).map_err(|e| e.to_string())?;
    let text =
        String::from_utf8(strip_utf8_bom(&raw).to_vec()).map_err(|e| e.to_string())?;
    parse_evidence_chain(&text)
}

/// Build a step-keyed index; duplicate step names are rejected.
pub fn build_index(chain: &[ParsedEvidence]) -> Result<EvidenceIndex, String> {
    let mut by_step = BTreeMap::new();
    for p in chain {
        if by_step.insert(p.step.clone(), p.clone()).is_some() {
            return Err(format!("duplicate evidence step: {}", p.step));
        }
    }
    Ok(EvidenceIndex { by_step })
}
