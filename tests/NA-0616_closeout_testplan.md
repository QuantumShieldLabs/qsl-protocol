Goals: G5 (primary), supports G1, G2, G3, G4

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-07

# NA-0616 Closeout and NA-0617 Restoration Test Plan

## Scope

Records closeout validation for NA-0616 under directive QSL-DIR-2026-07-07-553 (D553).
Closeout consumes D-1227, marks NA-0616 DONE, and restores NA-0617 as the sole READY
successor. It does not implement NA-0617 and changes no source/test/Cargo/workflow/
spec/`.claude`/hook.

## Required Markers

- NA0616_CLOSEOUT_D1227_CONSUMED_OK
- NA0616_CLOSEOUT_IMPL_PR_1510_MERGED_OK
- NA0616_CLOSEOUT_POSTMERGE_CHECKS_GREEN_OK
- NA0616_CLOSEOUT_MAIN_FASTFORWARD_OK
- NA0616_CLOSEOUT_RECOVERED_FAILURE_EVIDENCE_OK
- NA0616_CLOSEOUT_NO_CHECK_BYPASS_OK
- NA0616_CLOSEOUT_D1228_RECORDED_ONCE_OK
- NA0616_CLOSEOUT_NA0616_MARKED_DONE_OK
- NA0616_CLOSEOUT_NA0617_RESTORED_SOLE_READY_OK
- NA0616_CLOSEOUT_ONE_READY_INVARIANT_OK
- NA0616_CLOSEOUT_BOUNDARY_MUTATION_OK
- NA0616_CLOSEOUT_NO_SUCCESSOR_IMPLEMENTATION_OK
- NA0616_CLOSEOUT_PRIVATE_MATERIAL_SCAN_OK

## Validation Plan (class-only)

1. Verify implementation PR #1510 merged at `8167686cdf96`; local main fast-forwarded;
   worktree clean before closeout edits.
2. Verify post-merge required checks green with no failed/pending (public-safety,
   advisories, suite2-vectors, goal-lint, CodeQL, qsc-adversarial-smoke/miri, ci-4a..4d).
3. Recovered-failure evidence: the first public-safety run (`91da0394`) failed
   transiently (main public-safety `in_progress` + a GitHub API 403 on
   branch-protection); classified environmental (35 other checks green); corrective
   action was no bypass / no `--admin`, wait for main to settle green, then an empty
   re-trigger commit `ce8019c8` (identical tree); final result all checks green, merge
   CLEAN.
4. Verify D-1227 once and D-1228 absent before closeout; after patch, D-1228 once and
   D-1229 absent.
5. Verify NEXT_ACTIONS marks NA-0616 DONE and NA-0617 READY as the sole READY item
   (one-READY invariant).
6. Confirm closeout mutates only the allowed governance paths and implements no NA-0617
   work; run the no-private-material scan.

## Result

NA-0616 DONE; NA-0617 restored as the sole READY successor; D-1228 recorded once;
recovered-failure evidence captured (transient public-safety failure resolved without
bypass).
