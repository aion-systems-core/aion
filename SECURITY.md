# Security policy

## Supported versions

Security updates are applied to the **latest minor release** on the default
branch (`main`) and, when applicable, backported to the previous release line.
Use tagged releases for production evaluation.

## Reporting a vulnerability

**Do not** open a public GitHub issue for security-sensitive reports.

Instead, email **security@sealrun.dev** with:

- A short description of the issue and its impact
- Steps to reproduce (commands, versions, minimal example if possible)
- Whether you believe the issue is exploitable in default configurations

We will acknowledge receipt within **5 business days** (goal: 72 hours for
critical issues). We aim to provide an initial assessment within **10 business
days** and coordinate disclosure once a fix is ready.

## What to expect

- We treat reports as confidential unless you agree otherwise.
- We may ask follow-up questions; please use the same thread.
- Credit in release notes is offered if you want it (handle or name as you
  prefer).

## Out of scope (examples)

- Social engineering against maintainers or users
- Issues in third-party dependencies without a practical impact on SealRun
  defaults (we still forward dependency advisories when relevant)

For general bugs and feature requests, use the issue templates in
`.github/ISSUE_TEMPLATE/`.
