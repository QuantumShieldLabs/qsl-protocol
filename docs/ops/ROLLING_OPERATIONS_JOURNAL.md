Goals: G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-12

# Rolling Operations Journal

# Rolling Operations Journal Entry

- Directive: `DIRECTIVE 287 — NA-0233A Closeout / Queue Truth Repair / NA-0233 Restore-to-READY`
- Begin timestamp (America/Chicago): 2026-04-12T17:50:29-05:00
- Begin timestamp (UTC): 2026-04-12T22:50:29Z
- End timestamp (America/Chicago): in progress
- End timestamp (UTC): in progress

## Repo SHAs
- qsl-protocol branch: `na-0233a-closeout-ci-rebalance`
- qsl-protocol HEAD: `pending governance closeout commit`
- qsl-protocol main: `96e02a79db5e`
- qsl-protocol origin/main: `96e02a79db5e`
- qsl-protocol mirror/main: `96e02a79db5e`
- qsl-server main: `0826ffa4d6f3`
- qsl-server origin/main: `0826ffa4d6f3`
- qsl-server mirror/main: `0826ffa4d6f3`
- qsl-attachments main: `e94107ac094d`
- qsl-attachments origin/main: `e94107ac094d`
- qsl-attachments mirror/main: `e94107ac094d`

## READY proof
- READY_COUNT: `1`
- Sole READY item: `NA-0233A — qsc PR Critical-Path CI Rebalance`
- Proof source: refreshed `NEXT_ACTIONS.md` on `main`

## Worktree / branch / PR
- Worktree path: `/srv/qbuild/work/NA-0233A/qsl-protocol`
- Branch: `na-0233a-closeout-ci-rebalance`
- PR: `pending creation`
- Merge commit: `n/a`

## Failures / recoveries
- `git remote -v` using `/srv/qbuild/mirror/qsl-protocol.git`, `/srv/qbuild/mirror/qsl-server.git`, and `/srv/qbuild/mirror/qsl-attachments.git` -> recoverable because the mirror roots are `/srv/qbuild/mirrors/*` and the first probe was a simple workdir-path mistake during preflight; corrected by rerunning against the actual mirror/worktree paths; final result: remotes-aware refresh proof captured for all three repos.
- `sed -n '1,220p' docs/archive/testplans/NA-0232_qsc_handshake_seed_closeout_evidence.md` -> recoverable because the archived filename on `main` is `docs/archive/testplans/NA-0232_qsc_handshake_seed_deterministic_rng_path_resolution_evidence.md`; corrected by rerunning against the real path; final result: prior closeout artifact pattern captured before patching this governance lane.

## Validation / CI notes
- Pre-mutation authority proof completed: disk watermark green, configured-remotes-only refresh completed for `qsl-protocol`, `qsl-server`, and `qsl-attachments`, `READY_COUNT=1` with `NA-0233A` as the sole READY item, `qsl-server READY=0`, and `qsl-attachments READY=0`.
- Refreshed current-main proof shows PR #690 merged as `96e02a79db5e` and that merge commit is present on refreshed `main`; `.github/workflows/ci.yml` now keeps protected `ci-4a` as Linux qsc build plus smoke coverage and `.github/workflows/macos-build.yml` now keeps protected `macos-qsc-qshield-build` as macOS build plus smoke coverage, while the old broad Linux and timed full-serial macOS suites remain available outside pull-request critical-path gating.
- Refreshed current PR #688 proof shows it remains OPEN at head `d9a0d3260ae0` with merge state `DIRTY`; current required-context snapshot on that stale head still reports `ci-4a=failure` and `macos-qsc-qshield-build=cancelled`, so the remaining blocker is now stale-base resume work rather than unresolved PR critical-path CI design.
- Planned local validation for this governance-only lane: goal-lint, markdown inventory counts, manual markdown link-integrity, added-line leak-safe scan, and scope guard only; no runtime battery reruns.

## Disk watermark
- Filesystem: `/srv/qbuild`
- Total GiB: `484`
- Used GiB: `208`
- Free GiB: `276`
- Used %: `43%`

## Next-watch items
- Run the governance-only validation bundle on the final branch tree, push `na-0233a-closeout-ci-rebalance`, create exactly one PR, poll protected contexts only via bounded REST checks, merge only with a merge commit once the required set is green, and then refresh `main` again to prove `NA-0233A` is `DONE`, `NA-0233` is the sole `READY` item, the journal entry is present, PR #688 remains open, and the workspace is clean.

