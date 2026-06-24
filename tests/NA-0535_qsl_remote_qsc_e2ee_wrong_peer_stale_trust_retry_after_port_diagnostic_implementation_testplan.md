Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-24

# NA-0535 Remote qsc E2EE Wrong-Peer / Stale-Trust Retry After Port Diagnostic Implementation Testplan

## Objective

Record the validation plan and evidence expectations for NA-0535, which retries remote qsc E2EE wrong-peer and stale/replaced-peer negative boundaries only after a fresh integrated port 39176 marker/ACK precheck.

## Required gates

- qwork proof files exist, are read without rerunning qwork, and match live pre-fetch HEAD/origin.
- READY_COUNT is 1 and the sole READY item is NA-0535.
- D-1058 and D-1059 exist once; D-1060 and D-1061 are absent before patch.
- D440/D439/D436/D435/D425/D419 inheritance is consumed.
- Retained remote qsc path, owner, mode, size, hash, and help output match NA-0526/D425.
- Current local qsc is built from the clean checkout under proof-root target dir.
- If local/remote binary hashes differ, qsc runtime/dependency paths since D425 must be empty or the lane stops.
- Command manifest is written before integrated forwarding or E2EE execution.
- Local/remote boundary and port-state rechecks pass.
- Integrated marker traversal and ACK pass before any qsc E2EE command.
- Remote writes after precheck are confined to `/home/qslcodex/qsl-remote-test/e2ee/<PROOF_ID>/`.
- No qsl-server or qsl-attachments use occurs.

## Executed remote qsc validation

- Establish isolated local Alice, local Alice2, and remote Bob qsc roots.
- Establish Alice/Bob identity, contact, trusted-device, relay inbox, and handshake state.
- Prove baseline Alice-to-Bob qsc send/receive succeeds with `recv_commit`.
- Execute wrong-peer negative by receiving Bob mailbox with `--from charlie`; expect fail-closed, no plaintext, no new output artifact, and unchanged selected Bob/Alice session digest.
- Execute stale/replaced-peer negative by initiating a second `alice` identity from Alice2; expect `identity_mismatch` / `peer_mismatch` and unchanged selected Bob/Alice session digest.
- Prove valid Alice-to-Bob path still succeeds after negatives.
- Scan qsc/SSH stdout and stderr for generated synthetic passphrase values, generated route-token values, private key blocks, API/bearer markers, and private/passphrase marker fixtures.
- Clean up local sensitive runtime, remote E2EE root, qsc relay, SSH forward, and port 39176 listeners.

## Required local validation

- `git diff --check`
- exact five-path scope guard
- link-check
- leak-scan
- overclaim scan
- classifier
- PR body preflight
- goal-lint
- marker proof for NA-0535 evidence
- private key/passphrase/token/password material scan
- proof local sensitive runtime root was deleted
- proof remote E2EE root was deleted
- `cargo audit --deny warnings`
- `cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock`
- `cargo test -p qsc --locked --test same_host_client_to_client_e2e -- --test-threads=1 --nocapture`
- `cargo test -p qsc --locked --test receive_e2e -- --test-threads=1 --nocapture`
- `cargo test -p qsc --locked --test key_lifecycle_zeroization_expansion -- --test-threads=1 --nocapture`
- `cargo test -p qsc --locked --test secret_material_diagnostic_boundary -- --test-threads=1 --nocapture`
- `cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture`
- `python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics`
- `python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus`
- `python3 formal/run_model_checks.py`
- `cargo fmt --check`
- `sh -n scripts/ci/qsc_adversarial.sh`
- `bash -n scripts/ci/qsc_adversarial.sh`

## Expected result markers

- `REMOTE_E2EE_WRONG_PEER_STALE_TRUST_NEGATIVES_PASS`
- integrated listener `marker_match=true`
- integrated listener `ack_sent=true`
- remote trigger `ack_received=true`
- baseline receive `recv_commit`
- wrong-peer fail-closed marker class `protocol_inactive` or `qsp_hdr_auth_failed`
- stale/replaced-peer marker class `identity_mismatch` / `peer_mismatch`
- valid receive after negatives `recv_commit`
- cleanup `RESULT=PASS`

## Stop conditions

Stop if qwork proof is stale or missing, queue is not sole READY NA-0535, D-1060 already exists, retained qsc hash is unexpected, integrated marker/ACK precheck fails, command manifest cannot be completed from existing qsc surfaces, boundary recheck fails, qsl-server or qsl-attachments are required or used, package install or remote source build is required, qsc source/test/fuzz/Cargo mutation is required, wrong-peer or stale/replaced-peer fails open, no-mutation check fails, valid path fails after negatives, secret material appears in output, cleanup cannot be proven, root or nested audit is red, required qsc tests fail, or any forbidden readiness/completeness claim is introduced.

## Claim boundary

This testplan supports bounded internal remote qsc hardening only. It does not make a public-readiness, production-readiness, public-internet-readiness, external-review-complete, crypto-complete, identity-complete, trust-complete, replay-proof, downgrade-proof, secret-material-complete, side-channel-free, vulnerability-free, bug-free, or perfect-crypto claim.
