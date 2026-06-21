Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-21

# NA-0514 closeout and NA-0515 restoration testplan

## Objective

Validate that NA-0514 is closed only after the implementation PR merged, D-1017
exists on main, post-merge public-safety is green inside the short attach
window, and NA-0515 is restored as the sole READY successor without
implementing NA-0515.

## Scope checks

Allowed closeout paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0514_closeout_restore_na0515_testplan.md`

Forbidden closeout mutation:

- NA-0515 implementation evidence.
- remote SSH or remote commands.
- qsc send/receive or remote E2EE execution.
- qsc source/test/fuzz/Cargo files.
- workflow/script/helper files.
- dependency/lockfiles.
- corpus/vector/input files.
- formal/refimpl/service/public/backup files.
- qwork/qstart/qresume or qsl-backup mutation.

## Required proof

Pass criteria:

- PR #1300 merged with a merge commit.
- D-1017 exists once on merged main.
- D-1018 is absent before closeout patch.
- post-merge public-safety for the PR #1300 merge commit completed success
  inside the short attach/early-failure window.
- NA-0514 is marked DONE.
- NA-0515 is restored READY.
- READY_COUNT is 1.
- duplicate decision count is zero.
- no remote action occurs in closeout.
- no public-readiness claim is made.
- no production-readiness claim is made.

## Validation commands

Run:

- `git diff --check`
- exact five-path closeout scope guard.
- deterministic relative markdown link check.
- leak scan.
- added-line overclaim scan.
- classifier.
- PR body preflight.
- goal-lint.
- queue and decision proof.

## Expected markers

- `NA0514_CLOSEOUT_PR1300_MERGED_OK`
- `NA0514_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK`
- `NA0514_CLOSEOUT_D1017_ACCEPTED_OK`
- `NA0514_CLOSEOUT_D1018_RESTORED_NA0515_OK`
- `NA0514_CLOSEOUT_NO_NA0515_IMPLEMENTATION_OK`
- `NA0514_CLOSEOUT_NO_REMOTE_ACTION_OK`
- `NA0514_CLOSEOUT_NO_SSH_EXECUTION_OK`
- `NA0514_CLOSEOUT_NO_REMOTE_E2E_OK`
- `NA0514_CLOSEOUT_NO_QSC_IMPLEMENTATION_MUTATION_OK`
- `NA0514_CLOSEOUT_NO_WORKFLOW_DEPENDENCY_MUTATION_OK`
- `NA0514_CLOSEOUT_NO_CORPUS_VECTOR_INPUT_MUTATION_OK`
- `NA0514_CLOSEOUT_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0514_CLOSEOUT_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0514_CLOSEOUT_ONE_READY_INVARIANT_OK`
