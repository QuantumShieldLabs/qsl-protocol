Status: Supporting
Owner: QSL Governance / Core Assurance
Last-Updated: 2026-06-20

# NA-0509 Remote Host Capability Probe Scope Authorization Testplan

## Objective

Verify that NA-0509 consumes NA-0508/D397 inheritance, designs the exact future
remote capability probe scope, selects the read-only NA-0510 successor, and
preserves no remote action by Codex in this lane.

## Protected invariants

- Exactly one READY item remains mandatory.
- NA-0509 advances G4 without regressing G1, G2, G3, or G5.
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

- `docs/governance/evidence/NA-0509_qsl_remote_host_capability_probe_scope_authorization_plan.md`
- `tests/NA-0509_qsl_remote_host_capability_probe_scope_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

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

## Required evidence doc

Required path:

- `docs/governance/evidence/NA-0509_qsl_remote_host_capability_probe_scope_authorization_plan.md`

Expected sections:

- executive summary.
- live NA-0509 scope.
- qwork proof-file verification.
- NA-0508 / D397 inheritance.
- prior operator proof boundary.
- remote capability probe scope design.
- exact future command list.
- expected future outputs.
- redaction rules.
- stop conditions.
- no remote E2E boundary.
- option review.
- Hostile Cryptographer Review.
- Red-Team Review.
- Production SRE Review.
- Release-Claim Boundary Review.
- prioritization matrix.
- authorization decision.
- selected NA-0510 successor.
- future scope bundle.
- future validation / marker plan.
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

- D397 response exists and is consumed.
- NA-0508 is DONE.
- NA-0509 is READY.
- D-1005 exists once.
- D-1006 exists once.
- D-1007 is absent before the patch and exists once after the patch.
- duplicate decision ID count is zero.
- NA-0508 operator setup proof was accepted as setup-boundary proof only.
- NA-0508 proof was not remote E2E evidence.

## Future command-scope checks

Expected:

- Future local pre-SSH proof uses redacted `ssh -G inspiron` output only.
- Future optional alias proof verifies `remote` is not an approved qslcodex route
  unless explicitly approved by a later directive.
- Future remote command family is exactly one bounded invocation:
  `ssh -o BatchMode=yes -o PasswordAuthentication=no -o ConnectTimeout=10 inspiron 'bash -s'`.
- Future remote script is limited to hostname, id, pwd, HOME printf, test, sudo
  negative check, command lookup, and fixed printf markers.
- Future remote script performs no file writes.
- Future probe remains no remote E2E.

## Redaction checks

Expected:

- no full `ssh -G` output appears in evidence, decision, traceability, journal,
  or PR body.
- no private key block appears in evidence/testplan/decision/traceability/
  journal changes.
- no passphrase, token, password, credential, production endpoint, or backup
  material appears.
- hostnames, IPs, identity paths, `$HOME`, and `pwd` are redacted or summarized
  in future paste-back rules.

## Stop-condition checks

Expected future stop conditions include:

- stale qwork proof.
- alias drift.
- host-key mismatch or ambiguity.
- wrong remote user.
- UID 0.
- privileged group membership.
- sudo success.
- missing or non-writable workdir.
- readable `/backup/qsl`.
- remote qwork present.
- remote qsl-backup present.
- remote file write.
- remote E2E attempt.
- private material in output.
- redaction ambiguity.

## Required markers

Evidence/testplan/decision must contain:

- `NA0509_QWORK_PROOF_FILES_VERIFIED_OK`
- `NA0509_D397_INHERITANCE_CONSUMED_OK`
- `NA0509_OPERATOR_PROOF_BOUNDARY_RESTATED_OK`
- `NA0509_REMOTE_PROBE_SCOPE_DESIGNED_OK`
- `NA0509_EXACT_FUTURE_COMMANDS_SELECTED_OK`
- `NA0509_READ_ONLY_PROBE_SELECTED_OK`
- `NA0509_REMOTE_WRITE_PROBE_DEFERRED_OK`
- `NA0509_NO_REMOTE_E2E_OK`
- `NA0509_REDACTION_RULES_SELECTED_OK`
- `NA0509_STOP_CONDITIONS_SELECTED_OK`
- `NA0509_NO_REMOTE_ACTION_BY_CODEX_OK`
- `NA0509_NO_SSH_EXECUTION_BY_CODEX_OK`
- `NA0509_NO_SSH_KEY_GENERATION_OK`
- `NA0509_NO_SSH_CONFIG_MUTATION_OK`
- `NA0509_NO_QSC_IMPLEMENTATION_MUTATION_OK`
- `NA0509_NO_WORKFLOW_DEPENDENCY_MUTATION_OK`
- `NA0509_NO_CORPUS_VECTOR_INPUT_MUTATION_OK`
- `NA0509_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0509_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0509_ONE_READY_INVARIANT_OK`

## Validation commands

Required local validation:

- `git diff --check`
- exact five-path scope guard.
- link-check.
- leak-scan.
- overclaim scan.
- classifier.
- PR body preflight.
- goal-lint.
- marker proof for NA-0509 evidence.
- private-key-block proof.
- `cargo test -p qsc --locked --test same_host_client_to_client_e2e -- --test-threads=1 --nocapture`
- `cargo test -p qsc --locked --test key_lifecycle_zeroization_expansion -- --test-threads=1 --nocapture`
- `cargo test -p qsc --locked --test secret_material_diagnostic_boundary -- --test-threads=1 --nocapture`
- `cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture`
- `python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics`
- `python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus`
- `python3 formal/run_model_checks.py`
- `cargo audit --deny warnings`
- `cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock`
- `cargo fmt --check`
- `sh -n scripts/ci/qsc_adversarial.sh`
- `bash -n scripts/ci/qsc_adversarial.sh`

Forbidden validation commands in NA-0509:

- `ssh`
- `scp`
- `sftp`
- `rsync`
- `ssh-keygen`
- `ssh-keyscan`
- `sudo`
- `qwork`
- `qsl-backup`

## Acceptance markers

Expected acceptance markers:

- `NA0509_QWORK_PROOF_FILES_VERIFIED_OK`
- `NA0509_D397_INHERITANCE_CONSUMED_OK`
- `NA0509_REMOTE_PROBE_SCOPE_DESIGNED_OK`
- `NA0509_READ_ONLY_PROBE_SELECTED_OK`
- `NA0509_REMOTE_WRITE_PROBE_DEFERRED_OK`
- `NA0509_NO_REMOTE_ACTION_BY_CODEX_OK`
- `NA0509_NO_SSH_EXECUTION_BY_CODEX_OK`
- `NA0509_NO_REMOTE_E2E_OK`
- `NA0509_ONE_READY_INVARIANT_OK`

## Expected classification

- `REMOTE_READ_ONLY_CAPABILITY_PROBE_IMPLEMENTATION_READY`

## Expected successor

- `NA-0510 -- QSL Remote Host Read-Only Capability Probe Implementation Harness`

## Post-fix hardening review checklist

Before declaring complete, verify:

1. Correctness under stress: stop conditions are explicit for host drift,
   privilege drift, backup exposure, forbidden tool presence, redaction
   ambiguity, and remote command deviation.
2. Minimality: the lane changes only the five allowed governance/testplan paths
   and authorizes only a future read-only probe.
3. Maintainability: future command list, markers, redaction rules, and proof
   artifacts are self-contained.
4. Coverage quality: validation checks prove markers, scope, no private material,
   no overclaim, and inherited qsc/formal/audit health.
5. Cross-lane stability: macOS/Linux qsc full-suite skip/success policy remains
   unchanged for docs/governance scope and no qsc implementation files change.