# Rolling Operations Journal Entry

- Directive: `DIRECTIVE 286 — NA-0233A qsc PR Critical-Path CI Rebalance`
- Begin timestamp (America/Chicago): 2026-04-12T08:07:09-05:00
- Begin timestamp (UTC): 2026-04-12T13:07:09Z
- End timestamp (America/Chicago): 2026-04-12T17:43:06-05:00
- End timestamp (UTC): 2026-04-12T22:43:06Z

## Repo SHAs
- qsl-protocol branch: `na-0233a-ci-critical-path-rebalance`
- qsl-protocol HEAD: `0e37e676b20f`
- qsl-protocol main: `96e02a79db5e`
- qsl-protocol origin/main: `96e02a79db5e`
- qsl-protocol mirror/main: `96e02a79db5e`
- qsl-server main: `0826ffa4d6f3`
- qsl-server origin/main: `0826ffa4d6f3`
- qsl-server mirror/main: `0826ffa4d6f3`
- qsl-attachments main: `e94107ac094d`
- qsl-attachments origin/main: `e94107ac094d`
- qsl-attachments mirror/main: `e94107ac094d`

## READY proof
- READY_COUNT: `1`
- Sole READY item: `NA-0233A — qsc PR Critical-Path CI Rebalance`
- Proof source: refreshed `NEXT_ACTIONS.md` on `main`

## Worktree / branch / PR
- Worktree path: `/srv/qbuild/work/NA-0233A/qsl-protocol`
- Branch: `na-0233a-ci-critical-path-rebalance`
- PR: `PR #690 https://github.com/QuantumShieldLabs/qsl-protocol/pull/690`
- Merge commit: `96e02a79db5e`

## Failures / recoveries
- `rg -n -A60 -B10 'ci-4a:|macos-qsc-qshield-build:' .github/workflows/ci.yml .github/workflows/macos-build.yml` -> recoverable because the zero-match result came from probing the wrong workflow keys before anchoring on the live `name:` fields and command lines; corrected by rerunning with exact job-name and command patterns; final result: current workflow blocker proof captured.
- `sed -n '1,240p' .github/workflows/goal-lint.yml` -> recoverable because the goal-lint workflow file is actually `.github/workflows/goal-compliance.yml`; corrected by rerunning against the real file path; final result: goal-lint workflow and `tools/goal_lint.py` usage confirmed.
- The first bounded required-context poll exited after one iteration because `set -e` treated the intentional “not green yet” probe status as fatal, and the second attempt overflowed `/usr/bin/python3` argv by passing full JSON payloads on the command line; recoverable because both failures were local polling-script shape issues inside the directive’s bounded retry budget; corrected by handling probe status explicitly and moving JSON handoff to temp files; final result: required protected-context polling completed successfully with the protected set green.

## Validation / CI notes
- Pre-mutation authority proof completed: disk watermark green, configured-remotes-only refresh completed for `qsl-protocol`, `qsl-server`, and `qsl-attachments`, `READY_COUNT=1` with `NA-0233A` as the sole READY item, `qsl-server READY=0`, and `qsl-attachments READY=0`.
- Refreshed blocker proof still shows protected `ci-4a` running `cargo +stable build -p qsc --release --locked` plus `cargo +stable test -p qsc --locked`, protected `macos-qsc-qshield-build` running the full serial qsc suite under `timeout-minutes: 45`, and branch protection on `main` still requiring both status names unchanged.
- Local validation already green on the working tree for workflow YAML load (`.github/workflows/ci.yml`, `.github/workflows/macos-build.yml`), docs inventory counts (`tests/*.md=43`, `tests/**/*.md=1`, `docs/*.md=224`, `docs/**/*.md=219`), manual markdown link-integrity (`TOTAL_MISSING 0`), and added-line leak-safe scan (`v1-path pattern count: 0`, `hex32plus pattern count: 0`).
- Local required-command proof already green on qbuild: `cargo +stable build -p qsc --release --locked`; `cargo +stable test -p qsc --locked --test vault -- --test-threads=1`; `cargo +stable test -p qsc --locked --test handshake_contract_na0217i -- --test-threads=1`; `cargo +stable test -p qsc --locked --test qsp_protocol_gate -- --test-threads=1`; overlapping platform-neutral macOS smoke-shape commands also passed locally, including `cargo build -p qshield-cli --release --locked`.
- Local helper validation after adding the classifier correction: `bash -n scripts/ci/classify_ci_scope.sh` passes, and `scripts/ci/classify_ci_scope.sh .github/workflows/ci.yml tests/NA-0233A_rolling_journal_entry_testplan.md` now reports `docs_only=false`, `workflow_security=true`, `runtime_critical=false`, `scope_class=workflow_security`.
- Local `goal-lint` passed on the committed branch head via synthesized `GITHUB_EVENT_PATH` before the first push, and the branch was pushed immediately after the full local validation bundle turned green.
- Post-push/current PR state: PR #690 merged at `2026-04-12T22:41:08Z` from branch head `0e37e676b20f` via merge commit `96e02a79db5e`; the required protected contexts reached green, the markdown-under-`tests/` classifier correction removed unrelated non-required advisory churn from this workflow-only lane, and refreshed `main` now carries the rebalance while PR #688 remains open for later resume.

