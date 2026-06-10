Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-10

# NA-0454 QSL Provider RNG Failure Fake / Test Seam Strategy Authorization Testplan

Goals: G1, G2, G3, G4, G5

## Purpose

Validate the NA-0454 provider RNG fake/test-seam strategy authorization plan and
prove that it selects split-scope NA-0455 without implementation mutation.

## Required markers

- `NA0454_PROVIDER_RNG_STRATEGY_CONSUMED_OK`
- `NA0454_QWORK_PROOF_FILE_VERIFIED_OK`
- `NA0454_NA0453_INHERITANCE_CONSUMED_OK`
- `NA0454_EXISTING_FAKE_SEAM_INVENTORY_OK`
- `NA0454_STRATEGY_OPTIONS_REVIEWED_OK`
- `NA0454_QSC_REFIMPL_SPLIT_STRATEGY_SELECTED_OK`
- `NA0454_PROVIDER_RNG_FAKE_SEAM_SPLIT_SCOPE_NEEDED_OK`
- `NA0454_SUCCESSOR_NA0455_SELECTED_OK`
- `NA0454_NO_RUNTIME_CHANGE_OK`
- `NA0454_NO_CRYPTO_CHANGE_OK`
- `NA0454_NO_DEPENDENCY_CHANGE_OK`
- `NA0454_NO_WORKFLOW_CHANGE_OK`
- `NA0454_NO_TEST_IMPLEMENTATION_OK`
- `NA0454_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0454_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0454_NO_RNG_FAILURE_COMPLETE_CLAIM_OK`
- `NA0454_NO_PROVIDER_RNG_COMPLETE_CLAIM_OK`
- `NA0454_STEWARD_REVIEW_TEMPLATE_USED_OK`
- `NA0454_ONE_READY_INVARIANT_OK`

## qwork proof-file gate

Verify Codex did not run `qwork`, `qstart`, or `qresume`.

Verify read-only proof files:

- `/srv/qbuild/work/NA-0454/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0454/.qwork/startup.qsl-protocol.json`

Required result:

- proof parse succeeds;
- proof HEAD equals live HEAD before fetch;
- proof `origin/main` equals live `origin/main` before fetch;
- fetch does not advance `origin/main`;
- PR #1176 is merged at `7009a0c29f0d`;
- READY_COUNT 1 and READY NA-0454.

## Strategy checks

Required:

- NA-0453 provider RNG boundary inheritance is consumed;
- existing qsc cfg seams are recorded as qsc-local RNG seams only;
- existing qsc provider-error no-mutation proof is recorded as generic
  provider-error evidence only;
- existing refimpl test fakes are recorded as generic provider fakes, not
  concrete provider RNG failure proof;
- concrete provider RNG failure remains hidden behind `OsRng` or infallible
  trait/helper methods;
- qsc no-mutation proof is separated from refimpl provider-boundary proof;
- primary classification is `PROVIDER_RNG_FAKE_SEAM_SPLIT_SCOPE_NEEDED`;
- selected successor is
  `NA-0455 -- QSL Provider RNG Failure Fake / Test Seam Split-Scope Authorization Plan`.

## Future scope checks

Future NA-0455 split-scope allowed paths must be limited to:

- `docs/governance/evidence/NA-0455_qsl_provider_rng_failure_fake_test_seam_split_scope_authorization_plan.md`
- `tests/NA-0455_qsl_provider_rng_failure_fake_test_seam_split_scope_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

NA-0454 must not authorize direct runtime, crypto, dependency, Cargo, lockfile,
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

- `docs/governance/evidence/NA-0454_qsl_provider_rng_failure_fake_test_seam_strategy_authorization_plan.md`
- `tests/NA-0454_qsl_provider_rng_failure_fake_test_seam_strategy_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

No runtime, crypto, dependency, Cargo, lockfile, workflow, executable test,
fuzz target, vector, formal model, qsl-server, qsl-attachments, qshield runtime,
qshield-cli, website, public docs, README, START_HERE, qwork/qstart/qresume,
qshell, backup, restore, qsl-backup, backup status, backup plan, rollback, or
backup tree mutation is allowed.

## Public claim boundary

NA-0454 is bounded internal evidence only.

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

NA-0454 classifies provider RNG fake/test-seam strategy as
`PROVIDER_RNG_FAKE_SEAM_SPLIT_SCOPE_NEEDED`, records D-0895 once, leaves
D-0896 absent before optional closeout, keeps READY_COUNT one with READY
NA-0454, and selects NA-0455 split-scope authorization as the next successor.
