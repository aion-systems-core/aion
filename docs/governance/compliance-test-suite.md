# Governance Compliance Test Suite

## Objective

Define repeatable checks for policy bundle validity and runtime enforcement behavior.

## Test categories

- Schema validity: required keys exist in bundle YAML.
- Rule validity: allowed model/seed/external-call constraints parse correctly.
- Positive evaluation: compliant input returns pass.
- Negative evaluation: disallowed model/seed/external-call returns violations.
- Evidence enforcement: missing required evidence fields returns violations.

## Suggested command flow

1. Convert selected bundle YAML to policy JSON payload.
2. Run `sealrun enterprise policy-api validate --policy <policy.json>`.
3. Run `sealrun enterprise policy-api evaluate --policy <policy.json> --input <input.json>`.
4. Archive outputs using audit evidence templates.
