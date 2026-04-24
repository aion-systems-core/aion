# Marketing‑ und Launch‑Plan (Plan §7)

---

SealRun guarantees deterministic execution, replay symmetry, drift detection and audit‑grade evidence chains.  
SealRun intentionally does not enforce filesystem or network isolation.  
The kernel isolation modules are contract surfaces only; they define the interface but do not restrict access.

This is a deliberate design choice: SealRun is an Execution‑OS, not a Security‑Sandbox‑OS.  
Because SealRun does not modify kernel privileges or intercept syscalls, it is safe to adopt in existing environments without admin rights, without risk to workloads, and without operational friction.

If isolation is required (e.g., for regulated industries), the same contract surfaces can be backed by seccomp/landlock/micro‑VM isolation in a future "SealRun Secure Runtime" module — without breaking compatibility.

---

## 7.1 Phase 0: Vorbereitung (1–2 Wochen)

**Ziele:** Repository vorbereiten, Dokumentation schreiben, Website erstellen.

| Aufgabe | Verantwortlich | Status |
| :--- | :---: | :---: |
| Öffentliches GitHub‑Repo anlegen (MIT‑Lizenz) | Du | ⬜ |
| `README.md` nach Vorlage (Anhang 11.1) schreiben | Du | ⬜ |
| `os_contract_spec.md` als separates Dokument aufbereiten | Du | ⬜ |
| Einfache Landingpage (`aion.sh` oder `aion.dev`) mit Tailwind/Carrd | Du | ⬜ |
| Twitter/X‑Account `@aion_os` erstellen | Du | ⬜ |
| LinkedIn‑Profil optimieren | Du | ⬜ |

## 7.2 Phase 1: Hacker News Launch (Tag 1)

**Ziel:** Maximale Sichtbarkeit in der Entwickler‑Community, erste GitHub Stars, Feedback.

| Plattform | Aktion | Zeitpunkt |
| :--- | :--- | :--- |
| **Hacker News** | „Show HN: SealRun – Deterministic Execution OS & Evidence Fabric for AI Agents“ | Di–Do, 15–16 Uhr MEZ |
| **Reddit** | Post in `/r/rust` und `/r/programming` | 1–2 Stunden nach HN |
| **Twitter/X** | Ankündigung mit Link zum HN‑Post | Parallel |
| **LinkedIn** | Persönlicher Post zur Motivation | Abends |

**HN‑Post‑Text:** siehe `docs/launch/hacker_news_show_hn.md` (Plan §11.3).

## 7.3 Phase 2: Content‑Marketing (Woche 2–4)

**Ziel:** Langfristige Sichtbarkeit, SEO, Vertrauensaufbau.

| Inhalt | Plattform | Fokus |
| :--- | :--- | :--- |
| **Blogpost:** „Warum wir ein deterministisches OS für KI brauchen“ | DEV.to, Hashnode | Problem‑Bewusstsein |
| **Blogpost:** „SealRun unter der Haube: Wie wir Subprozesse deterministisch machen“ | Eigenes Blog | Technische Tiefe |
| **YouTube‑Video:** 5‑Minuten‑Demo von `aion execute` | YouTube | Visueller Beweis |
| **Case Study (fiktiv):** „Wie eine Bank SealRun für AI‑Governance nutzt“ | Website | Enterprise‑Relevanz |

## 7.4 Phase 3: Outbound (Monat 2–6)

**Ziel:** Erste zahlende Kunden gewinnen.

| Aktion | Zielgruppe | Vorgehen |
| :--- | :--- | :--- |
| **LinkedIn Outreach** | CTOs, CISOs von FinTechs | Persönliche Nachricht mit Link zur Doku |
| **Konferenz‑Einreichungen** | RustConf, KubeCon, AI‑Events | Talk über deterministische Ausführung |
| **Guest Posts** | The New Stack, InfoQ | Gastbeitrag zu AI‑Compliance |
| **Beta‑Programm** | 5–10 Unternehmen | Kostenlose Enterprise‑Lizenz im Austausch für Feedback |

## 7.5 Metriken und Erfolgskontrolle

| Metrik | Ziel (Monat 6) |
| :--- | ---: |
| GitHub Stars | > 500 |
| Website‑Besucher / Monat | > 2.000 |
| Newsletter‑Abonnenten | > 200 |
| Enterprise‑Anfragen | > 20 |
| Zahlende Kunden | > 3 |
