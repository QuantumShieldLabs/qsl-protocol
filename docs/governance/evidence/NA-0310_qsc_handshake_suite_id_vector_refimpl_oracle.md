Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-17
Replaces: n/a
Superseded-By: n/a

Goals: G1, G2, G3, G4, G5

# NA-0310 qsc Handshake Suite-ID Vector Schema and Refimpl Oracle

Directive: QSL-DIR-2026-05-17-118 / NA-0310

## Executive summary

NA-0310 adds a deterministic vector schema and refimpl oracle test for future
qsc handshake suite-id semantics selected by NA-0307, designed in NA-0308, and
bounded by the NA-0309 executable model.

This lane adds:

- `inputs/suite2/vectors/qshield_qsc_handshake_suite_id_vectors_na0310.json`
- `tools/refimpl/quantumshield_refimpl/tests/na_0310_qsc_suite_id_vector_oracle.rs`

This lane does not implement qsc runtime behavior, QHSM/QSP production wire
format, production handshake behavior, crypto state-machine behavior, key
schedule behavior, dependency changes, service changes, website changes,
workflow changes, branch-protection changes, or public-safety changes.

Selected successor:

NA-0311 -- qsc Handshake Suite-ID qsc Harness Requirements and Test Seam Plan

## Live NA-0310 scope

Live `NEXT_ACTIONS.md` authorizes vector schema and refimpl oracle
requirements or implementation only if the live directive explicitly
authorizes exact vector/refimpl files. The live directive authorizes this
vector/refimpl lane and forbids qsc runtime, QHSM wire-format, production
handshake, key schedule, QSP wire-format, production suite-id field,
dependency, service, website, README, START_HERE, workflow, branch-protection,
and public-safety changes.

## Inherited NA-0309 model

NA-0309 delivered `formal/model_qsc_handshake_suite_id_bounded.py` and wired it
into `formal/run_model_checks.py`. The model covers:

- valid QHSM v2 Suite-2 accept;
- explicit legacy compatibility accept;
- legacy required-mode reject;
- unsupported, downgraded, stripped, mismatched, duplicate, unknown critical,
  noncanonical, malformed, inconsistent, transcript, and key-context rejects;
- no accepted-state mutation on rejects;
- no output or `recv_commit` on rejects;
- no secret/sentinel leak on rejects;
- no downgrade path from suite-required mode to compatibility mode; and
- deterministic reason labels.

The model is bounded and future-facing. It is not qsc runtime implementation,
wire-format implementation, or a complete cryptographic proof.

## Sources inspected

