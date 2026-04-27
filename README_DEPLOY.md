# Deploy and distribute (pilot)

## Overview

This file satisfies the repository **README_DEPLOY** entry point for packaging and deployment pointers. Canonical technical content lives under `docs/`.

## Installers and packages

- Installer and distribution overview: [docs/installers.md](docs/installers.md)  
- Homebrew, RPM, APT metadata: [packaging/homebrew/aion.rb](packaging/homebrew/aion.rb), [packaging/rpm/aion.spec](packaging/rpm/aion.spec), [packaging/apt/README.md](packaging/apt/README.md)

## Pilot packaging

- **Release zip / CST checklist:** [docs/enterprise/pilot/zip-new-cst.md](docs/enterprise/pilot/zip-new-cst.md)  
- **DSj12 deterministic ship checklist:** [docs/enterprise/pilot/DSj12-checklist.md](docs/enterprise/pilot/DSj12-checklist.md)

## Build and verify

- Build the CLI from source per [README.md](README.md) (Developer Quickstart).  
- Before customer handoff, run workspace tests: `cargo test --workspace --all-targets`.

## Next steps

Complete the pilot evaluation using [docs/enterprise/pilot/EVAL_REPORT_TEMPLATE.md](docs/enterprise/pilot/EVAL_REPORT_TEMPLATE.md).
