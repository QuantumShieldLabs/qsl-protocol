Goals: G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-21

# Rolling Operations Journal

# Rolling Operations Journal Entry

- Directive: `DIRECTIVE 337 — NA-0237 Resume KT Verifier Implementation from Preserved WIP on Fresh Worktree`
- Begin timestamp (America/Chicago): 2026-04-21T07:34:04-05:00
- Begin timestamp (UTC): 2026-04-21T12:34:04Z
- End timestamp (America/Chicago): pending at authoring time
- End timestamp (UTC): pending at authoring time

## Repo SHAs
- qsl-protocol branch: `na-0237-kt-verifier-fail-closed-v2`
- qsl-protocol HEAD: `pending local suite2-vector repair commit at authoring time (previous branch head a1a23072bd8e)`
- qsl-protocol main: `905c32f4e325`
- qsl-protocol origin/main: `9643c566b485`
- qsl-protocol mirror/main: `905c32f4e325`
- qsl-server main: `0826ffa4d6f3`
- qsl-server origin/main: `0826ffa4d6f3`
- qsl-server mirror/main: `0826ffa4d6f3`
- qsl-attachments main: `1e1ae272a4cb`
- qsl-attachments origin/main: `1e1ae272a4cb`
- qsl-attachments mirror/main: `1e1ae272a4cb`

## READY proof
- READY_COUNT: `1`
- Sole READY item: `NA-0237 — KT Verifier Fail-Closed Implementation + Responder Coverage`
- Proof source: refreshed `NEXT_ACTIONS.md` on `origin/main`

## Worktree / branch / PR
- Dirty fallback worktree path: `/srv/qbuild/work/NA-0237/qsl-protocol`
- Preservation bundle path: `/srv/qbuild/tmp/na0237_scope_repair_preservation`
- Clean implementation worktree path: `/srv/qbuild/work/NA-0237-scope-repair/qsl-protocol`
- Branch: `na-0237-kt-verifier-fail-closed-v2`
- PR: `#708 open at authoring time`
- Merge commit: `n/a`

## What changed
- Re-proved from refreshed configured-remotes-only state that qsl-protocol `origin/main` now carries the merged `NA-0237` scope repair at `9643c566b485`, that `READY_COUNT=1` with `NA-0237` as the sole READY item, and that qsl-server plus qsl-attachments remain `READY=0`.
- Confirmed refreshed `origin/main` still lacks the live `NA-0237` implementation outputs while still exposing the expected placeholder surfaces: `KtError::NotImplemented` in `tools/refimpl/quantumshield_refimpl/src/kt/mod.rs` and `KtAllowEmptyOnly` / actor-local deferred bundle TBS handling in `tools/actors/refimpl_actor_rs/src/main.rs`.
- Reused the previously clean scope-repair worktree as the sole clean implementation worktree by branching it directly from refreshed `origin/main`; direct `switch main` was not possible because `main` is intentionally still checked out in the untouched dirty fallback worktree.
- Replayed only the preserved bounded KT runtime/test/vector files from the preservation bundle into the clean branch, explicitly excluding the preserved governance files, and applied the minimal `sort_by_key` clippy-only fix in `tools/refimpl/quantumshield_refimpl/src/qsp/state.rs`.
- Opened PR `#708` for the implementation/evidence lane after the first full green local validation bundle, then repaired the `suite2-vectors` CI failure in-scope by rewrapping `inputs/suite2/vectors/qshield_suite2_kt_verifier_vectors_v1.json` to the standard Suite-2 vector-set schema and updating `tools/refimpl/quantumshield_refimpl/tests/kt_verifier_vectors.rs` to consume the canonical `vectors[].expect/ext` shape.

## Failures / recoveries
- A zero-match implementation-marker probe for `D-0424` / `NA-0237 implementation/evidence` on refreshed `origin/main` produced no match, which is a valid proof that the resumed implementation was not yet present. Corrective action: reran the proof with zero-safe `awk` counting so the absence is explicit. Final result: `D0424_COUNT=0`, `TRACE_NA0237_IMPL_COUNT=0`.
- `git -C /srv/qbuild/work/NA-0237-scope-repair/qsl-protocol switch main` failed because branch `main` is already checked out in the untouched dirty fallback worktree. Classified as a recoverable command/worktree-selection issue. Corrective action: created and switched the clean implementation worktree directly to `na-0237-kt-verifier-fail-closed-v2` from refreshed `origin/main` instead. Final result: clean implementation branch now tracks `origin/main` at `9643c566b485` without mutating the dirty fallback worktree.
- The first PR-check polling wrapper exited early because `set -e` treated the local Python status probe's pending-check return code as fatal. Classified as a recoverable command-shape issue. Corrective action: reran polling with explicit return-code handling around the classifier. Final result: check-state polling resumed truthfully without using watch mode.
- `cargo fmt --check` failed after the `suite2-vectors` repair because the patched negative-case assertion block needed standard rustfmt indentation. Classified as a recoverable in-scope formatting failure. Corrective action: ran `cargo fmt` and reran the affected checks. Final result: `cargo fmt --check`, `python3 scripts/ci/validate_suite2_vectors.py`, and `cargo test --manifest-path tools/refimpl/quantumshield_refimpl/Cargo.toml --locked kt_verifier_vectors -- --nocapture` are green.
- `rg -n \"DIRECTIVE 337|Directive: \`DIRECTIVE 337|na-0237-kt-verifier-fail-closed-v2|#708|a1a23072bd8e\" docs/ops/ROLLING_OPERATIONS_JOURNAL.md` failed because the shell consumed the unmatched backtick before `rg` executed. Classified as a recoverable command-shape issue. Corrective action: reran the journal lookup with single-quoted, backtick-safe patterns. Final result: the Directive 337 journal entry was located and updated without widening scope.

## Validation / CI notes
- Pre-mutation authority proof is complete: disk watermark green (`468 GiB` total / `24 GiB` used / `421 GiB` free / `6%` used), configured-remotes-only refresh completed for qsl-protocol, qsl-server, and qsl-attachments, `STATUS.md` drift remains non-blocking, the preservation bundle is present and usable, and the clean implementation worktree is selected.
- Policy review confirms this implementation lane is satisfied by the authorized journal surface plus one matching rolling-journal testplan stub; no additional `docs/ops/**` path or extra docs-only testplan stub is required.
- First full local validation bundle is green on commit `645fb243f896`: `cargo fmt --check`; `cargo build --locked`; `cargo clippy --locked -- -D warnings`; `cargo audit --deny warnings`; `cargo build -p qshield-cli --release --locked`; `cargo test --manifest-path tools/refimpl/quantumshield_refimpl/Cargo.toml --locked`; `cargo test --manifest-path tools/actors/refimpl_actor_rs/Cargo.toml --locked`; direct KT regressions `kt_verifier_vectors`, `responder_requires_bundle_equivalent_initiator_evidence`, and `responder_binding_rejects_missing_or_mismatched_bundle`; qsc handshake canaries `handshake_security_closure`, `handshake_contract_na0217i`, `handshake_mvp`, and `qsp_protocol_gate`; local goal-lint via synthetic event payload; markdown inventory counts (`tests/*.md=62`, `tests/**/*.md=1`, `docs/*.md=238`, `docs/**/*.md=233`); manual markdown link-integrity runbook (`TOTAL_MISSING 0`); and added-line leak-safe scan (`ADDED_LINE_COUNT 1678`, `v1-path pattern count 3`, `hex32plus pattern count 0`, `secret-like marker count 0`).
- PR `#708` is open and required checks are being repolled after the in-scope `suite2-vectors` repair. The repaired surfaces are green locally at authoring time: `cargo fmt --check`; `python3 scripts/ci/validate_suite2_vectors.py`; and `cargo test --manifest-path tools/refimpl/quantumshield_refimpl/Cargo.toml --locked kt_verifier_vectors -- --nocapture`.
- Remaining at authoring time: push the repair commit to PR `#708`, repoll the required protected contexts, merge with a merge commit once green/accepted-neutral, and capture the refreshed-main post-merge proof.

## Disk watermark
- Filesystem: `/srv/qbuild`
- Total GiB: `468`
- Used GiB: `24`
- Free GiB: `421`
- Used %: `6%`

## Next-watch items
- Keep the replay confined to the preserved KT runtime/test/vector surfaces plus the minimal clippy-only `qsp/state.rs` fix and fresh governance companions.
- Push the suite2-vector repair to PR `#708`, then continue bounded REST polling for the required protected contexts without using watch mode.
- After merge, re-prove that `NA-0237` remains the sole READY item, the journal entry is present on refreshed `main`, the clean worktree is clean, and the dirty fallback worktree remains untouched.

# Rolling Operations Journal Entry

- Directive: `DIRECTIVE 336 — NA-0237 Scope Repair for qsp/state Clippy Gate + Refimpl Test Surface`
- Begin timestamp (America/Chicago): 2026-04-21T07:06:18-05:00
- Begin timestamp (UTC): 2026-04-21T12:06:18Z
- End timestamp (America/Chicago): pending at authoring time
- End timestamp (UTC): pending at authoring time

## Repo SHAs
- qsl-protocol branch: `na-0237-scope-repair-qsp-state-clippy`
- qsl-protocol HEAD: `pending local scope-repair commit at authoring time (refreshed main base 905c32f4e325)`
- qsl-protocol main: `905c32f4e325`
- qsl-protocol origin/main: `905c32f4e325`
- qsl-protocol mirror/main: `905c32f4e325`
- qsl-server main: `0826ffa4d6f3`
- qsl-server origin/main: `0826ffa4d6f3`
- qsl-server mirror/main: `0826ffa4d6f3`
- qsl-attachments main: `1e1ae272a4cb`
- qsl-attachments origin/main: `1e1ae272a4cb`
- qsl-attachments mirror/main: `1e1ae272a4cb`

## READY proof
- READY_COUNT: `1`
- Sole READY item: `NA-0237 — KT Verifier Fail-Closed Implementation + Responder Coverage`
- Proof source: refreshed `NEXT_ACTIONS.md` on `main`

## Worktree / branch / PR
- Dirty KT worktree path: `/srv/qbuild/work/NA-0237/qsl-protocol`
- Preservation bundle path: `/srv/qbuild/tmp/na0237_scope_repair_preservation`
- Temporary governance worktree path: `/srv/qbuild/work/NA-0237-scope-repair/qsl-protocol`
- Branch: `na-0237-scope-repair-qsp-state-clippy`
- PR: `pending at authoring time`
- Merge commit: `n/a`

## What changed
- Re-proved from refreshed configured-remotes-only state that `qsl-protocol` `main`, `origin/main`, and `mirror/main` all match at `905c32f4e325`, that `READY_COUNT=1` with `NA-0237` as the sole READY item, and that `qsl-server` plus `qsl-attachments` each remain `READY=0`.
- Preserved the dirty local KT implementation WIP off-repo without mutating tracked files by capturing `status.txt`, `changed_paths.txt`, `diffstat.txt`, `tracked.patch`, `untracked.zlist`, `untracked.tgz`, and `head_sha.txt` under `/srv/qbuild/tmp/na0237_scope_repair_preservation`.
- Confirmed the first local KT implementation attempt stopped for one narrow scope reason: the lane's required `cargo clippy --locked -- -D warnings` gate fails on untouched out-of-scope code in `tools/refimpl/quantumshield_refimpl/src/qsp/state.rs`, while the newly added direct KT regression file `tools/refimpl/quantumshield_refimpl/tests/kt_verifier_vectors.rs` is part of the same bounded verifier evidence seam.
- Limited this governance-only repair to `NEXT_ACTIONS.md`, `DECISIONS.md`, `TRACEABILITY.md`, `docs/archive/testplans/NA-0237_scope_repair_qsp_state_clippy_and_refimpl_tests_evidence.md`, `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`, and `tests/NA-0237_scope_repair_testplan.md`.

## Failures / recoveries
- None at authoring time.

## Validation / CI notes
- Pre-mutation authority proof is complete: disk watermark green (`468 GiB` total / `24 GiB` used / `421 GiB` free / `6%` used), configured-remotes-only refresh completed for `qsl-protocol`, `qsl-server`, and `qsl-attachments`, refreshed `main` still shows `NA-0237` as the sole READY item, and `STATUS.md` drift remains non-blocking because it still reports the stale `NA-0177` READY state.
- Policy review confirms this governance-only lane is satisfied by the authorized journal surface plus one matching scope-repair testplan stub; no additional `docs/ops/**` path or extra docs-only testplan stub is required.
- Dirty-worktree preservation proof is complete and non-empty: the tracked patch is present, the untracked archive contains the four KT-added files, and the dirty KT worktree remains untouched after preservation.
- Local governance validation, changed-path scope proof, PR creation, protected-check polling, merge, and post-merge refresh proof remain pending at authoring time.

## Disk watermark
- Filesystem: `/srv/qbuild`
- Total GiB: `468`
- Used GiB: `24`
- Free GiB: `421`
- Used %: `6%`

