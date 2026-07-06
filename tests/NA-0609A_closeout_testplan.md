Goals: G4 (primary), supports G1–G5

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-06

# NA-0609A Closeout Test Plan

## Scope

Records class-safe closeout validation for NA-0609A under directive
QSL-DIR-2026-07-06-542 (D542). Closeout consumes D-1211, marks NA-0609A DONE, and
leaves NA-0609 as the sole READY item. It does not implement NA-0609 and changes
no qsc/qsl-server/qsl-attachments source, test, dependency, lockfile, or workflow;
no protocol behavior; no `.claude/settings.json` or guardrail-hook edit; and no
runtime/LAN action.

## Required Markers

- NA0609A_CLOSEOUT_D1211_CONSUMED_OK
- NA0609A_CLOSEOUT_IMPL_PR_1494_MERGED_OK
- NA0609A_CLOSEOUT_POSTMERGE_CHECKS_GREEN_OK
- NA0609A_CLOSEOUT_MAIN_FASTFORWARD_OK
- NA0609A_CLOSEOUT_D1212_RECORDED_ONCE_OK
- NA0609A_CLOSEOUT_NA0609A_MARKED_DONE_OK
- NA0609A_CLOSEOUT_NA0609_SOLE_READY_OK
- NA0609A_CLOSEOUT_ONE_READY_INVARIANT_OK
- NA0609A_CLOSEOUT_BOUNDARY_MUTATION_OK
- NA0609A_CLOSEOUT_NO_SUCCESSOR_IMPLEMENTATION_OK
- NA0609A_CLOSEOUT_PRIVATE_MATERIAL_SCAN_OK

## Validation Plan (class-only)

1. Verify implementation PR #1494 merged at `a179157be2a5`; local main
   fast-forwarded to origin/main; worktree clean.
2. Verify post-merge required checks green with no failed or pending required
   checks (public-safety, advisories, suite2-vectors, goal-lint, CodeQL).
3. Verify D-1211 once and D-1212 absent before closeout; after patch, D-1212 once.
4. Verify NEXT_ACTIONS marks NA-0609A DONE and NA-0609 READY as the sole READY item.
5. Confirm closeout mutates only the allowed governance paths and implements no
   NA-0609 work; run the no-private-material publication scan.

## Result

NA-0609A DONE; NA-0609 remains the sole READY successor; D-1212 recorded once.
