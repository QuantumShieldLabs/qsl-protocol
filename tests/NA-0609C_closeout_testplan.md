Goals: G4 (primary), supports G1, G2, G3, G5

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-06

# NA-0609C Closeout Test Plan

## Scope

Records class-safe closeout validation for NA-0609C under directive
QSL-DIR-2026-07-06-544 (D544). Closeout consumes D-1214, marks NA-0609C DONE, and
leaves NA-0609 as the sole READY item. It does not implement NA-0609 and changes
no source, test, Cargo, workflow, spec, `.claude`, or guardrail-hook file.

## Required Markers

- NA0609C_CLOSEOUT_D1214_CONSUMED_OK
- NA0609C_CLOSEOUT_IMPL_PR_1497_MERGED_OK
- NA0609C_CLOSEOUT_POSTMERGE_CHECKS_GREEN_OK
- NA0609C_CLOSEOUT_MAIN_FASTFORWARD_OK
- NA0609C_CLOSEOUT_D1215_RECORDED_ONCE_OK
- NA0609C_CLOSEOUT_NA0609C_MARKED_DONE_OK
- NA0609C_CLOSEOUT_NA0609_SOLE_READY_OK
- NA0609C_CLOSEOUT_ONE_READY_INVARIANT_OK
- NA0609C_CLOSEOUT_BOUNDARY_MUTATION_OK
- NA0609C_CLOSEOUT_PRIVATE_MATERIAL_SCAN_OK

## Validation Plan (class-only)

1. Verify implementation PR #1497 merged at `1c1c808fbb2e`; local main
   fast-forwarded to origin/main; worktree clean.
2. Verify post-merge required checks green with no failed or pending required
   checks (public-safety, advisories, suite2-vectors, goal-lint, CodeQL,
   qsc-adversarial-smoke/miri, ci-4a through ci-4d).
3. Verify D-1214 once and D-1215 absent before closeout; after patch, D-1215 once.
4. Verify NEXT_ACTIONS marks NA-0609C DONE and NA-0609 READY as the sole READY item.
5. Confirm closeout mutates only the allowed governance paths; run the
   no-private-material scan.

## Result

NA-0609C DONE; NA-0609 remains the sole READY successor; D-1215 recorded once;
ledger ENG-0003 closed.
