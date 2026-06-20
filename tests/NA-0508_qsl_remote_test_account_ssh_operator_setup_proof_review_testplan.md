Status: Supporting
Owner: QSL Governance / Core Assurance
Last-Updated: 2026-06-20

# NA-0508 Remote Test Account SSH Operator Setup Proof Review Testplan

## Objective

Verify that NA-0508 consumes NA-0507/D396 inheritance, reviews only the
operator-provided redacted proof embedded in D397, classifies the proof, selects
the exact NA-0509 successor, and preserves no remote action by Codex.

## Protected invariants

- Exactly one READY item remains mandatory.
- NA-0508 advances G4 without regressing G1, G2, G3, or G5.
- Proof-review-only governance evidence.
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

- `docs/governance/evidence/NA-0508_qsl_remote_test_account_ssh_operator_setup_proof_review.md`
- `tests/NA-0508_qsl_remote_test_account_ssh_operator_setup_proof_review_testplan.md`
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

- `docs/governance/evidence/NA-0508_qsl_remote_test_account_ssh_operator_setup_proof_review.md`

Expected sections:

- executive summary.
- live NA-0508 scope.
- qwork proof-file verification.
- NA-0507 / D396 inheritance.
- operator proof reviewed.
- redaction review.
- private key / passphrase / token absence proof.
- account boundary proof.
- SSH config / alias proof.
- host key / public key fingerprint proof.
- Step 10 proof-shape issue and Step 10B correction.
- no Codex remote action proof.
- qwork / qsl-backup remote absence proof.
- backup exposure proof.
- Hostile Cryptographer Review.
- Red-Team Review.
- Production SRE Review.
- Release-Claim Boundary Review.
- prioritization matrix.
- authorization decision.
- selected NA-0509 successor.
- future scope bundle.
- future validation / marker plan.
- remote E2E deferral.
- public claim / website / external review boundary.
- backup-impact statement.
- rejected alternatives.
- next recommendation.

## qwork proof checks

Expected:

- qwork proof files exist and are copied to the proof root.
- `.kv` and `.json` proofs mirror required fields.
- proof HEAD equals live HEAD before fetch.
- proof origin/main equals live origin/main before fetch.
- Codex does not run qwork, qstart, or qresume.

## Inheritance checks

Expected:

- D396 response exists and is consumed.
- NA-0507 is DONE.
- NA-0508 is READY.
- D-1003 exists once.
- D-1004 exists once.
- D-1005 is absent before the patch and exists once after the patch.
- duplicate decision ID count is zero.
- NA-0507 authorized operator-owned manual setup only.
- NA-0508 is proof-review-only.

## Operator proof review checks

Expected proof-review outcomes:

- operator proof consumed.
- proof is redacted enough.
- private key absent.
- passphrase absent.
- token absent.
- password absent.
- production endpoint absent.
- backup material absent.
- remote alias `inspiron` documented.
- optional alias `remote` not approved/configured.
- remote user `qslcodex` documented.
- `qslcodex` non-root.
- `qslcodex` has no privileged groups.
- `qslcodex` sudo denied.
- workdir exists and writable.
- `/backup/qsl` absent or not readable.
- qwork absent.
- qsl-backup absent.
- host key fingerprint match documented.
- public key fingerprint match documented.
- SSH config key-only / BatchMode / no password / no forwarding documented.
- Step 10 path-expansion proof issue recorded and corrected by Step 10B.
- no remote action by Codex in this directive.

Expected classification:

- `OPERATOR_REMOTE_SETUP_PROOF_ACCEPTED`

## Redaction checks

Expected:

- no private key block appears in evidence/testplan/decision/traceability/
  journal changes.
- no passphrase, token, password, credential, production endpoint, or backup
  material appears in added lines.
- only public fingerprints and non-secret account/config metadata are recorded.

## Successor checks

Expected:

