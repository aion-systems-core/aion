# Runbook: Incident - Drift Anomaly

## Trigger

Unexpected or high-severity drift detected in production or pre-release validation.

## Steps

1. Snapshot both compared artifacts and drift report.
2. Classify drift type (expected/unknown/critical).
3. Check policy bundle constraints for affected run.
4. Pause rollout if critical drift affects governed surfaces.
5. Create incident timeline and assign owner.

## Exit criteria

- Drift disposition documented.
- Governance decision recorded.
- Follow-up action tracked (fix, policy update, or exception).
