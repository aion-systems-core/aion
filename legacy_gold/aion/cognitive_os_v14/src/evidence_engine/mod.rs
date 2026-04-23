//! OS-level evidence records, timelines, index, diff, and replay (no policy).
//!
//! **Not** [`cos_core::evidence`]: kernel exposes `EvidenceRecordV2` / chain
//! stubs; this module is the **runtime** evidence index and timeline for workflows.

pub mod diff;
pub mod index;
pub mod model;
pub mod record;
pub mod replay;
pub mod timeline;

pub use diff::{diff, EvidenceDiff};
pub use index::EvidenceIndex;
pub use model::EvidenceModel;
pub use record::EvidenceTimelineRow;
pub use replay::replay;
pub use timeline::EvidenceTimeline;
