# Policy engine

The enterprise policy engine validates and evaluates deterministic governance rules.

## Policy schema

```json
{
  "allowed_models": ["gpt-4o-mini"],
  "allowed_seeds": [1, 42],
  "allowed_external_calls": ["https://api.example.com"],
  "required_evidence_fields": ["trace_id", "policy_id"]
}
```

## Evaluation input

```json
{
  "model": "gpt-4o-mini",
  "seed": 42,
  "external_calls": ["https://api.example.com"],
  "evidence_fields": {"trace_id": "t1", "policy_id": "p1"}
}
```

## CLI

```bash
sealrun enterprise policy-api validate --policy policy.json
sealrun enterprise policy-api evaluate --policy policy.json --input input.json
```
