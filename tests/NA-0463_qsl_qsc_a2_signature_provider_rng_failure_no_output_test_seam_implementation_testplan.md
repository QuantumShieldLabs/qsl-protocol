Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-11

# NA-0463 qsc A2 Signature Provider RNG Failure No-Output Test Seam Implementation Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate the NA-0463 bounded implementation lane for a qsc cfg-only A2
signature provider RNG failure test seam. The lane must force selected A2
signing failure only when `qsc_rng_failure_test_seam` is active, prove
sanitized `sig_sign_failed`, prove no A2 output and no relay A2, preserve the
A2 post-mutation timing caveat, and preserve normal no-cfg production
semantics.

## Protected invariants

- The A2 seam is compiled only under `--cfg qsc_rng_failure_test_seam`.
- Normal builds do not read `QSC_RNG_FAILURE_TEST_SEAM`.
- Forced A2 signing failure returns sanitized `sig_sign_failed`.
- Forced A2 signing failure emits no A2 `handshake_send`.
- Forced A2 signing failure emits no relay A2.
- A2 signing failure is not described as pre-mutation no-mutation.
- Initiator session storage and pending clear timing are acknowledged.
- B1 signing evidence remains background only.
- KEM provider RNG evidence remains background only.
- Identity provider RNG, X25519, and refimpl provider RNG remain residual.
- No dependency, Cargo, lockfile, workflow, refimpl, qsl-server,
  qsl-attachments, qshield runtime, qshield-cli, public docs, README,
  START_HERE, fuzz target, vector, or formal model mutation occurs.
- No public-readiness claim is made.
- No production-readiness claim is made.
- No external-review-complete claim is made.
- No crypto-complete claim is made.
- No signature-complete claim is made.
- No identity-complete claim is made.
- No RNG-failure-complete claim is made.
- No provider-RNG-complete claim is made.
- No side-channel-free claim is made.
- No vulnerability-free claim is made.
- No bug-free claim is made.
- No perfect-crypto claim is made.
- Exactly one READY item remains mandatory.

## Allowed scope

Implementation paths:

- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/tests/a2_signature_provider_rng_failure.rs`

Governance paths:

- `docs/governance/evidence/NA-0463_qsl_qsc_a2_signature_provider_rng_failure_no_output_test_seam_implementation_harness.md`
- `tests/NA-0463_qsl_qsc_a2_signature_provider_rng_failure_no_output_test_seam_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

No mutation is allowed outside the paths above. In particular, no mutation is
allowed to identity source, main CLI source, TUI command source, refimpl,
dependencies, Cargo manifests, lockfiles, workflows, fuzz target source,
vectors, formal models, qsl-server, qsl-attachments, qshield runtime,
qshield-cli, website, public docs, README, START_HERE, qwork/qstart/qresume/
qshell, backup, restore, qsl-backup, backup status, backup plan, rollback, or
backup tree paths.

## cfg seam tests

Run:

```bash
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test a2_signature_provider_rng_failure -- --test-threads=1 --nocapture
```

Required test:

- `a2_signature_rng_failure_emits_no_a2_output`

Required markers:

- `NA0463_A2_SIGNATURE_PROVIDER_RNG_SEAM_IMPLEMENTED_OK`
- `NA0463_A2_SIGNATURE_PROVIDER_RNG_FAILURE_FORCED_OK`
- `NA0463_A2_SIGNATURE_PROVIDER_RNG_FAILURE_SIG_SIGN_FAILED_OK`
- `NA0463_A2_SIGNATURE_PROVIDER_RNG_FAILURE_NO_A2_OUTPUT_OK`
- `NA0463_A2_SIGNATURE_PROVIDER_RNG_FAILURE_NO_RELAY_A2_OK`
- `NA0463_A2_SIGNATURE_PROVIDER_RNG_FAILURE_NO_FALSE_NO_MUTATION_CLAIM_OK`
- `NA0463_A2_SIGNATURE_PROVIDER_RNG_FAILURE_POST_MUTATION_TIMING_ACKNOWLEDGED_OK`

## normal-build tests

Run:

```bash
cargo test -p qsc --locked --test a2_signature_provider_rng_failure -- --test-threads=1 --nocapture
```

Required test:

- `a2_signature_rng_failure_seam_inactive_without_cfg`

Required marker:

- `NA0463_PRODUCTION_SEMANTICS_UNCHANGED_OK`

Required behavior:

- setting `QSC_RNG_FAILURE_TEST_SEAM=QSC.SIG.A2` has no effect without cfg;
- normal A2 `sig_status reason=a2_sign` appears;
- normal A2 `handshake_send msg=A2` appears;
- normal A2 relay emission occurs;
- no `sig_sign_failed` appears.

## no A2 output assertions

The cfg forced-failure test must assert absence of:

- `reason=a2_sign`;
- `event=handshake_send`;
- `msg=A2`;
- `event=handshake_complete`.

## no relay A2 assertions

The cfg forced-failure test must drain Bob's relay channel after forced A2
signing failure and require it to be empty.

## false no-mutation guard

The cfg forced-failure test must explicitly avoid an A2 pre-mutation
no-mutation claim. It must assert:

- Alice has non-empty pending state and no Bob session before A2 processing;
- Alice has a Bob session after forced A2 signing failure;
- Alice effective pending state is cleared after forced A2 signing failure.

## inherited B1 provider RNG tests

Run:

```bash
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test b1_signature_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test b1_signature_provider_rng_failure -- --test-threads=1 --nocapture
```

Required: both pass. B1 evidence remains background only.

## inherited KEM provider RNG tests

Run:

```bash
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test kem_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test kem_provider_rng_failure -- --test-threads=1 --nocapture
```

Required: both pass. KEM evidence remains background only.

## inherited RNG residual tests

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

Required: pass. This is dependency-health evidence only.

## nested fuzz lock audit

Run:

```bash
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
rg -n "pqcrypto-mlkem|pqcrypto-traits|pqcrypto-internals" qsl/qsl-client/qsc/fuzz/Cargo.lock || true
```

Required: audit passes. The residual scan is inventory evidence only.

## qsc adversarial check

Run:

```bash
rg -n "NA0439_QSC_PROVIDER_ERROR_NO_MUTATION_ADVERSARIAL_STEP|handshake_provider_error_no_mutation" scripts/ci/qsc_adversarial.sh
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
```

If feasible, run `scripts/ci/qsc_adversarial.sh`. If local cargo-fuzz is
unavailable, record exact output and rely on PR CI qsc-adversarial-smoke.

## formal checks

Run:

```bash
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

Required: both pass.

## scope guard

Run:

```bash
git diff --check
git diff --name-only origin/main...HEAD
```

Required changed paths are exactly the allowed NA-0463 implementation and
governance paths. No Cargo, lockfile, workflow, refimpl, service, public-doc,
fuzz-target, vector, formal model, backup, qsl-backup, status/plan, or rollback
path may appear.

## public claim boundary

Scan added lines and PR body for prohibited overclaims. Required:

- No public-readiness claim.
- No production-readiness claim.
- No public-internet-readiness claim.
- No external-review-complete claim.
- No crypto-complete claim.
- No signature-complete claim.
- No identity-complete claim.
- No RNG-failure-complete claim.
- No provider-RNG-complete claim.
- No side-channel-free claim.
- No vulnerability-free claim.
- No bug-free claim.
- No perfect-crypto claim.

## closeout prerequisites

Closeout to NA-0464 may run only after the NA-0463 implementation PR merges and
post-merge public-safety is green. Closeout must restore exactly one READY item
and must not implement NA-0464.
