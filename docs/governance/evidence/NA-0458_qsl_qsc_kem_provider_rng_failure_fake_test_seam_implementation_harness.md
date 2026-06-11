Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-11

# NA-0458 QSL qsc KEM Provider RNG Failure Fake / Test Seam Implementation Harness

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0458 consumes the NA-0457 KEM-only strategy and implements a qsc-local,
cfg-gated KEM provider RNG failure test seam. The seam is compiled only when
Rust is invoked with `--cfg qsc_rng_failure_test_seam`.

Implemented labels:

- `QSC.KEM.KEYPAIR`
- `QSC.KEM.ENCAP`

The implementation adds bounded internal evidence that selected forced qsc KEM
provider failures stop before selected identity, vault, pending, session, and
responder B1 output mutation. Normal no-cfg builds ignore the selector and keep
production semantics unchanged.

This lane does not implement signature/identity provider RNG, X25519 provider
RNG, refimpl provider RNG, qshield-cli RNG, formal RNG, fuzz RNG, vector RNG, or
public surface work. `pq_decap_failed` remains background generic provider-error
no-mutation evidence. `pq_encap_failed` remains a defensive branch caveat under
normal provider behavior; NA-0458 only proves bounded forced-seam behavior.

No public-readiness claim is made. No production-readiness claim is made. No
external-review-complete claim is made. No crypto-complete claim is made. No
KEM-complete claim is made. No RNG-failure-complete claim is made. No
provider-RNG-complete claim is made. Cargo audit green remains dependency-health
evidence only.

Required markers:

- `NA0458_KEM_PROVIDER_RNG_SEAM_IMPLEMENTED_OK`
- `NA0458_KEM_KEYPAIR_RNG_FAILURE_FORCED_OK`
- `NA0458_KEM_KEYPAIR_RNG_FAILURE_NO_PARTIAL_STATE_OK`
- `NA0458_KEM_ENCAP_RNG_FAILURE_FORCED_OK`
- `NA0458_KEM_ENCAP_RNG_FAILURE_NO_RESPONDER_STATE_OK`
- `NA0458_KEM_ENCAP_RNG_FAILURE_NO_B1_OUTPUT_OK`
- `NA0458_PRODUCTION_SEMANTICS_UNCHANGED_OK`
- `NA0458_PQ_DECAP_FAILED_BACKGROUND_PRESERVED_OK`
- `NA0458_PQ_ENCAP_FAILED_CAVEAT_PRESERVED_OK`
- `NA0458_NO_REFIMPL_CHANGE_OK`
- `NA0458_NO_DEPENDENCY_CHANGE_OK`
- `NA0458_NO_WORKFLOW_CHANGE_OK`
- `NA0458_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0458_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0458_NO_RNG_FAILURE_COMPLETE_CLAIM_OK`
- `NA0458_NO_PROVIDER_RNG_COMPLETE_CLAIM_OK`
- `NA0458_ONE_READY_INVARIANT_OK`

## Live NA-0458 scope

Startup proof established:

- READY_COUNT 1.
- READY item: NA-0458.
- NA-0457 through NA-0435 are DONE.
- NA-0434 and NA-0429 are BLOCKED.
- Latest decision before patch: D-0902.
- D-0901 exists once.
- D-0902 exists once.
- D-0903 was absent before the NA-0458 patch.
- Duplicate decision count was zero.

Allowed implementation paths:

- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/src/identity/mod.rs`
- `qsl/qsl-client/qsc/tests/kem_provider_rng_failure.rs`

Allowed governance paths:

- `docs/governance/evidence/NA-0458_qsl_qsc_kem_provider_rng_failure_fake_test_seam_implementation_harness.md`
- `tests/NA-0458_qsl_qsc_kem_provider_rng_failure_fake_test_seam_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden mutation scope included dependencies, Cargo files, lockfiles,
workflows, qsl-server, qsl-attachments, qshield runtime, qshield-cli, website,
public docs, README, START_HERE, fuzz targets, vectors, formal model files,
refimpl, qwork/qstart/qresume/qshell, backup/restore/local-ops files, and
backup tree paths.

## qwork proof-file verification

Codex read, but did not run or regenerate, these proof files:

- `/srv/qbuild/work/NA-0458/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0458/.qwork/startup.qsl-protocol.json`

Verified values:

