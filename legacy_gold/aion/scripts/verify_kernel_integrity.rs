//! Phase 5.6 — kernel integrity checks (no workspace deps; `std` only).
//!
//! Run from repo root:
//!   cargo run -p kernel_integrity
//!
//! Exit code: 0 = pass, 1 = violation.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

fn main() {
    let root = repo_root();
    if !root.join("cos").join("core").is_dir() {
        eprintln!("kernel_integrity: expected cos/core under repo root: {}", root.display());
        process::exit(1);
    }

    let mut violations: Vec<String> = Vec::new();

    collect_rs(&root, &root, &mut violations);

    if !violations.is_empty() {
        eprintln!("kernel_integrity: FAILED\n");
        for v in &violations {
            eprintln!("  - {v}");
        }
        process::exit(1);
    }

    println!("kernel_integrity: OK (repo root {})", root.display());
}

fn repo_root() -> PathBuf {
    // scripts/kernel_integrity/Cargo.toml → manifest dir is .../scripts/kernel_integrity
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("manifest dir")
        .parent()
        .expect("scripts dir parent = repo root")
        .to_path_buf()
}

fn collect_rs(root: &Path, dir: &Path, violations: &mut Vec<String>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for e in entries.flatten() {
        let p = e.path();
        let name = e.file_name().to_string_lossy().into_owned();
        if p.is_dir() {
            if name == "target" || name == ".git" || name == "node_modules" {
                continue;
            }
            collect_rs(root, &p, violations);
            continue;
        }
        if p.extension().and_then(|s| s.to_str()) != Some("rs") {
            continue;
        }
        scan_file(root, &p, violations);
    }
}

fn norm(p: &Path) -> String {
    p.to_string_lossy().replace('\\', "/")
}

fn scan_file(root: &Path, path: &Path, violations: &mut Vec<String>) {
    let rel = norm(path.strip_prefix(root).unwrap_or(path));
    if rel.contains("/target/") {
        return;
    }

    let Ok(text) = fs::read_to_string(path) else {
        return;
    };

    for (i, raw) in text.lines().enumerate() {
        let line = raw.trim_start();
        if line.starts_with("//") {
            continue;
        }

        let trim = line.trim_start();
        if trim.starts_with("use ") && trim.contains("cos_governance") && !rel.contains("cos/runtime/") {
            violations.push(format!(
                "{rel}:{} — direct cos_governance import outside cos/runtime",
                i + 1
            ));
        }

        if line_has_pub_struct(line, "AuditRecord") && !rel.contains("cos/core/") {
            violations.push(format!(
                "{rel}:{} — kernel audit row struct outside cos/core",
                i + 1
            ));
        }

        if line_has_pub_struct(line, "AuditChain")
            && !rel.contains("cos/core/")
            && !(rel.contains("cognitive_os_v14") && rel.ends_with("builder/audit.rs"))
        {
            violations.push(format!(
                "{rel}:{} — pub struct AuditChain outside cos/core (and not build-audit allowlist)",
                i + 1
            ));
        }

        if line_has_pub_struct(line, "EvidenceRecordV2") && !rel.contains("cos/core/") {
            violations.push(format!(
                "{rel}:{} — pub struct EvidenceRecordV2 outside cos/core",
                i + 1
            ));
        }

        if line_defines_legacy_pub_struct_evidence_record(line)
            && !rel.replace('\\', "/").starts_with("cos/core/src/evidence/")
        {
            violations.push(format!(
                "{rel}:{} — pub struct EvidenceRecord (non-V2) outside cos/core/src/evidence/",
                i + 1
            ));
        }

        if line_has_pub_struct(line, "ReplayRecord") && !rel.contains("cos/core/") {
            violations.push(format!(
                "{rel}:{} — pub struct ReplayRecord outside cos/core",
                i + 1
            ));
        }
    }
}

fn line_has_pub_struct(line: &str, name: &str) -> bool {
    let p = format!("pub struct {name}");
    line.starts_with(&p) || line.starts_with(&format!("pub(crate) struct {name}"))
}

/// True for `pub struct EvidenceRecord` / `pub(crate) struct EvidenceRecord` but not `EvidenceRecordV2`.
fn line_defines_legacy_pub_struct_evidence_record(line: &str) -> bool {
    let t = line.trim_start();
    let rest = if let Some(r) = t.strip_prefix("pub struct EvidenceRecord") {
        r
    } else if let Some(r) = t.strip_prefix("pub(crate) struct EvidenceRecord") {
        r
    } else {
        return false;
    };
    if rest.starts_with("V2") {
        return false;
    }
    rest.is_empty()
        || rest.starts_with(' ')
        || rest.starts_with('{')
        || rest.starts_with('<')
        || rest.starts_with('\t')
}
