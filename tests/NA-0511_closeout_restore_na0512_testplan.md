Status: Supporting
Owner: QSL Governance / Core Assurance
Last-Updated: 2026-06-21

# NA-0511 Closeout and NA-0512 Restoration Testplan

## Objective

Verify that NA-0511 closes only after PR #1294 merged and post-merge
public-safety completed success, then restore the selected NA-0512 remote
read/write marker and toolchain/disk capability probe implementation harness as
the sole READY successor without implementing NA-0512.

## Protected invariants

- Exactly one READY item remains mandatory.
- NA-0511 is DONE.
- NA-0512 is READY.
- D-1011 exists once.
- D-1012 exists once after closeout.
- D-1013 remains absent.
- Duplicate decision ID count is zero.
- Closeout-only governance/testplan scope.
- No NA-0512 implementation in closeout.
- No remote action by Codex.
- No SSH execution by Codex.
- No scp/sftp/rsync execution to remote by Codex.
- No marker write/read/delete by Codex.
- No remote toolchain command by Codex.
- No remote E2E by Codex.
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
- No external-review-complete claim is made.
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
- `tests/NA-0511_closeout_restore_na0512_testplan.md`

## Forbidden scope

- NA-0512 implementation.
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
- remote marker write/read/delete.
- remote qsc protocol execution.
- remote source checkout/build.
- remote toolchain/disk probing during closeout.

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
- post-merge public-safety proof from PR #1294 merge commit.
- same_host_client_to_client_e2e.
- key_lifecycle_zeroization_expansion.
- secret_material_diagnostic_boundary.
- handshake_provider_error_no_mutation.
- binding corpus validator.
- all qsc fuzz corpus validator.
- formal model runner.
- root cargo audit.
- nested qsc fuzz lock audit.
- cargo fmt.
- qsc-adversarial shell syntax under sh and bash.

## Expected queue state

Before patch:

- READY_COUNT 1.
- READY NA-0511.
- D-1011 exists once.
- D-1012 absent.

After patch:

- READY_COUNT 1.
- READY NA-0512.
- NA-0511 DONE.
- D-1011 exists once.
- D-1012 exists once.
- D-1013 absent.
- duplicate decision ID count zero.

## Expected successor

`NA-0512 -- QSL Remote Host Read/Write Marker and Toolchain/Disk Capability
Probe Implementation Harness` is restored as READY.

NA-0512 is implementation scope for one bounded marker/toolchain/disk capability
probe only. It may run exactly one bounded SSH command only after fresh qwork
proof and a directive that explicitly authorizes it. This closeout does not run
SSH and does not implement NA-0512.

## Required markers

- `NA0511_CLOSEOUT_PR1294_MERGED_OK`
- `NA0511_CLOSEOUT_PUBLIC_SAFETY_GREEN_OK`
- `NA0511_CLOSEOUT_D1011_ACCEPTED_OK`
- `NA0511_CLOSEOUT_D1012_RESTORED_NA0512_OK`
- `NA0511_CLOSEOUT_NO_IMPLEMENTATION_MUTATION_OK`
- `NA0511_CLOSEOUT_NO_REMOTE_ACTION_OK`
- `NA0511_CLOSEOUT_NO_SSH_EXECUTION_OK`
- `NA0511_CLOSEOUT_NO_MARKER_WRITE_READ_DELETE_OK`
- `NA0511_CLOSEOUT_NO_QSC_IMPLEMENTATION_MUTATION_OK`
- `NA0511_CLOSEOUT_NO_WORKFLOW_DEPENDENCY_MUTATION_OK`
- `NA0511_CLOSEOUT_NO_CORPUS_VECTOR_INPUT_MUTATION_OK`
- `NA0511_CLOSEOUT_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0511_CLOSEOUT_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0511_CLOSEOUT_ONE_READY_INVARIANT_OK`
