Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-22

# NA-0522 Closeout and NA-0523 Restoration Testplan

## Purpose

Validate that NA-0522 closeout consumes the merged authorization evidence, marks NA-0522 DONE, restores NA-0523 as the sole READY successor, and preserves closeout-only scope without implementing NA-0523.

## Scope guard

Allowed mutation paths:
- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0522_closeout_restore_na0523_testplan.md`

No qsc source/test/fuzz/Cargo, workflow/script/helper, dependency/lockfile, corpus/vector/input, formal/refimpl/service/public/backup, qsl-server, qsl-attachments, qshield, qshield-cli, public docs, website, qsl-backup, backup, archive, move, delete, or remote path mutation is allowed.

## Required evidence checks

- PR #1316 merged with merge commit `17894693df6f`.
- Post-merge public-safety on `17894693df6f` completed success inside the short attach/early-failure window.
- D-1033 exists once.
- D-1034 exists once.
- NA-0522 is DONE.
- NA-0523 is READY.
- READY_COUNT is exactly 1.
- NA-0523 successor text matches the replay/corrupt delivery negative boundary implementation selected by D-1033.
- Closeout does not implement NA-0523.
- Closeout performs no remote action, SSH execution, qsc send/receive, remote E2EE, replay/corrupt negative execution, qsl-server/qsl-attachments use, package installation, sudo/admin action, key/config/host mutation, qwork/qstart/qresume, qsl-backup execution, qsc source/test/fuzz/Cargo mutation, workflow/script/helper/dependency mutation, corpus/vector/input mutation, or formal/refimpl/service/public/backup mutation.
- Closeout introduces no public-readiness, production-readiness, public-internet-readiness, external-review-complete, crypto-complete, replay-proof, downgrade-proof, secret-material-complete, side-channel-free, vulnerability-free, bug-free, or perfect-crypto claim.

## Required markers

Evidence or validation proof must contain:

- `NA0522_CLOSEOUT_PR1316_MERGED_OK`
- `NA0522_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK`
- `NA0522_CLOSEOUT_D1033_ACCEPTED_OK`
- `NA0522_CLOSEOUT_D1034_RESTORED_NA0523_OK`
- `NA0522_CLOSEOUT_NO_NA0523_IMPLEMENTATION_OK`
- `NA0522_CLOSEOUT_NO_REMOTE_ACTION_OK`
- `NA0522_CLOSEOUT_NO_SSH_EXECUTION_OK`
- `NA0522_CLOSEOUT_NO_QSC_SEND_RECEIVE_OK`
- `NA0522_CLOSEOUT_NO_REMOTE_E2EE_OK`
- `NA0522_CLOSEOUT_NO_QSL_SERVER_ATTACHMENTS_OK`
- `NA0522_CLOSEOUT_NO_QSC_IMPLEMENTATION_MUTATION_OK`
- `NA0522_CLOSEOUT_NO_WORKFLOW_DEPENDENCY_MUTATION_OK`
- `NA0522_CLOSEOUT_NO_CORPUS_VECTOR_INPUT_MUTATION_OK`
- `NA0522_CLOSEOUT_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0522_CLOSEOUT_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0522_CLOSEOUT_ONE_READY_INVARIANT_OK`

## Static validation

Run:

```bash
git diff --check
bash scripts/ci/classify_ci_scope.sh NEXT_ACTIONS.md DECISIONS.md TRACEABILITY.md docs/ops/ROLLING_OPERATIONS_JOURNAL.md tests/NA-0522_closeout_restore_na0523_testplan.md
```

Static validation must prove:
- exact five-path closeout scope.
- READY_COUNT 1.
- READY NA-0523.
- NA-0522 DONE.
- D-1033 exists once.
- D-1034 exists once.
- duplicate decision count zero.
- checked-in evidence has no private key blocks.
- checked-in evidence has no private key, passphrase, password, token, credential, production endpoint, backup material, qsc vault material, or raw private qsc material.
- added lines introduce no unsupported public, production, internet, external-review, crypto-completion, replay-completion, downgrade-completion, side-channel, vulnerability-free, bug-free, or perfect-crypto claims.
- qsl-server and qsl-attachments paths were not mutated.
- qsc source/test/fuzz/Cargo paths were not mutated.
- workflow/script/helper/dependency paths were not mutated.
- corpus/vector/input paths were not mutated.
- formal/refimpl/service/public/backup paths were not mutated.

## Acceptance classification

Expected closeout classification:

`NA0522_CLOSEOUT_D1034_RESTORED_NA0523_OK`

## Boundaries

This testplan does not authorize NA-0523 implementation. It does not authorize qsl-server, qsl-attachments, package installation, public service deployment, remote source checkout/build, qwork/qstart/qresume, qsl-backup execution, qsc source/test/fuzz/Cargo mutation, workflow/script/helper mutation, dependency mutation, corpus/vector/input mutation, formal/refimpl/service/public/backup mutation, remote E2EE execution, replay/corrupt negative execution, or any public/production readiness claim.

