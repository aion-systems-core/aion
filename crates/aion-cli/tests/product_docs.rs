//! Documentation link sanity (repo-relative markdown links).

use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn collect_md(dir: &Path, acc: &mut Vec<PathBuf>) {
    if !dir.is_dir() {
        return;
    }
    for e in fs::read_dir(dir).expect("read_dir") {
        let e = e.expect("dirent");
        let p = e.path();
        if p.is_dir() {
            collect_md(&p, acc);
        } else if p.extension().and_then(|s| s.to_str()) == Some("md") {
            acc.push(p);
        }
    }
}

fn extract_markdown_links(md: &str) -> Vec<String> {
    let re = Regex::new(r"!?\[([^\]]*)\]\(([^)]+)\)").expect("regex");
    re.captures_iter(md)
        .filter(|c| !c.get(0).unwrap().as_str().starts_with("!["))
        .filter_map(|c| c.get(2).map(|m| m.as_str().to_string()))
        .collect()
}

fn resolve_link_simple(md_path: &Path, raw: &str) -> Option<PathBuf> {
    let t = raw.trim();
    if t.is_empty()
        || t.starts_with('#')
        || t.starts_with("mailto:")
        || t.contains("://")
    {
        return None;
    }
    let base = md_path.parent().unwrap_or(Path::new("."));
    Some(base.join(t))
}

#[test]
fn test_docs_links() {
    let root = repo_root();
    let mut files = Vec::new();
    collect_md(&root.join("docs"), &mut files);
    files.push(root.join("README.md"));
    files.push(root.join("CONTRIBUTING.md"));

    let mut missing = Vec::new();
    for md_path in files {
        if !md_path.exists() {
            continue;
        }
        let text = fs::read_to_string(&md_path).expect("read md");
        for url in extract_markdown_links(&text) {
            let Some(target) = resolve_link_simple(&md_path, &url) else {
                continue;
            };
            if !target.exists() {
                missing.push(format!("{} -> {}", md_path.display(), url));
            }
        }
    }
    assert!(missing.is_empty(), "broken links:\n{}", missing.join("\n"));
}
