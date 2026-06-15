Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-15

# NA-0482 qsl Binding Negative Vector Suite Authorization Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate the NA-0482 authorization-only lane. The lane must consume NA-0481,
inventory current input/vector surfaces, inventory candidate negative vectors,
select a format/storage strategy, classify secret-material and public-claim
risk, decide combined versus split vector scope, select exactly one NA-0483
successor, and preserve no vector or implementation mutation in NA-0482.

## Protected invariants

- NA-0482 remains authorization-only before optional closeout.
- qwork proof files are read and copied, not regenerated.
- NA-0481 is consumed.
- Current input/vector surfaces are inventoried.
- KEM, signature, transcript/replay/suite, stale-record, refimpl
  provider-boundary, and formal-token vector candidates are inventoried.
- Vector format/storage strategy is selected.
- Secret-material and public-claim risk is classified.
- Combined versus split scope is decided.
- Exact NA-0483 successor is selected.
- No input or vector file is created or mutated by NA-0482.
- No implementation mutation is performed.
- No runtime, crypto, dependency, Cargo, lockfile, workflow, executable test,
  fuzz target, formal model, qsc source, refimpl source/test, service, public,
  website, README, START_HERE, qwork/qstart/qresume/qshell, backup, restore,
  qsl-backup, status, plan, rollback, durable Director State Index, or public
  technical paper mutation is performed.
- No public-readiness claim is introduced.
- No production-readiness claim is introduced.
- No public-internet-readiness claim is introduced.
- No external-review-complete claim is introduced.
- No crypto-complete claim is introduced.
- No vector-complete claim is introduced.
- No KEM-complete claim is introduced.
- No signature-complete claim is introduced.
- No identity-complete claim is introduced.
- No transcript-complete claim is introduced.
- No qsc/refimpl-equivalence-complete claim is introduced.
- No provider-boundary-complete claim is introduced.
- No provider-RNG-complete claim is introduced.
- No downgrade-proof claim is introduced.
- No replay-proof claim is introduced.
- No side-channel-free claim is introduced.
- No vulnerability-free claim is introduced.
- No bug-free claim is introduced.
- No perfect-crypto claim is introduced.
- Cargo audit green remains dependency-health evidence only.
- Exactly one READY item remains before optional closeout.

## Allowed scope

- `docs/governance/evidence/NA-0482_qsl_binding_negative_vector_suite_authorization_plan.md`
- this testplan
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

- input/vector mutation;
- implementation mutation;
- runtime/crypto/dependency/Cargo/lockfile/workflow mutation;
- qsc source or executable-test mutation;
- refimpl source or executable-test mutation;
- executable test/fuzz/vector/formal mutation;
- service/public/qshield/qsl-server/qsl-attachments mutation;
- backup/restore/qsl-backup/status/plan/rollback mutation;
- website, README, START_HERE, public docs, public paper, or durable Director
  State Index mutation.

## Required startup proof

Run and record:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py public-safety-status --sha "$(git rev-parse origin/main)"
```

Required:

- `READY_COUNT 1`;
- READY item is NA-0482;
- latest decision before edit is D-0951;
- D-0950 exists once;
- D-0951 exists once;
- D-0952 absent before edit;
- duplicate decision count is 0;
- current-main public-safety is green.

## Required review evidence

The evidence doc must include:

- Executive summary;
- Live NA-0482 scope;
- qwork proof-file verification;
- NA-0481 inheritance;
- Applicable Stewardship and Assurance Review;
- Current input / vector surface inventory;
- Negative vector candidate inventory;
- Vector format / storage strategy review;
- Secret-material / public-claim risk review;
- Combined vs split vector scope decision;
- Authorization decision;
- Future scope bundle;
- Future validation / marker plan;
- Public claim / external review / website boundary;
- Rejected alternatives;
- Backup-impact statement;
- Next recommendation.

## Selected future NA-0483 scope

Selected successor:

`NA-0483 -- QSL Binding Negative Vector Suite Implementation Harness`

Selected future vector/input paths:

- `inputs/suite2/internal_negative_binding_vectors/README.md`
- `inputs/suite2/internal_negative_binding_vectors/qsl_binding_negative_vector_manifest_v1.json`

Selected future governance paths:

- `docs/governance/evidence/NA-0483_qsl_binding_negative_vector_suite_implementation_harness.md`
- `tests/NA-0483_qsl_binding_negative_vector_suite_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future NA-0483 must not store private keys, signing keys, KEM secret keys,
passphrases, runtime keys, backup keys, operator data, user data, or live
service data in checked-in vectors.

