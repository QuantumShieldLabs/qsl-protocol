Goals: G1 (primary), supports G2, G3, G4, G5

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-07

# NA-0618 Closeout and NA-0619 Restoration Test Plan

## Scope

Records closeout validation for NA-0618 under directive QSL-DIR-2026-07-07-555 (D555).
Closeout consumes D-1232, marks NA-0618 DONE, and restores NA-0619 (ENG-0012 ratchet-liveness
design, docs-only) as the sole READY successor. It does not implement NA-0619 and changes no
source/test/Cargo/spec/workflow/`.claude`/hook.

## Required Markers

- NA0618_CLOSEOUT_D1232_CONSUMED_OK
- NA0618_CLOSEOUT_IMPL_PR_1515_MERGED_OK
- NA0618_CLOSEOUT_POSTMERGE_CHECKS_GREEN_OK
- NA0618_CLOSEOUT_MAIN_FASTFORWARD_OK
- NA0618_CLOSEOUT_NO_CHECK_BYPASS_OK
- NA0618_CLOSEOUT_D1233_RECORDED_ONCE_OK
- NA0618_CLOSEOUT_NA0618_MARKED_DONE_OK
- NA0618_CLOSEOUT_ENG0013_RESOLVED_OK
- NA0618_CLOSEOUT_NA0619_RESTORED_SOLE_READY_OK
- NA0618_CLOSEOUT_ONE_READY_INVARIANT_OK
- NA0618_CLOSEOUT_LIVE_QUEUE_HEADER_UPDATED_OK
- NA0618_CLOSEOUT_D1234_ABSENT_OK
- NA0618_CLOSEOUT_BOUNDARY_MUTATION_OK
- NA0618_CLOSEOUT_NO_SUCCESSOR_IMPLEMENTATION_OK
- NA0618_CLOSEOUT_PRIVATE_MATERIAL_SCAN_OK

## Validation Plan (class-only)

1. Verify implementation PR #1515 merged via a merge commit at `c6447afb`; local main
   fast-forwarded; `checked_counter_inc` and D-1232 present on main; worktree clean before
   closeout edits.
2. Verify post-merge required checks green with no failed/pending (public-ci/public-safety,
   advisories, goal-lint, suite2-ci, qsc-adversarial smoke+miri, macos-build, formal-ci,
   qshield-ci, demo-packaging, CodeQL, ci-4a..ci-4d, classify/scope); merge CLEAN. No
   transient failure occurred on PR #1515; no `--admin`, no `gh run rerun`, no dispatch.
3. Governance: D-1233 recorded exactly once; D-1234 absent; NA-0618 marked DONE with an
   outcome note; NA-0619 (ENG-0012 design) restored as the sole READY successor; exactly one
   `Status: READY`; `scripts/ci/qsl_evidence_helper.py queue` reports `READY_COUNT 1 /
   NA-0619`.
4. LIVE QUEUE header updated: STATE `READY=NA-0619 | HIGHEST_NA=0619 | HIGHEST_D=1233`; ON
   DECK re-prioritized (ENG-0012 promoted to READY; ENG-0014/ENG-0019/WF-0012 lead ON DECK).
5. ENG-0013 marked done in the ledger (resolved by NA-0618, D-1232).
6. Boundary: closeout mutates only NEXT_ACTIONS.md, DECISIONS.md, TRACEABILITY.md,
   ROLLING_OPERATIONS_JOURNAL.md, and this testplan; no NA-0619 implementation; no
   source/test/Cargo/spec/workflow/`.claude`/hook change; no operator-startup-command
   execution; no runtime/LAN action.
7. Private-material scan on all added lines.

## Result

`NA0618_CLOSEOUT_OK`. NA-0618 DONE; ENG-0013 resolved; NA-0619 (ENG-0012 ratchet-liveness
design) sole READY successor; begins at D-1234.
