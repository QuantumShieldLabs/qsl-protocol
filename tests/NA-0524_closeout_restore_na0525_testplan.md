Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-23

# NA-0524 Closeout and NA-0525 Restoration Testplan

## Purpose

Validate that NA-0524 closeout consumes the merged authorization evidence from PR #1320, the merged dual-lockfile remediation evidence from PR #1321, and the post-remediation green advisory gate before marking NA-0524 DONE and restoring NA-0525 as the sole READY successor.

## Scope

Allowed closeout mutation paths:
- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0524_closeout_restore_na0525_testplan.md`

No dependency, lockfile, Cargo.toml, qsc source/test/fuzz source, workflow/script/helper, corpus/vector/input, formal/refimpl/service/public/backup, qsl-server, qsl-attachments, qshield, qshield-cli, public docs, website, qsl-backup, backup, archive, move, delete, or remote path mutation is allowed.

## Required Proof

- qwork proof files exist, are read without rerunning qwork, and match live pre-fetch HEAD/origin-main.
- PR #1320 merged with merge commit `d309fd9d10c4`.
- D-1037 exists once.
- PR #1321 merged with merge commit `085806b8e79b`.
- D-1038 exists once.
- D-1039 was absent before closeout and exists once after closeout.
- D-1040 is absent.
- D420/D421/D422 inheritance is consumed.
- D422 late recheck and current main recheck prove advisories completed success.
- D422 late recheck and current main recheck prove public-safety completed success.
- Root `Cargo.lock` contains `quinn-proto 0.11.15`.
- Nested `qsl/qsl-client/qsc/fuzz/Cargo.lock` contains `quinn-proto 0.11.15`.
- NA-0524 is DONE.
- NA-0525 is READY.
- READY_COUNT is exactly 1.
- NA-0525 successor text matches the wrong-peer / stale-trust implementation harness selected by D-1037.
- Closeout does not implement NA-0525.
- Closeout performs no dependency mutation, Cargo.lock mutation, Cargo.toml mutation, qsc source/test/fuzz source mutation, workflow/script/helper mutation, corpus/vector/input mutation, formal/refimpl/service/public/backup mutation, qsl-server/qsl-attachments use, remote action, SSH execution, qsc send/receive, remote E2EE, qwork/qstart/qresume, qsl-backup execution, backup, or restore.
- Closeout introduces no public-readiness claim, no production-readiness claim, no public-internet-readiness claim, no external-review-complete claim, no crypto-complete claim, no identity-complete claim, no trust-complete claim, no replay-proof claim, no downgrade-proof claim, no secret-material-complete claim, no side-channel-free claim, no vulnerability-free claim, no bug-free claim, and no perfect-crypto claim.

## Required Markers

- `NA0524_CLOSEOUT_PR1320_MERGED_OK`
- `NA0524_CLOSEOUT_PR1321_MERGED_OK`
- `NA0524_CLOSEOUT_D1037_ACCEPTED_OK`
- `NA0524_CLOSEOUT_D1038_ACCEPTED_OK`
- `NA0524_CLOSEOUT_D1039_RESTORED_NA0525_OK`
- `NA0524_CLOSEOUT_ADVISORIES_GREEN_OK`
- `NA0524_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK`
- `NA0524_CLOSEOUT_ROOT_QUINN_PROTO_0_11_15_OK`
- `NA0524_CLOSEOUT_NESTED_QUINN_PROTO_0_11_15_OK`
- `NA0524_CLOSEOUT_NO_DEPENDENCY_MUTATION_OK`
- `NA0524_CLOSEOUT_NO_LOCKFILE_MUTATION_OK`
- `NA0524_CLOSEOUT_NO_NA0525_IMPLEMENTATION_OK`
- `NA0524_CLOSEOUT_NO_REMOTE_ACTION_OK`
- `NA0524_CLOSEOUT_NO_SSH_EXECUTION_OK`
- `NA0524_CLOSEOUT_NO_QSC_SEND_RECEIVE_OK`
- `NA0524_CLOSEOUT_NO_REMOTE_E2EE_OK`
- `NA0524_CLOSEOUT_NO_QSL_SERVER_ATTACHMENTS_OK`
- `NA0524_CLOSEOUT_NO_QSC_IMPLEMENTATION_MUTATION_OK`
- `NA0524_CLOSEOUT_NO_WORKFLOW_DEPENDENCY_MUTATION_OK`
- `NA0524_CLOSEOUT_NO_CORPUS_VECTOR_INPUT_MUTATION_OK`
- `NA0524_CLOSEOUT_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0524_CLOSEOUT_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0524_CLOSEOUT_ONE_READY_INVARIANT_OK`

## Static Validation

Run:

```bash
git diff --check
bash scripts/ci/classify_ci_scope.sh NEXT_ACTIONS.md DECISIONS.md TRACEABILITY.md docs/ops/ROLLING_OPERATIONS_JOURNAL.md tests/NA-0524_closeout_restore_na0525_testplan.md
```

Static validation must prove:
- exact five-path closeout scope.
- READY_COUNT 1.
- READY NA-0525.
- NA-0524 DONE.
- D-1037 exists once.
- D-1038 exists once.
- D-1039 exists once.
- D-1040 is absent.
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

This closeout is accepted only if NA-0524 is DONE, NA-0525 is READY, READY_COUNT is 1, D-1039 records the closeout, static governance checks pass, root and nested cargo audits remain green, post-closeout public-safety/advisories are green after merge, and no implementation, dependency, lockfile, or remote-action scope is introduced.

No public-readiness claim is made. No production-readiness claim is made. No public-internet-readiness claim is made. No external-review-complete claim is made. No crypto-complete claim is made. No identity-complete claim is made. No trust-complete claim is made. No replay-proof claim is made. No downgrade-proof claim is made. No secret-material-complete claim is made. No side-channel-free claim is made. No vulnerability-free claim is made. No bug-free claim is made. No perfect-crypto claim is made.
