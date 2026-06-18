Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-18

# NA-0497 QSL Binding Negative Vector Consumer Test Implementation Harness

## Executive Summary

NA-0497 consumes NA-0496 / D372 and implements the selected qsc integration test:

`qsl/qsl-client/qsc/tests/binding_negative_vector_consumer.rs`

The test parses the internal negative binding vector manifest with existing qsc dependencies and validates schema shape, layer/group coverage, all 34 vector records, qsc-frame mapping, refimpl/formal supporting-only boundaries, no-secret material policy, and public-claim caveats.

This is a manifest schema and mapping consumer only. It does not execute dynamic qsc handshakes from manifest data.

Claim boundaries:

- no public/conformance vector claim
- no public-readiness claim
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

## Live NA-0497 Scope

Allowed implementation path:

- `qsl/qsl-client/qsc/tests/binding_negative_vector_consumer.rs`

Allowed governance paths:

- `docs/governance/evidence/NA-0497_qsl_binding_negative_vector_consumer_test_implementation_harness.md`
- `tests/NA-0497_qsl_binding_negative_vector_consumer_test_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

No qsc source, qsc fuzz target, qsc fuzz Cargo, qsc fuzz lockfile, root Cargo metadata, root lockfile, dependency, corpus, vector, input, manifest, validator, qsc-adversarial script, workflow, helper, formal, refimpl, service, public, backup, qsl-backup, nightly/local-ops, qwork, qstart, or qresume path is in mutation scope.

## qwork Proof-File Verification

Codex read and copied qwork proof files without rerunning qwork:

- `/srv/qbuild/work/NA-0497/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0497/.qwork/startup.qsl-protocol.json`

Proof root:

- `/srv/qbuild/tmp/NA0497_binding_negative_vector_consumer_impl_20260618T193949Z`

Startup proof:

- `startup_result=OK`
- `lane=NA-0497`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0497/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0497`
- `requested_lane_status=READY`

Freshness:

- proof HEAD matched live HEAD before fetch: `c4f6e3bb8f30`
- proof `origin/main` matched live `origin/main` before fetch: `c4f6e3bb8f30`
- fetch occurred only after proof/live ref match
- `origin/main` equals or descends from `c4f6e3bb8f30`

Startup disk status:

- `/` was below the 95% stop threshold.
- `/backup/qsl` was checked read-only.
- qsl-backup helper hash matched the expected digest.
- the Codex ops source-list inclusion count was exactly 1.

Recovered startup parser issue:

- failing command: initial queue/decision parser counted broad decision references and then used a heading grammar not used by `DECISIONS.md`
- classification: recoverable command/proof-shape issue; no repo file had been changed
- corrective action: reran with the actual queue and `- **ID:** D-####` decision-entry grammar
- final result: READY_COUNT 1, READY NA-0497, NA-0494/NA-0495/NA-0496 DONE, D-0981 once, D-0982 once, D-0983 absent, duplicate decision count zero

## NA-0496 / D372 Inheritance

Consumed inherited state:

- NA-0496 is closed.
- NA-0497 is restored READY.
- D-0981 records NA-0496 scope authorization.
- D-0982 records NA-0496 closeout and NA-0497 restoration.
- D372 response exists at `/home/victor/work/qsl/codex/responses/NA0496_closeout_restore_na0497_20260618T191907Z_D372.md`.
- D372 selected future test path `qsl/qsl-client/qsc/tests/binding_negative_vector_consumer.rs`.
- D372 selected semantics: manifest schema/category/layer/mapping coverage.
- no dynamic crypto execution claim is inherited.
- qsc-frame vectors are the qsc-facing subset.
- refimpl and formal sections are supporting-only in the first consumer.
- no public/conformance vector claim is inherited.
- no vector-complete claim is inherited.
- no replay-proof claim is inherited.
- no downgrade-proof claim is inherited.
- disk recovery sequence D369/D370/D371 was consumed.
- startup disk status remained below the stop threshold.
- CI wait-waste policy going forward: use short post-merge attach/early-failure polling for implementation directives; reserve long public-safety waits for explicit closeout/evidence directives.

