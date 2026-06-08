Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-08

# NA-0446 QSL qsc Key Lifecycle Secret Cleanup / Zeroization Test Implementation Harness

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0446 implements the exact qsc test path authorized by NA-0445:

`qsl/qsl-client/qsc/tests/key_lifecycle_zeroization.rs`

The implementation adds bounded internal evidence for observable key-lifecycle
cleanup, no-mutation, encrypted-at-rest, and redaction boundaries through
existing qsc test/public APIs. It does not change runtime code, crypto code,
dependencies, Cargo manifests, lockfiles, workflows, fuzz targets, vectors,
formal models, services, public docs, website, backup/local-ops state, or
qwork tooling.

Primary implementation result:

`NA0446_KEY_LIFECYCLE_TEST_IMPLEMENTATION_OK`

Selected successor:

`NA-0447 -- QSL RNG Failure Behavior Scope Authorization Plan`

## Live NA-0446 scope

Live `NEXT_ACTIONS.md` lists:

`NA-0446 -- QSL qsc Key Lifecycle Secret Cleanup / Zeroization Test Implementation Harness`

Status: READY.

Allowed implementation mutation path:

- `qsl/qsl-client/qsc/tests/key_lifecycle_zeroization.rs`

Allowed governance mutation paths:

- `docs/governance/evidence/NA-0446_qsl_qsc_key_lifecycle_secret_cleanup_zeroization_test_implementation_harness.md`
- `tests/NA-0446_qsl_qsc_key_lifecycle_secret_cleanup_zeroization_test_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden mutation scope includes runtime code, crypto code, dependency
metadata, Cargo manifests, lockfiles, workflows, executable tests outside the
exact qsc test path, fuzz targets, vectors, formal models, qsl-server,
qsl-attachments, qshield runtime, qshield-cli, website, public docs, README,
START_HERE, qwork/qstart/qresume/qshell, qsl-backup, backup status files,
backup plan files, rollback subtree paths, `/backup/qsl`, public technical
paper content, branch protection, and public claim surfaces.

Acceptance criteria:

- qwork proof files are verified without running qwork, qstart, or qresume;
- NA-0445 exact test-path authorization is consumed;
- the exact qsc test file is implemented;
- the new qsc test passes;
- inherited provider-error, qsc send_commit, refimpl provider, audit, format,
  formal, and qsc adversarial evidence remains healthy or explicitly bounded;
- no direct memory overwrite, allocator, `Drop`, or side-channel claim is made;
- no all-material coverage claim is made;
- no secret-material-complete claim is made;
- qshield-cli remains demo-local boundary evidence only;
- refimpl remains deferred;
- selected successor is NA-0447 RNG failure behavior scope authorization;
- exactly one READY item remains mandatory.

## qwork proof-file verification

Codex read, but did not run, qwork proof files:

- `/srv/qbuild/work/NA-0446/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0446/.qwork/startup.qsl-protocol.json`

Required `.kv` markers passed:

- `startup_result=OK`
- `lane=NA-0446`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0446/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0446`
- `requested_lane_status=READY`

The JSON proof parsed successfully and mirrored the `.kv` proof for lane, repo,
path, HEAD, origin/main, clean-state fields, READY count, top READY item, and
requested lane status.

Initial live `HEAD` and `origin/main` matched the qwork proof at
`50e4387e8379`. After `git fetch --all --prune`, `origin/main` still matched
the qwork proof. PR #1160 was verified MERGED with merge commit
`50e4387e8379`.

Recorded timestamps:

- Local: `2026-06-08T16:20:00-05:00`
- UTC: `2026-06-08T21:20:00+00:00`

Proof root:

`/srv/qbuild/tmp/NA0446_qsc_key_lifecycle_zeroization_test_impl_20260608T212113Z`

## NA-0445 authorization inheritance

NA-0445 selected:

`QSC_ZEROIZATION_TEST_SCOPE_IMPLEMENTATION_READY`

NA-0445 authorized the exact future qsc test path:

`qsl/qsl-client/qsc/tests/key_lifecycle_zeroization.rs`

Inherited implementable surfaces:

- pending handshake material cleanup on success and no-mutation on selected
  reject paths;
- session/shared-secret store insertion only after successful session storage;
- session and vault encrypted-at-rest boundaries;
- vault/passphrase redaction boundaries;
- key-lifecycle output redaction sentinel scanning;
- reject paths preserving pending/session/vault state where the existing API
  does not clear state by design.

Inherited forbidden/residual scope:

- direct runtime memory overwrite proof remains out of scope;
- allocator overwrite behavior remains out of scope;
- `Drop` proof remains out of scope;
- side-channel proof remains out of scope;
- all key-material coverage remains out of scope;
- refimpl cleanup/zeroization remains deferred;
- qshield-cli remains demo-local boundary evidence only;
- RNG failure behavior remains a later candidate and is selected as NA-0447.

