# OIDC auth

SealRun enterprise auth supports OIDC device-code flow in CLI mode.

## Login flow

1. CLI requests a device code from the IdP.
2. User opens verification URL and enters user code.
3. CLI polls token endpoint and stores token locally.

## CLI

```bash
sealrun enterprise auth login \
  --client-id <client> \
  --device-authorization-endpoint <url> \
  --token-endpoint <url>
sealrun enterprise auth status
sealrun enterprise auth logout
```
