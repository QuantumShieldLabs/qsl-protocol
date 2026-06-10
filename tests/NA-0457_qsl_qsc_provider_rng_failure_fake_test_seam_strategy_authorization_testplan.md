Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-10

# NA-0457 QSL qsc Provider RNG Failure Fake / Test Seam Strategy Authorization Testplan

Goals: G1, G2, G3, G4, G5

## Purpose

Validate the NA-0457 qsc provider RNG failure fake/test-seam strategy
authorization plan and prove that it selects a KEM-only qsc cfg seam
implementation successor without implementing tests, seams, runtime code,
crypto code, dependencies, workflows, vectors, or public claims in NA-0457.

## Required markers

- `NA0457_QSC_PROVIDER_RNG_STRATEGY_CONSUMED_OK`
- `NA0457_QWORK_PROOF_FILE_VERIFIED_OK`
- `NA0457_NA0456_INHERITANCE_CONSUMED_OK`
- `NA0457_STEWARD_REVIEW_TEMPLATE_USED_OK`
- `NA0457_QSC_FAKE_SEAM_INVENTORY_OK`
- `NA0457_STRATEGY_MECHANISM_REVIEW_OK`
- `NA0457_QSC_KEM_PROVIDER_CFG_SEAM_READY_OK`
- `NA0457_QSC_SIGNATURE_IDENTITY_SPLIT_FURTHER_OK`
- `NA0457_COMBINED_REJECTED_KEM_ONLY_SELECTED_OK`
- `NA0457_QSC_STRATEGY_MATRIX_OK`
- `NA0457_QSC_KEM_IMPLEMENTATION_READY_SELECTED_OK`
- `NA0457_SUCCESSOR_NA0458_SELECTED_OK`
- `NA0457_NO_RUNTIME_CHANGE_OK`
- `NA0457_NO_CRYPTO_CHANGE_OK`
- `NA0457_NO_DEPENDENCY_CHANGE_OK`
- `NA0457_NO_WORKFLOW_CHANGE_OK`
- `NA0457_NO_TEST_IMPLEMENTATION_OK`
- `NA0457_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0457_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0457_NO_RNG_FAILURE_COMPLETE_CLAIM_OK`
- `NA0457_NO_PROVIDER_RNG_COMPLETE_CLAIM_OK`
- `NA0457_ONE_READY_INVARIANT_OK`

## qwork proof-file gate

Verify Codex did not run `qwork`, `qstart`, or `qresume`.

Verify read-only proof files:

- `/srv/qbuild/work/NA-0457/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0457/.qwork/startup.qsl-protocol.json`

Required result:

- proof parse succeeds;
- JSON mirrors `.kv` proof for lane, repo, path, HEAD, `origin/main`, READY
  count, queue top READY, requested lane status, and clean-state fields;
- proof HEAD equals live HEAD before fetch;
- proof `origin/main` equals live `origin/main` before fetch;
- fetch does not advance `origin/main`;
- PR #1182 is merged at `7b04809fac1d`;
- READY_COUNT 1 and READY NA-0457;
- D-0899 exists once;
- D-0900 exists once;
- D-0901 is absent before the NA-0457 patch.

## Strategy authorization checks

Required:

- NA-0456 inheritance is consumed;
- qsc fake/test-seam candidates are inventoried;
- qsc KEM keypair and responder KEM encap are classified as cfg-seam ready;
- KEM decap / `pq_decap_failed` remains generic provider-error evidence, not
  RNG-specific proof;
- signature/identity is classified as requiring further split before
  implementation;
- combined qsc KEM/signature/identity implementation is rejected for the next
  lane;
- refimpl-first is rejected as a prerequisite for qsc KEM state-boundary proof;
- documentation-only is rejected as insufficient because KEM-only scope is exact;
- primary classification is
  `QSC_PROVIDER_RNG_KEM_FAKE_SEAM_IMPLEMENTATION_READY`;
- selected successor is
  `NA-0458 -- QSL qsc KEM Provider RNG Failure Fake / Test Seam Implementation Harness`.

## Future scope checks

Future NA-0458 KEM-only implementation paths must be limited to:

- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/src/identity/mod.rs`
- `qsl/qsl-client/qsc/tests/kem_provider_rng_failure.rs`
- `docs/governance/evidence/NA-0458_qsl_qsc_kem_provider_rng_failure_fake_test_seam_implementation_harness.md`
- `tests/NA-0458_qsl_qsc_kem_provider_rng_failure_fake_test_seam_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future NA-0458 must not authorize signature/identity signing seams, X25519
seams, refimpl provider RNG fakes, dependency changes, Cargo/lockfile changes,
workflow changes, fuzz target changes, vector changes, formal model changes,
service changes, public docs, website changes, backup/restore changes,
qsl-backup changes, or public claim expansion unless a later exact directive
changes scope.

## Validation commands

Run:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --file <pr-body> --scan-overclaims
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
cargo tree -i ml-kem --locked || true
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

- `docs/governance/evidence/NA-0457_qsl_qsc_provider_rng_failure_fake_test_seam_strategy_authorization_plan.md`
- `tests/NA-0457_qsl_qsc_provider_rng_failure_fake_test_seam_strategy_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

No runtime, crypto, dependency, Cargo, lockfile, workflow, executable test,
fuzz target, vector, formal model, qsl-server, qsl-attachments, qshield runtime,
qshield-cli, website, public docs, README, START_HERE, qwork/qstart/qresume,
qshell, backup, restore, qsl-backup, backup status, backup plan, rollback, or
backup tree mutation is allowed in NA-0457.

## Public claim boundary

NA-0457 is bounded internal evidence only.

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

NA-0457 classifies qsc provider RNG fake/test-seam strategy as
`QSC_PROVIDER_RNG_KEM_FAKE_SEAM_IMPLEMENTATION_READY`, records D-0901 once,
leaves D-0902 absent before optional closeout, keeps READY_COUNT one with READY
NA-0457, and selects
`NA-0458 -- QSL qsc KEM Provider RNG Failure Fake / Test Seam Implementation Harness`.
