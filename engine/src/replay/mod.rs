//! RunResult stdout replay and AI replay symmetry helpers.

mod cross_machine;
mod stdout;
mod symmetry;

pub use cross_machine::{machine_fingerprint_tolerant_equal, validate_cross_machine_replay};
pub use stdout::{replay_report, replay_stdout, ReplayReport};
pub use symmetry::{assert_formal_replay_invariant, assert_replay_symmetry};
