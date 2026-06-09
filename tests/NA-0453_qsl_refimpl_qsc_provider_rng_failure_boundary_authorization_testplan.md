Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-09

# NA-0453 QSL refimpl / qsc Provider RNG Failure Boundary Authorization Testplan

Goals: G1, G2, G3, G4, G5

## Purpose

Validate the NA-0453 provider RNG boundary authorization plan and prove that it
selects the next strategy lane without implementation mutation.

## Required markers

- `NA0453_PROVIDER_RNG_BOUNDARY_AUTHORIZATION_CONSUMED_OK`
- `NA0453_QWORK_PROOF_FILE_VERIFIED_OK`
- `NA0453_NA0452_INHERITANCE_CONSUMED_OK`
- `NA0453_QSC_PROVIDER_RNG_SURFACES_CLASSIFIED_OK`
- `NA0453_REFIMPL_PROVIDER_RNG_SURFACES_CLASSIFIED_OK`
- `NA0453_PROVIDER_RNG_FAKE_OR_SEAM_AUTHORIZATION_SELECTED_OK`
- `NA0453_SUCCESSOR_NA0454_SELECTED_OK`
- `NA0453_NO_RUNTIME_CHANGE_OK`
- `NA0453_NO_CRYPTO_CHANGE_OK`
- `NA0453_NO_DEPENDENCY_CHANGE_OK`
- `NA0453_NO_WORKFLOW_CHANGE_OK`
- `NA0453_NO_TEST_IMPLEMENTATION_OK`
- `NA0453_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0453_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0453_NO_RNG_FAILURE_COMPLETE_CLAIM_OK`
- `NA0453_NO_PROVIDER_RNG_COMPLETE_CLAIM_OK`
- `NA0453_STEWARD_REVIEW_TEMPLATE_USED_OK`
- `NA0453_ONE_READY_INVARIANT_OK`

## qwork proof-file gate

Verify Codex did not run `qwork`, `qstart`, or `qresume`.

Verify read-only proof files:

- `/srv/qbuild/work/NA-0453/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0453/.qwork/startup.qsl-protocol.json`

Required result:

- proof parse succeeds;
- proof HEAD equals live HEAD before fetch;
- proof `origin/main` equals live `origin/main` before fetch;
- fetch does not advance `origin/main`;
- PR #1174 is merged at `2c73503c0a67`;
- READY_COUNT 1 and READY NA-0453.

## Surface classification checks

Required qsc classification:

- qsc KEM keypair, encap, and decap boundaries classified;
- qsc signature keypair, sign, and verify boundaries classified;
- qsc X25519 keypair boundary classified;
- qsc identity bootstrap provider boundary classified;
- existing qsc provider-error no-mutation evidence is distinguished from
  concrete provider RNG failure evidence;
- existing qsc RNG seam is recorded as unable to reach provider internals.

Required refimpl classification:

- ML-KEM keypair, encap, and decap boundaries classified;
- ML-DSA keypair, sign, and verify boundaries classified;
- X25519 keypair boundary classified;
- `Rng12` nonce boundary classified;
- existing provider fakes are recorded as generic provider-error tools, not
  concrete provider RNG failure proof.

## Successor checks

Required:

- primary classification is `PROVIDER_RNG_FAKE_OR_SEAM_AUTHORIZATION_NEXT`;
- selected successor is `NA-0454 -- QSL Provider RNG Failure Fake / Test Seam Strategy Authorization Plan`;
- future NA-0454 allowed paths are limited to its evidence doc, testplan,
  `DECISIONS.md`, `TRACEABILITY.md`, and rolling journal;
- NA-0453 authorizes no direct implementation lane.

## Validation commands

Run:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test rng_failure_residual_surfaces -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test rng_failure_residual_surfaces -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo test -p qsc --locked --test rng_failure_behavior -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test rng_failure_behavior -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test key_lifecycle_zeroization -- --test-threads=1 --nocapture
cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test pqkem768
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo tree -i rustls-webpki --locked
cargo tree -i ml-kem --locked
cargo tree -i pqcrypto-mlkem --locked || true
cargo tree -i pqcrypto-traits --locked || true
cargo tree -i pqcrypto-internals --locked || true
cargo fmt --check
python3 formal/model_qsc_handshake_suite_id_bounded.py
python3 formal/run_model_checks.py
```

If local cargo-fuzz is unavailable during qsc adversarial smoke, record exact
output and rely on PR CI qsc-adversarial-smoke.

## Scope guard

Changed paths must be limited to:

- `docs/governance/evidence/NA-0453_qsl_refimpl_qsc_provider_rng_failure_boundary_authorization_plan.md`
- `tests/NA-0453_qsl_refimpl_qsc_provider_rng_failure_boundary_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

No runtime, crypto, dependency, Cargo, lockfile, workflow, executable test,
fuzz target, vector, formal model, qsl-server, qsl-attachments, qshield runtime,
qshield-cli, website, public docs, README, START_HERE, qwork/qstart/qresume,
qshell, backup, restore, qsl-backup, backup status, backup plan, rollback, or
backup tree mutation is allowed.

## Public claim boundary

NA-0453 is bounded internal evidence only.

No public-readiness claim is allowed.

No production-readiness claim is allowed.

No public-internet-readiness claim is allowed.

No external-review-complete claim is allowed.

No crypto-complete claim is allowed.

No RNG-failure-complete claim is allowed.

No provider-RNG-complete claim is allowed.

No side-channel-free claim is allowed.

No vulnerability-free claim is allowed.

No bug-free claim is allowed.

No perfect-crypto claim is allowed.

Cargo audit green remains dependency-health evidence only.

## Expected result

NA-0453 classifies provider-dependent qsc and refimpl RNG boundaries, selects
`PROVIDER_RNG_FAKE_OR_SEAM_AUTHORIZATION_NEXT`, records D-0893 once, leaves
D-0894 absent before optional closeout, keeps READY_COUNT one with READY
NA-0453, and selects NA-0454 fake/test-seam strategy authorization as the next
successor.
