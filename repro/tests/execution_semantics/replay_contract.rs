//! Concatenated Stdout events must equal aggregate artifact stdout.

use super::{assert_repro_ok, load_latest_artifact, run_repro, scratch_dir};
use repro::core::replay::replay_stdout;

#[test]
fn replay_stdout_equals_artifact_stdout() {
    let cwd = scratch_dir("replay_contract");
    let out = run_repro(&cwd, &["run", "--", "echo", "hello"]);
    assert_repro_ok(&out, "repro run");

    let art = load_latest_artifact(&cwd);
    assert_eq!(
        replay_stdout(&art.trace),
        art.stdout,
        "replay from trace must match artifact.stdout"
    );
}
