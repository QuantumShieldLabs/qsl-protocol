Goals: G5 (primary), supports G1, G2, G3, G4

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-07

# NA-0609 Closeout and NA-0610 Restoration Test Plan

## Scope

Records class-safe closeout validation for NA-0609 under directive
QSL-DIR-2026-07-06-546 (D546). Closeout consumes D-1217, marks NA-0609 DONE, and
restores NA-0610 as the sole READY successor. It does not implement NA-0610 and
changes no source/test/Cargo/workflow/spec/`.claude`/hook.

## Required Markers

- NA0609_CLOSEOUT_D1217_CONSUMED_OK
- NA0609_CLOSEOUT_IMPL_PR_1500_MERGED_OK
- NA0609_CLOSEOUT_POSTMERGE_CHECKS_GREEN_OK
- NA0609_CLOSEOUT_MAIN_FASTFORWARD_OK
- NA0609_CLOSEOUT_D1218_RECORDED_ONCE_OK
- NA0609_CLOSEOUT_NA0609_MARKED_DONE_OK
- NA0609_CLOSEOUT_NA0610_RESTORED_SOLE_READY_OK
- NA0609_CLOSEOUT_ONE_READY_INVARIANT_OK
- NA0609_CLOSEOUT_BOUNDARY_MUTATION_OK
- NA0609_CLOSEOUT_NO_SUCCESSOR_IMPLEMENTATION_OK
- NA0609_CLOSEOUT_PRIVATE_MATERIAL_SCAN_OK

## Validation Plan (class-only)

1. Verify implementation PR #1500 merged at `c4b9f8e34382`; local main
   fast-forwarded to origin/main; worktree clean.
2. Verify post-merge required checks green with no failed or pending required
   checks (public-safety, advisories, suite2-vectors, goal-lint, CodeQL).
3. Verify D-1217 once and D-1218 absent before closeout; after patch, D-1218 once.
4. Verify NEXT_ACTIONS marks NA-0609 DONE and NA-0610 READY as the sole READY item.
5. Confirm closeout mutates only the allowed governance paths and implements no
   NA-0610 work; run the no-private-material scan.

## Result

NA-0609 DONE; NA-0610 restored as the sole READY successor; D-1218 recorded once.