Level-1 stewardship and D328 assurance review were applied:

- Best-Known-Method Review: a qsc integration test is the selected executable consumer for schema/mapping evidence.
- Hostile Cryptographer Review: metadata parsing is kept separate from dynamic crypto execution.
- Red-Team Review: missing schema, mapping, caveat, or no-secret metadata fails closed.
- Production SRE Review: no dependency, workflow, or Cargo churn is introduced.
- Side-Channel Caveat: no side-channel-free claim is made.
- Formal-Model Mapping Residual: formal-token vectors remain supporting-only.
- External-Review Readiness: this is internal traceability evidence, not external-review-complete evidence.
- Release-Claim Boundary: the manifest remains internal-only and not public vectors.
- Assurance Gap Review Trigger: dynamic qsc/refimpl/formal execution or public claims require a later exact lane.

## Pre-Mutation Review / Manifest Inventory

Selected test file state before mutation:

- `qsl/qsl-client/qsc/tests/binding_negative_vector_consumer.rs` was absent.

Manifest path:

- `inputs/suite2/internal_negative_binding_vectors/qsl_binding_negative_vector_manifest_v1.json`

Manifest schema:

- `schema_version`: `1`
- `status`: `internal-negative-evidence-only`
- top-level keys: `metadata`, `public_claim_boundary`, `schema_version`, `secret_material_policy`, `sections`, `status`, `suite`, `title`, `traceability`, `vectors`
- section count: 3
- vector count: 34

Layer counts:

- `qsc_frame`: 21
- `refimpl_signature_provider_boundary`: 6
- `formal_token_mapping`: 7

Group counts:

- `kem_binding`: 4
- `signature_binding`: 5
- `transcript_replay_suite`: 8
- `stale_identity_rollback`: 4
- `refimpl_signature_provider_boundary`: 6
- `formal_token_mapping`: 7

Expected-result metadata:

- all 34 vectors record `reject=true`
- all 34 vectors record `no_success_output=true`
- all 34 vectors record `no_completed_session_mutation`
- all 34 vectors record an expected reject class or expected error class
- the 6 refimpl provider-boundary vectors keep no-session-mutation as not relevant and use provider return-shape metadata

Secret-material policy:

- all 34 vectors record `contains_secret_material=false`
- all 34 vectors record `contains_private_key=false`
- all 34 vectors record `contains_passphrase=false`
- all 34 vectors record `contains_user_data=false`
- manifest-level policy forbids private keys, KEM secret keys, signing keys, passphrases, runtime keys, backup keys, operator data, user data, live service data, and private production endpoint data

Public/conformance boundary:

- `internal_only=true`
- `not_completion_evidence=true`
- `not_conformance_vectors=true`
- `not_interoperability_vectors=true`
- `not_public_vectors=true`

## qsc Integration Test Implementation

Implemented:

- `qsl/qsl-client/qsc/tests/binding_negative_vector_consumer.rs`

Implementation properties:

- uses existing qsc `serde_json` dependency
- reads the manifest from `env!("CARGO_MANIFEST_DIR")` plus `../../../inputs/suite2/internal_negative_binding_vectors/qsl_binding_negative_vector_manifest_v1.json`
- does not mutate the manifest
- does not generate files
- does not read runtime secrets
- does not use network
- does not execute dynamic handshakes from manifest data
- emits required NA-0497 markers under `--nocapture`

## Manifest Schema / Count Proof

Local command:

`cargo test -p qsc --locked --test binding_negative_vector_consumer -- --test-threads=1 --nocapture`

Initial result:

- PASS
- 4 tests passed
- marker emitted: `NA0497_VECTOR_MANIFEST_SCHEMA_OK`

The schema/count test asserts:

- exact top-level key set
- schema version `1`
- internal-only status
- section IDs and layers
- 34 total vectors
- 21 qsc-frame vectors
- 6 refimpl signature-provider-boundary vectors
- 7 formal token-mapping vectors
- exact vector ID sets by layer
- exact group counts
- unique vector IDs
- required ID/title/description/layer/group/input/mutation/result/material/safe-public/source/marker/caveat/status metadata

## Category / Layer Mapping Proof