## Disk watermark
- Filesystem: `/srv/qbuild`
- Total GiB: `484`
- Used GiB: `206`
- Free GiB: `278`
- Used %: `43%`

## Next-watch items
- Push the classifier correction, confirm the required contexts remain green without the unrelated `advisories` lane, merge only with a merge commit once the PR rollup is clean, and then refresh `main` again to prove the rebalance landed while PR #688 remains open and resumable.

# Rolling Operations Journal Entry

- Directive: `DIRECTIVE 279 — NA-0232 Closeout / Evidence / Tier-0 Successor Promotion`
- Begin timestamp (America/Chicago): 2026-04-10T07:18:28-05:00
- Begin timestamp (UTC): 2026-04-10T12:18:28Z
- End timestamp (America/Chicago): in progress
- End timestamp (UTC): in progress

## Repo SHAs
- qsl-protocol branch: `na-0232-closeout-seed`
- qsl-protocol HEAD: `pending governance closeout commit`
- qsl-protocol main: `24d7a5a5d93e`
- qsl-protocol origin/main: `24d7a5a5d93e`
- qsl-protocol mirror/main: `24d7a5a5d93e`
- qsl-server main: `0826ffa4d6f3`
- qsl-server origin/main: `0826ffa4d6f3`
- qsl-server mirror/main: `0826ffa4d6f3`
- qsl-attachments main: `e94107ac094d`
- qsl-attachments origin/main: `e94107ac094d`
- qsl-attachments mirror/main: `e94107ac094d`

## READY proof
- READY_COUNT: `1`
- Sole READY item: `NA-0232 — QSC_HANDSHAKE_SEED Deterministic RNG Path Resolution`
- Proof source: refreshed `NEXT_ACTIONS.md` on `main`

## Worktree / branch / PR
- Worktree path: `/srv/qbuild/work/NA-0232/qsl-protocol`
- Branch: `na-0232-closeout-seed`
- PR: `pending creation`
- Merge commit: `n/a`

## Failures / recoveries
- No recovered failures at the time this entry was written.

## Validation / CI notes
- Pre-mutation authority proof completed: disk watermark green, configured-remotes-only refresh completed for `qsl-protocol`, `qsl-server`, and `qsl-attachments`, `READY_COUNT=1` with `NA-0232` as the sole READY item, and qsl-server/qsl-attachments both remained `READY=0`.
- Refreshed main carries PR #685 merge `24d7a5a5d93e`, `DECISIONS.md` `D-0400`, the `TRACEABILITY.md` `NA-0232 implementation/evidence` entry, the `DOC-AUD-003` `F02` resolved state, the merged handshake runtime removal, and the merged seed-regression test.
- Closeout changes are governance-only: archive evidence, `DECISIONS.md` `D-0401`, traceability entries, queue transition from `NA-0232` to approved `NA-0233`, this rolling journal entry, and the matching closeout testplan stub.
- Successor rationale: refreshed `DOC-AUD-003` orders `F03` MockProvider fixed vault key immediately after resolved `F02`; `F04` follows, and KT remains prerequisite-blocked on serialization/profile plus bundle-signature semantics.
- Planned local validation: goal-lint, manual markdown link-integrity, docs inventory, added-line leak-safe scan, and scope guard only; no runtime battery in this governance-only lane.
- Protected checks: pending branch push and PR creation.

