Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-19
Goals: G1, G2, G3, G4, G5

# NA-0502 QSL qsc Key Lifecycle Zeroization Expansion Test Implementation Harness

## Executive summary

NA-0502 consumes NA-0501/D386 inheritance and implements one bounded qsc integration test at `qsl/qsl-client/qsc/tests/key_lifecycle_zeroization_expansion.rs`.

The new test expands observable qsc key lifecycle and cleanup evidence beyond `key_lifecycle_zeroization.rs` by checking two selected surfaces:

- identity KEM/signing secret rotation and public-record artifact boundaries;
- responder pending-confirm reject and session-artifact boundaries.

The test uses existing qsc integration-test patterns, synthetic test roots, and test-generated qsc material only. It does not mutate qsc source, Cargo metadata, dependencies, workflows, scripts, helpers, corpus files, vectors, inputs, formal models, refimpl, service paths, public docs, or backup paths.

This is bounded internal verification evidence only. It does not prove secret-material completeness, zeroization completeness, memory erasure, side-channel freedom, public readiness, production readiness, crypto completeness, vulnerability freedom, bug freedom, or perfect crypto.

## Live NA-0502 scope

Allowed implementation path used:

- `qsl/qsl-client/qsc/tests/key_lifecycle_zeroization_expansion.rs`

Allowed governance paths used:

- `docs/governance/evidence/NA-0502_qsl_qsc_key_lifecycle_zeroization_expansion_test_implementation_harness.md`
- `tests/NA-0502_qsl_qsc_key_lifecycle_zeroization_expansion_test_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

No optional closeout to NA-0503 is performed by this implementation evidence.

## qwork proof-file verification

Codex did not run `qwork`, `qstart`, or `qresume`.

Read and copied into the proof root:

- `/srv/qbuild/work/NA-0502/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0502/.qwork/startup.qsl-protocol.json`

Required proof fields passed:

- `startup_result=OK`
- `lane=NA-0502`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0502/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0502`
- `requested_lane_status=READY`

Proof HEAD and proof origin/main matched live pre-fetch HEAD and origin/main at `5c893b83c283`. The `.kv` and `.json` proofs mirrored required values. Disk usage before fetch: `/` 79%; `/backup/qsl` 24%; the 95% stop threshold was not hit.

Read-only qsl-backup boundary passed: installed helper matched the expected digest prefix `e9ecff3d22ed`, Codex ops source inclusion count was exactly 1, and no backup or restore was run.

## NA-0501 / D386 inheritance

NA-0501 completed and NA-0502 was restored READY as the sole READY item by D-0992 / PR #1275.

Inherited facts consumed:

- selected classification: `KEY_LIFECYCLE_ZEROIZATION_EXPANSION_TEST_READY`;
- selected future test path: `qsl/qsl-client/qsc/tests/key_lifecycle_zeroization_expansion.rs`;
- new-file test preferred over modifying `key_lifecycle_zeroization.rs`;
- same-host client-to-client E2E remains high-value and near-term, but was deferred because lifecycle gaps were narrower and directly aligned with NA-0501;
- existing direct evidence includes `key_lifecycle_zeroization.rs`, `handshake_provider_error_no_mutation.rs`, `secret_material_diagnostic_boundary.rs`, qsc binding negative tests, and corpus validator scans;
- no source, dependency, Cargo, workflow, corpus/vector/input, helper, refimpl, formal, service, public, or backup mutation was selected;
- no secret-material-complete, zeroization-complete, memory-erasure-complete, or side-channel-free claim was selected.

Startup main public-safety and qsc-adversarial-smoke were green on `5c893b83c283`; qsc-linux-full-suite and macos-qsc-full-serial were attached and skipped under accepted policy.

## Pre-mutation lifecycle surface selection

Selected test path state before mutation: absent.

Candidate classifications:

| Candidate | Classification | Decision |
|---|---|---|
| identity key file lifecycle / cleanup observable from integration test | FEASIBLE_DIRECT_QSC_INTEGRATION_SURFACE | Selected. qsc identity rotate stores KEM/signing secrets in the mock vault and public records under identities; the test rotates twice and checks current artifact replacement/absence boundaries. |
| KEM/signature/transcript binding reject cleanup | FEASIBLE_ARTIFACT_BOUNDARY_SURFACE / SUPPORTING_ONLY_PATH | Existing NA-0476 and provider-error tests cover several binding rejects; NA-0502 uses a responder confirm reject artifact boundary instead of duplicating protocol validation logic. |
| pending-handshake/session temp-root cleanup | FEASIBLE_DIRECT_QSC_INTEGRATION_SURFACE | Selected through responder pending-confirm reject: pending state is present, a malformed confirm is rejected, and no completed session artifact is created. |
| session-store artifact cleanup | FEASIBLE_ARTIFACT_BOUNDARY_SURFACE | Selected as a no-session-artifact and no session-store-key-before-confirm check. |
| public-record/trusted-pin lifecycle, non-secret but lifecycle-relevant | FEASIBLE_ARTIFACT_BOUNDARY_SURFACE | Selected only as identity public-record boundary, not as a zeroization target. |
| vault/passphrase/operator-data boundary | SUPPORTING_ONLY_PATH | Existing NA-0446 and NA-0500 tests cover passphrase/output redaction; not duplicated as a primary surface. |
| X25519/ephemeral lifecycle | REQUIRES_QSC_SOURCE_MUTATION or SUPPORTING_ONLY_PATH | Direct lifetime or erasure proof remains residual without source instrumentation. |

Minimum implementation bar was met: at least two bounded lifecycle/cleanup surfaces are checked, at least one surface is distinct from `key_lifecycle_zeroization.rs`, and at least one surface covers reject/artifact behavior.

## qsc integration test implementation

The new test file contains three tests:

1. `expanded_key_lifecycle_boundaries_are_observable_and_bounded`
   - initializes a mock vault;
   - rotates Alice's identity twice through the real qsc CLI;
   - reads the test-visible mock vault to compare generated identity KEM/signing secret replacement;
   - asserts previous generated identity material is not retained in the current decrypted vault JSON or encrypted vault envelope;
   - asserts identity public records contain public fields only and do not carry private/secret field names or generated secret material;
   - prints `NA0502_KEY_LIFECYCLE_SCOPE_CONSUMED_OK`, `NA0502_ZEROIZATION_EXPANSION_TEST_IMPLEMENTED_OK`, and `NA0502_SELECTED_LIFECYCLE_SURFACES_CHECKED_OK`.

2. `reject_or_artifact_boundaries_do_not_retain_forbidden_markers`
   - creates two independent synthetic qsc client roots;
   - performs A1/B1 handshake setup until Bob has responder pending state and no completed session;
   - injects a malformed synthetic confirm marker through the existing inbox test server;
   - asserts qsc emits a bounded reject and does not emit handshake completion or receive-commit markers;
   - asserts Bob's vault bytes and session path are unchanged, no legacy pending plaintext artifact appears, no completed session is created, and the synthetic marker is not retained in selected qsc artifacts or diagnostics;
   - prints `NA0502_REJECT_OR_ARTIFACT_BOUNDARY_CHECKED_OK`.

3. `na0502_common_no_overclaim_markers`
   - prints the required no-source-change, no-dependency-change, no-workflow-change, no-overclaim, and one-READY markers.

The test uses no new dependency and no qsc source mutation.

## selected lifecycle/cleanup surfaces checked

Selected lifecycle/cleanup surface 1: identity KEM/signing rotation.

- Observable behavior: qsc `identity rotate --confirm` replaces the test-visible mock vault values for `identity.kem_sk.alice` and `identity.sig_sk.alice`, and updates the identity public record.
- Bound asserted: the previous generated KEM/signing values are not retained in the current decrypted mock vault JSON or encrypted vault envelope after a second rotation.
- Caveat: this is artifact replacement/absence evidence only; it is not memory-erasure evidence.

Selected lifecycle/cleanup surface 2: responder pending-confirm reject/session artifact boundary.

- Observable behavior: after A1/B1, Bob has responder pending state with a pending session snapshot but no completed session artifact.
- Bound asserted: a malformed synthetic confirm is rejected, does not create or change a completed session artifact, does not populate the session-store key, does not create the legacy pending plaintext artifact, and does not retain the synthetic marker in selected qsc artifacts or diagnostics.
- Caveat: this is reject/no-session-artifact evidence only; it is not a full transcript, replay, or side-channel proof.

## reject/artifact boundary proof

The reject/artifact boundary uses the real qsc handshake command path and existing test inbox server. It does not parse or reimplement protocol validation as a fake oracle. The test asserts qsc's own output contains `event=handshake_reject` and excludes success markers, then checks the observable artifacts around Bob's config root.

