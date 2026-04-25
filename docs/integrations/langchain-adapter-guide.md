# LangChain Adapter Guide (Documentation Scaffold)

## Architecture

- LangChain chain/agent outputs are wrapped in SealRun execution envelopes.
- Tenant-aware capsule registration is applied post-run.
- Policy and evidence checks gate downstream workflows.

## Example flow

1. Run chain invocation.
2. Persist deterministic capsule and evidence sidecars.
3. Evaluate governance policy and export events to SIEM/OTel.

## Evidence capture points

- Chain config snapshot
- Capsule replay artifacts
- Policy-evaluation output

## Policy enforcement points

- Allowed model registry
- Allowed external tools/endpoints
- Evidence field completeness checks
