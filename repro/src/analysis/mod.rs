// Internal self-reflection.
//
// This module is not part of the user-facing surface. It powers the
// `repro eval` command by producing two artifacts:
//
//   * `product_analyzer`  — an honest inventory of what the current
//                           implementation is *missing*, categorized by
//                           severity. Curated rather than derived; the
//                           author of each new feature is expected to
//                           remove or demote entries here.
//
//   * `system_evaluation` — a full structured introspection of repro
//                           itself, emitted as markdown *and* JSON.
//                           It reads the source tree via `include_str!`
//                           and pulls out symbol counts deterministically,
//                           so the report moves only when the code moves.

pub mod product_analyzer;
pub mod system_evaluation;