The synthetic marker is non-secret and is used only to verify it is not retained in selected artifacts or diagnostics.

## no qsc source / Cargo / dependency / workflow mutation proof

No qsc source file is changed. No Cargo manifest, lockfile, dependency metadata, workflow, script, helper, validator script, qsc-adversarial script, fuzz target, fuzz Cargo file, or fuzz lockfile is changed.

The only qsc path changed is the new integration test file authorized by NA-0502.

## no corpus/vector/input mutation proof

No corpus, vector, input, or internal-manifest path is changed. Validator scans are required and recorded as supporting evidence before PR.

## no public-readiness / secret-material-complete / zeroization-complete / memory-erasure-complete / side-channel-free claim proof

NA-0502 provides bounded internal qsc artifact and reject-boundary evidence only.

No public-readiness claim is made. No production-readiness claim is made. No public-internet-readiness claim is made. No external-review-complete claim is made. No crypto-complete claim is made. No KEM-complete claim is made. No signature-complete claim is made. No identity-complete claim is made. No provider-RNG-complete claim is made. No secret-material-complete claim is made. No zeroization-complete claim is made. No memory-erasure-complete claim is made. No side-channel-free claim is made. No vulnerability-free claim is made. No bug-free claim is made. No perfect-crypto claim is made.

## validation

Local validation passed after bounded recoveries:

- `git diff --check`
- `cargo test -p qsc --locked --test key_lifecycle_zeroization_expansion -- --test-threads=1 --nocapture`
- `cargo test -p qsc --locked --test key_lifecycle_zeroization -- --test-threads=1 --nocapture`
- `cargo test -p qsc --locked --test secret_material_diagnostic_boundary -- --test-threads=1 --nocapture`
- `cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture`
- `cargo test -p qsc --locked --test binding_negative_vector_consumer -- --test-threads=1 --nocapture`
- `python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics`
- `python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus`
- `python3 formal/run_model_checks.py`
- `cargo audit --deny warnings`
- `cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock`
- `cargo fmt --check`
- `sh -n scripts/ci/qsc_adversarial.sh`
- `bash -n scripts/ci/qsc_adversarial.sh`

Recovered validation issues:

- First targeted NA-0502 test run failed before reaching the selected surface because the new test used a synthetic route-token shape rejected by existing qsc validation. Classification: recoverable in-scope local validation failure with understood cause. Corrective action: changed only the new test constants to the valid route-token shape used by existing qsc integration tests. Final result: rerun passed with all three tests green and all required NA-0502 markers emitted.
- First validation wrapper stopped after `git diff --check` because the local proof wrapper used `exit` inside a redirected shell group. Classification: recoverable command-shape issue. Corrective action: reran with a corrected wrapper that captured rc without exiting the outer shell. Final result: validation proceeded.
- `cargo fmt --check` failed on the new qsc integration test formatting. Classification: recoverable in-scope formatting failure. Corrective action: ran `cargo fmt -p qsc`, verified no tracked out-of-scope file changed, reran `cargo fmt --check`, and reran the NA-0502 test. Final result: both passed.

## scope guard

Intended changed paths before PR are limited to:

- `qsl/qsl-client/qsc/tests/key_lifecycle_zeroization_expansion.rs`
- `docs/governance/evidence/NA-0502_qsl_qsc_key_lifecycle_zeroization_expansion_test_implementation_harness.md`
- `tests/NA-0502_qsl_qsc_key_lifecycle_zeroization_expansion_test_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## successor selection

Default successor selected after successful NA-0502 implementation: `NA-0503 -- QSL qsc Same-Host Client-to-Client End-to-End Test Scope Authorization Plan`.

Rationale: NA-0502 adds expanded key lifecycle / cleanup evidence. The Director has explicitly asked when client-to-client testing begins. Same-host client-to-client E2E is the next broad, user-realistic core assurance lane and should not be postponed indefinitely after lifecycle expansion. It remains authorization-only first and does not require remote SSH.

## client-to-client E2E note

Same-host client-to-client E2E remains deferred by this implementation lane only because NA-0502 is the exact restored implementation scope. It is selected as the default next authorization lane after successful merge and closeout.

## next recommendation

Merge NA-0502 only after required local validation and PR checks pass. If post-merge public-safety is green inside the short attach/early-failure window, a separate closeout may restore NA-0503. If public-safety is still running but healthy after the short attach window, hand off closeout without long polling in this implementation directive.
