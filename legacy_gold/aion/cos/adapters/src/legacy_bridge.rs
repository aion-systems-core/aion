//! JSON-level bridges from legacy shapes → `cos_core` (Phase 5.7).
//!
//! Shapes are detected only by JSON structure + required keys — no runtime branching on crate IDs.

use crate::kernel_compatible::KernelCompatible;
use cos_core::audit::records::record::AuditRecord;
use cos_core::evidence::record::EvidenceRecordV2;
use serde_json::{json, Value};

const SCHEMA_V14_EVIDENCE: &str = "aion/bridge/v14-evidence/1";
const SCHEMA_V1_EVIDENCE: &str = "aion/bridge/v1-kernel-evidence/1";
const SCHEMA_BUILD_AUDIT_CHAIN: &str = "aion/bridge/v14-build-audit-chain/1";

#[derive(Debug, Clone)]
pub struct BridgeError(pub String);

impl std::fmt::Display for BridgeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for BridgeError {}

fn require_obj(value: &Value) -> Result<&serde_json::Map<String, Value>, BridgeError> {
    value
        .as_object()
        .ok_or_else(|| BridgeError("expected JSON object".into()))
}

/// Wrapper: serde JSON of a `cognitive_os_v14::evidence_engine::EvidenceTimelineRow` plus schema tag.
#[derive(Debug, Clone)]
pub struct V14EvidenceBridge(pub Value);

impl V14EvidenceBridge {
    pub fn try_to_kernel(&self) -> Result<EvidenceRecordV2, BridgeError> {
        let root = require_obj(&self.0)?;
        let schema = root
            .get("$schema")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BridgeError("missing or non-string `$schema`".into()))?;
        if schema != SCHEMA_V14_EVIDENCE {
            return Err(BridgeError(format!(
                "expected $schema={SCHEMA_V14_EVIDENCE}, got {schema:?}"
            )));
        }
        let body = root
            .get("record")
            .ok_or_else(|| BridgeError("missing `record` object".into()))?;
        let obj = require_obj(body)?;
        let process_id = obj
            .get("process_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BridgeError("missing `process_id`".into()))?
            .to_string();
        let action = obj
            .get("action")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BridgeError("missing `action`".into()))?
            .to_string();
        let prev_hash = obj
            .get("prev_hash")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let hash = obj
            .get("hash")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        if hash.is_empty() {
            return Err(BridgeError(
                "v14 evidence `hash` must be non-empty for strict kernel projection".into(),
            ));
        }
        let ts = obj
            .get("timestamp")
            .ok_or_else(|| BridgeError("missing `timestamp`".into()))?;
        let ts_str = if let Some(s) = ts.as_str() {
            s.to_string()
        } else {
            serde_json::to_string(ts).map_err(|e| BridgeError(e.to_string()))?
        };
        let input = obj.get("input").cloned().unwrap_or(Value::Null);
        let output = obj.get("output").cloned().unwrap_or(Value::Null);
        let input_s = serde_json::to_string(&input).map_err(|e| BridgeError(e.to_string()))?;
        let output_s = serde_json::to_string(&output).map_err(|e| BridgeError(e.to_string()))?;
        Ok(EvidenceRecordV2 {
            index: 0,
            step_name: format!("{process_id}::{action}"),
            input: input_s,
            output: output_s,
            determinism: format!("{SCHEMA_V14_EVIDENCE}|{ts_str}"),
            forbidden_ops: Vec::new(),
            previous_hash: prev_hash,
            current_hash: hash,
        })
    }
}

impl KernelCompatible for V14EvidenceBridge {
    type Target = EvidenceRecordV2;

    fn to_kernel(&self) -> EvidenceRecordV2 {
        self.try_to_kernel()
            .unwrap_or_else(|e| panic!("V14EvidenceBridge::to_kernel: {e}"))
    }
}

/// Wrapper: legacy cos-v1 kernel evidence as JSON `{ "id", "description" }` + schema tag.
#[derive(Debug, Clone)]
pub struct V1KernelEvidenceBridge(pub Value);

fn v1_kernel_evidence_canon_json(id: &str, description: &str) -> Result<String, BridgeError> {
    let id_s = serde_json::to_string(id).map_err(|e| BridgeError(e.to_string()))?;
    let desc_s = serde_json::to_string(description).map_err(|e| BridgeError(e.to_string()))?;
    // Fixed key order (lexical) — never rely on `serde_json::Map` iteration for digest bytes.
    Ok(format!("{{\"description\":{desc_s},\"id\":{id_s}}}"))
}

impl V1KernelEvidenceBridge {
    pub fn try_to_kernel(&self) -> Result<EvidenceRecordV2, BridgeError> {
        let root = require_obj(&self.0)?;
        let schema = root
            .get("$schema")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BridgeError("missing or non-string `$schema`".into()))?;
        if schema != SCHEMA_V1_EVIDENCE {
            return Err(BridgeError(format!(
                "expected $schema={SCHEMA_V1_EVIDENCE}, got {schema:?}"
            )));
        }
        let body = root
            .get("record")
            .ok_or_else(|| BridgeError("missing `record` object".into()))?;
        let obj = require_obj(body)?;
        let id = obj
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BridgeError("missing `id`".into()))?
            .to_string();
        let description = obj
            .get("description")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BridgeError("missing `description`".into()))?
            .to_string();
        let canon_s = v1_kernel_evidence_canon_json(&id, &description)?;
        let digest = hex_lower(&sha256_bytes(canon_s.as_bytes()));
        Ok(EvidenceRecordV2 {
            index: 0,
            step_name: id,
            input: description,
            output: String::new(),
            determinism: SCHEMA_V1_EVIDENCE.to_string(),
            forbidden_ops: Vec::new(),
            previous_hash: String::new(),
            current_hash: digest,
        })
    }
}

