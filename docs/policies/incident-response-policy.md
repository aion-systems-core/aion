# Incident Response Policy

## Purpose

Provide a standardized response to security, reliability, and compliance incidents.

## Policy

- Incidents are triaged by severity (SEV1-SEV4).
- Initial acknowledgment follows SLA targets in `docs/sla.md`.
- A designated incident commander owns coordination.
- Containment and evidence preservation are mandatory before remediation.
- Post-incident review is required for SEV1/SEV2 within 5 business days.

## Related runbooks

- `docs/runbooks/incident-replay-failure.md`
- `docs/runbooks/incident-drift-anomaly.md`
- `docs/runbooks/incident-evidence-corruption.md`
- `docs/runbooks/incident-tenant-isolation-breach-attempt.md`
- `docs/runbooks/incident-siem-otel-exporter-failure.md`
