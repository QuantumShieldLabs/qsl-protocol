Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-17
Replaces: n/a
Superseded-By: n/a

Goals: G1, G2, G3, G4, G5

# NA-0310 qsc Handshake Suite-ID Vector Refimpl Oracle Testplan

## Objective

Add deterministic vector schema and refimpl oracle proof for the NA-0309 qsc
handshake suite-id model properties without implementing qsc runtime behavior,
QHSM/QSP production wire format, production handshake behavior, crypto
state-machine behavior, key schedule behavior, or dependency changes.

## Protected invariants

- NA-0310 remains the sole READY item until separate closeout.
- D-0599 exists exactly once after the patch and D-0600 remains absent.
- Existing NA-0309 formal/model checks remain active.
- Vector/refimpl proof is not represented as qsc runtime implementation.
- Persisted Suite-2 state is not represented as explicit suite-id admission.
- All qsc suite-id implementation gaps remain visible.

## Allowed scope

- `inputs/suite2/vectors/qshield_qsc_handshake_suite_id_vectors_na0310.json`
- `tools/refimpl/quantumshield_refimpl/tests/na_0310_qsc_suite_id_vector_oracle.rs`
- `docs/governance/evidence/NA-0310_qsc_handshake_suite_id_vector_refimpl_oracle.md`
- `tests/NA-0310_qsc_handshake_suite_id_vector_refimpl_oracle_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

- qsc/qsl runtime source and tests.
- QHSM/QSP production wire-format implementation.
- production handshake, crypto state-machine, and key schedule paths.
- Cargo manifests and lockfiles.
- workflows, scripts, services, qsl-server, qsl-attachments, qsc-desktop,
  website/external repo, README, START_HERE, docs/public, branch protection,
  and public-safety configuration.

## Vector schema requirements

The vector JSON must include required top-level metadata and one `vectors`
array. Every vector must include:

- identity and category fields;
- qhsm version and compatibility/required-mode fields;
- frame sequence;
- protocol/suite tuple fields;
- negotiated-parameter representation;
- transcript and key-context labels;
- canonical encoding expectation;
- deterministic expected result and reason label;
- mutation, recv_commit, output, and secret-leak expectations;
- model-property references;
- refimpl oracle expectation; and
- future-gated qsc harness expectation.

## Vector category requirements

All required categories must be present exactly once:

- valid v2 Suite-2 parameter block;
- legacy compatibility allowed;
- legacy required-mode reject;
- unsupported, downgraded, stripped, mismatched, duplicate, unknown critical,
  unknown noncritical, noncanonical, malformed, inconsistent, replayed A1,
  replayed A2, transcript mismatch, key-context mismatch, and missing
  key-context rejects; and
- valid Suite-2 transcript binding.

## Refimpl oracle requirements

The oracle must:

- parse the NA-0310 vector JSON;
- validate metadata and per-vector required fields;
- reject duplicate IDs or duplicate categories;
- require all categories;
- ensure explicit Suite-2 accepts use `0x0500` / `0x0002`;
- keep legacy compatibility separate from explicit admission;
- require reject vectors to have deterministic reason labels;
- require reject no-mutation, no-output, no-recv_commit, and no-leak
  expectations;
- validate model-property references against the NA-0309 reference set; and
- keep all qsc harness expectations future-gated.

## Model cross-check requirements

Every vector must map to one or more NA-0309 model marker/property references.
Replay and unknown-noncritical vectors may map to generic NA-0309 reject-boundary
properties while keeping their qsc harness status future-gated.

## Qsc harness expectation requirements

Every vector must state that qsc harness proof is a future gate. No vector may
claim current qsc runtime admission, QHSM v2 parsing, production wire-format
behavior, or key-schedule behavior.

## Marker requirements

The targeted oracle must emit:

- `NA0310_VECTOR_SCHEMA_OK`
- `NA0310_VECTOR_CATEGORIES_OK`
- `NA0310_VALID_SUITE2_VECTOR_OK`
- `NA0310_LEGACY_COMPAT_VECTOR_OK`
- `NA0310_REQUIRED_MODE_REJECT_VECTOR_OK`
- `NA0310_UNSUPPORTED_SUITE_VECTOR_OK`
- `NA0310_DOWNGRADE_VECTOR_OK`
- `NA0310_STRIPPED_SUITE_VECTOR_OK`
- `NA0310_MISMATCH_VECTOR_OK`
- `NA0310_DUPLICATE_VECTOR_OK`
- `NA0310_UNKNOWN_CRITICAL_VECTOR_OK`
- `NA0310_NONCANONICAL_VECTOR_OK`
- `NA0310_MALFORMED_VECTOR_OK`
- `NA0310_TRANSCRIPT_VECTOR_OK`
- `NA0310_KEY_CONTEXT_VECTOR_OK`
- `NA0310_NO_MUTATION_EXPECTATIONS_OK`
- `NA0310_NO_OUTPUT_EXPECTATIONS_OK`
- `NA0310_NO_SECRET_LEAK_EXPECTATIONS_OK`
- `NA0310_REFIMPL_ORACLE_OK`
- `NA0310_QSC_SUITE_ID_VECTOR_REFIMPL_ORACLE_OK`

## Coverage matrix requirements

Evidence must map model property, vector category, refimpl oracle assertion,
qsc harness expectation, current status, expected future artifact, risk, and
next action. Valid statuses include `PROVEN_MODEL`, `PROVEN_VECTOR`,
`PROVEN_REFIMPL_ORACLE`, `READY_FOR_QSC_HARNESS`, `BLOCKED`, and
`FUTURE_GATE`.

## Successor-selection requirements

If vector/refimpl succeeds, select:

NA-0311 -- qsc Handshake Suite-ID qsc Harness Requirements and Test Seam Plan

If vector/refimpl is blocked, select:

NA-0311 -- qsc Handshake Suite-ID Vector/Oracle Blocker Resolution

NA-0311 must not be implemented in this lane.

## Claim-boundary requirements

Evidence and PR text must not claim production readiness, public internet
readiness, external review completion, anonymity, metadata-free behavior,
untraceability, qsc runtime suite-id implementation, production wire-format
implementation, key-schedule implementation, or complete cryptographic proof.

## Backup-impact requirements

Record whether the patch changes evidence locations, response paths, source
roots, excluded backup paths, or non-rebuildable artifacts outside current
backup scope. Expected result: no backup-plan update if changes stay under the
qsl-protocol worktree.

## Required local checks

- `python3 -m json.tool inputs/suite2/vectors/qshield_qsc_handshake_suite_id_vectors_na0310.json`
- `cargo test --manifest-path tools/refimpl/quantumshield_refimpl/Cargo.toml --test na_0310_qsc_suite_id_vector_oracle -- --nocapture`
- `cargo test --manifest-path tools/refimpl/quantumshield_refimpl/Cargo.toml`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `scripts/ci/metadata_conformance_smoke.sh`
- `scripts/ci/metadata_phase2_identifier_padding_harness.sh`
- `scripts/ci/metadata_phase2_sanitized_errors_retention_harness.sh`
- `scripts/ci/demo_cli_smoke.sh`
- `DEMO_STRESS_PROFILE=baseline scripts/ci/demo_adversarial_stress.sh`
- focused NA-0304/NA-0303/NA-0302/NA-0301/NA-0300 harnesses where feasible
- queue, decisions, scope-guard, link-check, leak-scan, classifier, overclaim
  scan, and goal-lint proof.

## CI expectations

Required checks must attach and complete green before merge. `public-safety`
must remain required and complete green before merge and after merge.

## Successor handoff

NA-0311 should define qsc harness requirements and a test seam plan for these
vectors. NA-0311 must not be implemented by NA-0310 closeout, and must not
authorize qsc runtime or wire-format changes without explicit future scope.
