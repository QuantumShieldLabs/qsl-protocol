Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-18
Replaces: n/a
Superseded-By: n/a

Goals: G1, G2, G3, G4, G5

# NA-0311 qsc Handshake Suite-ID qsc Harness Requirements Testplan

## Objective

Validate that NA-0311 records qsc harness requirements, vector-to-fixture
mapping, seam analysis, implementation-authorization prerequisites, successor
selection, and claim boundaries for future explicit qsc handshake suite-id
evidence without implementing runtime or wire-format behavior.

## Protected invariants

- NA-0311 remains the sole READY item until separate closeout.
- D-0601 exists exactly once after the patch and D-0602 remains absent.
- NA-0310 vector/refimpl oracle artifacts remain unchanged.
- NA-0309 model properties remain unchanged.
- qsc runtime, QHSM/QSP production wire-format, production handshake, crypto
  state-machine, key schedule, dependency, service, website, README,
  START_HERE, workflow, branch-protection, and public-safety configuration
  paths remain untouched.
- Missing direct qsc suite-id admission evidence remains visible.
- Persisted Suite-2 state is not represented as explicit suite-id admission.
- Metadata runtime reduction remains visible as near-term work.

## Allowed scope

- `docs/governance/evidence/NA-0311_qsc_handshake_suite_id_qsc_harness_requirements.md`
- `tests/NA-0311_qsc_handshake_suite_id_qsc_harness_requirements_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

- qsc/qsl runtime source and qsc runtime tests.
- QHSM/QSP production wire-format implementation.
- production handshake, crypto state-machine, and key schedule paths.
- `Cargo.toml`, `Cargo.lock`, workflows, scripts, services, apps, refimpl
  implementation, formal models, inputs, qsl-server, qsl-attachments,
  qsc-desktop, website/external repo, README, START_HERE, docs/public,
  branch-protection, and public-safety configuration.

## Prior vector/refimpl review requirements

Evidence must review:

- NA-0310 vector schema path.
- NA-0310 refimpl oracle path.
- 20 vector categories.
- expected accept, compatibility-accept, and reject categories.
- deterministic reason-label expectations.
- no-mutation/no-output/no-leak expectations.
- `future_gate` qsc harness expectation.
- NA-0309 model-property mapping.

## qsc seam analysis requirements

Evidence must analyze:

- existing qsc CLI/relay seam;
- existing qsc test helper seam;
- new test-only fixture builder;
- parser-only seam;
- runtime implementation seam; and
- formal/vector/refimpl-only continuation.

Each option must state feasibility, likely files, production behavior impact,
wire-format impact, qsc runtime impact, coverage ability, authorization need,
overclaim risk, and recommendation.

## Harness input requirements

Evidence must require:

- NA-0310 vector JSON;
- future QHSM v2 fixtures or equivalent shared parser fixtures;
- compatibility-mode config;
- suite-required-mode config;
- passphrase, route-token, identity, and sentinel fixtures;
- temp output and session-state paths; and
- deterministic reason labels.

## Accepted path requirements

Evidence must require:

- valid v2 Suite-2 vector accepted only with explicit parameter block;
- expected session-state tuple after accept;
- byte-exact A1/B1/A2 suite-context binding;
- transcript binding before accepted state;
- key-context binding before accepted state;
- legacy v1 compatibility accepted only by explicit compatibility mode; and
- no secret/sentinel/log leak on accepted paths.

## Reject path requirements

Evidence must require rejects for:

- legacy v1 in suite-required mode;
- unsupported suite;
- downgraded tuple;
- stripped or missing suite context;
- mismatched A1/B1 and B1/A2 contexts;
- duplicate suite parameter;
- unknown critical parameter;
- unknown noncritical parameter unless later authorized;
- noncanonical order;
- malformed length;
- inconsistent protocol/suite tuple;
- transcript mismatch;
- key-context mismatch or missing context; and
- replayed A1/A2 or a later explicitly authorized deterministic no-op.

## No-mutation/no-output/no-leak requirements

Evidence must require that rejects create or change no accepted session-state
file, emit no `recv_commit`, produce no decrypted or temp output, leave no
partial state, remain deterministic on repeated execution, and leak no
passphrase, route token, plaintext sentinel, vector sentinel, panic/backtrace,
or raw key material.

## Future marker requirements

Evidence must include:

- `NA0311_QSC_HARNESS_REQUIREMENTS_OK`
- `NA0311_QSC_TEST_SEAM_BLOCKED` or `NA0311_QSC_TEST_SEAM_IDENTIFIED_OK`
- `NA0311_QSC_VALID_V2_EXPECTATION_OK`
- `NA0311_QSC_LEGACY_COMPAT_EXPECTATION_OK`
- `NA0311_QSC_REQUIRED_MODE_REJECT_EXPECTATION_OK`
- `NA0311_QSC_UNSUPPORTED_SUITE_REJECT_EXPECTATION_OK`
- `NA0311_QSC_DOWNGRADE_REJECT_EXPECTATION_OK`
- `NA0311_QSC_MALFORMED_REJECT_EXPECTATION_OK`
- `NA0311_QSC_NO_MUTATION_EXPECTATION_OK`
- `NA0311_QSC_NO_OUTPUT_EXPECTATION_OK`
- `NA0311_QSC_NO_LEAK_EXPECTATION_OK`

## Implementation authorization requirements

Evidence must define:

- what would be required if a future test-only seam is proposed;
- what would be required if qsc runtime/QHSM v2 implementation is required;
- exact likely files for future lanes;
- proof that production behavior is untouched unless authorized;
- transcript/key-context prerequisites;
- compatibility migration rules;
- external-review considerations without claiming completion; and
- stop conditions.

## Coverage matrix requirements

Evidence must map each NA-0310 vector category to:

- NA-0309 model property;
- refimpl oracle assertion;
- required qsc harness fixture;
- required seam;
- current status using the allowed status vocabulary;
- expected future artifact;
- risk; and
- next action.

## Successor-selection requirements

Evidence must select exactly one NA-0312 successor and explain why alternatives
were rejected. If the current qsc code lacks a sufficient test-only seam, the
successor should be an implementation-authorization lane rather than a harness
implementation lane.

## Metadata agenda requirements

Evidence must state that metadata runtime identifier/default-padding work
remains a near-term priority and must not claim that NA-0311 implements
metadata runtime behavior.

## Claim-boundary requirements

Evidence and PR text must not claim production deployment readiness, broad
public deployment readiness, completed external-review status, anonymous
operation, metadata elimination, absence-of-traceability behavior, qsc runtime
suite-id implementation, production wire-format implementation, key-schedule
implementation, or full cryptographic verification.

## Backup-impact requirements

Evidence must record whether the patch changes evidence locations, response
paths, source roots, excluded backup paths, or non-rebuildable artifacts
outside current backup scope. Expected result: no backup-plan update if changes
stay under the qsl-protocol worktree.

## Required local checks

- `python3 -m json.tool inputs/suite2/qsc_handshake_suite_id_vectors_na0310.json`
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
  scan, and goal-lint evidence.

## CI expectations

Required checks must attach and complete green before merge. `public-safety`
must remain required and complete green before merge and after merge.

## Successor handoff

If NA-0311 merges and post-merge public-safety is green, closeout should mark
NA-0311 DONE and restore exactly one READY successor:

NA-0312 -- qsc Handshake Suite-ID Parameter-Block Implementation Authorization

NA-0312 must not be implemented by NA-0311 closeout.
