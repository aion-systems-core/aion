//! Zero-copy views over [`EventStore`](super::store::EventStore): iterate, filter, map, fold;
//! projections for replay, diff, why, and graph.

use super::model::{EventCategory, EventEnvelope, RunEvent};
use super::store::EventStore;
use aion_core::{DriftReport, DriftToleranceProfile, WhyReport};
use serde_json::{json, Value};

/// Borrowed view over ordered envelopes (immutable store).
#[derive(Debug, Clone, Copy)]
pub struct EventReader<'a> {
    inner: &'a [EventEnvelope],
}

impl<'a> EventReader<'a> {
    pub fn new(store: &'a EventStore) -> Self {
        Self {
            inner: store.as_slice(),
        }
    }

    pub fn from_slice(slice: &'a [EventEnvelope]) -> Self {
        Self { inner: slice }
    }

    pub fn iter(&self) -> impl Iterator<Item = &'a EventEnvelope> + Clone {
        self.inner.iter()
    }

    pub fn filter<'b, F>(&'b self, mut f: F) -> impl Iterator<Item = &'a EventEnvelope> + 'b
    where
        F: FnMut(&'a EventEnvelope) -> bool + 'b,
    {
        self.inner.iter().filter(move |ev| f(*ev))
    }

    pub fn filter_category(
        &self,
        cat: EventCategory,
    ) -> impl Iterator<Item = &'a EventEnvelope> + '_ {
        self.inner.iter().filter(move |e| e.event.category() == cat)
    }

    /// Deterministic `map` + materialize (avoids RPITIT capture issues).
    pub fn map_collect<B, F>(&self, f: F) -> Vec<B>
    where
        F: FnMut(&'a EventEnvelope) -> B,
    {
        self.inner.iter().map(f).collect()
    }

    pub fn fold<B, F>(&self, init: B, f: F) -> B
    where
        F: FnMut(B, &'a EventEnvelope) -> B,
    {
        self.inner.iter().fold(init, f)
    }

    /// Concatenate stdout chunks in `seq` order.
    pub fn replay_stdout(&self) -> String {
        self.inner
            .iter()
            .filter_map(|e| match &e.event {
                RunEvent::StdoutChunk { text } => Some(text.as_str()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .concat()
    }

    /// Concatenate stderr chunks in `seq` order.
    pub fn replay_stderr(&self) -> String {
        self.inner
            .iter()
            .filter_map(|e| match &e.event {
                RunEvent::StderrChunk { text } => Some(text.as_str()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .concat()
    }

    /// Deterministic JSON graph: linear chain `seq -> seq+1` plus category nodes.
    pub fn to_graph_json(&self) -> Result<String, String> {
        let nodes: Vec<Value> = self
            .inner
            .iter()
            .map(|e| {
                json!({
                    "id": format!("evt:{}", e.seq),
                    "seq": e.seq,
                    "category": format!("{:?}", e.event.category()),
                })
            })
            .collect();
        let mut edges = Vec::new();
        for w in self.inner.windows(2) {
            edges.push(json!({
                "from": format!("evt:{}", w[0].seq),
                "to": format!("evt:{}", w[1].seq),
            }));
        }
        Ok(json!({ "nodes": nodes, "edges": edges }).to_string())
    }

    /// Compare canonical summaries of two readers (stable JSON array diff at text level).
    pub fn diff_summaries(left: &EventReader<'_>, right: &EventReader<'_>) -> DriftReport {
        let ls = left
            .inner
            .iter()
            .map(envelope_stable_line)
            .collect::<Vec<_>>()
            .join("\n");
        let rs = right
            .inner
            .iter()
            .map(envelope_stable_line)
            .collect::<Vec<_>>()
            .join("\n");
        let mut fields = Vec::new();
        let mut labels = Vec::new();
        if ls != rs {
            fields.push("event_stream".into());
            labels.push("shape:event_stream_canonical_mismatch".into());
        }
        fields.sort();
        fields.dedup();
        labels.sort();
        labels.dedup();
        DriftReport {
            changed: !fields.is_empty(),
            categories: if labels.is_empty() {
                vec![]
            } else {
                vec!["shape".into()]
            },
            labels: labels.clone(),
            fields,
            details: labels,
            tolerance_profile: DriftToleranceProfile::deterministic_default(),
            tolerance_violations: vec![],
            overflow: false,
            error: None,
        }
    }

    /// Lightweight “why” from stream shape (pair with artifact diff for full story).
    pub fn why_against(&self, baseline: &EventReader<'_>) -> WhyReport {
        let d = EventReader::diff_summaries(baseline, self);
        if !d.changed {
            return WhyReport {
                summary: "Event streams are identical under canonical line projection.".into(),
                first_divergent_field: None,
                suggestion: None,
            };
        }
        WhyReport {
            summary: "Event stream divergence detected (ordering or payload).".into(),
            first_divergent_field: Some("event_stream".into()),
            suggestion: Some("Diff canonical summaries or inspect per-category iterators.".into()),
        }
    }
}

fn envelope_stable_line(e: &EventEnvelope) -> String {
    let v = json!({
        "seq": e.seq,
        "attrs": e.attrs,
        "event": e.event,
    });
    v.to_string()
}
