Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-23

# NA-0528 Closeout and NA-0529 Restoration Testplan

Goals: G1, G2, G3, G4, G5

## Purpose

Validate that NA-0528 closeout consumes the merged D-1046 authorization evidence from PR #1329, preserves the one-READY queue invariant, and restores NA-0529 as the selected reverse-forwarding diagnostic implementation successor without implementing NA-0529.

## Scope

Allowed closeout mutation paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0528_closeout_restore_na0529_testplan.md`

No NA-0529 implementation, remote action, SSH execution, scp, sftp, rsync, remote command execution, qsc send/receive, qsc protocol commands, remote E2EE, forwarding diagnostic, qsl-server/qsl-attachments use, dependency, lockfile, Cargo.toml, qsc source/test/fuzz/Cargo, workflow/script/helper, corpus/vector/input, formal/refimpl/service/public/backup, qwork/qstart/qresume, qsl-backup, backup, archive, move, delete, or remote path mutation is allowed.

## Required Proof

- qwork proof files were read from `/srv/qbuild/work/NA-0528/.qwork/`, and qwork/qstart/qresume were not run by closeout.
- PR #1329 merged with merge commit `ba54e32e3012`.
- D-1046 exists once and records classification `REMOTE_FORWARDING_DIAGNOSTIC_IMPLEMENTATION_READY`.
- D428 response exists and records that NA-0528 was authorization-only and selected `NA-0529 -- QSL Remote qsc E2EE Reverse-Forwarding Diagnostic Implementation Harness`.
- D428 response records optional closeout did not run because public-safety was missing and advisories were still in progress inside the short window, with no red checks observed.
- Current recheck proves public-safety completed success on `ba54e32e3012`.
- Current recheck proves advisories completed success on `ba54e32e3012`.
- D427 forwarding failure classification `REMOTE_E2EE_FORWARDING_RECHECK_FAILURE` is consumed.
- NA-0520 successful forwarding classification `SSH_FORWARDING_CAPABILITY_PROBE_PASS` is consumed.
- D-1047 was absent before closeout and exists once after closeout.
- D-1048 remains absent.
- NA-0528 is DONE.
- NA-0529 is READY.
- READY_COUNT is exactly 1.
- NA-0529 successor text matches the D-1046-selected reverse-forwarding diagnostic implementation harness.
- Closeout performs no remote action, SSH execution, scp, sftp, rsync, remote command execution, qsc send/receive, qsc protocol command, remote E2EE, forwarding diagnostic, dependency mutation, Cargo.lock mutation, Cargo.toml mutation, qsc source/test/fuzz/Cargo mutation, workflow/script/helper mutation, corpus/vector/input mutation, formal/refimpl/service/public/backup mutation, qsl-server/qsl-attachments use, qwork/qstart/qresume, qsl-backup execution, backup, or restore.
- Closeout introduces no public-readiness claim, no production-readiness claim, no public-internet-readiness claim, no external-review-complete claim, no crypto-complete claim, no identity-complete claim, no trust-complete claim, no replay-proof claim, no downgrade-proof claim, no secret-material-complete claim, no side-channel-free claim, no vulnerability-free claim, no bug-free claim, and no perfect-crypto claim.

## Required Markers

- `NA0528_CLOSEOUT_PR1329_MERGED_OK`
- `NA0528_CLOSEOUT_D1046_ACCEPTED_OK`
- `NA0528_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK`
- `NA0528_CLOSEOUT_ADVISORIES_GREEN_OK`
- `NA0528_CLOSEOUT_D1047_RESTORED_NA0529_OK`
- `NA0528_CLOSEOUT_D1048_ABSENT_OK`
- `NA0528_CLOSEOUT_NA0528_DONE_OK`
- `NA0528_CLOSEOUT_NA0529_READY_OK`
- `NA0528_CLOSEOUT_NO_NA0529_IMPLEMENTATION_OK`
- `NA0528_CLOSEOUT_NO_REMOTE_ACTION_OK`
- `NA0528_CLOSEOUT_NO_SSH_EXECUTION_OK`
- `NA0528_CLOSEOUT_NO_QSC_SEND_RECEIVE_OK`
- `NA0528_CLOSEOUT_NO_REMOTE_E2EE_OK`
- `NA0528_CLOSEOUT_NO_QSL_SERVER_ATTACHMENTS_OK`
- `NA0528_CLOSEOUT_NO_DEPENDENCY_LOCKFILE_MUTATION_OK`
- `NA0528_CLOSEOUT_NO_QSC_IMPLEMENTATION_MUTATION_OK`
- `NA0528_CLOSEOUT_NO_WORKFLOW_SCRIPT_HELPER_MUTATION_OK`
- `NA0528_CLOSEOUT_NO_CORPUS_VECTOR_INPUT_MUTATION_OK`
- `NA0528_CLOSEOUT_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0528_CLOSEOUT_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0528_CLOSEOUT_ONE_READY_INVARIANT_OK`

## Static Validation

Run:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py queue --select NA-0528 --select NA-0529
python3 scripts/ci/qsl_evidence_helper.py decisions --select D-1046 --select D-1047 --select D-1048
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --head HEAD --allowed-file <allowed-closeout-paths>
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --file <pr-body> --scan-overclaims
bash scripts/ci/classify_ci_scope.sh <changed-paths>
```

Static validation must prove:

- exact five-path closeout scope.
- READY_COUNT 1.
- READY NA-0529.
- NA-0528 DONE.
- D-1046 exists once.
- D-1047 exists once.
- D-1048 is absent.
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

## Required Local Validation

Run:

```bash
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo fmt --check
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
```

Preferred focused tests:

```bash
cargo test -p qsc --locked --test same_host_client_to_client_e2e -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test receive_e2e -- --test-threads=1 --nocapture
```

## Acceptance

This closeout is accepted only if NA-0528 is DONE, NA-0529 is READY, READY_COUNT is 1, D-1047 records the closeout, D-1048 remains absent, static governance checks pass, dependency health checks pass, post-closeout public-safety/advisories are green after merge, and no implementation, dependency, lockfile, qsc diagnostic, qsc send/receive, remote E2EE, or remote-action scope is introduced.

No public-readiness claim is made. No production-readiness claim is made. No public-internet-readiness claim is made. No external-review-complete claim is made. No crypto-complete claim is made. No identity-complete claim is made. No trust-complete claim is made. No replay-proof claim is made. No downgrade-proof claim is made. No secret-material-complete claim is made. No side-channel-free claim is made. No vulnerability-free claim is made. No bug-free claim is made. No perfect-crypto claim is made.
