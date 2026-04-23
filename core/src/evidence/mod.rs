//! Evidence chain: deterministic digests over run + policy + determinism.

mod chain;
mod hashing;
mod proof;

pub use chain::{EvidenceChain, EvidenceContract, EvidenceRecord, EvidenceReplayAnchors};
pub use hashing::sha256_hex;
pub use proof::{seal_run, verify_linear};
