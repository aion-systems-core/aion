# Telemetry (opt-in)

SealRun telemetry is disabled by default.

## At a glance

- Telemetry is optional and off by default.
- Deterministic contract outputs do not require telemetry.
- Local execution and local artifacts remain first-class.

Use CLI commands to manage preference:

```bash
sealrun telemetry enable
sealrun telemetry status
sealrun telemetry disable
```

Preference file path:

- `%USERPROFILE%/.sealrun/telemetry.toml` (Windows)
- `$HOME/.sealrun/telemetry.toml` (Unix-like)

## CLI surface

```bash
sealrun telemetry enable
sealrun telemetry status
sealrun telemetry disable
```

## Enterprise-readiness

Enterprise usage can keep telemetry disabled while preserving deterministic auditability via doctor and contract artifacts.
