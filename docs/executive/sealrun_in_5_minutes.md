# SealRun in 5 Minuten (Executive Summary)

## Purpose (EN)

Five-minute **executive** orientation: deterministic execution, contract controls, and why SealRun is an **Execution-OS** (contracts) rather than a sandbox OS—before deeper reading in [Architecture](../architecture.md).

SealRun ist eine deterministische Ausführungs-Engine mit Contract-Layer-Steuerfläche für KI-Ausführung.

## At a glance

- Deterministische Ausführung: gleiche Eingaben -> prüfbare Ergebnisse
- Contract-first Steuerung: Replay, Drift, Evidence, Governance, Measurement
- Enterprise-readiness über Phase-1-12 Vertragsmodell

---

SealRun guarantees deterministic execution, replay symmetry, drift detection and audit‑grade evidence chains.  
SealRun intentionally does not enforce filesystem or network isolation.  
The kernel isolation modules are contract surfaces only; they define the interface but do not restrict access.

This is a deliberate design choice: SealRun is a deterministic execution engine, not a Security-Sandbox-OS.  
Because SealRun does not modify kernel privileges or intercept syscalls, it is safe to adopt in existing environments without admin rights, without risk to workloads, and without operational friction.

If isolation is required (e.g., for regulated industries), the same contract surfaces can be backed by seccomp/landlock/micro‑VM isolation in a future "SealRun Secure Runtime" module — without breaking compatibility.

---

## Was ist SealRun?

SealRun ist eine **deterministische Ausführungsschicht für KI‑Runs**: Jeder Lauf wird als **Capsule** festgehalten — mit Token‑Spur, Evidenz‑Kette, Erklärbarkeit (Why), Kausalgraph und Governance‑Prüfungen.

## Warum deterministisch?

Weil **gleiche Eingaben (Modell, Prompt, Seed, eingefrorene Laufzeitparameter)** **überprüfbare** Ergebnisse liefern: Regressionstests, Abnahme in regulierten Umgebungen und **technische Nachvollziehbarkeit** werden möglich, statt „Black Box plus Logfile“.

## Warum auditierbar?

Weil Artefakte **strukturiert** (JSON/HTML/SVG) exportiert werden und sich an **Replay**, **Drift** und **Policy‑Validation** koppeln lassen — ein Audit kann auf **Maschinenprüfungen** statt auf Screenshots bauen.

## Warum evidence‑fähig?

Weil eine **lineare Evidenzkette** mit rollierenden Digest‑Schritten integriert ist: Integrität ist **kryptographisch gebunden**, nicht nur eine Zeile im Textlog.

## Warum modell‑agnostisch?

Weil der Vertrag auf **aufgezeichneten Runs** und **Backends** basiert, die das Determinismus‑Envelope respektieren — nicht auf einem einzelnen Closed‑Source‑SDK.

## Warum OS statt Framework?

Weil SealRun als **Schicht unterhalb der App** gedacht ist: **CLI**, **SDK**, Artefakt‑Pipelines, Governance und CI‑Anbindung — ein **Betriebssystem‑ähnliches** Fundament für verlässliche KI‑Ausführung, nicht nur eine Bibliotheksfunktion.

## CLI surface

```bash
sealrun doctor
sealrun governance status
sealrun reliability status
sealrun measure audits
```

## Enterprise-readiness

SealRun ist enterprise-ready, wenn deterministische Contract-Ausgaben release-übergreifend stabil, auditierbar und ohne Drift in der Betriebsrealität bleiben.

---

**Guided Link: SealRun in 5 Minuten** — Vertiefung: [Guided tour](../guided_tour.md) · [Compliance One-Pager](../compliance/sealrun_compliance_onepager.md) · [Evidence model](../evidence/evidence_model.md)
