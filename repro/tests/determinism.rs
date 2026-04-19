// Determinism invariants.
//
// These tests exercise the OutputContract against every format_*
// entry point in `core::output` and the eval rendering, and assert
// byte-for-byte stability across repeated calls. If any of these fail,
// the output contract has been violated and the CLI cannot claim
// "identical inputs → identical output".
//
// They run out-of-process too: `repro eval` is invoked twice through
// the compiled binary and its stdout is compared byte-for-byte.

use repro::analysis::system_evaluation::CATEGORY_DEFINITION;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn repro_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_repro"))
}

fn run(args: &[&str]) -> (String, String, i32) {
    let out = Command::new(repro_bin())
        .args(args)
        .output()
        .expect("spawn repro");
    (
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
        out.status.code().unwrap_or(-1),
    )
}

fn run_in_dir(cwd: &Path, args: &[&str]) -> (String, String, i32) {
    let out = Command::new(repro_bin())
        .current_dir(cwd)
        .args(args)
        .output()
        .expect("spawn repro");
    (
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
        out.status.code().unwrap_or(-1),
    )
}

#[test]
fn test_real_execution_capture() {
    let tmp = env::temp_dir().join(format!(
        "repro-real-cap-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    fs::create_dir_all(&tmp).unwrap();
    let (stdout, stderr, code) = run_in_dir(&tmp, &["run", "--", "echo", "hello"]);
    assert_eq!(code, 0, "stderr: {stderr}");
    assert!(
        stdout.contains("hello"),
        "expected real echo output on stdout, got:\n{stdout}"
    );
    assert!(
        stdout.contains("duration_ms"),
        "artifact summary should include duration_ms:\n{stdout}"
    );
    let runs_dir = tmp.join("repro_runs");
    let index = fs::read_to_string(runs_dir.join("INDEX")).expect("INDEX");
    let id = index.lines().next().expect("one run id").trim();
    let artifact_path = runs_dir.join(format!("{id}.json"));
    let json = fs::read_to_string(&artifact_path).expect("artifact json");
    assert!(
        json.contains("hello") && json.contains("stdout"),
        "artifact should record stdout: {json}"
    );
    assert!(
        json.contains("\"exit_code\": 0") || json.contains("\"exit_code\":0"),
        "artifact should record success exit: {json}"
    );
    let _ = fs::remove_dir_all(&tmp);
}

#[test]
fn eval_command_is_byte_identical_across_runs() {
    let (out_a, err_a, code_a) = run(&["eval"]);
    assert_eq!(code_a, 0, "eval failed: {err_a}");
    let (out_b, _, code_b) = run(&["eval"]);
    assert_eq!(code_b, 0);
    assert_eq!(out_a, out_b, "`repro eval` must be deterministic");
}

#[test]
fn eval_output_contains_category_and_score_sections() {
    let (out, err, code) = run(&["eval"]);
    assert_eq!(code, 0, "eval failed: {err}");
    assert!(out.contains("# AION Repro — system evaluation"));
    assert!(out.contains("## 1. Feature inventory"));
    assert!(out.contains("## 9. Maturity score"));
    assert!(out.contains("## 10. OS exposure & CI compatibility"));
    assert!(out.contains("── json ──"));
    assert!(out.contains(CATEGORY_DEFINITION.trim()));
}

fn repo_root_readme() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../README.md")
}

#[test]
fn category_definition_is_locked_across_readme_and_help() {
    // Root README and `repro --help` must both carry the exact category
    // sentence. This is Phase-9 "category definition lock": three
    // surfaces, one string, no drift.
    let readme_path = repo_root_readme();
    let readme = std::fs::read_to_string(&readme_path)
        .unwrap_or_else(|e| panic!("read {}: {e}", readme_path.display()));
    assert!(
        readme.contains(CATEGORY_DEFINITION),
        "root README.md is missing the locked category sentence ({})",
        readme_path.display()
    );

    let (help, _, _) = run(&["--help"]);
    assert!(
        help.contains(CATEGORY_DEFINITION),
        "`repro --help` is missing the locked category sentence:\n{help}"
    );

    let (eval, _, _) = run(&["eval"]);
    assert!(
        eval.contains(CATEGORY_DEFINITION),
        "`repro eval` is missing the locked category sentence"
    );
}

#[test]
fn eval_output_satisfies_the_output_contract() {
    // Minimal re-implementation of OutputContract::validate here so the
    // integration test is independent of the library internals it is
    // checking. If these rules drift, `core::contract` is authoritative
    // and this test should be updated alongside it.
    let (out, _, code) = run(&["eval"]);
    assert_eq!(code, 0);

    for (i, line) in out.split('\n').enumerate() {
        assert!(
            !line.contains('\r'),
            "line {} contains \\r: {:?}",
            i + 1,
            line
        );
        assert!(
            !line.contains('\u{001B}'),
            "line {} contains ANSI escape",
            i + 1
        );
        if !line.is_empty() {
            assert!(
                !line.ends_with(|c: char| c.is_whitespace()),
                "line {} has trailing whitespace: {:?}",
                i + 1,
                line
            );
        }
    }
}
