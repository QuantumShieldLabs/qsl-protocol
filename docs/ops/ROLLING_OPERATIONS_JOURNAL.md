Goals: G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-09

# Rolling Operations Journal

# Rolling Operations Journal Entry

- Directive: `DIRECTIVE 270 — NA-0228 Closeout / Evidence / Residual-TUI Successor Promotion`
- Begin timestamp (America/Chicago): 2026-04-08T20:03:17-05:00
- Begin timestamp (UTC): 2026-04-09T01:03:17Z
- End timestamp (America/Chicago): in progress
- End timestamp (UTC): in progress

## Repo SHAs
- qsl-protocol branch: `na-0228-closeout-tui-commands`
- qsl-protocol HEAD: `dca4cb7e127e`
- qsl-protocol main: `dca4cb7e127e`
- qsl-protocol origin/main: `dca4cb7e127e`
- qsl-protocol mirror/main: `dca4cb7e127e`
- qsl-server main: `0826ffa4d6f3`
- qsl-server origin/main: `0826ffa4d6f3`
- qsl-server mirror/main: `0826ffa4d6f3`
- qsl-attachments main: `e94107ac094d`
- qsl-attachments origin/main: `e94107ac094d`
- qsl-attachments mirror/main: `e94107ac094d`

## READY proof
- READY_COUNT: `1`
- Sole READY item: `NA-0228 — qsc TUI Command Residual Shell / Dispatch Decomposition`
- Proof source: refreshed `NEXT_ACTIONS.md` on `main`

## Worktree / branch / PR
- Worktree path: `/srv/qbuild/work/NA-0228/qsl-protocol`
- Branch: `na-0228-closeout-tui-commands`
- PR: `pending creation`
- Merge commit: `n/a`

## Failures / recoveries
- none so far

## Validation / CI notes
- What changed: this governance-only lane adds archive evidence for merged PR #677, closes `NA-0228`, updates `DECISIONS.md` and `TRACEABILITY.md`, writes the required journal/testplan companions, and promotes exactly one successor `READY` item.
- What worked: refreshed merged `main` already carries PR #677, `D-0392`, the `TRACEABILITY.md` `NA-0228 implementation/evidence` entry, and the extracted controller-local `commands/**` modules, so closeout truth is anchored on durable repo state rather than branch memory.
- Successor rationale: refreshed residual metrics now show `qsl/qsl-client/qsc/src/tui/controller/state.rs` is the next dominant remaining controller concentration at `2,336 / 9,033` (`25.86%`), ahead of `commands/contacts.rs` (`1,250 / 9,033`, `13.84%`), `state/ownership.rs` (`1,229 / 9,033`, `13.61%`), and `render.rs` (`1,044 / 9,033`, `11.56%`), so `NA-0229` is the next truthful successor.
- Local validation: pending docs-only goal-lint, markdown inventory, link-integrity, and added-line leak-safe scan on the final branch tree.
- Protected checks: pending PR creation.

## Disk watermark
- Filesystem: `/srv/qbuild`
- Total GiB: `484`
- Used GiB: `188`
- Free GiB: `297`
- Used %: `39%`

## Next-watch items
- Run the docs-only validation bundle on the final branch tree, then push and open exactly one governance-only PR once local proof is green.
- Watch only the required protected contexts via bounded REST polling, merge only with a merge commit, and then refresh `main` again to prove `NA-0228` is `DONE` and `NA-0229` is the sole `READY` item.

# Rolling Operations Journal Entry

- Directive: `DIRECTIVE 269 — NA-0228 qsc TUI Command Residual Shell / Dispatch Decomposition`
- Begin timestamp (America/Chicago): 2026-04-08T07:35:35-05:00
- Begin timestamp (UTC): 2026-04-08T12:35:35Z
- End timestamp (America/Chicago): in progress
- End timestamp (UTC): in progress

## Repo SHAs
- qsl-protocol branch: `na-0228-tui-command-shell-dispatch-decomposition`
- qsl-protocol HEAD: `574c38c1c64a`
- qsl-protocol main: `574c38c1c64a`
- qsl-protocol origin/main: `574c38c1c64a`
- qsl-protocol mirror/main: `574c38c1c64a`
- qsl-server main: `0826ffa4d6f3`
- qsl-server origin/main: `0826ffa4d6f3`
- qsl-server mirror/main: `0826ffa4d6f3`
- qsl-attachments main: `e94107ac094d`
- qsl-attachments origin/main: `e94107ac094d`
- qsl-attachments mirror/main: `e94107ac094d`

## READY proof
- READY_COUNT: `1`
- Sole READY item: `NA-0228 — qsc TUI Command Residual Shell / Dispatch Decomposition`
- Proof source: refreshed `NEXT_ACTIONS.md` on `main`

