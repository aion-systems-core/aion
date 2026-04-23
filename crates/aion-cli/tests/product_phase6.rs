use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[test]
fn test_phase6_product_files_exist() {
    let root = repo_root();
    let required = [
        ".github/workflows/ci.yml",
        ".github/workflows/release.yml",
        ".github/workflows/docker.yml",
        ".github/workflows/benchmark.yml",
        ".github/workflows/coverage.yml",
        ".github/workflows/signed-release.yml",
        ".github/ISSUE_TEMPLATE/bug_report.yml",
        ".github/ISSUE_TEMPLATE/feature_request.yml",
        "LICENSE",
        "Dockerfile",
        "docker-compose.demo.yml",
        "docs/faq.md",
        "docs/compatibility-matrix.md",
        "docs/migration.md",
        "docs/training.md",
        "docs/installers.md",
        "docs/telemetry.md",
        "docs/case-studies.md",
        "docs/whitepaper.md",
        "docs/community.md",
        "docs/benchmarks.md",
        "docs/enterprise-license.md",
        "docs/videos.md",
        "docs/feedback-survey.md",
        "packaging/homebrew/aion.rb",
        "packaging/rpm/aion.spec",
        "packaging/apt/README.md",
        "scripts/migrate_output_layout.ps1",
        "website/index.html",
    ];
    for rel in required {
        let p = root.join(rel);
        assert!(p.exists(), "missing required phase6 file: {}", p.display());
    }
}
