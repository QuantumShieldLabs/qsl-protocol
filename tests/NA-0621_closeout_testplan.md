Goals: G1 (primary), G2, supports G3, G4, G5

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-08

# NA-0621 Closeout and NA-0622 Restoration Test Plan

## Scope

Records closeout validation for NA-0621 (ENG-0012 Stage 1b-i) under directive
QSL-DIR-2026-07-08-558 (D558). Closeout consumes D-1237, marks NA-0621 DONE (ENG-0012
Stage-1b-i-done), and restores NA-0622 (Stage 1b-ii: qsc trigger + static-`rk` removal) as the
sole READY successor. It does not implement NA-0622 and changes no source/test/Cargo/spec/
workflow/`.claude`/hook.

## Required Markers

- NA0621_CLOSEOUT_D1237_CONSUMED_OK
- NA0621_CLOSEOUT_IMPL_PR_1520_MERGED_OK
- NA0621_CLOSEOUT_POSTMERGE_CHECKS_GREEN_OK
- NA0621_CLOSEOUT_MAIN_FASTFORWARD_OK
- NA0621_CLOSEOUT_DH_RATCHET_ON_MAIN_OK
- NA0621_CLOSEOUT_CLEAN_FIRST_RUN_NO_BYPASS_OK
- NA0621_CLOSEOUT_D1238_RECORDED_ONCE_OK
- NA0621_CLOSEOUT_NA0621_MARKED_DONE_OK
- NA0621_CLOSEOUT_ENG0012_STAGE1BI_DONE_P1_STILL_OPEN_OK
- NA0621_CLOSEOUT_NA0622_RESTORED_SOLE_READY_OK
- NA0621_CLOSEOUT_ONE_READY_INVARIANT_OK
- NA0621_CLOSEOUT_LIVE_QUEUE_HEADER_UPDATED_OK
- NA0621_CLOSEOUT_D1239_ABSENT_OK
- NA0621_CLOSEOUT_BOUNDARY_MUTATION_OK
- NA0621_CLOSEOUT_PRIVATE_MATERIAL_SCAN_OK

## Validation Plan (class-only)

1. Verify implementation PR #1520 merged via a merge commit at `6f2118e1`; local main
   fast-forwarded; `send_boundary`/`recv_dh_boundary` and D-1237 present on main; ENG-0012
   Stage-1b-i-done; worktree clean before closeout edits.
2. Clean first run: PR #1520 passed all required checks (38 total, 0 pending, 0 failed) on the
   first CI run — the WF-0013 workspace build was applied pre-push. No bypass, no `--admin`, no
   rerun.
3. Governance: D-1238 recorded exactly once; D-1239 absent; NA-0621 marked DONE (ENG-0012
   Stage-1b-i-done, the P1 still OPEN); NA-0622 (Stage 1b-ii) restored as the sole READY
   successor; exactly one `Status: READY`; `qsl_evidence_helper.py queue` reports
   `READY_COUNT 1 / NA-0622`.
4. LIVE QUEUE header updated: STATE `READY=NA-0622 | HIGHEST_NA=0622 | HIGHEST_D=1238`.
5. Boundary: closeout mutates only NEXT_ACTIONS.md, DECISIONS.md, TRACEABILITY.md,
   ROLLING_OPERATIONS_JOURNAL.md, and this testplan; no NA-0622 implementation; no
   source/test/Cargo/spec/workflow/`.claude`/hook change.
6. Private-material scan on all added lines.

## Result

`NA0621_CLOSEOUT_OK`. NA-0621 DONE; ENG-0012 Stage-1b-i-done (P1 still open); NA-0622 (Stage
1b-ii — where classical post-compromise security closes on the real qsc send path) sole READY
successor; begins at D-1239.