## Worktree / branch / PR
- Worktree path: `/srv/qbuild/work/NA-0228/qsl-protocol`
- Branch: `na-0228-tui-command-shell-dispatch-decomposition`
- PR: `pending creation`
- Merge commit: `n/a`

## Failures / recoveries
- `rustfmt qsl/qsl-client/qsc/src/tui/controller/commands.rs qsl/qsl-client/qsc/src/tui/controller/commands/key.rs qsl/qsl-client/qsc/src/tui/controller/commands/navigation.rs qsl/qsl-client/qsc/src/tui/controller/commands/dispatch.rs qsl/qsl-client/qsc/src/tui/controller/commands/contacts.rs qsl/qsl-client/qsc/src/tui/controller/commands/messages.rs qsl/qsl-client/qsc/tests/tui_command_catalog_invariants.rs` -> recoverable because file-scoped `rustfmt` defaulted to an older edition while traversing the `tests/common` module tree; corrected by rerunning `rustfmt --edition 2021` on the same file set; final result: green on rerun.
- `cargo test --test tui_command_catalog_invariants` -> recoverable because the bounded extraction initially hid two helper entrypoints (`wipe_account_local_state_best_effort`, `tui_receive_via_relay`) that sibling controller modules still imported through `commands.rs`; corrected by restoring thin wrapper entrypoints in `commands.rs` and rerunning the same test; final result: green on rerun.

## Validation / CI notes
- Local validation: direct canary `cargo test --test tui_command_catalog_invariants` is green after the bounded visibility fix; the full directive validation bundle remains pending on the final branch tree.
- Protected checks: pending PR creation.
- Retry notes: one bounded `rustfmt` rerun and one bounded local test fix/rerun on the same root cause.

## Disk watermark
- Filesystem: `/srv/qbuild`
- Total GiB: `484`
- Used GiB: `185`
- Free GiB: `299`
- Used %: `39%`

## Next-watch items
- Run the full local validation bundle on the final tree, then push immediately after the first fully green bundle so the implementation state is not left only on qbuild.
- Create exactly one PR, watch only the required protected contexts via bounded REST polling, and merge only with a merge commit once all required checks are green.

# Rolling Operations Journal Entry

- Directive: `DIRECTIVE 274 — NA-0230 Security Audit Packet Intake / Verification / Remediation Plan Canon`
- Begin timestamp (America/Chicago): 2026-04-09T00:26:31-05:00
- Begin timestamp (UTC): 2026-04-09T05:26:31Z
- End timestamp (America/Chicago): in progress
- End timestamp (UTC): in progress

## Repo SHAs
- qsl-protocol branch: `na-0230-security-audit-intake-plan`
- qsl-protocol HEAD: `89205567d129`
- qsl-protocol main: `89205567d129`
- qsl-protocol origin/main: `89205567d129`
- qsl-protocol mirror/main: `89205567d129`
- qsl-server main: `0826ffa4d6f3`
- qsl-server origin/main: `0826ffa4d6f3`
- qsl-server mirror/main: `0826ffa4d6f3`
- qsl-attachments main: `e94107ac094d`
- qsl-attachments origin/main: `e94107ac094d`
- qsl-attachments mirror/main: `e94107ac094d`

## READY proof
- READY_COUNT: `1`
- Sole READY item: `NA-0230 — Security Audit Packet Intake / Verification / Remediation Plan Canon`
- Proof source: refreshed `NEXT_ACTIONS.md` on `main`

## Worktree / branch / PR
- Worktree path: `/srv/qbuild/work/NA-0230/qsl-protocol`
- Branch: `na-0230-security-audit-intake-plan`
- PR: `pending creation`
- Merge commit: `n/a`

## What changed
- Read the full 8-file staged security-audit packet from `docs/audit/incoming/2026-04-09_security_batch/`, verified every finding against refreshed current-main truth where repo code/docs/tests permit, and normalized the packet into one canonical de-duplicated remediation program.
- Added `docs/audit/DOC-AUD-003_Security_Audit_Packet_Intake_and_Remediation_Plan_v0.1.0_DRAFT.md` as the canonical intake/remediation-plan artifact with an overlap map, a finding-by-finding current-main status matrix, and a single remediation ordering across Tier 0 through Tier 3.
- Updated `DECISIONS.md` and `TRACEABILITY.md` so the packet ingestion, focused-audit override rule, KT prerequisite-blocked status, and implementation/evidence-only posture are canonical in repo governance.
- Added the matching docs-only companion stub at `tests/NA-0230_security_audit_intake_and_remediation_plan_testplan.md`.

