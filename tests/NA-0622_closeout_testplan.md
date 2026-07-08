Goals: G1 (primary), G2, supports G3, G4, G5

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-08

# NA-0622 Closeout and NA-0623 Restoration Test Plan

## Scope

Records closeout validation for NA-0622 (ENG-0012 Stage 1b-ii) under directive
QSL-DIR-2026-07-08-559 (D559). Closeout consumes D-1239, marks NA-0622 DONE (ENG-0012
Stage-1b-ii-done — the classical half of the P1 closed), and restores NA-0623 (Stage 2: PQ-reseed
sender) as the sole READY successor. It does not implement NA-0623 and changes no
source/test/Cargo/spec/workflow/`.claude`/hook.

## Required Markers

- NA0622_CLOSEOUT_D1239_CONSUMED_OK
- NA0622_CLOSEOUT_IMPL_PR_1522_MERGED_OK
- NA0622_CLOSEOUT_POSTMERGE_CHECKS_GREEN_OK
- NA0622_CLOSEOUT_MAIN_FASTFORWARD_OK
- NA0622_CLOSEOUT_DH_RATCHET_ON_REAL_SEND_PATH_ON_MAIN_OK
- NA0622_CLOSEOUT_STATIC_RK_BOOTSTRAP_ABSENT_ON_MAIN_OK
- NA0622_CLOSEOUT_REGRESSION_FIXES_NO_BYPASS_OK
- NA0622_CLOSEOUT_D1240_RECORDED_ONCE_OK
- NA0622_CLOSEOUT_NA0622_MARKED_DONE_OK
- NA0622_CLOSEOUT_ENG0012_STAGE1BII_DONE_CLASSICAL_P1_CLOSED_OK
- NA0622_CLOSEOUT_NA0623_RESTORED_SOLE_READY_OK
- NA0622_CLOSEOUT_ONE_READY_INVARIANT_OK
- NA0622_CLOSEOUT_LIVE_QUEUE_HEADER_UPDATED_OK
- NA0622_CLOSEOUT_D1241_ABSENT_OK
- NA0622_CLOSEOUT_BOUNDARY_MUTATION_OK
- NA0622_CLOSEOUT_PRIVATE_MATERIAL_SCAN_OK

## Validation Plan (class-only)

1. Verify implementation PR #1522 merged via a merge commit at `e3e210c5`; local main
   fast-forwarded; the DH-boundary send+recv wiring present and the static-`rk` bootstrap absent
   on main; D-1239 present; ENG-0012 Stage-1b-ii-done; worktree clean before closeout edits.
2. Regression pass (no bypass): two local full-qsc-suite passes surfaced 8 failures, all
   test-model artifacts / behavior-change test updates (session-based gate correction, v2-strip
   helpers, send-ready transition), fixed locally and re-verified green (WF-0013 workspace build +
   144-binary suite) before pushing. The PR then passed CI green on the first run; no bypass, no
   `--admin`, no rerun.
3. Governance: D-1240 recorded exactly once; D-1241 absent; NA-0622 marked DONE (ENG-0012
   Stage-1b-ii-done, the classical P1 closed); NA-0623 (Stage 2) restored as the sole READY
   successor; exactly one `Status: READY`; `qsl_evidence_helper.py queue` reports
   `READY_COUNT 1 / NA-0623`.
4. LIVE QUEUE header updated: STATE `READY=NA-0623 | HIGHEST_NA=0623 | HIGHEST_D=1240`.
5. Boundary: closeout mutates only NEXT_ACTIONS.md, DECISIONS.md, TRACEABILITY.md,
   ROLLING_OPERATIONS_JOURNAL.md, and this testplan; no NA-0623 implementation; no
   source/test/Cargo/spec/workflow/`.claude`/hook change.
6. Private-material scan on all added lines.

## Result

`NA0622_CLOSEOUT_OK`. NA-0622 DONE; ENG-0012 Stage-1b-ii-done (classical half of the P1 closed —
classical PCS runs on live qsc traffic); NA-0623 (Stage 2: PQ-reseed sender — the post-quantum
half that fully closes the P1 and is the genuine edge over Signal's classical ratchet) sole READY
successor; begins at D-1241.
