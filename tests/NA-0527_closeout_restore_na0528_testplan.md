Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-23

# NA-0527 Closeout and NA-0528 Restoration Testplan

## Objective

Validate closeout of NA-0527 after D-1044 classified the implementation run as `REMOTE_E2EE_FORWARDING_RECHECK_FAILURE`, PR #1327 merged, and post-merge public-safety/advisories completed success. Restore the selected NA-0528 reverse-forwarding diagnostic / retry scope authorization plan as the sole READY successor.

## Scope

Allowed checked-in paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0527_closeout_restore_na0528_testplan.md`

No NA-0528 implementation, remote action, SSH execution, forwarding probe, qsc send/receive, remote E2EE, qsl-server/qsl-attachments use, package installation, key/config/host mutation, qwork/qstart/qresume, qsl-backup execution, qsc source/test/fuzz/Cargo mutation, workflow/script/helper/dependency mutation, corpus/vector/input mutation, or formal/refimpl/service/public/backup mutation may occur in closeout.

## Expected evidence

- PR #1327 merged with merge commit `7743e3926a52`.
- Post-merge public-safety completed success.
- Post-merge advisories completed success.
- D-1044 exists once.
- D-1045 exists once after closeout patch.
- D-1046 is absent.
- NA-0527 is DONE.
- NA-0528 is READY.
- READY_COUNT is 1.
- Duplicate decision count is zero.
- NA-0528 block is the exact selected remediation successor for forwarding diagnostics.
- No qsc send/receive, remote E2EE, qsl-server, qsl-attachments, remote action, qwork/qstart/qresume, or qsl-backup execution occurs in closeout.

## Static validation

Run:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main \
  --allowed NEXT_ACTIONS.md \
  --allowed DECISIONS.md \
  --allowed TRACEABILITY.md \
  --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md \
  --allowed tests/NA-0527_closeout_restore_na0528_testplan.md
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
bash scripts/ci/classify_ci_scope.sh \
  NEXT_ACTIONS.md \
  DECISIONS.md \
  TRACEABILITY.md \
  docs/ops/ROLLING_OPERATIONS_JOURNAL.md \
  tests/NA-0527_closeout_restore_na0528_testplan.md
```

Expected: scope guard reports only the five closeout paths, link-check passes, leak-scan reports zero added-line findings, and classifier accepts docs/governance-only scope.

## Queue and decision validation

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue --select NA-0527 --select NA-0528
python3 scripts/ci/qsl_evidence_helper.py decisions --select D-1044 --select D-1045 --select D-1046
```

Expected:

- READY_COUNT 1.
- NA-0527 DONE.
- NA-0528 READY.
- D-1044 count 1.
- D-1045 count 1.
- D-1046 count 0.
- DUPLICATE_COUNT 0.

## Marker validation

Closeout evidence must contain:

```text
NA0527_CLOSEOUT_PR1327_MERGED_OK
NA0527_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK
NA0527_CLOSEOUT_ADVISORIES_GREEN_OK
NA0527_CLOSEOUT_D1044_ACCEPTED_OK
NA0527_CLOSEOUT_D1045_RESTORED_NA0528_OK
NA0527_CLOSEOUT_NO_NA0528_IMPLEMENTATION_OK
NA0527_CLOSEOUT_ONE_READY_INVARIANT_OK
```

## Boundary assertions

- No NA-0528 implementation.
- No remote action.
- No SSH execution.
- No qsc send/receive.
- No remote E2EE.
- No qsl-server use.
- No qsl-attachments use.
- No package installation.
- No qwork/qstart/qresume execution.
- No qsl-backup execution.
- No qsc source/test/fuzz/Cargo mutation.
- No workflow/script/helper/dependency mutation.
- No corpus/vector/input mutation.
- No formal/refimpl/service/public/backup mutation.
- No public-readiness claim.
- No production-readiness claim.
- No public-internet-readiness claim.
- No external-review-complete claim.
- No crypto-complete claim.
- No identity-complete claim.
- No trust-complete claim.
- No replay-proof claim.
- No downgrade-proof claim.
- No secret-material-complete claim.
- No side-channel-free claim.
- No vulnerability-free, bug-free, or perfect-crypto claim.
