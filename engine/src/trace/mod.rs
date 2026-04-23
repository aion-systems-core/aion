//! Trace engine: connects [`RunResult`](aion_core::RunResult) to [`EventStore`](crate::events::EventStore) semantics.

mod builder;
mod model;
mod query;

pub use builder::trace_from_run;
pub use model::{Trace, TraceSpan};
pub use query::{span_by_seq, spans_with_op, window};
