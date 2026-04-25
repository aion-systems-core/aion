# SIEM and OTel

SealRun enterprise can export governance/capsule events to SIEM and OTel endpoints.

## Supported sinks

- Splunk HEC
- Datadog Logs
- Elastic Ingest

## CLI

```bash
sealrun enterprise sinks send-test --sink splunk --endpoint <url> --token <token>
sealrun enterprise sinks send-test --sink datadog --endpoint <url> --token <token>
sealrun enterprise sinks send-test --sink elastic --endpoint <url> --token <token>
sealrun enterprise otel export --endpoint <otlp-http-endpoint>
```
