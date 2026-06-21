Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-21

# NA-0517 remote qsc E2EE transport remediation scope authorization testplan

## Purpose

Validate the NA-0517 authorization-only governance/security lane. This testplan verifies qwork proof-file handling, NA-0516 / D410 inheritance, qsc-native forwarding-free review, SSH forwarding policy review, forwarding model selection, future proof rules, scope guard, claim boundaries, and local validation.

## Scope guard

Allowed changed paths for the NA-0517 evidence PR:

- `docs/governance/evidence/NA-0517_qsl_remote_qsc_e2ee_transport_remediation_scope_authorization_plan.md`
- `tests/NA-0517_qsl_remote_qsc_e2ee_transport_remediation_scope_authorization_testplan.md`
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
- `origin/main` equals or descends from `5be44390c6c5`.
- READY_COUNT is 1.
- READY item is NA-0517.
- NA-0516, NA-0515, and NA-0514 are DONE.
- D-1021 exists once.
- D-1022 exists once.
- D-1023 is absent before patch and exists once after patch.
- No duplicate decision IDs exist.

## Required inheritance proof

Evidence must record:

- NA-0516 completed and NA-0517 restored READY.
- D409 stopped before repo mutation due literal-dollar-HOME remote path mistake.
- D410 cleaned the D409 remote literal-dollar-HOME tree and local sensitive runtime root.
- D410 hardened remote path linting.
- D410 classification is `REMOTE_E2EE_TRANSPORT_FAILURE`.
- Retained qsc was preserved and rechecked.
- Local qsc provenance was recorded.
- No qsl-server use.
- No qsl-attachments use.
- No send/receive/reply completed.
- No negative boundary completed.
- SSH reverse forwarding was refused.
- SSH local forwarding was administratively prohibited.
- No custom proxy introduced.
- No qsc source/test/fuzz/Cargo mutation.
- No workflow/dependency/corpus/formal/refimpl/service/public/backup mutation.
- No public-readiness claim and no production-readiness claim.

## qsc native forwarding-free review proof

Evidence must show read-only inspection of current qsc tests and command/transport modules.

Required findings:

- qsc send exposes only relay transport.
- qsc receive active path requires relay transport.
- qsc handshake init/poll require relay URL.
- qsc relay serve binds loopback and stores inbox data in memory.
- receive `--file` is not an offline mailbox import path.
- No built-in forwarding-free remote E2EE path exists today.
- Forwarding-free retry would require out-of-scope qsc implementation work, service use, public exposure, or a custom helper/proxy.

## SSH forwarding policy review proof

Evidence must compare:

- Existing qslcodex key with narrowly enabled forwarding.
- Separate dedicated forwarding key.
- qsc-native forwarding-free path.
- Operator-managed sshd/account policy.
- qsl-server/qsl-attachments transport.
- Deferral.

Required selected policy:

- `SSH_FORWARDING_OPERATOR_ACTION_AUTHORIZATION_READY`.
- Separate dedicated forwarding key selected as the safer default.
- Existing qslcodex key deferred as fallback only.
- Key-level constraints preferred over account-wide or daemon-wide forwarding changes.
- No PTY broadening.
- No agent forwarding.
- No X11 forwarding.
- No sudo/admin.
- No backup exposure.
- No qwork/qsl-backup exposure.
- No qsl-server/qsl-attachments.

## Forwarding model proof

Evidence must compare:

- Shape 1: Build-local qsc relay plus remote reverse forwarding.
- Shape 2: Inspiron remote qsc relay plus local forwarding.

Required selected model:

- Shape 1 first.
- Loopback-only remote reverse forwarding.
- Default remote listen endpoint `127.0.0.1:39176`, or one explicitly recorded replacement port if unavailable in a future lane.
- No public exposure.
- No long-running remote relay process.
- Cleanup by stopping local relay and closing the SSH tunnel.

## Required markers

- `NA0517_QWORK_PROOF_FILES_VERIFIED_OK`
- `NA0517_D410_TRANSPORT_FAILURE_CONSUMED_OK`
- `NA0517_D409_RESIDUE_CLEANUP_CONSUMED_OK`
- `NA0517_QSC_NATIVE_FORWARDING_FREE_REVIEWED_OK`
- `NA0517_QSC_NATIVE_FORWARDING_FREE_REJECTED_OK`
- `NA0517_SSH_FORWARDING_POLICY_REVIEWED_OK`
- `NA0517_FORWARDING_MODEL_SELECTED_OK`
- `NA0517_DEDICATED_FORWARDING_KEY_SELECTED_OK`
- `NA0517_OPERATOR_ACTION_ONLY_SELECTED_OK`
- `NA0517_NO_REMOTE_ACTION_BY_CODEX_OK`
- `NA0517_NO_SSH_EXECUTION_BY_CODEX_OK`
- `NA0517_NO_AUTHORIZED_KEYS_MUTATION_OK`
- `NA0517_NO_QSC_SOURCE_TEST_FUZZ_CARGO_MUTATION_OK`
- `NA0517_NO_QSL_SERVER_ATTACHMENTS_OK`
- `NA0517_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0517_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0517_SSH_FORWARDING_OPERATOR_ACTION_AUTHORIZATION_READY`
- `NA0517_SELECTED_NA0518_SUCCESSOR_OK`
- `NA0517_ONE_READY_INVARIANT_OK`

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
- marker proof for NA-0517 evidence.
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

1. Correctness under stress: the lane fails closed by rejecting existing-setting E2EE retry and requiring operator-owned proof before future transport use.
2. Minimality: repository mutation is governance/testplan/traceability/journal only.
3. Maintainability: qsc-native transport facts, SSH policy choices, and future proof rules are explicit and reusable.
4. Coverage quality: validation separates authorization proof from E2EE success proof and keeps send/receive/reply unclaimed.
5. Cross-lane stability: qsc source/tests/workflows/dependencies remain untouched, preserving macOS/Linux behavior for existing gates.
