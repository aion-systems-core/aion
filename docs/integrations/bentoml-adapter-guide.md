# BentoML Adapter Guide (Documentation Scaffold)

## Architecture

- BentoML service calls are instrumented for deterministic run capture.
- Tenant and RBAC context is applied before storage and replay operations.
- Governance policy engine validates output acceptability.

## Example flow

1. Invoke Bento endpoint with deterministic profile.
2. Record capsule + drift/replay evidence.
3. Evaluate policy constraints and emit governance events.

## Evidence capture points

- Request/response metadata
- Capsule and replay artifacts
- Governance decision artifacts

## Policy enforcement points

- Allowed model constraints
- External call constraints
- Required evidence fields
