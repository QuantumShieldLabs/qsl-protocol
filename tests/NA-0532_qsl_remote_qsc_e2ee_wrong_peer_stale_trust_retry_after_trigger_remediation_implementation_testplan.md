Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-06-24

# NA-0532 qsl remote qsc E2EE wrong-peer stale-trust retry after trigger remediation implementation testplan

Goals: G1, G2, G3, G4, G5

## Purpose

Record validation for NA-0532, which retried the remote qsc E2EE wrong-peer / stale-trust lane after D435 trigger remediation, but stopped before qsc E2EE because the mandatory integrated forwarding precheck failed.

## Expected classification

`REMOTE_E2EE_INTEGRATED_FORWARDING_PRECHECK_FAILURE`

This classification records a transport precondition failure only. It does not prove qsc E2EE, wrong-peer rejection, stale-trust rejection, identity correctness, trust correctness, replay resistance, downgrade resistance, side-channel safety, or secret lifecycle.

## Scope guard

Allowed changed paths:

- `docs/governance/evidence/NA-0532_qsl_remote_qsc_e2ee_wrong_peer_stale_trust_retry_after_trigger_remediation_implementation_harness.md`
- `tests/NA-0532_qsl_remote_qsc_e2ee_wrong_peer_stale_trust_retry_after_trigger_remediation_implementation_testplan.md`
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
- READY item is NA-0532.
- NA-0531 and NA-0530 are DONE.
- D-1052 and D-1053 exist once.
- D-1054 and D-1055 are absent before patch.
- Duplicate decision count is 0.

## Inheritance checks

Expected inherited facts:

- D435 classification is `REMOTE_TRIGGER_QUOTING_REMEDIATION_MARKER_TRAVERSAL_PASS`.
- D435 safe trigger shape is `ssh -T inspiron 'python3 -' < <proof-root trigger script>`.
- D435 integrated marker traversal and ACK passed.
- D435 ran no qsc E2EE and no qsc send/receive.
- D433 D432 residue cleanup passed.
- D433 retained qsc recheck passed.
- D425 retained qsc hash is `6bf9e59fdae397c2e0f88538d700cccbee80d229c6a979cc79555e39fea2b4f7`.
- D419 replay/corrupt negatives passed.
- No qsl-server or qsl-attachments boundary is inherited.

## Retained qsc and local provenance checks

Expected retained qsc metadata/hash:

- path `/home/qslcodex/qsl-remote-test/bin/qsc`
- owner/group `qslcodex/qslcodex`
- mode `700`
- size `102103920`
- SHA-256 `6bf9e59fdae397c2e0f88538d700cccbee80d229c6a979cc79555e39fea2b4f7`
- remote `--help` captured locally only

Expected local qsc proof:

- `cargo build -p qsc --locked --bin qsc`
- proof-root target dir
- local `--help` success
- local size recorded
- local hash recorded
- if local hash differs, qsc runtime/dependency diff since D425 is empty

## Command manifest checks

Expected manifest artifacts:

- proof-root `command_manifest/na0532_command_manifest.md`
- proof-root `command_manifest/na0532_command_manifest.json`

Expected manifest content:

- retained-qsc recheck commands
- integrated forwarding precheck commands
- local and remote qsc command shapes
- relay and forwarding commands
- runtime roots
- synthetic labels
- passphrase-file handling by path only
- wrong-peer negative plan
- stale-trust/replaced-peer negative plan
- selected-state/no-mutation checks
- cleanup commands
- no qsl-server use
- no qsl-attachments use
- no remote writes outside directive E2EE root after precheck

## Boundary checks

Expected SSH/boundary results:

- safe `ssh -G inspiron` fields parsed without storing private key contents or full SSH config
- operational alias `clearallforwardings yes` recorded but proof-root forwarding config uses `ClearAllForwardings no`
- no PTY, no agent forwarding, no X11 forwarding
- remote user `qslcodex`
- remote UID nonzero
- no privileged groups
- `sudo -n true` fails
- `/backup/qsl` absent or not readable
- qwork absent from remote PATH
- qsl-backup absent from remote PATH
- directive-specific remote E2EE root absent before precheck

## Integrated marker/ACK precheck

Expected precheck behavior:

- local trigger script compiles with `python3 -m py_compile`
- local rehearsal passes
- marker contains no newline or shell-sensitive splitting characters
- local listener binds to `127.0.0.1:39176`
- proof-root reverse-forward uses `ExitOnForwardFailure=yes`
- remote trigger uses `ssh -T inspiron 'python3 -' < <proof-root trigger script>`
- qsc E2EE is forbidden unless listener marker match and ACK receipt pass

Observed expected failure for this lane:

- SSH reverse-forward exits before trigger
- stderr contains `remote port forwarding failed for listen port 39176`
- marker match false
- ACK sent false
- ACK received false
- no qsc E2EE

## E2EE deferral checks

Expected deferrals after precheck failure:

- baseline E2EE setup did not run
- wrong-peer negative did not run
- stale-trust/replaced-peer negative did not run
- selected qsc no-mutation proof is not applicable
- valid-path usability proof is deferred

## Cleanup check

Expected cleanup:

- listener stopped
- reverse-forward process absent
- local sensitive runtime removed
- remote E2EE root absent
- port `39176` free
- no proof-root listener/forward/qsc relay child process remains
- retained remote qsc unchanged

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
- marker proof for NA-0532 evidence
- proof evidence contains no private key blocks or API token-style fixtures
- proof no private key/passphrase/token/password material appears in checked-in evidence
- proof local sensitive runtime root was deleted
- proof remote E2EE root was deleted or absent
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

NA-0532 introduces no public-readiness, production-readiness, public-internet-readiness, crypto-complete, identity-complete, trust-complete, replay-proof, downgrade-proof, secret-material-complete, side-channel-free, vulnerability-free, bug-free, or perfect-crypto claim.