## Pre-mutation test path review

The authorized test file was absent before mutation:

`qsl/qsl-client/qsc/tests/key_lifecycle_zeroization.rs`

Rollback marker created:

`$PROOF_DIR/rollback/key_lifecycle_zeroization.rs.absent`

Existing qsc test inventory was recorded under:

`$PROOF_DIR/preimage/qsc_tests_inventory.txt`

Source/test patterns reviewed included:

- `qsl/qsl-client/qsc/tests/handshake_provider_error_no_mutation.rs`
- `qsl/qsl-client/qsc/tests/session_state_at_rest.rs`
- `qsl/qsl-client/qsc/tests/vault.rs`
- `qsl/qsl-client/qsc/tests/send_commit.rs`
- qsc handshake pending/session/vault helper patterns in `qsl/qsl-client/qsc/src/`

## Test implementation summary

Implemented tests:

- `pending_handshake_secret_cleanup_success_and_reject_boundaries`
- `session_secret_store_inserted_only_after_success_and_encrypted_at_rest`
- `key_lifecycle_output_redaction_sentinel_scan`
- `reject_paths_preserve_pending_session_vault_state`
- `session_and_vault_encrypted_at_rest_boundaries`
- `vault_passphrase_redaction_and_no_plaintext_boundary`

Implemented markers:

- `NA0446_KEY_LIFECYCLE_TEST_IMPLEMENTATION_OK`
- `NA0446_PENDING_SECRET_CLEANUP_SUCCESS_BOUNDARY_OK`
- `NA0446_REJECT_NO_MUTATION_BOUNDARY_OK`
- `NA0446_SESSION_SECRET_STORE_BOUNDARY_OK`
- `NA0446_ENCRYPTED_AT_REST_BOUNDARY_OK`
- `NA0446_REDACTION_SENTINEL_BOUNDARY_OK`
- `NA0446_NO_RUNTIME_HOOK_USED_OK`
- `NA0446_DIRECT_MEMORY_ZEROIZATION_NOT_CLAIMED_OK`
- `NA0446_NO_SECRET_MATERIAL_COMPLETE_CLAIM_OK`
- `NA0446_QSHIELD_CLI_DEMO_BOUNDARY_PRESERVED_OK`
- `NA0446_REFIMPL_SCOPE_DEFERRED_OK`

The implementation uses real qsc command paths and existing test helpers. It
does not add runtime hooks, provider fakes, new dependencies, or non-qsc test
mutations.

## Pending handshake material test proof

`pending_handshake_secret_cleanup_success_and_reject_boundaries` establishes a
real two-party qsc handshake using the existing mock-vault and relay test
helpers.

Assertions:

- Alice pending handshake material exists after `handshake init`.
- Bob pending responder material exists after Bob processes A1.
- Alice pending material is cleared after Alice processes B1 and stores a
  session.
- Bob pending material is cleared after Bob processes A2 and stores a session.
- A malformed inbound frame against an existing Alice pending handshake emits a
  reject marker, does not complete the handshake, does not create session
  state, and leaves the pending vault bytes unchanged.

Marker:

`NA0446_PENDING_SECRET_CLEANUP_SUCCESS_BOUNDARY_OK`

## Session / shared secret store test proof

`session_secret_store_inserted_only_after_success_and_encrypted_at_rest`
checks the `qsp_session_store_key_v1` vault secret boundary and session blob
creation order.

Assertions:

- session-store key is absent before handshake success;
- session-store key remains absent after Alice init;
- session-store key remains absent for Bob after Bob creates only responder
  pending state;
- session-store key appears only after qsc stores a successful session;
- encrypted `.qsv` session blobs exist after success and use the qsc session
  envelope magic;
- session blobs do not contain structured plaintext state markers such as
  `session_id`, chain-key labels, or send/receive state labels.

Marker:

`NA0446_SESSION_SECRET_STORE_BOUNDARY_OK`

## Vault / passphrase / runtime key boundary proof

`vault_passphrase_redaction_and_no_plaintext_boundary` initializes a
passphrase-backed vault with a controlled test sentinel through stdin.

Assertions:

- vault init succeeds through the explicit passphrase path;
- command output does not echo the controlled passphrase sentinel;
- `vault.qsv` exists and uses the encrypted vault envelope magic;
- `vault.qsv` does not contain the controlled passphrase sentinel bytes.

This is a vault/passphrase redaction and encrypted-at-rest boundary only. No
direct runtime key memory overwrite, passphrase memory overwrite, allocator
behavior, or `Drop` proof is claimed.

