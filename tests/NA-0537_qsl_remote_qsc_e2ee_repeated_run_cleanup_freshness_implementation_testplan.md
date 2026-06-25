Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-25

# NA-0537 Remote qsc E2EE Repeated-Run / Cleanup / Freshness Implementation Testplan

## Objective

Record the validation plan and acceptance evidence for NA-0537, which recovers the D445 Bob baseline receive peer-label command shape and executes two bounded synthetic remote qsc E2EE repeated runs with retained-qsc freshness, marker/ACK prechecks, selected negatives, cleanup, and no-secret-output review.

## Required startup gates

- qwork proof files exist and are read without rerunning qwork.
- qwork `.kv` and `.json` proofs record `startup_result=OK`, lane NA-0537, repo `qsl-protocol`, clean worktree/index/untracked state, READY_COUNT 1, queue top READY NA-0537, and requested lane READY.
- qwork proof HEAD and origin/main match live refs before fetch.
- qwork proof timestamp is after the D445 response timestamp.
- Disk usage is below the 95% stop threshold before fetch.
- READY_COUNT is 1 and the sole READY item is NA-0537.
- NA-0536, NA-0535, and NA-0534 are DONE.
- D-1062 and D-1063 exist once.
- D-1064 and D-1065 are absent before patch.
- Duplicate decision count is zero.
- Current main has `public-safety` success and `advisories` success.
- Root and nested qsc fuzz lockfiles retain `quinn-proto 0.11.15`.
- qsl-backup read-only boundary check passes without executing qsl-backup.

## Inheritance requirements

- D445 stopped with `REMOTE_E2EE_REPEATED_RUN1_BASELINE_FAILURE`.
- D445 marker/ACK precheck passed.
- D445 handshake progressed through Alice send commit.
- D445 Bob baseline receive failed closed with `qsp_hdr_auth_failed`.
- D445 suspected peer-label harness bug is consumed.
- D445 cleanup passed.
- D444 retained-qsc command-shape stop is consumed.
- D443 listener compile failure and response-writer violation are consumed.
- D442 authorized NA-0537.
- D441 classification `REMOTE_E2EE_WRONG_PEER_STALE_TRUST_NEGATIVES_PASS` is consumed.
- D439 port diagnostic success is consumed.
- D435 trigger remediation success is consumed.
- D419 replay/corrupt negatives passed.
- No qsl-server or qsl-attachments boundary is reopened.

## Peer-label proof requirements

- Existing qsc tests/source/help surfaces prove Bob receives Alice's message with Alice as the peer/session label.
- Bob baseline receive uses `--from alice`, not `--from bob`.
- Wrong-peer negative uses a distinct synthetic wrong peer label.
- Bob-to-Alice reply, if used, would use Bob as Alice's receive peer label.
- If the proof cannot be established, classification must be `REMOTE_E2EE_REPEATED_PEER_LABEL_SURFACE_AMBIGUOUS` and no qsc E2EE may run.

## Per-run runtime gates

- Retained remote qsc path, owner, mode, size, hash, and help are rechecked before each run.
- Generated listener, trigger, local harness, remote harness, and boundary checker scripts pass `python3 -m py_compile` before use.
- Local-only listener/trigger self-test passes before SSH marker precheck.
- Local and remote boundary checks pass before each run.
- Local and remote port 39176 are free before each run.
- Integrated marker traversal and ACK pass before qsc E2EE for each run.
- Remote writes are limited to `$HOME/qsl-remote-test/e2ee/<PROOF_ID>/run-<N>` and subpaths.
- Local sensitive runtime stays under the proof root and is deleted after each run.

## Run 1 acceptance

- Run 1 uses fresh local and remote roots.
- Valid Alice-to-Bob qsc E2EE path succeeds.
- Bob baseline receive uses `--from alice`.
- Wrong-peer repeat executes if safe.
- Wrong-peer repeat fails closed, produces no plaintext in checked-in evidence, and leaves selected state unchanged.
- Remote run-1 root is deleted.
- Local run-1 sensitive runtime is deleted.
- Local and remote port 39176 are closed after cleanup.
- Proof-root process scan is clean after cleanup.

## Run 2 acceptance

- Before run 2, run-1 local and remote roots are absent.
- Run 2 uses fresh local and remote roots.
- Valid Alice-to-Bob qsc E2EE path succeeds.
- Bob baseline receive uses `--from alice`.
- Stale/replaced-peer repeat executes if safe.
- Stale/replaced-peer repeat fails closed and leaves selected state unchanged.
- Remote run-2 root is deleted.
- Local run-2 sensitive runtime is deleted.
- Remote parent proof root is absent or removed when empty.
- Local and remote port 39176 are closed after cleanup.
- Proof-root process scan is clean after cleanup.
- Retained qsc remains unchanged after cleanup.