## Disk watermark
- Filesystem: `/srv/qbuild`
- Total GiB: `484`
- Used GiB: `196`
- Free GiB: `288`
- Used %: `41%`

## Next-watch items
- Before merge, prove the PR diff is limited to the six authorized governance paths and poll protected contexts only via bounded REST checks.
- After merge, refresh `main` and prove `NA-0232` is `DONE`, `NA-0233` is the sole `READY` item, this journal entry is present, the staged packet remains unchanged, and the workspace is clean.

# Rolling Operations Journal Entry

- Directive: `DIRECTIVE 278 — NA-0232 QSC_HANDSHAKE_SEED Deterministic RNG Path Resolution`
- Begin timestamp (America/Chicago): 2026-04-10T06:16:23-05:00
- Begin timestamp (UTC): 2026-04-10T11:16:23Z
- End timestamp (America/Chicago): in progress
- End timestamp (UTC): in progress

## Repo SHAs
- qsl-protocol branch: `na-0232-handshake-seed-resolution`
- qsl-protocol HEAD: `pending commit after first green local bundle`
- qsl-protocol main: `635f14a84542`
- qsl-protocol origin/main: `635f14a84542`
- qsl-protocol mirror/main: `635f14a84542`
- qsl-server main: `0826ffa4d6f3`
- qsl-server origin/main: `0826ffa4d6f3`
- qsl-server mirror/main: `0826ffa4d6f3`
- qsl-attachments main: `e94107ac094d`
- qsl-attachments origin/main: `e94107ac094d`
- qsl-attachments mirror/main: `e94107ac094d`

## READY proof
- READY_COUNT: `1`
- Sole READY item: `NA-0232 — QSC_HANDSHAKE_SEED Deterministic RNG Path Resolution`
- Proof source: refreshed `NEXT_ACTIONS.md` on `main`

## Worktree / branch / PR
- Worktree path: `/srv/qbuild/work/NA-0232/qsl-protocol`
- Branch: `na-0232-handshake-seed-resolution`
- PR: `pending creation`
- Merge commit: `n/a`

## Failures / recoveries
- `awk 'BEGIN{inblock=0} /^## NA-0232/{inblock=1} inblock{print} /^## NA-/{if(inblock && $0 !~ /^## NA-0232/ && NR>1) exit}' NEXT_ACTIONS.md` and the first broad READY counter produced unusable queue proof because the item heading level and starter text did not match the command shape; recoverable as a pre-mutation command-shape issue; corrected by rerunning with a line-based parser for `### NA-*` blocks and exact `Status: READY`; final result: `READY_COUNT=1`, sole READY `NA-0232`.

## Validation / CI notes
- Pre-mutation authority proof completed: disk watermark green, configured-remotes-only refresh completed for `qsl-protocol`, `qsl-server`, and `qsl-attachments`, `READY_COUNT=1` with `NA-0232` as the sole READY item, and qsl-server/qsl-attachments both remained `READY=0`.
- Current-main truth: the deterministic RNG path was still reachable in shipped/shared `qsc` through `perform_handshake_init_with_route()` -> `hs_session_id()` -> `hs_rand_bytes()` -> `QSC_HANDSHAKE_SEED`; final determination `still_reachable`.
- Planned local validation: full directive bundle after the bounded runtime fix and companion governance evidence are complete.
- Protected checks: pending branch push and PR creation.
- Retry notes: one pre-mutation command-shape recovery; no local validation retries or CI reruns yet.

## Disk watermark
- Filesystem: `/srv/qbuild`
- Total GiB: `484`
- Used GiB: `193`
- Free GiB: `291`
- Used %: `40%`

## Next-watch items
- Run the full local validation bundle on the final branch tree, push immediately after the first green local bundle, create exactly one PR, poll required protected contexts only via bounded REST checks, and merge only with a merge commit once the protected set is green.
- After merge, refresh `main` and prove the seed-path resolution, sole READY `NA-0232`, journal entry presence, and clean workspace without starting closeout.

# Rolling Operations Journal Entry

