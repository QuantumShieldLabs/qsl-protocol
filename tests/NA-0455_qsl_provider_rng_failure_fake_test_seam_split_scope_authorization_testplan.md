Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-10

# NA-0455 QSL Provider RNG Failure Fake / Test Seam Split-Scope Authorization Testplan

Goals: G1, G2, G3, G4, G5

## Purpose

Validate the NA-0455 provider RNG fake/test-seam split-scope authorization plan
and prove that it selects qsc-first NA-0456 without implementation mutation.

## Required markers

- `NA0455_PROVIDER_RNG_SPLIT_SCOPE_CONSUMED_OK`
- `NA0455_QWORK_PROOF_FILE_VERIFIED_OK`
- `NA0455_NA0454_INHERITANCE_CONSUMED_OK`
- `NA0455_SPLIT_SCOPE_INVENTORY_OK`
- `NA0455_QSC_FIRST_REVIEW_OK`
- `NA0455_REFIMPL_FIRST_REVIEW_OK`
- `NA0455_QSC_FIRST_SELECTED_OK`
- `NA0455_SUCCESSOR_NA0456_SELECTED_OK`
- `NA0455_NO_RUNTIME_CHANGE_OK`
- `NA0455_NO_CRYPTO_CHANGE_OK`
- `NA0455_NO_DEPENDENCY_CHANGE_OK`
- `NA0455_NO_WORKFLOW_CHANGE_OK`
- `NA0455_NO_TEST_IMPLEMENTATION_OK`
- `NA0455_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0455_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0455_NO_RNG_FAILURE_COMPLETE_CLAIM_OK`
- `NA0455_NO_PROVIDER_RNG_COMPLETE_CLAIM_OK`
- `NA0455_STEWARD_REVIEW_TEMPLATE_USED_OK`
- `NA0455_ONE_READY_INVARIANT_OK`

## qwork proof-file gate

Verify Codex did not run `qwork`, `qstart`, or `qresume`.

Verify read-only proof files:

- `/srv/qbuild/work/NA-0455/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0455/.qwork/startup.qsl-protocol.json`

Required result:

- proof parse succeeds;
- proof HEAD equals live HEAD before fetch;
- proof `origin/main` equals live `origin/main` before fetch;
- fetch does not advance `origin/main`;
- PR #1178 is merged at `c6dca3e2415`;
- READY_COUNT 1 and READY NA-0455.

## Split-scope checks

Required:

- NA-0454 provider RNG fake/test-seam split-scope inheritance is consumed;
- qsc KEM, signature/identity, and handshake provider RNG no-mutation surfaces
  are inventoried;
- refimpl ML-KEM, ML-DSA, X25519, and `Rng12` provider RNG surfaces are
  inventoried;
- qsc-first is classified as `QSC_PROVIDER_RNG_SPLIT_SCOPE_READY`;
- refimpl-first is classified as `REFIMPL_PROVIDER_RNG_NEEDS_MORE_TRIAGE`;
- primary classification is `PROVIDER_RNG_SPLIT_QSC_FIRST`;
- selected successor is
  `NA-0456 -- QSL qsc Provider RNG Failure No-Mutation Scope Authorization Plan`.

## Future scope checks

Future NA-0456 qsc-first authorization paths must be limited to:

- `docs/governance/evidence/NA-0456_qsl_qsc_provider_rng_failure_no_mutation_scope_authorization_plan.md`
- `tests/NA-0456_qsl_qsc_provider_rng_failure_no_mutation_scope_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Candidate implementation paths remain candidates only and are not authorized by
NA-0455:

- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/src/identity/mod.rs`
- one exact future qsc test path under `qsl/qsl-client/qsc/tests/`

NA-0455 must not authorize direct runtime, crypto, dependency, Cargo, lockfile,
workflow, executable test, fuzz target, vector, formal model, service, public,
backup, restore, qsl-backup, or qwork tooling mutation.

## Validation commands

Run:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
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
output and rely on PR CI qsc-adversarial-smoke if attached or required.

## Scope guard

Changed paths must be limited to:

- `docs/governance/evidence/NA-0455_qsl_provider_rng_failure_fake_test_seam_split_scope_authorization_plan.md`
- `tests/NA-0455_qsl_provider_rng_failure_fake_test_seam_split_scope_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

No runtime, crypto, dependency, Cargo, lockfile, workflow, executable test,
fuzz target, vector, formal model, qsl-server, qsl-attachments, qshield runtime,
qshield-cli, website, public docs, README, START_HERE, qwork/qstart/qresume,
qshell, backup, restore, qsl-backup, backup status, backup plan, rollback, or
backup tree mutation is allowed.

## Public claim boundary

NA-0455 is bounded internal evidence only.

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

NA-0455 classifies provider RNG split scope as
`PROVIDER_RNG_SPLIT_QSC_FIRST`, records D-0897 once, leaves D-0898 absent before
optional closeout, keeps READY_COUNT one with READY NA-0455, and selects
qsc-first NA-0456 as the next successor.