- `startup_result=OK`
- `lane=NA-0458`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0458/qsl-protocol`
- `head_equals_origin_main=yes`
- clean worktree, index, and untracked state
- `ready_count=1`
- `queue_top_ready=NA-0458`
- `requested_lane_status=READY`

The JSON proof mirrored the `.kv` proof for lane, repo, path, HEAD,
`origin/main`, READY count, queue top READY, requested lane status, and
clean-state fields.

Proof HEAD and proof `origin/main` matched live local refs before fetch at
`4cacba333820`. Fetch did not advance `origin/main`.

PR #1184 was verified merged at `4cacba333820`.

Proof root:

`/srv/qbuild/tmp/NA0458_qsc_kem_provider_rng_seam_impl_20260611T004907Z`

## NA-0457 inheritance

NA-0457 selected:

`QSC_PROVIDER_RNG_KEM_FAKE_SEAM_IMPLEMENTATION_READY`

Inherited facts:

- qsc KEM keypair/provider generation is selected for cfg-only forced-failure
  evidence.
- qsc responder KEM encapsulation is selected for cfg-only forced-failure
  evidence.
- qsc KEM decap / `pq_decap_failed` already has generic provider-error
  no-mutation evidence and remains background, not RNG-specific completion.
- qsc KEM encap / `pq_encap_failed` remains defensive under normal provider
  behavior and is not externally forceable through the active public API.
- signature/identity provider RNG, X25519 provider RNG, and refimpl provider
  RNG remain residual.
- no trait/API, dependency, Cargo, lockfile, workflow, fuzz, vector, formal, or
  service mutation was authorized.

## KEM-only implementation summary

Implementation paths changed:

- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/src/identity/mod.rs`
- `qsl/qsl-client/qsc/tests/kem_provider_rng_failure.rs`

`handshake/mod.rs` adds a cfg-only helper for qsc KEM keypair forced failure and
a cfg-only responder encapsulation guard before `StdCrypto::encap`.

`identity/mod.rs` consumes the keypair helper only in the lazy
`identity_self_kem_keypair` path. The existing `hs_kem_keypair()` signature is
unchanged, so out-of-scope callers and normal builds are not changed.

`kem_provider_rng_failure.rs` adds cfg and no-cfg integration coverage using
the existing qsc mock vault and mock relay test style.

## cfg seam labels and normal-build boundary

The seam is active only under:

`RUSTFLAGS='--cfg qsc_rng_failure_test_seam'`

Labels:

- `QSC.KEM.KEYPAIR`: forces the lazy identity KEM keypair path to return a
  deterministic sanitized qsc error before identity, vault-secret, pending,
  session, or A1 output writes.
- `QSC.KEM.ENCAP`: forces responder KEM encapsulation to take the existing
  sanitized `pq_encap_failed` reject path before responder pending/session state
  or B1 output writes.

Normal builds compile the no-cfg path. The no-cfg integration test sets
`QSC_RNG_FAILURE_TEST_SEAM` while exercising both keypair and encap call sites;
the handshake succeeds and emits `NA0458_PRODUCTION_SEMANTICS_UNCHANGED_OK`.

## KEM keypair forced-failure proof

Test:

`kem_keypair_rng_failure_writes_no_identity_or_session_state`

Command:

```bash
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test kem_provider_rng_failure -- --test-threads=1 --nocapture
```

Evidence:

- forced label: `QSC.KEM.KEYPAIR`;
- qsc returned deterministic sanitized error output containing
  `identity_secret_unavailable` and the test-only forced marker;
- no Alice public identity file was written;
- no Alice KEM identity secret was written;
- no Alice signature identity secret was written;
- Alice vault bytes were unchanged;
- no initiator pending state was written;
- no Alice session blob was written;
- no A1 relay output was emitted.

Markers:

- `NA0458_KEM_KEYPAIR_RNG_FAILURE_FORCED_OK`
- `NA0458_KEM_KEYPAIR_RNG_FAILURE_NO_PARTIAL_STATE_OK`

## KEM encap forced-failure proof

Test:

`kem_encap_rng_failure_writes_no_responder_state_or_b1`

Command:

```bash
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test kem_provider_rng_failure -- --test-threads=1 --nocapture
```

Evidence:

- forced label: `QSC.KEM.ENCAP`;
- qsc emitted a sanitized `handshake_reject` with reason `pq_encap_failed`;
- Bob responder vault bytes were unchanged;
- no Bob responder pending state was written;
- no Bob session blob was written;
- no B1 relay output was emitted.

Markers:

- `NA0458_KEM_ENCAP_RNG_FAILURE_FORCED_OK`
- `NA0458_KEM_ENCAP_RNG_FAILURE_NO_RESPONDER_STATE_OK`
- `NA0458_KEM_ENCAP_RNG_FAILURE_NO_B1_OUTPUT_OK`

## production semantics unchanged proof

Test:

`kem_provider_rng_failure_seam_inactive_without_cfg`

Command:

```bash
cargo test -p qsc --locked --test kem_provider_rng_failure -- --test-threads=1 --nocapture
```

The normal no-cfg test sets `QSC_RNG_FAILURE_TEST_SEAM` for both
`QSC.KEM.KEYPAIR` and `QSC.KEM.ENCAP`. It proves the environment selector is
ignored without the custom cfg: Alice creates identity and A1, Bob creates B1,
and no forced-failure marker or `pq_encap_failed` reject appears.

Marker:

- `NA0458_PRODUCTION_SEMANTICS_UNCHANGED_OK`

## pq_decap_failed background preservation

Existing test:

```bash
cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture
```

Result: PASS.

Existing markers preserved:

