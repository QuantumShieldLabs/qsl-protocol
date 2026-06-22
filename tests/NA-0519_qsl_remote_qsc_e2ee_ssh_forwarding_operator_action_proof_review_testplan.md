Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-22

# NA-0519 proof review testplan

## Scope

This testplan validates the NA-0519 proof-review-only lane. It verifies that the governance evidence accepts or rejects operator-provided redacted SSH forwarding proof without Codex running SSH, testing forwarding, generating keys, installing keys, editing authorized_keys, mutating SSH config, mutating known_hosts, mutating remote host state, running qsc send/receive, running remote E2EE, or using qsl-server/qsl-attachments.

## Required local validation

Run from the qsl-protocol repo root:

```bash
git diff --check
```

Scope guard must show exactly these five changed paths for the evidence PR:

```text
docs/governance/evidence/NA-0519_qsl_remote_qsc_e2ee_ssh_forwarding_operator_action_proof_review.md
tests/NA-0519_qsl_remote_qsc_e2ee_ssh_forwarding_operator_action_proof_review_testplan.md
DECISIONS.md
TRACEABILITY.md
docs/ops/ROLLING_OPERATIONS_JOURNAL.md
```

Queue and decision proof must show:

- READY_COUNT 1.
- READY NA-0519.
- NA-0518 DONE.
- NA-0517 DONE.
- NA-0516 DONE.
- D-1025 once.
- D-1026 once.
- D-1027 once after patch.
- D-1028 absent before closeout.
- Duplicate decision count 0.

Evidence marker proof must find:

- `NA0519_QWORK_PROOF_FILES_VERIFIED_OK`
- `NA0519_D412_INHERITANCE_CONSUMED_OK`
- `NA0519_OPERATOR_PROOF_CONSUMED_OK`
- `NA0519_DEDICATED_FORWARDING_KEY_PROOF_ACCEPTED_OK`
- `NA0519_AUTHORIZED_KEYS_OPTIONS_REVIEWED_OK`
- `NA0519_NO_PTY_AGENT_X11_OK`
- `NA0519_PRIVILEGE_BACKUP_QWORK_QSLBACKUP_REVIEWED_OK`
- `NA0519_CLEANUP_REVOCATION_REVIEWED_OK`
- `NA0519_SSH_FORWARDING_OPERATOR_PROOF_ACCEPTED_WITH_COMPATIBILITY_CAVEAT`
- `NA0519_SELECTED_NA0520_SUCCESSOR_OK`
- `NA0519_NO_REMOTE_ACTION_BY_CODEX_OK`
- `NA0519_NO_SSH_EXECUTION_BY_CODEX_OK`
- `NA0519_NO_AUTHORIZED_KEYS_MUTATION_OK`
- `NA0519_NO_KEY_GENERATION_INSTALLATION_OK`
- `NA0519_NO_QSC_SOURCE_TEST_FUZZ_CARGO_MUTATION_OK`
- `NA0519_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0519_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0519_ONE_READY_INVARIANT_OK`

Secret-material scans must prove the evidence does not contain:

- OpenSSH private-key block headers;
- RSA private-key block headers;
- generic private-key block headers;
- passphrases;
- passwords;
- tokens;
- unrelated authorized_keys entries;
- known_hosts dumps.

Required project validation:

```bash
cargo test -p qsc --locked --test same_host_client_to_client_e2e -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test receive_e2e -- --test-threads=1 --nocapture
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

## Forbidden validation

The NA-0519 validation must not run:

- SSH;
- scp;
- sftp;
- rsync to a remote;
- qsc send/receive;
- remote E2EE;
- key generation;
- key installation;
- authorized_keys mutation;
- SSH config mutation;
- known_hosts mutation;
- sudo/admin action;
- qwork/qstart/qresume;
- qsl-backup.

## Acceptance criteria

- qwork proof files are read and copied, not regenerated.
- qwork proof fields and live refs are fresh before fetch.
- Operator proof is consumed as operator-supplied proof only.
- Dedicated forwarding key proof is accepted with fingerprint `SHA256:7guT/Hd72vuu+u1Jrwpuy0hSFETVrxYaXHwupuFfaPM`.
- `restrict`, `port-forwarding`, loopback `permitlisten`, loopback `permitopen`, forced no-shell command, no PTY, no agent forwarding, and no X11 forwarding are reviewed.
- qslcodex no-sudo, no privileged group, no backup exposure, no qwork, and no qsl-backup proof are reviewed.
- Cleanup/revocation command is documented.
- Classification is `SSH_FORWARDING_OPERATOR_PROOF_ACCEPTED_WITH_COMPATIBILITY_CAVEAT`.
- Selected successor is NA-0520 forwarding capability probe.
- No remote action occurs in NA-0519.
- No qsc source/test/fuzz/Cargo, workflow/script/helper, dependency/lockfile, corpus/vector/input, formal/refimpl/service/public/backup, qsl-server, or qsl-attachments mutation occurs.
- No public-readiness, production-readiness, public-internet-readiness, external-review-complete, crypto-complete, replay-proof, downgrade-proof, secret-material-complete, side-channel-free, vulnerability-free, bug-free, or perfect-crypto claim is introduced.
- Exactly one READY item remains before closeout.
