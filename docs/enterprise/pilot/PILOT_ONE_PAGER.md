# SealRun pilot — one pager

## Overview

One-page summary for security, platform, and engineering stakeholders.

## Problem

Teams need **deterministic** execution, **replayable** evidence, and **policy-governed** AI workloads without losing auditability.

## What SealRun does (pilot scope)

- **Capsule** execution with structured outputs  
- **Replay** to verify identical behavior for the same inputs  
- **Drift** observation when behavior diverges from baseline  
- **Policy evaluation** and governance decisions on critical paths  
- **Evidence** export suitable for audit and SIEM / OTel handoff (as configured)

## Pilot success (examples)

- Golden path completes: install → execute → replay → drift → policy → evidence  
- Documented incident response for at least one synthetic failure (replay mismatch or policy violation)  
- Evaluation report completed using [EVAL_REPORT_TEMPLATE.md](EVAL_REPORT_TEMPLATE.md)

## Out of scope (unless explicitly added)

- Production SLAs beyond the agreed pilot window  
- Custom hardware attestation programs  
- Third-party model training or fine-tuning

## Risks and mitigations

| Risk | Mitigation |
|------|------------|
| Environment drift | Freeze OS and dependency versions for pilot duration |
| Scope creep | Written change control with named approvers |
| Data handling | Pilot data classification per [data-classification.md](data-classification.md) |

## Contacts

- **Pilot sponsor (customer):** [name]  
- **Engineering lead (customer):** [name]  
- **SealRun contact:** [email]

## Next steps

1. Read [procurement-mini-pack.md](procurement-mini-pack.md).  
2. Run the hands-on sequence `00`–`06`.  
3. Book closeout review and submit the evaluation report.
