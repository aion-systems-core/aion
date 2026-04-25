# Runbook: Incident - SIEM/OTel Exporter Failure

## Trigger

Sink or OTel export tests fail, or monitoring pipeline drops enterprise events.

## Steps

1. Run sink/OTel test commands and capture outputs.
2. Verify endpoint reachability, auth token validity, and payload shape.
3. Switch to fallback sink path if available.
4. Backfill missed events from deterministic artifact store.
5. Open vendor/provider incident if external outage is confirmed.

## Exit criteria

- Export path restored and validated.
- Event delivery gap assessed and remediated.
- Post-incident action items created.