## Failures / recoveries
- `rg -c '^Status: READY' /srv/qbuild/work/NA-0230/qsl-server/NEXT_ACTIONS.md /srv/qbuild/work/NA-0230/qsl-attachments/NEXT_ACTIONS.md` -> recoverable because zero READY matches are a valid discovery outcome and `rg` exits non-zero for zero matches; corrected by rerunning the READY proof with a short Python counter over each `NEXT_ACTIONS.md`; final result: `qsl-server READY=0` and `qsl-attachments READY=0`.
- `printf '--- KT 1-140 ---\n'` -> recoverable because the format string started with `-` and triggered a shell command-shape error before any repo mutation; corrected by continuing the KT report read with safer `sed` chunking instead of that `printf` form; final result: the full KT focused audit was read and verified against refreshed current-main surfaces.

## Validation / CI notes
- Pre-mutation authority proof completed: disk watermark green, configured-remotes-only refresh completed for `qsl-protocol`, `qsl-server`, and `qsl-attachments`, `READY_COUNT=1` with `NA-0230` as the sole READY item, and qsl-server/qsl-attachments both remained `READY=0`.
- Current-main verification completed for the staged packet: Tier 0 remains four live items (`F01` through `F04`), KT is still prerequisite-blocked, transcript-binding and PQ-KEM findings are narrowed but not closed, and assurance-expansion harness work remains absent/incomplete.
- Local docs/governance validation, branch push, PR creation, protected-check polling, and merge remain pending at this entry.

## Disk watermark
- Filesystem: `/srv/qbuild`
- Total GiB: `484`
- Used GiB: `190`
- Free GiB: `294`
- Used %: `40%`

## Next-watch items
- Run the full docs/governance validation bundle on the final tree, then push immediately after the first green local bundle so the lane does not remain only on qbuild.
- Create exactly one PR, poll required protected contexts only via bounded REST checks, and merge only with a merge commit once the protected set is green.

# Rolling Operations Journal Entry

- Directive: `DIRECTIVE 273 — NA-0229 Closeout / Audit-Packet Staging / Security-Intake Successor Promotion`
- Begin timestamp (America/Chicago): 2026-04-08T23:20:32-05:00
- Begin timestamp (UTC): 2026-04-09T04:20:32Z
- End timestamp (America/Chicago): in progress
- End timestamp (UTC): in progress

## Repo SHAs
- qsl-protocol branch: `na-0229-closeout-tui-state`
- qsl-protocol HEAD: `c7e224a0f413`
- qsl-protocol main: `c7e224a0f413`
- qsl-protocol origin/main: `c7e224a0f413`
- qsl-protocol mirror/main: `c7e224a0f413`
- qsl-server main: `0826ffa4d6f3`
- qsl-server origin/main: `0826ffa4d6f3`
- qsl-server mirror/main: `0826ffa4d6f3`
- qsl-attachments main: `e94107ac094d`
- qsl-attachments origin/main: `e94107ac094d`
- qsl-attachments mirror/main: `e94107ac094d`

## READY proof
- READY_COUNT: `1`
- Sole READY item: `NA-0229 — qsc TUI State Residual Shell / Ownership Mediation Decomposition`
- Proof source: refreshed `NEXT_ACTIONS.md` on `main`

## Worktree / branch / PR
- Worktree path: `/srv/qbuild/work/NA-0229/qsl-protocol`
- Branch: `na-0229-closeout-tui-state`
- PR: `#680`
- Merge commit: `n/a`

## What changed
- Prepared the governance-only `NA-0229` closeout lane by archiving durable evidence for the already-merged PR #679 implementation/evidence state now present on refreshed `main`.
- Staged the externally provided 8-file security audit packet verbatim under `docs/audit/incoming/2026-04-09_security_batch/` so the next lane can ingest findings from repo truth instead of host-only storage.
- The successor choice remains bounded and evidence-driven: with the packet now staged in repo truth, `NA-0230 — Security Audit Packet Intake / Verification / Remediation Plan Canon` is the sole truthful READY follow-on.

## Failures / recoveries
- `gh run view 24171422368 --json status,conclusion,jobs,workflowName,url`, `gh run view 24171422394 --json status,conclusion,jobs,workflowName,url`, and `gh run view 24171422409 --json status,conclusion,jobs,workflowName,url` -> recoverable because the run IDs were guessed instead of being read from the live PR metadata; corrected by querying `gh pr view 680 --json statusCheckRollup` and using the current details URLs to identify the actual long-running contexts; final result: remaining protected checks were truthfully identified without changing scope.