## Redaction / logging test proof

`key_lifecycle_output_redaction_sentinel_scan` uses a controlled output
sentinel in a rejected vault-init command.

Assertions:

- the command fails closed with `key_source_invalid`;
- stdout/stderr do not contain the controlled sentinel;
- stdout/stderr do not contain controlled route-token sentinels, the mock-vault
  passphrase, panic text, or stack-backtrace text;
- no vault file is created on that rejected path.

Marker:

`NA0446_REDACTION_SENTINEL_BOUNDARY_OK`

## Encrypted-at-rest boundary proof

`session_and_vault_encrypted_at_rest_boundaries` completes a real qsc
handshake, then inspects on-disk vault and session envelopes.

Assertions:

- Alice and Bob vault files use the encrypted vault envelope magic;
- Alice and Bob session files use the encrypted session envelope magic;
- vault bytes do not contain controlled route-token sentinels, passphrase
  sentinels, pending secret key names, or the session-store key name;
- session bytes do not contain structured plaintext session-state markers;
- Alice and Bob encrypted envelope bytes differ.

Marker:

`NA0446_ENCRYPTED_AT_REST_BOUNDARY_OK`

## Direct memory zeroization caveat

No direct memory zeroization claim is made.

NA-0446 does not observe allocator contents, stack/register contents, runtime
key memory, passphrase memory after use, or `Drop` behavior. Those remain
runtime-hook or implementation-scope candidates only if a later directive
authorizes exact hooks or runtime/test APIs.

Marker:

`NA0446_DIRECT_MEMORY_ZEROIZATION_NOT_CLAIMED_OK`

## qshield-cli demo-local boundary proof

qshield-cli remains demo-local boundary evidence only.

NA-0446 does not read, execute, or mutate qshield-cli implementation paths. No
qshield-cli behavior is represented as qsc runtime cleanup assurance or
service-readiness evidence.

Marker:

`NA0446_QSHIELD_CLI_DEMO_BOUNDARY_PRESERVED_OK`

## refimpl deferred-scope proof

refimpl remains deferred for key-lifecycle cleanup/zeroization scope.

NA-0446 runs the inherited `pqkem768` provider test as dependency/provider
health evidence, but does not mutate refimpl and does not represent refimpl as
qsc runtime cleanup proof.

Marker:

`NA0446_REFIMPL_SCOPE_DEFERRED_OK`

## No runtime / crypto / dependency / workflow mutation proof

Changed paths are limited to the exact authorized test path and NA-0446
governance paths:

