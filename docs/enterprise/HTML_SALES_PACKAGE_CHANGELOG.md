# Enterprise sales package — packaging changelog

## Purpose

Tracks packaging and copy changes for the HTML sales bundle when maintained alongside Markdown sources.

Canonical narrative + line references: [`SealRun_Enterprise_Sales_Package.md`](SealRun_Enterprise_Sales_Package.md).

Current packaging state:

- **HTML package:** not present in this repository.
- **Markdown specs stubs:** removed; use `docs/os_contract_spec.md`, `docs/architecture.md`, and `docs/compliance/sealrun_compliance_onepager.md` as canonical technical references.

## 2026-04-22 — Deterministic enterprise sync pass

- **Change:** Rebuilt the HTML edition to match the current phase 1-12 contract narrative.
- **Structure:** Added normalized sections: At a glance, Contract surface, CLI surface, 12-phase story, governance/compliance, replay/drift/evidence, enterprise-readiness.
- **Terminology:** Standardized deterministic execution engine and contract layer wording; aligned 7-domain CLI surface names.
- **Links:** Pointed HTML consumers to canonical markdown package, OS contract spec, architecture, and CLI reference.

## 2026-04-21 — Part 6: 8-Step Pilot Framework (repackaging)

- **Change:** Part 6 is framed as an **8-Step Pilot Framework** (validation gates: Build & Doctor → … → Policy Integration), not a literal **eight-week** calendar.
- **Copy:** Added **operating-model** language (steps may run **in any order** or **in parallel**; multiple steps often complete in the **same week**) plus the **German customer-facing** paragraph from sales enablement.
- **HTML:** Executive “next steps” and 1-pager checklist updated to **Step 1–8** labels; sidebar label **Pilot Framework**.

## 2026-04-21 — Accessibility, JS correctness, copy precision

1. **Tabs and top bar** — Replaced `<div onclick>` tabs with `<button type="button">`, `role="tablist"` / `role="tab"`, `aria-selected`, `aria-controls`, and `id` hooks. Removes reliance on the non-standard global `event` object (which broke tab activation in strict environments).
2. **Sidebar** — Replaced `<div onclick>` nav links with `<button class="sidebar-link">` and `data-scroll-target`; scroll + active state no longer depend on `event.target`.
3. **Script** — Wrapped in an IIFE; `switchTab(view, tabEl)` and `scrollToSection(id, linkEl)` take explicit elements; scroll uses `prefers-reduced-motion` for `behavior: 'auto'` vs `smooth`.
4. **Scroll spy** — Uses `getBoundingClientRect()` + `data-scroll-target` instead of parsing the `onclick` attribute string.
5. **Skip link** — Visible on keyboard focus (`Skip to content` → `#s-hero`).
6. **Focus styles** — `:focus-visible` outlines for tabs and sidebar buttons.
7. **Responsive layout** — `@media` stacks `vp-triad`, `exec-next`, `exec-two-col`, `op-grid`, and `comp-gate` on narrow viewports; `exec-kpi-row` drops to two columns.
8. **Executive summary** — Clarified that **TLA+ / TLC** are optional and **not** in default GitHub Actions CI; distinguished **`assert_formal_replay_invariant`** coverage via `engine/tests/formal_replay.rs` from the abstract TLA+ module.
9. **Hero chip** — Renamed / titled to reduce confusion: “TLA+ (abstract) + Rust tests” with a short tooltip on the distinction vs CI.
10. **Canonical links** — Footer points to the Markdown source and this changelog (paths differ for root vs `docs/enterprise/` copy).

When editing content, prefer updating **Markdown first**, then syncing the HTML body if you maintain both formats.
