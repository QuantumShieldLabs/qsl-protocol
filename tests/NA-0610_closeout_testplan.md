Goals: G4 (primary), supports G1, G2, G3, G5

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-07

# NA-0610 Closeout and NA-0611 Restoration Test Plan

## Scope

Records closeout validation for NA-0610 under directive QSL-DIR-2026-07-07-547
(D547). Closeout consumes D-1219, marks NA-0610 DONE, and restores NA-0611 as the
sole READY successor. It does not implement NA-0611 and changes no source/test/
Cargo/workflow/spec/`.claude`/hook.

## Required Markers

- NA0610_CLOSEOUT_D1219_CONSUMED_OK
- NA0610_CLOSEOUT_IMPL_PR_1502_MERGED_OK
- NA0610_CLOSEOUT_POSTMERGE_CHECKS_GREEN_OK
- NA0610_CLOSEOUT_MAIN_FASTFORWARD_OK
- NA0610_CLOSEOUT_D1220_RECORDED_ONCE_OK
- NA0610_CLOSEOUT_NA0610_MARKED_DONE_OK
- NA0610_CLOSEOUT_NA0611_RESTORED_SOLE_READY_OK
- NA0610_CLOSEOUT_ONE_READY_INVARIANT_OK
- NA0610_CLOSEOUT_BOUNDARY_MUTATION_OK
- NA0610_CLOSEOUT_NO_SUCCESSOR_IMPLEMENTATION_OK
- NA0610_CLOSEOUT_PRIVATE_MATERIAL_SCAN_OK

## Validation Plan (class-only)

1. Verify implementation PR #1502 merged at `087ec2816e16`; local main
   fast-forwarded to origin/main; worktree clean.
2. Verify post-merge required checks green with no failed or pending required
   checks (public-safety, advisories, suite2-vectors, goal-lint, CodeQL,
   qsc-adversarial-smoke/miri, ci-4a through ci-4d).
3. Verify D-1219 once and D-1220 absent before closeout; after patch, D-1220 once.
4. Verify NEXT_ACTIONS marks NA-0610 DONE and NA-0611 READY as the sole READY item.
5. Confirm closeout mutates only the allowed governance paths and implements no
   NA-0611 work; run the no-private-material scan.

## Result

NA-0610 DONE; NA-0611 restored as the sole READY successor; D-1220 recorded once.
