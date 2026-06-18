Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-18

# NA-0497 QSL Binding Negative Vector Consumer Test Implementation Testplan

## Objective

Validate the NA-0497 qsc integration test that consumes the internal negative binding vector manifest as schema and mapping evidence only.

## Protected Invariants

- exactly one READY item remains mandatory
- qsc-frame vectors are the qsc-facing subset
- refimpl signature-provider-boundary vectors remain supporting-only
- formal token-mapping vectors remain supporting-only
- no dynamic crypto execution claim is made from manifest metadata
- no public/conformance vector claim is made
- no public-readiness claim is made
- no crypto-complete claim is made
- no fuzz-complete claim is made
- no corpus-complete claim is made
- no vector-complete claim is made
- no replay-proof claim is made
- no downgrade-proof claim is made
- no side-channel-free claim is made
- no vulnerability-free claim is made
- no bug-free claim is made
- no perfect-crypto claim is made

## Allowed Scope

- `qsl/qsl-client/qsc/tests/binding_negative_vector_consumer.rs`
- `docs/governance/evidence/NA-0497_qsl_binding_negative_vector_consumer_test_implementation_harness.md`
- `tests/NA-0497_qsl_binding_negative_vector_consumer_test_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

- qsc source mutation
- existing qsc test mutation outside the new test file
- qsc fuzz target mutation
- qsc fuzz Cargo or lockfile mutation
- root Cargo or lockfile mutation
- dependency mutation
- corpus/vector/input/manifest mutation
- validator or qsc-adversarial script mutation
- workflow/helper mutation
- formal/refimpl/service/public/backup mutation
- qwork/qstart/qresume mutation

## Manifest Schema / Count Test

Command:

```bash
cargo test -p qsc --locked --test binding_negative_vector_consumer -- --test-threads=1 --nocapture
```

Required assertions:

- manifest JSON parses
- top-level schema keys match expected v1 shape
- `schema_version` is `1`
- `status` is `internal-negative-evidence-only`
- total vector count is 34
- qsc-frame vector count is 21
- refimpl signature-provider-boundary vector count is 6
- formal token-mapping vector count is 7
- vector IDs are unique
- every vector has ID, title, description, layer, group, input kind, mutation kind, expected result, material policy, safe-public material, source evidence, related markers, public-claim caveat, and validation status
- marker `NA0497_VECTOR_MANIFEST_SCHEMA_OK` appears

## Mapping / Traceability Test

Required assertions:

- qsc-frame vectors map to qsc-facing evidence groups
- refimpl vectors remain supporting-only provider-boundary metadata
- formal vectors remain supporting-only formal-token metadata
- every vector carries source evidence and related marker metadata
- every vector carries a public-claim caveat
- no vector caveat claims public/conformance/interoperability vector status
- markers `NA0497_VECTOR_CATEGORY_COVERAGE_OK`, `NA0497_VECTOR_MAPPING_TRACEABILITY_OK`, `NA0497_REFIMPL_VECTORS_SUPPORTING_ONLY_OK`, `NA0497_FORMAL_TOKEN_VECTORS_SUPPORTING_ONLY_OK`, and `NA0497_NO_PUBLIC_CONFORMANCE_VECTOR_CLAIM_OK` appear

## No-Secret Policy Test

Required assertions:

- manifest-level secret-material policy exists
- forbidden material types include private keys, KEM secret keys, signing keys, passphrases, runtime keys, backup keys, operator data, user data, live service data, and private production endpoint data
- every vector has `contains_secret_material=false`
- every vector has `contains_private_key=false`
- every vector has `contains_passphrase=false`
- every vector has `contains_user_data=false`
- serialized manifest text lacks obvious private-key and token markers
- marker `NA0497_NO_SECRET_MATERIAL_POLICY_OK` appears

## No-Overclaim Marker Test

Required markers:

- `NA0497_VECTOR_CONSUMER_SCOPE_CONSUMED_OK`
- `NA0497_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0497_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0497_NO_FUZZ_COMPLETE_CLAIM_OK`
- `NA0497_NO_CORPUS_COMPLETE_CLAIM_OK`
- `NA0497_NO_VECTOR_COMPLETE_CLAIM_OK`
- `NA0497_NO_REPLAY_PROOF_CLAIM_OK`
- `NA0497_NO_DOWNGRADE_PROOF_CLAIM_OK`
- `NA0497_ONE_READY_INVARIANT_OK`
- `NA0497_QSC_VECTOR_CONSUMER_TEST_IMPLEMENTED_OK`
- `NA0497_QSC_FRAME_VECTORS_MAPPED_OK`
- `NA0497_NO_QSC_SOURCE_CHANGE_OK`
- `NA0497_NO_DEPENDENCY_CHANGE_OK`

## Cargo Test Command

Primary command:

```bash
cargo test -p qsc --locked --test binding_negative_vector_consumer -- --test-threads=1 --nocapture
```

Required result:

- PASS
- required markers appear in output

## Inherited Validator / Formal / qsc / Refimpl Tests

Required validation:

```bash
cargo test -p qsc --locked --test kem_signature_transcript_binding_negative -- --test-threads=1 --nocapture
RUSTFLAGS='--cfg qsc_binding_fuzz_helper' cargo test -p qsc --locked --test kem_signature_transcript_binding_negative -- --test-threads=1 --nocapture
python3 -m json.tool inputs/suite2/internal_negative_binding_vectors/qsl_binding_negative_vector_manifest_v1.json >/dev/null
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics
python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus
python3 formal/model_qsc_kem_signature_transcript_binding_bounded.py
python3 formal/run_model_checks.py
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test signature_provider_boundary -- --test-threads=1 --nocapture
cargo test -p quantumshield_refimpl --features pqcrypto --locked --test pqkem768
```

## Audit / Fmt Checks

Required validation:

```bash
git diff --check
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo fmt --check
sh -n scripts/ci/qsc_adversarial.sh
bash -n scripts/ci/qsc_adversarial.sh
```

Also required:

- exact allowed path scope guard
- link-check
- leak-scan
- added-line overclaim scan
- classifier
- PR body preflight
- goal-lint

## Public Claim Boundary

The PR body and evidence must preserve:

- no public/conformance vector claim
- no public-readiness claim
- no production-readiness claim
- no public-internet-readiness claim
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

## Closeout Prerequisites

Closeout to NA-0498 is allowed only after NA-0497 implementation PR merges and post-merge public-safety is green within the short attach/early-failure window.

If public-safety is healthy but still running after the short attach window, stop and hand off closeout.
