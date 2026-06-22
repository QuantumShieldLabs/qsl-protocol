Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-22

# NA-0520 SSH forwarding capability probe implementation testplan

## Scope

This testplan validates the NA-0520 forwarding-capability probe evidence. NA-0520 is limited to proving whether the dedicated qslcodex forwarding key can establish the selected loopback-only reverse-forwarding path and carry one synthetic marker from Inspiron loopback to a Build-local listener.

NA-0520 validation must not run qsc E2EE, qsc send/receive, qsl-server, qsl-attachments, package installation, sudo/admin action, SSH key generation/installation, authorized_keys mutation, known_hosts mutation, remote file writes, remote host mutation, qwork/qstart/qresume, qsl-backup, qsc source/test/fuzz/Cargo mutation, workflow/script/helper mutation, dependency/lockfile mutation, corpus/vector/input mutation, or formal/refimpl/service/public/backup mutation.

## Required startup proof

qwork proof files must exist and be read without rerunning qwork:

```text
/srv/qbuild/work/NA-0520/.qwork/startup.qsl-protocol.kv
/srv/qbuild/work/NA-0520/.qwork/startup.qsl-protocol.json
```

Required proof fields:

- `startup_result=OK`
- `lane=NA-0520`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0520/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0520`
- `requested_lane_status=READY`

Before fetch, proof HEAD must equal live HEAD and proof `origin_main` must equal live `origin/main`.

## Required probe proof

The evidence must prove:

- NA-0519 / D413 inheritance was consumed.
- The dedicated forwarding key path was used through a proof-root-local SSH config.
- Local port `39176` was free before listener start.
- The local listener bound only to `127.0.0.1:39176`.
- The reverse-forward command used `-N -T`, `ExitOnForwardFailure=yes`, and `-R 127.0.0.1:39176:127.0.0.1:39176`.
- The reverse-forward process stayed alive after the startup wait.
- Exactly one remote trigger command was run through the existing operational `inspiron` path.
- The remote trigger used loopback `127.0.0.1:39176`.
- The remote trigger wrote no files and required no PTY or sudo/admin action.
- The synthetic marker traversed the tunnel and returned `NA0520_TUNNEL_ACK_OK`.
- Cleanup found no proof-root listener process and no SSH process using the proof-root forwarding config.

## Required markers

Evidence marker proof must find:

- `NA0520_FORWARDING_PROOF_REVIEW_CONSUMED_OK`
- `NA0520_DEDICATED_FORWARDING_KEY_USED_OK`
- `NA0520_LOOPBACK_LISTENER_STARTED_OK`
- `NA0520_REVERSE_FORWARD_STARTED_OK`
- `NA0520_EXIT_ON_FORWARD_FAILURE_OK`
- `NA0520_REMOTE_TRIGGER_STARTED_OK`
- `NA0520_TUNNEL_MARKER_SENT_OK`
- `NA0520_TUNNEL_ACK_RECEIVED_OK`
- `NA0520_TUNNEL_MARKER_TRAVERSED_OK`
- `NA0520_NO_PTY_REQUIRED_OK`
- `NA0520_NO_AGENT_X11_FORWARDING_OK`
- `NA0520_NO_QSL_SERVER_ATTACHMENTS_OK`
- `NA0520_NO_REMOTE_E2EE_OK`
- `NA0520_NO_REMOTE_FILE_WRITE_OK`
- `NA0520_CLEANUP_COMPLETED_OK`
- `NA0520_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0520_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0520_ONE_READY_INVARIANT_OK`

## Required local validation

Run from the qsl-protocol repo root after the governance patch:

```bash
git diff --check
```

Scope guard must show exactly these five changed paths for the implementation PR:

```text
docs/governance/evidence/NA-0520_qsl_remote_qsc_e2ee_ssh_forwarding_capability_probe_implementation_harness.md
tests/NA-0520_qsl_remote_qsc_e2ee_ssh_forwarding_capability_probe_implementation_testplan.md
DECISIONS.md
TRACEABILITY.md
docs/ops/ROLLING_OPERATIONS_JOURNAL.md
```

Queue and decision proof before PR must show:

- READY_COUNT 1.
- READY NA-0520.
- NA-0519 DONE.
- NA-0518 DONE.
- NA-0517 DONE.
- D-1027 once.
- D-1028 once.
- D-1029 once after patch.
- D-1030 absent.
- Duplicate decision count 0.

Secret-material scans must prove the checked-in evidence does not contain:

- OpenSSH private-key block headers;
- RSA private-key block headers;
- generic private-key block headers;
- API token style fixtures;
- private key contents;
- passphrase material;
- token material;
- password material;
- production endpoint secrets;
- backup private material;
- authorized_keys dumps;
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

The NA-0520 validation must not run:

- qsc E2EE;
- qsc send/receive;
- qsl-server;
- qsl-attachments;
- package install;
- sudo/admin action;
- ssh-keygen;
- ssh-keyscan;
- authorized_keys mutation;
- SSH config mutation outside proof root;
- known_hosts mutation;
- remote file writes;
- remote source checkout/build;
- qwork/qstart/qresume;
- qsl-backup;
- qsc source/test/fuzz/Cargo mutation;
- workflow/script/helper mutation;
- dependency/lockfile mutation;
- corpus/vector/input mutation;
- formal/refimpl/service/public/backup mutation.

## Acceptance criteria

- qwork proof files are verified without rerunning qwork.
- NA-0520 is the sole READY item at startup.
- NA-0519 / D413 inheritance is consumed.
- The dedicated forwarding key proof is consumed without reading private key contents.
- The proof-root SSH config is safe and requires no PTY, agent forwarding, or X11 forwarding.
- The Build-local listener binds only to `127.0.0.1:39176`.
- The SSH reverse-forward starts with `ExitOnForwardFailure=yes`.
- The remote trigger sends one synthetic marker through remote loopback and receives the ACK.
- Cleanup completes.
- Result classification is exactly one of the allowed classifications; for this run it is `SSH_FORWARDING_CAPABILITY_PROBE_PASS`.
- Selected successor is `NA-0521 -- QSL Build-to-Inspiron Remote qsc E2EE Retry Implementation Harness`.
- No qsc E2EE, qsc send/receive, qsl-server, qsl-attachments, remote file write, package install, sudo/admin action, key generation/installation, authorized_keys mutation, known_hosts mutation, qwork/qstart/qresume, qsl-backup, qsc source/test/fuzz/Cargo mutation, workflow/script/helper/dependency mutation, corpus/vector/input mutation, or formal/refimpl/service/public/backup mutation occurs.
- No public-readiness, production-readiness, public-internet-readiness, external-review-complete, crypto-complete, replay-proof, downgrade-proof, secret-material-complete, side-channel-free, vulnerability-free, bug-free, or perfect-crypto claim is introduced.
- Exactly one READY item remains before closeout.
