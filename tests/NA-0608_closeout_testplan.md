# NA-0608 Closeout and NA-0609 Restoration Test Plan

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-06

Goals: G1, G2, G3, G4, G5

## Scope

Records class-safe closeout validation for NA-0608 under directive
QSL-DIR-2026-07-06-541 (D541). Closeout consumes D-1209, marks NA-0608 DONE, and
restores NA-0609 as the sole READY successor. It does not implement NA-0609 and
changes no qsc/qsl-server/qsl-attachments source, test, dependency, lockfile, or
workflow; no protocol behavior; no qwork/qstart/qresume execution; no
sudo/systemd/firewall/Tailnet action; and no qscwork/laptop access.

## Required Markers

- NA0608_CLOSEOUT_D1209_CONSUMED_OK
- NA0608_CLOSEOUT_IMPL_PR_1492_MERGED_OK
- NA0608_CLOSEOUT_POSTMERGE_CHECKS_GREEN_OK
- NA0608_CLOSEOUT_MAIN_FASTFORWARD_OK
- NA0608_CLOSEOUT_D1210_RECORDED_ONCE_OK
- NA0608_CLOSEOUT_NA0608_MARKED_DONE_OK
- NA0608_CLOSEOUT_NA0609_RESTORED_SOLE_READY_OK
- NA0608_CLOSEOUT_ONE_READY_INVARIANT_OK
- NA0608_CLOSEOUT_BOUNDARY_MUTATION_OK
- NA0608_CLOSEOUT_NO_SUCCESSOR_IMPLEMENTATION_OK
- NA0608_CLOSEOUT_PRIVATE_MATERIAL_SCAN_OK

## Validation Plan (class-only)

1. Verify implementation PR #1492 merged at `eb63edb8e29a`; local main
   fast-forwarded to origin/main; worktree clean.
2. Verify post-merge required checks green with no failed or pending required
   checks (public-safety, advisories, suite2-vectors, goal-lint, CodeQL).
3. Verify D-1209 once and D-1210 absent before closeout; after patch, D-1210 once.
4. Verify NEXT_ACTIONS marks NA-0608 DONE and NA-0609 READY as the sole READY item.
5. Confirm closeout mutates only the allowed governance paths and implements no
   NA-0609 work; run the no-private-material publication scan.

## Result

NA-0608 DONE; NA-0609 restored as the sole READY successor; D-1210 recorded once.
