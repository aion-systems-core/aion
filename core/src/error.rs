//! Deterministic, machine-readable errors: packed lines `CODE|CONTEXT|CAUSE` and
//! versioned JSON envelopes [`AionError`] (schema_version 1).
//!
//! - **CODE**: stable `AION_*` token (no spaces).
//! - **CONTEXT**: static callsite label (ASCII).
//! - **CAUSE**: bounded, sanitized detail (`|` / newlines stripped). Prefer structured
//!   tokens (e.g. `NotFound`, `token_trace_lenMismatch:3:4`) over prose.

use serde::{Deserialize, Serialize};

/// Stable error code literals (public contract for logs, FFI mapping, tests).
pub mod code {
    pub const KERNEL_SPAWN: &str = "AION_KERNEL_SPAWN";

    pub const CAPSULE_IO: &str = "AION_CAPSULE_IO";
    pub const CAPSULE_JSON: &str = "AION_CAPSULE_JSON";
    pub const CAPSULE_VALIDATE: &str = "AION_CAPSULE_VALIDATE";
    pub const CAPSULE_INPUT: &str = "AION_CAPSULE_INPUT";
    pub const CAPSULE_SERIALIZE: &str = "AION_CAPSULE_SERIALIZE";

    pub const CAPSULE_SAVE_MKDIR: &str = "AION_CAPSULE_SAVE_MKDIR";
    pub const CAPSULE_SAVE_EXISTS: &str = "AION_CAPSULE_SAVE_EXISTS";
    pub const CAPSULE_SAVE_IO: &str = "AION_CAPSULE_SAVE_IO";

    /// Product output path already occupied (no overwrite).
    pub const OUTPUT_AIONAI_EXISTS: &str = "AION_OUTPUT_AIONAI_EXISTS";
    /// `serde_json` serialization of an output artefact failed.
    pub const OUTPUT_JSON_SERIALIZE: &str = "AION_OUTPUT_JSON_SERIALIZE";

    pub const SDK_IO: &str = "AION_SDK_IO";
    pub const SDK_PARSE: &str = "AION_SDK_PARSE";
    pub const SDK_VALIDATION: &str = "AION_SDK_VALIDATION";
    pub const SDK_VERSION: &str = "AION_SDK_VERSION";
    pub const SDK_OTHER: &str = "AION_SDK_OTHER";

    pub const CAPTURE_EMPTY: &str = "AION_CAPTURE_EMPTY";

    pub const CLI_IO_READ: &str = "AION_CLI_IO_READ";
    pub const CLI_JSON_PARSE: &str = "AION_CLI_JSON_PARSE";
    pub const CLI_SPEC_SHAPE: &str = "AION_CLI_SPEC_SHAPE";
    pub const CLI_JSON_SERIALIZE: &str = "AION_CLI_JSON_SERIALIZE";

    pub const GOVERNANCE_IO: &str = "AION_GOVERNANCE_IO";
    pub const GOVERNANCE_JSON: &str = "AION_GOVERNANCE_JSON";

    pub const EVIDENCE_IO: &str = "AION_EVIDENCE_IO";
    pub const EVIDENCE_HASH: &str = "AION_EVIDENCE_HASH";
    pub const EVIDENCE_ANCHOR: &str = "AION_EVIDENCE_ANCHOR";
    pub const DRIFT_JSON: &str = "AION_DRIFT_JSON";
    pub const DRIFT_TOLERANCE: &str = "AION_DRIFT_TOLERANCE";
    pub const DRIFT_OVERFLOW: &str = "AION_DRIFT_OVERFLOW";
    pub const REPLAY_MISMATCH: &str = "AION_REPLAY_MISMATCH";
    pub const REPLAY_PROFILE: &str = "AION_REPLAY_PROFILE";
    pub const REPLAY_SYMMETRY: &str = "AION_REPLAY_SYMMETRY";

    pub const FFI_NULL_ARG: &str = "AION_FFI_NULL_ARG";
    pub const FFI_IO: &str = "AION_FFI_IO";
    pub const FFI_IDLE: &str = "AION_FFI_IDLE";
    pub const FFI_UTF8: &str = "AION_FFI_UTF8";
    pub const FFI_CSTRING_INTERIOR_NULL: &str = "AION_FFI_CSTRING_INTERIOR_NULL";

