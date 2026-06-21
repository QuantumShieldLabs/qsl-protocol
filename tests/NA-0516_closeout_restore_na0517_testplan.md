Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-21

# NA-0516 closeout and NA-0517 restoration testplan

## Purpose

Validate that NA-0516 is closed only after the implementation PR merged and
post-merge public-safety completed success, and that the successor is the
transport-remediation authorization lane selected by D-1021.

## Scope guard

Allowed changed paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0516_closeout_restore_na0517_testplan.md`

No qsc source/test/fuzz/Cargo, workflow/script/helper/dependency,
corpus/vector/input, formal/refimpl/service/public/backup, qsl-server,
qsl-attachments, qshield, or qshield-cli path may change.

## Required proof checks

- PR #1304 merged with merge commit `105e793908fd`.
- Post-merge public-safety on `105e793908fd` completed success inside the
  short attach/early-failure window.
- D-1021 exists once.
- D-1022 exists once after closeout patch.
- NA-0516 is DONE.
- READY_COUNT is 1 and READY is NA-0517.
- NA-0517 is authorization-only and does not implement transport remediation.
- No remote action, SSH execution, qsc send/receive, remote E2EE, package
  installation, remote source checkout/build, qwork execution, or qsl-backup
  execution occurs in closeout.
- No public/production/security completion claim is added.

## Required markers

- `NA0516_CLOSEOUT_PR1304_MERGED_OK`
- `NA0516_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK`
- `NA0516_CLOSEOUT_D1021_ACCEPTED_OK`
- `NA0516_CLOSEOUT_D1022_RESTORED_NA0517_OK`
- `NA0516_CLOSEOUT_NO_NA0517_IMPLEMENTATION_OK`
- `NA0516_CLOSEOUT_NO_REMOTE_ACTION_OK`
- `NA0516_CLOSEOUT_NO_SSH_EXECUTION_OK`
- `NA0516_CLOSEOUT_NO_QSC_SEND_RECEIVE_OK`
- `NA0516_CLOSEOUT_NO_REMOTE_E2EE_OK`
- `NA0516_CLOSEOUT_NO_QSC_IMPLEMENTATION_MUTATION_OK`
- `NA0516_CLOSEOUT_NO_WORKFLOW_DEPENDENCY_MUTATION_OK`
- `NA0516_CLOSEOUT_NO_CORPUS_VECTOR_INPUT_MUTATION_OK`
- `NA0516_CLOSEOUT_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0516_CLOSEOUT_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0516_CLOSEOUT_ONE_READY_INVARIANT_OK`

## Static validation

Run and record:

- `git diff --check`
- exact five-path closeout scope guard.
- queue and decision proof.
- changed-file Markdown link check.
- leak scan.
- added-line overclaim scan.
- classifier.
- PR body preflight.
- goal-lint.
- marker proof for this testplan and D-1022.
- root cargo audit.
- nested qsc fuzz lock cargo audit.
- public-safety status proof for PR and post-merge.

## Post-fix hardening review

1. Correctness under stress: closeout accepts the transport failure honestly and
   restores a remediation authorization lane, not an E2EE success lane.
2. Minimality: closeout changes only the five governance/testplan paths.
3. Maintainability: the successor has a narrow transport decision objective and
   keeps implementation out of closeout.
4. Coverage quality: validation checks queue, decision, marker, claim-boundary,
   audit, and public-safety evidence rather than relying on narrative only.
5. Cross-lane stability: qsc source/tests/workflows/dependencies are untouched,
   preserving existing Linux/macOS gates.
