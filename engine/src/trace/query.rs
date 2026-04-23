//! Deterministic queries over a [`Trace`](super::model::Trace).

use super::model::{Trace, TraceSpan};

pub fn span_by_seq<'a>(t: &'a Trace, seq: u64) -> Option<&'a TraceSpan> {
    t.spans.iter().find(|s| s.seq == seq)
}

pub fn spans_with_op<'a>(t: &'a Trace, op: &str) -> Vec<&'a TraceSpan> {
    t.spans.iter().filter(|s| s.op == op).collect()
}

pub fn window<'a>(t: &'a Trace, from_seq: u64, to_seq: u64) -> Vec<&'a TraceSpan> {
    t.spans
        .iter()
        .filter(|s| s.seq >= from_seq && s.seq <= to_seq)
        .collect()
}
