Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-24

# NA-0536 Remote qsc E2EE Repeated-Run / Cleanup / Freshness Scope Authorization Testplan

## Objective

Record the validation plan and evidence expectations for NA-0536, an authorization-only lane that consumes NA-0535/D441 wrong-peer and stale-trust success and selects the next bounded repeated-run cleanup/freshness implementation successor.

## Required gates

- qwork proof files exist, are read without rerunning qwork, and match live pre-fetch HEAD/origin.
- READY_COUNT is 1 and the sole READY item is NA-0536.
- NA-0535 is DONE.
- NA-0534 is DONE.
- D-1060 and D-1061 exist once.
- D-1062 and D-1063 are absent before patch.
- Duplicate decision count is zero.
- Current main has `public-safety` success and `advisories` success.
- Root and nested qsc fuzz lockfiles retain `quinn-proto 0.11.15`.
- Cargo.toml drift check is empty.
- qsl-backup read-only boundary check passes without executing qsl-backup.

## Inheritance requirements

- D441 classification `REMOTE_E2EE_WRONG_PEER_STALE_TRUST_NEGATIVES_PASS` is consumed.
- D441 retained-qsc freshness recheck passed.
- D441 integrated port 39176 marker/ACK precheck passed.
- D441 baseline remote qsc E2EE setup passed.
- D441 wrong-peer negative passed.
- D441 stale/replaced-peer negative passed.
- D441 selected-state no-mutation checks passed.
- D441 valid path after negatives passed.
- D441 cleanup passed.
- D439 port 39176 diagnostic success is consumed.
- D435 trigger remediation success is consumed.
- D419 replay/corrupt negatives passed.
- No qsl-server or qsl-attachments boundary is reopened.

## Authorization requirements

- Residual inventory covers repeated-run freshness, cleanup robustness, repeated-run determinism, negative-path repeatability, redaction/no-secret-output, retained-binary policy, public evidence sync, and qsl-server/qsl-attachments deferral.
- Option review compares the eight required candidates.
- Selected classification is `REMOTE_E2EE_REPEATED_RUN_CLEANUP_FRESHNESS_IMPLEMENTATION_READY`.
- Selected successor is `NA-0537 -- QSL Remote qsc E2EE Repeated-Run / Cleanup / Freshness Implementation Harness`.
- Future command family, proof/redaction rules, stop conditions, stewardship reviews, prioritization matrix, and marker plan are recorded.
- NA-0536 performs no remote action, SSH execution, qsc send/receive, qsc E2EE, qsl-server use, or qsl-attachments use.

## Required local validation

- `git diff --check`
- exact five-path scope guard
- link-check
- leak-scan
- overclaim scan
- classifier
- PR body preflight
- goal-lint
- marker proof for NA-0536 evidence
- proof that evidence contains no private key blocks:
  - no OpenSSH private-key block marker
  - no RSA private-key block marker
  - no generic private-key block marker
- proof that checked-in evidence does not include private key/passphrase/token/password material
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

## Expected markers

- `REMOTE_E2EE_REPEATED_RUN_CLEANUP_FRESHNESS_IMPLEMENTATION_READY`
- `NA0537_D441_SUCCESS_CONSUMED_OK`
- `NA0537_RETAINED_QSC_HASH_RECHECKED_RUN1_OK`
- `NA0537_RETAINED_QSC_HASH_RECHECKED_RUN2_OK`
- `NA0537_PORT_39176_MARKER_ACK_RUN1_OK`
- `NA0537_PORT_39176_MARKER_ACK_RUN2_OK`
- `NA0537_REMOTE_E2EE_RUN1_OK`
- `NA0537_REMOTE_E2EE_RUN2_OK`
- `NA0537_NO_STALE_STATE_REUSE_OK`
- `NA0537_REMOTE_ROOT_CLEANUP_RUN1_OK`
- `NA0537_REMOTE_ROOT_CLEANUP_RUN2_OK`
- `NA0537_LOCAL_SENSITIVE_RUNTIME_CLEANUP_RUN1_OK`
- `NA0537_LOCAL_SENSITIVE_RUNTIME_CLEANUP_RUN2_OK`
- `NA0537_LOCAL_REMOTE_PORTS_CLOSED_OK`
- `NA0537_NO_SECRET_OUTPUT_OK`
- `NA0537_NO_QSL_SERVER_ATTACHMENTS_OK`
- `NA0537_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0537_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0537_ONE_READY_INVARIANT_OK`

## Stop conditions

Stop if qwork proof is missing/stale/not OK, queue is not sole READY NA-0536, D-1061 is absent, D-1062 already exists at startup, disk usage is at or above 95%, required inheritance is absent, public-safety/advisories are red or missing on main, root or nested audit is red, any required qsc test fails, corpus validators fail, formal checks fail, qsl-backup source-list verification fails, checked-in evidence contains private material, any forbidden mutation path changes, more than one READY item exists, or any forbidden public/production/security-completion claim is introduced.

## Claim boundary

This testplan supports bounded internal authorization only. It does not make a public-readiness, production-readiness, public-internet-readiness, external-review-complete, crypto-complete, identity-complete, trust-complete, replay-proof, downgrade-proof, secret-material-complete, side-channel-free, vulnerability-free, bug-free, or perfect-crypto claim.