- selected successor is
  `NA-0509 -- QSL Remote Host Capability Probe Scope Authorization Plan`.
- future NA-0509 remains authorization-only.
- future NA-0509 does not run SSH or remote commands.
- future NA-0509 does not perform remote E2E.
- future NA-0509 selects exact future probe command list or stop/no-action
  rationale.

## Marker proof

Expected NA-0508 markers:

- `NA0508_OPERATOR_SETUP_PROOF_CONSUMED_OK`
- `NA0508_REDACTED_PROOF_ONLY_OK`
- `NA0508_PRIVATE_KEY_ABSENT_OK`
- `NA0508_PASSPHRASE_ABSENT_OK`
- `NA0508_TOKEN_ABSENT_OK`
- `NA0508_PASSWORD_ABSENT_OK`
- `NA0508_PRODUCTION_ENDPOINT_ABSENT_OK`
- `NA0508_BACKUP_MATERIAL_ABSENT_OK`
- `NA0508_REMOTE_ALIAS_INSPIRON_REVIEWED_OK`
- `NA0508_OPTIONAL_REMOTE_ALIAS_NOT_CONFIGURED_OK`
- `NA0508_ACCOUNT_BOUNDARY_REVIEWED_OK`
- `NA0508_NO_SUDO_ADMIN_PROOF_REVIEWED_OK`
- `NA0508_NO_BACKUP_EXPOSURE_PROOF_REVIEWED_OK`
- `NA0508_HOST_KEY_FINGERPRINT_MATCH_REVIEWED_OK`
- `NA0508_PUBLIC_KEY_FINGERPRINT_MATCH_REVIEWED_OK`
- `NA0508_STEP10_PATH_EXPANSION_CORRECTED_OK`
- `NA0508_NO_REMOTE_ACTION_BY_CODEX_OK`
- `NA0508_NO_SSH_EXECUTION_BY_CODEX_OK`
- `NA0508_NO_SSH_KEY_GENERATION_BY_CODEX_OK`
- `NA0508_NO_SSH_CONFIG_MUTATION_BY_CODEX_OK`
- `NA0508_NO_REMOTE_HOST_MUTATION_BY_CODEX_OK`
- `NA0508_NO_QSC_IMPLEMENTATION_MUTATION_OK`
- `NA0508_NO_WORKFLOW_DEPENDENCY_MUTATION_OK`
- `NA0508_NO_CORPUS_VECTOR_INPUT_MUTATION_OK`
- `NA0508_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0508_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0508_ONE_READY_INVARIANT_OK`

Expected future NA-0509 markers:

- `NA0509_OPERATOR_SETUP_PROOF_ACCEPTED_OK`
- `NA0509_REMOTE_CAPABILITY_PROBE_SCOPE_SELECTED_OK`
- `NA0509_NO_REMOTE_ACTION_IN_AUTHORIZATION_OK`
- `NA0509_NO_REMOTE_E2E_SCOPE_OK`
- `NA0509_NO_SSH_KEY_GENERATION_OK`
- `NA0509_NO_SSH_CONFIG_MUTATION_OK`
- `NA0509_NO_REMOTE_HOST_MUTATION_OK`
- `NA0509_NO_SUDO_ADMIN_SCOPE_OK`
- `NA0509_NO_BACKUP_EXPOSURE_OK`
- `NA0509_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0509_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0509_ONE_READY_INVARIANT_OK`

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
- marker proof for NA-0508 evidence.
- no private key block proof.
- queue/decision proof.

## Closeout prerequisites

Closeout to NA-0509 may proceed only after:

- NA-0508 evidence PR is merged.
- D-1005 exists on main.
- post-merge public-safety is attached and green inside the short
  attach/early-failure window.
- no red required check is observed.
- NA-0508 remains READY before closeout.
- selected NA-0509 successor text is exact.
- closeout remains limited to allowed closeout paths.

NA-0508 closeout must not implement NA-0509.
