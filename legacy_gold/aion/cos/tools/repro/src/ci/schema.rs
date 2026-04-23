// CI execution artifact schema — versioned, immutable contract.
//
// **CI execution is immutable truth:** once written under `./repro_ci_store/`,
// an artifact directory is never mutated in place. New runs append new
// rows to `INDEX.jsonl`. Breaking changes to `artifact.json` require a
// new `SCHEMA_VERSION` and an explicit migration path (future work: add
// `migrate_v1_to_v2()` when `SCHEMA_VERSION` bumps).

/// Current on-disk schema for `artifact.json` inside each run directory.
/// v4: unified [`crate::core::artifact::ExecutionArtifact`] (same as `repro_runs/`).
pub const ARTIFACT_SCHEMA_VERSION: u32 = 3;

/// Store layout version for the whole `./repro_ci_store/` tree (v2 adds `meta.json`).
pub const CI_STORE_LAYOUT_VERSION: u32 = 2;
