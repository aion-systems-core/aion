//! Deterministic-friendly wall clock (seconds). Freeze via [`FrozenClock`].

/// Seconds since UNIX epoch (best-effort).
pub fn now_secs() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

/// Fixed clock for reproducible captures.
#[derive(Debug, Clone, Copy)]
pub struct FrozenClock(pub u64);

impl FrozenClock {
    pub fn now_secs(&self) -> u64 {
        self.0
    }
}
