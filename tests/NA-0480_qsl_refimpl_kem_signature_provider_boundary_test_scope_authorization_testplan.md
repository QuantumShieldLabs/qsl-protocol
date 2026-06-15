Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-15

# NA-0480 qsl refimpl KEM / Signature Provider Boundary Test Scope Authorization Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate the NA-0480 governance-only authorization for the next refimpl KEM/signature provider-boundary test scope after NA-0479 mapping evidence.

## Protected invariants

- NA-0480 is authorization-only.
- qwork proof files are read and copied, not regenerated.
- NA-0479 inheritance is consumed.
- refimpl KEM provider-boundary candidates are inventoried and selected or rejected.
- refimpl signature provider-boundary candidates are inventoried and selected or rejected.
- combined versus split implementation scope is decided.
- provider RNG failure remains residual unless exact future scope can force it without source/runtime changes.
- qsc sanitized error mapping remains supporting-only unless exact future scope authorizes qsc test mutation.
- Exact NA-0481 successor is selected.
- No implementation mutation is performed.
- No runtime, crypto, dependency, Cargo, lockfile, workflow, executable test, fuzz target, vector, formal model, qsc source, refimpl source/test, service, public, website, README, START_HERE, qwork/qstart/qresume/qshell, backup, restore, qsl-backup, rollback, durable Director State Index, or public technical paper mutation is performed.
- No public-readiness, production-readiness, public-internet-readiness, external-review-complete, crypto-complete, KEM-complete, signature-complete, qsc/refimpl-equivalence-complete, provider-boundary-complete, provider-RNG-complete, formal-proof-complete, side-channel-free, vulnerability-free, bug-free, or perfect-crypto claim is introduced.
- Cargo audit green remains dependency-health evidence only.
- Exactly one READY item remains.

## Allowed scope

- `docs/governance/evidence/NA-0480_qsl_refimpl_kem_signature_provider_boundary_test_scope_authorization_plan.md`
- this testplan
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

- implementation mutation;
- runtime/crypto/dependency/Cargo/lockfile/workflow mutation;
- qsc source or qsc executable-test mutation;
- refimpl source or executable-test mutation;
- executable test/fuzz/vector/formal mutation;
- service/public/qshield/qsl-server/qsl-attachments mutation;
- backup/restore/qsl-backup/status/plan/rollback mutation;
- website, README, START_HERE, public docs, public paper, or durable Director State Index mutation.

## Required startup proof

Run and record:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py public-safety-status --sha "$(git rev-parse origin/main)"
```

Required:

- `READY_COUNT 1`;
- READY item is NA-0480;
- latest decision before edit is D-0947;
- D-0946 exists once;
- D-0947 exists once;
- D-0948 absent before edit;
- duplicate decision count is 0;
- current-main public-safety is green.

## Required scope review

Review read-only:

- `docs/governance/evidence/NA-0479_qsl_qsc_refimpl_kem_signature_binding_mapping_authorization_plan.md`
- `tests/NA-0479_qsl_qsc_refimpl_kem_signature_binding_mapping_authorization_testplan.md`
- `tools/refimpl/quantumshield_refimpl/src/crypto/traits.rs`
- `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs`
- `tools/refimpl/quantumshield_refimpl/tests/pqkem768.rs`
- qsc handshake/identity source and relevant qsc tests
- formal model files
- D-0946 and D-0947

Required evidence sections:

- Executive summary;
- Live NA-0480 scope;
- qwork proof-file verification;
- NA-0479 inheritance;
- Applicable Stewardship and Assurance Review;
- Refimpl provider-boundary candidate inventory;
- KEM provider-boundary test scope review;
- Signature provider-boundary test scope review;
- Combined vs split test-scope decision;
- Authorization decision;
- Future scope bundle;
- Future validation / marker plan;
- Public claim / external review / website boundary;
- Rejected alternatives;
- Backup-impact statement;
- Next recommendation.

## Validation commands

Run:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main \
  --allowed docs/governance/evidence/NA-0480_qsl_refimpl_kem_signature_provider_boundary_test_scope_authorization_plan.md \
  --allowed tests/NA-0480_qsl_refimpl_kem_signature_provider_boundary_test_scope_authorization_testplan.md \
  --allowed DECISIONS.md \
  --allowed TRACEABILITY.md \
  --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
bash scripts/ci/classify_ci_scope.sh $(git diff --name-only origin/main)
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions --select D-0946 --select D-0947 --select D-0948 --select D-0949
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

If feasible, run the qsc adversarial script locally. If local cargo-fuzz is unavailable, record exact output and rely on PR CI qsc-adversarial-smoke.

## Future selected NA-0481 validation markers

Selected successor markers:

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

## PR body requirements

The PR body must include:

```md
Goals: G1, G2, G3, G4, G5
Impact:
No-regression:
Tests/Vectors:
```

The body must state:

- refimpl KEM/signature provider-boundary test-scope authorization;
- selected signature-only NA-0481 successor;
- no implementation mutation;
- no runtime/crypto/dependency/Cargo/lockfile/workflow/test/fuzz/vector/formal/refimpl mutation;
- no public overclaim;
- no external-review-complete claim;
- no KEM/signature/qsc-refimpl-equivalence/provider-boundary/formal-proof-complete claim.

Validate with:

```bash
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight --file "$PR_BODY_FILE"
python3 tools/goal_lint.py
```

## Acceptance criteria

- D-0948 exists once after edit.
- D-0949 absent before optional closeout.
- Duplicate decision count is 0.
- Changed paths are limited to the five allowed NA-0480 paths.
- KEM provider-boundary scope is reviewed and classified.
- Signature provider-boundary scope is reviewed and classified.
- Combined versus split decision is recorded.
- Exact NA-0481 successor is selected.
- Root cargo audit is green.
- Nested qsc fuzz lock audit is green.
- Formal model and formal runner are green.
- Inherited qsc tests are green.
- Refimpl `pqkem768` is green.
- qsc adversarial script syntax is green.
- No public overclaim is introduced.
- No backup or restore is run.
- Exactly one READY item remains before optional closeout.
