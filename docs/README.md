# SealRun documentation index

## Overview

This index lists **canonical** Markdown hubs. Prefer linking here and to the [Trust Center](trust-center.md) rather than maintaining parallel copies under `aion/docs/` (those mirrors are **pointers** to root `docs/` where applicable).

## Architecture and product

| Topic | Document |
|--------|----------|
| Overview | [overview.md](overview.md) |
| Architecture | [architecture.md](architecture.md) |
| OS contract | [os_contract_spec.md](os_contract_spec.md) |
| Compatibility | [compatibility-matrix.md](compatibility-matrix.md) |
| CLI | [cli-reference.md](cli-reference.md) |

## Core execution concepts

| Topic | Document |
|--------|----------|
| Capsules | [capsules.md](capsules.md) |
| Replay | [replay.md](replay.md) |
| Drift | [drift.md](drift.md) |
| Evidence | [evidence/evidence_model.md](evidence/evidence_model.md) |
| Governance (product) | [governance.md](governance.md) |
| Policy engine (enterprise) | [policy-engine.md](policy-engine.md) |

## Enterprise and trust

| Topic | Document |
|--------|----------|
| Trust Center hub | [trust-center.md](trust-center.md) |
| Security guide | [security-guide.md](security-guide.md) |
| Security whitepaper | [security-whitepaper.md](security-whitepaper.md) |
| Operations guide | [operations-guide.md](operations-guide.md) |
| Multi-tenancy | [multi-tenancy.md](multi-tenancy.md) |
| RBAC | [rbac.md](rbac.md) |
| OIDC | [oidc-auth.md](oidc-auth.md) |
| SIEM and OTel | [siem-otel.md](siem-otel.md) |
| Release attestation | [release-attestation.md](release-attestation.md) |
| Buyer guide | [buyer-guide.md](buyer-guide.md) |
| Public roadmap | [roadmap-public.md](roadmap-public.md) |

## Pilot and procurement

| Topic | Document |
|--------|----------|
| Pilot Readiness Pack index | [enterprise/pilot/procurement-mini-pack.md](enterprise/pilot/procurement-mini-pack.md) |
| Success criteria | [enterprise/pilot/success-criteria.md](enterprise/pilot/success-criteria.md) |
| Hands-on pilot steps | [enterprise/pilot/00_install.md](enterprise/pilot/00_install.md) (sequence `00`–`06`) |
| Onboarding email (template) | [enterprise/pilot/ONBOARDING_EMAIL.md](enterprise/pilot/ONBOARDING_EMAIL.md) |
| Pilot one pager | [enterprise/pilot/PILOT_ONE_PAGER.md](enterprise/pilot/PILOT_ONE_PAGER.md) |
| Evaluation report template | [enterprise/pilot/EVAL_REPORT_TEMPLATE.md](enterprise/pilot/EVAL_REPORT_TEMPLATE.md) |
| Release zip / CST checklist | [enterprise/pilot/zip-new-cst.md](enterprise/pilot/zip-new-cst.md) |
| DSj12 ship checklist | [enterprise/pilot/DSj12-checklist.md](enterprise/pilot/DSj12-checklist.md) |
| Deploy entry (`README_DEPLOY`) | [../README_DEPLOY.md](../README_DEPLOY.md) |

## Compliance, policies, runbooks

| Topic | Path |
|--------|------|
| Controls matrix | [enterprise/compliance/controls-matrix.md](enterprise/compliance/controls-matrix.md) |
| ISO Annex A mapping | [enterprise/compliance/iso27001-annex-a-mapping.md](enterprise/compliance/iso27001-annex-a-mapping.md) |
| Compliance one-pager | [enterprise/compliance/sealrun_compliance_onepager.md](enterprise/compliance/sealrun_compliance_onepager.md) |
| Policies | [policies/](policies/) |
| Runbooks | [enterprise/runbooks/](enterprise/runbooks/) |
| Audit templates | [enterprise/templates/](enterprise/templates/) |
| Governance bundles | [enterprise/governance/bundles/](enterprise/governance/bundles/) |

## Integrations

| Guide | Document |
|--------|----------|
| Hugging Face | [integrations/huggingface-adapter-guide.md](integrations/huggingface-adapter-guide.md) |
| LangChain | [integrations/langchain-adapter-guide.md](integrations/langchain-adapter-guide.md) |
| Modal | [integrations/modal-adapter-guide.md](integrations/modal-adapter-guide.md) |
| BentoML | [integrations/bentoml-adapter-guide.md](integrations/bentoml-adapter-guide.md) |

## SealRun-AI

| Topic | Document |
|--------|----------|
| AI docs hub | [ai/README.md](ai/README.md) |
| Generator | [ai/generator.md](ai/generator.md) |
| Runner | [ai/runner.md](ai/runner.md) |
| Evaluator | [ai/evaluator.md](ai/evaluator.md) |
| Fixtures | [ai/fixtures.md](ai/fixtures.md) |
| CI | [ai/ci.md](ai/ci.md) |

## Terminology (use consistently)

**Capsule**, **evidence**, **replay**, **drift**, **governance decision**, **policy evaluation**, **tenancy** / **tenant isolation**, **RBAC**, **OIDC**, **SIEM**, **OpenTelemetry (OTel)**, **attestation**, **SBOM**, **Cosign**, **Sigstore**.

## Next steps

For documentation governance: edit **one** canonical file per topic; update this index when adding new top-level hubs.
