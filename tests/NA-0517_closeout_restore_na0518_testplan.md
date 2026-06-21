Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-21

# NA-0517 closeout and NA-0518 restoration testplan

## Purpose

Validate the closeout-only PR that marks NA-0517 DONE and restores the selected NA-0518 successor as the sole READY item. This closeout does not implement NA-0518.

## Scope guard

Allowed changed paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0517_closeout_restore_na0518_testplan.md`

No NA-0518 implementation path may change.

## Required proof checks

- PR #1306 merged at the recorded merge commit.
- Post-merge public-safety for the NA-0517 evidence merge commit completed success inside the short attach/early-failure window.
- D-1023 exists once.
- D-1024 is absent before patch and exists once after patch.
- NA-0517 is marked DONE.
- NA-0518 is restored READY.
- READY_COUNT is exactly 1.
- Duplicate decision ID count is zero.
- Scope guard is exactly the five closeout paths.
- No qsc source/test/fuzz/Cargo mutation.
- No workflow/script/helper/dependency mutation.
- No corpus/vector/input mutation.
- No formal/refimpl/service/public/backup mutation.
- No qsl-server/qsl-attachments mutation.
- No remote action, SSH execution, qsc send/receive, or remote E2EE.
- no public-readiness claim and no production-readiness claim.

## Required markers

- `NA0517_CLOSEOUT_PR1306_MERGED_OK`
- `NA0517_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK`
- `NA0517_CLOSEOUT_D1023_ACCEPTED_OK`
- `NA0517_CLOSEOUT_D1024_RESTORED_NA0518_OK`
- `NA0517_CLOSEOUT_NO_NA0518_IMPLEMENTATION_OK`
- `NA0517_CLOSEOUT_NO_REMOTE_ACTION_OK`
- `NA0517_CLOSEOUT_NO_SSH_EXECUTION_OK`
- `NA0517_CLOSEOUT_NO_QSC_SEND_RECEIVE_OK`
- `NA0517_CLOSEOUT_NO_REMOTE_E2EE_OK`
- `NA0517_CLOSEOUT_NO_QSC_IMPLEMENTATION_MUTATION_OK`
- `NA0517_CLOSEOUT_NO_WORKFLOW_DEPENDENCY_MUTATION_OK`
- `NA0517_CLOSEOUT_NO_CORPUS_VECTOR_INPUT_MUTATION_OK`
- `NA0517_CLOSEOUT_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0517_CLOSEOUT_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0517_CLOSEOUT_ONE_READY_INVARIANT_OK`

## Static validation

Run and record:

- `git diff --cached --check`
- exact five-path closeout scope guard.
- queue and decision proof.
- link-check.
- leak scan.
- added-line overclaim scan.
- classifier.
- marker proof.
- PR body preflight and goal-lint preflight.
- cargo audit for root lockfile.
- cargo audit for qsc fuzz lockfile.

## Post-fix hardening review

1. Correctness under stress: closeout changes only queue/governance state after public-safety is green.
2. Minimality: no NA-0518 implementation is performed.
3. Maintainability: selected successor scope is explicit in NEXT_ACTIONS.
4. Coverage quality: markers distinguish closeout proof from future SSH forwarding authorization proof.
5. Cross-lane stability: qsc runtime and CI/workflow paths remain untouched.
