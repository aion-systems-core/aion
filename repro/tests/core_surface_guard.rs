//! Enforce the `core/` visibility contract: no directory walks, and no
//! `std::process::Command` outside `capture.rs` (subprocess I/O boundary).

use std::fs;
use std::path::PathBuf;

fn core_rs_files() -> Vec<PathBuf> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/core");
    let mut out = Vec::new();
    walk_rs(&root, &mut out);
    out
}

fn walk_rs(dir: &std::path::Path, out: &mut Vec<PathBuf>) {
    for e in fs::read_dir(dir).expect("read_dir src/core") {
        let e = e.expect("dir entry");
        let p = e.path();
        if p.is_dir() {
            walk_rs(&p, out);
        } else if p.extension().and_then(|s| s.to_str()) == Some("rs") {
            out.push(p);
        }
    }
}

#[test]
fn core_has_no_read_dir_calls() {
    for p in core_rs_files() {
        let text = fs::read_to_string(&p).expect("read file");
        assert!(
            !text.contains("read_dir"),
            "{} must not call std::fs::read_dir (use append-only INDEX patterns instead)",
            p.display()
        );
    }
}

#[test]
fn command_new_only_in_capture_or_boundary() {
    for p in core_rs_files() {
        let text = fs::read_to_string(&p).expect("read file");
        let ok = matches!(
            p.file_name().and_then(|s| s.to_str()),
            Some("capture.rs") | Some("execution_boundary.rs")
        );
        if !ok && text.contains("Command::new") {
            panic!(
                "{} uses Command::new — subprocess spawn is restricted to capture / execution_boundary",
                p.display()
            );
        }
    }
}