## Next-watch items
- Keep the governance PR changed-path set limited to `NEXT_ACTIONS.md`, `DECISIONS.md`, `TRACEABILITY.md`, `docs/archive/testplans/NA-0237_scope_repair_qsp_state_clippy_and_refimpl_tests_evidence.md`, `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`, and `tests/NA-0237_scope_repair_testplan.md`.
- After merge, re-prove that `NA-0237` remains the sole READY item on refreshed `main`, that the repaired scope lines are present there, and that the dirty KT worktree plus its preservation bundle remain untouched.
- Do not continue KT implementation or reapply the preserved patch in this directive.

# Rolling Operations Journal Entry

- Directive: `DIRECTIVE 324 — NA-0236 Closeout + Promote NA-0237 KT Verifier Implementation`
- Begin timestamp (America/Chicago): 2026-04-19T08:04:46-05:00
- Begin timestamp (UTC): 2026-04-19T13:04:46Z
- End timestamp (America/Chicago): pending at authoring time
- End timestamp (UTC): pending at authoring time

## Repo SHAs
- qsl-protocol branch: `na-0236-closeout-promote-na0237`
- qsl-protocol HEAD: `af9300ac04a8`
- qsl-protocol main: `af9300ac04a8`
- qsl-protocol origin/main: `af9300ac04a8`
- qsl-protocol mirror/main: `af9300ac04a8`
- qsl-server main: `0826ffa4d6f3`
- qsl-server origin/main: `0826ffa4d6f3`
- qsl-server mirror/main: `0826ffa4d6f3`
- qsl-attachments main: `1e1ae272a4cb`
- qsl-attachments origin/main: `1e1ae272a4cb`
- qsl-attachments mirror/main: `1e1ae272a4cb`

## READY proof
- READY_COUNT: `1`
- Sole READY item: `NA-0236 — KT Serialization/Profile + BundleTBS / Bundle-Signature Canon Closure`
- Proof source: refreshed `NEXT_ACTIONS.md` on `main`

## Worktree / branch / PR
- Worktree path: `/srv/qbuild/work/NA-0236/qsl-protocol`
- Branch: `na-0236-closeout-promote-na0237`
- PR: `pending at authoring time`
- Merge commit: `n/a`

## What changed
- Re-proved from refreshed configured-remotes-only state that `qsl-protocol` `main`, `origin/main`, and `mirror/main` all match at `af9300ac04a8`, that `READY_COUNT=1` with `NA-0236` as the sole READY item, and that `qsl-server` plus `qsl-attachments` each remain `READY=0`.
- Re-proved merged-state truth for `NA-0236`: PR `#705` is merged on refreshed GitHub truth as merge commit `af9300ac04a8`, refreshed `main` contains `DOC-CAN-008` plus the supporting schema/spec-closure updates, and the new archive evidence path for this closeout is still absent on `main`.
- Re-read the governance spine, `DOC-OPS-003`, `DOC-AUD-003`, the focused KT audit, and the merged KT canon so this governance-only lane can close `NA-0236` truthfully and promote the exact `NA-0237` successor block without reopening runtime scope.
- Confirmed why KT implementation is now the next truthful runtime lane: the bounded refimpl/actor path still carries `KtAllowEmptyOnly` / `NotImplemented` KT surfaces and caller-deferred bundle semantics, while `DOC-CAN-008` removes the old serialization/profile and `BundleTBS` design blocker that previously prevented a truthful implementation lane.

## Failures / recoveries
- None at authoring time.

## Validation / CI notes
- Pre-mutation authority proof is complete: disk watermark green (`484 GiB` total / `221 GiB` used / `264 GiB` free / `46%` used), configured-remotes-only refresh completed for `qsl-protocol`, `qsl-server`, and `qsl-attachments`, and the active worktree was clean before branch creation.
- Refreshed `main` still lacks the `NA-0236` closeout artifacts at lane start: `NEXT_ACTIONS.md` still marks `NA-0236` `READY`, `NA-0237` is absent, and `docs/archive/testplans/NA-0236_kt_serialization_profile_bundle_signature_closure_evidence.md` is absent.
- Policy review confirms this governance-only lane is satisfied by the authorized journal surface plus one matching closeout testplan stub; no additional `docs/ops/**` path or extra docs-only testplan stub is required.
- Local validation, PR creation, protected-check polling, merge, and post-merge queue proof remain pending at authoring time.

## Disk watermark
- Filesystem: `/srv/qbuild`
- Total GiB: `484`
- Used GiB: `221`
- Free GiB: `264`
- Used %: `46%`

## Next-watch items
- Keep the changed-path set limited to `NEXT_ACTIONS.md`, `DECISIONS.md`, `TRACEABILITY.md`, `docs/archive/testplans/NA-0236_kt_serialization_profile_bundle_signature_closure_evidence.md`, `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`, and `tests/NA-0236_closeout_evidence_testplan.md`.
- Preserve the sole-READY rule: after merge, `NA-0236` must be `DONE` and `NA-0237` must be the only `READY` item.
- Do not let later `F06` / fuzz / adversarial lanes outrank KT verifier implementation now that the merged canon removed the old KT design blocker.

# Rolling Operations Journal Entry

- Directive: `DIRECTIVE 323 — NA-0236 KT Serialization/Profile + BundleTBS / Bundle-Signature Canon Closure`
- Begin timestamp (America/Chicago): 2026-04-19T07:21:12-05:00
- Begin timestamp (UTC): 2026-04-19T12:21:12Z
- End timestamp (America/Chicago): pending at authoring time
- End timestamp (UTC): pending at authoring time

## Repo SHAs
- qsl-protocol branch: `na-0236-kt-canon-closure-v2`
- qsl-protocol HEAD: `58176c02245d`
- qsl-protocol main: `1438fb2015bd`
- qsl-protocol origin/main: `1438fb2015bd`
- qsl-protocol mirror/main: `1438fb2015bd`
- qsl-server main: `0826ffa4d6f3`
- qsl-server origin/main: `0826ffa4d6f3`
- qsl-server mirror/main: `0826ffa4d6f3`
- qsl-attachments main: `1e1ae272a4cb`
- qsl-attachments origin/main: `1e1ae272a4cb`
- qsl-attachments mirror/main: `1e1ae272a4cb`

## READY proof
- READY_COUNT: `1`
- Sole READY item: `NA-0236 — KT Serialization/Profile + BundleTBS / Bundle-Signature Canon Closure`
- Proof source: refreshed `NEXT_ACTIONS.md` on `main`

## Worktree / branch / PR
- Worktree path: `/srv/qbuild/work/NA-0236/qsl-protocol`
- Branch: `na-0236-kt-canon-closure-v2`
- PR: `pending at authoring time`
- Merge commit: `n/a`

## What changed
- Re-proved from refreshed configured-remotes-only state that `qsl-protocol` `main`, `origin/main`, and `mirror/main` all match at `1438fb2015bd`, that `READY_COUNT=1` with `NA-0236` as the sole READY item, and that `qsl-server` plus `qsl-attachments` each remain `READY=0`.
- Completed the required governance/audit read set for this lane, including the refreshed governance spine, `DOC-OPS-003`, `DOC-AUD-003`, and the full eight-document audit packet, then isolated the exact KT prerequisite closure needed before later verifier implementation.
- Confirmed the docs-only lane can stay within the authorized seam by using exactly one rolling-journal file (`docs/ops/ROLLING_OPERATIONS_JOURNAL.md`) plus one matching testplan stub and by limiting substantive changes to canonical/schema/spec-closure/governance surfaces only.
- Added the canonical closure doc `DOC-CAN-008`, the matching `NA-0236` testplan stub, the supporting schema/spec-closure clarifications, and the `D-0421` / `TRACEABILITY.md` implementation-evidence anchors, then committed that bounded docs/governance bundle as `58176c02245d`.

## Failures / recoveries
- `rg -n '^READY' NEXT_ACTIONS.md` exited non-zero because the live queue file does not use a root-level `READY` marker line. Classified as a recoverable zero-match discovery outcome. Corrective action: read `NEXT_ACTIONS.md` directly and switched the READY proof to `Status: READY` plus direct queue-block inspection. Final result: sole READY proof completed truthfully.
- `printf '=== qsl-protocol READY count ===\n' && rg -n '^Status: READY' NEXT_ACTIONS.md && printf 'COUNT=' && rg -c '^Status: READY' ...` exited non-zero after the zero-match branch in sibling repos. Classified as a recoverable zero-match discovery outcome because `qsl-server` and `qsl-attachments` truthfully have no READY items. Corrective action: reran the READY count proof with zero-safe `awk` counting for all three repos. Final result: `qsl-protocol READY=1`, `qsl-server READY=0`, `qsl-attachments READY=0`.
- `printf 'qsl-protocol READY count: ' && grep -c '^Status: READY' NEXT_ACTIONS.md && ...` exited non-zero because `grep -c` still returns status `1` on zero matches even when printing `0`. Classified as a recoverable command-shape/tool-behavior mistake. Corrective action: replaced the count step with `awk` so zero-match repos remain success-path evidence. Final result: READY-count proof is now stable and reusable.

## Validation / CI notes
- Pre-mutation authority proof is complete: disk watermark green (`484 GiB` total / `221 GiB` used / `264 GiB` free / `46%` used), configured-remotes-only refresh completed for `qsl-protocol`, `qsl-server`, and `qsl-attachments`, and the active worktree was clean before branch creation.
- Refreshed main lacked the `NA-0236` implementation/evidence outputs at lane start: no KT canon-closure doc existed yet, no `NA-0236 implementation/evidence` trace entry existed yet, and the required testplan stub was absent.
- Policy review confirms this docs-only lane is satisfied by the authorized journal surface plus one matching testplan stub; no additional `docs/ops/**` path or extra docs-only testplan stub is required.
- First green local docs/governance validation bundle is complete on the staged/committed tree:
  - schema JSON validation: `python3 -m json.tool docs/schemas/DOC-SCL-002_Shared_Schemas_v1.0.json`
  - markdown inventory: `tests/*.md=60`, `tests/**/*.md=1`, `docs/*.md=236`, `docs/**/*.md=231`
  - manual markdown link-integrity runbook: `TOTAL_MISSING 0`
  - changed-path scope proof: `DECISIONS.md`, `TRACEABILITY.md`, `docs/canonical/DOC-CAN-008_QSP_Key_Transparency_Profile_and_Bundle_Signature_Closure_v0.1.0_DRAFT.md`, `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`, `docs/schemas/DOC-SCL-002_Shared_Schemas_v1.0.json`, `docs/spec-closure/DOC-SCL-001_Suite_Parameter_Registry_Deployment_Profiles_v1.0_DRAFT.md`, `docs/spec-closure/DOC-SCL-002_Shared_Schemas_Error_Reason_Code_Registry_v1.0_DRAFT.md`, `tests/NA-0236_kt_serialization_profile_bundle_signature_closure_testplan.md`
  - added-line leak-safe scan: `ADDED_LINE_COUNT 426`, `v1-path pattern count: 0`, `hex32plus pattern count: 0`, `auth-header pattern count: 0`, `bearer token pattern count: 0`
- Remaining at authoring time: commit this journal refresh, run local goal-lint against the final commit and intended PR body, push the branch, open exactly one PR, poll protected checks via bounded REST, merge with a merge commit, and re-prove refreshed-main READY truth without queue closeout.

## Disk watermark
- Filesystem: `/srv/qbuild`
- Total GiB: `484`
- Used GiB: `221`
- Free GiB: `264`
- Used %: `46%`

## Next-watch items
- Keep the KT closure limited to one primary canonical doc plus the minimum schema/spec-closure updates needed to freeze BundleLeafData, BundleTBS, proof serialization, pinning, freshness, and responder obligations.
- Preserve the no-closeout rule: `NEXT_ACTIONS.md` must remain untouched so refreshed `main` still shows `NA-0236` as the sole READY item after merge.
- Recheck changed-path scope before push and after PR creation to ensure no `.github/**`, runtime/source/test code, sibling repo, or extra `docs/ops/**` paths slipped into the lane.

# Rolling Operations Journal Entry

- Directive: `DIRECTIVE 312 — NA-0235 Workflow/Governance Repair Salvage from Refreshed Main`
- Begin timestamp (America/Chicago): 2026-04-17T23:11:59-05:00
- Begin timestamp (UTC): 2026-04-18T04:11:59Z
- End timestamp (America/Chicago): pending at authoring time
- End timestamp (UTC): pending at authoring time

## Repo SHAs
- qsl-protocol branch: `na-0235-pr-dependency-audit-fullsuite-governance`
- qsl-protocol HEAD: `pending refreshed-main salvage merge commit`
- qsl-protocol main: `569d21cfcb19`
- qsl-protocol origin/main: `569d21cfcb19`
- qsl-protocol mirror/main: `569d21cfcb19`
- qsl-server main: `0826ffa4d6f3`
- qsl-server origin/main: `0826ffa4d6f3`
- qsl-server mirror/main: `0826ffa4d6f3`
- qsl-attachments main: `1e1ae272a4cb`
- qsl-attachments origin/main: `1e1ae272a4cb`
- qsl-attachments mirror/main: `1e1ae272a4cb`