    pub const BINDINGS_HOME: &str = "AION_BINDINGS_HOME";
    pub const BINDINGS_IO: &str = "AION_BINDINGS_IO";
    pub const BINDINGS_JSON: &str = "AION_BINDINGS_JSON";

    /// Leaf node when `CAUSE` is not itself a packed `AION_*|…|…` line.
    pub const NESTED_CAUSE: &str = "AION_NESTED_CAUSE";
    /// Normalizer could not interpret the payload.
    pub const OPAQUE: &str = "AION_OPAQUE";
}

/// Versioned JSON error contract (all subsystems).
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AionError {
    /// Schema version for this object shape; currently `1` only.
    #[serde(default = "schema_version_default")]
    pub schema_version: u32,
    pub code: String,
    pub message: String,
    pub context: String,
    pub origin: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cause: Option<Box<AionError>>,
}

fn schema_version_default() -> u32 {
    1
}

/// Deterministic short `message` from `code` (suffix after `AION_`, lowercased).
pub fn message_from_code(code: &str) -> String {
    code.strip_prefix("AION_").unwrap_or(code).to_lowercase()
}

/// Subsystem label for [`AionError::origin`].
pub fn origin_for_code(code: &str) -> &'static str {
    if code.starts_with("AION_KERNEL_") {
        "kernel"
    } else if code.starts_with("AION_CAPTURE_") {
        "capture"
    } else if code.starts_with("AION_CAPSULE_") {
        "capsule"
    } else if code.starts_with("AION_OUTPUT_") {
        "output"
    } else if code.starts_with("AION_SDK_") {
        "sdk"
    } else if code.starts_with("AION_CLI_") {
        "cli"
    } else if code.starts_with("AION_FFI_") {
        "ffi"
    } else if code.starts_with("AION_BINDINGS_") {
        "bindings"
    } else if code.starts_with("AION_GOVERNANCE_") {
        "governance"
    } else if code.starts_with("AION_EVIDENCE_") {
        "evidence"
    } else if code.starts_with("AION_DRIFT_") {
        "drift"
    } else if code.starts_with("AION_REPLAY_") {
        "replay"
    } else if code.starts_with("AION_NESTED_") || code == code::NESTED_CAUSE {
        "core"
    } else if code.starts_with("AION_OPAQUE") {
        "core"
    } else {
        "core"
    }
}

/// True if `s` is a packed error line `AION_*|context|cause`.
pub fn is_packed_line(s: &str) -> bool {
    s.starts_with("AION_") && s.matches('|').count() >= 2
}

/// Parse `CODE|CONTEXT|CAUSE` into [`AionError`] (schema_version 1).
pub fn aion_error_from_line(line: &str) -> Option<AionError> {
    let mut it = line.splitn(3, '|');
    let code = it.next()?;
    let context = it.next()?;
    let cause = it.next()?;
    if !code.starts_with("AION_") {
        return None;
    }
    let message = message_from_code(code);
    let origin = origin_for_code(code).to_string();
    let cause = nested_cause_from_token(cause, &origin);
    Some(AionError {
        schema_version: 1,
        code: code.to_string(),
        message,
        context: context.to_string(),
        origin,
        cause,
    })
}

fn nested_cause_from_token(cause: &str, parent_origin: &str) -> Option<Box<AionError>> {
    if cause.is_empty() {
        return None;
    }
    if is_packed_line(cause) {
        return Some(Box::new(aion_error_from_line(cause)?));
    }
    Some(Box::new(AionError {
        schema_version: 1,
        code: code::NESTED_CAUSE.to_string(),
        message: message_from_code(code::NESTED_CAUSE),
        context: sanitize_cause(cause),
        origin: parent_origin.to_string(),
        cause: None,
    }))
}

/// Serialize [`AionError`] as compact JSON (deterministic key order = struct field order).
pub fn error_to_json(e: &AionError) -> Result<String, serde_json::Error> {
    serde_json::to_string(e)
}