## Required local validation

- `git diff --check`
- exact five-path scope guard
- link-check
- leak-scan
- overclaim scan
- classifier
- PR body preflight
- goal-lint
- marker proof for NA-0537 evidence
- proof evidence contains no private key block markers
- proof evidence does not include private key/passphrase/token/password material
- proof local sensitive runtime root was deleted
- proof remote E2EE roots were deleted
- proof no stale proof-root processes remain
- response-writer safety policy proof present
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

- `REMOTE_E2EE_REPEATED_RUN_CLEANUP_FRESHNESS_PASS`
- `NA0537_D445_PEER_LABEL_RECOVERED_OK`
- `NA0537_D443_LISTENER_COMPILE_RECOVERED_OK`
- `NA0537_D443_RESPONSE_WRITER_RECOVERED_OK`
- `NA0537_D444_RETAINED_QSC_PREFLIGHT_RECOVERED_OK`
- `NA0537_PEER_LABEL_PROOF_OK`
- `NA0537_GENERATED_SCRIPTS_COMPILED_OK`
- `NA0537_LOCAL_LISTENER_SELFTEST_OK`
- `NA0537_RETAINED_QSC_HASH_RECHECKED_RUN1_OK`
- `NA0537_RETAINED_QSC_HASH_RECHECKED_RUN2_OK`
- `NA0537_PORT_39176_MARKER_ACK_RUN1_OK`
- `NA0537_PORT_39176_MARKER_ACK_RUN2_OK`
- `NA0537_REMOTE_E2EE_RUN1_OK`
- `NA0537_WRONG_PEER_FAIL_CLOSED_OK`
- `NA0537_REMOTE_ROOT_CLEANUP_RUN1_OK`
- `NA0537_LOCAL_SENSITIVE_RUNTIME_CLEANUP_RUN1_OK`
- `NA0537_REMOTE_E2EE_RUN2_OK`
- `NA0537_STALE_TRUST_FAIL_CLOSED_OK`
- `NA0537_REMOTE_ROOT_CLEANUP_RUN2_OK`
- `NA0537_LOCAL_SENSITIVE_RUNTIME_CLEANUP_RUN2_OK`
- `NA0537_NO_STALE_STATE_REUSE_OK`
- `NA0537_NO_SECRET_OUTPUT_OK`
- `NA0537_RETAINED_QSC_UNCHANGED_OK`
- `NA0537_NO_QSL_SERVER_ATTACHMENTS_OK`
- `NA0537_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0537_NO_PRODUCTION_READINESS_CLAIM_OK`
- `NA0537_ONE_READY_INVARIANT_OK`

## Stop conditions

Stop if qwork proof is missing/stale/not OK; qwork/qstart/qresume is run by Codex; queue is not sole READY NA-0537; D-1063 is absent; D-1064 already exists at startup; disk usage is at or above 95%; D443/D444/D445 residue cannot be verified clean; response-writer safety policy is absent; retained qsc path/owner/mode/size/hash mismatches; qsc runtime/dependency drift requires retained-qsc restage; peer-label proof cannot be established; generated scripts or local listener self-test fail after bounded recovery; marker/ACK fails for either run; qsl-server or qsl-attachments is required or used; remote boundary check fails; package install or remote source checkout/build is required; qsc source/test/fuzz/Cargo or workflow/dependency mutation is required; corrected run 1 or run 2 baseline receive fails; wrong-peer or stale-trust negative fails open; stale state reuse is detected; cleanup fails; checked-in evidence contains private material; root or nested audit is red; any required qsc test, validator, formal check, fmt, or shell syntax check fails; public-safety is red/missing before required merge decisions; or any forbidden public/production/security-completion claim is introduced.

## Claim boundary

This testplan supports bounded internal direct-qsc repeated-run cleanup/freshness evidence only. No public-readiness claim is made. No production-readiness claim is made. No public-internet-readiness claim is made. No external-review-complete claim is made. No crypto-complete claim is made. No identity-complete claim is made. No trust-complete claim is made. No replay-proof claim is made. No downgrade-proof claim is made. No secret-material-complete claim is made. No side-channel-free claim is made. No vulnerability-free claim is made. No bug-free claim is made. No perfect-crypto claim is made.