## Validation commands

Run from the qsl-protocol repo root:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main \
  --allowed docs/governance/evidence/NA-0482_qsl_binding_negative_vector_suite_authorization_plan.md \
  --allowed tests/NA-0482_qsl_binding_negative_vector_suite_authorization_testplan.md \
  --allowed DECISIONS.md \
  --allowed TRACEABILITY.md \
  --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
bash scripts/ci/classify_ci_scope.sh $(git diff --name-only origin/main)
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions --select D-0950 --select D-0951 --select D-0952 --select D-0953
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
PYTHONDONTWRITEBYTECODE=1 python3 formal/model_qsc_kem_signature_transcript_binding_bounded.py
PYTHONDONTWRITEBYTECODE=1 python3 formal/run_model_checks.py
cargo test -p qsc --locked --test kem_signature_transcript_binding_negative -- --test-threads=1 --nocapture
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test signature_provider_boundary -- --test-threads=1 --nocapture
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
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo tree -i rustls-webpki --locked
cargo tree -i ml-kem --locked || true
cargo tree -i pqcrypto-mlkem --locked || true
cargo tree -i pqcrypto-traits --locked || true
cargo tree -i pqcrypto-internals --locked || true
cargo fmt --check
```

If feasible, run:

```bash
sh scripts/ci/qsc_adversarial.sh
```

If local cargo-fuzz is unavailable, record exact output and rely on PR CI
`qsc-adversarial-smoke`.

## Expected results

- D-0952 exists once after edit.
- D-0953 is absent before optional closeout.
- Duplicate decision count is 0.
- Changed paths are limited to the five allowed NA-0482 governance paths.
- Current input/vector surface inventory is present.
- Negative vector candidate matrix is present.
- Manifest-plus-test storage strategy is selected.
- Secret-material risk is classified `VECTOR_SECRET_MATERIAL_SAFE_SCOPE_READY`.
- Combined vector implementation is selected.
- Exact NA-0483 successor is selected.
- No input/vector mutation occurs.
- No runtime, crypto, dependency, Cargo, lockfile, workflow, test, fuzz,
  formal, refimpl, qsc, service, public, backup, restore, or qsl-backup
  mutation occurs.
- Root cargo audit is green.
- Nested qsc fuzz lock audit is green.
- Formal model and formal runner are green.
- Inherited qsc tests are green.
- Refimpl signature provider-boundary and `pqkem768` tests are green.
- qsc adversarial script syntax is green.
- No public overclaim is introduced.
- No backup or restore is run.
- Exactly one READY item remains before optional closeout.

## Markers

- `NA0482_QWORK_PROOF_FILE_VERIFIED_OK`
- `NA0482_NA0481_CONSUMED_OK`
- `NA0482_CURRENT_VECTOR_SURFACE_INVENTORY_OK`
- `NA0482_NEGATIVE_VECTOR_CANDIDATE_MATRIX_OK`
- `NA0482_MANIFEST_PLUS_TEST_STRATEGY_SELECTED_OK`
- `NA0482_VECTOR_SECRET_MATERIAL_SAFE_SCOPE_READY_OK`
- `NA0482_COMBINED_VECTOR_SCOPE_SELECTED_OK`
- `NA0482_BINDING_NEGATIVE_VECTOR_COMBINED_IMPLEMENTATION_READY_OK`
- `NA0482_NA0483_SUCCESSOR_SELECTED_OK`
- `NA0482_NO_IMPLEMENTATION_MUTATION_OK`
- `NA0482_NO_VECTOR_INPUT_MUTATION_OK`
- `NA0482_NO_PUBLIC_OVERCLAIM_OK`
- `NA0482_BACKUP_BOUNDARY_OK`

## PR body requirements

The PR body must include:

```md
Goals: G1, G2, G3, G4, G5
Impact:
No-regression:
Tests/Vectors:
```

The body must state:

- binding negative vector suite authorization;
- selected NA-0483 successor;
- no implementation mutation;
- no vector/input mutation;
- no runtime/crypto/dependency/Cargo/lockfile/workflow/test/fuzz/formal/refimpl mutation;
- no public overclaim;
- no external-review-complete claim;
- no vector/KEM/signature/qsc-refimpl-equivalence/provider-boundary/formal-proof-complete claim.
