# Runbook: Incident - Evidence Corruption

## Trigger

Evidence artifact cannot be parsed, verified, or linked to expected capsule lineage.

## Steps

1. Quarantine affected evidence files.
2. Validate hash/provenance relationships.
3. Attempt deterministic regeneration from source capsule.
4. Check storage integrity and recent lifecycle actions.
5. Escalate to security if tampering is suspected.

## Exit criteria

- Corrupted artifacts replaced or invalidated.
- Integrity impact assessment completed.
- Preventive control action recorded.