The manifest uses the actual schema field `group` for category-like grouping. NA-0497 validates layer and group coverage without inventing a non-existent `category` field.

Validated qsc-facing groups:

- `kem_binding`
- `signature_binding`
- `transcript_replay_suite`
- `stale_identity_rollback`

Validated supporting groups:

- `refimpl_signature_provider_boundary`
- `formal_token_mapping`

Marker emitted:

- `NA0497_VECTOR_CATEGORY_COVERAGE_OK`

## qsc-Frame Mapping Proof

The qsc-frame vector set is asserted exactly:

- `kem_wrong_peer_public_key`
- `kem_stale_public_record`
- `kem_wrong_ciphertext`
- `kem_wrong_key_ciphertext_pair`
- `signature_wrong_identity_public_record`
- `signature_cross_message_replay_b1_as_a2`
- `signature_wrong_message_context`
- `signature_tampered_signature`
- `signature_wrong_public_key`
- `transcript_mutation`
- `transcript_truncation`
- `replayed_a1`
- `replayed_b1`
- `replayed_a2`
- `wrong_role_replay`
- `suite_confusion_wrong_suite_token`
- `downgrade_wrong_suite_block`
- `stale_public_record_replay`
- `public_record_rollback`
- `identity_rotation_stale_peer_state`
- `stale_trusted_pin_mismatch`

Markers emitted:

- `NA0497_VECTOR_MAPPING_TRACEABILITY_OK`
- `NA0497_QSC_FRAME_VECTORS_MAPPED_OK`

## Refimpl Supporting-Only Proof

The refimpl signature-provider-boundary vector set is asserted exactly:

- `refimpl_signature_wrong_public_key_length`
- `refimpl_signature_wrong_signature_length`
- `refimpl_signature_malformed_signing_key`
- `refimpl_signature_tampered_signature_invalid`
- `refimpl_signature_wrong_public_key_invalid`
- `refimpl_signature_err_vs_false_classification`

The test asserts these vectors remain provider-boundary metadata only.

Marker emitted:

- `NA0497_REFIMPL_VECTORS_SUPPORTING_ONLY_OK`

## Formal Token Supporting-Only Proof

The formal token-mapping vector set is asserted exactly:

- `formal_wrong_kem_token`
- `formal_wrong_signature_token`
- `formal_transcript_mutation`
- `formal_replay`
- `formal_suite_confusion`
- `formal_stale_public_record`
- `formal_no_session_mutation_on_reject`

The test asserts these vectors remain formal-token mapping only.

Marker emitted:

- `NA0497_FORMAL_TOKEN_VECTORS_SUPPORTING_ONLY_OK`

## No Secret-Material Policy Proof

The test asserts manifest-level forbidden material policy and per-vector false flags for:

- checked-in secret material
- private keys
- passphrases
- user data

The test also scans serialized manifest text for obvious private-key and token markers such as `-----BEGIN`, `PRIVATE KEY-----`, `OPENSSH PRIVATE KEY`, `BEGIN PGP PRIVATE KEY`, `AKIA`, `ghp_`, and `xoxb-`.

Marker emitted:

- `NA0497_NO_SECRET_MATERIAL_POLICY_OK`

Validator proof:

- binding corpus validator JSON: PASS, 7 files, 56 bytes, 0 findings
- all qsc fuzz corpus validator JSON: PASS, 17 files, 1238 bytes, 0 findings

## No Public / Conformance Vector Claim Proof

The test asserts manifest-level claim-boundary booleans:

- `internal_only=true`
- `not_completion_evidence=true`
- `not_conformance_vectors=true`
- `not_interoperability_vectors=true`
- `not_public_vectors=true`

The test asserts every vector carries a public-claim caveat and rejects completion overclaim.

Markers emitted:

- `NA0497_NO_PUBLIC_CONFORMANCE_VECTOR_CLAIM_OK`
- `NA0497_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0497_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0497_NO_FUZZ_COMPLETE_CLAIM_OK`
- `NA0497_NO_CORPUS_COMPLETE_CLAIM_OK`
- `NA0497_NO_VECTOR_COMPLETE_CLAIM_OK`
- `NA0497_NO_REPLAY_PROOF_CLAIM_OK`
- `NA0497_NO_DOWNGRADE_PROOF_CLAIM_OK`