- `qsl/qsl-client/qsc/tests/key_lifecycle_zeroization.rs`
- `docs/governance/evidence/NA-0446_qsl_qsc_key_lifecycle_secret_cleanup_zeroization_test_implementation_harness.md`
- `tests/NA-0446_qsl_qsc_key_lifecycle_secret_cleanup_zeroization_test_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

No runtime, crypto, dependency, Cargo, lockfile, workflow, fuzz target, vector,
formal model, qsl-server, qsl-attachments, qshield runtime, qshield-cli,
website, public-doc, README, START_HERE, qwork/qstart/qresume/qshell, backup,
qsl-backup, status/plan, rollback, or backup-tree mutation is included.

## Validation

Local validation results before PR:

- `cargo test -p qsc --locked --test key_lifecycle_zeroization -- --test-threads=1 --nocapture`: PASS, 6 tests, required markers emitted.
- `cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture`: PASS, inherited NA-0436 markers emitted.
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`: PASS, 3 tests.
- `cargo test -p quantumshield_refimpl --features pqcrypto --locked --test pqkem768`: PASS, 3 tests.
- `sh -n scripts/ci/qsc_adversarial.sh`: PASS.
- `bash -n scripts/ci/qsc_adversarial.sh`: PASS.
- `cargo audit --deny warnings`: PASS.
- `cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock`: PASS.
- `cargo tree -i rustls-webpki --locked`: PASS, `rustls-webpki v0.103.13`.
- `cargo tree -i ml-kem --locked`: PASS, `ml-kem v0.2.1`.
- root pqcrypto inverse probes reported expected package-ID absence for `pqcrypto-mlkem`, `pqcrypto-traits`, and `pqcrypto-internals`.
- nested qsc fuzz lock pqcrypto residual scan returned zero matches.
- `cargo fmt --check`: PASS.
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`: PASS.
- `python3 formal/run_model_checks.py`: PASS.
- local `scripts/ci/qsc_adversarial.sh`: stable Rust phases and provider-error step passed; local execution then stopped at missing local `cargo fuzz`. PR CI `qsc-adversarial-smoke` remains required for cargo-fuzz-backed evidence.

Public-safety on current main before implementation:

`main sha=50e4387e8379 check=public-safety status=completed conclusion=success`

Root backup boundary proof:

- `/usr/local/sbin/qsl-backup` SHA matched `e9ecff3d22ed`.
- backup source-list inclusion count for the Codex ops source path was exactly
  1.
- No backup or restore was run.

## Recovered failures / warnings

- Failing command: `python3 scripts/ci/public_safety_gate.py check-main-public-safety --owner-repo QuantumShieldLabs/qsl-protocol --branch main --check-name public-safety`.
  Classification: recoverable command-shape issue because the helper expects
  `--repo`, not `--owner-repo`.
  Corrective action: reran with `--repo`.
  Final result: the command advanced to authentication setup.
- Failing command: `python3 scripts/ci/public_safety_gate.py check-main-public-safety --repo QuantumShieldLabs/qsl-protocol --branch main --check-name public-safety`.
  Classification: recoverable local tool-environment issue because `gh` was
  authenticated but the helper requires `GH_TOKEN`.
  Corrective action: reran with `GH_TOKEN` populated from `gh auth token`
  without printing the token.
  Final result: public-safety verified green on `50e4387e8379`.
- Failing command: `rustfmt qsl/qsl-client/qsc/tests/key_lifecycle_zeroization.rs`.
  Classification: recoverable command-shape issue because standalone rustfmt
  parsed the test module tree without the crate edition.
  Corrective action: reran `rustfmt --edition 2021` on the exact authorized
  test file.
  Final result: formatting succeeded and `cargo fmt --check` passed.
- Failing command: local qsc adversarial script under `pipefail`.
  Classification: recoverable local tooling limitation because both stable
  Rust phases and the provider-error step passed before local cargo-fuzz
  availability stopped the script.
  Corrective action: no toolchain, dependency, workflow, or source mutation;
  require PR CI `qsc-adversarial-smoke`.
  Final result: local output ended with `error: no such command: fuzz`.
- Non-fatal warnings: parallel read-only cargo audit/tree probes printed
  package-cache/advisory-db lock wait messages. Classification: benign
  contention warnings. Final result: audits and dependency probes completed.
- Valid zero-match proof: root pqcrypto inverse probes reported package-ID
  absence for `pqcrypto-mlkem`, `pqcrypto-traits`, and
  `pqcrypto-internals`. Classification: expected absence proof.
- Valid zero-match proof: nested qsc fuzz lock pqcrypto residual scan returned
  zero matches. Classification: expected absence proof.

## Public claim/external review/website boundary

NA-0446 tests are internal bounded evidence only.

No production-readiness claim is made.
No public-readiness claim is made.
No public-internet-readiness claim is made.
No external-review-complete claim is made.
No crypto-complete claim is made.
No side-channel-free claim is made.
No secret-material-complete claim is made.
No bug-free claim is made.
No vulnerability-free claim is made.
No perfect-crypto claim is made.
No public technical paper content is created.
No README, START_HERE, public docs, or website path is changed.

Cargo audit green is dependency-health evidence only.

## Rejected alternatives

- Adding runtime hooks for direct memory overwrite observation was rejected as
  out of scope.
- Adding provider fakes or modifying provider/runtime code was rejected as out
  of scope.
- Mutating existing qsc tests instead of adding the exact NA-0445-authorized
  test file was rejected as out of scope.
- Treating qshield-cli demo-local behavior or refimpl behavior as qsc runtime
  cleanup proof was rejected as overbroad.
- No secret-material-complete claim was added; treating it as supported was rejected.
- No side-channel-free claim was added; treating it as supported was rejected.
- No crypto-complete claim was added; treating it as supported was rejected.
- No bug-free claim was added; treating it as supported was rejected.
- No vulnerability-free claim was added; treating it as supported was rejected.
- No perfect-crypto claim was added; treating it as supported was rejected.
- No public-readiness claim was added; treating it as supported was rejected.
- No production-readiness claim was added; treating it as supported was rejected.

## Backup-impact statement

No backup was run.
No restore was run.
No sudo was run.
No qsl-backup mutation occurred.
No backup status or backup plan file mutation occurred.
No rollback subtree or `/backup/qsl` mutation occurred.

The qsl-backup SHA and source-list count are read-only boundary evidence only.

## Successor selection

Successful implementation leaves F-0441-03 RNG failure behavior as the obvious
next domain.

Selected successor:

`NA-0447 -- QSL RNG Failure Behavior Scope Authorization Plan`

NA-0446 does not implement NA-0447.

## Next recommendation

After this implementation PR merges and post-merge public-safety is green,
perform the optional closeout to mark NA-0446 DONE and restore NA-0447 as the
sole READY item with no runtime, crypto, dependency, workflow, public-claim, or
backup expansion.
