# DSj12 — deterministic ship checklist (12 items)

## Overview

Twelve-item checklist for **D**eterministic **S**hipping with **J**ustified controls before a pilot or customer receives bits. Complete in order; do not skip integrity steps.

## Checklist

| # | Item | Owner | Done (Y/N) |
|---|------|-------|------------|
| 1 | Release commit frozen and tagged | | |
| 2 | CI green on that commit (fmt, clippy, tests) | | |
| 3 | Docs link check passed (`cargo test -p aion-cli test_docs_links`) | | |
| 4 | Compatibility matrix reviewed for target OS | | |
| 5 | SBOM generated (if required) | | |
| 6 | Signatures / checksums attached | | |
| 7 | Secrets scan on bundle path | | |
| 8 | License file included and matches entitlement | | |
| 9 | Support and escalation path communicated | | |
| 10 | Rollback plan documented | | |
| 11 | Data classification for pilot data agreed | | |
| 12 | Recipient verification of checksums before install | | |

## Evidence

Link to tickets, CI runs, and attestation artifacts here:

- |

## Next steps

Store the signed checklist with your release record and reference it from [EVAL_REPORT_TEMPLATE.md](EVAL_REPORT_TEMPLATE.md).
