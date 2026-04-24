//! SealRun Kernel — deterministische Systemaufrufe (Plan §2.3).
//!
//! | Modul | Beschreibung |
//! | :--- | :--- |
//! | `execute` | Subprocess‑Ausführung (`run_command`), erfasst stdout/stderr/exit/duration. |
//! | `envelope` | Erfassung des Ausführungskontexts (Zeit, Env, CWD, RNG). |
//! | `integrity` | Regel‑Evaluierung, Enforcement, Berichterstellung. |
//! | `env` | Whitelist‑Filterung (`PATH`, `SYSTEMROOT` etc.), `BTreeMap`‑Ordnung. |
//! | `time` | Einfrieren der aktuellen Zeit (`freeze_time_ms`). |
//! | `random` | Deterministischer RNG (`xorshift64*`) mit Seed‑Ableitung. |
//! | `fs` | **Stub** – Vertrag für Dateisystem‑Isolation, keine Durchsetzung. |
//! | `network` | **Stub** – Vertrag für Netzwerk‑Isolation, keine Durchsetzung. |

#![forbid(unsafe_code)]

pub mod envelope;
pub mod env;
pub mod execute;
pub mod fs;
pub mod integrity;
pub mod network;
pub mod random;
pub mod time;
