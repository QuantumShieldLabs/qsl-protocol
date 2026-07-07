Goals: G5 (primary), supports G1, G2, G3, G4

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-07

# NA-0614 Closeout and NA-0615 Restoration Test Plan

## Scope

Records closeout validation for NA-0614 under directive QSL-DIR-2026-07-07-551 (D551).
Closeout consumes D-1224, marks NA-0614 DONE, and restores NA-0615 as the sole READY
successor. It does not implement NA-0615 and changes no source/test/Cargo/workflow/
spec/`.claude`/hook.

## Required Markers

- NA0614_CLOSEOUT_D1224_CONSUMED_OK
- NA0614_CLOSEOUT_IMPL_PR_1507_MERGED_OK
- NA0614_CLOSEOUT_POSTMERGE_CHECKS_GREEN_OK
- NA0614_CLOSEOUT_MAIN_FASTFORWARD_OK
- NA0614_CLOSEOUT_D1225_RECORDED_ONCE_OK
- NA0614_CLOSEOUT_NA0614_MARKED_DONE_OK
- NA0614_CLOSEOUT_NA0615_RESTORED_SOLE_READY_OK
- NA0614_CLOSEOUT_ONE_READY_INVARIANT_OK
- NA0614_CLOSEOUT_BOUNDARY_MUTATION_OK
- NA0614_CLOSEOUT_NO_SUCCESSOR_IMPLEMENTATION_OK
- NA0614_CLOSEOUT_DESIGN_TENET_PERSISTED_OK
- NA0614_CLOSEOUT_PRIVATE_MATERIAL_SCAN_OK

## Validation Plan (class-only)

1. Verify implementation PR #1507 merged at `cbea1c7db67f`; local main fast-forwarded;
   worktree clean.
2. Verify post-merge required checks green with no failed/pending (public-safety,
   advisories, suite2-vectors, goal-lint, CodeQL, qsc-adversarial-smoke/miri, ci-4a..4d).
3. Verify D-1224 once and D-1225 absent before closeout; after patch, D-1225 once.
4. Verify NEXT_ACTIONS marks NA-0614 DONE and NA-0615 READY as the sole READY item.
5. Confirm closeout mutates only the allowed governance paths and implements no NA-0615
   work; the design tenet is persisted to memory; run the no-private-material scan.

## Result

NA-0614 DONE; NA-0615 restored as the sole READY successor; D-1225 recorded once.
