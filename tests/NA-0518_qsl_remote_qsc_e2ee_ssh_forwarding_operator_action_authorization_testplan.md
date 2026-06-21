Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-21

# NA-0518 remote qsc E2EE SSH forwarding operator action authorization testplan

## Purpose

Validate the NA-0518 authorization-only governance/security lane. This testplan verifies qwork proof-file handling, NA-0517 / D411 inheritance, operator PTY root-cause context handling, forwarding key strategy selection, authorized_keys option template selection, future proof/rejection rules, scope guard, claim boundaries, and local validation.

## Scope guard

Allowed changed paths for the NA-0518 evidence PR:

- `docs/governance/evidence/NA-0518_qsl_remote_qsc_e2ee_ssh_forwarding_operator_action_authorization_plan.md`
- `tests/NA-0518_qsl_remote_qsc_e2ee_ssh_forwarding_operator_action_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

No qsc source/test/fuzz/Cargo, workflow/script/helper, dependency/lockfile, corpus/vector/input, formal/refimpl/service/public/backup, qsl-server, qsl-attachments, qshield, qshield-cli, public docs, website, README, START_HERE, backup, rollback, archive, move, or deletion path may change.

## Required startup proof

- qwork proof files exist and are read without rerunning qwork.
- qwork `.kv` and `.json` required fields pass.
- qwork proof HEAD and origin/main match live refs before fetch.
- `/` disk usage is below 95%.
- qsl-backup helper is read only, digest matches expected short digest, and Codex ops source inclusion count is exactly 1.
- Fetch occurs only after qwork proof/live refs match.
- `origin/main` equals or descends from `741262357e3f`.
- READY_COUNT is 1.
- READY item is NA-0518.
- NA-0517, NA-0516, and NA-0515 are DONE.
- D-1023 exists once.
- D-1024 exists once.
- D-1025 is absent before patch and exists once after patch.
- No duplicate decision IDs exist.

## Required inheritance proof

Evidence must record:

- NA-0517 completed and NA-0518 restored READY.
- D411 classification `SSH_FORWARDING_OPERATOR_ACTION_AUTHORIZATION_READY` consumed.
- D410 transport failure consumed.
- qsc-native forwarding-free path rejected for current retained qsc surface.
- qsl-server/qsl-attachments deferred.
- Selected model from D411: Build-local relay plus remote reverse forwarding.
- Selected default from D411: separate dedicated forwarding key.
- Selected policy from D411: no PTY, no agent forwarding, no X11 forwarding, no sudo/admin, no backup exposure, no qwork/qsl-backup exposure, loopback-only constraints.
- NA-0517 performed no remote action, no SSH execution, no authorized_keys mutation, no key generation/installation, no qsc send/receive, no remote E2EE, no public-readiness claim and no production-readiness claim.

## Operator PTY context proof

Evidence must record the operator context as supplied, without Codex SSH verification:

- qslcodex can allocate PTYs locally on Inspiron.
- qslcodex authorized_keys line begins with `restrict`.
- `restrict` explains SSH PTY allocation failure unless `pty` is re-enabled.
- No operator changes were made during the investigation.
- PTY must remain disabled for QSL E2EE forwarding remediation.
- `restrict,pty` is rejected/not selected for this path.

## Forwarding strategy proof

Evidence must compare:

- Dedicated forwarding key.
- Existing operational qslcodex key update.
- PTY enablement.
- sshd_config/account-wide forwarding.
- qsc-native forwarding-free retry.
- qsl-server/qsl-attachments integration.
- Deferral or cleanup/abandonment.

Required selected policy:

- `SSH_FORWARDING_DEDICATED_KEY_OPERATOR_ACTION_READY`.
- Separate dedicated forwarding key selected as the safer default.
- Existing qslcodex key deferred as fallback only.
- Key-level loopback-only constraints selected.
- No PTY broadening.
- No agent forwarding.
- No X11 forwarding.
- No sudo/admin.
- No backup exposure.
- No qwork/qsl-backup exposure.
- No qsl-server/qsl-attachments.

## authorized_keys template proof

Evidence must include the selected template:

```text
restrict,port-forwarding,permitlisten="127.0.0.1:39176",permitopen="127.0.0.1:39176",command="/bin/false" ssh-ed25519 <PUBLIC_KEY> qsl-inspiron-qslcodex-forward-<date>
```

Evidence must state:

- `restrict` remains present.
- `port-forwarding` is explicitly enabled.
- `permitlisten` and `permitopen` are loopback-only and port-specific.
- `command="/bin/false"` is included if compatible.
- `pty`, agent forwarding, and X11 forwarding are absent.
- OpenSSH option compatibility must be validated by operator proof.
- If `command="/bin/false"` breaks forwarding, operator must stop and report rather than silently remove it.
- Any alternate port requires later authorization.

## Required future proof and rejection rules

Future NA-0519 proof must include redacted key fingerprint/comment, selected key strategy, relevant option summary for only the forwarding key, restrict/port-forwarding/permitlisten/permitopen status, PTY absent, agent/X11 absent, no-shell forced command status or compatibility report, qslcodex non-sudo, no backup exposure, qwork absent, qsl-backup absent, no production data, and cleanup/revocation command.

Future NA-0519 must reject private key material, passphrase material, full authorized_keys with unrelated keys, known_hosts dumps, unrelated host/IP inventory, passwords, tokens, credentials, backup material, production endpoints, personal data, PTY enabled proof, agent/X11 enabled proof, broad forwarding, non-loopback exposure, sudo/admin exposure, backup exposure, qwork/qsl-backup exposure, qsl-server/qsl-attachments dependency, and ad hoc port drift.

## Required markers

- `NA0518_QWORK_PROOF_FILES_VERIFIED_OK`
- `NA0518_D411_INHERITANCE_CONSUMED_OK`
- `NA0518_OPERATOR_PTY_CONTEXT_CONSUMED_OK`
- `NA0518_PTY_BROADENING_REJECTED_OK`
- `NA0518_FORWARDING_KEY_OPTIONS_REVIEWED_OK`
- `NA0518_DEDICATED_FORWARDING_KEY_SELECTED_OK`
- `NA0518_AUTHORIZED_KEYS_TEMPLATE_SELECTED_OK`
- `NA0518_OPERATOR_ACTION_CHECKLIST_SELECTED_OK`
- `NA0518_FUTURE_PROOF_OUTPUTS_SELECTED_OK`
- `NA0518_PROOF_REJECTION_RULES_SELECTED_OK`
- `NA0518_QSL_SERVER_ATTACHMENTS_BOUNDARY_OK`
- `NA0518_NO_REMOTE_ACTION_BY_CODEX_OK`
- `NA0518_NO_SSH_EXECUTION_BY_CODEX_OK`
- `NA0518_NO_AUTHORIZED_KEYS_MUTATION_OK`
- `NA0518_NO_KEY_GENERATION_INSTALLATION_OK`
- `NA0518_NO_QSC_SOURCE_TEST_FUZZ_CARGO_MUTATION_OK`
- `NA0518_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0518_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0518_SSH_FORWARDING_DEDICATED_KEY_OPERATOR_ACTION_READY`
- `NA0518_SELECTED_NA0519_SUCCESSOR_OK`
- `NA0518_ONE_READY_INVARIANT_OK`

## Static validation

Run and record:

- `git diff --check`
- exact five-path scope guard.
- link-check.
- leak-scan.
- overclaim scan.
- classifier.
- PR body preflight.
- goal-lint.
- marker proof for NA-0518 evidence.
- private key block scan on checked-in evidence.
- proof checked-in evidence includes no private key, passphrase, token, password, credential, production endpoint, or backup material.
- queue and decision proof.

## Required local validation

Run and record:

- `cargo test -p qsc --locked --test same_host_client_to_client_e2e -- --test-threads=1 --nocapture`
- `cargo test -p qsc --locked --test receive_e2e -- --test-threads=1 --nocapture`
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

## Forbidden commands

Validation must not run SSH, scp, sftp, rsync to remote, qsc send/receive, remote E2EE, ssh-keygen, ssh-keyscan, sudo, qwork, qstart, qresume, qsl-backup, backup, or restore.

## Post-fix hardening review

1. Correctness under stress: the lane fails closed by selecting a dedicated key, rejecting PTY broadening, and requiring proof review before any forwarding use.
2. Minimality: repository mutation is governance/testplan/traceability/journal only.
3. Maintainability: option review, template, operator checklist, proof outputs, and rejection rules are explicit and reusable.
4. Coverage quality: validation separates SSH forwarding authorization from forwarding capability proof and E2EE success proof.
5. Cross-lane stability: qsc source/tests/workflows/dependencies remain untouched, preserving macOS/Linux behavior for existing gates.