## Validation / CI notes
- Pre-mutation proof completed: disk watermark green, remotes refreshed sequentially from configured remotes only, `READY_COUNT=1` with `NA-0229` as the sole READY item, `qsl-server READY=0`, and `qsl-attachments READY=0`.
- Refreshed merged-main proof completed: PR #679 is already merged at `c7e224a0f413`, and the implementation/evidence surfaces from that PR are durable on `main`.
- Local validation: green for markdown inventory counts, the manual markdown link-integrity runbook, the staged added-line leak-safe scan, host-side versus repo-copy SHA-256 equality proof for the 8-file audit packet, and local goal-lint via a synthesized event payload.
- Host-side audit-packet proof completed: all 8 required files exist at `/srv/qbuild/docs/audit/incoming/2026-04-09_security_batch/` with recorded size and SHA-256 evidence.
- Protected checks: PR `#680` is open and the required contexts are attached and in progress with no failures at the time of this update.

## Disk watermark
- Filesystem: `/srv/qbuild`
- Total GiB: `484`
- Used GiB: `190`
- Free GiB: `294`
- Used %: `40%`

## Next-watch items
- Run the governance-only validation bundle: local goal-lint via synthesized event payload, markdown inventory counts, manual markdown link-integrity check, added-line leak-safe scan, and host-versus-repo SHA-256 equality proof for the staged packet.
- Create exactly one PR, watch only the required protected contexts via bounded REST polling, and merge only with a merge commit once all required checks are green.

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

# Rolling Operations Journal Entry

- Directive: `DIRECTIVE 271 — NA-0229 qsc TUI State Residual Shell / Ownership Mediation Decomposition`
- Begin timestamp (America/Chicago): 2026-04-08T20:59:53-05:00
- Begin timestamp (UTC): 2026-04-09T01:59:53Z
- End timestamp (America/Chicago): in progress
- End timestamp (UTC): in progress

## Repo SHAs
- qsl-protocol branch: `na-0229-tui-state-ownership-decomposition`
- qsl-protocol HEAD: `0a20be8749ca`
- qsl-protocol main: `0a20be8749ca`
- qsl-protocol origin/main: `0a20be8749ca`
- qsl-protocol mirror/main: `0a20be8749ca`
- qsl-server main: `0826ffa4d6f3`
- qsl-server origin/main: `0826ffa4d6f3`
- qsl-server mirror/main: `0826ffa4d6f3`
- qsl-attachments main: `e94107ac094d`
- qsl-attachments origin/main: `e94107ac094d`
- qsl-attachments mirror/main: `e94107ac094d`

## READY proof
- READY_COUNT: `1`
- Sole READY item: `NA-0229 — qsc TUI State Residual Shell / Ownership Mediation Decomposition`
- Proof source: refreshed `NEXT_ACTIONS.md` on `main`

## Worktree / branch / PR
- Worktree path: `/srv/qbuild/work/NA-0229/qsl-protocol`
- Branch: `na-0229-tui-state-ownership-decomposition`
- PR: `pending creation`
- Merge commit: `n/a`

## What changed
- Prepared the bounded `NA-0229` implementation/evidence lane by keeping `qsl/qsl-client/qsc/src/tui/controller/state.rs` as the retained shell and moving residual account/contact/timeline/file-state mediation into the new controller-local child module `qsl/qsl-client/qsc/src/tui/controller/state/account.rs`.
- Refreshed controller metrics now show `state.rs` reduced from `2,336 / 9,033` controller-local lines (`25.86%`) to `1,756 / 9,046` (`19.41%`), while `state/account.rs` now carries `593 / 9,046` (`6.56%`) alongside the existing `state/ownership.rs` and `state/poll.rs` seams.
- Updated `qsl/qsl-client/qsc/tests/tui_command_catalog_invariants.rs` so the source-inventory guard truthfully includes the new `state/account.rs` file.

## Failures / recoveries
- `cargo test --test tui_command_catalog_invariants` -> recoverable because moving `contact_record_cached` into `state/account.rs` initially narrowed visibility too far for the existing `qsl/qsl-client/qsc/src/contacts/mod.rs` caller; corrected by restoring `contact_record_cached` to `pub(crate)` inside the new child module and rerunning the same canary; final result: green on rerun.

## Validation / CI notes
- Local validation: direct canary `cargo test --test tui_command_catalog_invariants` is green after the bounded visibility fix; the full directive validation bundle remains pending on the final tree.
- Protected checks: pending PR creation.
- Retry notes: one bounded local test/build visibility recovery on the same root cause.

## Disk watermark
- Filesystem: `/srv/qbuild`
- Total GiB: `484`
- Used GiB: `188`
- Free GiB: `297`
- Used %: `39%`

## Next-watch items
- Run the full local validation bundle on the final tree, then push immediately after the first fully green bundle so the implementation state is not left only on qbuild.
- Create exactly one PR, watch only the required protected contexts via bounded REST polling, and merge only with a merge commit once all required checks are green.
