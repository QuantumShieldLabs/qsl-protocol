Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-11

# NA-0462 qsc A2 Signature Provider RNG Failure Scope Authorization Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate the NA-0462 authorization-only lane for qsc A2 signature provider RNG
failure scope. The lane must consume NA-0461, classify A2 signing state timing,
select one truthful successor, and make no runtime, crypto, dependency, Cargo,
lockfile, workflow, executable-test, fuzz-target, vector, formal-model,
service, public-surface, refimpl, backup, restore, qsl-backup, or qwork
mutation.

## Protected invariants

- NA-0461 B1 signing evidence is consumed and remains bounded internal qsc
  evidence.
- A2 signing is classified as post-mutation/pre-output.
- A2 must not reuse the B1 pre-mutation no-mutation invariant.
- The selected A2 invariant is no A2 output plus sanitized fail-closed
  `sig_sign_failed`.
- The selected successor is implementation-ready only for the exact no-output
  future scope.
- Identity provider RNG, X25519 / ephemeral generation, and refimpl provider
  RNG remain residual.
- Cargo audit green remains dependency-health evidence only.
- No public-readiness, production-readiness, external-review-complete,
  crypto-complete, signature-complete, identity-complete,
  RNG-failure-complete, provider-RNG-complete, side-channel-free,
  vulnerability-free, bug-free, or perfect-crypto claim is made.
- Exactly one READY item remains mandatory.

## Allowed scope

Changed paths must be limited to:

- `docs/governance/evidence/NA-0462_qsl_qsc_a2_signature_provider_rng_failure_scope_authorization_plan.md`
- `tests/NA-0462_qsl_qsc_a2_signature_provider_rng_failure_scope_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

No mutation is allowed outside the paths above. In particular, no mutation is
allowed to qsc source, executable qsc tests, refimpl, dependencies, Cargo
manifests, lockfiles, workflows, fuzz target source, vectors, formal models,
qsl-server, qsl-attachments, qshield runtime, qshield-cli, website, public docs,
README, START_HERE, qwork/qstart/qresume/qshell, backup, restore, qsl-backup,
backup status, backup plan, rollback, or backup tree paths.

## qwork proof-file verification

Required:

- Read `/srv/qbuild/work/NA-0462/.qwork/startup.qsl-protocol.kv`.
- Read `/srv/qbuild/work/NA-0462/.qwork/startup.qsl-protocol.json`.
- Verify both are present and valid.
- Verify startup OK, lane NA-0462, repo qsl-protocol, expected path, clean
  worktree/index/untracked state, READY_COUNT 1, READY NA-0462, and requested
  lane status READY.
- Verify JSON mirrors `.kv` for lane, repo, path, HEAD, origin/main, clean
  state, READY count, queue top READY, and requested lane status.
- Verify proof HEAD and proof origin/main match live local refs before fetch.
- Do not run `qwork`, `qstart`, or `qresume`.

## source timing proof

Required read-only source evidence:

- `qsl/qsl-client/qsc/src/handshake/mod.rs` line 1429 stores the initiator
  session.
- `qsl/qsl-client/qsc/src/handshake/mod.rs` line 1431 clears initiator pending
  state.
- `qsl/qsl-client/qsc/src/handshake/mod.rs` line 1453 signs A2.
- `qsl/qsl-client/qsc/src/handshake/mod.rs` lines 1456-1461 emit sanitized
  `sig_sign_failed` and return on A2 signing failure.
- `qsl/qsl-client/qsc/src/handshake/mod.rs` lines 1477-1482 emit and relay A2
  output after successful signing.

Required classification:

`A2_STATE_TIMING_POST_MUTATION_PRE_OUTPUT`

Forbidden classification:

`A2_STATE_TIMING_PRE_MUTATION`

## selected successor proof

Required primary classification:

`A2_SIGNATURE_PROVIDER_RNG_NO_OUTPUT_IMPLEMENTATION_READY`

Required successor:

`NA-0463 -- QSL qsc A2 Signature Provider RNG Failure No-Output Test Seam Implementation Harness`

Required future paths if closeout restores NA-0463:

- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/tests/a2_signature_provider_rng_failure.rs`
- `docs/governance/evidence/NA-0463_qsl_qsc_a2_signature_provider_rng_failure_no_output_test_seam_implementation_harness.md`
- `tests/NA-0463_qsl_qsc_a2_signature_provider_rng_failure_no_output_test_seam_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## inherited qsc tests

Run:

```bash
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test b1_signature_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test b1_signature_provider_rng_failure -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test kem_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test kem_provider_rng_failure -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test rng_failure_residual_surfaces -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test rng_failure_residual_surfaces -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test rng_failure_behavior -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test rng_failure_behavior -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test key_lifecycle_zeroization -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test pqkem768
```

Required: all pass.

## dependency health

Run:

```bash
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo tree -i rustls-webpki --locked
cargo tree -i ml-kem --locked || true
cargo tree -i pqcrypto-mlkem --locked || true
cargo tree -i pqcrypto-traits --locked || true
cargo tree -i pqcrypto-internals --locked || true
rg -n "pqcrypto-mlkem|pqcrypto-traits|pqcrypto-internals" qsl/qsl-client/qsc/fuzz/Cargo.lock || true
```

Required: audits pass. Expected zero-match inverse probes and fuzz-lock scan are
inventory evidence only.

## qsc adversarial and formal checks

Run:

```bash
rg -n "NA0439_QSC_PROVIDER_ERROR_NO_MUTATION_ADVERSARIAL_STEP|handshake_provider_error_no_mutation" scripts/ci/qsc_adversarial.sh
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

If feasible, run `scripts/ci/qsc_adversarial.sh`. If local `cargo fuzz` is not
available, record exact output and rely on PR CI qsc-adversarial-smoke.

## governance validation

Run:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --file <pr-body> --scan-overclaims
cargo fmt --check
```

Required:

- READY_COUNT 1.
- READY NA-0462.
- D-0911 exists once after patch.
- D-0912 absent before optional closeout.
- duplicate decision count zero.
- changed paths are limited to the five allowed NA-0462 paths.
- no public overclaim is detected.
- no false A2 no-mutation claim is introduced.

## public claim boundary

NA-0462 is bounded internal governance evidence only. No public-readiness claim
is allowed. No production-readiness claim is allowed. No public-internet
readiness claim is allowed. No external-review-complete claim is allowed. No
crypto-complete claim is allowed. No signature-complete claim is allowed. No
identity-complete claim is allowed. No RNG-failure-complete claim is allowed.
No provider-RNG-complete claim is allowed. No side-channel-free claim is
allowed. No vulnerability-free claim is allowed. No bug-free claim is allowed.
No perfect-crypto claim is allowed.

## closeout prerequisites

Closeout to NA-0463 may run only after the NA-0462 evidence PR merges and
post-merge public-safety is green. Closeout must restore exactly one READY item
and must not implement NA-0463.
