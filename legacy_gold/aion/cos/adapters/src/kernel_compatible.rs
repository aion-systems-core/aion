//! Test-driven semantic bridge — **transform only**, no policy (Phase 5.7).
//!
//! Concrete legacy payloads are represented as [`serde_json::Value`] with documented
//! shapes so `cos_adapters` stays free of `cognitive_os_v14` / `cos-v1` path dependencies
//! (avoids workspace cycles).

/// Maps a documented legacy representation into a single `cos_core` kernel DTO.
pub trait KernelCompatible {
    type Target;
    /// **Panics** if the mapping is not total for the given payload (tests catch drift).
    fn to_kernel(&self) -> Self::Target;
}
