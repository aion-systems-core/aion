//! CI engine 2.0: schema, baseline, runtime, failure taxonomy.

mod baseline;
mod bundle;
mod failure;
mod meta;
mod runtime;
mod schema;

pub use baseline::{check_baseline, record_baseline};
pub use bundle::{CiDriftBundle, CiRunBundle};
pub use failure::FailureKind;
pub use meta::CiRunMeta;
pub use runtime::{check_with_meta, CiCheckOutcome};
pub use schema::{BASELINE_FILE_VERSION, CI_LEDGER_SCHEMA_VERSION};
