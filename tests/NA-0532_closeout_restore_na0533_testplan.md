Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-06-24

# NA-0532 closeout and NA-0533 restoration testplan

Goals: G1, G2, G3, G4, G5

## Purpose

Record closeout validation for NA-0532 after implementation PR #1337 merged and post-merge public-safety/advisories completed success. This closeout marks NA-0532 DONE and restores the supplied NA-0533 reverse-forward port 39176 diagnostic authorization successor without implementing NA-0533.

## Expected queue result

- NA-0532 is DONE.
- NA-0533 is READY.
- READY_COUNT remains 1.
- NA-0533 title is `QSL Remote qsc E2EE Reverse-Forward Port 39176 Regression Diagnostic / Retry Scope Authorization Plan`.

## Expected decision result

- D-1054 exists once.
- D-1055 exists once after patch.
- D-1056 is absent.
- Duplicate decision count is 0.

## Inherited failure classification

- D436 classification is `REMOTE_E2EE_INTEGRATED_FORWARDING_PRECHECK_FAILURE`.
- Retained qsc recheck passed before the failed precheck.
- The command manifest was written.
- Local trigger compile/rehearsal passed.
- The listener bound to `127.0.0.1:39176`.
- The dedicated-key reverse-forward used `ExitOnForwardFailure=yes` and loopback-only `-R 127.0.0.1:39176:127.0.0.1:39176`.
- The reverse-forward exited before remote trigger execution with `remote port forwarding failed for listen port 39176`.
- Marker traversal did not occur.
- ACK did not occur.
- qsc relay did not start.
- Remote E2EE root was not created.
- qsc E2EE did not run.
- qsc send/receive did not run.
- Baseline E2EE did not run.
- Wrong-peer negative did not run.
- Stale-trust negative did not run.
- qsl-server/qsl-attachments did not run.
- Cleanup passed.

## Scope guard

Allowed changed paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0532_closeout_restore_na0533_testplan.md`

No implementation, NA-0533 evidence/testplan, qsc source/test/fuzz/Cargo, workflow/script/helper, dependency/lockfile, corpus/vector/input, formal/refimpl/service/public/backup, qsl-server, qsl-attachments, qwork/qstart/qresume, or qsl-backup path may change.

## Boundary assertions

This closeout performs no remote action, SSH execution, scp/sftp/rsync, qsc E2EE, qsc send/receive, qsc protocol command, qsl-server use, qsl-attachments use, remote file write, package install, remote source checkout/build, key/config/host mutation, qwork/qstart/qresume execution, qsl-backup execution, backup, or restore.

No public-readiness, production-readiness, public-internet-readiness, crypto-complete, identity-complete, trust-complete, replay-proof, downgrade-proof, secret-material-complete, side-channel-free, vulnerability-free, bug-free, or perfect-crypto claim is introduced.

## Validation

Expected validation:

- `git diff --check`
- exact five-path closeout scope guard
- queue/decision parser
- link-check
- leak-scan
- overclaim scan
- classifier
- PR body preflight
- goal-lint after PR creation
- marker proof for this testplan
- root cargo audit
- nested qsc fuzz lock cargo audit
- `cargo fmt --check`
- preferred local qsc tests when runtime allows
- required PR checks green before merge
- post-merge public-safety green
- post-merge advisories green or accepted not-applicable status

## Required markers

- `NA0532_CLOSEOUT_PR1337_MERGED_OK`
- `NA0532_CLOSEOUT_D1054_ACCEPTED_OK`
- `NA0532_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK`
- `NA0532_CLOSEOUT_ADVISORIES_GREEN_OK`
- `NA0532_CLOSEOUT_FORWARDING_PRECHECK_FAILURE_CONSUMED_OK`
- `NA0532_CLOSEOUT_NO_REMOTE_ACTION_OK`
- `NA0532_CLOSEOUT_NO_SSH_EXECUTION_OK`
- `NA0532_CLOSEOUT_NO_QSC_E2EE_OK`
- `NA0532_CLOSEOUT_NO_QSC_SEND_RECEIVE_OK`
- `NA0532_CLOSEOUT_NO_QSL_SERVER_ATTACHMENTS_OK`
- `NA0532_CLOSEOUT_NO_DEPENDENCY_LOCKFILE_MUTATION_OK`
- `NA0532_CLOSEOUT_NO_QSC_IMPLEMENTATION_MUTATION_OK`
- `NA0532_CLOSEOUT_NA0532_DONE_OK`
- `NA0532_CLOSEOUT_NA0533_READY_OK`
- `NA0532_CLOSEOUT_ONE_READY_INVARIANT_OK`
