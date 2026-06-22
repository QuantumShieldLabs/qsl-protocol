Goals: G1, G2, G3, G4, G5

# NA-0521 Closeout and NA-0522 Restoration Testplan

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-22

## Purpose

Validate that NA-0521 closeout consumes the merged implementation evidence from PR #1314, preserves the one-READY queue invariant, and restores NA-0522 as an authorization-only successor without implementing NA-0522.

## Scope

Allowed closeout mutation paths:
- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0521_closeout_restore_na0522_testplan.md`

## Required proof

- PR #1314 merged at `3c1645fe0874`.
- Post-merge public-safety attached and completed success for `3c1645fe0874` inside the short attach/early-failure window.
- D-1031 exists once.
- D-1032 exists once.
- NA-0521 is DONE.
- NA-0522 is READY.
- READY_COUNT is 1.
- No D-1033 exists.
- Changed paths are exactly the five allowed closeout paths.

## Validation commands

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py queue --select NA-0521 --select NA-0522
python3 scripts/ci/qsl_evidence_helper.py decisions --select D-1031 --select D-1032 --select D-1033
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --head HEAD --allowed-file <allowed-closeout-paths>
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --file <pr-body> --scan-overclaims
scripts/ci/classify_ci_scope.sh <changed-paths>
```

## Required markers

- `NA0521_CLOSEOUT_PR1314_MERGED_OK`
- `NA0521_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK`
- `NA0521_CLOSEOUT_D1031_ACCEPTED_OK`
- `NA0521_CLOSEOUT_D1032_RESTORED_NA0522_OK`
- `NA0521_CLOSEOUT_NO_NA0522_IMPLEMENTATION_OK`
- `NA0521_CLOSEOUT_NO_REMOTE_ACTION_OK`
- `NA0521_CLOSEOUT_NO_QSC_SEND_RECEIVE_OK`
- `NA0521_CLOSEOUT_NO_QSL_SERVER_ATTACHMENTS_OK`
- `NA0521_CLOSEOUT_NO_QSC_IMPLEMENTATION_MUTATION_OK`
- `NA0521_CLOSEOUT_NO_WORKFLOW_DEPENDENCY_MUTATION_OK`
- `NA0521_CLOSEOUT_NO_CORPUS_VECTOR_INPUT_MUTATION_OK`
- `NA0521_CLOSEOUT_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0521_CLOSEOUT_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0521_CLOSEOUT_ONE_READY_INVARIANT_OK`

## Acceptance

This closeout is accepted only if NA-0521 is DONE, NA-0522 is READY, READY_COUNT is 1, D-1032 records the closeout, static governance checks pass, and no implementation or remote-action scope is introduced.

No public-readiness claim is made. No production-readiness claim is made. No public-internet-readiness claim is made. No external-review-complete claim is made. No crypto-complete claim is made. No replay-proof claim is made. No downgrade-proof claim is made. No secret-material-complete claim is made. No side-channel-free claim is made. No vulnerability-free, bug-free, or perfect-crypto claim is made.
