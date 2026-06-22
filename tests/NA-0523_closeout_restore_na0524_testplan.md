Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-22

# NA-0523 Closeout and NA-0524 Restoration Testplan

## Purpose

Validate that NA-0523 closeout consumes the merged implementation evidence from PR #1318, preserves the one-READY queue invariant, and restores NA-0524 as an authorization-only successor without implementing NA-0524.

## Scope

Allowed closeout mutation paths:
- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0523_closeout_restore_na0524_testplan.md`

No qsc source/test/fuzz/Cargo, workflow/script/helper, dependency/lockfile, corpus/vector/input, formal/refimpl/service/public/backup, qsl-server, qsl-attachments, qshield, qshield-cli, public docs, website, qsl-backup, backup, archive, move, delete, or remote path mutation is allowed.

## Required Proof

- PR #1318 merged with merge commit `2747e756fa34`.
- Post-merge public-safety on `2747e756fa34` completed success inside the short attach/early-failure window.
- D-1035 exists once.
- D-1036 exists once.
- NA-0523 is DONE.
- NA-0524 is READY.
- READY_COUNT is exactly 1.
- NA-0524 successor text matches the wrong-peer / stale-trust authorization plan selected by D-1035.
- Closeout does not implement NA-0524.
- Closeout performs no remote action, SSH execution, qsc send/receive, remote E2EE, wrong-peer/stale-trust negative execution, qsl-server/qsl-attachments use, package installation, sudo/admin action, key/config/host mutation, qwork/qstart/qresume, qsl-backup execution, qsc source/test/fuzz/Cargo mutation, workflow/script/helper/dependency mutation, corpus/vector/input mutation, or formal/refimpl/service/public/backup mutation.
- Closeout introduces no public-readiness claim, no production-readiness claim, no public-internet-readiness claim, no external-review-complete claim, no crypto-complete claim, no replay-proof claim, no downgrade-proof claim, no secret-material-complete claim, no side-channel-free claim, no vulnerability-free claim, no bug-free claim, and no perfect-crypto claim.

## Required Markers

- `NA0523_CLOSEOUT_PR1318_MERGED_OK`
- `NA0523_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK`
- `NA0523_CLOSEOUT_D1035_ACCEPTED_OK`
- `NA0523_CLOSEOUT_D1036_RESTORED_NA0524_OK`
- `NA0523_CLOSEOUT_NO_NA0524_IMPLEMENTATION_OK`
- `NA0523_CLOSEOUT_NO_REMOTE_ACTION_OK`
- `NA0523_CLOSEOUT_NO_SSH_EXECUTION_OK`
- `NA0523_CLOSEOUT_NO_QSC_SEND_RECEIVE_OK`
- `NA0523_CLOSEOUT_NO_REMOTE_E2EE_OK`
- `NA0523_CLOSEOUT_NO_QSL_SERVER_ATTACHMENTS_OK`
- `NA0523_CLOSEOUT_NO_QSC_IMPLEMENTATION_MUTATION_OK`
- `NA0523_CLOSEOUT_NO_WORKFLOW_DEPENDENCY_MUTATION_OK`
- `NA0523_CLOSEOUT_NO_CORPUS_VECTOR_INPUT_MUTATION_OK`
- `NA0523_CLOSEOUT_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0523_CLOSEOUT_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0523_CLOSEOUT_ONE_READY_INVARIANT_OK`

## Static Validation

Run:

```bash
git diff --check
bash scripts/ci/classify_ci_scope.sh NEXT_ACTIONS.md DECISIONS.md TRACEABILITY.md docs/ops/ROLLING_OPERATIONS_JOURNAL.md tests/NA-0523_closeout_restore_na0524_testplan.md
```

Static validation must prove:
- exact five-path closeout scope.
- READY_COUNT 1.
- READY NA-0524.
- NA-0523 DONE.
- D-1035 exists once.
- D-1036 exists once.
- duplicate decision count zero.
- checked-in evidence has no private key blocks.
- checked-in evidence has no private key, passphrase, password, token, credential, production endpoint, backup material, qsc vault material, or raw private qsc material.
- added lines introduce no unsupported restricted public/security/completion claims.
- qsl-server and qsl-attachments paths were not mutated.
- qsc source/test/fuzz/Cargo paths were not mutated.
- workflow/script/helper/dependency paths were not mutated.
- corpus/vector/input paths were not mutated.
- formal/refimpl/service/public/backup paths were not mutated.

## Acceptance

This closeout is accepted only if NA-0523 is DONE, NA-0524 is READY, READY_COUNT is 1, D-1036 records the closeout, static governance checks pass, and no implementation or remote-action scope is introduced.

No public-readiness claim is made. No production-readiness claim is made. No public-internet-readiness claim is made. No external-review-complete claim is made. No crypto-complete claim is made. No replay-proof claim is made. No downgrade-proof claim is made. No secret-material-complete claim is made. No side-channel-free claim is made. No vulnerability-free claim is made. No bug-free claim is made. No perfect-crypto claim is made.
