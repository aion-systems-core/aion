# Telemetry (opt-in)

AION telemetry is disabled by default.

## At a glance

- Telemetry is optional and off by default.
- Deterministic contract outputs do not require telemetry.
- Local execution and local artifacts remain first-class.

Use CLI commands to manage preference:

```bash
aion telemetry enable
aion telemetry status
aion telemetry disable
```

Preference file path:

- `%USERPROFILE%/.aion/telemetry.toml` (Windows)
- `$HOME/.aion/telemetry.toml` (Unix-like)

## CLI surface

```bash
aion telemetry enable
aion telemetry status
aion telemetry disable
```

## Enterprise-readiness

Enterprise usage can keep telemetry disabled while preserving deterministic auditability via doctor and contract artifacts.
