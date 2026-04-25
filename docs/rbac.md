# RBAC

SealRun enterprise RBAC uses a YAML policy file and deterministic evaluator.

## Roles

- `admin`
- `auditor`
- `operator`
- `viewer`

## Permissions

- `replay`
- `diff`
- `purge`
- `retention-set`
- `legal-hold`
- `tenant-admin`

## Policy file

Stored at `sealrun_enterprise/rbac.policy.yaml`:

```yaml
assignments:
  alice: admin
  bob: viewer
```

## CLI

```bash
sealrun enterprise rbac assign --subject alice --role admin
sealrun enterprise rbac check --subject alice --permission tenant-admin
sealrun enterprise rbac export
```
