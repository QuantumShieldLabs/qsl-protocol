Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-18

# NA-0496 QSL Binding Negative Vector Consumer Test Scope Authorization Testplan

## Purpose

This testplan validates the NA-0496 authorization-only evidence lane. It proves that the lane selected future vector-consumer scope without implementing it and without mutating qsc, corpus, vectors, inputs, scripts, workflows, dependencies, formal models, refimpl, services, public docs, backup paths, qwork, qstart, or qresume.

## Required Startup Proof

- qwork proof files exist and are copied into the directive proof root.
- `.kv` and `.json` proof values match required NA-0496 startup values.
- proof HEAD and proof `origin/main` match live refs before fetch.
- fetch occurs only after proof/live ref match.
- refreshed `origin/main` equals or descends from `1170a9189707`.
- READY count is 1.
- READY item is NA-0496.
- NA-0495, NA-0494, and NA-0493 are DONE.
- D-0979 exists once.
- D-0980 exists once.
- D-0981 is absent before patch and exists once after patch.
- duplicate decision record count is zero.
- startup `public-safety` is green.
- startup qsc-adversarial-smoke evidence is green through the public-safety helper output.
- `/` usage is below 95%.
- qsl-backup digest and source-list count match the directive requirement.

## Required Inventory Proof

- internal manifest JSON validates with `python3 -m json.tool`.
- manifest schema keys are recorded.
- manifest sections are recorded.
- total actual vector count is 34.
- qsc-frame vector count is 21.
- refimpl signature-provider-boundary vector count is 6.
- formal token-mapping vector count is 7.
- groups/layers/categories are recorded.
- expected reject/no-success/no-mutation expectations are recorded.
- vector IDs appearing directly in qsc tests, qsc fuzz target, corpus, formal model, and refimpl surfaces are recorded.
- vector IDs not directly consumed by executable tests are recorded.
- manifest metadata-only and no-secret-material policy are recorded.
- qsc crate `serde_json` availability is recorded.
- future no-Cargo/no-dependency/no-qsc-source scope is recorded.

## Authorization Markers

The evidence doc and D-0981 must establish:

- `BINDING_NEGATIVE_VECTOR_SCHEMA_MAPPING_TEST_READY`
- `VECTOR_MANIFEST_CONSUMER_SURFACE_READY`
- `VECTOR_MANIFEST_METADATA_ONLY`
- `VECTOR_MANIFEST_CONSUMER_NEEDS_SPLIT`
- `VECTOR_CONSUMER_SCHEMA_MAPPING_READY`
- `VECTOR_CONSUMER_QSC_FRAME_MAPPING_READY`
- `VECTOR_CONSUMER_ALL_SECTIONS_READY`
- `VECTOR_CONSUMER_SELECTED_SUBSET_READY`
- `VECTOR_CONSUMER_NO_DEPENDENCY_READY`
- `VECTOR_CONSUMER_QSC_TEST_SCOPE_READY`
- `VECTOR_CONSUMER_SCOPE_SPLIT_NEEDED`

## Selected Successor Proof

Selected successor:

`NA-0497 -- QSL Binding Negative Vector Consumer Test Implementation Harness`

Future allowed paths:

- `qsl/qsl-client/qsc/tests/binding_negative_vector_consumer.rs`
- `docs/governance/evidence/NA-0497_qsl_binding_negative_vector_consumer_test_implementation_harness.md`
- `tests/NA-0497_qsl_binding_negative_vector_consumer_test_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future forbidden paths unless a later exact directive authorizes them:

- qsc source
- qsc fuzz target
- qsc fuzz Cargo metadata
- qsc fuzz lockfile
- root Cargo metadata
- dependencies and lockfiles
- corpus, vectors, and inputs
- workflows, scripts, and helpers
- formal models
- refimpl
- services
- public docs
- backup paths and backup tooling
- qwork, qstart, and qresume

## Local Validation Commands

Run and capture output under the proof root:

```bash
git diff --check
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus
python3 -m json.tool inputs/suite2/internal_negative_binding_vectors/qsl_binding_negative_vector_manifest_v1.json >/dev/null
python3 formal/model_qsc_kem_signature_transcript_binding_bounded.py
python3 formal/run_model_checks.py
cargo test -p qsc --locked --test kem_signature_transcript_binding_negative -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_binding_fuzz_helper' cargo test -p qsc --locked --test kem_signature_transcript_binding_negative -- --test-threads=1 --nocapture
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test signature_provider_boundary -- --test-threads=1 --nocapture
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test pqkem768
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo fmt --check
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
```

Additional required proof:

- exact five-path NA-0496 scope guard
- link-check
- leak-scan
- added-line overclaim scan
- classifier
- PR body preflight
- goal-lint
- READY count remains 1
- READY remains NA-0496 before optional closeout
- D-0982 remains absent before optional closeout

## Scope Guard Expectations

Changed paths for NA-0496 evidence PR must be exactly:

- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/governance/evidence/NA-0496_qsl_binding_negative_vector_consumer_test_scope_authorization_plan.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0496_qsl_binding_negative_vector_consumer_test_scope_authorization_testplan.md`

Forbidden mutation proof must show:

- no implementation mutation
- no qsc source/test/fuzz/Cargo mutation
- no corpus/vector/input mutation
- no workflow/script/helper mutation
- no dependency/lockfile mutation
- no formal/refimpl/service/public/backup mutation
- no qwork/qstart/qresume mutation

## Claim Boundary Expectations

The evidence, decision, traceability, journal, PR body, and final response must preserve:

- no public-readiness claim
- no production-readiness claim
- no public-internet-readiness claim
- no public/conformance vector claim
- no interoperability-vector claim
- no external-review-complete claim
- no crypto-complete claim
- no fuzz-complete claim
- no corpus-complete claim
- no vector-complete claim
- no replay-proof claim
- no downgrade-proof claim
- no side-channel-free claim
- no vulnerability-free claim
- no bug-free claim
- no perfect-crypto claim

Cargo audit green remains dependency-health evidence only.

## Post-Fix Hardening Review Checklist

After patch and validation, report:

- Correctness under stress: manifest mapping fails closed on missing schema/mapping/caveat requirements in future NA-0497.
- Minimality: NA-0496 changed only five allowed governance/testplan paths and performed no implementation mutation.
- Maintainability: selected a new integration test instead of overloading the existing dynamic negative handshake test.
- Coverage quality: future NA-0497 validates all 34 metadata records but avoids dynamic vector-complete overclaim.
- Cross-lane stability: no macOS/Linux workflow, dependency, Cargo, qsc source, qsc fuzz, corpus, formal, refimpl, service, public, or backup drift.
