Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-11

# NA-0458 qsc KEM Provider RNG Failure Fake / Test Seam Implementation Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate the NA-0458 qsc KEM provider RNG failure cfg seam implementation and
prove bounded no-mutation behavior for selected forced KEM keypair and
responder KEM encapsulation failures.

## Protected invariants

- The seam is compiled only under `--cfg qsc_rng_failure_test_seam`.
- Normal builds ignore `QSC_RNG_FAILURE_TEST_SEAM`.
- Forced KEM keypair failure stops before identity, vault-secret, pending,
  session, and A1 output writes.
- Forced responder KEM encap failure stops before responder pending/session
  state and B1 output writes.
- Existing `pq_decap_failed` generic provider-error no-mutation coverage is
  preserved.
- `pq_encap_failed` remains a bounded forced-seam caveat, not an external
  triggerability claim.
- No refimpl, dependency, Cargo, lockfile, workflow, fuzz target, vector,
  formal model, service, public docs, README, START_HERE, backup, restore, or
  qsl-backup mutation occurs.

## Allowed scope

- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/src/identity/mod.rs`
- `qsl/qsl-client/qsc/tests/kem_provider_rng_failure.rs`
- `docs/governance/evidence/NA-0458_qsl_qsc_kem_provider_rng_failure_fake_test_seam_implementation_harness.md`
- `tests/NA-0458_qsl_qsc_kem_provider_rng_failure_fake_test_seam_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

No dependency, Cargo, lockfile, workflow, qsl-server, qsl-attachments, qshield
runtime, qshield-cli, website, public docs, README, START_HERE, fuzz target,
vector, formal model, refimpl, qwork/qstart/qresume/qshell, backup, restore,
qsl-backup, backup status, backup plan, rollback, or backup tree mutation is
allowed.

## cfg seam tests

Run:

```bash
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test kem_provider_rng_failure -- --test-threads=1 --nocapture
```

Required:

- `kem_keypair_rng_failure_writes_no_identity_or_session_state` passes.
- `kem_encap_rng_failure_writes_no_responder_state_or_b1` passes.
- `NA0458_KEM_PROVIDER_RNG_SEAM_IMPLEMENTED_OK` appears.
- `NA0458_KEM_KEYPAIR_RNG_FAILURE_FORCED_OK` appears.
- `NA0458_KEM_KEYPAIR_RNG_FAILURE_NO_PARTIAL_STATE_OK` appears.
- `NA0458_KEM_ENCAP_RNG_FAILURE_FORCED_OK` appears.
- `NA0458_KEM_ENCAP_RNG_FAILURE_NO_RESPONDER_STATE_OK` appears.
- `NA0458_KEM_ENCAP_RNG_FAILURE_NO_B1_OUTPUT_OK` appears.

## normal-build tests

Run:

```bash
cargo test -p qsc --locked --test kem_provider_rng_failure -- --test-threads=1 --nocapture
```

Required:

- `kem_provider_rng_failure_seam_inactive_without_cfg` passes.
- `NA0458_PRODUCTION_SEMANTICS_UNCHANGED_OK` appears.
- no forced-failure marker appears in normal output.
- normal A1 and B1 handshake output is produced despite the selector
  environment variable being set.

## no-mutation assertions

Keypair forced-failure assertions:

- no public identity file;
- no KEM identity secret;
- no signature identity secret;
- no vault byte mutation;
- no pending state;
- no session blob;
- no A1 output.

Encap forced-failure assertions:

- no responder vault byte mutation;
- no responder pending state;
- no responder session blob;
- no B1 output.

## B1/output absence check

The encap forced-failure test must drain the Alice relay route after Bob polls
and prove no B1 item was emitted. Output must include `pq_encap_failed` and must
not include `msg=B1`.

## keypair partial-state absence check

The keypair forced-failure test must snapshot Alice vault bytes before the
forced failure, then prove the public identity path, KEM/signature identity
secret entries, pending state, session state, and A1 relay output remain absent.

## inherited qsc RNG tests

Run:

```bash
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test rng_failure_residual_surfaces -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test rng_failure_residual_surfaces -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test rng_failure_behavior -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test rng_failure_behavior -- --test-threads=1 --nocapture
```

Required: all pass.

## inherited provider-error/key-lifecycle tests

Run:

```bash
cargo test -p qsc --locked --test key_lifecycle_zeroization -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test pqkem768
```

Required: all pass.

## root audit

Run:

```bash
cargo audit --deny warnings
```

Required: PASS. This is dependency-health evidence only.

## nested fuzz lock audit

Run:

```bash
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
rg -n "pqcrypto-mlkem|pqcrypto-traits|pqcrypto-internals" qsl/qsl-client/qsc/fuzz/Cargo.lock || true
```

Required: audit PASS and pqcrypto scan zero-match.

## qsc adversarial check

Run:

```bash
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
```

If feasible locally, run the script itself. If local cargo-fuzz is unavailable,
record exact output and rely on PR CI `qsc-adversarial-smoke`.

## formal checks

Run:

```bash
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

Required: PASS.

## scope guard

Changed paths before optional closeout must be exactly:

- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/src/identity/mod.rs`
- `qsl/qsl-client/qsc/tests/kem_provider_rng_failure.rs`
- `docs/governance/evidence/NA-0458_qsl_qsc_kem_provider_rng_failure_fake_test_seam_implementation_harness.md`
- `tests/NA-0458_qsl_qsc_kem_provider_rng_failure_fake_test_seam_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## public claim boundary

NA-0458 is bounded internal qsc evidence only.

No public-readiness claim is allowed. No production-readiness claim is allowed.
No external-review-complete claim is allowed. No crypto-complete claim is
allowed. No KEM-complete claim is allowed. No RNG-failure-complete claim is
allowed. No provider-RNG-complete claim is allowed. No side-channel-free claim
is allowed. No vulnerability-free claim is allowed. No bug-free claim is
allowed. No perfect-crypto claim is allowed.

## closeout prerequisites

Closeout to NA-0459 may run only after the NA-0458 implementation PR merges and
post-merge public-safety is green. Closeout must restore exactly one READY item
and must not implement NA-0459.
