# Lifecycle controls

Tenant lifecycle controls define retention, purge, and legal hold behavior.

## Controls

- Retention policy is configured per tenant (`days`).
- Purge removes expired capsule registrations.
- Legal hold blocks purge and tenant deletion.

## CLI

```bash
sealrun enterprise lifecycle retention get --tenant <id>
sealrun enterprise lifecycle retention set --tenant <id> --days 30
sealrun enterprise lifecycle purge --tenant <id>
sealrun enterprise lifecycle legal-hold enable --tenant <id>
sealrun enterprise lifecycle legal-hold disable --tenant <id>
```
