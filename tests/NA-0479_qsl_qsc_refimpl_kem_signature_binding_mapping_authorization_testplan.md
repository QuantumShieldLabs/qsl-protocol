Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-14

# NA-0479 qsc/refimpl KEM / Signature Binding Mapping Authorization Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate the NA-0479 governance-only mapping authorization for qsc/refimpl KEM
and signature binding semantics after qsc binding negative tests and the
NA-0478 bounded formal model.

## Protected invariants

- NA-0479 is read-only authorization and mapping evidence.
- qwork proof files are read and copied, not regenerated.
- qsc KEM and signature assumptions are inventoried.
- refimpl KEM and signature provider surfaces are reviewed.
- qsc/refimpl error semantics are mapped.
- NA-0478 opaque-token formal assumptions are mapped to qsc/refimpl surfaces.
- Exactly one NA-0480 successor is selected.
- No implementation mutation is performed.
- No runtime, crypto, dependency, Cargo, lockfile, workflow, executable test,
  fuzz target, vector, formal model, qsc source, refimpl, service, public,
  website, README, START_HERE, qwork/qstart/qresume/qshell, backup, restore,
  qsl-backup, rollback, durable Director State Index, or public technical paper
  mutation is performed.
- No public-readiness, production-readiness, public-internet-readiness,
  external-review-complete, crypto-complete, KEM-complete, signature-complete,
  identity-complete, transcript-complete, qsc/refimpl-equivalence-complete,
  formal-proof-complete, side-channel-free, vulnerability-free, bug-free, or
  perfect-crypto claim is introduced.
- Cargo audit green remains dependency-health evidence only.

## Allowed scope

- `docs/governance/evidence/NA-0479_qsl_qsc_refimpl_kem_signature_binding_mapping_authorization_plan.md`
- this testplan
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

- implementation mutation;
- runtime/crypto/dependency/Cargo/lockfile/workflow mutation;
- executable test/fuzz/vector/formal mutation;
- qsc source mutation;
- refimpl mutation;
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
- READY item is NA-0479;
- latest decision before edit is D-0945;
- D-0944 exists once;
- D-0945 exists once;
- D-0946 absent before edit;
- duplicate decision count is 0;
- current-main public-safety is green.

## Required mapping review

Review read-only:

- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/src/identity/mod.rs`
- qsc binding/provider-error tests
- `tools/refimpl/quantumshield_refimpl/src/crypto/traits.rs`
- `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs`
- `tools/refimpl/quantumshield_refimpl/tests/pqkem768.rs`
- refimpl signature provider unit coverage
- `formal/model_qsc_kem_signature_transcript_binding_bounded.py`
- NA-0478 evidence and testplan

Required evidence sections:

- qsc KEM / signature assumption inventory;
- refimpl KEM surface review;
- refimpl signature surface review;
- qsc / refimpl error semantics mapping;
- formal model / refimpl mapping review;
- mapping gap matrix;
- successor selection;
- future scope bundle.

## Validation commands

Run:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main \
  --allowed docs/governance/evidence/NA-0479_qsl_qsc_refimpl_kem_signature_binding_mapping_authorization_plan.md \
  --allowed tests/NA-0479_qsl_qsc_refimpl_kem_signature_binding_mapping_authorization_testplan.md \
  --allowed DECISIONS.md \
  --allowed TRACEABILITY.md \
  --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
bash scripts/ci/classify_ci_scope.sh $(git diff --name-only origin/main)
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions --select D-0944 --select D-0945 --select D-0946 --select D-0947
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
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
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo tree -i rustls-webpki --locked
cargo tree -i ml-kem --locked || true
cargo tree -i pqcrypto-mlkem --locked || true
cargo tree -i pqcrypto-traits --locked || true
cargo tree -i pqcrypto-internals --locked || true
cargo fmt --check
```

If feasible, run the qsc adversarial script locally. If local cargo-fuzz is not
available, record exact output and rely on PR CI qsc-adversarial-smoke.

## PR body requirements

The PR body must include:

```md
Goals: G1, G2, G3, G4, G5
Impact:
No-regression:
Tests/Vectors:
```

The body must state:

- qsc/refimpl KEM signature binding mapping authorization;
- selected NA-0480 successor;
- no implementation mutation;
- no runtime/crypto/dependency/Cargo/lockfile/workflow/test/fuzz/vector/formal/refimpl mutation;
- no public overclaim;
- no external-review-complete claim;
- no KEM/signature/qsc-refimpl-equivalence/formal-proof-complete claim.

Validate with:

```bash
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --file "$PR_BODY_FILE"
python3 tools/goal_lint.py
```

## Acceptance criteria

- D-0946 exists once after edit.
- D-0947 absent before optional closeout.
- Duplicate decision count is 0.
- Changed paths are limited to the five allowed NA-0479 paths.
- Root cargo audit is green.
- Nested qsc fuzz lock audit is green.
- Formal model and formal runner are green.
- Inherited qsc tests are green.
- Refimpl `pqkem768` is green.
- qsc adversarial script syntax is green.
- No public overclaim is introduced.
- No backup or restore is run.
- Exactly one READY item remains before optional closeout.
