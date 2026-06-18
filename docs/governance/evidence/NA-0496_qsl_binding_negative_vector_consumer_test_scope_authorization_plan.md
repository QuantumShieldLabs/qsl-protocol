Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-18

# NA-0496 QSL Binding Negative Vector Consumer Test Scope Authorization Plan

## Executive Summary

NA-0496 is authorization-only. It consumes the NA-0495 / D367 state, inventories the internal negative binding vector manifest and existing test surfaces, and authorizes a narrow NA-0497 implementation lane:

`NA-0497 -- QSL Binding Negative Vector Consumer Test Implementation Harness`

Primary classification:

`BINDING_NEGATIVE_VECTOR_SCHEMA_MAPPING_TEST_READY`

Selected implementation surface:

`qsl/qsl-client/qsc/tests/binding_negative_vector_consumer.rs`

The future consumer should be a new qsc integration test. It should read and parse the internal manifest at test runtime using the qsc crate's existing `serde_json` dependency, validate schema, layer/group/category coverage, secret-material policy, public-claim caveats, and manifest-to-evidence mapping for all 34 manifest vectors. It must not generate direct qsc handshake assertions from manifest metadata in the first implementation. It must not claim that all 34 vectors are dynamically executed, conformance vectors, public vectors, replay-proof evidence, downgrade-proof evidence, crypto-complete evidence, fuzz-complete evidence, corpus-complete evidence, vector-complete evidence, side-channel-free evidence, vulnerability-free evidence, bug-free evidence, or perfect-crypto evidence.

Refimpl and formal sections may be consumed in the same manifest schema/mapping test only as supporting-only metadata. Dynamic refimpl/formal execution remains split from the first qsc consumer lane unless a later exact directive authorizes it.

## Live NA-0496 Scope

Allowed NA-0496 mutation paths:

- `docs/governance/evidence/NA-0496_qsl_binding_negative_vector_consumer_test_scope_authorization_plan.md`
- `tests/NA-0496_qsl_binding_negative_vector_consumer_test_scope_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

NA-0496 does not implement vector-consumer tests. It does not mutate qsc source, qsc tests, qsc fuzz target, qsc fuzz corpus, qsc fuzz Cargo metadata, root Cargo metadata, lockfiles, workflows, qsc-adversarial scripts, validator scripts, formal models, refimpl code, services, public docs, backup files, nightly/local-ops scripts, qwork tooling, qstart, or qresume.

## qwork Proof-File Verification

Codex read and copied the qwork startup proof files from:

- `/srv/qbuild/work/NA-0496/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0496/.qwork/startup.qsl-protocol.json`

Proof root:

- `/srv/qbuild/tmp/NA0496_binding_negative_vector_consumer_scope_20260618T150231Z`

Startup verification:

- `startup_result=OK`
- `lane=NA-0496`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0496/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0496`
- `requested_lane_status=READY`

Freshness:

- proof HEAD matched live HEAD before fetch: `1170a9189707`
- proof `origin/main` matched live `origin/main` before fetch: `1170a9189707`
- fetch occurred only after proof/live ref match
- refreshed `origin/main` equals or descends from `1170a9189707`

Queue and decision startup gates:

- READY count: 1
- READY item: NA-0496
- NA-0495: DONE
- NA-0494: DONE
- NA-0493: DONE
- D-0979: one decision record
- D-0980: one decision record
- D-0981: absent before NA-0496 patch
- duplicate decision record count: 0

Recovered startup parser issue:

- failing command: initial startup gate parser counted broad decision-ID references and expected JSON from `public-safety-status`
- classification: recoverable command/proof-shape issue; the helper output is text and decision records use `- **ID:**`
- corrective action: reran with text-aware public-safety parsing and decision-record anchored counts
- final result: startup gates passed with READY NA-0496, D-0981 absent, and public-safety success

## NA-0495 / D367 Inheritance

Consumed inherited state:

