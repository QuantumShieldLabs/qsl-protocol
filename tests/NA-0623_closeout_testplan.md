Goals: G1 (primary), G2, supports G3, G4, G5

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-08

# NA-0623 Closeout and NA-0624 Restoration Test Plan

## Scope

Records closeout validation for NA-0623 (ENG-0012 Stage 2a) under directive
QSL-DIR-2026-07-08-560 (D560). Closeout consumes D-1241, marks NA-0623 DONE (ENG-0012 Stage-2a-done
— the refimpl SCKA sender core + the both-sides RK advance), and restores NA-0624 (Stage 2b: qsc
SCKA wiring) as the sole READY successor. It does not implement NA-0624 and changes no
source/test/Cargo/spec/workflow/`.claude`/hook.

## Required Markers

- NA0623_CLOSEOUT_D1241_CONSUMED_OK
- NA0623_CLOSEOUT_IMPL_PR_1524_MERGED_OK
- NA0623_CLOSEOUT_POSTMERGE_CHECKS_GREEN_OK          (38 total, 0 pending, 0 failed; 2 qsc suites path-skipped)
- NA0623_CLOSEOUT_MAIN_FASTFORWARD_OK                (main == origin/main == 07658773)
- NA0623_CLOSEOUT_SCKA_SENDER_ON_MAIN_OK             (send_pq_advertise/send_pq_reseed/track_peer_adv present)
- NA0623_CLOSEOUT_BOTH_SIDES_RK_ADVANCE_ON_MAIN_OK   (recv_boundary_in_order RK-advance present)
- NA0623_CLOSEOUT_APPLY_PQ_RESEED_SEMANTICS_FROZEN_OK (frozen vectors byte-identical)
- NA0623_CLOSEOUT_CLEAN_LANE_NO_BYPASS_OK            (impl PR green on the first run)
- NA0623_CLOSEOUT_D1242_RECORDED_ONCE_OK
- NA0623_CLOSEOUT_NA0623_MARKED_DONE_OK
- NA0623_CLOSEOUT_ENG0012_STAGE2A_DONE_P1_OPEN_OK
- NA0623_CLOSEOUT_NA0624_RESTORED_SOLE_READY_OK
- NA0623_CLOSEOUT_ONE_READY_INVARIANT_OK
- NA0623_CLOSEOUT_LIVE_QUEUE_HEADER_UPDATED_OK       (READY=NA-0624 | HIGHEST_NA=0624 | HIGHEST_D=1242)
- NA0623_CLOSEOUT_STALE_NA0621_READY_PROSE_CORRECTED_OK
- NA0623_CLOSEOUT_D1243_ABSENT_OK
- NA0623_CLOSEOUT_NHK_DEVIATION_CARRIED_TO_2B_OK
- NA0623_CLOSEOUT_BOUNDARY_MUTATION_OK
- NA0623_CLOSEOUT_PRIVATE_MATERIAL_SCAN_OK

## Validation Plan (class-only)

1. Verify implementation PR #1524 merged via a merge commit at `07658773`; local main
   fast-forwarded; the SCKA sender (`send_pq_advertise`/`send_pq_reseed`/`track_peer_adv`) and the
   receiver RK-advance present on main; D-1241 present; ENG-0012 Stage-2a-done; worktree clean
   before closeout edits.
2. Clean lane (no bypass): the NA-0623 implementation PR passed CI green on the first run (WF-0013
   workspace build run pre-push; frozen conformance sets byte-identical). No `--admin`, no rerun.
3. Governance: D-1242 recorded exactly once; D-1243 absent; NA-0623 marked DONE (ENG-0012
   Stage-2a-done, the P1 still open); NA-0624 (Stage 2b) restored as the sole READY successor;
   exactly one `Status: READY`.
4. LIVE QUEUE header updated: STATE `READY=NA-0624 | HIGHEST_NA=0624 | HIGHEST_D=1242`; the stale
   READY-prose pointer (which still named NA-0621) corrected to NA-0624.
5. Boundary: closeout mutates only NEXT_ACTIONS.md, DECISIONS.md, TRACEABILITY.md,
   ROLLING_OPERATIONS_JOURNAL.md, and this testplan; no NA-0624 implementation; no
   source/test/Cargo/spec/workflow/`.claude`/hook change.
6. Private-material scan on all added lines.

## Result

`NA0623_CLOSEOUT_OK`. NA-0623 DONE; ENG-0012 Stage-2a-done (the refimpl SCKA sender core + the
both-sides RK advance land on main; PQ-PCS proven to survive a classical DH ratchet); the P1 stays
open. NA-0624 (Stage 2b: qsc SCKA wiring — the stage that delivers post-quantum PCS on live traffic
and fully closes the P1) sole READY successor; begins at D-1243. The refimpl-vs-spec NHK deviation
is carried forward for Stage 2b / a spec-alignment lane.