/// If `input` is already a valid schema-1 [`AionError`] JSON, re-serialize canonically.
/// If it is a packed `AION_*|…|…` line, convert then serialize.
/// Otherwise wrap as [`code::OPAQUE`] with optional nested token row.
pub fn canonical_error_json(input: &str, default_origin: &str) -> String {
    let input = input.trim();
    if input.is_empty() {
        return fallback_opaque("(empty)", default_origin);
    }
    if input.starts_with('{') {
        if let Ok(mut e) = serde_json::from_str::<AionError>(input) {
            if e.schema_version != 1 {
                e.schema_version = 1;
            }
            if e.origin.is_empty() {
                e.origin = default_origin.to_string();
            }
            return error_to_json(&e).unwrap_or_else(|_| fallback_opaque(input, default_origin));
        }
    }
    if is_packed_line(input) {
        if let Some(e) = aion_error_from_line(input) {
            return error_to_json(&e).unwrap_or_else(|_| fallback_opaque(input, default_origin));
        }
    }
    fallback_opaque(input, default_origin)
}

fn fallback_opaque(raw: &str, default_origin: &str) -> String {
    let sanitized = sanitize_cause(raw);
    let outer = AionError {
        schema_version: 1,
        code: code::OPAQUE.to_string(),
        message: message_from_code(code::OPAQUE),
        context: "normalize".to_string(),
        origin: default_origin.to_string(),
        cause: Some(Box::new(AionError {
            schema_version: 1,
            code: code::NESTED_CAUSE.to_string(),
            message: message_from_code(code::NESTED_CAUSE),
            context: sanitized,
            origin: default_origin.to_string(),
            cause: None,
        })),
    };
    error_to_json(&outer).unwrap_or_else(|_| {
        format!(
            "{{\"schema_version\":1,\"code\":\"{}\",\"message\":\"opaque\",\"context\":\"serialize\",\"origin\":\"{}\"}}",
            code::OPAQUE, default_origin
        )
    })
}

/// Normalize `CAUSE` so the line stays splittable and log-stable.
pub fn sanitize_cause(s: &str) -> String {
    let mut s = s.replace(['|', '\n', '\r'], "/");
    const MAX: usize = 512;
    if s.len() > MAX {
        s.truncate(MAX);
    }
    s
}

/// One canonical error line: `CODE|CONTEXT|CAUSE`.
pub fn line(code: &'static str, context: &'static str, cause: &str) -> String {
    let c = sanitize_cause(cause);
    format!("{code}|{context}|{c}")
}

/// Portable I/O failure token (`std::io::ErrorKind` `Debug`).
pub fn io_cause(e: &std::io::Error) -> String {
    format!("{:?}", e.kind())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_splits_and_sanitizes() {
        let s = line(code::CAPSULE_JSON, "read_ai_capsule_v1", "a|b\nc");
        assert_eq!(s, "AION_CAPSULE_JSON|read_ai_capsule_v1|a/b/c");
        let parts: Vec<_> = s.splitn(3, '|').collect();
        assert_eq!(parts.len(), 3);
        assert_eq!(parts[0], code::CAPSULE_JSON);
    }

    #[test]
    fn canonical_json_from_line_roundtrip_fields() {
        let ln = line(code::CAPSULE_JSON, "read_ai_capsule_v1", "invalid_json");
        let j = canonical_error_json(&ln, "cli");
        let v: serde_json::Value = serde_json::from_str(&j).unwrap();
        assert_eq!(v["schema_version"], 1);
        assert_eq!(v["code"], code::CAPSULE_JSON);
        assert_eq!(v["message"], "capsule_json");
        assert_eq!(v["context"], "read_ai_capsule_v1");
        assert_eq!(v["origin"], "capsule");
        assert!(v.get("cause").is_some());
    }

    #[test]
    fn canonical_json_preserves_valid_envelope() {
        let e = AionError {
            schema_version: 1,
            code: code::CLI_JSON_PARSE.to_string(),
            message: message_from_code(code::CLI_JSON_PARSE),
            context: "t".into(),
            origin: "cli".into(),
            cause: None,
        };
        let j0 = error_to_json(&e).unwrap();
        let j1 = canonical_error_json(&j0, "ffi");
        assert_eq!(j0, j1);
    }
}