## READY proof
- READY_COUNT: `1`
- Sole READY item: `NA-0235 — PR Dependency-Audit Gate + Full-Suite Governance Repair`
- Proof source: refreshed `origin/main:NEXT_ACTIONS.md`

## Worktree / branch / PR
- Worktree path: `/srv/qbuild/work/NA-0235/qsl-protocol`
- Branch: `na-0235-pr-dependency-audit-fullsuite-governance`
- PR: `PR #695 https://github.com/QuantumShieldLabs/qsl-protocol/pull/695`
- Merge commit: `n/a`

## What changed
- Re-proved from refreshed live state that merged `NA-0235A` work on current `main` resolves the old dependency-health blocker and restores `NA-0235` as the sole READY item while PR `#695` remains OPEN on head `68a3a8081889`.
- Re-proved that salvaging PR `#695` in place is truthful because the local branch still matches the PR head and merging refreshed `main` into it creates conflicts only in `DECISIONS.md`, `TRACEABILITY.md`, and this journal file, all within the allowed governance scope.
- Began the in-place salvage merge from refreshed `main` so the runtime-free workflow/governance repair can be revalidated on current main without history rewrite or a superseding PR.

## Failures / recoveries
- `git merge origin/main` exited non-zero with content conflicts in `DECISIONS.md`, `TRACEABILITY.md`, and `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`. Classified as recoverable because the conflict surface was predicted in read-only proof and remained entirely within the allowed governance scope. Corrective action: take refreshed-main history as the baseline, renumber/update the `NA-0235` decision and traceability metadata, and continue the salvage merge in place. Final result: conflict resolution in progress on the truthful salvage branch.

## Validation / CI notes
- Pre-mutation authority proof completed again: disk watermark green, configured-remotes-only refresh completed for `qsl-protocol`, `qsl-server`, and `qsl-attachments`, `READY_COUNT=1` with `NA-0235` as the sole READY item on refreshed `origin/main`, `NA-0235A` already `DONE`, `qsl-server READY=0`, and `qsl-attachments READY=0`.
- Refreshed `main` still lacks the `NA-0235` repair itself: `.github/workflows/public-ci.yml` on `origin/main` remains the older `pull_request`-based workflow with no `pull_request_target` or `workflow_dispatch` support.
- Refreshed PR proof shows PR `#695` is `OPEN` on head `68a3a8081889`, `mergeable=CONFLICTING`, `mergeStateStatus=DIRTY`, its last required-context conclusions are green from the prior branch head, and its changed-path set remains limited to `.github/workflows/public-ci.yml`, `scripts/ci/public_safety_gate.py`, `DECISIONS.md`, `TRACEABILITY.md`, `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`, and `tests/NA-0235_rolling_journal_entry_testplan.md`.
- Remaining at authoring time: finish the refreshed-main salvage resolution, rerun the required local validation bundle on the final branch head, push immediately after the first green local bundle, poll required contexts on PR `#695`, and merge if the protected set is green or accepted-neutral and GitHub reports `MERGEABLE`.

## Disk watermark
- Filesystem: `/srv/qbuild`
- Total GiB: `484`
- Used GiB: `220`
- Free GiB: `264`
- Used %: `46%`

## Next-watch items
- Finish the refreshed-main salvage resolution and run the full required local validation bundle before push.
- Push the PR `#695` branch immediately after the first green local bundle; no force-push, no superseding PR unless in-place salvage ceases to be truthful.
- Poll required contexts only via bounded REST and merge PR `#695` with a standard merge commit only when GitHub accepts a normal merge.

# Rolling Operations Journal Entry

- Directive: `DIRECTIVE 310 — NA-0235A Paired Dependency Remediation Salvage (Phase A qsl-attachments macOS hotfix PR first, Phase B resume PR #702 in place)`
- Begin timestamp (America/Chicago): 2026-04-17T21:07:07-05:00
- Begin timestamp (UTC): 2026-04-18T02:07:07Z
- End timestamp (America/Chicago): pending at authoring time
- End timestamp (UTC): pending at authoring time

## Repo SHAs
- qsl-protocol branch: `na-0235a-protocol-dependency-unblock-v3`
- qsl-protocol HEAD: `4341cc0ec26a`
- qsl-protocol main: `e49d4b699fa9`
- qsl-protocol origin/main: `e49d4b699fa9`
- qsl-protocol mirror/main: `e49d4b699fa9`
- qsl-server main: `0826ffa4d6f3`
- qsl-server origin/main: `0826ffa4d6f3`
- qsl-server mirror/main: `0826ffa4d6f3`
- qsl-attachments branch: `na-0235a-qsl-attachments-macos-width-fix`
- qsl-attachments branch head: `c056fe3c4675`
- qsl-attachments main: `1e1ae272a4cb`
- qsl-attachments origin/main: `1e1ae272a4cb`
- qsl-attachments mirror/main: `pending refreshed mirror update at authoring time`

## READY proof
- READY_COUNT: `1`
- Sole READY item: `NA-0235A — Runtime Dependency Advisory Remediation for Public-Safety Unblock`
- Proof source: refreshed `NEXT_ACTIONS.md` on `main`

## Worktree / branch / PR
- qsl-protocol worktree path: `/srv/qbuild/work/NA-0235A/qsl-protocol`
- qsl-protocol branch: `na-0235a-protocol-dependency-unblock-v3`
- qsl-protocol PR: `#702`
- qsl-attachments worktree path: `/srv/qbuild/work/NA-0235A/qsl-attachments`
- qsl-attachments branch: `na-0235a-qsl-attachments-macos-width-fix`
- qsl-attachments PR: `#31`
- qsl-attachments merge commit: `1e1ae272a4cb`

## What changed
- Re-proved from refreshed live state that PR `#695` remains OPEN and blocked by failing `public-safety` / `advisories`, that qsl-attachments `main` still contains the earlier rand-core migration merge commit `a1a4c1269899`, and that PR `#702` remains OPEN and salvageable in place.
- Re-proved that the only new blocker beyond the already-open protocol remediation is the deterministic macOS width mismatch at `qsl-attachments/src/lib.rs:232`, where `stats.f_bavail.saturating_mul(stats.f_frsize)` fails to compile on macOS because the operands have different integer widths there.
- Applied the smallest truthful qsl-attachments hotfix: normalize the `statvfs` block-count width on Apple targets before multiplication without changing service/runtime semantics or touching Cargo metadata.
- Validated the qsl-attachments hotfix locally with `cargo fmt --check`, `cargo build --locked`, `cargo clippy --locked -- -D warnings`, `cargo test --locked`, and `cargo audit --deny warnings`, then pushed `na-0235a-qsl-attachments-macos-width-fix`, opened PR `#31`, proved that PR `#702` went fully green on the hotfix SHA, and merged PR `#31` as `1e1ae272a4cb`.
- Updated the already-open qsl-protocol lane first to the hotfix commit `c056fe3c4675`, then to the merged qsl-attachments commit `1e1ae272a4cb` so PR `#702` stays truthful and can be merged in place instead of superseded.

## Failures / recoveries
- `cargo clippy --locked -- -D warnings` on the first hotfix shape failed with `clippy::unnecessary_cast` because the raw `as u64` normalization is a no-op on Linux, where both `statvfs` fields already resolve to `u64`. Classified as an in-scope local lint failure with understood cause. Corrective action: replaced the unconditional cast with a portable typed conversion attempt. Final result: root cause isolated further but not yet fixed.
- `cargo clippy --locked -- -D warnings` on the second hotfix shape failed with `clippy::useless_conversion` because `.try_into()` is still a no-op on Linux for the same fields. Classified as an in-scope local lint failure with understood cause. Corrective action: narrowed the fix to the actual platform split by converting `f_bavail` only on Apple targets and leaving non-Apple builds unchanged. Final result: the qsl-attachments validation bundle went green.

## Validation / CI notes
- Pre-mutation authority proof completed again: disk watermark green, configured-remotes-only refresh completed for `qsl-protocol`, `qsl-server`, and `qsl-attachments`, `READY_COUNT=1` with `NA-0235A` as the sole READY item, `NA-0235` still `BLOCKED`, `qsl-server READY=0`, and `qsl-attachments READY=0`.
- Refreshed protocol-side truth confirms PR `#702` still contains the full dependency-remediation scope and first went fully green on the hotfix SHA before the qsl-attachments merge.
- qsl-attachments local hotfix validation passed before push: `cargo fmt --check`, `cargo build --locked`, `cargo clippy --locked -- -D warnings`, `cargo test --locked`, `cargo audit --deny warnings`.
- Local Darwin-target compile proof was not possible on qbuild because only `x86_64-unknown-linux-gnu` is installed. The authoritative cross-platform proof for this hotfix remains downstream macOS CI on the updated protocol PR.
- Remaining at authoring time: rerun the qsl-protocol local validation bundle on the merged-commit truth update, push PR `#702` in place again, poll required contexts on that final head, and merge PR `#702` with a merge commit once the required set is green.

## Disk watermark
- Filesystem: `/srv/qbuild`
- Total GiB: `484`
- Used GiB: `218`
- Free GiB: `266`
- Used %: `46%`

## Next-watch items
- Finish the qsl-protocol salvage update and rerun the full required local validation bundle before waiting on long CI.
- Keep PR `#702` as the sole protocol salvage target: no supersede, no force-push, no history rewrite.
- Merge qsl-attachments PR `#31` first once the updated PR `#702` required set is green on the hotfix commit, then refresh PR `#702` to merged-commit truth if needed before final merge.

# Rolling Operations Journal Entry

- Directive: `DIRECTIVE 308 — NA-0235A Scope Repair for qsl-attachments Lockfile Remediation`
- Begin timestamp (America/Chicago): 2026-04-16T21:20:26-05:00
- Begin timestamp (UTC): 2026-04-17T02:20:26Z
- End timestamp (America/Chicago): pending at authoring time
- End timestamp (UTC): pending at authoring time

## Repo SHAs
- qsl-protocol branch: `na-0235a-scope-repair-qsl-attachments-lockfile`
- qsl-protocol HEAD: `pending governance scope-repair v5 commit`
- qsl-protocol main: `ab47e89bb987`
- qsl-protocol origin/main: `ab47e89bb987`
- qsl-protocol mirror/main: `ab47e89bb987`
- qsl-server main: `0826ffa4d6f3`
- qsl-server origin/main: `0826ffa4d6f3`
- qsl-server mirror/main: `0826ffa4d6f3`
- qsl-attachments main: `e94107ac094d`
- qsl-attachments origin/main: `e94107ac094d`
- qsl-attachments mirror/main: `e94107ac094d`

## READY proof
- READY_COUNT: `1`
- Sole READY item: `NA-0235A — Runtime Dependency Advisory Remediation for Public-Safety Unblock`
- Proof source: refreshed `NEXT_ACTIONS.md` on `main`

## Worktree / branch / PR
- Worktree path: `/srv/qbuild/work/NA-0235A/qsl-protocol`
- Branch: `na-0235a-scope-repair-qsl-attachments-lockfile`
- PR: `pending`
- Merge commit: `n/a`

## What changed
- Re-proved from refreshed live state that PR `#695` remains OPEN on head `68a3a8081889`, that `advisories` and `public-safety` still fail, and that the queue blocker remains live dependency health rather than stale workflow wiring.
- Re-proved that the current `NA-0235A` block still omits one concrete path the next implementation lane requires: `qsl-attachments/Cargo.lock`.
- Re-proved that `/srv/qbuild/work/NA-0235A/qsl-attachments/Cargo.toml` still pins `rand = "0.8"`, `/srv/qbuild/work/NA-0235A/qsl-attachments/Cargo.lock` still records that dependency in the root package dependency list, and the smallest truthful Phase A migration therefore invalidates the checked-in qsl-attachments lockfile while the directive itself still requires locked validation.
- Re-proved that the cross-repo `qsl-attachments` harness path and active refimpl runtime `rand 0.8` API usage remain real blockers, while the earlier TUI-stack theory is still non-blocking.
- Added governance-only scope-repair artifacts so `NA-0235A` stays the sole READY item while its Problem and Scope text now authorize the final missing `qsl-attachments/Cargo.lock` surface needed for the paired implementation set.

## Failures / recoveries
- None so far at authoring time.