- D-0979 implemented qsc-adversarial validator integration.
- D-0980 closed NA-0495 and restored NA-0496.
- qsc-adversarial runs the binding fuzz corpus secret-material validator.
- qsc-adversarial scans `qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics`.
- qsc-adversarial scans `qsl/qsl-client/qsc/fuzz/corpus`.
- qsc-adversarial does not use `--allow-missing` for the checked-in binding corpus.
- The checked-in binding corpus exists with exactly seven seed files.
- Every checked-in binding corpus seed is 8 bytes.
- The validator passes the checked-in binding corpus.
- The validator passes the full qsc fuzz corpus.
- Startup `public-safety` on `origin/main` completed success.
- Startup `qsc-adversarial-smoke` on `origin/main` completed success.
- The internal negative binding vector manifest remains internal-only.
- NA-0496 was restored as authorization-only.
- No public-readiness claim is inherited.
- No crypto-complete claim is inherited.
- No fuzz-complete claim is inherited.
- No corpus-complete claim is inherited.
- No vector-complete claim is inherited.
- No replay-proof claim is inherited.
- No downgrade-proof claim is inherited.
- No side-channel-free claim is inherited.
- No vulnerability-free, bug-free, or perfect-crypto claim is inherited.

Startup disk status:

- `/` usage: 94%, below the 95% stop threshold.
- `/backup/qsl` checked read-only.
- qsl-backup installed helper hash matched the expected digest.
- `/home/victor/work/qsl/codex/ops` appears exactly once in the qsl-backup source list.

CI wait-waste policy consumed from D367:

- Implementation and authorization directives should use short post-merge attach/early-failure polling.
- Long 180-iteration polling belongs in explicit closeout/evidence directives unless the Director explicitly says otherwise.

## Applicable Stewardship and Assurance Review

Level-1 stewardship and D328 assurance requirements were applied as advisory review inputs. Lead Director remains final authority.

- Best-Known-Method Review: use a deterministic qsc integration test because it is closer to executable qsc evidence than documentation-only mapping and stronger than a standalone audit script.
- Hostile Cryptographer Review: do not let a metadata manifest imply dynamic cryptographic execution; the consumer must separate schema/mapping assertions from handshake assertions.
- Red-Team Review: fail closed if a new manifest vector lacks mapping metadata, claim caveats, or no-secret material policy.
- Production SRE Review: avoid workflow, dependency, and Cargo churn; run by existing qsc test invocation so CI cost and operational surface stay bounded.
- Side-Channel Caveat: this lane and the future consumer provide no side-channel-free claim.
- Formal-Model Mapping Residual: formal-token manifest vectors remain supporting-only in the first consumer and are not direct qsc runtime proof.
- External-Review Readiness: the manifest consumer may improve internal traceability, but it is not external-review-complete evidence.
- Release-Claim Boundary: internal negative metadata remains not public vectors, not conformance vectors, not interoperability vectors, and not public release evidence.
- Assurance Gap Review Trigger: if a future implementation wants dynamic qsc assertions, generated test cases, formal execution coupling, refimpl execution coupling, or public claims, stop and split scope.

## Current Vector / Test / Manifest Inventory

Manifest path:

- `inputs/suite2/internal_negative_binding_vectors/qsl_binding_negative_vector_manifest_v1.json`

Manifest schema keys:

- `schema_version`
- `suite`
- `status`
- `title`
- `metadata`
- `public_claim_boundary`
- `secret_material_policy`
- `traceability`
- `sections`
- `vectors`

Manifest section names:

- `qsc_binding`
- `refimpl_signature_provider_boundary`
- `formal_token_mapping`

Actual vector count:

- total vectors: 34
- qsc-frame vectors: 21
- refimpl signature-provider-boundary vectors: 6
- formal token-mapping vectors: 7

Vector IDs by layer:

- qsc-frame: `kem_wrong_peer_public_key`, `kem_stale_public_record`, `kem_wrong_ciphertext`, `kem_wrong_key_ciphertext_pair`, `signature_wrong_identity_public_record`, `signature_cross_message_replay_b1_as_a2`, `signature_wrong_message_context`, `signature_tampered_signature`, `signature_wrong_public_key`, `transcript_mutation`, `transcript_truncation`, `replayed_a1`, `replayed_b1`, `replayed_a2`, `wrong_role_replay`, `suite_confusion_wrong_suite_token`, `downgrade_wrong_suite_block`, `stale_public_record_replay`, `public_record_rollback`, `identity_rotation_stale_peer_state`, `stale_trusted_pin_mismatch`
- refimpl signature-provider-boundary: `refimpl_signature_wrong_public_key_length`, `refimpl_signature_wrong_signature_length`, `refimpl_signature_malformed_signing_key`, `refimpl_signature_tampered_signature_invalid`, `refimpl_signature_wrong_public_key_invalid`, `refimpl_signature_err_vs_false_classification`
- formal token-mapping: `formal_wrong_kem_token`, `formal_wrong_signature_token`, `formal_transcript_mutation`, `formal_replay`, `formal_suite_confusion`, `formal_stale_public_record`, `formal_no_session_mutation_on_reject`

