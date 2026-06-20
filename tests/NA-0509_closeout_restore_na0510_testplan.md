Status: Supporting
Owner: QSL Governance / Core Assurance
Last-Updated: 2026-06-20

# NA-0509 Closeout and NA-0510 Restoration Testplan

## Objective

Verify that NA-0509 closes only after PR #1290 merged and post-merge
public-safety completed success, then restore the selected NA-0510 read-only
remote capability probe implementation harness as the sole READY successor
without implementing NA-0510.

## Protected invariants

- Exactly one READY item remains mandatory.
- NA-0509 is DONE.
- NA-0510 is READY.
- D-1007 exists once.
- D-1008 exists once after closeout.
- D-1009 remains absent.
- Duplicate decision ID count is zero.
- Closeout-only governance/testplan scope.
- No NA-0510 implementation in closeout.
- No remote action by Codex.
- No SSH execution by Codex.
- No scp/sftp/rsync execution to remote by Codex.
- No ssh-keygen.
- No ssh-keyscan.
- No remote account creation by Codex.
- No SSH key generation or installation by Codex.
- No local SSH config mutation by Codex.
- No system SSH config mutation by Codex.
- No known_hosts mutation by Codex.
- No authorized_keys mutation by Codex.
- No remote host mutation by Codex.
- No sudo/admin action by Codex.
- No package installation by Codex.
- No qwork/qstart/qresume mutation.
- No qsl-backup execution or mutation.
- No backup or restore.
- No qsc source/test/fuzz/Cargo mutation.
- No workflow/script/helper/dependency mutation.
- No corpus/vector/input mutation.
- No formal/refimpl/service/public/backup mutation.
- No public-readiness claim is made.
- No production-readiness claim is made.
- No public-internet-readiness claim is made.
- No crypto-complete claim is made.
- No replay-proof claim is made.
- No downgrade-proof claim is made.
- No secret-material-complete claim is made.
- No side-channel-free claim is made.
- No vulnerability-free claim is made.
- No bug-free claim is made.
- No perfect-crypto claim is made.

## Allowed scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0509_closeout_restore_na0510_testplan.md`

## Forbidden scope

- NA-0510 evidence implementation.
- running SSH/scp/sftp/rsync to remote.
- running ssh-keygen.
- running ssh-keyscan.
- generating or installing SSH keys.
- mutating local SSH config.
- mutating system SSH config.
- mutating known_hosts.
- mutating authorized_keys.
- mutating remote hosts.
- creating remote users.
- sudo/admin action.
- package installation.
- qwork/qstart/qresume mutation.
- qsl-backup execution or mutation.
- qsc source/test/fuzz/Cargo mutation.
- workflow, script, helper, validator, or dependency mutation.
- corpus, vector, input, or internal-manifest mutation.
- formal, refimpl, qsl-server, qsl-attachments, qshield, qshield-cli, service,
  public-doc, website, README, START_HERE, backup, backup status, backup plan,
  rollback, archive, move, or delete mutation.
- remote E2E.
- remote file write.

## Required validation

- `git diff --check`
- exact five-path closeout scope guard.
- link-check.
- leak-scan.
- added-line overclaim scan.
- docs/governance classifier.
- PR body preflight.
- goal-lint.
- queue/decision proof.
- post-merge public-safety proof from PR #1290 merge commit.

## Expected queue state

Before patch:

- READY_COUNT 1.
- READY NA-0509.
- D-1007 exists once.
- D-1008 absent.

After patch:

- READY_COUNT 1.
- READY NA-0510.
- NA-0509 DONE.
- D-1007 exists once.
- D-1008 exists once.
- D-1009 absent.
- duplicate decision ID count zero.

## Expected successor

`NA-0510 -- QSL Remote Host Read-Only Capability Probe Implementation Harness`
is restored as READY.

NA-0510 may run the selected bounded read-only capability probe only after a
future directive and fresh qwork proof authorize it. This closeout does not run
SSH and does not implement NA-0510.

## Required markers

- `NA0509_CLOSEOUT_PR1290_MERGED_OK`
- `NA0509_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK`
- `NA0509_CLOSEOUT_D1007_ACCEPTED_OK`
- `NA0509_CLOSEOUT_D1008_RESTORED_NA0510_OK`
- `NA0509_CLOSEOUT_NO_NA0510_IMPLEMENTATION_OK`
- `NA0509_CLOSEOUT_NO_REMOTE_ACTION_OK`
- `NA0509_CLOSEOUT_NO_SSH_EXECUTION_OK`
- `NA0509_CLOSEOUT_NO_QSC_IMPLEMENTATION_MUTATION_OK`
- `NA0509_CLOSEOUT_NO_WORKFLOW_DEPENDENCY_MUTATION_OK`
- `NA0509_CLOSEOUT_NO_CORPUS_VECTOR_INPUT_MUTATION_OK`
- `NA0509_CLOSEOUT_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0509_CLOSEOUT_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0509_CLOSEOUT_ONE_READY_INVARIANT_OK`
