Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-15

# NA-0481 qsl refimpl Signature Provider Boundary Test Implementation Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0481 implements only the selected refimpl ML-DSA signature provider-boundary integration tests while preserving no-runtime/no-crypto/no-dependency/no-workflow/no-public-claim boundaries.

## Protected invariants

- NA-0481 consumes NA-0480.
- qwork proof files are read and copied, not regenerated.
- The only implementation file added is `tools/refimpl/quantumshield_refimpl/tests/signature_provider_boundary.rs`.
- Wrong public-key length rejects with an error.
- Wrong signature length rejects with an error.
- Malformed signing-key length rejects with an error.
- Tampered signature returns invalid, not success.
- Wrong public key returns invalid, not success.
- Err versus `Ok(false)` classification is tested where exposed by current refimpl APIs.
- Provider RNG failure remains residual unless a later exact directive authorizes a seam or source/runtime mutation.
- qsc sanitized error mapping remains supporting-only.
- KEM provider-boundary remains supporting-only for this lane.
- No refimpl source mutation is performed.
- No qsc source or executable-test mutation is performed.
- No runtime, crypto, dependency, Cargo, lockfile, workflow, fuzz target, vector, formal model, service, public, website, README, START_HERE, backup, restore, qsl-backup, status, plan, rollback, durable Director State Index, or public technical paper mutation is performed.
- No public-readiness, production-readiness, public-internet-readiness, external-review-complete, crypto-complete, KEM-complete, signature-complete, qsc/refimpl-equivalence-complete, provider-boundary-complete, provider-RNG-complete, formal-proof-complete, side-channel-free, vulnerability-free, bug-free, or perfect-crypto claim is introduced.
- Cargo audit green remains dependency-health evidence only.
- Exactly one READY item remains before optional closeout.

## Allowed scope

- `tools/refimpl/quantumshield_refimpl/tests/signature_provider_boundary.rs`
- `docs/governance/evidence/NA-0481_qsl_refimpl_signature_provider_boundary_test_implementation_harness.md`
- this testplan
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

- refimpl source mutation;
- qsc source or executable-test mutation;
- runtime/crypto/dependency/Cargo/lockfile/workflow mutation;
- fuzz target, vector, formal model, qsl-server, qsl-attachments, qshield runtime, qshield-cli, website, public-doc, README, START_HERE, qwork/qstart/qresume/qshell, backup, restore, qsl-backup, status, plan, rollback, backup tree, durable Director State Index, or public technical paper mutation;
- public overclaim or completion claims.

## Signature provider-boundary tests

Required test file:

`tools/refimpl/quantumshield_refimpl/tests/signature_provider_boundary.rs`

The test crate must remain gated with `#![cfg(feature = "pqcrypto")]`, because the public refimpl ML-DSA provider APIs exercised by this suite are available only under the `pqcrypto` feature. The no-feature target should compile with zero tests; the feature-enabled target must run the marker-bearing tests.

Required cases:

- wrong public-key length returns `Err(CryptoError::InvalidKey)`;
- wrong signature length returns `Err(CryptoError::InvalidKey)`;
- malformed signing-key length returns `Err(CryptoError::InvalidKey)`;
- tampered length-valid signature returns `Ok(false)`;
- wrong length-valid public key returns `Ok(false)`;
- classification test proves malformed inputs are Err while well-shaped invalid verification inputs are false.

## Marker plan

Required markers:

- `NA0481_PROVIDER_BOUNDARY_SCOPE_CONSUMED_OK`
- `NA0481_REFIMPL_SIGNATURE_WRONG_PUBLIC_KEY_LENGTH_REJECT_OK`
- `NA0481_REFIMPL_SIGNATURE_WRONG_SIGNATURE_LENGTH_REJECT_OK`
- `NA0481_REFIMPL_SIGNATURE_MALFORMED_SIGNING_KEY_REJECT_OK`
- `NA0481_REFIMPL_SIGNATURE_TAMPERED_SIGNATURE_INVALID_OK`
- `NA0481_REFIMPL_SIGNATURE_WRONG_PUBLIC_KEY_INVALID_OK`
- `NA0481_REFIMPL_SIGNATURE_ERR_VS_FALSE_CLASSIFICATION_OK`
- `NA0481_NO_RUNTIME_CHANGE_OK`
- `NA0481_NO_DEPENDENCY_CHANGE_OK`
- `NA0481_NO_WORKFLOW_CHANGE_OK`
- `NA0481_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0481_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0481_NO_KEM_COMPLETE_CLAIM_OK`
- `NA0481_NO_SIGNATURE_COMPLETE_CLAIM_OK`
- `NA0481_NO_PROVIDER_BOUNDARY_COMPLETE_CLAIM_OK`
- `NA0481_NO_QSC_REFIMPL_EQUIVALENCE_COMPLETE_CLAIM_OK`
- `NA0481_NO_PUBLIC_CLAIM_EXPANSION_OK`
- `NA0481_ONE_READY_INVARIANT_OK`