## Validation / CI notes
- Pre-mutation authority proof completed again: disk watermark green, configured-remotes-only refresh completed for `qsl-protocol`, `qsl-server`, and `qsl-attachments`, `READY_COUNT=1` with `NA-0235A` as the sole READY item, `NA-0235` still `BLOCKED`, `qsl-server READY=0`, and `qsl-attachments READY=0`.
- Refreshed contradiction proof confirms PR `#695` still carries failing `advisories` and `public-safety`, `qsl/qsl-client/qsc/Cargo.toml:32` still pulls `qsl-attachments`, `qsl/qsl-client/qsc/tests/common/mod.rs:5` still imports the harness, `/srv/qbuild/work/NA-0235A/qsl-attachments/Cargo.toml:8` still pins `rand = "0.8"`, `/srv/qbuild/work/NA-0235A/qsl-attachments/Cargo.lock` still records `rand` in the root package dependency list, and the refimpl runtime source still carries the old `rand` API pattern.
- Remaining at authoring time: finish the docs-only validation bundle on the committed branch head, then push, open one governance-only PR, poll protected contexts, merge, refresh `main`, and re-prove sole READY `NA-0235A`.

## Disk watermark
- Filesystem: `/srv/qbuild`
- Total GiB: `484`
- Used GiB: `214`
- Free GiB: `270`
- Used %: `45%`

## Next-watch items
- Finish the docs-only validation bundle on the final scope-repair v5 tree, then push the governance branch immediately.
- Open exactly one governance-only PR, poll protected contexts only via bounded REST, and merge with a merge commit once the required set is green.
- After merge, refresh `main` again and re-prove `NA-0235A` is still the sole READY item, its repaired scope text is present, the journal entry is present on `main`, and the workspace is clean.

# Rolling Operations Journal Entry

- Directive: `DIRECTIVE 306 — NA-0235A Scope Repair for Refimpl Runtime rand Remediation`
- Begin timestamp (America/Chicago): 2026-04-16T17:11:44-05:00
- Begin timestamp (UTC): 2026-04-16T22:11:44Z
- End timestamp (America/Chicago): pending at authoring time
- End timestamp (UTC): pending at authoring time

## Repo SHAs
- qsl-protocol branch: `na-0235a-scope-repair-refimpl-rand`
- qsl-protocol HEAD: `pending governance scope-repair v4 commit`
- qsl-protocol main: `8421616b4a2b`
- qsl-protocol origin/main: `8421616b4a2b`
- qsl-protocol mirror/main: `8421616b4a2b`
- qsl-server main: `0826ffa4d6f3`
- qsl-server origin/main: `0826ffa4d6f3`
- qsl-server mirror/main: `0826ffa4d6f3`
- qsl-attachments main: `e94107ac094d`
- qsl-attachments origin/main: `e94107ac094d`
- qsl-attachments mirror/main: `e94107ac094d`

## READY proof
- READY_COUNT: `1`
- Sole READY item: `NA-0235A — Runtime Dependency Advisory Remediation for Public-Safety Unblock`
- Proof source: refreshed `NEXT_ACTIONS.md` on `main`

## Worktree / branch / PR
- Worktree path: `/srv/qbuild/work/NA-0235A/qsl-protocol`
- Branch: `na-0235a-scope-repair-refimpl-rand`
- PR: `pending`
- Merge commit: `n/a`

## What changed
- Re-proved from refreshed live state that PR `#695` remains OPEN on head `68a3a8081889`, that `advisories` and `public-safety` still fail, and that the queue blocker remains live dependency health rather than stale workflow wiring.
- Re-proved that the current `NA-0235A` block still understates the remaining active blocker: the cross-repo `qsl-attachments` harness path is still live, but active refimpl runtime source also still imports `rand 0.8` and uses `OsRng.fill_bytes(...)` in `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs`, with the same old API pattern still present in `tools/refimpl/quantumshield_refimpl/src/qsp/mod.rs` and `tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs`.
- Re-proved that the earlier TUI-stack theory is no longer the active blocker because inverse-tree proof for `ratatui-termwiz`, `termwiz`, and `phf_generator` still prints nothing, while the direct `apps/qsl-tui` pin cleanup plus `rustls-webpki` and `rand 0.9.2` bumps remain useful but insufficient alone.
- Added governance-only scope-repair artifacts so `NA-0235A` stays the sole READY item while its Problem and Scope text now authorize the minimal refimpl runtime source/API compatibility seam in addition to the already-proven cross-repo `qsl-attachments` dependency-fix surface.

## Failures / recoveries
- None so far at authoring time.

## Validation / CI notes
- Pre-mutation authority proof completed again: disk watermark green, configured-remotes-only refresh completed for `qsl-protocol`, `qsl-server`, and `qsl-attachments`, `READY_COUNT=1` with `NA-0235A` as the sole READY item, `NA-0235` still `BLOCKED`, `qsl-server READY=0`, and `qsl-attachments READY=0`.
- Refreshed contradiction proof confirms PR `#695` remains open and blocked, `qsl/qsl-client/qsc/Cargo.toml:32` still pulls `qsl-attachments`, `qsl/qsl-client/qsc/tests/common/mod.rs:5` still imports the harness, `qsl-attachments/Cargo.toml:14` still pins `rand = "0.8"`, and the active refimpl source tree still uses `rand::{rngs::OsRng, RngCore}` plus `OsRng.fill_bytes(...)` callsites in runtime code.
- The temp compatibility proof again shows `rand 0.9` is not source-compatible with the current `OsRng.fill_bytes` usage without source edits: a minimal compile against `rand 0.9.4` fails with `E0599` because `OsRng` no longer satisfies `RngCore`.
- Remaining at authoring time: finish the docs-only validation bundle on the committed branch head, then push, open one governance-only PR, poll protected contexts, merge, refresh `main`, and re-prove sole READY `NA-0235A`.

## Disk watermark
- Filesystem: `/srv/qbuild`
- Total GiB: `484`
- Used GiB: `214`
- Free GiB: `270`
- Used %: `45%`

## Next-watch items
- Finish the docs-only validation bundle on the final scope-repair v4 tree, then push the governance branch immediately.
- Open exactly one governance-only PR, poll protected contexts only via bounded REST, and merge with a merge commit once the required set is green.
- After merge, refresh `main` again and re-prove `NA-0235A` is still the sole READY item, its repaired scope text is present, the journal entry is present on `main`, and the workspace is clean.

- Directive: `DIRECTIVE 304 — NA-0235A Scope Repair for Cross-Repo qsl-attachments Dependency Remediation`
- Begin timestamp (America/Chicago): 2026-04-16T13:41:41-05:00
- Begin timestamp (UTC): 2026-04-16T18:41:41Z
- End timestamp (America/Chicago): pending at authoring time
- End timestamp (UTC): pending at authoring time

## Repo SHAs
- qsl-protocol branch: `na-0235a-scope-repair-qsl-attachments-harness`
- qsl-protocol HEAD: `pending governance scope-repair v3 commit`
- qsl-protocol main: `7308805edbb8`
- qsl-protocol origin/main: `7308805edbb8`
- qsl-protocol mirror/main: `7308805edbb8`
- qsl-server main: `0826ffa4d6f3`
- qsl-server origin/main: `0826ffa4d6f3`
- qsl-server mirror/main: `0826ffa4d6f3`
- qsl-attachments main: `e94107ac094d`
- qsl-attachments origin/main: `e94107ac094d`
- qsl-attachments mirror/main: `e94107ac094d`

## READY proof
- READY_COUNT: `1`
- Sole READY item: `NA-0235A — Runtime Dependency Advisory Remediation for Public-Safety Unblock`
- Proof source: refreshed `NEXT_ACTIONS.md` on `main`

## Worktree / branch / PR
- Worktree path: `/srv/qbuild/work/NA-0235A/qsl-protocol`
- Branch: `na-0235a-scope-repair-qsl-attachments-harness`
- PR: `pending`
- Merge commit: `n/a`

## What changed
- Re-proved from refreshed live state that PR `#695` remains OPEN on head `68a3a8081889`, that `advisories` and `public-safety` still fail, and that the queue blocker remains live dependency health rather than stale workflow wiring.
- Re-proved that the current `NA-0235A` block still points at the wrong remaining blocker: the active path is the cross-repo `qsl-attachments` test harness because `qsl/qsl-client/qsc/Cargo.toml` still pulls that git dependency, `qsl/qsl-client/qsc/tests/common/mod.rs` still imports it, and `qsl-attachments/Cargo.toml` still pins `rand = "0.8"`.
- Re-proved that the earlier `ratatui -> ratatui-termwiz -> termwiz -> terminfo -> phf_generator` chain is no longer the active blocker, while the direct `apps/qsl-tui` pin cleanup and `rustls-webpki` / `rand 0.9.2` bumps remain useful but insufficient alone.
- Added governance-only scope-repair artifacts so `NA-0235A` stays the sole READY item while its Problem and Scope text now authorize the qsl-protocol rev/update seam plus the minimal cross-repo `qsl-attachments` dependency-fix surface and paired implementation note identified by refreshed contradiction proof.

## Failures / recoveries
- None so far at authoring time.

## Validation / CI notes
- Pre-mutation authority proof completed again: disk watermark green, configured-remotes-only refresh completed for `qsl-protocol`, `qsl-server`, and `qsl-attachments`, `READY_COUNT=1` with `NA-0235A` as the sole READY item, `NA-0235` still `BLOCKED`, `qsl-server READY=0`, and `qsl-attachments READY=0`.
- Refreshed contradiction proof confirms `qsl/qsl-client/qsc/Cargo.toml:32` still pulls `qsl-attachments`, `qsl/qsl-client/qsc/tests/common/mod.rs:5` still imports the harness, `qsl-attachments/Cargo.toml:14` still pins `rand = "0.8"`, and the inverse trees for `ratatui-termwiz`, `termwiz`, and `phf_generator` now print nothing.
- Completed local validation so far on the branch tree: markdown inventory counts (`tests/*.md=52`, `tests/**/*.md=1`, `docs/*.md=230`, `docs/**/*.md=225`), the manual markdown link-integrity runbook (`TOTAL_MISSING 0`), and the added-line leak-safe scan (`ADDED_LINE_COUNT 86`, `v1-path pattern count: 0`, `hex32plus pattern count: 0`, `secret-like marker count: 0`).
- Remaining at authoring time: rerun local goal-lint once on the committed branch head so the synthetic event reflects the actual branch diff, then branch push, PR creation, protected-check polling, merge, refreshed-main proof, and final evidence capture.

## Disk watermark
- Filesystem: `/srv/qbuild`
- Total GiB: `484`
- Used GiB: `214`
- Free GiB: `270`
- Used %: `45%`

## Next-watch items
- Finish the docs-only validation bundle on the final scope-repair v3 tree, then push the governance branch immediately.
- Open exactly one governance-only PR, poll protected contexts only via bounded REST, and merge with a merge commit once the required set is green.
- After merge, refresh `main` again and re-prove `NA-0235A` is still the sole READY item, its repaired scope text is present, the journal entry is present on `main`, and the workspace is clean.

- Directive: `DIRECTIVE 302 — NA-0235A Scope Repair for TUI Dependency-Stack Remediation`
- Begin timestamp (America/Chicago): 2026-04-16T11:04:15-05:00
- Begin timestamp (UTC): 2026-04-16T16:04:15Z
- End timestamp (America/Chicago): pending at authoring time
- End timestamp (UTC): pending at authoring time

## Repo SHAs
- qsl-protocol branch: `na-0235a-scope-repair-tui-dependency-stack`
- qsl-protocol HEAD: `pending governance scope-repair v2 commit`
- qsl-protocol main: `efa8458fe8b3`
- qsl-protocol origin/main: `efa8458fe8b3`
- qsl-protocol mirror/main: `efa8458fe8b3`
- qsl-server main: `0826ffa4d6f3`
- qsl-server origin/main: `0826ffa4d6f3`
- qsl-server mirror/main: `0826ffa4d6f3`
- qsl-attachments main: `e94107ac094d`
- qsl-attachments origin/main: `e94107ac094d`
- qsl-attachments mirror/main: `e94107ac094d`

## READY proof
- READY_COUNT: `1`
- Sole READY item: `NA-0235A — Runtime Dependency Advisory Remediation for Public-Safety Unblock`
- Proof source: refreshed `NEXT_ACTIONS.md` on `main`

## Worktree / branch / PR
- Worktree path: `/srv/qbuild/work/NA-0235A/qsl-protocol`
- Branch: `na-0235a-scope-repair-tui-dependency-stack`
- PR: `pending`
- Merge commit: `n/a`

## What changed
- Re-proved from refreshed live state that PR `#695` remains OPEN on head `68a3a8081889`, that `advisories` and `public-safety` still fail, and that the queue blocker is still live dependency health rather than stale workflow wiring.
- Re-proved that the current `NA-0235A` block already includes `apps/qsl-tui/Cargo.toml` plus bounded `apps/qsl-tui/src/**` fallout, but still understates the remaining stale `rand 0.8.5` lock path carried by `ratatui -> ratatui-termwiz -> termwiz -> terminfo -> phf_generator`.
- Added governance-only scope-repair artifacts so `NA-0235A` stays the sole READY item while its Problem and Scope text now authorize the bounded TUI dependency-stack replacement surfaces identified by refreshed contradiction proof.

