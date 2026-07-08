Goals: G1 (primary), G2, supports G3, G4, G5

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-08

# NA-0620 Closeout and NA-0621 Restoration Test Plan

## Scope

Records closeout validation for NA-0620 (ENG-0012 Stage 1a) under directive
QSL-DIR-2026-07-08-557 (D557). Closeout consumes D-1235, marks NA-0620 DONE, files WF-0012/
WF-0013, and restores NA-0621 (Stage 1b DH-ratchet behavior) as the sole READY successor. It
does not implement NA-0621 and changes no source/test/Cargo/spec/workflow/`.claude`/hook.

## Required Markers

- NA0620_CLOSEOUT_D1235_CONSUMED_OK
- NA0620_CLOSEOUT_IMPL_PR_1518_MERGED_OK
- NA0620_CLOSEOUT_POSTMERGE_CHECKS_GREEN_OK
- NA0620_CLOSEOUT_MAIN_FASTFORWARD_OK
- NA0620_CLOSEOUT_RECOVERED_FAILURE_EVIDENCE_ACTOR_BUILD_OK
- NA0620_CLOSEOUT_NO_CHECK_BYPASS_OK
- NA0620_CLOSEOUT_D1236_RECORDED_ONCE_OK
- NA0620_CLOSEOUT_NA0620_MARKED_DONE_OK
- NA0620_CLOSEOUT_ENG0012_STAGE1A_DONE_OK
- NA0620_CLOSEOUT_WF0012_WF0013_FILED_OK
- NA0620_CLOSEOUT_NA0621_RESTORED_SOLE_READY_OK
- NA0620_CLOSEOUT_ONE_READY_INVARIANT_OK
- NA0620_CLOSEOUT_LIVE_QUEUE_HEADER_UPDATED_OK
- NA0620_CLOSEOUT_D1237_ABSENT_OK
- NA0620_CLOSEOUT_BOUNDARY_MUTATION_OK
- NA0620_CLOSEOUT_PRIVATE_MATERIAL_SCAN_OK

## Validation Plan (class-only)

1. Verify implementation PR #1518 merged via a merge commit at `685e4367`; local main
   fast-forwarded; the DH-ratchet plumbing (Suite2DhRatchetState / snapshot v2) and D-1235
   present on main; worktree clean before closeout edits.
2. Recovered-failure evidence: the first CI run (`68a16173`) failed the ci-4*/demo/metadata
   build checks with `missing field \`dh\`` in the `refimpl_actor` (two direct
   `Suite2SessionState` literals missed by a refimpl+qsc-only local build); corrective commit
   `245bdc7b` added the field; verified `cargo build --workspace --all-targets` clean; the
   re-run went green. No bypass, no `--admin`, no rerun. Lesson filed as WF-0013.
3. Governance: D-1236 recorded exactly once; D-1237 absent; NA-0620 marked DONE (ENG-0012
   Stage-1a-done); WF-0012 (ledger tool) and WF-0013 (build-workspace lesson) filed; NA-0621
   (Stage 1b) restored as the sole READY successor; exactly one `Status: READY`;
   `qsl_evidence_helper.py queue` reports `READY_COUNT 1 / NA-0621`.
4. LIVE QUEUE header updated: STATE `READY=NA-0621 | HIGHEST_NA=0621 | HIGHEST_D=1236`.
5. Boundary: closeout mutates only NEXT_ACTIONS.md, IMPROVEMENT_LEDGER.md, DECISIONS.md,
   TRACEABILITY.md, ROLLING_OPERATIONS_JOURNAL.md, and this testplan; no NA-0621
   implementation; no source/test/Cargo/spec/workflow/`.claude`/hook change.
6. Private-material scan on all added lines.

## Result

`NA0620_CLOSEOUT_OK`. NA-0620 DONE; ENG-0012 Stage-1a-done; WF-0012/WF-0013 filed; NA-0621
(Stage 1b DH-ratchet behavior — where classical post-compromise security lands) sole READY
successor; begins at D-1237.
