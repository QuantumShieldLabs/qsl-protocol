Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-23

# NA-0526 Closeout and NA-0527 Restoration Testplan

Goals: G1, G2, G3, G4, G5

## Purpose

Validate that NA-0526 closeout consumes the merged retained-qsc restaging evidence from PR #1325, preserves the one-READY queue invariant, and restores NA-0527 as the selected wrong-peer / stale-trust retry-after-restaging successor without implementing NA-0527.

## Scope

Allowed closeout mutation paths:
- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0526_closeout_restore_na0527_testplan.md`

No NA-0527 implementation, remote action, SSH execution, scp, sftp, rsync, forwarding setup, qsc send/receive, qsc E2EE, wrong-peer/stale-trust testing, qsc restaging, dependency, lockfile, Cargo.toml, qsc source/test/fuzz source, workflow/script/helper, corpus/vector/input, formal/refimpl/service/public/backup, qsl-server, qsl-attachments, qwork/qstart/qresume, qsl-backup, backup, archive, move, delete, or remote path mutation is allowed.

## Required Proof

- qwork proof files were read from `/srv/qbuild/work/NA-0526/.qwork/` and qwork/qstart/qresume were not run by closeout.
- PR #1325 merged with merge commit `d12385252c3d`.
- D-1042 exists once and records classification `REMOTE_PREBUILT_QSC_RESTAGING_AFTER_SECURITY_REMEDIATION_PASS_RETAINED`.
- D425 response exists and records that optional closeout did not run because post-merge public-safety was queued at the end of the short attach/early-failure window.
- Current recheck proves public-safety completed success on `d12385252c3d`.
- Current recheck proves advisories completed success on `d12385252c3d`.
- Old stale retained qsc hash is `6f12ab5eec2468a4146012dfc247cef15be2337cb8e8e99cefa96a8c258d91ea`.
- New retained remote qsc hash is `6bf9e59fdae397c2e0f88538d700cccbee80d229c6a979cc79555e39fea2b4f7`.
- Retained remote qsc path is `$HOME/qsl-remote-test/bin/qsc`.
- D425 evidence records final owner `qslcodex`, mode `700`, size `102103920`, and remote `qsc --help` success.
- D-1043 was absent before closeout and exists once after closeout.
- D-1044 remains absent.
- NA-0526 is DONE.
- NA-0527 is READY.
- READY_COUNT is exactly 1.
- NA-0527 successor text matches the D-1042-selected retry-after-restaging implementation harness.
- Closeout performs no remote action, SSH execution, scp, sftp, rsync, forwarding setup, qsc send/receive, qsc E2EE, wrong-peer/stale-trust testing, qsc restaging, dependency mutation, Cargo.lock mutation, Cargo.toml mutation, qsc source/test/fuzz source mutation, workflow/script/helper mutation, corpus/vector/input mutation, formal/refimpl/service/public/backup mutation, qsl-server/qsl-attachments use, qwork/qstart/qresume, qsl-backup execution, backup, or restore.
- Closeout introduces no public-readiness claim, no production-readiness claim, no public-internet-readiness claim, no external-review-complete claim, no crypto-complete claim, no identity-complete claim, no trust-complete claim, no replay-proof claim, no downgrade-proof claim, no secret-material-complete claim, no side-channel-free claim, no vulnerability-free claim, no bug-free claim, and no perfect-crypto claim.

## Required Markers

- `NA0526_CLOSEOUT_PR1325_MERGED_OK`
- `NA0526_CLOSEOUT_D1042_ACCEPTED_OK`
- `NA0526_CLOSEOUT_D1043_RESTORED_NA0527_OK`
- `NA0526_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK`
- `NA0526_CLOSEOUT_ADVISORIES_GREEN_OK`
- `NA0526_CLOSEOUT_RETAINED_QSC_HASH_ACCEPTED_OK`
- `NA0526_CLOSEOUT_OLD_STALE_HASH_RECORDED_OK`
- `NA0526_CLOSEOUT_NEW_RETAINED_HASH_RECORDED_OK`
- `NA0526_CLOSEOUT_RETAINED_QSC_PATH_ACCEPTED_OK`
- `NA0526_CLOSEOUT_NA0526_DONE_OK`
- `NA0526_CLOSEOUT_NA0527_READY_OK`
- `NA0526_CLOSEOUT_NO_NA0527_IMPLEMENTATION_OK`
- `NA0526_CLOSEOUT_NO_REMOTE_ACTION_OK`
- `NA0526_CLOSEOUT_NO_SSH_EXECUTION_OK`
- `NA0526_CLOSEOUT_NO_QSC_SEND_RECEIVE_OK`
- `NA0526_CLOSEOUT_NO_REMOTE_E2EE_OK`
- `NA0526_CLOSEOUT_NO_QSC_RESTAGING_OK`
- `NA0526_CLOSEOUT_NO_QSL_SERVER_ATTACHMENTS_OK`
- `NA0526_CLOSEOUT_NO_QSC_IMPLEMENTATION_MUTATION_OK`
- `NA0526_CLOSEOUT_NO_WORKFLOW_DEPENDENCY_MUTATION_OK`
- `NA0526_CLOSEOUT_NO_CORPUS_VECTOR_INPUT_MUTATION_OK`
- `NA0526_CLOSEOUT_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0526_CLOSEOUT_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0526_CLOSEOUT_ONE_READY_INVARIANT_OK`

## Static Validation

Run:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py queue --select NA-0526 --select NA-0527
python3 scripts/ci/qsl_evidence_helper.py decisions --select D-1042 --select D-1043 --select D-1044
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --head HEAD --allowed-file <allowed-closeout-paths>
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --file <pr-body> --scan-overclaims
bash scripts/ci/classify_ci_scope.sh <changed-paths>
```

Static validation must prove:
- exact five-path closeout scope.
- READY_COUNT 1.
- READY NA-0527.
- NA-0526 DONE.
- D-1042 exists once.
- D-1043 exists once.
- D-1044 is absent.
- duplicate decision count zero.
- retained qsc old and new hashes are recorded.
- retained qsc path is recorded.
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

This closeout is accepted only if NA-0526 is DONE, NA-0527 is READY, READY_COUNT is 1, D-1043 records the closeout, static governance checks pass, post-closeout public-safety/advisories are green after merge, and no implementation, dependency, lockfile, qsc restaging, or remote-action scope is introduced.

No public-readiness claim is made. No production-readiness claim is made. No public-internet-readiness claim is made. No external-review-complete claim is made. No crypto-complete claim is made. No identity-complete claim is made. No trust-complete claim is made. No replay-proof claim is made. No downgrade-proof claim is made. No secret-material-complete claim is made. No side-channel-free claim is made. No vulnerability-free claim is made. No bug-free claim is made. No perfect-crypto claim is made.