## Failures / recoveries
- `rg -n "use rand::|rand::|thread_rng|rng\\(" apps/qsl-tui/src -g '*.rs'` -> recoverable because a zero-match result is valid contradiction proof in this lane; corrected by recording the zero-match as evidence rather than treating it as an implementation failure; final result: refreshed `main` still shows zero local rand callsites under `apps/qsl-tui/src/**`.
- `GITHUB_EVENT_PATH="$tmp" python3 tools/goal_lint.py` with an initial synthetic event payload lacking `pull_request.base.sha` and `pull_request.head.sha` -> recoverable because this was a command-shape mistake in the local docs-only validation harness rather than a repo defect; corrective action: record the failure, then rerun goal-lint once on the committed branch head with explicit base/head SHAs in the synthetic event; final result: pending at authoring time.

## Validation / CI notes
- Pre-mutation authority proof completed again: disk watermark green, configured-remotes-only refresh completed for `qsl-protocol`, `qsl-server`, and `qsl-attachments`, `READY_COUNT=1` with `NA-0235A` as the sole READY item, `NA-0235` still `BLOCKED`, `qsl-server READY=0`, and `qsl-attachments READY=0`.
- Refreshed contradiction proof confirms PR `#695` remains open and blocked, the remaining stale `rand 0.8.5` lock path is still carried by the current `ratatui` / `termwiz` chain, and the additional bounded remediation surface is `apps/qsl-tui/src/main.rs`, `qsl/qsl-client/qsc/src/main.rs`, and `qsl/qsl-client/qsc/src/tui/**`.
- Completed local validation so far on the branch tree: markdown inventory counts (`tests/*.md=51`, `tests/**/*.md=1`, `docs/*.md=229`, `docs/**/*.md=224`), the manual markdown link-integrity runbook (`TOTAL_MISSING 0`), and the added-line leak-safe scan (`ADDED_LINE_COUNT 79`, `v1-path pattern count: 0`, `hex32plus pattern count: 0`, `secret-like marker count: 0`).
- Remaining at authoring time: rerun local goal-lint once on the committed branch head with explicit base/head SHAs, then branch push, PR creation, protected-check polling, merge, refreshed-main proof, and final evidence capture.

## Disk watermark
- Filesystem: `/srv/qbuild`
- Total GiB: `484`
- Used GiB: `214`
- Free GiB: `270`
- Used %: `45%`

## Next-watch items
- Finish the docs-only validation bundle on the final scope-repair v2 tree, then push the governance branch immediately.
- Open exactly one governance-only PR, poll protected contexts only via bounded REST, and merge with a merge commit once the required set is green.
- After merge, refresh `main` again and re-prove `NA-0235A` is still the sole READY item, its repaired scope text is present, the journal entry is present on `main`, and the workspace is clean.

# Rolling Operations Journal Entry

- Directive: `DIRECTIVE 300 — NA-0235A Scope Repair for Dependency Remediation`
- Begin timestamp (America/Chicago): 2026-04-16T09:12:39-05:00
- Begin timestamp (UTC): 2026-04-16T14:12:39Z
- End timestamp (America/Chicago): pending at authoring time
- End timestamp (UTC): pending at authoring time

## Repo SHAs
- qsl-protocol branch: `na-0235a-scope-repair-dependency-manifest`
- qsl-protocol HEAD: `pending governance scope-repair commit`
- qsl-protocol main: `db4457325aeb`
- qsl-protocol origin/main: `db4457325aeb`
- qsl-protocol mirror/main: `db4457325aeb`
- qsl-server main: `0826ffa4d6f3`
- qsl-server origin/main: `0826ffa4d6f3`
- qsl-server mirror/main: `0826ffa4d6f3`
- qsl-attachments main: `e94107ac094d`
- qsl-attachments origin/main: `e94107ac094d`
- qsl-attachments mirror/main: `e94107ac094d`

## READY proof
- READY_COUNT: `1`
- Sole READY item: `NA-0235A — Runtime Dependency Advisory Remediation for Public-Safety Unblock`
- Proof source: refreshed `NEXT_ACTIONS.md` on `main`

## Worktree / branch / PR
- Worktree path: `/srv/qbuild/work/NA-0235A/qsl-protocol`
- Branch: `na-0235a-scope-repair-dependency-manifest`
- PR: `pending`
- Merge commit: `n/a`

## What changed
- Re-proved from refreshed live state that PR `#695` remains OPEN on head `68a3a8081889`, that `public-safety` still fails because `advisories` fails, and that the queue blocker is real rather than a stale workflow or governance artifact.
- Re-proved that the current `NA-0235A` scope understated the real bounded dependency surface because `apps/qsl-tui/Cargo.toml` still directly pins `rand = "0.8"` while `apps/qsl-tui/src/**` shows zero local rand callsites on refreshed `main`.
- Added governance-only scope-repair artifacts so `NA-0235A` stays the sole READY item while its Problem and Scope text now authorize the real blocking manifest surface without widening into runtime or workflow changes.

## Failures / recoveries
- `rg -n "use rand::|rand::|thread_rng|rng\\(" apps/qsl-tui/src` -> recoverable because a zero-match result is valid contradiction proof in this lane; corrected by recording the zero-match as evidence rather than treating it as an implementation failure; final result: refreshed `main` shows zero local rand callsites under `apps/qsl-tui/src/**`.
- `sed -n '1,220p' docs/archive/testplans/NA-0230_closeout_evidence_testplan.md` -> recoverable because the example requested during format review lives under `tests/` rather than `docs/archive/testplans/`; corrected by reusing the existing `NA-0233` scope-repair archive and testplan patterns already present on refreshed `main`; final result: no additional path discovery was needed before patching this governance lane.

## Validation / CI notes
- Pre-mutation authority proof completed again: disk watermark green, configured-remotes-only refresh completed for `qsl-protocol`, `qsl-server`, and `qsl-attachments`, `READY_COUNT=1` with `NA-0235A` as the sole READY item, `NA-0235` still `BLOCKED`, `qsl-server READY=0`, and `qsl-attachments READY=0`.
- Refreshed contradiction proof confirms `cargo update -p rustls-webpki --precise 0.103.12 --dry-run` and `cargo update -p rand@0.9.2 --precise 0.9.3 --dry-run` succeed, while `cargo update -p rand@0.8.5 --precise 0.9.3 --dry-run` still fails on a live `^0.8` requirement, so another implementation attempt would remain untruthful without scope repair.
- Completed local validation so far on the branch tree: markdown inventory counts (`tests/*.md=50`, `tests/**/*.md=1`, `docs/*.md=228`, `docs/**/*.md=223`), the manual markdown link-integrity runbook (`TOTAL_MISSING 0`), and the added-line leak-safe scan (`ADDED_LINE_COUNT 77`, `v1-path pattern count: 0`, `hex32plus pattern count: 0`, `secret-like marker count: 0`).
- Remaining at authoring time: local goal-lint on the committed branch head, branch push, PR creation, protected-check polling, merge, refreshed-main proof, and final evidence capture.

## Disk watermark
- Filesystem: `/srv/qbuild`
- Total GiB: `484`
- Used GiB: `214`
- Free GiB: `271`
- Used %: `45%`

## Next-watch items
- Finish the docs-only validation bundle on the final scope-repair tree, then push the governance branch immediately.
- Open exactly one governance-only PR, poll protected contexts only via bounded REST, and merge with a merge commit once the required set is green.
- After merge, refresh `main` again and re-prove `NA-0235A` is still the sole READY item, its repaired scope text is present, the journal entry is present on `main`, and the workspace is clean.

# Rolling Operations Journal Entry

- Directive: `DIRECTIVE 296 — NA-0235 Queue-Truth Repair / Dependency-Unblock Successor Promotion`
- Begin timestamp (America/Chicago): 2026-04-15T21:18:40-05:00
- Begin timestamp (UTC): 2026-04-16T02:18:40Z
- End timestamp (America/Chicago): pending at authoring time
- End timestamp (UTC): pending at authoring time

## Repo SHAs
- qsl-protocol branch: `na-0235-blocked-on-dependencies-repair`
- qsl-protocol HEAD: `pending governance queue-repair commit`
- qsl-protocol main: `fd4400406d80`
- qsl-protocol origin/main: `fd4400406d80`
- qsl-protocol mirror/main: `fd4400406d80`
- qsl-server main: `0826ffa4d6f3`
- qsl-server origin/main: `0826ffa4d6f3`
- qsl-server mirror/main: `0826ffa4d6f3`
- qsl-attachments main: `e94107ac094d`
- qsl-attachments origin/main: `e94107ac094d`
- qsl-attachments mirror/main: `e94107ac094d`

## READY proof
- READY_COUNT: `1`
- Sole READY item: `NA-0235 — PR Dependency-Audit Gate + Full-Suite Governance Repair`
- Proof source: refreshed `NEXT_ACTIONS.md` on `main`

## Worktree / branch / PR
- Worktree path: `/srv/qbuild/work/NA-0235/qsl-protocol`
- Branch: `na-0235-blocked-on-dependencies-repair`
- PR: `pending`
- Merge commit: `n/a`

## What changed
- Re-proved from refreshed live state that PR `#695` remains OPEN on head `68a3a8081889`, that the sanctioned `public-safety` bootstrap now attaches truthfully, and that `public-safety` fails because `advisories` fails on live RustSec findings while the rest of the protected required set is green.
- Re-proved that current `main` still lacks the `NA-0235` workflow/governance repair because refreshed `main` still carries the older `pull_request`-based `public-ci` definition from before PR `#695`.
- Added governance-only queue-repair artifacts to mark `NA-0235` `BLOCKED`, promote `NA-0235A` as the sole `READY` successor, and record the dependency-unblock rationale without changing runtime code, workflows, branch protection, or PR `#695`.

## Failures / recoveries
- None at authoring time.

## Validation / CI notes
- Pre-mutation authority proof completed again: disk watermark green, configured-remotes-only refresh completed for `qsl-protocol`, `qsl-server`, and `qsl-attachments`, `READY_COUNT=1` with `NA-0235` as the sole READY item, `NA-0234` already `DONE` on refreshed `main`, `qsl-server READY=0`, and `qsl-attachments READY=0`.
- Refreshed blocker proof shows the workflow/governance repair itself is now functioning: `public-safety` is attached on PR `#695`, the job fails only because `advisories` fails, and the rest of the protected set is green.
- Remaining at authoring time: docs-only validation bundle, branch push, PR creation, protected-check polling, merge, refreshed-main proof, and final journal end-state update.

## Disk watermark
- Filesystem: `/srv/qbuild`
- Total GiB: `484`
- Used GiB: `214`
- Free GiB: `271`
- Used %: `45%`

## Next-watch items
- Finish the docs-only validation bundle on the final queue-repair tree, then push the governance branch immediately.
- Open exactly one governance-only PR, poll protected contexts only via bounded REST, and merge with a merge commit once the protected set is green.
- After merge, refresh `main` again and re-prove `NA-0235` is `BLOCKED`, `NA-0235A` is sole `READY`, this journal entry is present on `main`, and PR `#695` remains open and untouched.

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

# Rolling Operations Journal Entry

- Directive: `DIRECTIVE 282 — NA-0233 MockProvider Fixed Vault Key Resolution`
- Begin timestamp (America/Chicago): 2026-04-10T19:11:24-05:00
- Begin timestamp (UTC): 2026-04-11T00:11:24Z
- End timestamp (America/Chicago): pending at authoring time
- End timestamp (UTC): pending at authoring time

## Repo SHAs
- qsl-protocol branch: `na-0233-mockprovider-fixed-key-resolution`
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
- Branch: `na-0233-mockprovider-fixed-key-resolution`
- PR: `#688`
- Merge commit: `n/a`

## What changed
- Retired the production/shared MockProvider path in `qsl/qsl-client/qsc/src/vault/mod.rs`: `mock` key-source selection now fails with deterministic `vault_mock_provider_retired`, fixed-key derivation for key-source tag `4` is removed from runtime, and status now surfaces existing tag `4` envelopes as `mock_retired`.
- Removed the shipped/shared MockProvider auto-unlock reachability from `qsl/qsl-client/qsc/src/main.rs` bootstrap and `qsl/qsl-client/qsc/src/tui/controller/commands/dispatch.rs` unlock handling.
- Reworked directly affected qsc integration-test mock-vault helpers to use a passphrase-backed test harness with explicit desktop compatibility unlock env/argv wiring, and added regressions proving retired MockProvider init rejects without mutation while existing key-source `4` envelopes fail closed.

