Status: Supporting
Owner: QSL Governance / Core Assurance
Last-Updated: 2026-06-20

# NA-0507 Remote Test Account SSH Operator Setup Readiness Manual Action Authorization Testplan

## Objective

Verify that NA-0507 consumes NA-0506/D394 inheritance, reviews the NA-0506
runbook, authorizes only operator-owned manual setup steps and redacted proof
outputs for a future lane, and preserves no remote action by Codex.

## Protected invariants

- Exactly one READY item remains mandatory.
- NA-0507 advances G4 without regressing G1, G2, G3, or G5.
- Authorization-only governance evidence.
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
- No public-readiness claim.
- No production-readiness claim.
- No public-internet-readiness claim.
- No crypto-complete claim.
- No replay-proof claim.
- No downgrade-proof claim.
- No secret-material-complete claim.
- No side-channel-free claim.
- No vulnerability-free claim, no bug-free claim, and no perfect-crypto claim.

## Allowed scope

- `docs/governance/evidence/NA-0507_qsl_remote_test_account_ssh_operator_setup_readiness_manual_action_authorization_plan.md`
- `tests/NA-0507_qsl_remote_test_account_ssh_operator_setup_readiness_manual_action_authorization_testplan.md`
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

## Required evidence doc

Required path:

- `docs/governance/evidence/NA-0507_qsl_remote_test_account_ssh_operator_setup_readiness_manual_action_authorization_plan.md`

Expected sections:

- executive summary.
- live NA-0507 scope.
- qwork proof-file verification.
- NA-0506 / D394 inheritance.
- runbook review.
- manual action authorization checklist.
- approved proof outputs.
- forbidden paste-back material.
- alias policy.
- stop-and-ask-Director gates.
- cleanup/revocation requirements.
- option review.
- Hostile Cryptographer Review.
- Red-Team Review.
- Production SRE Review.
- Release-Claim Boundary Review.
- prioritization matrix.
- authorization decision.
- selected NA-0508 successor.
- future scope bundle.
- future validation / marker plan.
- remote action deferral.
- public claim / website / external review boundary.
- backup-impact statement.
- rejected alternatives.
- next recommendation.

## Runbook review checks

Expected:

- account model reviewed.
- SSH key model reviewed.
- host alias model reviewed.
- future setup checklist reviewed.
- proof checklist reviewed.
- cleanup/revocation checklist reviewed.
- forbidden action list reviewed.
- paste-back boundary reviewed.
- claim boundary reviewed.
- all NA-0506 markers are present.
- no private key block appears in NA-0506 runbook/testplan.
- no instruction tells Codex to run SSH, generate keys, or mutate a remote host.

## Manual action authorization checks

Expected:

- checklist uses placeholders only.
- checklist states Codex must not execute the setup.
- checklist requires non-production host confirmation.
- checklist requires no `/backup/qsl` exposure.
- checklist requires dedicated non-root no-sudo test user.
- checklist requires dedicated remote work root `~/qsl-remote-test/`.
- checklist requires operator-owned per-project key generation outside Codex.
- checklist requires public key installation by operator only.
- checklist requires key-only login if feasible.
- checklist selects default alias `qsl-remote-test`.
- checklist permits `remote` only after no-collision proof and explicit
  operator approval.
- checklist requires host-key trust confirmation.
- checklist requires redacted proof collection.
- checklist requires cleanup/revocation planning.

## Approved proof output checks

Expected approved paste-back categories:

- redacted username and host alias.
- redacted `ssh -G <alias>` relevant non-secret fields.
- redacted no-sudo proof.
- redacted remote work directory proof.
- redacted key-only login proof if feasible.
- redacted no `/backup/qsl` access proof.
- public key fingerprint only.
- host key fingerprint confirmation only.
- cleanup/revocation checklist status.

## Forbidden paste-back checks

Expected forbidden categories:

- private key.
- passphrase.
- token.
- password.
- credential.
- sensitive full hostname/IP.
- production endpoint.
- personal data.
- backup material.
- authorized_keys content if it exposes unrelated keys.
- known_hosts content if it exposes unrelated infrastructure.
- private host material.
- long secret-like dumps.

## Successor checks

Expected:

- primary classification is `REMOTE_OPERATOR_MANUAL_SETUP_AUTHORIZATION_READY`.
- selected successor is `NA-0508 -- QSL Remote Test Account / SSH Operator Manual Setup Proof Review Harness`.
- future NA-0508 is proof-review only.
- future NA-0508 does not authorize Codex to generate keys, install keys, run
  SSH, mutate SSH config, mutate remote hosts, or run remote tests.

## Marker proof

Expected NA-0507 markers:

- `NA0507_QWORK_PROOF_FILES_VERIFIED_OK`
- `NA0507_D394_INHERITANCE_CONSUMED_OK`
- `NA0507_NA0506_RUNBOOK_REVIEWED_OK`
- `NA0507_MANUAL_ACTION_AUTHORIZATION_READY_OK`
- `NA0507_APPROVED_PROOF_OUTPUTS_SELECTED_OK`
- `NA0507_FORBIDDEN_PASTE_BACK_SELECTED_OK`
- `NA0507_ALIAS_POLICY_SELECTED_OK`
- `NA0507_NO_REMOTE_ACTION_OK`
- `NA0507_NO_SSH_EXECUTION_OK`
- `NA0507_NO_ACCOUNT_CREATION_OK`
- `NA0507_NO_SSH_KEY_GENERATION_OK`
- `NA0507_NO_SSH_CONFIG_MUTATION_OK`
- `NA0507_NO_REMOTE_HOST_MUTATION_OK`
- `NA0507_NO_QSC_IMPLEMENTATION_MUTATION_OK`
- `NA0507_NO_WORKFLOW_DEPENDENCY_MUTATION_OK`
- `NA0507_NO_CORPUS_VECTOR_INPUT_MUTATION_OK`
- `NA0507_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0507_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0507_ONE_READY_INVARIANT_OK`

Expected future NA-0508 markers:

- `NA0508_OPERATOR_SETUP_PROOF_CONSUMED_OK`
- `NA0508_REDACTED_PROOF_ONLY_OK`
- `NA0508_PRIVATE_KEY_ABSENT_OK`
- `NA0508_PASSPHRASE_ABSENT_OK`
- `NA0508_TOKEN_ABSENT_OK`
- `NA0508_NO_REMOTE_ACTION_BY_CODEX_OK`
- `NA0508_ALIAS_COLLISION_PROOF_REVIEWED_OK`
- `NA0508_NO_SUDO_ADMIN_PROOF_REVIEWED_OK`
- `NA0508_NO_BACKUP_EXPOSURE_PROOF_REVIEWED_OK`
- `NA0508_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0508_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0508_ONE_READY_INVARIANT_OK`

## Required validation commands

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
- overclaim scan.
- classifier.
- PR body preflight.
- goal-lint.
- marker proof for NA-0507 evidence.
- no private key block proof.
- queue/decision proof.

## Closeout prerequisites

Closeout to NA-0508 may proceed only after:

- NA-0507 authorization PR is merged.
- D-1003 exists on main.
- post-merge public-safety is attached and green inside the short
  attach/early-failure window.
- no red required check is observed.
- NA-0507 remains READY before closeout.
