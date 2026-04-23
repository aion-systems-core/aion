//! AION Engine — Orchestrierung, KI‑Logik, Ausgabe, SDK (Plan §2.4).
//!
//! | Modul | Beschreibung |
//! | :--- | :--- |
//! | `ai` | KI‑Capsule‑Modell (`AICapsuleV1`), Prompt‑Verarbeitung, Token‑Tracing. |
//! | `replay` | Symmetrie‑Prüfung, Cross‑Machine‑Replay, Mismatch‑Taxonomie. |
//! | `drift` | Divergenz‑Evaluator zwischen zwei Capsules. |
//! | `why` | Erklärungs‑Engine (Rationale Slices). |
//! | `graph` | Abhängigkeitsgraphen für Ausführungsschritte. |
//! | `governance` | Policy‑Validierung, CI‑Gates, Audit‑Bündel. |
//! | `ci` | Baseline‑Aufzeichnung und deterministische Gate‑Checks. |
//! | `output` | HTML‑/SVG‑Rendering, Layout‑Determinismus, Bundle‑Writer. |
//! | `sdk` | Rust‑SDK, C‑FFI, Bindings (Python, Go, Node, etc.). |

#![forbid(unsafe_code)]

pub mod ai;
pub mod capture;
pub mod ci;
pub mod drift;
pub mod governance;
pub mod graph;
pub mod output;
pub mod replay;
pub mod sdk;
pub mod why;
