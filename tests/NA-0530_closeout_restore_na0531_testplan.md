Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-06-24

# NA-0530 closeout and NA-0531 restoration testplan

Goals: G1, G2, G3, G4, G5

## Purpose

Record validation for the NA-0530 closeout-only lane that accepts D-1050 after PR #1333 merged, consumes the D433 integrated trigger quoting failure, marks NA-0530 DONE, and restores NA-0531 READY using the approved trigger-quoting remediation successor block.

This testplan does not implement NA-0531 and does not authorize remote action, SSH execution, qsc E2EE, qsc send/receive, qsl-server, or qsl-attachments.

## Required markers

- `NA0530_CLOSEOUT_PR1333_MERGED_OK`
- `NA0530_CLOSEOUT_D1050_ACCEPTED_OK`
- `NA0530_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK`
- `NA0530_CLOSEOUT_ADVISORIES_GREEN_OK`
- `NA0530_CLOSEOUT_TRIGGER_QUOTING_FAILURE_CONSUMED_OK`
- `NA0530_CLOSEOUT_D432_RESIDUE_RECOVERY_CONSUMED_OK`
- `NA0530_CLOSEOUT_RETAINED_QSC_SMOKE_CONSUMED_OK`
- `NA0530_CLOSEOUT_NO_REMOTE_ACTION_OK`
- `NA0530_CLOSEOUT_NO_SSH_EXECUTION_OK`
- `NA0530_CLOSEOUT_NO_QSC_SEND_RECEIVE_OK`
- `NA0530_CLOSEOUT_NO_REMOTE_E2EE_OK`
- `NA0530_CLOSEOUT_NO_QSL_SERVER_ATTACHMENTS_OK`
- `NA0530_CLOSEOUT_NO_DEPENDENCY_LOCKFILE_MUTATION_OK`
- `NA0530_CLOSEOUT_NO_QSC_IMPLEMENTATION_MUTATION_OK`
- `NA0530_CLOSEOUT_NA0530_DONE_OK`
- `NA0530_CLOSEOUT_NA0531_READY_OK`
- `NA0530_CLOSEOUT_ONE_READY_INVARIANT_OK`

## Static validation

- `git diff --check` passes.
- Exact closeout scope guard allows only:
  - `NEXT_ACTIONS.md`
  - `DECISIONS.md`
  - `TRACEABILITY.md`
  - `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
  - `tests/NA-0530_closeout_restore_na0531_testplan.md`
- Queue and decision proof shows:
  - READY_COUNT 1.
  - READY NA-0531.
  - NA-0530 DONE.
  - D-1050 once.
  - D-1051 once.
  - D-1052 absent.
  - duplicate decision count 0.
- Local markdown link-check passes.
- Leak scan passes for the closeout diff.
- Overclaim scan passes for added lines.
- Classifier confirms closeout-only scope.
- PR body preflight includes Goals, Impact, No-regression, and Tests/Vectors.
- Goal-lint detects `Goals: G1, G2, G3, G4, G5`.

## Dependency and formatting validation

- `cargo audit --deny warnings` passes.
- `cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock` passes.
- `cargo fmt --check` passes.
- `sh -n scripts/ci/qsc_adversarial.sh` passes.
- `bash -n scripts/ci/qsc_adversarial.sh` passes.

## Preferred focused tests

- `cargo test -p qsc --locked --test same_host_client_to_client_e2e -- --test-threads=1 --nocapture` passes when runtime budget allows.
- `cargo test -p qsc --locked --test receive_e2e -- --test-threads=1 --nocapture` passes when runtime budget allows.

## Boundary assertions

- NA-0530 implementation PR #1333 is merged at `532e273b9480`.
- D-1050 exists once and is accepted by D-1051.
- D433 classification `REMOTE_E2EE_INTEGRATED_TRIGGER_QUOTING_FAILURE` is consumed.
- D432 residue cleanup passed and is not rerun by closeout.
- Retained-qsc local-only smoke capture passed and is not rerun by closeout.
- The integrated precheck failed before qsc E2EE because the remote trigger command was quoted incorrectly.
- Listener marker_match, ack_sent, and ack_received were false in the D433 failure.
- No qsc E2EE occurred in NA-0530.
- No qsc send/receive occurred in NA-0530.
- No baseline E2EE occurred in NA-0530.
- No wrong-peer negative occurred in NA-0530.
- No stale-trust negative occurred in NA-0530.
- No qsl-server/qsl-attachments use occurred.
- No remote action or SSH execution occurs in closeout.
- No dependency or lockfile mutation occurs.
- No qsc source/test/fuzz/Cargo mutation occurs.
- No workflow/script/helper mutation occurs.
- No corpus/vector/input mutation occurs.
- No formal/refimpl/service/public/backup mutation occurs.
- No qsl-backup execution occurs.
- No public-readiness claim is made.
- No production-readiness claim is made.
- No public-internet-readiness claim is made.
- No crypto-complete claim is made.
- No identity-complete claim is made.
- No trust-complete claim is made.
- No replay-proof claim is made.
- No downgrade-proof claim is made.
- No secret-material-complete claim is made.
- No side-channel-free claim is made.
- No vulnerability-free, bug-free, or perfect-crypto claim is made.

## Post-closeout requirements

- PR checks attach and required checks pass before merge.
- Merge uses a merge commit, not squash or rebase.
- After merge, `main` equals `origin/main`.
- Post-closeout public-safety completes success on the closeout merge commit.
- Post-closeout advisories completes success or an accepted not-applicable status on the closeout merge commit.
- No required red checks remain.
- Final queue has exactly one READY item: NA-0531.
- D-1051 exists once.
- D-1052 remains absent.