Groups represented:

- `kem_binding`: 4
- `signature_binding`: 5
- `transcript_replay_suite`: 8
- `stale_identity_rollback`: 4
- `refimpl_signature_provider_boundary`: 6
- `formal_token_mapping`: 7

Expected reject and mutation expectations:

- all 34 vectors expect rejection and no success output
- 28 vectors expect no completed session mutation
- 6 refimpl provider-boundary vectors use provider-return-shape expectations where completed-session mutation is not the relevant assertion

Material policy:

- all 34 vectors record `contains_secret_material=false`
- all 34 vectors record `contains_private_key=false`
- all 34 vectors record `contains_passphrase=false`
- all 34 vectors record `contains_user_data=false`
- the manifest is metadata-only and the scan found zero hex32plus pattern occurrences

Direct vector ID occurrences:

- qsc binding negative test: `transcript_mutation`, `replayed_a2`
- qsc fuzz target: `kem_wrong_peer_public_key`, `kem_stale_public_record`, `kem_wrong_ciphertext`, `signature_wrong_identity_public_record`, `signature_cross_message_replay_b1_as_a2`, `transcript_mutation`, `suite_confusion_wrong_suite_token`, `stale_public_record_replay`
- qsc corpus comments/categories: no direct vector IDs in checked-in seed files
- formal model: `transcript_mutation`
- refimpl test surface: no direct manifest vector IDs

Marker-based evidence mapping exists for more vectors than direct ID occurrences, but direct manifest-to-executable coupling is currently sparse. Thirty-two manifest vector IDs are not directly consumed by executable tests as exact IDs. This is the residual that NA-0497 should reduce.

Existing qsc integration tests that could host consumer logic:

- `qsl/qsl-client/qsc/tests/kem_signature_transcript_binding_negative.rs`
- other existing qsc integration tests under `qsl/qsl-client/qsc/tests/`

Inventory classification:

- `VECTOR_MANIFEST_CONSUMER_SURFACE_READY`
- `VECTOR_MANIFEST_METADATA_ONLY`
- `VECTOR_MANIFEST_CONSUMER_NEEDS_SPLIT`

The split is semantic, not path-blocking: all sections can be schema/mapping-checked together, but qsc/refimpl/formal dynamic execution must remain split unless later authorized.

## Consumer Test Option Review

Option 1 -- new qsc integration test reads the manifest and validates schema/category/coverage mapping.

- selection: selected
- evidence: qsc already depends on `serde_json`; the test can parse the manifest without Cargo, lockfile, dependency, workflow, qsc source, qsc fuzz, corpus, vector, input, script, formal, or refimpl changes
- future allowed path: `qsl/qsl-client/qsc/tests/binding_negative_vector_consumer.rs`
- future forbidden paths: qsc source, qsc fuzz target, qsc fuzz Cargo, qsc fuzz lockfile, root Cargo, lockfiles, workflows, scripts, helpers, corpus, vectors, inputs, formal, refimpl, services, public docs, backup
- validation requirements: `cargo test -p qsc --locked --test binding_negative_vector_consumer -- --test-threads=1 --nocapture`, existing qsc binding tests, validator scans, manifest JSON validation, formal runner, audits, fmt
- public-claim caveat: no public/conformance vector claim and no vector-complete claim
- risks: P0 overclaim if mapped vectors are described as dynamically executed; P1 brittle path assumptions; P2 extra governance text

Option 2 -- extend `kem_signature_transcript_binding_negative.rs`.

- selection: rejected for NA-0497
- evidence: the existing file already owns dynamic negative handshake assertions; adding manifest schema/mapping logic would make it harder to maintain
- future allowed paths: none selected for first consumer
- future forbidden paths: same forbidden paths as Option 1 unless later authorized
- validation requirements: same qsc negative test validation if ever selected
- public-claim caveat: no replay-proof claim and no downgrade-proof claim
- risks: P0 conflating metadata mapping with dynamic handshake semantics; P1 maintainability regression; P2 longer test output

Option 3 -- standalone repo audit script.

