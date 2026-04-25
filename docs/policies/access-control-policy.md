# Access Control Policy

## Purpose

Define authentication, authorization, and tenant-boundary controls for SealRun enterprise environments.

## Policy

- All privileged enterprise actions require authenticated identity.
- OIDC is the default enterprise authentication mechanism.
- Authorization is role-based: `admin`, `auditor`, `operator`, `viewer`.
- Access is least-privilege and granted by explicit assignment.
- Tenant boundaries must be enforced in storage, replay lookup, and evidence queries.
- Access changes are tracked as governance events.

## Control checks

- `sealrun enterprise auth status`
- `sealrun enterprise rbac export`
- `sealrun enterprise tenants list`