- `NA0436_PQ_DECAP_FAILED_MARKER_OK`
- `NA0436_NO_SESSION_MUTATION_ON_DECAP_REJECT_OK`
- `NA0436_PENDING_STORE_NO_MUTATION_ON_DECAP_REJECT_OK`
- `NA0436_PQ_ENCAP_FAILED_CAVEAT_PRESERVED_OK`
- `NA0436_NO_RUNTIME_HOOK_USED_OK`

NA-0458 records `NA0458_PQ_DECAP_FAILED_BACKGROUND_PRESERVED_OK` as background
preservation only. It is not RNG-specific completion.

## pq_encap_failed caveat preservation

NA-0458 adds a forced test-seam path for `QSC.KEM.ENCAP`, but it does not claim
that `pq_encap_failed` is externally triggerable under the active provider and
public API.

Marker:

- `NA0458_PQ_ENCAP_FAILED_CAVEAT_PRESERVED_OK`

## no refimpl/dependency/workflow mutation proof

Changed source/test paths are qsc-local. No refimpl file was mutated.

No dependency file was mutated:

- no `Cargo.toml` mutation;
- no `Cargo.lock` mutation;
- no qsc fuzz `Cargo.toml` mutation;
- no qsc fuzz `Cargo.lock` mutation.

No workflow file was mutated.

Markers:

- `NA0458_NO_REFIMPL_CHANGE_OK`
- `NA0458_NO_DEPENDENCY_CHANGE_OK`
- `NA0458_NO_WORKFLOW_CHANGE_OK`

## public claim boundary

NA-0458 is bounded internal qsc evidence only.

No public-readiness claim is made. No production-readiness claim is made. No
public-internet-readiness claim is made. No external-review-complete claim is
made. No crypto-complete claim is made. No KEM-complete claim is made. No
RNG-failure-complete claim is made. No provider-RNG-complete claim is made. No
side-channel-free claim is made. No vulnerability-free claim is made. No
bug-free claim is made. No perfect-crypto claim is made.

## validation

Local validation after implementation:

- `cargo fmt --check`: PASS after one rustfmt recovery.
- cfg `kem_provider_rng_failure`: PASS, 3 tests.
- no-cfg `kem_provider_rng_failure`: PASS, 1 test.
- cfg `rng_failure_residual_surfaces`: PASS.
- no-cfg `rng_failure_residual_surfaces`: PASS.
- cfg `rng_failure_behavior`: PASS.
- no-cfg `rng_failure_behavior`: PASS.
- `key_lifecycle_zeroization`: PASS.
- `handshake_provider_error_no_mutation`: PASS.
- stable `send_commit`: PASS.
- refimpl `pqkem768`: PASS.
- `sh -n scripts/ci/qsc_adversarial.sh`: PASS.
- `bash -n scripts/ci/qsc_adversarial.sh`: PASS.
- root `cargo audit --deny warnings`: PASS.
- nested qsc fuzz lock audit: PASS.
- `cargo tree -i rustls-webpki --locked`: PASS.
- `cargo tree -i ml-kem --locked`: PASS.
- pqcrypto inverse probes: expected zero-match inventory.
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`: PASS.
- `python3 formal/run_model_checks.py`: PASS.
- local qsc adversarial script: Rust phases and provider-error step PASS;
  local cargo-fuzz command unavailable, so PR CI remains the cargo-fuzz-backed
  adversarial smoke authority.

## scope guard

Expected changed paths before optional closeout:

- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/src/identity/mod.rs`
- `qsl/qsl-client/qsc/tests/kem_provider_rng_failure.rs`
- `docs/governance/evidence/NA-0458_qsl_qsc_kem_provider_rng_failure_fake_test_seam_implementation_harness.md`
- `tests/NA-0458_qsl_qsc_kem_provider_rng_failure_fake_test_seam_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

No qsl-server, qsl-attachments, qshield-cli, refimpl, dependency, Cargo,
lockfile, workflow, fuzz target, vector, formal model, public docs, README,
START_HERE, qwork/qstart/qresume/qshell, backup, restore, qsl-backup, backup
status, backup plan, rollback, or backup tree path is changed.

## backup-impact statement

No backup was run. No restore was run. No qsl-backup file, backup status file,
backup plan file, rollback subtree, or backup tree path was mutated.

The qsl-backup script SHA matched the expected boundary value. The script-local
Codex ops source-list inclusion count was one.

## successor selection

Selected successor:

`NA-0459 -- QSL qsc Signature / Identity Provider RNG Failure Scope Authorization Plan`

Rationale:

- NA-0458 implements KEM-only scope.
- NA-0457 and NA-0458 leave signature/identity provider RNG as the next qsc
  residual.
- refimpl provider RNG remains deferred until qsc signature/identity scope is
  triaged or explicitly superseded.

## next recommendation

After NA-0458 merges and post-merge public-safety is green, close out NA-0458
and restore NA-0459 as the sole READY item. Do not implement NA-0459 in the
NA-0458 implementation PR.