- selection: rejected as primary
- evidence: a Python audit script would be simple, but it would be weaker than qsc integration test evidence for qsc manifest consumption
- future allowed paths: none selected
- future forbidden paths: no script/helper mutation in NA-0497 unless a later exact lane chooses audit-script scope
- validation requirements: script lint plus deterministic fixture tests if later selected
- public-claim caveat: no public/conformance vector claim
- risks: P0 lower executable qsc evidence; P1 another script surface; P2 duplicate parsing logic

Option 4 -- formal model consumer.

- selection: rejected as first consumer
- evidence: formal-token vectors are supporting-only and not direct qsc runtime evidence
- future allowed paths: none selected
- future forbidden paths: formal model mutation remains forbidden for NA-0497
- validation requirements: `python3 formal/model_qsc_kem_signature_transcript_binding_bounded.py` and `python3 formal/run_model_checks.py`
- public-claim caveat: no formal-model-complete or crypto-complete claim
- risks: P0 overclaiming formal support as runtime proof; P1 model/test coupling; P2 queue overhead

Option 5 -- documentation-only mapping.

- selection: rejected as primary
- evidence: documentation-only mapping underuses the manifest now that validator, corpus, fuzz target, and qsc-adversarial integration are in place
- future allowed paths: none selected
- future forbidden paths: executable path remains unimplemented if this is chosen
- validation requirements: link-check, leak-scan, overclaim scan, goal-lint
- public-claim caveat: no vector-complete claim
- risks: P0 traceability residual remains open; P1 stale docs; P2 low cost but low value

Option 6 -- split qsc/refimpl/formal manifest sections into separate lanes.

- selection: selected as a semantic boundary, rejected as first-lane queue split
- evidence: qsc/refimpl/formal dynamic execution should be split, but schema and mapping metadata can be validated together in the first qsc consumer test
- future allowed paths: first lane only adds the qsc integration test and governance; later lanes may be proposed separately
- future forbidden paths: formal/refimpl mutation forbidden in NA-0497
- validation requirements: qsc integration test plus existing formal/refimpl validations as supporting checks
- public-claim caveat: no all-section dynamic execution claim
- risks: P0 all-section overclaim; P1 future lane fragmentation; P2 queue overhead

Option 7 -- do not consume manifest; continue ephemeral/corpus/fuzz path.

- selection: rejected
- evidence: this leaves manifest-to-test traceability residual open after prerequisites are in place
- future allowed paths: none selected
- future forbidden paths: all implementation remains deferred
- validation requirements: existing qsc, validator, formal, refimpl, audit checks
- public-claim caveat: no vector-complete claim
- risks: P0 stale internal metadata; P1 traceability drift; P2 no immediate code risk

## Consumer Semantics / Claim Boundary Review

Selected future consumer semantics:

- read and parse the internal manifest at qsc integration-test runtime
- assert the schema keys and manifest status/classification
- assert section names and counts
- assert all 34 vectors have IDs, groups, layers, source evidence, material policy, expected reject result, validation status, public-claim caveat, and related marker metadata
- assert all 34 vectors have no checked-in secret material indicators
- assert all 34 vectors carry internal-only and no public/conformance/interoperability/completion claim caveats
- assert every vector ID is mapped to at least one evidence class
- assert qsc-frame vectors map to qsc-facing evidence classes where present
- keep refimpl and formal-token sections supporting-only in the first consumer
- fail if future manifest vectors lack mapping metadata or claim caveats
- run the existing corpus/manifest validator checks outside the consumer as validation evidence

Rejected future consumer semantics for first NA-0497 implementation:

- dynamic negative handshakes generated from manifest metadata
- qsc source mutation to expose additional internals
- qsc fuzz target mutation
- corpus/vector/input mutation
- workflow/script/helper mutation
- dependency/lockfile mutation
- formal/refimpl execution coupling
- no public/conformance vector publication

Consumer classifications:

- `VECTOR_CONSUMER_SCHEMA_MAPPING_READY`
- `VECTOR_CONSUMER_QSC_FRAME_MAPPING_READY`
- `VECTOR_CONSUMER_ALL_SECTIONS_READY` for metadata-only schema/mapping
- `VECTOR_CONSUMER_SELECTED_SUBSET_READY` for qsc-frame evidence classification

Rejected or deferred classifications:

- `VECTOR_CONSUMER_DYNAMIC_TESTS_NEEDED` is not required for first consumer
- `VECTOR_CONSUMER_PUBLIC_CLAIM_RISK_NEEDS_SPLIT` is mitigated by explicit internal-only/no-claim assertions
- `VECTOR_CONSUMER_AMBIGUOUS_STOP` is not selected

