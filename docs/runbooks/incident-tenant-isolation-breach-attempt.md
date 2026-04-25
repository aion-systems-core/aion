# Runbook: Incident - Tenant Isolation Breach Attempt

## Trigger

Cross-tenant capsule or evidence access attempt detected.

## Steps

1. Capture request context, actor, and attempted resource.
2. Confirm RBAC and tenant binding evaluations.
3. Block subject/session and preserve relevant logs.
4. Verify no unauthorized data access occurred.
5. Notify security and compliance stakeholders.

## Exit criteria

- Attempt classified and contained.
- Access controls validated or patched.
- Incident report and corrective actions completed.
