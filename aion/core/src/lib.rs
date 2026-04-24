//! SealRun Core — Datenmodelle, Verträge, Fehlerdefinitionen (Plan §2.2).
//!
//! | Modul | Beschreibung |
//! | :--- | :--- |
//! | `contracts` | Globale Konsistenzverträge, Finalitätsmodell. |
//! | `os_contract` | Spezifikations‑Hash und Versionierung des OS‑Vertrags. |
//! | `error` | Kanonische Fehler‑Namespaces (`AION_*`). |
//! | `identity` | Systemidentität und Kompatibilitätsmatrix. |
//! | `evidence` | Evidence‑Chain, Proof‑Hashing, lineare Verkettung. |
//! | `capsule` | Generisches Capsule‑Container‑Format. |
//! | `determinism_contract` | Determinismus‑Vertrag. |
//! | `determinism_matrix` | Matrix, Invarianten, Replay‑Gates. |

#![forbid(unsafe_code)]

pub mod capsule;
pub mod contracts;
pub mod determinism_contract;
pub mod determinism_matrix;
pub mod error;
pub mod evidence;
pub mod identity;
pub mod os_contract;
