# Pricing

---

AION guarantees deterministic execution, replay symmetry, drift detection and audit‑grade evidence chains.  
AION intentionally does not enforce filesystem or network isolation.  
The kernel isolation modules are contract surfaces only; they define the interface but do not restrict access.

This is a deliberate design choice: AION is an Execution‑OS, not a Security‑Sandbox‑OS.  
Because AION does not modify kernel privileges or intercept syscalls, it is safe to adopt in existing environments without admin rights, without risk to workloads, and without operational friction.

If isolation is required (e.g., for regulated industries), the same contract surfaces can be backed by seccomp/landlock/micro‑VM isolation in a future "AION Secure Runtime" module — without breaking compatibility.

---

## Produktübersicht (Plan §6.1)

| Produkt | Enthaltene Funktionen | Lizenz | Preis (jährlich) |
| :--- | :--- | :--- | ---: |
| **AION Community** | Kernel, Engine, CLI, Capsules, Replay, Drift, Why/Graph, Evidence‑Chain, Doctor | MIT/Apache 2.0 | **kostenlos** |
| **AION Secure Runtime** | FS‑Isolation, Netzwerk‑Isolation, seccomp/landlock, Micro‑VM‑Mode, Policy‑Enforcement | AION‑ELv1 | **4.900 €** / Jahr |
| **AION Governance Pack** | Compliance‑Exports, Evidence‑Exports, Policy‑Pack‑Editor, Governance‑Snapshots, Determinism‑Proof‑Bundles | AION‑ELv1 | **2.900 €** / Jahr |
| **AION CI/CD Pack** | GitHub/GitLab‑Integration, Drift‑Gates, Replay‑Gates, Release‑Admission, Baseline‑Recorder | AION‑ELv1 | **3.900 €** / Jahr |
| **AION Enterprise Suite** | Secure Runtime + Governance Pack + CI/CD Pack + Evidence Export + SLA | AION‑ELv1 | **9.900 €** / Jahr |
| **AION Enterprise SLA** | 24h‑Response, Bugfix‑Priorisierung, Integration‑Support | Zusatz zur Suite | **+ 2.500 €** / Jahr |

## Early Access (first 30 days)

- Secure Runtime: 3.900 €/year
- Governance Pack: 1.900 €/year
- CI/CD Pack: 2.900 €/year
- Enterprise Suite: 7.900 €/year
- SLA: +1.900 €/year

## Standard pricing (after 30 days)

- Secure Runtime: 4.900 €/year
- Governance Pack: 2.900 €/year
- CI/CD Pack: 3.900 €/year
- Enterprise Suite: 9.900 €/year
- SLA: +2.500 €/year

## Volumen‑ und Startup‑Rabatte (Plan §6.3)

- **Startups (< 20 Mitarbeiter, < 1 Mio. € Funding):** 50 % Rabatt im ersten Jahr.
- **Non‑Profit / Universitäten:** Kostenlose Enterprise‑Lizenz für nicht‑kommerzielle Forschung.
- **Volumen‑Rabatt:** Ab 50 Lizenzen 20 % Rabatt, ab 100 Lizenzen individuelle Preisgestaltung.

## Zahlungsabwicklung (Plan §6.4)

- **Stripe** für Kreditkartenzahlungen (automatisierte Rechnungsstellung).
- Für größere Kunden: **Angebot / Rechnung** mit Zahlungsziel 30 Tage.
- **Kein** aufwändiges Sales‑Team nötig; der Kaufprozess ist self‑service über die Website.

## Website copy (Plan §11.4)

```markdown
# Pricing

## Community – Free forever
For individual developers, researchers, and open‑source projects.

- ✅ Deterministic execution engine
- ✅ Replay & drift detection
- ✅ Evidence chain
- ✅ Capsule export (.aionai)
- ✅ CLI & SDK
- ❌ Filesystem/network isolation
- ❌ Compliance exports
- ❌ Priority support

[Get started](#) – `cargo install aion-cli`

---

## Secure Runtime – €4,900 / year
For teams that need strict isolation.

Everything in Community, plus:
- ✅ Filesystem isolation (Landlock/seccomp)
- ✅ Network egress control
- ✅ Micro‑VM mode (optional)
- ✅ Policy enforcement engine

[Buy now](#) | [Book a demo](#)

---

## Enterprise Suite – €9,900 / year
For regulated industries and large organizations.

Everything in Secure Runtime, plus:
- ✅ Governance Pack (compliance exports)
- ✅ CI/CD Pack (GitHub/GitLab gates)
- ✅ Evidence Export (long‑term storage)
- ✅ SLA with 24h response

[Contact sales](#)
```
