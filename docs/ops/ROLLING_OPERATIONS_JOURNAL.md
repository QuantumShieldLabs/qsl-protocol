Goals: G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-07

# Rolling Operations Journal

# Rolling Operations Journal Entry

- Directive: `DIRECTIVE 267 — NA-0227 qsc TUI State / Poll-Loop Mediation Decomposition`
- Begin timestamp (America/Chicago): 2026-04-07T20:14:18-05:00
- Begin timestamp (UTC): 2026-04-08T01:14:18Z
- End timestamp (America/Chicago): in progress
- End timestamp (UTC): in progress

## Repo SHAs
- qsl-protocol branch: `na-0227-tui-state-poll-loop-decomposition`
- qsl-protocol HEAD: `0485d9c19571`
- qsl-protocol main: `0485d9c19571`
- qsl-protocol origin/main: `0485d9c19571`
- qsl-protocol mirror/main: `0485d9c19571`
- qsl-server main: `0826ffa4d6f3`
- qsl-server origin/main: `0826ffa4d6f3`
- qsl-server mirror/main: `0826ffa4d6f3`
- qsl-attachments main: `e94107ac094d`
- qsl-attachments origin/main: `e94107ac094d`
- qsl-attachments mirror/main: `e94107ac094d`

## READY proof
- READY_COUNT: `1`
- Sole READY item: `NA-0227 — qsc TUI State / Poll-Loop Mediation Decomposition`
- Proof source: refreshed `NEXT_ACTIONS.md` on `main`

## Worktree / branch / PR
- Worktree path: `/srv/qbuild/work/NA-0227/qsl-protocol`
- Branch: `na-0227-tui-state-poll-loop-decomposition`
- PR: `n/a`
- Merge commit: `n/a`

## Failures / recoveries
- `cargo fmt --check` -> recoverable because the in-scope extraction temporarily left a missing `}` in `qsl/qsl-client/qsc/src/tui/controller/state/ownership.rs`; corrected by restoring the delimiter and rerunning formatting; final result: green on rerun.
- `cargo build` -> recoverable because moving `TuiState` methods into nested `state/*` modules narrowed method visibility from the original `controller`-visible seam and left one parent-module helper call unresolved; corrected by widening extracted method visibility to `pub(in super::super)` and routing the file-snapshot helper through `super::load_tui_files_snapshot()`; final result: green on rerun.

## Validation / CI notes
- Local validation: green for local goal-lint via synthesized event payload, `cargo fmt --check`, `cargo build`, `cargo clippy -- -D warnings`, `cargo test --test tui_charter`, `cargo test --test tui_product_polish_na0214a`, `cargo test --test tui_fixed_polling`, `cargo test --test tui_relay_drop_reorder`, `cargo test --test tui_contract_na0217j`, `cargo test --test tui_command_catalog_invariants`, `cargo test --test aws_tui_handshake_na0191`, `cargo test --test output_marker_contract_na0217a`, `cargo test --test desktop_gui_contract_na0215b`, the docs inventory commands, the manual markdown link-integrity runbook, and the staged added-line leak-safe scan.
- Protected checks: pending PR creation.
- Retry notes: one bounded formatting recovery, one bounded build recovery, and one bounded leak-scan pattern refinement to exclude a false positive on a route-token normalizer code line.

## Disk watermark
- Filesystem: `/srv/qbuild`
- Total GiB: `484`
- Used GiB: `183`
- Free GiB: `302`
- Used %: `38%`

## Next-watch items
- Push immediately after the first full green local validation bundle so the continuity state does not remain only on qbuild.
- Create exactly one PR, watch only the required protected contexts via bounded REST polling, and merge only with a merge commit once all required checks are green.

# Rolling Operations Journal Entry

- Directive: `DIRECTIVE 268 — NA-0227 Closeout / Evidence / Residual-TUI Successor Promotion`
- Begin timestamp (America/Chicago): 2026-04-07T21:29:34-05:00
- Begin timestamp (UTC): 2026-04-08T02:29:34Z
- End timestamp (America/Chicago): in progress
- End timestamp (UTC): in progress

## Repo SHAs
- qsl-protocol branch: `na-0227-closeout-tui-state`
- qsl-protocol HEAD: `6aa48816879e`
- qsl-protocol main: `6aa48816879e`
- qsl-protocol origin/main: `6aa48816879e`
- qsl-protocol mirror/main: `6aa48816879e`
- qsl-server main: `0826ffa4d6f3`
- qsl-server origin/main: `0826ffa4d6f3`
- qsl-server mirror/main: `0826ffa4d6f3`
- qsl-attachments main: `e94107ac094d`
- qsl-attachments origin/main: `e94107ac094d`
- qsl-attachments mirror/main: `e94107ac094d`

## READY proof
- READY_COUNT: `1`
- Sole READY item: `NA-0227 — qsc TUI State / Poll-Loop Mediation Decomposition`
- Proof source: refreshed `NEXT_ACTIONS.md` on `main`

## Worktree / branch / PR
- Worktree path: `/srv/qbuild/work/NA-0227/qsl-protocol`
- Branch: `na-0227-closeout-tui-state`
- PR: `pending creation`
- Merge commit: `n/a`

## What changed
- Prepared the governance-only closeout lane for `NA-0227` by correcting stale merged-main implementation metrics in `DECISIONS.md` and `TRACEABILITY.md`, archiving the implementation evidence, recording the queue transition, and promoting the next truthful successor.
- The successor choice remains bounded and evidence-driven: refreshed merged-main controller metrics show `qsl/qsl-client/qsc/src/tui/controller/commands.rs` is now the dominant residual concentration at `2,857 / 9,072` controller-local lines (`31.49%`), so `NA-0228 — qsc TUI Command Residual Shell / Dispatch Decomposition` is the sole truthful READY follow-on.

## Failures / recoveries
- `rg -n "closeout path: \`C|### NA-0226|### NA-0225|### NA-0224" NEXT_ACTIONS.md` -> recoverable because the shell pattern used mismatched quoting and failed before any repo mutation; corrected by rerunning the search with simpler quoted `rg` patterns; final result: confirmed the latest neighboring closeout token was `CY1`, so `CZ1` is truthful for `NA-0227`.

## Validation / CI notes
- Pre-mutation proof completed: disk watermark green, remotes refreshed sequentially from configured remotes only, `READY_COUNT=1` with `NA-0227` as the sole READY item, `qsl-server READY=0`, and `qsl-attachments READY=0`.
- Refreshed merged-main proof completed: PR #675 is already merged at `6aa48816879e`, and the implementation/evidence surfaces from that PR are durable on `main`.
- Docs-only validation and PR creation remain pending until the governance edits land.

## Disk watermark
- Filesystem: `/srv/qbuild`
- Total GiB: `484`
- Used GiB: `185`
- Free GiB: `299`
- Used %: `39%`

## Next-watch items
- Run the governance-only validation bundle: local goal-lint via synthesized event payload, markdown inventory counts, manual markdown link-integrity check, changed-path scope proof, and added-line leak-safe scan.
- Create exactly one PR, watch only the required protected contexts via bounded REST polling, and merge only with a merge commit once all required checks are green.
