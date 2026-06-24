Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-06-24

# NA-0534 closeout and NA-0535 restoration testplan

Goals: G1, G2, G3, G4, G5

## Purpose

Record validation for the closeout-only PR after NA-0534 implementation merged. This closeout accepts D-1058, marks NA-0534 DONE, restores NA-0535 READY, and does not implement NA-0535.

## Scope guard

Allowed changed paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0534_closeout_restore_na0535_testplan.md`

No NA-0535 evidence implementation file, runtime source, qsc source/test/fuzz/Cargo, workflow/script/helper, dependency/lockfile, corpus/vector/input, formal/refimpl/service/public/backup, qsl-server, qsl-attachments, qwork/qstart/qresume, qsl-backup, SSH key/config, authorized_keys, known_hosts, or remote host path may change.

## Expected queue and decision state

- PR #1341 merged at `b52988a80e43`.
- Post-merge public-safety completed success.
- Post-merge advisories completed success.
- D-1058 exists once.
- D-1059 is absent before patch and exists once after patch.
- D-1060 is absent.
- NA-0534 is DONE.
- NA-0535 is READY.
- READY_COUNT is 1.
- Duplicate decision count is 0.

## Expected inherited evidence

- Classification `REMOTE_FORWARD_PORT_39176_DIAGNOSTIC_MARKER_TRAVERSAL_PASS` is consumed.
- Port 39176 diagnostic passed with marker traversal and ACK.
- Remote loopback bind probe passed.
- Corrected single-lifetime integrated marker/ACK probe passed.
- Cleanup passed.
- No qsc E2EE occurred in NA-0534.
- No qsc send/receive occurred in NA-0534.
- No qsc protocol command occurred in NA-0534.
- No qsl-server use occurred.
- No qsl-attachments use occurred.

## Expected closeout boundaries

- No NA-0535 implementation.
- No remote action.
- No SSH execution.
- No qsc E2EE.
- No qsc send/receive.
- No remote E2EE.
- No qsc protocol commands.
- No qsl-server use.
- No qsl-attachments use.
- No package installation.
- No sudo/admin action.
- No key/config/host mutation.
- No qwork/qstart/qresume.
- No qsl-backup execution.
- No dependency/lockfile mutation.
- No qsc source/test/fuzz/Cargo mutation.
- No workflow/script/helper mutation.
- No corpus/vector/input mutation.
- No formal/refimpl/service/public/backup mutation.

## Expected validation

- `git diff --check`.
- exact five-path closeout scope guard.
- queue/decision parser.
- link-check.
- leak-scan/private-material scan.
- overclaim scan.
- classifier.
- PR body preflight.
- goal-lint.
- marker proof.
- root `cargo audit --deny warnings`.
- nested qsc fuzz lock `cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock`.
- `cargo fmt --check`.
- Preferred local qsc tests when runtime allows:
  - `cargo test -p qsc --locked --test same_host_client_to_client_e2e -- --test-threads=1 --nocapture`.
  - `cargo test -p qsc --locked --test receive_e2e -- --test-threads=1 --nocapture`.

## Required markers

- `NA0534_CLOSEOUT_PR1341_MERGED_OK`
- `NA0534_CLOSEOUT_D1058_ACCEPTED_OK`
- `NA0534_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK`
- `NA0534_CLOSEOUT_ADVISORIES_GREEN_OK`
- `NA0534_CLOSEOUT_PORT_DIAGNOSTIC_PASS_CONSUMED_OK`
- `NA0534_CLOSEOUT_NO_REMOTE_ACTION_OK`
- `NA0534_CLOSEOUT_NO_SSH_EXECUTION_OK`
- `NA0534_CLOSEOUT_NO_QSC_E2EE_OK`
- `NA0534_CLOSEOUT_NO_QSC_SEND_RECEIVE_OK`
- `NA0534_CLOSEOUT_NO_REMOTE_E2EE_OK`
- `NA0534_CLOSEOUT_NO_QSC_PROTOCOL_COMMAND_OK`
- `NA0534_CLOSEOUT_NO_QSL_SERVER_ATTACHMENTS_OK`
- `NA0534_CLOSEOUT_NO_DEPENDENCY_LOCKFILE_MUTATION_OK`
- `NA0534_CLOSEOUT_NO_QSC_IMPLEMENTATION_MUTATION_OK`
- `NA0534_CLOSEOUT_NA0534_DONE_OK`
- `NA0534_CLOSEOUT_NA0535_READY_OK`
- `NA0534_CLOSEOUT_ONE_READY_INVARIANT_OK`

## Claim boundary

This closeout introduces no public-readiness, production-readiness, public-internet-readiness, external-review-complete, crypto-complete, identity-complete, trust-complete, replay-proof, downgrade-proof, secret-material-complete, side-channel-free, vulnerability-free, bug-free, or perfect-crypto claim.