## Failures / recoveries
- `cargo test --test qsp_protocol_gate` -> recoverable because the first pass of the test harness used a retired env name for explicit unlock; corrected by switching the shared helper to the allowed `QSC_DESKTOP_SESSION_PASSPHRASE` desktop compatibility env and rerunning the test; final result: green.
- `cargo fmt --check` -> recoverable because the initial implementation left formatting drift in the touched qsc test files; corrected by running `cargo fmt` and rerunning `cargo fmt --check`; final result: green.
- `cargo test --test send_semantics` and `cargo test --test receipts_delivered` -> recoverable because directly affected mock-vault consumers still spawned `qsc` without the new explicit unlock args after `common::init_mock_vault()` moved to a passphrase-backed vault; corrected by adding shared `qsc_std_command` / `qsc_assert_command` helpers and updating the directly affected consumer tests; final result: green on rerun.
- `cargo test --tests --no-run` -> recoverable because the first broad touched-test compile sweep still contained helper-type mismatches after the command-constructor conversion; corrected by removing duplicate unlock-helper calls, aligning helper return types, and rerunning the compile sweep; final result: green.
- Protected CI `macos-qsc-qshield-build` on PR `#688` -> recoverable because `qsl/qsl-client/qsc/tests/cli.rs` still initialized `vault init --key-source mock`, which the runtime now rejects fail-closed; corrected by moving that test to the shared passphrase-backed `common::init_mock_vault()` plus explicit unlock-aware `common::qsc_assert_command()`, then rerunning `cargo test --test cli`; final result: local rerun green and PR head updated for CI.
- Protected CI `macos-qsc-qshield-build` retry on PR `#688` -> recoverable because `qsl/qsl-client/qsc/tests/identity_secret_at_rest.rs` still expected `vault_locked` for a no-vault migration path even though the shared mock-vault helper now injects explicit unlock args and the fail-closed runtime marker is `vault_missing`; corrected by updating that assertion and rerunning `cargo test --test identity_secret_at_rest`; final result: local rerun green and PR head updated for CI.

## Validation / CI notes
- Local validation now includes the required runtime regressions (`vault`, `qsp_protocol_gate`, handshake/identity/state canaries) plus a broad compile-only sweep of qsc test targets and a grouped rerun of the directly touched mock-vault consumer binaries.
- Docs/governance validation, goal-lint, and the first implementation push already completed; PR `#688` is open and a follow-up in-scope test-harness fix is being pushed after the protected macOS job exposed one remaining direct mock-vault consumer.
- This lane is implementation/evidence only; queue closeout remains out of scope.

## Disk watermark
- Filesystem: `/srv/qbuild`
- Total GiB: `484`
- Used GiB: `197`
- Free GiB: `287`
- Used %: `41%`

## Next-watch items
- Finish the remaining local validation bundle, then push immediately after the first fully green local bundle so the implementation state is not left only on qbuild.

# Rolling Operations Journal Entry

- Directive: `DIRECTIVE 283 — NA-0233 PR #688 Salvage / CI Resolution / Merge-or-Stop`
- Begin timestamp (America/Chicago): 2026-04-10T22:57:49-05:00
- Begin timestamp (UTC): 2026-04-11T03:57:49Z
- End timestamp (America/Chicago): pending at authoring time
- End timestamp (UTC): pending at authoring time

## Repo SHAs
- qsl-protocol branch: `na-0233-mockprovider-fixed-key-resolution`
- qsl-protocol HEAD: `4bb7a1c1b141`
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
- Branch: `na-0233-mockprovider-fixed-key-resolution`
- PR: `#688`
- Merge commit: `n/a`

## What changed
- Revalidated that PR `#688` is still open, mergeable in place, and the local salvage branch head still matches the PR head SHA.
- Confirmed the cancelled `macos-qsc-qshield-build` run timed out at the workflow’s 45-minute cap without a concrete assertion failure.
- Reproduced the concrete `ci-4a` failure locally and repaired `qsl/qsl-client/qsc/tests/meta_min.rs` so `pad_invalid_rejects_no_mutation` initializes the passphrase-backed test vault before asserting the fail-closed `meta_pad_invalid` marker.
- Repaired `qsl/qsl-client/qsc/tests/qsp_protocol_gate.rs` so the directly affected protocol-gate tests rely on the shared helper’s explicit unlock args once, instead of passing duplicate `--unlock-passphrase-env` flags that masked the real protocol-gate assertions.
- Reproduced the exact serial macOS workflow command locally and repaired `qsl/qsl-client/qsc/tests/meta_phase2.rs` so vault-free `meta plan` coverage uses a plain `qsc` command instead of the unlock-aware helper, which now fails closed with `vault_locked`.
- Split `qsl/qsl-client/qsc/tests/meta_phase2.rs` command helpers so vault-free `meta plan` cases stay plain while `contacts_route_set` keeps the explicit unlock-aware helper required for vault-backed route-token writes.

## Failures / recoveries
- `cargo test --manifest-path qsl/qsl-client/qsc/Cargo.toml --locked --test meta_min pad_invalid_rejects_no_mutation -- --exact --nocapture` -> recoverable because the test still assumed the pre-fix implicit MockProvider unlock path and failed early with `vault_missing`; corrected by initializing the passphrase-backed test vault before the send attempt and rerunning the focused test; final result: green.
- `cargo test --locked --test qsp_protocol_gate` -> recoverable because several directly affected protocol-gate tests still appended `--unlock-passphrase-env` on top of `common::qsc_std_command()`, causing an argument-shape failure instead of the intended protocol-gate assertions; corrected by removing the duplicate unlock args and rerunning the affected validation bundle; final result: pending at authoring time.
- `TMPDIR=\"$PWD/.tmp\" cargo +stable test -p qsc --locked --jobs 1 -- --test-threads=1` -> recoverable because the exact serial workflow repro exposed an additional in-scope `meta_phase2` failure: vault-free `meta plan` tests were still using the unlock-aware helper and therefore tripped `vault_locked` before the intended metadata assertions; corrected by switching those `meta_phase2` cases to a plain `qsc` command helper and rerunning the affected validation bundle; final result: pending at authoring time.
- `cargo test --locked --test meta_phase2` rerun after the first helper split -> recoverable because `contacts_route_set` in the same file legitimately needs explicit unlock args for vault-backed route-token writes, so the first plain-helper sweep overcorrected that call site; corrected by introducing a dedicated unlock-aware helper for that route-token path and rerunning the affected validation bundle; final result: pending at authoring time.
- Required context `ci-4a` on PR `#688` -> recoverable because the job logs exposed the same focused `meta_min` assertion failure in an in-scope test surface; corrected with the targeted `meta_min` test setup fix and slated for rerun within directive budget; final result: pending at authoring time.
- Required context `macos-qsc-qshield-build` on PR `#688` -> recoverable because job logs showed timeout cancellation without a concrete code failure; corrected by capturing the exact workflow command shape for local reproduction before any rerun; final result: pending at authoring time.

## Validation / CI notes
- Salvage validation is intentionally narrowed to the focused `meta_min` repro/fix, the exact workflow command reproduction for the cancelled macOS lane, and any broader in-scope reruns that become necessary after the new patch lands.
- The existing implementation/evidence scope, journal companion, and markdown stub remain the only governance artifacts for this PR; no new queue edits or archive docs are introduced in the salvage lane.

## Disk watermark
- Filesystem: `/srv/qbuild`
- Total GiB: `483`
- Used GiB: `205`
- Free GiB: `278`
- Used %: `42%`

## Next-watch items
- Rerun the focused local validation touched by the `meta_min` repair, then push the salvage commit to PR `#688`.
- Rerun only the affected required contexts within Directive 283 budget and merge immediately if they go green.
- Create exactly one PR, poll required protected contexts only via bounded REST checks, and merge only with a merge commit once all required checks are green.

# Rolling Operations Journal Entry

- Directive: `DIRECTIVE 288 — NA-0233 MockProvider Fixed Vault Key Resolution`
- Begin timestamp (America/Chicago): 2026-04-12T18:29:35-05:00
- Begin timestamp (UTC): 2026-04-12T23:29:35Z
- End timestamp (America/Chicago): pending at authoring time
- End timestamp (UTC): pending at authoring time

## Repo SHAs
- qsl-protocol branch: `na-0233-mockprovider-fixed-key-resolution`
- qsl-protocol HEAD: `d9a0d3260ae0`
- qsl-protocol main: `2fed053e7f80`
- qsl-protocol origin/main: `2fed053e7f80`
- qsl-protocol mirror/main: `2fed053e7f80`
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
- Branch: `na-0233-mockprovider-fixed-key-resolution`
- PR: `#688`
- Merge commit: `n/a`

## What changed
- Refreshed `qsl-protocol` to current `main` `2fed053e7f80` and re-proved the shipped/shared MockProvider fixed/default vault-key path is still live through `qsl/qsl-client/qsc/src/vault/mod.rs`, with call sites in `qsl/qsl-client/qsc/src/main.rs` and `qsl/qsl-client/qsc/src/tui/controller/commands/dispatch.rs`.
- Re-proved that local branch `na-0233-mockprovider-fixed-key-resolution` still matches PR `#688` head `d9a0d3260ae0`, while refreshed current state leaves that PR `OPEN` with merge state `DIRTY`.
- Performed a non-destructive refreshed-main integration proof and confirmed the only conflicts are in-scope governance files: `DECISIONS.md`, `TRACEABILITY.md`, and `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.
- Chose `resume_in_place` as the truthful/minimal path because the existing branch remains salvageable without force-push/history rewrite and the refreshed-main integration conflicts stay inside the allowed scope.
- Merged refreshed `main` into the existing implementation branch and renumbered the runtime-fix governance/evidence history on top of `D-0403` through `D-0405` from the merged CI-rebalance lane.

## Failures / recoveries
- `git merge --no-ff main` -> recoverable because refreshed `main` introduced only expected governance-history conflicts in `DECISIONS.md`, `TRACEABILITY.md`, and `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`; corrective action: resolve the three in-scope files against refreshed `main` and continue on the existing branch; final result: merge-resolution in progress at authoring time.

## Validation / CI notes
- Full local validation on the refreshed implementation head remains pending.
- The final implementation head will reuse PR `#688` in place; push is pending the first fully green local validation bundle.
- No queue edits, archive docs, workflow changes, or runtime work outside the approved NA-0233 scope are part of this lane.

## Disk watermark
- Filesystem: `/srv/qbuild`
- Total GiB: `484`
- Used GiB: `208`
- Free GiB: `276`
- Used %: `43%`

## Next-watch items
- Finish the refreshed-main merge resolution, then rerun the full NA-0233 local validation bundle on the final implementation head.
- Push PR `#688` immediately after the first fully green local validation bundle, then poll required protected contexts only via bounded REST checks and merge with a merge commit once all required contexts are green.

# Rolling Operations Journal Entry

- Directive: `DIRECTIVE 289 — NA-0233 Closeout / Evidence / Tier-0 Vault Successor Promotion`
- Begin timestamp (America/Chicago): 2026-04-12T20:23:44-05:00
- Begin timestamp (UTC): 2026-04-13T01:23:44Z
- End timestamp (America/Chicago): pending at authoring time
- End timestamp (UTC): pending at authoring time

## Repo SHAs
- qsl-protocol branch: `na-0233-closeout-mockprovider`
- qsl-protocol HEAD: `c6c5f44e32b5`
- qsl-protocol main: `c6c5f44e32b5`
- qsl-protocol origin/main: `c6c5f44e32b5`
- qsl-protocol mirror/main: `c6c5f44e32b5`
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
- Branch: `na-0233-closeout-mockprovider`
- PR: `pending at authoring time`
- Merge commit: `n/a`

## What changed
- Re-proved on refreshed merged `main` that PR `#688` is already merged at `c6c5f44e32b5`, `DECISIONS.md` already carries `D-0406`, `TRACEABILITY.md` already carries `NA-0233 implementation/evidence`, directly affected runtime/test updates are on `main`, and `DOC-AUD-003` now marks `F03` resolved.
- Re-read refreshed `DOC-AUD-003` to extract the de-duplicated Tier 0 ordering after `F03` resolution and confirm that `F04` is now the sole remaining immediate vault-hardening item while KT stays prerequisite-blocked.
- Prepared the governance-only closeout lane: archive evidence for merged PR `#688`, `NEXT_ACTIONS.md` transition from `NA-0233 READY` to `NA-0233 DONE`, `DECISIONS.md` closeout entry `D-0407`, `TRACEABILITY.md` closeout plus `NA-0234 READY` entries, and the required companion testplan stub.
- Kept the staged 8-file audit packet read-only and unchanged.

## Failures / recoveries
- `printf '---DOC-AUD-003 KEY---\n'` -> recoverable because bash treated the leading `---` as an invalid option in a read-only evidence-gathering step; corrective action: ignore the decorative print and use the succeeding `rg`/`sed` proof output directly; final result: Tier 0 ordering, KT prerequisite-blocked status, and `F04` successor basis were captured successfully.

## Validation / CI notes
- This lane is governance-only: local validation is limited to goal-lint, markdown inventory, manual link-integrity, and leak-safe added-line scanning.
- No runtime battery or CI/workflow implementation work is part of this directive.