- Directive: `DIRECTIVE 276 — NA-0231 ML-DSA-65 Timing Oracle Resolution`
- Begin timestamp (America/Chicago): 2026-04-09T20:19:22-05:00
- Begin timestamp (UTC): 2026-04-10T01:19:22Z
- End timestamp (America/Chicago): in progress
- End timestamp (UTC): in progress

## Repo SHAs
- qsl-protocol branch: `na-0231-mldsa-timing-resolution`
- qsl-protocol HEAD: `pending commit after first green local bundle`
- qsl-protocol main: `df3850e903ce`
- qsl-protocol origin/main: `df3850e903ce`
- qsl-protocol mirror/main: `df3850e903ce`
- qsl-server main: `0826ffa4d6f3`
- qsl-server origin/main: `0826ffa4d6f3`
- qsl-server mirror/main: `0826ffa4d6f3`
- qsl-attachments main: `e94107ac094d`
- qsl-attachments origin/main: `e94107ac094d`
- qsl-attachments mirror/main: `e94107ac094d`

## READY proof
- READY_COUNT: `1`
- Sole READY item: `NA-0231 — ML-DSA-65 Timing Oracle Resolution`
- Proof source: refreshed `NEXT_ACTIONS.md` on `main`

## Worktree / branch / PR
- Worktree path: `/srv/qbuild/work/NA-0231/qsl-protocol`
- Branch: `na-0231-mldsa-timing-resolution`
- PR: `pending creation`
- Merge commit: `n/a`

## Failures / recoveries
- `cargo tree --manifest-path qsl/qsl-client/qsc/Cargo.toml -e normal -i ml-dsa@0.0.4` -> recoverable because zero matches are the expected proof outcome for the shipped `qsc` runtime graph; corrected by treating the zero-match result as evidence that `ml-dsa 0.0.4` is absent from the runtime path and confirming the surviving lockfile hit via `Cargo.lock`; final result: runtime path proved to use only `ml-dsa 0.1.0-rc.7`.
- `cargo fmt --check` -> recoverable because the new handshake regression tests needed standard rustfmt wrapping only; corrected by running `rustfmt --edition 2021 qsl/qsl-client/qsc/tests/handshake_mvp.rs`; final result: `cargo fmt --check` passed on rerun.
- `cargo test --manifest-path tools/refimpl/quantumshield_refimpl/Cargo.toml --locked` from `qsl/qsl-client/qsc/` -> recoverable because the manifest path was correct only from the repo root, making this a bounded command-context mistake; corrected by rerunning the same command from `/srv/qbuild/work/NA-0231/qsl-protocol`; final result: refimpl test suite passed.

## Validation / CI notes
- Current-main truth: refreshed dependency and advisory proof shows the staged ML-DSA verify-path finding is stale on current `main`; shipped `qsc` / shared refimpl resolves `ml-dsa 0.1.0-rc.7`, while upstream `RUSTSEC-2025-0144` / `GHSA-hcp2-x6j4-29j7` scope the issue to signing and mark `>= 0.1.0-rc.3` as patched.
- Local validation: `cargo test --test handshake_mvp`, `cargo fmt --check`, `cargo build --locked`, `cargo clippy --locked -- -D warnings`, `cargo test --test handshake_security_closure`, `cargo test --test qsp_protocol_gate`, `cargo test --test handshake_contract_na0217i`, `cargo test --test identity_binding`, `cargo test --test identity_foundation_contract_na0217d`, `cargo test --test protocol_state_contract_na0217c`, `cargo test --test fs_store_contract_na0217b`, `cargo test --manifest-path tools/refimpl/quantumshield_refimpl/Cargo.toml --locked`, markdown inventory counts, manual markdown link-integrity check, and added-line leak-safe scan are green on the local branch tree.
- Protected checks: pending branch push and PR creation.
- Retry notes: one bounded rustfmt rerun and one bounded manifest-path rerun; no CI reruns yet.

## Disk watermark
- Filesystem: `/srv/qbuild`
- Total GiB: `484`
- Used GiB: `190`
- Free GiB: `294`
- Used %: `40%`

## Next-watch items
- Commit the stale-on-main evidence lane, push immediately after the first green local bundle, and capture the push timestamp plus branch SHA.
- Create exactly one PR, run local goal-lint against the real head SHA, poll required protected contexts only via bounded REST checks, and merge only with a merge commit once the protected set is green.

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

