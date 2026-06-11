Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-11

# NA-0461 qsc B1 Signature Provider RNG Failure Test Seam Implementation Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate the bounded NA-0461 implementation of a qsc cfg-gated B1 responder
signature provider-failure seam. The lane must force only the selected B1
signing failure path under `qsc_rng_failure_test_seam`, prove sanitized
`sig_sign_failed`, prove no selected responder pending/session mutation, prove
no B1 output, and prove normal no-cfg builds ignore the selector.

## Protected invariants

- B1 signing forced failure occurs before responder pending/session insertion.
- B1 signing forced failure occurs before B1 relay output.
- The forced failure returns sanitized `sig_sign_failed`.
- Normal no-cfg builds do not read or act on the seam selector at the B1 signing
  site.
- A2 signing remains deferred.
- Identity provider RNG remains deferred.
- X25519 / ephemeral RNG remains deferred.
- refimpl provider RNG remains deferred.
- NA-0458 KEM provider RNG seam evidence remains background preserved.
- No dependency, Cargo, lockfile, workflow, fuzz target, vector, formal model,
  refimpl, qsl-server, qsl-attachments, qshield runtime, qshield-cli, public
  docs, website, README, START_HERE, backup, restore, qsl-backup, status, plan,
  rollback, or backup tree mutation is allowed.
- Exactly one READY item remains mandatory.

## Allowed scope

Changed paths must be limited to:

- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/tests/b1_signature_provider_rng_failure.rs`
- `docs/governance/evidence/NA-0461_qsl_qsc_b1_signature_provider_rng_failure_test_seam_implementation_harness.md`
- `tests/NA-0461_qsl_qsc_b1_signature_provider_rng_failure_test_seam_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

No mutation is allowed outside the paths above. In particular, no mutation is
allowed to identity source, CLI source, TUI command source, refimpl,
dependencies, Cargo manifests, lockfiles, workflows, fuzz target source,
vectors, formal models, qsl-server, qsl-attachments, qshield runtime,
qshield-cli, website, public docs, README, START_HERE, qwork/qstart/qresume,
qshell, backup, restore, qsl-backup, backup status, backup plan, rollback, or
backup tree paths.

## cfg seam tests

Run:

```bash
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test b1_signature_provider_rng_failure -- --test-threads=1 --nocapture
```

Required:

- `b1_signature_rng_failure_writes_no_responder_state_or_b1` passes.
- `NA0461_B1_SIGNATURE_PROVIDER_RNG_SEAM_IMPLEMENTED_OK` appears.
- `NA0461_B1_SIGNATURE_PROVIDER_RNG_FAILURE_FORCED_OK` appears.
- `NA0461_B1_SIGNATURE_PROVIDER_RNG_FAILURE_SIG_SIGN_FAILED_OK` appears.
- `NA0461_B1_SIGNATURE_PROVIDER_RNG_FAILURE_NO_RESPONDER_MUTATION_OK` appears.
- `NA0461_B1_SIGNATURE_PROVIDER_RNG_FAILURE_NO_B1_OUTPUT_OK` appears.
- residual and no-claim markers appear.

## normal-build tests

Run:

```bash
cargo test -p qsc --locked --test b1_signature_provider_rng_failure -- --test-threads=1 --nocapture
```

Required:

- `b1_signature_rng_failure_seam_inactive_without_cfg` passes.
- `QSC_RNG_FAILURE_TEST_SEAM=QSC.SIG.B1` is ignored without cfg.
- normal B1 `sig_status` and `handshake_send msg=B1` are emitted.
- `sig_sign_failed` is absent.
- `NA0461_PRODUCTION_SEMANTICS_UNCHANGED_OK` appears.

## no responder mutation assertions

The forced cfg test must assert:

- Bob vault bytes are unchanged.
- `handshake.pending.bob.alice` is absent from the mock vault.
- legacy pending file is absent.
- Bob session blob for Alice is absent.

## no B1 output assertions

The forced cfg test must assert:

- no `event=handshake_send`;
- no `msg=B1`;
- no relay output on the initiator channel.

## inherited KEM provider RNG tests

Run:

```bash
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test kem_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test kem_provider_rng_failure -- --test-threads=1 --nocapture
```

Required: both pass. NA-0458 KEM evidence remains background preserved only.

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
cargo tree -i rustls-webpki --locked
cargo tree -i ml-kem --locked || true
cargo tree -i pqcrypto-mlkem --locked || true
cargo tree -i pqcrypto-traits --locked || true
cargo tree -i pqcrypto-internals --locked || true
```

Required: audit passes. pqcrypto inverse probes may be expected zero-match
inventory results under `|| true`.

## nested fuzz lock audit

Run:

```bash
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
rg -n "pqcrypto-mlkem|pqcrypto-traits|pqcrypto-internals" qsl/qsl-client/qsc/fuzz/Cargo.lock || true
```

Required: audit passes. Residual pqcrypto scan reports zero matches.

## qsc adversarial check

Run:

```bash
rg -n "NA0439_QSC_PROVIDER_ERROR_NO_MUTATION_ADVERSARIAL_STEP|handshake_provider_error_no_mutation" scripts/ci/qsc_adversarial.sh
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
```

If feasible, run `scripts/ci/qsc_adversarial.sh`; if local cargo-fuzz is
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
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --file <pr-body> --scan-overclaims
```

Required:

- READY_COUNT 1.
- READY NA-0461.
- D-0909 exists once after patch.
- D-0910 absent before optional closeout.
- duplicate decision count zero.
- changed paths are limited to the allowed NA-0461 paths.
- no public overclaim is detected.

## public claim boundary

NA-0461 is bounded internal qsc evidence only. No public-readiness claim is
allowed. No production-readiness claim is allowed. No public-internet-readiness
claim is allowed. No external-review-complete claim is allowed. No
crypto-complete claim is allowed. No signature-complete claim is allowed. No
identity-complete claim is allowed. No RNG-failure-complete claim is allowed.
No provider-RNG-complete claim is allowed. No side-channel-free claim is
allowed. No vulnerability-free claim is allowed. No bug-free claim is allowed.
No perfect-crypto claim is allowed.

## closeout prerequisites

Closeout to NA-0462 may run only after the NA-0461 implementation PR merges and
post-merge public-safety is green. Closeout must restore exactly one READY item
and must not implement NA-0462.
