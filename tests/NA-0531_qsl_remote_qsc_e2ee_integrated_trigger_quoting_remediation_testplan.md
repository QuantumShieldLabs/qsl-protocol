Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-06-24

# NA-0531 qsl remote qsc E2EE integrated trigger quoting remediation testplan

Goals: G1, G2, G3, G4, G5

## Purpose

Record validation for NA-0531, which remediated the D433 integrated remote trigger quoting failure by replacing the complex `python3 -c` shape with a locally rehearsed stdin Python script and proving marker/ACK traversal through one integrated listener / dedicated-key reverse-forward / trigger lifetime.

## Expected classification

`REMOTE_TRIGGER_QUOTING_REMEDIATION_MARKER_TRAVERSAL_PASS`

This classification authorizes only successor selection for a later qsc E2EE retry lane. It does not prove qsc E2EE, identity correctness, trust correctness, replay resistance, downgrade resistance, side-channel safety, or secret lifecycle.

## Scope guard

Allowed changed paths:

- `docs/governance/evidence/NA-0531_qsl_remote_qsc_e2ee_integrated_trigger_quoting_remediation_harness.md`
- `tests/NA-0531_qsl_remote_qsc_e2ee_integrated_trigger_quoting_remediation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

No qsc source/test/fuzz/Cargo, workflow/script/helper, dependency/lockfile, corpus/vector/input, formal/refimpl/service/public/backup, qsl-server, qsl-attachments, public-doc, backup, or service path may change.

## qwork and queue checks

Expected startup proof:

- qwork proof files exist and are copied into the proof root.
- Codex does not run `qwork`, `qstart`, or `qresume`.
- qwork proof HEAD and `origin/main` match live refs before fetch.
- READY_COUNT is 1.
- READY item is NA-0531.
- NA-0530 and NA-0529 are DONE.
- D-1050 and D-1051 exist once.
- D-1052 and D-1053 are absent before patch.
- Duplicate decision count is 0.

## Inheritance checks

Expected inherited facts:

- D434 restored NA-0531 READY and recorded D433 classification `REMOTE_E2EE_INTEGRATED_TRIGGER_QUOTING_FAILURE`.
- D433 failed before connecting due remote trigger quoting.
- D433 listener `marker_match`, `ack_sent`, and `ack_received` were false.
- D433 ran no qsc E2EE and no qsc send/receive.
- D433 D432 residue cleanup passed.
- D433 retained qsc recheck passed.
- D431 manual integrated forwarding marker/ACK proof passed.

## Retained qsc check

Expected retained qsc metadata/hash:

- path `/home/qslcodex/qsl-remote-test/bin/qsc`
- owner/group `qslcodex/qslcodex`
- mode `700`
- size `102103920`
- SHA-256 `6bf9e59fdae397c2e0f88538d700cccbee80d229c6a979cc79555e39fea2b4f7`

Forbidden retained-qsc actions:

- no `qsc --help`
- no qsc E2EE
- no qsc send/receive
- no qsc protocol command

## Trigger command validation

Expected local validation:

- trigger script compiles with `python3 -m py_compile`
- local rehearsal prints `NA0531_LOCAL_TRIGGER_REHEARSAL_OK`
- marker is exactly `QSL_TRIGGER_QUOTING_REMEDIATION_SYNTHETIC_NA0531_20260624T053219Z_D435`
- marker contains no newline or shell-sensitive splitting characters
- safe remote command shape is `ssh -T inspiron 'python3 -' < <proof-root trigger script>`
- no `python3 -c` complex payload is used
- no remote temp file or remote file write is used

## SSH and boundary checks

Expected SSH/boundary results:

- safe `ssh -G inspiron` fields parsed without storing private key contents or full SSH config
- operational alias hazard `clearallforwardings yes` recorded but not used for reverse forwarding
- proof-root forwarding config effective `clearallforwardings no`
- dedicated key basename `qslcodex_forward_ed25519`
- no PTY, no agent forwarding, no X11 forwarding
- remote user `qslcodex`
- remote UID nonzero
- no privileged groups
- `sudo -n true` fails
- `/backup/qsl` absent or not readable
- qwork absent from remote PATH
- qsl-backup absent from remote PATH

## Integrated marker/ACK check

Expected integrated result:

- local listener binds to `127.0.0.1:39176`
- dedicated-key reverse-forward process stays alive with `ExitOnForwardFailure=yes`
- remote trigger starts with `NA0531_REMOTE_TRIGGER_STARTED_OK`
- remote trigger sends marker with `NA0531_TRIGGER_MARKER_SENT_OK`
- listener records `marker_match: true`
- listener records `ack_sent: true`
- remote trigger records `NA0531_TRIGGER_ACK_RECEIVED_OK`
- remote trigger records `NA0531_REMOTE_TRIGGER_DONE_OK`
- no corrected retry is used if the primary attempt passes

## Cleanup check

Expected cleanup:

- reverse-forward process terminated
- listener process exited
- no concrete NA-0531 listener, harness, or reverse-forward process remains
- no listening socket remains on port `39176`
- cleanup proof records any proof-method false positive as a recovered failure

## Required validation commands

Required local validation after governance patch:

- `git diff --check`
- exact five-path scope guard
- link-check
- leak-scan
- overclaim scan
- classifier
- PR body preflight
- goal-lint
- marker proof for NA-0531 evidence
- proof evidence contains no private key blocks or API token-style fixtures
- proof no remote file write happened
- proof no qsc E2EE happened
- proof no qsc send/receive happened
- cleanup proof
- `cargo audit --deny warnings`
- `cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock`
- `cargo test -p qsc --locked --test same_host_client_to_client_e2e -- --test-threads=1 --nocapture`
- `cargo test -p qsc --locked --test receive_e2e -- --test-threads=1 --nocapture`
- `cargo test -p qsc --locked --test key_lifecycle_zeroization_expansion -- --test-threads=1 --nocapture`
- `cargo test -p qsc --locked --test secret_material_diagnostic_boundary -- --test-threads=1 --nocapture`
- `cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture`
- binding corpus validator
- all qsc fuzz corpus validator
- `python3 formal/run_model_checks.py`
- `cargo fmt --check`
- `sh -n scripts/ci/qsc_adversarial.sh`
- `bash -n scripts/ci/qsc_adversarial.sh`

## Boundary assertions

NA-0531 introduces no public-readiness, production-readiness, public-internet-readiness, crypto-complete, identity-complete, trust-complete, replay-proof, downgrade-proof, secret-material-complete, side-channel-free, vulnerability-free, bug-free, or perfect-crypto claim.