- Directive: `DIRECTIVE 275 — NA-0230 Closeout / Evidence / Tier-0 Security Successor Promotion`
- Begin timestamp (America/Chicago): 2026-04-09T15:07:57-05:00
- Begin timestamp (UTC): 2026-04-09T20:07:57Z
- End timestamp (America/Chicago): in progress
- End timestamp (UTC): in progress

## Repo SHAs
- qsl-protocol branch: `na-0230-closeout-audit-intake`
- qsl-protocol HEAD: `0084fabe8be0`
- qsl-protocol main: `0084fabe8be0`
- qsl-protocol origin/main: `0084fabe8be0`
- qsl-protocol mirror/main: `0084fabe8be0`
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
- Branch: `na-0230-closeout-audit-intake`
- PR: `pending creation`
- Merge commit: `n/a`

## What changed
- Prepared the governance-only `NA-0230` closeout lane by archiving durable evidence for the already-merged PR #681 implementation/evidence state now present on refreshed `main`.
- Updated the queue-closeout surfaces so `DECISIONS.md`, `TRACEABILITY.md`, and `NEXT_ACTIONS.md` can record the merged intake canon truthfully without reopening runtime scope.
- What worked: refreshed merged `main` already carries `D-0396`, the `NA-0230 implementation/evidence` traceability entry, `DOC-AUD-003`, and the staged 8-file packet unchanged, so the closeout can stay governance-only.
- The successor choice remains bounded and evidence-driven: `DOC-AUD-003` orders Tier 0 as `F01` ML-DSA timing, `F02` `QSC_HANDSHAKE_SEED`, `F03` MockProvider vault-key hardening, and `F04` the vault read-path floor, while KT remains prerequisite-blocked, so `NA-0231 — ML-DSA-65 Timing Oracle Resolution` is the sole truthful READY follow-on.

## Failures / recoveries
- `rg -n 'DC1' NEXT_ACTIONS.md` -> recoverable because zero matches are a valid proof outcome while confirming whether `DC1` is already used; corrected by treating the zero-match result as evidence together with the neighboring `DB1` closeout block already present in `NEXT_ACTIONS.md`; final result: `DC1` is the next unused truthful closeout token for `NA-0230`.

## Validation / CI notes
- Pre-mutation authority proof completed: disk watermark green, configured-remotes-only refresh completed for `qsl-protocol`, `qsl-server`, and `qsl-attachments`, `READY_COUNT=1` with `NA-0230` as the sole READY item, and qsl-server/qsl-attachments both remained `READY=0`.
- Refreshed merged-main proof completed: PR #681 is already merged at `0084fabe8be0`, and refreshed `main` still carries the implementation/evidence surfaces needed for truthful closeout.
- Current-main closeout-basis proof completed: `DOC-AUD-003` orders Tier 0 as `F01` through `F04`, explicitly leaves KT prerequisite-blocked, and therefore makes ML-DSA timing the first direct successor.
- Local docs/governance validation, branch push, PR creation, protected-check polling, and merge remain pending at this entry.

## Disk watermark
- Filesystem: `/srv/qbuild`
- Total GiB: `484`
- Used GiB: `190`
- Free GiB: `294`
- Used %: `40%`

## Next-watch items
- Run the full docs/governance validation bundle on the final tree, then push immediately after the first green local bundle so the closeout state does not remain only on qbuild.
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

- Directive: `DIRECTIVE 277 — NA-0231 Closeout / Evidence / Tier-0 Successor Promotion`
- Begin timestamp (America/Chicago): 2026-04-09T21:23:20-05:00
- Begin timestamp (UTC): 2026-04-10T02:23:20Z
- End timestamp (America/Chicago): 2026-04-09T21:25:57-05:00
- End timestamp (UTC): 2026-04-10T02:25:57Z

## Repo SHAs
- qsl-protocol branch: `na-0231-closeout-mldsa`
- qsl-protocol HEAD: `pending commit after governance-only validation`
- qsl-protocol main: `8db0e709a37c`
- qsl-protocol origin/main: `8db0e709a37c`
- qsl-protocol mirror/main: `8db0e709a37c`
- qsl-server main: `0826ffa4d6f3`
- qsl-server origin/main: `0826ffa4d6f3`
- qsl-server mirror/main: `0826ffa4d6f3`
- qsl-attachments main: `e94107ac094d`
- qsl-attachments origin/main: `e94107ac094d`
- qsl-attachments mirror/main: `e94107ac094d`

