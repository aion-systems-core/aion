//! CI run metadata (deterministic JSON).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CiRunMeta {
    pub schema_version: u32,
    pub baseline_file_version: u32,
    pub recorded_at_epoch_secs: u64,
}
