Status: Supporting
Owner: QSL Governance / Core Assurance
Last-Updated: 2026-06-20

# NA-0508 Closeout and NA-0509 Restoration Testplan

## Objective

Verify that NA-0508 is closed after PR #1288 merged and post-merge
public-safety completed success, and that exactly one READY successor is
restored: `NA-0509 -- QSL Remote Host Capability Probe Scope Authorization
Plan`.

## Protected invariants

- Exactly one READY item remains mandatory.
- NA-0508 is DONE.
- NA-0509 is READY.
- D-1005 exists once.
- D-1006 exists once.
- No duplicate decision IDs.
- Closeout-only governance/testplan scope.
- No NA-0509 implementation.
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
- `tests/NA-0508_closeout_restore_na0509_testplan.md`

## Forbidden scope

- implementing NA-0509.
- creating remote users.
- generating or installing SSH keys.
- running SSH/scp/sftp/rsync to remote.
- running ssh-keygen.
- running ssh-keyscan.
- mutating local SSH config.
- mutating system SSH config.
- mutating known_hosts.
- mutating authorized_keys.
- mutating remote hosts.
- sudo/admin action.
- package installation.
- qwork/qstart/qresume mutation or remote execution.
- qsl-backup execution or mutation.
- qsc source/test/fuzz/Cargo mutation.
- workflow, script, helper, validator, or dependency mutation.
- corpus, vector, input, or internal-manifest mutation.
- formal, refimpl, qsl-server, qsl-attachments, qshield, qshield-cli, service,
  public-doc, website, README, START_HERE, backup, backup status, backup plan,
  rollback, archive, move, or delete mutation.

## Required checks

Expected:

- PR #1288 merged at `b7328760b45d`.
- post-merge public-safety completed success for `b7328760b45d`.
- NA-0508 marked DONE in `NEXT_ACTIONS.md`.
- NA-0509 restored READY in `NEXT_ACTIONS.md`.
- D-1006 records closeout and restoration.
- TRACEABILITY records the closeout.
- rolling journal records the closeout.
- changed paths are exactly the five allowed closeout paths.

## Required validation commands

```bash
git diff --check
```

Required local proof checks:

- exact five-path scope guard.
- link-check.
- leak-scan.
- overclaim scan.
- docs/governance classifier.
- PR body preflight.
- goal-lint.
- queue/decision proof.
- private-key-block scan.

## Required markers

- `NA0508_CLOSEOUT_PR_MERGED_OK`
- `NA0508_POSTMERGE_PUBLIC_SAFETY_OK`
- `NA0508_D1005_ACCEPTED_OK`
- `NA0508_D1006_CLOSEOUT_RECORDED_OK`
- `NA0508_MARKED_DONE_OK`
- `NA0509_RESTORED_READY_OK`
- `NA0509_AUTHORIZATION_ONLY_OK`
- `NA0509_NO_REMOTE_ACTION_IN_CLOSEOUT_OK`
- `NA0509_NO_REMOTE_E2E_IMPLEMENTATION_OK`
- `NA0509_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0509_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0509_ONE_READY_INVARIANT_OK`
