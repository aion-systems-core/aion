// CI ledger: storage ordering, env hash determinism, diff stability.

use repro::ci::diff::compare_ci_runs;
use repro::ci::environment::{capture_snapshot, compute_environment_hash};
use repro::ci::meta::CiExecutionContext;
use repro::ci::storage::{list_runs, resolve_alias, save_ci_run};
use repro::core::capture::{
    capture_command_real_with_clock, reset_counter_for_tests, FixedClock, CAPTURE_TEST_LOCK,
};

use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn repro_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_repro"))
}

fn tmpdir(tag: &str) -> PathBuf {
    let mut d = std::env::temp_dir();
    d.push(format!(
        "repro-ci-ledger-{}-{}",
        tag,
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&d).unwrap();
    d
}

#[test]
fn ci_cli_run_list_roundtrip() {
    let root = tmpdir("cli");
    let o = Command::new(repro_bin())
        .current_dir(&root)
        .args(["ci", "run", "--", "echo", "ci-mark"])
        .output()
        .expect("spawn");
    assert!(
        o.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&o.stderr)
    );

    let out = String::from_utf8_lossy(&o.stdout);
    assert!(out.contains("CI RUN SUCCESS"));
    assert!(out.contains("Run:"));

    let o2 = Command::new(repro_bin())
        .current_dir(&root)
        .args(["ci", "list"])
        .output()
        .expect("spawn");
    assert!(
        o2.status.success(),
        "{}",
        String::from_utf8_lossy(&o2.stderr)
    );
    let list = String::from_utf8_lossy(&o2.stdout);
    assert!(list.contains("total: 1"));
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn env_hash_stable_for_fixed_snapshot() {
    let s = capture_snapshot();
    let h1 = compute_environment_hash(&s);
    let h2 = compute_environment_hash(&s);
    assert_eq!(h1, h2);
}

#[test]
fn storage_index_ordering_stable() {
    let _g = CAPTURE_TEST_LOCK.lock().unwrap();
    let root = tmpdir("store");
    reset_counter_for_tests();
    let a1 = capture_command_real_with_clock("echo", &["a".into()], "echo a", &FixedClock(100));
    reset_counter_for_tests();
    let a2 = capture_command_real_with_clock("echo", &["b".into()], "echo b", &FixedClock(100));
    save_ci_run(&root, &a1, &CiExecutionContext::local_default()).unwrap();
    save_ci_run(&root, &a2, &CiExecutionContext::local_default()).unwrap();
    let ids = list_runs(&root).unwrap();
    assert_eq!(ids.len(), 2);
    assert_eq!(resolve_alias(&root, "last").unwrap(), a2.run_id);
    assert_eq!(resolve_alias(&root, "prev").unwrap(), a1.run_id);
    let _ = fs::remove_dir_all(&root);
}

#[test]
fn diff_semantic_buckets_stable() {
    let _g = CAPTURE_TEST_LOCK.lock().unwrap();
    let root = tmpdir("diff");
    reset_counter_for_tests();
    let a1 = capture_command_real_with_clock("echo", &["x".into()], "echo x", &FixedClock(1));
    reset_counter_for_tests();
    let a2 = capture_command_real_with_clock("echo", &["y".into()], "echo y", &FixedClock(1));
    let r1 = compare_ci_runs(&a1, &a2);
    reset_counter_for_tests();
    let b1 = capture_command_real_with_clock("echo", &["x".into()], "echo x", &FixedClock(1));
    reset_counter_for_tests();
    let b2 = capture_command_real_with_clock("echo", &["y".into()], "echo y", &FixedClock(1));
    let r2 = compare_ci_runs(&b1, &b2);
    assert_eq!(r1.semantic_classification, r2.semantic_classification);
    let _ = fs::remove_dir_all(&root);
}
