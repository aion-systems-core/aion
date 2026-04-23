# Roadmap und Meilensteine (Plan §8)

---

AION guarantees deterministic execution, replay symmetry, drift detection and audit‑grade evidence chains.  
AION intentionally does not enforce filesystem or network isolation.  
The kernel isolation modules are contract surfaces only; they define the interface but do not restrict access.

This is a deliberate design choice: AION is an Execution‑OS, not a Security‑Sandbox‑OS.  
Because AION does not modify kernel privileges or intercept syscalls, it is safe to adopt in existing environments without admin rights, without risk to workloads, and without operational friction.

If isolation is required (e.g., for regulated industries), the same contract surfaces can be backed by seccomp/landlock/micro‑VM isolation in a future "AION Secure Runtime" module — without breaking compatibility.

---

## 8.1 Kurzfristig (0–3 Monate)

- [ ] Open‑Source‑Launch (HN, Reddit)
- [ ] Feedback aus der Community sammeln und priorisieren
- [ ] Bugfixes und Stabilitätsverbesserungen
- [ ] Erste Version der Enterprise‑Website mit Preisen
- [ ] Beta‑Programm mit 3–5 Unternehmen starten

## 8.2 Mittelfristig (3–9 Monate)

- [ ] **Secure Runtime** implementieren (FS‑/Netz‑Isolation via Landlock/seccomp)
- [ ] **Governance Pack** fertigstellen (Compliance‑Exports, Policy‑Editor)
- [ ] **CI/CD Pack** mit GitHub Action und GitLab CI‑Komponente
- [ ] Erste zahlende Kunden onboarden
- [ ] Sprecher‑Slot auf einer Konferenz (RustConf, KubeCon)

## 8.3 Langfristig (9–18 Monate)

- [ ] **Enterprise Dashboard** (Web‑UI) als optionales Add‑on
- [ ] Integration mit **SPIFFE/SPIRE** für Workload‑Identität
- [ ] **Micro‑VM‑Mode** (Firecracker, Cloud Hypervisor) für höchste Isolation
- [ ] **SOC2 / ISO27001** Zertifizierung des Unternehmens (für Enterprise‑Sales)
- [ ] Ausbau des Teams (1–2 Entwickler, 1 Sales/Support)
