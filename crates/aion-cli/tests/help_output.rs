//! Determinism and structure checks for `sealrun --help` (section order, terminology, stability).

use std::process::Command;

fn help(args: &[&str]) -> String {
    let out = Command::new(env!("CARGO_BIN_EXE_sealrun"))
        .args(args)
        .output()
        .expect("spawn sealrun");
    assert!(
        out.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8_lossy(&out.stdout).into_owned()
}

#[test]
fn help_output_is_deterministic() {
    let a = help(&["--help"]);
    let b = help(&["--help"]);
    assert_eq!(a, b, "help output must be deterministic");
}

#[test]
fn help_contains_required_sections_in_order() {
    let s = help(&["--help"]);
    let required = ["USAGE", "ARGS", "FLAGS", "OPTIONS", "EXAMPLES"];
    let mut pos = 0usize;
    for h in required {
        let i = s[pos..]
            .find(h)
            .map(|x| pos + x)
            .unwrap_or_else(|| panic!("missing section: {h}"));
        pos = i + h.len();
    }
}

#[test]
fn help_sections_are_not_duplicated() {
    let s = help(&["--help"]);
    for h in ["USAGE", "ARGS", "FLAGS", "OPTIONS", "EXAMPLES"] {
        let needle = format!("\n{h}\n");
        let count = s.matches(&needle).count();
        assert_eq!(count, 1, "section {h} appears {count} times");
    }
}

#[test]
fn help_terminology_is_consistent() {
    let s = help(&["--help"]);
    for term in ["capsule", "evidence", "policy", "profile", "replay"] {
        assert!(
            s.to_lowercase().contains(term),
            "missing canonical term: {term}"
        );
    }
}
