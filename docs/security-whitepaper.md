# SealRun Security Whitepaper

## Overview

SealRun provides deterministic execution and audit-grade evidence for AI/automation workflows. This document summarizes core security architecture and enterprise controls.

## Deterministic execution model

- Execution outputs are serialized in deterministic envelopes.
- Replay verifies behavioral symmetry against preserved capsule state.
- Drift reporting isolates meaningful deltas for governance decisions.

## Evidence chain

- Capsule-linked evidence records provide provenance for replay/drift/governance outcomes.
- Policy decisions and governance events are represented as machine-readable artifacts.

## Tenancy isolation

- Enterprise storage is tenant-scoped.
- Capsule and evidence indexes are partitioned per tenant.
- Replay and query surfaces enforce tenant context.

## RBAC model

- Roles: `admin`, `auditor`, `operator`, `viewer`.
- Permissions include replay/diff/purge/retention/legal-hold/tenant-admin.
- Assignments are stored in YAML policy artifacts for reviewability.

## OIDC auth flow

- Device-code flow supports CLI-native enterprise authentication.
- Login, status, and logout are explicit CLI actions.
- Token state is persisted locally for authenticated sessions.

## Supply-chain security

- Release attestation integrates Cosign/Sigstore sign and verify operations.
- SBOM generation integrates `cargo sbom`.
- Attestation and SBOM outputs are structured as evidence artifacts for compliance workflows.
