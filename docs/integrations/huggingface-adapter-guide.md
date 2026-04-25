# HuggingFace Adapter Guide (Documentation Scaffold)

## Architecture

- HuggingFace model invocation emits deterministic run output.
- Capsule artifacts are generated and attached to tenant context.
- Governance evaluation gates acceptance decisions.

## Example flow

1. Execute model call through adapter boundary.
2. Capture replay/drift-capable capsule output.
3. Evaluate policy constraints and record decision.

## Evidence capture points

- Invocation metadata
- Capsule and replay outputs
- Governance decision output

## Policy enforcement points

- Allowed model list
- Allowed external call host
- Required evidence fields