## Inherited tests

Run:

```bash
python3 formal/model_qsc_kem_signature_transcript_binding_bounded.py
python3 formal/run_model_checks.py
cargo test -p qsc --locked --test kem_signature_transcript_binding_negative -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test tui_account_bootstrap_transactionality -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test tui_account_bootstrap_transactionality -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test cli_identity_rotation_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test cli_identity_rotation_provider_rng_failure -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test legacy_identity_public_record_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test legacy_identity_public_record_provider_rng_failure -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test lazy_identity_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test lazy_identity_provider_rng_failure -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test a2_signature_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test a2_signature_provider_rng_failure -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test b1_signature_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test b1_signature_provider_rng_failure -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test kem_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test kem_provider_rng_failure -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test key_lifecycle_zeroization -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test pqkem768
```

## Root audit

Run:

```bash
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo tree -i ml-kem --locked || true
cargo tree -i pqcrypto-mlkem --locked || true
cargo tree -i pqcrypto-traits --locked || true
cargo tree -i pqcrypto-internals --locked || true
```

Root audit must be green. Cargo audit output is dependency-health evidence only.

## Nested fuzz lock audit

Run:

```bash
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
rg -n "pqcrypto-mlkem|pqcrypto-traits|pqcrypto-internals" qsl/qsl-client/qsc/fuzz/Cargo.lock || true
```

Nested audit must be green.

## qsc adversarial check

Run:

```bash
rg -n "NA0439_QSC_PROVIDER_ERROR_NO_MUTATION_ADVERSARIAL_STEP|handshake_provider_error_no_mutation" scripts/ci/qsc_adversarial.sh
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
```

If feasible, run:

```bash
sh scripts/ci/qsc_adversarial.sh
```

If local cargo-fuzz is unavailable, record exact output and rely on PR CI qsc-adversarial-smoke.

## Public claim boundary

Evidence and PR wording must not introduce public-readiness, production-readiness, public-internet-readiness, external-review-complete, crypto-complete, KEM-complete, signature-complete, qsc/refimpl-equivalence-complete, provider-boundary-complete, provider-RNG-complete, formal-proof-complete, side-channel-free, vulnerability-free, bug-free, or perfect-crypto claims.

## Closeout prerequisites

Do not close NA-0481 until:

- implementation PR merged;
- post-merge public-safety is green;
- D-0950 exists once on main;
- READY remains NA-0481 before closeout;
- selected NA-0482 successor preserves no-runtime/no-crypto/no-dependency/no-workflow/no-public-overclaim boundaries;
- closeout changes are confined to the optional closeout paths.

## PR body requirements

The PR body must include:

```md
Goals: G1, G2, G3, G4, G5
Impact:
No-regression:
Tests/Vectors:
```

The body must state:

- refimpl signature provider-boundary test implementation;
- exact changed paths;
- no refimpl source mutation;
- no qsc mutation;
- no dependency, workflow, formal, vector, or fuzz mutation;
- no public overclaim;
- no external-review-complete claim;
- no KEM/signature/qsc-refimpl-equivalence/provider-boundary/formal-proof-complete claim.

## Acceptance criteria

- D-0950 exists once after edit.
- D-0951 is absent before optional closeout.
- Duplicate decision count is 0.
- Changed paths are limited to the six allowed NA-0481 paths.
- New refimpl signature provider-boundary test passes and emits all required markers.
- Existing refimpl `pqkem768` passes.
- Root cargo audit is green.
- Nested qsc fuzz lock audit is green.
- Formal model and formal runner are green.
- Inherited qsc tests are green.
- qsc adversarial syntax is green.
- No public overclaim is introduced.
- No backup or restore is run.
- Exactly one READY item remains before optional closeout.
