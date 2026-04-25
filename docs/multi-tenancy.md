# Multi-tenancy

SealRun enterprise storage is tenant-aware and storage-isolated.

## Design

- Every capsule registration belongs to exactly one tenant.
- Capsule and evidence indexes are stored per tenant.
- Tenant delete is blocked when legal hold is enabled.

## CLI

```bash
sealrun enterprise tenants list
sealrun enterprise tenants create <id>
sealrun enterprise tenants delete <id>
sealrun enterprise tenants capsules list --tenant <id>
sealrun enterprise tenants capsules replay --tenant <id> --capsule <path>
sealrun enterprise tenants evidence query --tenant <id> --field <k> --value <v>
```
