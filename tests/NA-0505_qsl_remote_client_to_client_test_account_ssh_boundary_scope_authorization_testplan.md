Status: Supporting
Owner: QSL Governance / Core Assurance
Last-Updated: 2026-06-20

# NA-0505 Remote Client-to-Client Test Account SSH Boundary Scope Authorization Testplan

## Objective

Verify that NA-0505 authorizes the remote test account and SSH boundary for a
future remote/LAN client-to-client test setup without performing remote setup,
SSH execution, key generation, SSH config mutation, remote host mutation, or
remote testing.

## Protected invariants

- Exactly one READY item remains mandatory until closeout.
- NA-0505 advances G4 without regressing G1, G2, G3, or G5.
- NA-0505 is authorization-only.
- No remote action is performed.
- No SSH key generation or installation is performed.
- No local or system SSH config mutation is performed.
- No known_hosts or authorized_keys mutation is performed.
- No remote host mutation is performed.
- No qsc source/test/fuzz/Cargo mutation is performed.
- No workflow/script/helper/dependency mutation is performed.
- No corpus/vector/input mutation is performed.
- No formal/refimpl/service/public/backup mutation is performed.
- no public-readiness claim is made.
- no production-readiness claim is made.
- no public-internet-readiness claim is made.
- no crypto-complete claim is made.
- no replay-proof claim is made.
- no downgrade-proof claim is made.
- no secret-material-complete claim is made.
- no side-channel-free claim is made.

## Allowed scope

- `docs/governance/evidence/NA-0505_qsl_remote_client_to_client_test_account_ssh_boundary_scope_authorization_plan.md`
- `tests/NA-0505_qsl_remote_client_to_client_test_account_ssh_boundary_scope_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

- creating remote users.
- generating or installing SSH keys.
- running SSH, scp, sftp, or rsync to remote.
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
- formal, refimpl, qsl-server, qsl-attachments, qshield, qshield-cli,
  service, public-doc, website, README, START_HERE, backup, backup status,
  backup plan, rollback, archive, move, or delete mutation.

## Inheritance checks

Expected:

- D392 response exists and states the previous stop was only the missing exact
  D391 response path.
- D392 made no repository mutation.
- Exact D391 response path absence is recorded as a residual.
- Bounded response discovery finds either one plausible D391 response or uses
  in-tree fallback proof.
- D-0998 exists once and records NA-0504 closeout / NA-0505 restoration.
- NA-0504 is DONE and NA-0505 is READY.
- PR #1281 metadata and changed paths are collected.

Validation marker:

- `NA0505_D391_RECOVERY_HANDLED_OK`

## Remote boundary inventory checks

Expected:

- Dedicated non-root remote test user selected.
- No sudo/admin capability selected.
- Key-only login selected where feasible.
- Dedicated per-project SSH key selected.
- Personal SSH key reuse rejected.
- Dedicated remote work directory selected.
- No production data selected.
- No backup material selected.
- No qwork or qsl-backup remote execution selected.
- Cleanup and revocation proof required.

Validation marker:

- `NA0505_REMOTE_BOUNDARY_INVENTORY_OK`

## Host alias checks

Expected:

- `qsl-remote-test` selected as the default safe alias.
- `remote` is not selected as the default.
- `remote` is allowed only as an optional convenience alias after a collision
  check and explicit operator confirmation.
- No local SSH config mutation is performed by NA-0505.

Validation markers:

- `NA0505_HOST_ALIAS_MODEL_OK`
- `NA0505_NO_SSH_CONFIG_MUTATION_OK`

## Option and review checks

Expected:

- Option review evaluates the eight directive candidates.
- Hostile Cryptographer Review is completed.
- Red-Team Review is completed.
- Production SRE Review is completed.
- Release-Claim Boundary Review is completed.
- Prioritization matrix is completed.
- Primary classification is selected.
- NA-0506 successor is selected.

Validation markers:

- `NA0505_OPTION_REVIEW_OK`
- `NA0505_STEWARDSHIP_REVIEWS_OK`
- `NA0505_PRIORITIZATION_MATRIX_OK`
- `NA0505_AUTHORIZATION_DECISION_OK`
- `NA0505_SUCCESSOR_SELECTED_OK`

## Required local validation

Run and require pass:

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

Also run local proof checks:

- exact five-path scope guard.
- link-check.
- leak-scan.
- overclaim scan.
- classifier.
- PR body preflight.
- goal-lint.
- queue/decision proof.

## Required marker plan

Evidence and decision should preserve:

- `NA0505_D391_RECOVERY_HANDLED_OK`
- `NA0505_REMOTE_BOUNDARY_INVENTORY_OK`
- `NA0505_REMOTE_ACCOUNT_MODEL_SELECTED_OK`
- `NA0505_SSH_KEY_MODEL_SELECTED_OK`
- `NA0505_HOST_ALIAS_MODEL_OK`
- `NA0505_OPERATOR_PROOF_CHECKLIST_OK`
- `NA0505_NO_REMOTE_ACTION_OK`
- `NA0505_NO_SSH_EXECUTION_OK`
- `NA0505_NO_ACCOUNT_CREATION_OK`
- `NA0505_NO_SSH_KEY_GENERATION_OK`
- `NA0505_NO_SSH_CONFIG_MUTATION_OK`
- `NA0505_NO_REMOTE_HOST_MUTATION_OK`
- `NA0505_NO_QSC_IMPLEMENTATION_MUTATION_OK`
- `NA0505_NO_WORKFLOW_DEPENDENCY_MUTATION_OK`
- `NA0505_NO_CORPUS_VECTOR_INPUT_MUTATION_OK`
- `NA0505_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0505_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0505_ONE_READY_INVARIANT_OK`

## Post-fix hardening review checklist

Before declaring complete, verify:

- Correctness under stress: account/key/alias recommendations remain safe if
  the remote host is compromised, the key leaks, or a command is misdirected.
- Minimality: only the five allowed governance/testplan paths changed.
- Maintainability: NA-0506 successor has exact paths, deliverables, and
  acceptance criteria.
- Coverage quality: tests and scans check inherited qsc surfaces plus
  authorization-only no-remote boundaries.
- Cross-lane stability: Linux/macOS policy boundaries remain unchanged; no
  qsc implementation or workflow behavior changed.

## Public claim boundary

The PR and evidence must preserve:

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

## Closeout prerequisites

NA-0505 closeout to NA-0506 is allowed only after:

- NA-0505 evidence PR merges.
- post-merge public-safety is green inside the short attach/early-failure
  window.
- D-0999 exists once on main.
- NA-0505 remains READY before closeout.
- NA-0506 is restored by a separate closeout patch without implementing the
  NA-0506 runbook.
