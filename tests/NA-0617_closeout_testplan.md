Goals: G4 (primary), supports G1, G2, G3, G5

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-07

# NA-0617 Closeout, Audit-Finding Ledger Intake, and NA-0618 Restoration Test Plan

## Scope

Records closeout validation for NA-0617 under directive QSL-DIR-2026-07-07-554 (D554).
Closeout consumes D-1229, marks NA-0617 DONE, files the external Suite-2 review findings
into the improvement ledger (WF-0003 Director triage), and restores NA-0618 as the sole
READY successor. It does not implement NA-0618 and changes no source/test/Cargo/workflow/
spec/`.claude`/hook.

## Required Markers

- NA0617_CLOSEOUT_D1229_CONSUMED_OK
- NA0617_CLOSEOUT_IMPL_PR_1512_MERGED_OK
- NA0617_CLOSEOUT_POSTMERGE_CHECKS_GREEN_OK
- NA0617_CLOSEOUT_MAIN_FASTFORWARD_OK
- NA0617_CLOSEOUT_NO_CHECK_BYPASS_OK
- NA0617_CLOSEOUT_D1230_RECORDED_ONCE_OK
- NA0617_CLOSEOUT_NA0617_MARKED_DONE_OK
- NA0617_CLOSEOUT_LEDGER_INTAKE_ENG0012_TO_ENG0018_WF0010_OK
- NA0617_CLOSEOUT_CROSS_REPO_OWNERSHIP_TBD_NOTED_OK
- NA0617_CLOSEOUT_NA0618_RESTORED_SOLE_READY_OK
- NA0617_CLOSEOUT_ONE_READY_INVARIANT_OK
- NA0617_CLOSEOUT_D1231_ABSENT_OK
- NA0617_CLOSEOUT_BOUNDARY_MUTATION_OK
- NA0617_CLOSEOUT_NO_SUCCESSOR_IMPLEMENTATION_OK
- NA0617_CLOSEOUT_PRIVATE_MATERIAL_SCAN_OK

## Validation Plan (class-only)

1. Verify implementation PR #1512 merged via a merge commit at `d597afa0`; local main
   fast-forwarded to `d597afa0`; the fix and D-1229 present on main; worktree clean before
   closeout edits.
2. Verify post-merge required checks green with no failed/pending (public-ci/public-safety,
   advisories, goal-lint, suite2-ci, qsc-adversarial smoke+miri, macos-build, formal-ci,
   qshield-ci, demo-packaging, CodeQL, ci-4a..ci-4d, classify/scope gates); merge CLEAN.
   No recovered-failure evidence required (no transient failure occurred on PR #1512).
3. No-bypass: no `--admin` merge, no `gh run rerun`, no workflow dispatch; merge on CLEAN
   with all required checks green.
4. Governance: D-1230 recorded exactly once; D-1231 absent; NA-0617 marked DONE with an
   outcome note; NA-0618 (ENG-0013) restored as the sole READY successor; exactly one READY.
5. Ledger intake (WF-0003): ENG-0012..ENG-0018 and WF-0010 present exactly once each;
   ENG-0002 marked done; ENG-0014 (qsl-server) and ENG-0012's multi-repo implementation
   recorded as cross-repo with driving-queue ownership TBD.
6. Boundary: closeout mutates only NEXT_ACTIONS.md, DECISIONS.md, IMPROVEMENT_LEDGER.md,
   TRACEABILITY.md, ROLLING_OPERATIONS_JOURNAL.md, and this testplan; no NA-0618
   implementation; no source/test/Cargo/workflow/spec/`.claude`/hook change; no
   operator-startup-command execution; no runtime/LAN action.
7. Private-material scan on all added lines (class-only; no endpoints, tokens,
   capabilities, keys, plaintext, ciphertext bodies, seeds, or raw private command lines).

## Result

`NA0617_CLOSEOUT_OK`. NA-0617 DONE; ENG-0002 resolved (fixed); Suite-2 audit findings filed
(ENG-0012..0018, WF-0010); NA-0618 (ENG-0013) sole READY successor; begins at D-1231.