impl KernelCompatible for V1KernelEvidenceBridge {
    type Target = EvidenceRecordV2;

    fn to_kernel(&self) -> EvidenceRecordV2 {
        self.try_to_kernel()
            .unwrap_or_else(|e| panic!("V1KernelEvidenceBridge::to_kernel: {e}"))
    }
}

/// Wrapper: `cognitive_os_v14::builder::audit::AuditChain` JSON + schema tag.
#[derive(Debug, Clone)]
pub struct BuildAuditChainBridge(pub Value);

impl BuildAuditChainBridge {
    pub fn try_to_kernel_rows(&self) -> Result<Vec<AuditRecord>, BridgeError> {
        let root = require_obj(&self.0)?;
        let schema = root
            .get("$schema")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BridgeError("missing or non-string `$schema`".into()))?;
        if schema != SCHEMA_BUILD_AUDIT_CHAIN {
            return Err(BridgeError(format!(
                "expected $schema={SCHEMA_BUILD_AUDIT_CHAIN}, got {schema:?}"
            )));
        }
        let chain = root
            .get("chain")
            .ok_or_else(|| BridgeError("missing `chain` object".into()))?;
        let cobj = require_obj(chain)?;
        let chain_kind = cobj
            .get("chain_kind")
            .and_then(|v| v.as_str())
            .ok_or_else(|| BridgeError("missing `chain_kind`".into()))?
            .to_string();
        let events = cobj
            .get("events")
            .and_then(|v| v.as_array())
            .ok_or_else(|| BridgeError("missing `events` array".into()))?;
        let mut out = Vec::with_capacity(events.len());
        for ev in events {
            let o = require_obj(ev)?;
            let event_type = o
                .get("event_type")
                .and_then(|v| v.as_str())
                .ok_or_else(|| BridgeError("missing `event_type`".into()))?
                .to_string();
            let ts = o
                .get("timestamp_utc")
                .and_then(|v| v.as_str())
                .ok_or_else(|| BridgeError("missing `timestamp_utc`".into()))?
                .to_string();
            let payload = o.get("payload").cloned().unwrap_or(Value::Null);
            let payload_hash = o
                .get("payload_hash")
                .and_then(|v| v.as_str())
                .ok_or_else(|| BridgeError("missing `payload_hash`".into()))?
                .to_string();
            let event_hash = o
                .get("event_hash")
                .and_then(|v| v.as_str())
                .ok_or_else(|| BridgeError("missing `event_hash`".into()))?
                .to_string();
            let details = serde_json::to_string(&json!({
                "payload": payload,
                "payload_hash": payload_hash,
                "event_hash": event_hash,
            }))
            .map_err(|e| BridgeError(e.to_string()))?;
            out.push(AuditRecord {
                timestamp: ts,
                actor: chain_kind.clone(),
                action: event_type,
                details,
            });
        }
        Ok(out)
    }
}

impl KernelCompatible for BuildAuditChainBridge {
    type Target = Vec<AuditRecord>;

    fn to_kernel(&self) -> Vec<AuditRecord> {
        self.try_to_kernel_rows()
            .unwrap_or_else(|e| panic!("BuildAuditChainBridge::to_kernel: {e}"))
    }
}

/// Build tagged JSON for tests (v14 evidence).
pub fn tagged_v14_evidence(record: &Value) -> Value {
    json!({
        "$schema": SCHEMA_V14_EVIDENCE,
        "record": record,
    })
}

/// Build tagged JSON for tests (v1 kernel evidence stub).
pub fn tagged_v1_kernel_evidence(record: &Value) -> Value {
    json!({
        "$schema": SCHEMA_V1_EVIDENCE,
        "record": record,
    })
}

/// Phase 6.2 alias — same envelope as [`tagged_v1_kernel_evidence`].
#[inline]
pub fn tagged_v1_evidence(record: &Value) -> Value {
    tagged_v1_kernel_evidence(record)
}

/// cos-v1 integration JSONL **outer** row as canonical `EvidenceRecordV2` (via strict bridge).
#[inline]
pub fn cos_v1_integration_outer_v2(id: &str, description: &str) -> EvidenceRecordV2 {
    let raw = json!({ "id": id, "description": description });
    V1KernelEvidenceBridge(tagged_v1_kernel_evidence(&raw)).to_kernel()
}

/// Build tagged JSON for tests (builder audit chain).
pub fn tagged_build_audit_chain(chain: &Value) -> Value {
    json!({
        "$schema": SCHEMA_BUILD_AUDIT_CHAIN,
        "chain": chain,
    })
}

fn sha256_bytes(data: &[u8]) -> [u8; 32] {
    use sha2::{Digest, Sha256};
    let mut h = Sha256::new();
    h.update(data);
    h.finalize().into()
}

fn hex_lower(bytes: &[u8; 32]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
