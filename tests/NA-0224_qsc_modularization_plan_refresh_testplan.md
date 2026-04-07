Goals: G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-07

# NA-0224 — qsc Modularization / File-Size Reduction Plan Refresh Test Plan

## Purpose

This file exists to satisfy the docs/governance coupling rule for `NA-0224`.
The lane is implementation/evidence only: it refreshes the checked-in modularization plan against refreshed merged `main` without changing qsc runtime paths.

## Required proof points

1. Refreshed-main authority proof shows:
   - `qsl-protocol` `HEAD`, `mirror/main`, and `origin/main` match.
   - `qsl-server` remains `READY=0`.
   - `qsl-attachments` remains `READY=0`.
   - `NA-0224` is the sole live `READY` item in `NEXT_ACTIONS.md`.
2. Current qsc metrics are captured from merged main:
   - `qsl/qsl-client/qsc/src/main.rs` LOC
   - total `qsl/qsl-client/qsc/src/**` LOC
   - `main.rs` share of total
   - top-10 largest Rust source files
   - current subsystem ownership map for `output`, `fs_store`, `protocol_state`, `identity`, `contacts`, `timeline`, `transport`, `attachments`, `handshake`, `tui`, and the remaining `main.rs` ownership
3. `docs/design/DOC-QSC-011_qsc_Modularization_and_File_Size_Reduction_Plan_v0.1.0_DRAFT.md` is refreshed to:
   - record the current concentration metrics;
   - mark the pre-`NA-0217A` baseline as stale;
   - state whether another bounded extraction lane is still justified; and
   - if yes, name the next bounded lane with protected invariants, no-drift concerns, and representative regression surfaces.
4. `DECISIONS.md` and `TRACEABILITY.md` record `NA-0224` as implementation/evidence only, with no queue closeout or promotion in this PR.
5. Local docs/governance validation stays green:
   - local goal-lint via synthesized `GITHUB_EVENT_PATH`
   - manual markdown link-integrity runbook from `AGENTS.md`
   - markdown inventory counts
   - added-line leak-safe scan

## Non-goals

- No qsc runtime edits
- No qsc-desktop edits
- No qsl-server or qsl-attachments edits
- No `.github`, `Cargo.toml`, or `Cargo.lock` edits
- No `NEXT_ACTIONS.md` closeout or successor promotion
