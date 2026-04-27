# Pilot onboarding email (template)

## Overview

Use this template when inviting a pilot tenant to the SealRun evaluation. Replace bracketed placeholders before sending.

## Subject line (suggested)

`[Pilot] SealRun pilot kickoff — next steps and access`

## Body

Hello [Name],

Thank you for joining the SealRun pilot. This email confirms scope, timeline, and how we will measure success.

**Pilot window:** [start date] → [end date]  
**Primary use case:** [execute / replay / drift / policy / evidence export — pick one lead]  
**Technical owner (customer):** [name, role]  
**Technical owner (vendor):** [name, role]

**What you will receive**

- Access to the agreed build or release tag: `[version or commit]`  
- A short onboarding session (45–60 minutes) covering install, first capsule, replay, drift, and evidence export  
- A shared channel for questions during the pilot: `[Slack / Teams / email]`

**What we need from you**

- A non-production environment that matches your target constraints (OS, network egress, identity provider if applicable)  
- One designated approver for scope changes and break-glass decisions  
- Agreement to share anonymized metrics for the evaluation report (no customer secrets)

**Success criteria (pre-agreed)**

See [Success criteria](success-criteria.md) and complete the [Pilot evaluation report](EVAL_REPORT_TEMPLATE.md) at pilot end.

**Next steps**

1. Confirm the pilot window and technical owners (reply to this thread).  
2. Complete [00_install.md](00_install.md) through [06_evidence_chain.md](06_evidence_chain.md) with your team.  
3. Schedule the kickoff call: `[calendar link]`

Regards,  
[Your name]

## Compliance notes

Do not include secrets, credentials, or production data in email. Use your corporate approved channel for attachments.

## Next steps

Link this template from your procurement pack and keep a single canonical copy here under `docs/enterprise/pilot/`.