## READY proof
- READY_COUNT: `1`
- Sole READY item: `NA-0231 — ML-DSA-65 Timing Oracle Resolution`
- Proof source: refreshed `NEXT_ACTIONS.md` on `main`

## Worktree / branch / PR
- Worktree path: `/srv/qbuild/work/NA-0231/qsl-protocol`
- Branch: `na-0231-closeout-mldsa`
- PR: `pending creation`
- Merge commit: `n/a`

## What changed
- Prepared the governance-only `NA-0231` closeout lane by adding durable archive evidence for the merged stale-on-main ML-DSA resolution, marking `NA-0231` `DONE`, appending `DECISIONS.md` `D-0399`, adding `TRACEABILITY.md` closeout and successor entries, and adding the matching docs-only closeout testplan stub.
- Promoted exactly one successor, `NA-0232 — QSC_HANDSHAKE_SEED Deterministic RNG Path Resolution`, because refreshed `DOC-AUD-003` orders `F02` as the first still-live Tier 0 item after stale `F01`; KT remains prerequisite-blocked and cannot leapfrog the remaining Tier 0 runtime debt.
- The staged 8-file audit packet remains read-only and unchanged.

## Failures / recoveries
- `sha256sum $(git ls-files 'docs/audit/incoming/2026-04-09_security_batch/**')` -> recoverable because filenames contain spaces and Unicode dashes and the command used whitespace-delimited expansion; corrected with `git ls-files -z ... | xargs -0 sha256sum`; final result: 8-file incoming packet inventory hashed successfully.

## Validation / CI notes
- Local validation: pending governance-only bundle after edits.
- Protected checks: pending PR creation and bounded REST polling.
- Retry notes: one command-shape recovery for null-delimited staged-packet hashing.

## Disk watermark
- Filesystem: `/srv/qbuild`
- Total GiB: `484`
- Used GiB: `193`
- Free GiB: `291`
- Used %: `40%`

## Next-watch items
- Run the governance-only validation bundle: local goal-lint via synthesized event payload, markdown inventory counts, manual markdown link-integrity check, changed-path scope proof, added-line leak-safe scan, and no runtime battery.
- Create exactly one PR, poll protected contexts via bounded REST only, and merge only with a merge commit once required checks are green.

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

# Rolling Operations Journal Entry

- Directive: `DIRECTIVE 281 — NA-0233 MockProvider Fixed Vault Key Scope Repair`
- Begin timestamp (America/Chicago): 2026-04-10T18:25:12-05:00
- Begin timestamp (UTC): 2026-04-10T23:25:12Z
- End timestamp (America/Chicago): pending at authoring time
- End timestamp (UTC): pending at authoring time

## Repo SHAs
- qsl-protocol branch: `na-0233-scope-repair-mockprovider`
- qsl-protocol HEAD: `4a83de93c311`
- qsl-protocol main: `4a83de93c311`
- qsl-protocol origin/main: `4a83de93c311`
- qsl-protocol mirror/main: `4a83de93c311`
- qsl-server main: `0826ffa4d6f3`
- qsl-server origin/main: `0826ffa4d6f3`
- qsl-server mirror/main: `0826ffa4d6f3`
- qsl-attachments main: `e94107ac094d`
- qsl-attachments origin/main: `e94107ac094d`
- qsl-attachments mirror/main: `e94107ac094d`

## READY proof
- READY_COUNT: `1`
- Sole READY item: `NA-0233 — MockProvider Fixed Vault Key Resolution`
- Proof source: refreshed `NEXT_ACTIONS.md` on `main`

## Worktree / branch / PR
- Worktree path: `/srv/qbuild/work/NA-0233/qsl-protocol`
- Branch: `na-0233-scope-repair-mockprovider`
- PR: `pending creation`
- Merge commit: `n/a`

