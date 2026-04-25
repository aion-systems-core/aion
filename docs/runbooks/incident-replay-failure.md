# Runbook: Incident - Replay Failure

## Trigger

Replay fails for an expected deterministic capsule.

## Steps

1. Capture replay output and command context.
2. Confirm tenant context and capsule path integrity.
3. Run policy validation for the same capsule.
4. Compare with last known successful replay artifact.
5. Escalate if deterministic contract breach is confirmed.

## Exit criteria

- Root cause classified.
- Mitigation applied or rollback initiated.
- Evidence artifacts attached to incident ticket.
