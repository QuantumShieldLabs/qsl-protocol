Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-19
Goals: G1, G2, G3, G4, G5

# NA-0500 QSL qsc Secret-Material Diagnostic No-Output Boundary Test Implementation Harness

## Executive summary

NA-0500 consumes NA-0499/D379 inheritance and implements one bounded qsc integration test at `qsl/qsl-client/qsc/tests/secret_material_diagnostic_boundary.rs`.

The test exercises real qsc reject/error/diagnostic output surfaces and asserts the captured stdout, stderr, exit/error text, and test-visible diagnostic strings do not contain synthetic secret-shaped labels or long high-entropy-looking material. It uses only synthetic labels and synthetic temp roots. It does not mutate qsc source, qsc fuzz targets, Cargo metadata, dependencies, workflows, scripts, helpers, corpus files, vectors, inputs, formal models, refimpl, services, public docs, or backup paths.

Primary result: `cargo test -p qsc --locked --test secret_material_diagnostic_boundary -- --test-threads=1 --nocapture` passed and emitted the required NA-0500 markers.

## Live NA-0500 scope

Allowed implementation path used:

- `qsl/qsl-client/qsc/tests/secret_material_diagnostic_boundary.rs`

Allowed governance paths used:

- `docs/governance/evidence/NA-0500_qsl_qsc_secret_material_diagnostic_no_output_boundary_test_implementation_harness.md`
- `tests/NA-0500_qsl_qsc_secret_material_diagnostic_no_output_boundary_test_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

No optional closeout to NA-0501 is performed in this implementation evidence.

## qwork proof-file verification

Codex did not run `qwork`, `qstart`, or `qresume`.

Read and copied into the proof root:

- `/srv/qbuild/work/NA-0500/.qwork/startup.qsl-protocol.kv`
- `/srv/qbuild/work/NA-0500/.qwork/startup.qsl-protocol.json`

Required proof fields passed:

- `startup_result=OK`
- `lane=NA-0500`
- `repo=qsl-protocol`
- `path=/srv/qbuild/work/NA-0500/qsl-protocol`
- `head_equals_origin_main=yes`
- `worktree_clean=yes`
- `index_clean=yes`
- `untracked_clean=yes`
- `ready_count=1`
- `queue_top_ready=NA-0500`
- `requested_lane_status=READY`

Proof HEAD and proof origin/main matched live pre-fetch HEAD and origin/main at `87dd24ef1219`. The qwork `.kv` and `.json` proofs mirrored required values. Disk usage before fetch: `/` 77%; `/backup/qsl` 24%; the 95% stop threshold was not hit.

Read-only qsl-backup boundary passed: installed helper SHA-256 matched `e9ecff3d22ed...0052f6232`, Codex ops source inclusion count was exactly 1, and no backup or restore was run.

## NA-0499 / D379 inheritance

NA-0499 is closed. NA-0500 was restored READY as the sole READY item by D-0988 / PR #1270. D379 response exists at `/home/victor/work/qsl/codex/responses/NA0499_closeout_restore_na0500_20260619T144213Z_D379.md`.

Inherited facts consumed:

- selected classification: `SECRET_MATERIAL_DIAGNOSTIC_NO_OUTPUT_TEST_READY`;
- selected future test path: `qsl/qsl-client/qsc/tests/secret_material_diagnostic_boundary.rs`;
- selected goal: direct qsc evidence that selected reject/error/diagnostic outputs do not expose secret-shaped material;
- no source, dependency, Cargo, workflow, corpus/vector/input, helper, refimpl, formal, service, public, or backup mutation selected;
- no public-readiness, crypto-complete, secret-material-complete, zeroization-complete, memory-erasure-complete, or side-channel-free claim selected.

Startup main public-safety and qsc-adversarial-smoke were green on `87dd24ef1219`.

## Pre-mutation diagnostic surface inventory

Selected test path state before mutation: absent.

Existing integration-test patterns found:

- `common::TestIsolation` creates isolated HOME, XDG config, and TMPDIR roots.
- `common::qsc_std_command` invokes the real qsc binary and applies the mock vault unlock path.
- Existing tests use `QSC_MARK_FORMAT=plain`, capture `std::process::Output`, and combine stdout/stderr.
- qsc reject paths emit deterministic `QSC_MARK/1` markers such as `event=error`, `event=handshake_reject`, and command-specific marker events.

Candidate classifications:

| Candidate | Classification | Decision |
|---|---|---|
| `config set policy-profile bad` -> `event=error code=invalid_policy_profile` | FEASIBLE_DIRECT_QSC_OUTPUT_PATH | Selected; deterministic CLI error, no vault needed. |
| `util sanitize` with missing `--print` -> `event=util_sanitize code=usage` plus stderr usage text | FEASIBLE_DIRECT_QSC_OUTPUT_PATH | Selected; covers stdout and stderr capture. |
| `handshake init` with local identity and peer route set but no peer pin -> `identity_unknown`, `handshake_reject`, and error marker | FEASIBLE_TEST_VISIBLE_ERROR_PATH | Selected; real handshake/binding reject surface without network traffic or source mutation. |
| Provider decap failure no-mutation path | SUPPORTING_ONLY_PATH | Inherited validation remains in `handshake_provider_error_no_mutation`; not duplicated in the new narrow test. |
| Binding transcript mutation tests | SUPPORTING_ONLY_PATH | Existing tests remain direct inherited binding evidence; not duplicated to keep NA-0500 minimal. |
| Direct panic-hook or crash-artifact instrumentation | REQUIRES_QSC_SOURCE_MUTATION | Rejected by scope. |
| Fuzz helper-only binding surfaces | REQUIRES_DEPENDENCY_OR_CARGO_MUTATION or REQUIRES_QSC_SOURCE_MUTATION | Rejected by scope. |
| Full qsc-adversarial fuzz run | TOO_FLAKY_OR_PROCESS_HEAVY | Rejected for local implementation test scope. |

## qsc integration test implementation

The new test file contains three tests:

1. `reject_diagnostics_do_not_contain_secret_markers`
   - captures real qsc output for invalid config, util-sanitize usage, and handshake `identity_unknown` reject paths;
   - scans captured stdout/stderr/error text for forbidden synthetic marker labels and high-entropy-looking tokens;
   - prints `NA0500_NO_SECRET_OUTPUT_BOUNDARY_OK` and `NA0500_DIAGNOSTIC_REJECT_PATHS_CHECKED_OK`.

2. `diagnostic_scrubber_rejects_synthetic_secret_markers`
   - feeds synthetic marker labels into the scanner;
   - proves the scanner fails on those synthetic labels;
   - prints private-key, passphrase, KEM secret, signature secret, and shared-secret marker-absence OK markers.

3. `na0500_common_no_overclaim_markers`
   - prints the required scope, no-source-change, no-dependency-change, no-workflow-change, no-overclaim, and one-READY markers.

The test uses no new dependency and no qsc source mutation.

## real reject/error/diagnostic surfaces exercised

Selected real qsc surfaces:

- CLI invalid config reject: `QSC_MARK/1 event=error code=invalid_policy_profile`.
- CLI util-sanitize usage reject: `QSC_MARK/1 event=util_sanitize code=usage` and stderr usage text.
- Handshake reject: `event=identity_unknown`, `event=handshake_reject reason=identity_unknown`, and `event=error code=identity_unknown`.

These are real qsc command/error surfaces, not hardcoded output-only fixtures.

## no-secret-output scanner proof

The scanner checks captured qsc diagnostic text for synthetic labels representing private-key material, passphrases, KEM secret material, signature secret material, shared secrets, backup/recovery key material, runtime/service secret material, private endpoint labels, operator/user data labels, route-token marker labels, session-store secret labels, pending-handshake secret labels, and identity-signing secret labels.

It also rejects long high-entropy-looking alphanumeric tokens. Real qsc diagnostics passed the scanner in the targeted test.

## synthetic marker fail proof

The scanner fail proof feeds synthetic strings such as `synthetic diagnostic carried private_key_marker`, `passphrase_marker`, `kem_secret_marker`, `signature_secret_marker`, and `shared_secret_marker` into the same scanner and asserts findings are produced. The fixture uses synthetic labels only and no real secret values.

## real qsc diagnostics pass proof

`cargo test -p qsc --locked --test secret_material_diagnostic_boundary -- --test-threads=1 --nocapture` passed with all three tests green and emitted:

- `NA0500_SECRET_MATERIAL_SCOPE_CONSUMED_OK`
- `NA0500_NO_SECRET_OUTPUT_BOUNDARY_OK`
- `NA0500_DIAGNOSTIC_REJECT_PATHS_CHECKED_OK`
- `NA0500_PRIVATE_KEY_MARKER_ABSENT_OK`
- `NA0500_PASSPHRASE_MARKER_ABSENT_OK`
- `NA0500_KEM_SECRET_MARKER_ABSENT_OK`
- `NA0500_SIGNATURE_SECRET_MARKER_ABSENT_OK`
- `NA0500_SHARED_SECRET_MARKER_ABSENT_OK`
- `NA0500_NO_QSC_SOURCE_CHANGE_OK`
- `NA0500_NO_DEPENDENCY_CHANGE_OK`
- `NA0500_NO_WORKFLOW_CHANGE_OK`
- `NA0500_NO_PUBLIC_READINESS_CLAIM_OK`
- `NA0500_NO_CRYPTO_COMPLETE_CLAIM_OK`
- `NA0500_NO_SECRET_MATERIAL_COMPLETE_CLAIM_OK`
- `NA0500_NO_ZEROIZATION_COMPLETE_CLAIM_OK`
- `NA0500_NO_MEMORY_ERASURE_COMPLETE_CLAIM_OK`
- `NA0500_NO_SIDE_CHANNEL_FREE_CLAIM_OK`
- `NA0500_ONE_READY_INVARIANT_OK`

## no qsc source / Cargo / dependency / workflow mutation proof

No qsc source file is changed. No Cargo manifest, lockfile, dependency metadata, workflow, script, helper, validator script, qsc-adversarial script, fuzz target, fuzz Cargo file, fuzz lockfile, corpus, vector, or input file is changed.

The only qsc path changed is the new integration test file authorized by NA-0500.

## no corpus/vector/input mutation proof

The binding corpus validator passed for both the binding corpus and all qsc fuzz corpus paths. No corpus/vector/input/internal-manifest path is changed.

## no public-readiness / secret-material-complete / side-channel-free claim proof

NA-0500 provides bounded internal qsc diagnostic/no-output evidence for selected reject/error surfaces only.

No public-readiness claim is made. No production-readiness claim is made. No public-internet-readiness claim is made. No external-review-complete claim is made. No crypto-complete claim is made. No KEM-complete claim is made. No signature-complete claim is made. No identity-complete claim is made. No provider-RNG-complete claim is made. No secret-material-complete claim is made. No zeroization-complete claim is made. No memory-erasure-complete claim is made. No side-channel-free claim is made. No vulnerability-free claim is made. No bug-free claim is made. No perfect-crypto claim is made.

## validation

Initial implementation validation passed:

- `cargo test -p qsc --locked --test secret_material_diagnostic_boundary -- --test-threads=1 --nocapture`
- `cargo test -p qsc --locked --test key_lifecycle_zeroization -- --test-threads=1 --nocapture`
- `cargo test -p qsc --locked --test handshake_provider_error_no_mutation -- --test-threads=1 --nocapture`
- `python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus/qsc_binding_semantics`
- `python3 scripts/audit/validate_binding_fuzz_corpus_no_secrets.py --format json --path qsl/qsl-client/qsc/fuzz/corpus`

Full validation remains required before PR creation and merge per the NA-0500 testplan.

## scope guard

Intended changed paths before PR are limited to:

- `qsl/qsl-client/qsc/tests/secret_material_diagnostic_boundary.rs`
- `docs/governance/evidence/NA-0500_qsl_qsc_secret_material_diagnostic_no_output_boundary_test_implementation_harness.md`
- `tests/NA-0500_qsl_qsc_secret_material_diagnostic_no_output_boundary_test_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## successor selection

Default successor selected after successful NA-0500 implementation: `NA-0501 -- QSL qsc Key Lifecycle / Zeroization Expansion Scope Authorization Plan`.

Rationale: NA-0500 adds selected diagnostic/no-output evidence. The next highest secret-material lifecycle residual is deeper key lifecycle, zeroization, and memory-lifetime scope. NA-0501 must not be implemented by this lane.

## next recommendation

Merge NA-0500 only after required local validation and PR checks pass. If post-merge public-safety is green inside the short attach/early-failure window, a separate closeout may restore NA-0501. If public-safety is still running but healthy after the short attach window, hand off closeout without long polling in this implementation directive.