## CI / Dependency / Scope Impact Review

Future qsc consumer impact:

- new dependencies required: no
- Cargo manifest changes required: no
- lockfile changes required: no
- qsc source changes required: no
- workflow or script changes required: no
- qsc fuzz target changes required: no
- corpus/vector/input changes required: no
- formal/refimpl changes required: no
- can run under existing qsc test commands: yes
- intended CI path: existing qsc test invocation, without workflow changes
- local validation path: targeted qsc integration test plus existing qsc/formal/refimpl/validator/audit checks

Dependency/scope classifications:

- `VECTOR_CONSUMER_NO_DEPENDENCY_READY`
- `VECTOR_CONSUMER_QSC_TEST_SCOPE_READY`
- `VECTOR_CONSUMER_SCOPE_SPLIT_NEEDED` for dynamic refimpl/formal/qsc execution, not for schema/mapping

Rejected dependency/scope classifications:

- `VECTOR_CONSUMER_TEST_DEPENDENCY_RISK`
- `VECTOR_CONSUMER_CARGO_SCOPE_NEEDED`
- `VECTOR_CONSUMER_SCRIPT_SCOPE_READY`
- `VECTOR_CONSUMER_UNSAFE_OR_AMBIGUOUS_STOP`

## Authorization Decision

Primary classification:

`BINDING_NEGATIVE_VECTOR_SCHEMA_MAPPING_TEST_READY`

Selected successor:

`NA-0497 -- QSL Binding Negative Vector Consumer Test Implementation Harness`

Decision:

- consume NA-0495/D367 inherited validator, corpus, qsc-adversarial, public-safety, and qsc-adversarial-smoke state
- authorize a new qsc integration test rather than extending the existing dynamic negative handshake test
- parse the internal manifest at test runtime using existing qsc dependencies
- validate schema, sections, layer/group counts, all 34 vector metadata records, no-secret material policy, public claim caveats, and evidence mapping metadata
- treat refimpl and formal-token manifest sections as supporting-only in the first qsc consumer
- do not implement dynamic qsc handshakes from manifest metadata in NA-0497
- do not mutate qsc source, qsc fuzz target, qsc fuzz Cargo, qsc fuzz lockfile, root Cargo files, workflows, scripts, helpers, corpus, vectors, inputs, dependencies, lockfiles, formal, refimpl, services, public docs, backup, qwork, qstart, or qresume
- preserve exactly one READY successor

## Future Scope Bundle

Future successor:

`NA-0497 -- QSL Binding Negative Vector Consumer Test Implementation Harness`

Potential future allowed paths:

- `qsl/qsl-client/qsc/tests/binding_negative_vector_consumer.rs`
- `docs/governance/evidence/NA-0497_qsl_binding_negative_vector_consumer_test_implementation_harness.md`
- `tests/NA-0497_qsl_binding_negative_vector_consumer_test_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future forbidden paths unless later exact scope authorizes:

- qsc source mutation
- qsc fuzz target mutation
- qsc fuzz Cargo mutation
- qsc fuzz lockfile mutation
- root Cargo mutation
- dependency or lockfile mutation
- corpus/vector/input mutation
- workflow/script/helper mutation
- formal/refimpl/service/public/backup mutation
- qwork/qstart/qresume mutation
- public claim expansion

Future exact implementation requirements:

- create a new qsc integration test file, not an extension of `kem_signature_transcript_binding_negative.rs`
- read the manifest from `inputs/suite2/internal_negative_binding_vectors/qsl_binding_negative_vector_manifest_v1.json`
- use existing qsc crate dependencies only
- fail closed on malformed JSON, missing keys, wrong counts, missing mapping metadata, missing caveats, or secret-material policy drift
- assert no public/conformance vector claim in the manifest
- assert no public-readiness claim, no crypto-complete claim, no fuzz-complete claim, no corpus-complete claim, no vector-complete claim, no replay-proof claim, no downgrade-proof claim, no side-channel-free claim, no vulnerability-free claim, no bug-free claim, and no perfect-crypto claim

## Future Validation / Marker Plan

Common future markers:

- `NA0497_VECTOR_CONSUMER_SCOPE_CONSUMED_OK`
- `NA0497_VECTOR_MANIFEST_SCHEMA_OK`
- `NA0497_VECTOR_CATEGORY_COVERAGE_OK`
- `NA0497_VECTOR_MAPPING_TRACEABILITY_OK`
- `NA0497_NO_PUBLIC_CONFORMANCE_VECTOR_CLAIM_OK`
- `NA0497_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0497_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0497_NO_FUZZ_COMPLETE_CLAIM_OK`
- `NA0497_NO_CORPUS_COMPLETE_CLAIM_OK`
- `NA0497_NO_VECTOR_COMPLETE_CLAIM_OK`
- `NA0497_NO_REPLAY_PROOF_CLAIM_OK`
- `NA0497_NO_DOWNGRADE_PROOF_CLAIM_OK`
- `NA0497_ONE_READY_INVARIANT_OK`

Qsc integration test future markers:

- `NA0497_QSC_VECTOR_CONSUMER_TEST_IMPLEMENTED_OK`
- `NA0497_QSC_FRAME_VECTORS_MAPPED_OK`
- `NA0497_REFIMPL_VECTORS_SUPPORTING_ONLY_OK`
- `NA0497_FORMAL_TOKEN_VECTORS_SUPPORTING_ONLY_OK`
- `NA0497_NO_QSC_SOURCE_CHANGE_OK`
- `NA0497_NO_DEPENDENCY_CHANGE_OK`

Required future validation commands:

- `git diff --check`
- exact NA-0497 scope guard
- link-check
- leak-scan
- overclaim scan
- classifier
- PR body preflight
- goal-lint
- `python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics`
- `python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus`
- `python3 -m json.tool inputs/suite2/internal_negative_binding_vectors/qsl_binding_negative_vector_manifest_v1.json >/dev/null`
- `python3 formal/model_qsc_kem_signature_transcript_binding_bounded.py`
- `python3 formal/run_model_checks.py`
- `cargo test -p qsc --locked --test binding_negative_vector_consumer -- --test-threads=1 --nocapture`
- `cargo test -p qsc --locked --test kem_signature_transcript_binding_negative -- --test-threads=1 --nocapture`
- `RUSTFLAGS='--cfg qsc_binding_fuzz_helper' cargo test -p qsc --locked --test kem_signature_transcript_binding_negative -- --test-threads=1 --nocapture`
- `cargo test -p quantumshield_refimpl --features pqcrypto --locked --test signature_provider_boundary -- --test-threads=1 --nocapture`
- `cargo test -p quantumshield_refimpl --features pqcrypto --locked --test pqkem768`
- `cargo audit --deny warnings`
- `cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock`
- `cargo fmt --check`
- `sh -n scripts/ci/qsc_adversarial.sh`
- `bash -n scripts/ci/qsc_adversarial.sh`

## Public Claim / External Review / Website Boundary

This authorization is internal governance evidence only.

- no public-readiness claim is made
- no production-readiness claim is made
- no public-internet-readiness claim is made
- no public/conformance vector claim is made
- no interoperability-vector claim is made
- no external-review-complete claim is made
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

Cargo audit green remains dependency-health evidence only.

## Rejected Alternatives

Rejected as primary:

- extending `kem_signature_transcript_binding_negative.rs`
- standalone audit script
- formal-model consumer
- documentation-only mapping
- full section split before first schema/mapping consumer
- no-action/defer
- manifest-generated dynamic qsc assertions

Rationale:

- a new qsc integration test has the best balance of deterministic executable evidence, no dependency impact, no workflow impact, maintainability, and claim-boundary control

## Backup-Impact Statement

NA-0496 does not run backup or restore. It does not mutate qsl-backup, backup status, backup plan, rollback paths, `/backup/qsl`, qwork, qstart, qresume, qshell, nightly scripts, or local-ops tooling.

Backup-plan update classification:

- no backup-plan update required for NA-0496 because mutations are limited to tracked governance evidence, testplan, decision, traceability, and rolling journal paths
- future NA-0497 qsc test implementation remains tracked repository content and does not require backup tooling changes unless a later directive widens scope

## Next Recommendation

Proceed with NA-0497 as a qsc integration test implementation lane after NA-0496 closes:

`NA-0497 -- QSL Binding Negative Vector Consumer Test Implementation Harness`

NA-0497 should implement only the new manifest schema/mapping qsc integration test and governance evidence. It should not mutate qsc source, qsc fuzz, corpus, vectors, inputs, Cargo metadata, workflows, scripts, helpers, dependencies, lockfiles, formal models, refimpl code, services, public docs, backup paths, qwork, qstart, or qresume.
