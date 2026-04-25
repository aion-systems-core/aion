# Modal Adapter Guide (Documentation Scaffold)

## Architecture

- Modal job execution emits deterministic output bundles.
- Tenant context is passed through execution and storage indexes.
- Policy decision and attestation evidence become pipeline outputs.

## Example flow

1. Submit Modal task with fixed seed/profile.
2. Collect capsule and replay evidence.
3. Enforce policy and push audit events to observability sinks.

## Evidence capture points

- Job metadata and deterministic inputs
- Capsule/evidence artifacts
- Governance and attestation outputs

## Policy enforcement points

- Seed allow-list
- External-call allow-list
- Required audit fields
