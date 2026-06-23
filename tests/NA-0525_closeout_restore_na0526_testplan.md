Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-23

# NA-0525 Closeout and NA-0526 Restoration Testplan

Goals: G1, G2, G3, G4, G5

## Purpose

Validate that NA-0525 closeout consumes the merged stale-retained-qsc implementation evidence from PR #1323, preserves the one-READY queue invariant, and restores NA-0526 as the selected restaging successor without implementing NA-0526.

## Scope

Allowed closeout mutation paths:
- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0525_closeout_restore_na0526_testplan.md`

No NA-0526 implementation, remote action, SSH execution, forwarding setup, qsc send/receive, qsc E2EE, wrong-peer/stale-trust testing, qsc restaging, dependency, lockfile, Cargo.toml, qsc source/test/fuzz source, workflow/script/helper, corpus/vector/input, formal/refimpl/service/public/backup, qsl-server, qsl-attachments, qwork/qstart/qresume, qsl-backup, backup, archive, move, delete, or remote path mutation is allowed.

## Required Proof

- PR #1323 merged with merge commit `2644946204c`.
- D-1040 exists once and records classification `REMOTE_E2EE_RETAINED_QSC_STALE_AFTER_SECURITY_REMEDIATION`.
- Post-merge public-safety completed success on `2644946204c` inside the short attach/early-failure window.
- Post-merge advisories completed success on `2644946204c` inside the short attach/early-failure window.
- D-1041 was absent before closeout and exists once after closeout.
- D-1042 is absent.
- NA-0525 is DONE.
- NA-0526 is READY.
- READY_COUNT is exactly 1.
- NA-0526 successor text matches the restaging implementation harness selected by D-1040.
- Closeout does not implement NA-0526.
- Closeout performs no remote action, SSH execution, forwarding setup, qsc send/receive, qsc E2EE, wrong-peer/stale-trust testing, qsc restaging, dependency mutation, Cargo.lock mutation, Cargo.toml mutation, qsc source/test/fuzz source mutation, workflow/script/helper mutation, corpus/vector/input mutation, formal/refimpl/service/public/backup mutation, qsl-server/qsl-attachments use, qwork/qstart/qresume, qsl-backup execution, backup, or restore.
- Closeout introduces no public-readiness claim, no production-readiness claim, no public-internet-readiness claim, no external-review-complete claim, no crypto-complete claim, no identity-complete claim, no trust-complete claim, no replay-proof claim, no downgrade-proof claim, no secret-material-complete claim, no side-channel-free claim, no vulnerability-free claim, no bug-free claim, and no perfect-crypto claim.

## Required Markers

- `NA0525_CLOSEOUT_PR1323_MERGED_OK`
- `NA0525_CLOSEOUT_D1040_ACCEPTED_OK`
- `NA0525_CLOSEOUT_D1041_RESTORED_NA0526_OK`
- `NA0525_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK`
- `NA0525_CLOSEOUT_ADVISORIES_GREEN_OK`
- `NA0525_CLOSEOUT_NA0525_DONE_OK`
- `NA0525_CLOSEOUT_NA0526_READY_OK`
- `NA0525_CLOSEOUT_NO_NA0526_IMPLEMENTATION_OK`
- `NA0525_CLOSEOUT_NO_REMOTE_ACTION_OK`
- `NA0525_CLOSEOUT_NO_SSH_EXECUTION_OK`
- `NA0525_CLOSEOUT_NO_QSC_SEND_RECEIVE_OK`
- `NA0525_CLOSEOUT_NO_REMOTE_E2EE_OK`
- `NA0525_CLOSEOUT_NO_QSC_RESTAGING_OK`
- `NA0525_CLOSEOUT_NO_QSL_SERVER_ATTACHMENTS_OK`
- `NA0525_CLOSEOUT_NO_QSC_IMPLEMENTATION_MUTATION_OK`
- `NA0525_CLOSEOUT_NO_WORKFLOW_DEPENDENCY_MUTATION_OK`
- `NA0525_CLOSEOUT_NO_CORPUS_VECTOR_INPUT_MUTATION_OK`
- `NA0525_CLOSEOUT_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0525_CLOSEOUT_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0525_CLOSEOUT_ONE_READY_INVARIANT_OK`

## Static Validation

Run:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py queue --select NA-0525 --select NA-0526
python3 scripts/ci/qsl_evidence_helper.py decisions --select D-1040 --select D-1041 --select D-1042
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --head HEAD --allowed-file <allowed-closeout-paths>
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --file <pr-body> --scan-overclaims
bash scripts/ci/classify_ci_scope.sh <changed-paths>
```

Static validation must prove:
- exact five-path closeout scope.
- READY_COUNT 1.
- READY NA-0526.
- NA-0525 DONE.
- D-1040 exists once.
- D-1041 exists once.
- D-1042 is absent.
- duplicate decision count zero.
- checked-in evidence has no private key blocks.
- checked-in evidence has no private key, passphrase, password, token, credential, production endpoint, backup material, qsc vault material, or raw private qsc material.
- added lines introduce no unsupported restricted public/security/completion claims.
- dependency, lockfile, and Cargo.toml paths were not mutated.
- qsl-server and qsl-attachments paths were not mutated.
- qsc source/test/fuzz/Cargo paths were not mutated.
- workflow/script/helper/dependency paths were not mutated.
- corpus/vector/input paths were not mutated.
- formal/refimpl/service/public/backup paths were not mutated.

## Acceptance

This closeout is accepted only if NA-0525 is DONE, NA-0526 is READY, READY_COUNT is 1, D-1041 records the closeout, static governance checks pass, post-closeout public-safety/advisories are green after merge, and no implementation, dependency, lockfile, qsc restaging, or remote-action scope is introduced.

No public-readiness claim is made. No production-readiness claim is made. No public-internet-readiness claim is made. No external-review-complete claim is made. No crypto-complete claim is made. No identity-complete claim is made. No trust-complete claim is made. No replay-proof claim is made. No downgrade-proof claim is made. No secret-material-complete claim is made. No side-channel-free claim is made. No vulnerability-free claim is made. No bug-free claim is made. No perfect-crypto claim is made.