## What changed
- Re-proved that refreshed current `main` still routes the live MockProvider fixed/default vault-key path through `qsl/qsl-client/qsc/src/vault/mod.rs`, with shipped/shared call sites in `qsl/qsl-client/qsc/src/main.rs` and `qsl/qsl-client/qsc/src/tui/controller/commands/dispatch.rs`.
- Re-proved that the directly affected helper/test seam includes `qsl/qsl-client/qsc/tests/common/mod.rs`, `qsl/qsl-client/qsc/tests/vault.rs`, and additional current-main mock-vault consumers under `qsl/qsl-client/qsc/tests/**`.
- Repaired `NA-0233` queue truth in governance only so the later runtime lane can touch the actual bounded fix surfaces without widening past refreshed contradiction proof.

## Failures / recoveries
- None.

## Validation / CI notes
- Planned local validation: goal-lint via synthesized event payload, markdown inventory, manual markdown link-integrity, added-line leak-safe scan, and changed-path scope proof only.
- No runtime battery is part of this governance-only lane.
- PR creation and protected-check polling are pending at authoring time.

## Disk watermark
- Filesystem: `/srv/qbuild`
- Total GiB: `484`
- Used GiB: `197`
- Free GiB: `287`
- Used %: `41%`

## Next-watch items
- Finish the governance-only validation bundle, then create exactly one PR and poll required contexts only via bounded REST checks.
- Retry the actual MockProvider runtime lane only from refreshed `main` using the repaired `NA-0233` scope; the prior queue block was too narrow to authorize the real fix truthfully.

# Rolling Operations Journal Entry

- Directive: `DIRECTIVE 285 — NA-0233 Queue-Truth Repair / CI-Critical-Path Successor Promotion`
- Begin timestamp (America/Chicago): 2026-04-12T07:16:40-05:00
- Begin timestamp (UTC): 2026-04-12T12:16:40Z
- End timestamp (America/Chicago): pending at authoring time
- End timestamp (UTC): pending at authoring time

## Repo SHAs
- qsl-protocol branch: `na-0233-blocked-on-ci-repair`
- qsl-protocol HEAD: `00ed2d13dcda`
- qsl-protocol main: `00ed2d13dcda`
- qsl-protocol origin/main: `00ed2d13dcda`
- qsl-protocol mirror/main: `00ed2d13dcda`
- qsl-server main: `0826ffa4d6f3`
- qsl-server origin/main: `0826ffa4d6f3`
- qsl-server mirror/main: `0826ffa4d6f3`
- qsl-attachments main: `e94107ac094d`
- qsl-attachments origin/main: `e94107ac094d`
- qsl-attachments mirror/main: `e94107ac094d`

## READY proof
- READY_COUNT: `1`
- Sole READY item: `NA-0233 — MockProvider Fixed Vault Key Resolution`
- Proof source: refreshed `NEXT_ACTIONS.md` on `main`

## Worktree / branch / PR
- Worktree path: `/srv/qbuild/work/NA-0233/qsl-protocol`
- Branch: `na-0233-blocked-on-ci-repair`
- PR: `pending creation`
- Merge commit: `n/a`

## What changed
- Re-proved that PR #688 remains OPEN at head `d9a0d3260ae0` with merge state `BLOCKED`.
- Re-proved that required `ci-4a` currently fails while `.github/workflows/ci.yml` still runs `cargo +stable build -p qsc --release --locked` plus `cargo +stable test -p qsc --locked` as a broad whole-package qsc gate.
- Re-proved that required `macos-qsc-qshield-build` currently cancels while `.github/workflows/macos-build.yml` still runs `cargo test -p qsc --locked --jobs 1 -- --test-threads=1` under `timeout-minutes: 45`.
- Repaired queue truth in governance only so `NA-0233` now reflects the real blocker and `NA-0233A — qsc PR Critical-Path CI Rebalance` becomes the next truthful successor.

## Failures / recoveries
- None.

## Validation / CI notes
- Planned local validation: goal-lint via synthesized event payload, markdown inventory, manual markdown link-integrity, added-line leak-safe scan, and changed-path scope proof only.
- No runtime battery is part of this governance-only lane.
- PR creation, protected-check polling, and merge are pending at authoring time.

## Disk watermark
- Filesystem: `/srv/qbuild`
- Total GiB: `484`
- Used GiB: `206`
- Free GiB: `278`
- Used %: `43%`

## Next-watch items
- Finish the governance-only validation bundle, then create exactly one PR and poll required contexts only via bounded REST checks.
- Leave PR #688 open and untouched; resume that runtime lane only after the CI-critical-path successor lands on refreshed `main`.
