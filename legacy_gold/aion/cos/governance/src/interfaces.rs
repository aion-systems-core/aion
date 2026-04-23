//! Policy-facing capability markers only — no execution engines, no I/O, no v14 symbols.

/// Opaque clock abstraction for policy tests (never implemented here with real time I/O).
pub trait PolicyClock: Send + Sync {}

/// Marker for “something that can supply serialized policy inputs” without pulling runtime.
pub trait PolicyInputSource: Send + Sync {}
