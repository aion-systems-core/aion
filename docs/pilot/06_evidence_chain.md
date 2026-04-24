# Pilot onboarding — Evidence chain

## Purpose

Pilot step for the **Evidence** layer: digest continuity inside capsules and sidecar files ([Evidence model](../evidence/evidence_model.md)).

The **evidence chain** is a linear sequence of records (`leaf_digest`, `payload_digest`, `parent_digest`) anchored to the run. Verifiers recompute rolling digests to detect tampering.

## Where it shows up

- Inside each **capsule** (`evidence` field).
- As standalone **evidence** artefacts in output bundles (paths vary by command).

## Operations

1. **Continuity** — Verify that each step’s `payload_digest` matches `hash(parent, leaf)` style linkage (see engine/core proof utilities).
2. **Optional signing** — Ed25519 over evidence bytes for external non-repudiation (see governance and release-signing contract flows in [Governance](../governance.md)).

## Next

- [Evidence model (diagram)](../evidence/evidence_model.md)  
- [Compliance one-pager](../compliance/sealrun_compliance_onepager.md)
