Status: Supporting
Owner: QSL Governance / Core Assurance
Last-Updated: 2026-06-20

# NA-0506 Remote Test Account SSH Operator Setup Testplan

## Objective

Verify that NA-0506 implements only the in-repo operator runbook and proof
checklist for a future least-privilege remote test account and SSH boundary,
without performing setup, remote action, SSH execution, key handling, config
mutation, remote host mutation, backup exposure, or claim expansion.

## Protected invariants

- Exactly one READY item remains mandatory.
- NA-0506 advances G4 without regressing G1, G2, G3, or G5.
- Runbook-only implementation.
- No remote action.
- No SSH execution.
- No ssh-keygen.
- No SSH key installation.
- No local SSH config mutation.
- No system SSH config mutation.
- No known_hosts mutation.
- No authorized_keys mutation.
- No remote host mutation.
- No account creation.
- No sudo/admin action.
- No package installation.
- No qwork/qstart/qresume mutation or remote execution.
- No qsl-backup execution or mutation.
- No backup exposure.
- No qsc source/test/fuzz/Cargo mutation.
- No workflow/script/helper/dependency mutation.
- No corpus/vector/input mutation.
- No formal/refimpl/service/public/backup mutation.
- No public-readiness claim.
- No production-readiness claim.
- No public-internet-readiness claim.
- No crypto-complete claim.
- No replay-proof claim.
- No downgrade-proof claim.
- No secret-material-complete claim.
- No side-channel-free claim.
- No vulnerability-free, bug-free, or perfect-crypto claim.

## Allowed scope

- `docs/governance/evidence/NA-0506_qsl_remote_test_account_ssh_operator_setup_runbook.md`
- `tests/NA-0506_qsl_remote_test_account_ssh_operator_setup_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

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

## Runbook path

Required runbook:

- `docs/governance/evidence/NA-0506_qsl_remote_test_account_ssh_operator_setup_runbook.md`

Expected:

- includes account model, SSH key model, host alias model, operator
  responsibilities, Codex restrictions, remote host restrictions, remote
  directory model, proof checklist, cleanup/revocation checklist, failure
  handling, paste-back boundaries, claim boundaries, and markers.

## Remote action prohibition

Expected proof:

- no command log entry shows remote SSH execution.
- no command log entry shows scp/sftp/rsync to remote.
- no command log entry shows remote account creation.
- no command log entry shows sudo/admin action.
- no command log entry shows package installation.
- changed paths are limited to the five allowed implementation paths.

## SSH key/material prohibition

Expected proof:

- no command log entry shows ssh-keygen.
- no command log entry shows SSH key installation.
- no private key block appears in the runbook or testplan.
- no passphrase, token, credential, production endpoint, or backup private
  material appears in added lines.
- runbook states private keys and passphrases must never be pasted to Codex.

## Alias/collision model

Expected proof:

- runbook selects `qsl-remote-test` as the default alias.
- runbook permits `remote` only after collision check and explicit operator
  approval.
- runbook explains why `remote` is convenient but riskier.
- runbook states no local SSH config mutation by Codex in NA-0506.

## Operator proof checklist

Expected proof:

- account identity proof.
- home/work directory proof.
- no sudo proof.
- key-only login proof if feasible.
- host-key verification proof.
- no backup access proof.
- cleanup/revocation proof.
- paste-back guidance for redacted outputs only.

## No-secret material policy

Expected proof:

- no private keys.
- no passphrases.
- no API tokens.
- no session tokens.
- no passwords.
- no credentials.
- no production endpoints.
- no backup private material.
- no long secret-like hex dumps.

## Backup/qsl-backup boundary

Expected proof:

- installed qsl-backup helper digest verified read-only.
- Codex ops source inclusion count remains exactly 1.
- no qsl-backup execution.
- no backup/restore execution.
- no `/backup/qsl` mutation.
- runbook requires no `/backup/qsl` access for the future remote test user.

## Public claim boundary

Expected proof:

- no public-readiness claim.
- no production-readiness claim.
- no public-internet-readiness claim.
- no external-review-complete claim.
- no crypto-complete claim.
- no replay-proof claim.
- no downgrade-proof claim.
- no secret-material-complete claim.
- no side-channel-free claim.
- no vulnerability-free, bug-free, or perfect-crypto claim.

## Validation commands

Required:

```bash
git diff --check
cargo test -p qsc --locked --test same_host_client_to_client_e2e -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test key_lifecycle_zeroization_expansion -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test secret_material_diagnostic_boundary -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus
python3 formal/run_model_checks.py
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo fmt --check
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
```

Required local proof checks:

- exact five-path scope guard.
- link-check.
- leak-scan.
- added-line overclaim scan.
- classifier.
- PR body preflight.
- goal-lint.
- marker grep.
- private-key-block grep.
- placeholder/no-real-host proof.
- queue/decision proof.

## Closeout prerequisites

Closeout to NA-0507 may proceed only after:

- NA-0506 implementation PR is merged.
- D-1001 exists on main.
- post-merge public-safety is attached and green inside the short
  attach/early-failure window.
- no red required check is observed.
- NA-0506 remains READY before closeout.
- selected NA-0507 successor text is exact.
- closeout remains limited to allowed closeout paths.

## Required markers

The runbook must contain:

- NA0506_REMOTE_BOUNDARY_SCOPE_CONSUMED_OK
- NA0506_OPERATOR_RUNBOOK_IMPLEMENTED_OK
- NA0506_REMOTE_ACCOUNT_MODEL_DOCUMENTED_OK
- NA0506_SSH_KEY_MODEL_DOCUMENTED_OK
- NA0506_HOST_ALIAS_MODEL_DOCUMENTED_OK
- NA0506_NO_REMOTE_ACTION_OK
- NA0506_NO_SSH_KEY_GENERATION_OK
- NA0506_NO_SSH_CONFIG_MUTATION_OK
- NA0506_NO_REMOTE_HOST_MUTATION_OK
- NA0506_NO_SUDO_ADMIN_SCOPE_OK
- NA0506_NO_BACKUP_EXPOSURE_OK
- NA0506_NO_PUBLIC_READINESS_CLAIM_OK
- NA0506_NO_PRODUCTION_READINESS_CLAIM_OK
- NA0506_ONE_READY_INVARIANT_OK
