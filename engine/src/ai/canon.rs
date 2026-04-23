//! Canonicalization primitives for the AION determinism model.
//!
//! This module defines the canonical byte/string form used by hash, drift, and replay paths.
//! Invariant: semantically equal payloads must map to identical canonical JSON/bytes.
//! All transforms are pure and order-stable (no clock, RNG, or host-dependent state).

use super::graph::CausalGraphV2;
use super::model::AICapsuleV1;
use super::why::WhyReportV2;
use aion_core::DriftReport;
use serde::Serialize;
use serde_json::{Map, Value};
use std::collections::BTreeMap;

/// Canonicalize a JSON value by recursively sorting object keys.
///
/// Purpose: normalize structurally equivalent JSON into one stable key order.
/// Invariant: input semantics are preserved; only object-key ordering changes.
/// I/O: any `serde_json::Value` -> canonicalized `Value` with sorted maps.
/// Determinism: uses `BTreeMap` ordering and recursive pure transformation.
pub fn canonicalize_value(v: &Value) -> Value {
    match v {
        Value::Object(map) => {
            let mut out: BTreeMap<String, Value> = BTreeMap::new();
            for (k, val) in map.iter() {
                out.insert(k.clone(), canonicalize_value(val));
            }
            let m: Map<String, Value> = out.into_iter().collect();
            Value::Object(m)
        }
        Value::Array(a) => Value::Array(a.iter().map(canonicalize_value).collect()),
        x => x.clone(),
    }
}

/// Render canonical JSON text from an arbitrary JSON value.
///
/// Purpose: produce the stable text form consumed by comparison and hashing.
/// Invariant: equal canonical values stringify to byte-identical output.
/// I/O: `Value` -> compact JSON `String` (`{}` on serialization failure).
/// Determinism: delegates to `canonicalize_value` then stable serializer.
pub fn canonical_json_string(v: &Value) -> String {
    let c = canonicalize_value(v);
    serde_json::to_string(&c).unwrap_or_else(|_| "{}".into())
}

/// Convert a serializable payload into JSON value space for canonicalization.
///
/// Purpose: normalize typed structs/enums into `Value` before canonical sorting.
/// Invariant: conversion failure never panics and yields `Value::Null`.
/// I/O: `T: Serialize` -> `Value`.
/// Determinism: serializer output is deterministic for identical input `T`.
pub fn value_from_serializable<T: Serialize>(t: &T) -> Value {
    serde_json::to_value(t).unwrap_or(Value::Null)
}

/// Canonical JSON string for `WhyReportV2`.
///
/// Purpose: provide replay/hash-safe representation of explainability graph payload.
/// Invariant: same report fields always produce identical output text.
/// I/O: `&WhyReportV2` -> canonical compact JSON `String`.
/// Determinism: based on `value_from_serializable` + recursive key sorting.
pub fn canonical_why_v2_json(w: &WhyReportV2) -> String {
    canonical_json_string(&value_from_serializable(w))
}

/// Canonical JSON string for `CausalGraphV2`.
///
/// Purpose: normalize graph payload before equality checks and hashing.
/// Invariant: node/edge values are preserved while object keys are ordered.
/// I/O: `&CausalGraphV2` -> canonical compact JSON `String`.
/// Determinism: canonical key order is stable across hosts/runs.
pub fn canonical_graph_v2_json(g: &CausalGraphV2) -> String {
    canonical_json_string(&value_from_serializable(g))
}

/// Canonical JSON string for `DriftReport`.
///
/// Purpose: compare drift snapshots without serializer-map ordering noise.
/// Invariant: logically equal drift contracts stringify identically.
/// I/O: `&DriftReport` -> canonical compact JSON `String`.
/// Determinism: pure canonicalization + stable serializer only.
pub fn canonical_drift_json(d: &DriftReport) -> String {
    canonical_json_string(&value_from_serializable(d))
}

/// Canonical UTF-8 bytes for a capsule with optional noise stripping.
///
/// Purpose: define replay/hash comparison bytes independent of non-semantic envelope fields.
/// Invariant: stripped fields are exactly `version`, execution envelope keys, and replay-only evidence flags.
/// I/O: `&AICapsuleV1` + strip flags -> canonical UTF-8 JSON bytes.
/// Determinism: deterministic field removal + recursive canonical key ordering + stable byte encoding.
pub fn canonical_capsule_bytes(
    c: &AICapsuleV1,
    strip_version_metadata: bool,
    strip_execution_envelope: bool,
) -> Vec<u8> {
    let mut v = value_from_serializable(c);
    if let Value::Object(ref mut m) = v {
        if strip_version_metadata {
            m.remove("version");
        }
        if strip_execution_envelope {
            m.remove("execution_envelope");
            m.remove("execution_environment");
        }
        if let Some(Value::Object(ev)) = m.get_mut("evidence") {
            ev.remove("formal_replay_invariant_ok");
            ev.remove("cross_machine_replay_ok");
        }
    }
    let c = canonicalize_value(&v);
    serde_json::to_vec(&c).unwrap_or_else(|_| b"{}".to_vec())
}
