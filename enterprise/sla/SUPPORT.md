# SLA (Plan §6.1 / §9.2)

---

SealRun guarantees deterministic execution, replay symmetry, drift detection and audit‑grade evidence chains.  
SealRun intentionally does not enforce filesystem or network isolation.  
The kernel isolation modules are contract surfaces only; they define the interface but do not restrict access.

This is a deliberate design choice: SealRun is an Execution‑OS, not a Security‑Sandbox‑OS.  
Because SealRun does not modify kernel privileges or intercept syscalls, it is safe to adopt in existing environments without admin rights, without risk to workloads, and without operational friction.

If isolation is required (e.g., for regulated industries), the same contract surfaces can be backed by seccomp/landlock/micro‑VM isolation in a future "SealRun Secure Runtime" module — without breaking compatibility.

---

## SealRun Enterprise SLA (Produktlinie)

| Produkt | Enthaltene Funktionen | Lizenz | Preis (jährlich) |
| :--- | :--- | :--- | ---: |
| **SealRun Enterprise SLA** | 24h‑Response, Bugfix‑Priorisierung, Integration‑Support | Zusatz zur Suite | **+ 2.500 €** / Jahr |

## Enterprise‑Vertrag (Plan §9.2)

Für zahlende Kunden wird ein **Software‑Lizenz‑ und Support‑Vertrag** geschlossen. Dieser regelt:

- **Lizenzumfang:** Welche Komponenten genutzt werden dürfen.
- **Nutzungsbeschränkungen:** Kein SaaS, keine Weiterlizenzierung.
- **Gewährleistung:** Zusicherung, dass die Software im Wesentlichen der Dokumentation entspricht.
- **Haftungsbeschränkung:** Haftung ist auf den Lizenzpreis begrenzt (marktüblich).
- **Support‑SLA:** Reaktionszeiten, Bugfix‑Priorisierung.