## Disk watermark
- Filesystem: `/srv/qbuild`
- Total GiB: `484`
- Used GiB: `208`
- Free GiB: `276`
- Used %: `43%`

## Next-watch items
- Finish docs-only validation, then push `na-0233-closeout-mockprovider` and open exactly one closeout PR.
- Merge only after required protected contexts are green, then refresh `main` and prove `NA-0233` is `DONE`, `NA-0234` is the sole `READY` item, and the workspace is clean.

# Rolling Operations Journal Entry

- Directive: `DIRECTIVE 290 — NA-0234 Vault Read-Path KDF Floor / Format Acceptance Resolution`
- Begin timestamp (America/Chicago): 2026-04-13T20:21:32-05:00
- Begin timestamp (UTC): 2026-04-14T01:21:32Z
- End timestamp (America/Chicago): pending at authoring time
- End timestamp (UTC): pending at authoring time

## Repo SHAs
- qsl-protocol branch: `na-0234-vault-readpath-floor-resolution`
- qsl-protocol HEAD: `pending local implementation commit at authoring time (refreshed main base 844784649192)`
- qsl-protocol main: `844784649192`
- qsl-protocol origin/main: `844784649192`
- qsl-protocol mirror/main: `844784649192`
- qsl-server main: `0826ffa4d6f3`
- qsl-server origin/main: `0826ffa4d6f3`
- qsl-server mirror/main: `0826ffa4d6f3`
- qsl-attachments main: `e94107ac094d`
- qsl-attachments origin/main: `e94107ac094d`
- qsl-attachments mirror/main: `e94107ac094d`

## READY proof
- READY_COUNT: `1`
- Sole READY item: `NA-0234 — Vault Read-Path KDF Floor / Format Acceptance Resolution`
- Proof source: refreshed `NEXT_ACTIONS.md` on `main`

## Worktree / branch / PR
- Worktree path: `/srv/qbuild/work/NA-0234/qsl-protocol`
- Branch: `na-0234-vault-readpath-floor-resolution`
- PR: `pending at authoring time`
- Merge commit: `n/a`

## What changed
- Re-proved on refreshed `main` that the shipped/shared passphrase-vault read path is still affected: `load_vault_runtime_with_passphrase()` reads bytes, `parse_envelope()` maps the stored KDF fields verbatim, and `derive_runtime_key()` previously passed those attacker-controlled values straight into `argon2::Params::new`.
- Captured a live shared-path proof by generating a valid weak-profile passphrase vault (`kdf_m_kib=4096`, `kdf_t=1`, `kdf_p=1`) outside the repo and confirming `qsc vault unlock` succeeded against it before the fix.
- Tightened `qsl/qsl-client/qsc/src/vault/mod.rs` so passphrase vault reads now reject any stored KDF profile other than the exact write-time `19456/2/1` profile and derive with those canonical constants only.
- Added direct regressions in `qsl/qsl-client/qsc/tests/vault.rs` proving both below-floor and otherwise non-canonical passphrase profiles fail closed without mutating the vault.
- Updated `DECISIONS.md` `D-0408`, `TRACEABILITY.md`, and `docs/audit/DOC-AUD-003_Security_Audit_Packet_Intake_and_Remediation_Plan_v0.1.0_DRAFT.md` to record the resolved runtime truth as implementation/evidence only; queue closeout remains out of scope.

## Failures / recoveries
- `cargo run --locked -p qsc -- vault init --non-interactive --key-source passphrase --passphrase-file "$passfile"` -> recoverable command-shape failure because the first live repro omitted `QSC_CONFIG_DIR`, so `vault init` hit an unrelated existing default-path vault and returned `vault_exists`; corrective action: reran the repro against an isolated temp config with `QSC_CONFIG_DIR` set explicitly; final result: reproduced the pre-fix acceptance truth with a valid weak-profile vault.
- `cargo fmt --check` -> recoverable local validation failure because the touched Rust files needed formatter normalization only; corrective action: ran `cargo fmt` and reran `cargo fmt --check`; final result: pass.

## Validation / CI notes
- Local validation already green on the implementation tree for: `cargo fmt --check`, `cargo build --locked`, `cargo clippy --locked -- -D warnings`, `cargo test --locked --test vault`, `cargo test --locked --test unlock_gate`, `cargo test --locked --test handshake_security_closure`, `cargo test --locked --test handshake_contract_na0217i`, `cargo test --locked --test handshake_mvp`, `cargo test --locked --test qsp_protocol_gate`, `cargo test --locked --test identity_binding`, `cargo test --locked --test identity_foundation_contract_na0217d`, `cargo test --locked --test protocol_state_contract_na0217c`, `cargo test --locked --test fs_store_contract_na0217b`, and `cargo test --locked --test desktop_gui_contract_na0215b`.
- Remaining local validation at authoring time: local goal-lint via synthesized event payload, docs inventory commands, manual markdown link-integrity runbook, added-line leak-safe scan, and changed-path scope proof.
- Branch push, PR creation, protected-check polling, and merge are still pending at authoring time.

## Disk watermark
- Filesystem: `/srv/qbuild`
- Total GiB: `484`
- Used GiB: `208`
- Free GiB: `276`
- Used %: `43%`

## Next-watch items
- Finish the remaining docs/governance validation bundle on the final tree, then push the branch immediately.
- Open exactly one implementation/evidence PR with the required Goals/Impact/No-regression/Tests metadata, poll protected checks only via bounded REST, and merge with a merge commit once all required contexts are green.
- After merge, refresh `main` again and re-prove that the resolved vault-read-path truth, sole READY `NA-0234`, and this journal entry are present on refreshed `main` without starting closeout work.

# Rolling Operations Journal Entry

- Directive: `DIRECTIVE 293 — NA-0234 Closeout / Evidence / CI-Security Governance Successor Promotion`
- Begin timestamp (America/Chicago): 2026-04-14T21:12:40-05:00
- Begin timestamp (UTC): 2026-04-15T02:12:40Z
- End timestamp (America/Chicago): pending at authoring time
- End timestamp (UTC): pending at authoring time

## Repo SHAs
- qsl-protocol branch: `na-0234-closeout-vault-readpath`
- qsl-protocol HEAD: `b04fae87a64c`
- qsl-protocol main: `7c48828fc1ef`
- qsl-protocol origin/main: `7c48828fc1ef`
- qsl-protocol mirror/main: `7c48828fc1ef`
- qsl-server main: `0826ffa4d6f3`
- qsl-server origin/main: `0826ffa4d6f3`
- qsl-server mirror/main: `0826ffa4d6f3`
- qsl-attachments main: `e94107ac094d`
- qsl-attachments origin/main: `e94107ac094d`
- qsl-attachments mirror/main: `e94107ac094d`

## READY proof
- READY_COUNT: `1`
- Sole READY item: `NA-0234 — Vault Read-Path KDF Floor / Format Acceptance Resolution`
- Proof source: refreshed `NEXT_ACTIONS.md` on `main`

## Worktree / branch / PR
- Worktree path: `/srv/qbuild/work/NA-0234/qsl-protocol`
- Branch: `na-0234-closeout-vault-readpath`
- PR: `#694`
- Merge commit: `n/a`

## What changed
- Re-proved on refreshed `main` that PR #693 is already merged as `7c48828fc1ef` and that `main` already carries `D-0408`, the `TRACEABILITY.md` `NA-0234 implementation/evidence` entry, the merged passphrase-vault profile guard in `qsl/qsl-client/qsc/src/vault/mod.rs`, the direct regressions in `qsl/qsl-client/qsc/tests/vault.rs`, and the `DOC-AUD-003` `F04` resolved state.
- Re-proved that KT remains prerequisite-blocked on unresolved serialization/profile closure and bundle-signature semantics, so a direct KT implementation lane is still not truthful on refreshed `main`.
- Re-proved the live CI/security governance gap after PR #690: `advisories` is not a required PR gate, `qsc-linux-full-suite` and `macos-qsc-full-serial` remain push-only outside protected PR gating, and PR #693 still merged while `advisories` failed.
- Added governance-only closeout artifacts to archive `NA-0234` evidence, mark `NA-0234` `DONE`, and promote `NA-0235 — PR Dependency-Audit Gate + Full-Suite Governance Repair` as the sole READY successor.

## Failures / recoveries
- None at authoring time.

## Validation / CI notes
- Pre-mutation authority proof completed: disk watermark green, configured-remotes-only refresh completed for `qsl-protocol`, `qsl-server`, and `qsl-attachments`, `READY_COUNT=1` with `NA-0234` as the sole READY item, `qsl-server READY=0`, and `qsl-attachments READY=0`.
- Refreshed current-main proof shows `NA-0234` implementation/evidence is already merged on `main` via PR #693 and that the closeout lane is governance-only.
- Completed local validation so far on the branch tree: markdown inventory counts (`tests/*.md=48`, `tests/**/*.md=1`, `docs/*.md=226`, `docs/**/*.md=221`), the manual markdown link-integrity runbook (`TOTAL_MISSING 0`), and the added-line leak-safe scan (`ADDED_LINE_COUNT 130`, `v1-path pattern count: 0`, `hex32plus pattern count: 0`).
- Local goal-lint is green against the real PR body: `scripts/audit/run_goal_lint_pr.sh 694` validated base `7c48828fc1efbda948ec575b3c4a0aeecebf8763` versus head `b04fae87a64c894f7d0c7327ce92f5423a60bc9e`.
- Post-creation changed-path scope proof is green: `gh pr diff 694 --name-only` shows only `DECISIONS.md`, `NEXT_ACTIONS.md`, `TRACEABILITY.md`, `docs/archive/testplans/NA-0234_vault_read_path_kdf_floor_format_acceptance_resolution_evidence.md`, `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`, and `tests/NA-0234_closeout_evidence_testplan.md`.
- Protected-check polling and merge are still pending at authoring time.

## Disk watermark
- Filesystem: `/srv/qbuild`
- Total GiB: `484`
- Used GiB: `212`
- Free GiB: `273`
- Used %: `44%`

## Next-watch items
- Finish the governance-only validation bundle on the final branch tree, then push `na-0234-closeout-vault-readpath` immediately.
- Open exactly one closeout PR with the required Goals/Impact/No-regression/Tests metadata, poll protected contexts only via bounded REST checks, and merge with a merge commit once the protected set is green.
- After merge, refresh `main` again and re-prove that `NA-0234` is `DONE`, `NA-0235` is the sole `READY` item, and this journal entry is present on refreshed `main` without reopening runtime scope.

# Rolling Operations Journal Entry

- Directive: `DIRECTIVE 309 — NA-0235A Paired Dependency Remediation (qsl-attachments first, qsl-protocol second)`
- Begin timestamp (America/Chicago): 2026-04-16T21:55:32-05:00
- Begin timestamp (UTC): 2026-04-17T02:55:32Z
- End timestamp (America/Chicago): pending at authoring time
- End timestamp (UTC): pending at authoring time

## Repo SHAs
- qsl-protocol branch: `na-0235a-protocol-dependency-unblock-v3`
- qsl-protocol HEAD: `pending local implementation commit at authoring time (refreshed main base e49d4b699fa9)`
- qsl-protocol main: `e49d4b699fa9`
- qsl-protocol origin/main: `e49d4b699fa9`
- qsl-protocol mirror/main: `e49d4b699fa9`
- qsl-server main: `0826ffa4d6f3`
- qsl-server origin/main: `0826ffa4d6f3`
- qsl-server mirror/main: `0826ffa4d6f3`
- qsl-attachments implementation branch: `na-0235a-qsl-attachments-rand-remediation-v3`
- qsl-attachments branch head: `a53459f73e51`
- qsl-attachments main: `a1a4c1269899`
- qsl-attachments origin/main: `a1a4c1269899`
- qsl-attachments mirror/main: `a1a4c1269899`

## READY proof
- READY_COUNT: `1`
- Sole READY item: `NA-0235A — Runtime Dependency Advisory Remediation for Public-Safety Unblock`
- Proof source: refreshed `NEXT_ACTIONS.md` on `main`

## Worktree / branch / PR
- qsl-protocol worktree path: `/srv/qbuild/work/NA-0235A/qsl-protocol`
- qsl-protocol branch: `na-0235a-protocol-dependency-unblock-v3`
- qsl-protocol PR: `pending at authoring time`
- qsl-attachments worktree path: `/srv/qbuild/work/NA-0235A/qsl-attachments`
- qsl-attachments PR: `#30`
- qsl-attachments merge commit: `a1a4c1269899`