No public-readiness claim is made. No production-readiness claim is made. No public-internet-readiness claim is made. No public/conformance vector claim is made. No external-review-complete claim is made. No crypto-complete claim is made. No fuzz-complete claim is made. No corpus-complete claim is made. No vector-complete claim is made. No replay-proof claim is made. No downgrade-proof claim is made. No side-channel-free claim is made. No vulnerability-free claim is made. No bug-free claim is made. No perfect-crypto claim is made.

## No Vector-Complete / Replay-Proof / Downgrade-Proof Claim Proof

The manifest consumer validates all 34 metadata records, but it does not claim all negative behavior is dynamically executed.

Boundaries preserved:

- no vector-complete claim
- no replay-proof claim
- no downgrade-proof claim
- no public/conformance vector claim
- no dynamic crypto execution claim

## No qsc Source / Cargo / Dependency / Corpus Mutation Proof

Changed paths at implementation evidence authoring are limited to:

- `qsl/qsl-client/qsc/tests/binding_negative_vector_consumer.rs`
- `docs/governance/evidence/NA-0497_qsl_binding_negative_vector_consumer_test_implementation_harness.md`
- `tests/NA-0497_qsl_binding_negative_vector_consumer_test_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

No qsc source mutation is introduced. No qsc fuzz target/Cargo mutation is introduced. No corpus/vector/input mutation is introduced. No workflow/script/helper mutation is introduced. No dependency/lockfile mutation is introduced. No formal/refimpl/service/public/backup mutation is introduced.

## Validation

Completed before governance patch:

- `cargo test -p qsc --locked --test binding_negative_vector_consumer -- --test-threads=1 --nocapture`: PASS
- `python3 -m json.tool inputs/suite2/internal_negative_binding_vectors/qsl_binding_negative_vector_manifest_v1.json >/dev/null`: PASS
- `python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics`: PASS
- `python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus`: PASS

Full inherited validation is required before PR creation and merge:

- qsc binding negative tests with and without `qsc_binding_fuzz_helper`
- formal model checks
- refimpl signature-provider-boundary and `pqkem768`
- root and nested cargo audit
- cargo fmt
- qsc-adversarial shell syntax
- scope, link, leak, overclaim, classifier, PR body, and goal-lint checks

## Scope Guard

NA-0497 implementation scope is exact. This lane does not mutate:

- qsc source
- qsc fuzz target
- qsc fuzz Cargo or lockfile
- root Cargo metadata or lockfile
- dependencies
- corpus/vector/input files
- internal negative vector manifest
- validator script
- qsc-adversarial script
- workflows
- formal models
- refimpl
- qsl-server
- qsl-attachments
- qshield or qshield-cli
- public docs, README, START_HERE, website
- backup or qsl-backup
- qwork, qstart, qresume, qshell
- archived paths, rollback paths, or `/backup/qsl`

## Public Claim Boundary

This evidence is internal assurance evidence only. It is not public vectors, not conformance vectors, not interoperability vectors, not completion evidence, and not release-readiness evidence.

Cargo audit green, when run, remains dependency-health evidence only.

Formal evidence remains bounded/supporting evidence only.

qsc/refimpl mapping remains bounded/supporting evidence only unless exact future scope expands it.

## Successor Selection

Default successor after successful NA-0497:

`NA-0498 -- QSL Core Assurance Checkpoint and Next Highest-Risk Security Lane Authorization Plan`

Rationale:

- NA-0497 completes the deterministic manifest-consumer test selected by NA-0496.
- The project should return to core assurance prioritization.
- NA-0498 should select the next highest-value security or assurance lane based on real risk reduction.

No NA-0498 implementation is performed by NA-0497.

## Next Recommendation

Merge NA-0497 only after local validation and required PR checks pass. After merge, use short public-safety attach/early-failure polling. If public-safety is attached and green quickly, a separate closeout may restore NA-0498. If public-safety remains healthy but running after the short window, stop cleanly and hand off closeout.
