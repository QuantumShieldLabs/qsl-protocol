Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-06-24

# NA-0534 qsl remote qsc E2EE reverse-forward port 39176 regression diagnostic implementation testplan

Goals: G1, G2, G3, G4, G5

## Purpose

Record validation for NA-0534, which diagnoses the reverse-forward port `39176` regression without running qsc E2EE, qsc send/receive, qsc protocol commands, qsl-server, or qsl-attachments.

## Expected classification

`REMOTE_FORWARD_PORT_39176_DIAGNOSTIC_MARKER_TRAVERSAL_PASS`

This classification proves only the bounded SSH reverse-forward marker/ACK diagnostic after port-state checks. It does not prove E2EE, identity correctness, trust correctness, replay resistance, downgrade resistance, side-channel safety, secret lifecycle, or qsc protocol correctness.

## Scope guard

Allowed changed paths:

- `docs/governance/evidence/NA-0534_qsl_remote_qsc_e2ee_reverse_forward_port_39176_regression_diagnostic_implementation_harness.md`
- `tests/NA-0534_qsl_remote_qsc_e2ee_reverse_forward_port_39176_regression_diagnostic_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

No `NEXT_ACTIONS.md`, qsc source/test/fuzz/Cargo, workflow/script/helper, dependency/lockfile, corpus/vector/input, formal/refimpl/service/public/backup, qsl-server, qsl-attachments, qshield, qshield-cli, qwork/qstart/qresume, qsl-backup, SSH key, authorized_keys, known_hosts, or SSH config outside the proof root may change.

## qwork and queue checks

Expected startup proof:

- qwork proof files exist and are copied into the proof root.
- Codex does not run `qwork`, `qstart`, or `qresume`.
- qwork proof HEAD and `origin/main` match live refs before fetch.
- READY_COUNT is 1.
- READY item is NA-0534.
- NA-0533, NA-0532, and NA-0531 are DONE.
- D-1056 and D-1057 exist once.
- D-1058 and D-1059 are absent before patch.
- duplicate decision ID count is 0.

## Inheritance checks

Required facts:

- D438 classification is `REMOTE_FORWARD_PORT_39176_REGRESSION_DIAGNOSTIC_IMPLEMENTATION_READY`.
- D438 records NA-0533 DONE and NA-0534 READY.
- D437 records NA-0532 closeout restored NA-0533.
- D436 classification is `REMOTE_E2EE_INTEGRATED_FORWARDING_PRECHECK_FAILURE`.
- D436 reverse-forward failed with `remote port forwarding failed for listen port 39176`.
- D436 local listener was ready and local trigger rehearsal passed.
- D436 remote trigger was never executed.
- D436 ran no qsc E2EE and no qsc send/receive.
- D435 classification is `REMOTE_TRIGGER_QUOTING_REMEDIATION_MARKER_TRAVERSAL_PASS`.
- D435 stdin-script trigger shape worked.
- D435 integrated marker traversal and ACK passed.
- D414 classification is `SSH_FORWARDING_CAPABILITY_PROBE_PASS`.
- D414 dedicated-key reverse-forward marker traversal and ACK passed.
- D413 accepted the dedicated forwarding key proof with loopback-only constraints.

## Command manifest checks

Expected:

- command manifest exists before remote diagnostics.
- manifest records safe `ssh -G` parse fields.
- manifest records proof-root SSH config fields.
- manifest records local port checks.
- manifest records remote boundary and remote port-state commands.
- manifest records optional bind-probe command.
- manifest records reverse-forward and stdin-script trigger commands.
- manifest records cleanup commands.
- manifest records no qsc E2EE, no qsc send/receive, no qsl-server/qsl-attachments, and no remote file write.

## SSH and port-state checks

Expected:

- operational alias `clearallforwardings yes` is not used for reverse forwarding.
- proof-root config uses the dedicated forwarding key path and `ClearAllForwardings no`.
- proof-root config has zero `ClearAllForwardings yes` matches.
- local `127.0.0.1:39176` is free before the probe.
- no proof-root listener/forward process exists before the probe.
- remote boundary command returns qslcodex, non-root UID, no privileged group, negative sudo, `/backup/qsl` absent or unreadable, qwork absent, and qsl-backup absent.
- remote pre-probe `ss` check shows no listener on port `39176`.
- remote bind probe prints `NA0534_REMOTE_LOOPBACK_BIND_PROBE_OK`.

## Integrated marker/ACK check

Expected corrected probe evidence:

- listener writes `ATTEMPT2_LISTENER_READY`.
- reverse-forward uses `ExitOnForwardFailure=yes`.
- reverse-forward writes `ATTEMPT2_SSH_FORWARD_ALIVE_AFTER_STARTUP`.
- remote port-state during forward shows `127.0.0.1:39176` listening.
- remote trigger prints `NA0534_TRIGGER_ACK_RECEIVED_OK`.
- listener result has `ok: true`, `marker_match: true`, and `ack_sent: true`.
- corrected driver prints `ATTEMPT2_MARKER_ACK_PASS`.

Recovered failure evidence required:

- first integrated attempt local process lifetime failure is recorded.
- failing command shape is recorded.
- recoverability classification is recorded.
- corrective action is recorded.
- final corrected result is recorded.

## Cleanup checks

Expected cleanup:

- local port `39176` closed after cleanup.
- no proof-root listener/forward/trigger process remains.
- remote port `39176` is not listening after cleanup.
- no remote cleanup is needed because no remote files were written.

## Required markers

Evidence marker proof must find:

- `REMOTE_FORWARD_PORT_39176_DIAGNOSTIC_MARKER_TRAVERSAL_PASS`
- `NA0534_REMOTE_LOOPBACK_BIND_PROBE_OK`
- `NA0534_TRIGGER_ACK_RECEIVED_OK`
- `ATTEMPT2_MARKER_ACK_PASS`
- `NA0534_NO_QSC_E2EE_OK`
- `NA0534_NO_QSC_SEND_RECEIVE_OK`
- `NA0534_NO_QSL_SERVER_ATTACHMENTS_OK`
- `NA0534_NO_REMOTE_FILE_WRITE_OK`
- `NA0534_CLEANUP_COMPLETED_OK`
- `NA0534_ONE_READY_INVARIANT_OK`

## Required local validation

Run and record:

- `git diff --check`
- exact five-path scope guard
- link-check
- leak-scan
- overclaim scan
- classifier
- PR body preflight
- goal-lint
- marker proof for NA-0534 evidence
- checked-in evidence contains no private key blocks
- proof evidence did not include private key/passphrase/token/password material
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

## PR body evidence requirements

The PR body must include:

- standalone `Goals: G1, G2, G3, G4, G5`
- impact statement
- no-regression statement
- tests/vectors statement
- D436 port 39176 failure consumed
- D435 trigger success consumed
- NA-0520 forwarding success consumed
- no qsc E2EE
- no qsc send/receive
- no qsl-server use
- no qsl-attachments use
- no remote file write
- no remote temp file
- no qwork/qsl-backup
- no qsc source/test/fuzz/Cargo mutation
- no workflow/script/helper/dependency mutation
- no corpus/vector/input mutation
- no public-readiness, production-readiness, crypto-complete, identity-complete, trust-complete, replay-proof, downgrade-proof, or public-internet-readiness claim

## Post-fix hardening review

Before declaring work complete, report:

- correctness under stress: single-lifetime probe avoids background process loss and proves port visibility during forward.
- minimality: checked-in changes remain the five allowed governance/testplan paths.
- maintainability: proof-root scripts are simple, local, and record exact commands.
- coverage quality: marker/ACK, bind probe, port-state, cleanup, scans, audits, qsc tests, validators, formal checks, and CI cover the lane risk.
- cross-lane stability: Linux/macOS CI must remain green or classifier-accepted; no platform-specific runtime code changed.

## Closeout preconditions

Optional closeout to NA-0535 may occur only after the implementation PR merges and post-merge public-safety is green inside the short attach/early-failure window. Closeout must restore the selected NA-0535 successor and must not implement NA-0535.