## What changed
- Re-proved on refreshed `main` that PR `#695` remains open and blocked by live `public-safety` / `advisories` failures, and that the blocker set is the combination of runtime `rustls-webpki 0.103.10`, tooling-only `rand 0.9.2`, the cross-repo `qsl-attachments` harness `rand 0.8.5` path, and the refimpl runtime `rand 0.8.5` path.
- Landed qsl-attachments PR #30 first: swapped its single opaque-handle generator helper from `rand` to `rand_core`, refreshed `qsl-attachments/Cargo.lock`, validated locally with `cargo fmt --check`, `cargo build --locked`, `cargo clippy --locked -- -D warnings`, `cargo test --locked`, and `cargo audit --deny warnings`, then merged the PR as `a1a4c1269899`.
- Updated qsl-protocol to the merged qsl-attachments rev, migrated refimpl `stdcrypto` / `qsp` / `suite2` RNG imports from `rand` to `rand_core`, removed the unused direct `apps/qsl-tui` `rand` pin, updated `rustls-webpki` to `0.103.12`, and updated the tooling-only `rand 0.9.x` path to `0.9.4`.
- Re-proved that the stale residual `rand 0.8.5` blocker remained only through `ratatui-termwiz -> termwiz -> phf_generator`, then replaced the umbrella `ratatui` dependency in qsc/qsl-tui with the direct `ratatui-core` / `ratatui-widgets` / `ratatui-crossterm` crates so the stale termwiz backend chain drops out of the lockfile without changing TUI behavior.
- Verified locally on the in-progress qsl-protocol tree that `cargo audit --deny warnings` is now green and the stale `rand 0.8.5` / `termwiz` / `phf_generator` package IDs no longer resolve in the lockfile.

## Failures / recoveries
- `cargo audit --deny warnings` in the initial qsl-protocol pre-mutation bundle exited non-zero and stopped the chained proof script; classified as recoverable because the non-zero was the expected live-advisory discovery the scan was meant to prove. Corrective action: reran the dependency-tree proof commands with zero-safe handling and continued once the blocker set was captured. Final result: blocker classification completed truthfully.
- `cargo build` on the first qsl-protocol TUI-split patch failed with `E0422` because `qsl/qsl-client/qsc/src/tui/controller/render.rs` still referenced `Margin` after the Ratatui crate split without importing it through the root qsc prelude. Classified as an in-scope local build failure with understood cause. Corrective action: added `Margin` to the root qsc Ratatui-core layout imports and reran the build. Final result: `cargo build` passed and the final lockfile no longer contains the stale termwiz chain.

## Validation / CI notes
- qsl-attachments local validation passed before push: `cargo fmt --check`, `cargo build --locked`, `cargo clippy --locked -- -D warnings`, `cargo test --locked`, `cargo audit --deny warnings`.
- qsl-attachments branch push timestamp: `2026-04-17T10:49:03-05:00` / `2026-04-17T10:49:03Z`.
- qsl-attachments PR #30 scope proof is green: `gh pr diff 30 --name-only` shows only `Cargo.toml`, `Cargo.lock`, and `src/lib.rs`.
- qsl-attachments required protected context proof is green: required set `rust`; bounded polling reached `ITER=3/180 REQUIRED=1 ATTACHED=1 SUCCESS=1 INPROG=0 FAILS=0 MISSING=0`.
- qsl-protocol local implementation is still in progress at authoring time; remaining required work is the full final validation bundle, branch push, PR creation, protected-check polling, merge, and refreshed-main evidence capture.

## Disk watermark
- Filesystem: `/srv/qbuild`
- Total GiB: `484`
- Used GiB: `214`
- Free GiB: `270`
- Used %: `45%`

## Next-watch items
- Finish the qsl-protocol validation bundle on the final implementation head without widening scope.
- Push `na-0235a-protocol-dependency-unblock-v3` immediately after the first full green local bundle, then open exactly one qsl-protocol PR with the required metadata.
- Poll only required protected contexts via bounded REST, merge with a merge commit once the required set is green, and then refresh `main` again to re-prove green audit truth, sole READY `NA-0235A`, journal presence, and a clean workspace.

# Rolling Operations Journal Entry

- Directive: `DIRECTIVE 311 — NA-0235A Closeout / Restore NA-0235 as Sole READY`
- Begin timestamp (America/Chicago): 2026-04-17T22:16:12-05:00
- Begin timestamp (UTC): 2026-04-18T03:16:12Z
- End timestamp (America/Chicago): pending at authoring time
- End timestamp (UTC): pending at authoring time

## Repo SHAs
- qsl-protocol branch: `na-0235a-closeout-restore-na0235`
- qsl-protocol HEAD: `pending local closeout commit at authoring time (refreshed main base 2113201edff6)`
- qsl-protocol main: `2113201edff6`
- qsl-protocol origin/main: `2113201edff6`
- qsl-protocol mirror/main: `2113201edff6`
- qsl-server main: `0826ffa4d6f3`
- qsl-server origin/main: `0826ffa4d6f3`
- qsl-server mirror/main: `0826ffa4d6f3`
- qsl-attachments main: `1e1ae272a4cb`
- qsl-attachments origin/main: `1e1ae272a4cb`
- qsl-attachments mirror/main: `1e1ae272a4cb`

## READY proof
- READY_COUNT: `1`
- Sole READY item: `NA-0235A — Runtime Dependency Advisory Remediation for Public-Safety Unblock`
- Proof source: refreshed `NEXT_ACTIONS.md` on `main`

## Worktree / branch / PR
- qsl-protocol worktree path: `/srv/qbuild/work/NA-0235A/qsl-protocol`
- qsl-protocol branch: `na-0235a-closeout-restore-na0235`
- qsl-protocol PR: `pending at authoring time`
- qsl-attachments worktree path: `/srv/qbuild/work/NA-0235A/qsl-attachments`
- qsl-attachments PR #31 merge commit: `1e1ae272a4cb`
- qsl-protocol PR #702 merge commit: `2113201edff6`

## What changed
- Re-proved on refreshed `main` that qsl-attachments Phase A PR #30 and salvage hotfix PR #31 are merged and durable on qsl-attachments `main`, qsl-protocol PR #702 is merged and durable on qsl-protocol `main`, refreshed qsl-protocol `main` now passes `cargo audit --deny warnings`, and PR `#695` remains OPEN and unmerged.
- Added governance-only closeout artifacts to archive the merged `NA-0235A` dependency-remediation evidence, mark `NA-0235A` `DONE`, and restore `NA-0235` as the sole `READY` item on refreshed `main`.
- Preserved the underlying `NA-0235` workflow/governance scope while tightening its resume note so the next truthful lane starts from refreshed `main` and either salvages or supersedes PR `#695` without changing the runtime-free nature of that work.

## Failures / recoveries
- `printf '--- qsl-protocol NEXT_ACTIONS READY/BLOCKED proof ---\n'` failed because bash `printf` treated the leading `---` as an option. Classified as a recoverable command-shape mistake. Corrective action: reran the proof bundle with `printf --`. Final result: READY / BLOCKED proof, merged PR proof, and audit proof all completed truthfully.
- `rg -n "Directive: `DIRECTIVE 31|Directive: `DIRECTIVE 30"` against the rolling journal failed because the shell interpreted the backticks before `rg` ran. Classified as a recoverable command-shape mistake. Corrective action: switched to direct `tail`/targeted inspection instead of shell-interpreted backtick patterns. Final result: journal formatting was inspected successfully without widening scope.

## Validation / CI notes
- Pre-mutation authority proof completed: disk watermark green, configured-remotes-only refresh completed for `qsl-protocol`, `qsl-server`, and `qsl-attachments`, `READY_COUNT=1` with `NA-0235A` as the sole READY item, `qsl-server READY=0`, `qsl-attachments READY=0`, qsl-attachments PR #31 merged durable on `main`, qsl-protocol PR #702 merged durable on `main`, refreshed qsl-protocol `main` audit-green, and PR `#695` still OPEN.
- Local validation still pending at authoring time: goal-lint, markdown inventory, manual link-integrity runbook, added-line leak-safe scan, changed-path scope proof, PR creation, protected-check polling, merge, and refreshed-main post-merge proof.

## Disk watermark
- Filesystem: `/srv/qbuild`
- Total GiB: `484`
- Used GiB: `220`
- Free GiB: `264`
- Used %: `46%`

## Next-watch items
- Finish the governance-only validation bundle on `na-0235a-closeout-restore-na0235`, then push the branch immediately.
- Open exactly one closeout PR with the required Goals/Impact/No-regression/Tests metadata, poll only required protected contexts via bounded REST, and merge with a merge commit once the protected set is green.
- After merge, refresh `main` again and re-prove that `NA-0235A` is `DONE`, `NA-0235` is the sole `READY` item, the Directive 311 journal entry is present on refreshed `main`, and PR `#695` remains OPEN and untouched.

# Rolling Operations Journal Entry

- Directive: `DIRECTIVE 321 — NA-0235 Closeout + Promote NA-0236 KT Canon Closure`
- Begin timestamp (America/Chicago): 2026-04-18T23:06:16-05:00
- Begin timestamp (UTC): 2026-04-19T04:06:16Z
- End timestamp (America/Chicago): pending at authoring time
- End timestamp (UTC): pending at authoring time

## Repo SHAs
- qsl-protocol branch: `na-0235-closeout-promote-na0236`
- qsl-protocol HEAD: `pending local closeout commit at authoring time (refreshed main base f071bdae0c6a)`
- qsl-protocol main: `f071bdae0c6a`
- qsl-protocol origin/main: `f071bdae0c6a`
- qsl-protocol mirror/main: `f071bdae0c6a`
- qsl-server main: `0826ffa4d6f3`
- qsl-server origin/main: `0826ffa4d6f3`
- qsl-server mirror/main: `0826ffa4d6f3`
- qsl-attachments main: `1e1ae272a4cb`
- qsl-attachments origin/main: `1e1ae272a4cb`
- qsl-attachments mirror/main: `1e1ae272a4cb`

## READY proof
- READY_COUNT: `1`
- Sole READY item: `NA-0235 — PR Dependency-Audit Gate + Full-Suite Governance Repair`
- Proof source: refreshed `NEXT_ACTIONS.md` on `main`

## Worktree / branch / PR
- qsl-protocol worktree path: `/srv/qbuild/work/NA-0235/qsl-protocol`
- qsl-protocol branch: `na-0235-closeout-promote-na0236`
- qsl-protocol PR: `pending at authoring time`
- merged implementation PR: `#695`
- merged implementation commit: `f071bdae0c6a`

## What changed
- Re-proved on refreshed `main` that PR `#695` merged normally as `f071bdae0c6a`, that its parent 1 is prior `main` `569d21cfcb19`, that parent 2 is final PR head `6c0e3385d861`, and that refreshed `main` contains exactly the six expected `NA-0235` workflow/governance paths.
- Re-proved post-incident branch-protection truth after the manual GitHub UI remove/re-add of `public-safety`: the required protected set remains intact, `public-safety` is still a GitHub Actions required check (`app_id 15368`), approvals remain `0`, conversation resolution remains `false`, `enforce_admins` remains `true`, and merge queue remains absent while merge-commit plus auto-merge settings remain enabled.
- Added governance-only closeout artifacts to archive the merged `NA-0235` evidence, mark `NA-0235` `DONE`, and promote the supplied `NA-0236` KT prerequisite-closure block as the sole READY successor in the order required by `DOC-AUD-003`.

## Failures / recoveries
- None.

## Validation / CI notes
- Pre-mutation authority proof completed: disk watermark green, configured-remotes-only refresh completed for `qsl-protocol`, `qsl-server`, and `qsl-attachments`, `READY_COUNT=1` with `NA-0235` as the sole READY item, `qsl-server READY=0`, `qsl-attachments READY=0`, PR `#695` merged durable on `main`, refreshed branch protection still requires `public-safety` from GitHub Actions, and refreshed `DOC-AUD-003` still orders `F05 prerequisite closure` before `F05` implementation and ahead of `F06`.
- Local validation is green so far on the branch tree: markdown inventory counts (`tests/*.md=58`, `tests/**/*.md=1`, `docs/*.md=234`, `docs/**/*.md=229`), the manual markdown link-integrity runbook (`TOTAL_MISSING 0`), the added-line leak-safe scan (`ADDED_LINE_COUNT 121`, `v1-path pattern count: 0`, `hex32plus pattern count: 0`, `secret-like marker count: 0`), and the read-only refreshed-main dependency-audit proof via `cargo audit --deny warnings --file <origin/main Cargo.lock snapshot>`.
- Remaining at authoring time: local goal-lint on the committed branch head, changed-path scope proof, PR creation, protected-check polling, merge, and refreshed-main post-merge proof.

## Disk watermark
- Filesystem: `/srv/qbuild`
- Total GiB: `484`
- Used GiB: `221`
- Free GiB: `264`
- Used %: `46%`

## Next-watch items
- Finish the governance-only validation bundle on `na-0235-closeout-promote-na0236`, then push the branch immediately.
- Open exactly one closeout PR with the required Goals/Impact/No-regression/Tests metadata, poll only required protected contexts via bounded REST, and merge with a merge commit once the protected set is green.
- After merge, refresh `main` again and re-prove that `NA-0235` is `DONE`, `NA-0236` is the sole `READY` item, the Directive 321 journal entry is present on refreshed `main`, and the workspace is clean.