- `NEXT_ACTIONS.md` NA-0310 entry
- `tests/NA-0309_closeout_restore_na0310_testplan.md`
- `docs/governance/evidence/NA-0309_qsc_handshake_suite_id_formal_model_properties.md`
- `tests/NA-0309_qsc_handshake_suite_id_formal_model_testplan.md`
- `formal/model_qsc_handshake_suite_id_bounded.py`
- `formal/run_model_checks.py`
- `docs/governance/evidence/NA-0308_qsc_handshake_suite_id_formal_vector_design.md`
- `inputs/suite2/vectors/**`
- `tools/refimpl/quantumshield_refimpl/tests/**`
- `qsl/qsl-client/qsc/tests/na_0302_suite2_negotiation_cross_surface.rs`
- `qsl/qsl-client/qsc/tests/na_0303_handshake_activation_negotiation.rs`
- `qsl/qsl-client/qsc/tests/na_0304_handshake_suite_id_negotiation.rs`
- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/src/protocol_state/mod.rs`
- `TRACEABILITY.md`
- `DECISIONS.md`

## Vector schema

Vector artifact:

`inputs/suite2/vectors/qshield_qsc_handshake_suite_id_vectors_na0310.json`

Top-level metadata:

- `schema_version`
- `artifact_id`
- `source_na`
- `design_refs`
- `model_refs`
- `generated_at_or_last_updated`
- `statement`
- `vectors`

Per-vector fields:

- `vector_id`
- `category`
- `purpose`
- `qhsm_version`
- `compatibility_mode`
- `suite_required_mode`
- `frame_sequence`
- `protocol_version`
- `suite_id`
- `negotiated_parameters`
- `transcript_context_label`
- `key_context_label`
- `canonical_encoding_expected`
- `expected_result`
- `expected_reason_label`
- `mutation_expected`
- `recv_commit_expected`
- `output_expected`
- `secret_leak_expected`
- `model_property_refs`
- `refimpl_oracle_expectation`
- `qsc_harness_expectation`
- `notes`

The top-level statement explicitly classifies the fixture as design/test vector
material for future qsc suite-id semantics, not runtime implementation.

## Vector categories

The vector set contains one deterministic vector for each required category:

1. `valid_v2_suite2_parameter_block`
2. `legacy_v1_compatibility_allowed`
3. `legacy_v1_rejected_in_suite_required_mode`
4. `unsupported_suite_id`
5. `downgraded_suite_id`
6. `stripped_suite_id_parameter`
7. `mismatched_suite_id_A1_B1`
8. `mismatched_suite_id_B1_A2`
9. `duplicate_suite_id_parameter`
10. `unknown_critical_parameter`
11. `unknown_noncritical_parameter`
12. `noncanonical_parameter_order`
13. `malformed_parameter_length`
14. `inconsistent_protocol_version_suite_id`
15. `replayed_A1_with_suite_context`
16. `replayed_A2_with_suite_context`
17. `valid_suite2_with_transcript_binding`
18. `transcript_binding_mismatch`
19. `key_schedule_context_mismatch`
20. `missing_key_context_in_required_mode`

Reject vectors all require deterministic `REJECT_QSC_HS_*` reason labels,
`mutation_expected=false`, `recv_commit_expected=false`,
`output_expected=false`, and `secret_leak_expected=false`.

## Vector artifact path

`inputs/suite2/vectors/qshield_qsc_handshake_suite_id_vectors_na0310.json`

The artifact follows the existing Suite-2 vector directory convention used by
prior vector packs. It is not wired into production qsc code.

## Refimpl oracle path

`tools/refimpl/quantumshield_refimpl/tests/na_0310_qsc_suite_id_vector_oracle.rs`

The oracle is a refimpl integration test. It uses existing dev-dependencies
already present in `tools/refimpl/quantumshield_refimpl/Cargo.toml`; no Cargo
manifest or lockfile change is made.

## Refimpl oracle markers

The targeted oracle emits:

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

## Model/vector/oracle cross-check

The oracle checks:

- required top-level metadata;
- required per-vector fields;
- unique vector IDs;
- all 20 required categories are present exactly once;
- explicit Suite-2 accept vectors use `protocol_version=0x0500` and
  `suite_id=0x0002`;
- legacy compatibility is represented as `compatibility_accept`, not explicit
  suite-id admission;
- reject vectors carry deterministic reason labels and no mutation, output,
  recv_commit, or secret-leak expectations;
- transcript and key-context categories carry explicit labels;
- all `model_property_refs` are from the NA-0309 model marker/property set;
- each vector has a `proven_refimpl_oracle` expectation; and
- each qsc harness expectation remains `future_gate` and avoids claiming qsc
  runtime implementation.

## Coverage matrix

| Vector category | Model/property mapping | Oracle status | qsc status | Risk / next action |
| --- | --- | --- | --- | --- |
| valid_v2_suite2_parameter_block | valid v2, transcript, key context | PROVEN_REFIMPL_ORACLE | FUTURE_GATE | qsc v2 harness still needed |
| legacy_v1_compatibility_allowed | legacy compatibility, no downgrade path | PROVEN_REFIMPL_ORACLE | FUTURE_GATE | keep compatibility separate from admission |
| legacy_v1_rejected_in_suite_required_mode | legacy required reject, reject boundary | PROVEN_REFIMPL_ORACLE | FUTURE_GATE | future required-mode qsc harness |
| unsupported_suite_id | unsupported suite reject | PROVEN_REFIMPL_ORACLE | FUTURE_GATE | future v2 parser injection |
| downgraded_suite_id | downgrade reject, no downgrade path | PROVEN_REFIMPL_ORACLE | FUTURE_GATE | future downgrade qsc harness |
| stripped_suite_id_parameter | stripped/missing suite reject | PROVEN_REFIMPL_ORACLE | FUTURE_GATE | future stripped-block qsc harness |
| mismatched_suite_id_A1_B1 | mismatch reject | PROVEN_REFIMPL_ORACLE | FUTURE_GATE | future initiator-side mismatch harness |
| mismatched_suite_id_B1_A2 | mismatch reject | PROVEN_REFIMPL_ORACLE | FUTURE_GATE | future responder-side mismatch harness |
| duplicate_suite_id_parameter | duplicate reject | PROVEN_REFIMPL_ORACLE | FUTURE_GATE | future parser canonicality harness |
| unknown_critical_parameter | unknown critical, no leak | PROVEN_REFIMPL_ORACLE | FUTURE_GATE | future unknown-critical harness |
| unknown_noncritical_parameter | reject boundary, reason labels | PROVEN_REFIMPL_ORACLE | FUTURE_GATE | keep reject default visible |
| noncanonical_parameter_order | noncanonical reject | PROVEN_REFIMPL_ORACLE | FUTURE_GATE | future ordering harness |
| malformed_parameter_length | malformed reject, no leak | PROVEN_REFIMPL_ORACLE | FUTURE_GATE | future length parser harness |
| inconsistent_protocol_version_suite_id | inconsistent tuple property | PROVEN_REFIMPL_ORACLE | FUTURE_GATE | future tuple namespace harness |
| replayed_A1_with_suite_context | reject boundary | PROVEN_REFIMPL_ORACLE | FUTURE_GATE | future replay harness with explicit suite context |
| replayed_A2_with_suite_context | reject boundary | PROVEN_REFIMPL_ORACLE | FUTURE_GATE | future replay harness with explicit suite context |
| valid_suite2_with_transcript_binding | valid v2, transcript, key context | PROVEN_REFIMPL_ORACLE | FUTURE_GATE | future transcript harness |
| transcript_binding_mismatch | transcript reject | PROVEN_REFIMPL_ORACLE | FUTURE_GATE | future transcript mismatch harness |
| key_schedule_context_mismatch | key context reject | PROVEN_REFIMPL_ORACLE | FUTURE_GATE | future key-context harness |
| missing_key_context_in_required_mode | key context reject | PROVEN_REFIMPL_ORACLE | FUTURE_GATE | future required key-context harness |

## Qsc harness future expectations

Every vector keeps `qsc_harness_expectation.status` set to `future_gate`.
Current qsc `QHSM` v1 frames still lack an explicit suite-id parameter block,
so NA-0310 does not claim direct qsc admission proof. The next qsc lane should
define the harness requirements and test seam needed to exercise these vectors
without implementing production wire behavior by accident.

## Limitations

- The vector/oracle artifacts are deterministic design/test artifacts.
- The refimpl oracle validates fixture semantics and claim boundaries; it is
  not qsc runtime proof.
- Replay vectors map to the NA-0309 generic reject boundary and remain
  future-gated for explicit suite-context qsc harness work.
- Unknown noncritical parameters remain reject-by-default until a future
  directive explicitly authorizes ignore semantics with transcript coverage.
- No current persisted qsc Suite-2 state is presented as explicit suite-id
  admission evidence.

## No qsc runtime/protocol/wire implementation change proof

Changed implementation-adjacent files are limited to a refimpl test oracle and
a vector fixture. No files under `qsl/qsl-client/qsc/src/**`,
`qsl/qsl-client/qsc/tests/**`, `tools/refimpl/quantumshield_refimpl/src/**`,
`qsp/**`, protocol-core runtime source, crypto state-machine source,
production handshake implementation, key schedule implementation, Cargo
manifests, Cargo locks, workflows, qsl-server, qsl-attachments, qsc-desktop,
website/external repo, README, START_HERE, docs/public, branch-protection, or
public-safety configuration are changed.

## Backup-plan impact statement

No backup-plan update is required. NA-0310 changes only tracked qsl-protocol
inputs, refimpl test, governance, testplan, traceability, decision, and rolling
journal files under `/srv/qbuild/work`, which is already in the current local
backup scope. No non-rebuildable artifact is created outside the repository
worktree.

## Selected successor

Selected successor:

NA-0311 -- qsc Handshake Suite-ID qsc Harness Requirements and Test Seam Plan

Rationale: vector schema and refimpl oracle proof now exist. The next narrow
safe lane is to define the qsc harness requirements and test seam before any
parameter-block implementation authorization.

Rejected successors:

- qsc Handshake Suite-ID Parameter-Block Implementation Authorization:
  premature before qsc harness requirements and seam boundaries are frozen.
- qsc Handshake Suite-ID qsc Harness Implementation: premature before a
  requirements/test-seam plan freezes allowed files and stop conditions.
- qsc Handshake Suite-ID Vector/Oracle Blocker Resolution: not needed because
  the vector/oracle lane succeeded.

## Next recommendation

After NA-0310 merges and post-merge public-safety is green, close NA-0310 and
restore exactly one READY successor:

NA-0311 -- qsc Handshake Suite-ID qsc Harness Requirements and Test Seam Plan

NA-0311 should not implement qsc runtime or QHSM/QSP production wire changes
unless a future live directive explicitly authorizes exact files and semantics.
