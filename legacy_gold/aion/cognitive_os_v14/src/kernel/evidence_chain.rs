use crate::evidence_engine::{EvidenceIndex, EvidenceTimeline, EvidenceTimelineRow};
use anyhow::{bail, Context, Result};
use serde_json::Value;
use sha2::{Digest, Sha256};

use super::system_truth::SystemTruth;

/// Canonical JSON for hashing: recursively sorted object keys, compact (no whitespace).
pub fn sort_value_keys(v: &Value) -> Value {
    match v {
        Value::Object(map) => {
            let mut keys: Vec<String> = map.keys().cloned().collect();
            keys.sort();
            let mut out = serde_json::Map::new();
            for k in keys {
                if let Some(inner) = map.get(&k) {
                    out.insert(k, sort_value_keys(inner));
                }
            }
            Value::Object(out)
        }
        Value::Array(items) => Value::Array(items.iter().map(sort_value_keys).collect()),
        _ => v.clone(),
    }
}

pub fn canonical_json_string(v: &Value) -> Result<String> {
    let sorted = sort_value_keys(v);
    Ok(sorted.to_string())
}

/// JSON payload included in [`EvidenceTimelineRow::hash`] (excludes `hash` only).
pub fn record_hash_payload(r: &EvidenceTimelineRow) -> Value {
    serde_json::json!({
        "action": r.action,
        "input": r.input,
        "output": r.output,
        "prev_hash": r.prev_hash,
        "process_id": r.process_id,
        "timestamp": r.timestamp.to_rfc3339(),
    })
}

pub fn compute_record_chain_hash(r: &EvidenceTimelineRow) -> Result<String> {
    let v = record_hash_payload(r);
    let s = canonical_json_string(&v)?;
    Ok(hex::encode(Sha256::digest(s.as_bytes())))
}

pub fn finalize_record_chain_fields(r: &mut EvidenceTimelineRow, prev_hash: Option<&str>) -> Result<()> {
    r.prev_hash = prev_hash.unwrap_or("").to_string();
    r.hash = compute_record_chain_hash(r)?;
    Ok(())
}

pub struct EvidenceChain;

impl EvidenceChain {
    pub fn append_to_truth(truth: &mut SystemTruth, process_id: &str, mut record: EvidenceTimelineRow) -> Result<()> {
        record.process_id = process_id.to_string();
        truth.ensure_timeline(process_id)?;
        let last = truth
            .staged_timelines
            .get(process_id)
            .and_then(|t| t.records.last())
            .map(|r| r.hash.as_str())
            .filter(|h| !h.is_empty());
        finalize_record_chain_fields(&mut record, last)?;
        let tl = truth
            .staged_timelines
            .get_mut(process_id)
            .expect("timeline must exist after ensure_timeline");
        tl.append(record);
        truth.dirty_timeline_ids.insert(process_id.to_string());
        Ok(())
    }
}

pub fn verify_evidence_chain(index: &EvidenceIndex) -> Result<()> {
    for pid in &index.processes {
        verify_process_timeline(pid)?;
    }
    Ok(())
}

fn verify_process_timeline(process_id: &str) -> Result<()> {
    let t = EvidenceTimeline::load(process_id).with_context(|| format!("load timeline {process_id}"))?;
    let mut expect_prev = String::new();
    for (i, rec) in t.records.iter().enumerate() {
        if rec.prev_hash != expect_prev {
            bail!(
                "evidence chain prev_hash mismatch at process={process_id} index={i}: expected {:?} got {:?}",
                expect_prev,
                rec.prev_hash
            );
        }
        let expected = compute_record_chain_hash(rec)?;
        if rec.hash != expected {
            bail!(
                "evidence chain hash mismatch at process={process_id} index={i}: expected {expected} got {}",
                rec.hash
            );
        }
        expect_prev = rec.hash.clone();
    }
    Ok(())
}
