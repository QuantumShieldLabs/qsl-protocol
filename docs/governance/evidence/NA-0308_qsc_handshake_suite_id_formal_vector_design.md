Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-17
Replaces: n/a
Superseded-By: n/a

# NA-0308 qsc Handshake Suite-ID Formal Model and Vector Design

Directive: QSL-DIR-2026-05-17-116 / NA-0308

## Executive summary

NA-0308 is a design-only lane. It records the formal/model properties, vector
schema, vector categories, refimpl oracle requirements, qsc harness
requirements, fail-closed stop conditions, and coverage matrix needed before
any future explicit qsc handshake suite-id implementation authorization.

NA-0308 does not implement a `QHSM` suite-id field, does not change QSP frame
schema, does not change qsc runtime source, does not change formal model code,
does not add executable vector fixtures, does not change crypto state-machine
or key schedule behavior, and does not change dependencies.

Selected successor:

NA-0309 -- qsc Handshake Suite-ID Formal Model Properties

Rationale: executable formal/model properties are the smallest next safe lane.
They can freeze the abstract state-machine, compatibility-mode, transcript
context, key-schedule context, and reject/no-mutation boundaries before vector
fixtures or qsc parser work depend on exact bytes.

## Live NA-0308 scope

Live `NEXT_ACTIONS.md` authorizes bounded formal/model, vector-design,
governance, and testplan artifacts. It forbids implementation by default:

- no protocol or wire semantic change;
- no crypto state-machine, key schedule, or production handshake
  implementation change;
- no qsc suite-id implementation unless a live directive explicitly authorizes
  exact files and semantics;
- no dependency, workflow, website, service implementation, docs/public,
  README, START_HERE, branch-protection, or public-safety configuration drift;
  and
- exactly one READY item remains NA-0308 until a separate closeout.

The live scope matches this directive. NA-0308 is a no-implementation formal
and vector design lane.

## Inherited NA-0307 conclusions

NA-0307 selected the compatibility, transcript-binding, and key-schedule
posture for future explicit qsc handshake suite-id semantics:

- future explicit qsc suite-id admission uses a version-gated `QHSM` v2
  negotiated-parameter block;
- `QHSM` v1 frames have no explicit suite-id semantics;
- v1 compatibility acceptance may exist only in an explicitly named
  compatibility mode and never as explicit suite-id admission evidence;
- suite-id-required mode rejects v1 frames, missing suite context, stripped
  suite context, unknown parameters under the selected reject policy,
  duplicate suite parameters, malformed blocks, noncanonical order,
  inconsistent tuples, and A1/B1/A2 mismatch;
- A1 is authoritative only when B1 and A2 echo or confirm the byte-exact
  canonical parameter-block bytes;
- the complete canonical parameter-block bytes must be transcript-bound before
  any B1 MAC, B1 signature, A2 confirm MAC, A2 signature, or accepted qsp
  session state; and
- the future negotiated context must feed qsc handshake KDF/context inputs
  before accepted session state is committed.

Persisted Suite-2 session state remains useful accepted-state evidence, but it
is not explicit qsc handshake suite-id admission evidence.

## Sources inspected

- `NEXT_ACTIONS.md` NA-0308 entry
- `tests/NA-0307_closeout_restore_na0308_testplan.md`
- `docs/governance/evidence/NA-0307_qsc_handshake_suite_id_compatibility_transcript_design.md`
- `tests/NA-0307_qsc_handshake_suite_id_compatibility_transcript_testplan.md`
- `docs/governance/evidence/NA-0306_qsc_handshake_suite_id_wire_format_authorization_plan.md`
- `docs/governance/evidence/NA-0305_qsc_handshake_suite_id_seam_authorization_plan.md`
- `docs/governance/evidence/NA-0304_qsc_handshake_suite_id_negotiation_harness.md`
- `docs/governance/evidence/NA-0303_qsc_handshake_activation_negotiation_harness.md`
- `docs/governance/evidence/NA-0302_suite2_negotiation_vector_qsc_cross_surface_harness.md`
- `docs/governance/evidence/NA-0301_suite2_negotiation_downgrade_harness.md`
- `docs/governance/evidence/NA-0300_core_replay_reject_no_mutation_harness.md`
- `formal/README.md`
- `formal/model_scka_bounded.py`
- `formal/model_suite2_negotiation_bounded.py`
- `formal/run_model_checks.py`
- `inputs/suite2/vectors/README.md`
- `inputs/suite2/vectors/qshield_suite2_negotiation_vectors_na0302.json`
- `tools/refimpl/quantumshield_refimpl/tests/na_0301_suite2_negotiation_downgrade.rs`
- `tools/refimpl/quantumshield_refimpl/tests/na_0302_suite2_negotiation_vectors.rs`
- `qsl/qsl-client/qsc/tests/na_0302_suite2_negotiation_cross_surface.rs`
- `qsl/qsl-client/qsc/tests/na_0303_handshake_activation_negotiation.rs`
- `qsl/qsl-client/qsc/tests/na_0304_handshake_suite_id_negotiation.rs`
- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/src/protocol_state/mod.rs`
- `tools/refimpl/quantumshield_refimpl/src/qsp/types.rs`
- `tools/refimpl/quantumshield_refimpl/src/qsp/handshake.rs`
- `tools/refimpl/quantumshield_refimpl/src/suite2/types.rs`
- `TRACEABILITY.md`
- `DECISIONS.md`
- `GOALS.md`
- `PROJECT_CHARTER.md`
- `CHECKLIST_PROTOCOL_CHANGE.md`

Discovery note: this checkout has no top-level `qsp/` or `qsc/` directory.
The qsc client path is `qsl/qsl-client/qsc/**`; QSP/refimpl surfaces live under
`tools/refimpl/quantumshield_refimpl/src/qsp/**`.

## Current formal/model coverage

Current executable formal coverage includes:

- bounded SCKA control-plane invariants for ADV monotonicity, one-time
  consumption, tombstones, reject/no-state-change, and transactional commit;
  and
- a bounded Suite-2 negotiation/downgrade model for mutually supported
  Suite-2, weaker committed suite rejects, inconsistent capability commitment
  rejects, inconsistent negotiated-suite transcript-view rejects, and
  accepted/durable negotiation no-mutation on rejects.

Current formal coverage does not yet model qsc `QHSM` A1/B1/A2
parameter-block semantics, legacy compatibility mode, suite-id-required mode,
duplicate/unknown/malformed parameter cases, byte-exact transcript context, or
future qsc handshake KDF/context binding.

## Current vector/refimpl coverage

Current Suite-2 vector coverage includes KDF, transcript, downgrade,
negotiation, parse, boundary receive, out-of-order/replay, crash/restart, SCKA,
KT, establish, interop, and hybrid message-key categories. NA-0301 and NA-0302
added refimpl and qsc receive-path negotiation coverage for valid Suite-2,
unsupported suite, downgrade-like version, unsupported algorithm or parameter,
malformed negotiation input, deterministic reject reasons, no mutation on
rejects, and no panic or sentinel leakage.

Current vector/refimpl coverage does not yet define qsc `QHSM` v2
parameter-block fixtures, A1/B1/A2 byte-exact echo/confirm expectations,
legacy v1 compatibility vectors, or key-schedule context labels for the future
qsc handshake suite context.

## Current qsc harness coverage

Current qsc harness coverage includes:

- NA-0302 qsc receive-path cross-surface negotiation mutation rejects without
  persisted qsc session mutation;
- NA-0303 qsc handshake activation version/type/malformed/replay rejects,
  no accepted qsp session mutation on rejects, no `recv_commit`, no panic, and
  no secret/sentinel leakage; and
- NA-0304 qsc handshake suite-id blocker evidence proving successful activation
  persists Suite-2 state with `protocol_version=0x0500` and `suite_id=0x0002`,
  while A1/B1/A2 `QHSM` frames expose no explicit suite-id field.

Current qsc harness coverage cannot express unsupported, downgraded, stripped,
mismatched, duplicate, or malformed qsc handshake suite-id admission input
because current `QHSM` v1 frames have no field for that input.

## Formal/model property design

Future executable model work should use a bounded model named
`model_qsc_handshake_suite_id_bounded`. The model should be crypto-agnostic and
state-machine focused. It should treat transcript and key-schedule binding as
explicit context values rather than proving cryptographic security.

| ID | Model property | Preconditions | Expected result | State-mutation boundary | NA-0307 relation | Future executable test |
| --- | --- | --- | --- | --- | --- | --- |
| P1 | Suite context present and canonical in negotiated parameter context | `QHSM` v2, suite-id-required mode, exactly one critical suite context parameter with `protocol_version=0x0500` and `suite_id=0x0002` | Acceptable only if A1/B1/A2 context bytes match | Accepted session state may mutate only after canonical context is confirmed | Implements selected v2 parameter-block posture | `python3 formal/run_model_checks.py` with qsc suite-id model enabled |
| P2 | Transcript includes suite context | Valid v2 path with canonical block | Transcript context records byte-exact block | No accepted state until transcript context matches | Implements byte-exact transcript binding | Same model command |
| P3 | Key schedule context includes suite context or explicit equivalent | Valid v2 path with canonical block | Key-schedule context label/hash includes canonical block or selected equivalent | No accepted state until KDF context is present | Implements NA-0307 key-schedule posture | Same model command |
| P4 | Unsupported suite id rejects before state mutation | v2 block names unsupported `suite_id` under supported protocol version | Reject with deterministic unsupported-suite reason | Parse-local scratch only | Covers unsupported tuple taxonomy | Same model command |
| P5 | Downgraded or stripped suite id rejects before state mutation | Suite-2 required and attacker selects weaker tuple or removes suite parameter | Reject with deterministic downgrade or missing-suite reason | Parse-local scratch only | Covers no silent fallback | Same model command |
| P6 | Mismatched suite id across A1/B1/A2 rejects before state mutation | A1, B1, or A2 carries different suite context bytes | Reject at first mismatch | No accepted initiator/responder state after mismatch | Covers byte-exact echo/confirm rule | Same model command |
| P7 | Duplicate suite parameter rejects | v2 block repeats suite context parameter | Reject as noncanonical duplicate | Parse-local scratch only | Covers canonical parameter-block rule | Same model command |
| P8 | Unknown critical parameter rejects | v2 block contains unknown critical parameter | Reject as unknown critical | Parse-local scratch only | Covers selected criticality posture | Same model command |
| P9 | Noncanonical parameter order rejects | v2 block parameter ids not strictly increasing | Reject as noncanonical order | Parse-local scratch only | Covers canonical order rule | Same model command |
| P10 | Malformed length rejects | v2 block length underflows, overflows, exceeds bound, or leaves trailing bytes | Reject as malformed length | Parse-local scratch only | Covers bounded parser posture | Same model command |
| P11 | Legacy frame rejects in suite-id-required mode | `QHSM` v1 observed while explicit suite context is required | Reject as legacy-not-explicit | No accepted qsp session | Covers v1 required-mode rule | Same model command |
| P12 | Legacy compatibility mode is explicit and cannot silently downgrade required mode | Policy is compatibility mode or suite-id-required mode | v1 accepted only in compatibility mode; required mode never falls back | Required-mode state does not mutate on v1 | Covers compatibility boundary | Same model command |
| P13 | Reject path produces no `recv_commit` or output | Any invalid parameter case | Reject outcome has no receive commit and no handshake output after reject | Output flags remain false | Covers output boundary | Same model command |
| P14 | Reject path leaks no secret/plaintext/sentinel | Any invalid parameter case with sentinel-bearing attacker bytes | Reject reason is deterministic label only | No output buffer contains sentinel or raw parameter dump | Covers no-leak boundary | Same model command |
| P15 | Accepted valid path preserves current Suite-2 semantics | Canonical v2 Suite-2 block across A1/B1/A2 | Accept Suite-2 state equivalent to current Suite-2 tuple semantics plus explicit context | Accepted state mutates only after A2 confirm for responder and B1 acceptance for initiator | Preserves existing Suite-2 semantics | Same model command |

The model must publish its limits: bounded exploration is not cryptographic
security proof, parser memory-safety proof, or qsc runtime proof.

## Vector schema design

Future vector fixtures should use a qsc-specific vector set, for example:

`inputs/suite2/vectors/qshield_qsc_handshake_suite_id_vectors_na0310.json`

Required top-level fields:

- `format`: `QSHIELD-QSC-HANDSHAKE-SUITE-ID-VECTOR-SET-1`
- `schema_version`
- `generated_at`
- `source`
- `protocol`
- `registries`
- `vectors`

Required per-vector fields:

- `vector_id`
- `purpose`
- `qhsm_version`
- `frame_sequence`
- `negotiated_parameters`
- `suite_id`
- `protocol_version`
- `transcript_binding_expected`
- `key_schedule_context_expected`
- `expected_result`
- `expected_reject_code` or `reason_label`
- `mutation_expected`
- `recv_commit_expected`
- `secret_leak_expected`
- `canonical_encoding_expected`
- `compatibility_mode`
- `legacy_mode`
- `notes`

Field constraints:

- `protocol_version` and `suite_id` use canonical fixed-width values such as
  `0x0500` and `0x0002`.
- `negotiated_parameters` records the canonical parameter-block bytes and the
  parsed semantic tuple.
- `frame_sequence` names A1/B1/A2 and whether each frame carries, echoes, or
  omits the canonical block.
- `mutation_expected` is `no` for every reject vector.
- `recv_commit_expected` is `no` for every reject vector.
- `secret_leak_expected` is always `no`.
- `compatibility_mode` and `legacy_mode` must be explicit booleans or enum
  labels, never implicit defaults.

## Vector category design

| Category | Expected result | Refimpl oracle requirement | qsc harness requirement | Formal/model property |
| --- | --- | --- | --- | --- |
| valid_v2_suite2_parameter_block | Accept explicit Suite-2 context | Parse canonical block and compute expected context labels | Future qsc v2 valid path establishes Suite-2 state | P1, P2, P3, P15 |
| legacy_v1_compatibility_allowed | Accept only as compatibility, not admission evidence | Label result as compatibility-only | Future qsc compatibility test keeps v1 separate from explicit admission | P12 |
| legacy_v1_rejected_in_suite_required_mode | Reject before state mutation | Reason `REJECT_QSC_HS_LEGACY_REQUIRED` | qsc rejects v1 when suite context is required | P11, P12, P13 |
| unsupported_suite_id | Reject before state mutation | Reason `REJECT_QSC_HS_SUITE_UNSUPPORTED` | qsc emits no B1/A2 or `recv_commit` | P4, P13, P14 |
| downgraded_suite_id | Reject before state mutation | Reason `REJECT_QSC_HS_DOWNGRADE` | qsc prevents fallback to weaker tuple | P5, P13, P14 |
| stripped_suite_id_parameter | Reject as missing or transcript mismatch | Reason `REJECT_QSC_HS_SUITE_MISSING` or mismatch label | qsc rejects stripped A1/B1/A2 | P5, P6, P13 |
| mismatched_suite_id_A1_B1 | Reject at B1 processing | Reason `REJECT_QSC_HS_CONTEXT_MISMATCH` | Initiator does not commit accepted state | P6, P13 |
| mismatched_suite_id_B1_A2 | Reject at A2 processing | Reason `REJECT_QSC_HS_CONTEXT_MISMATCH` | Responder does not commit accepted state | P6, P13 |
| duplicate_suite_id_parameter | Reject as noncanonical | Reason `REJECT_QSC_HS_DUPLICATE_PARAMETER` | qsc parser rejects before response/output | P7, P13 |
| unknown_critical_parameter | Reject | Reason `REJECT_QSC_HS_UNKNOWN_CRITICAL` | qsc emits deterministic label without dumping value | P8, P14 |
| unknown_noncritical_parameter | Reject unless a later directive authorizes ignore semantics | Reason `REJECT_QSC_HS_UNKNOWN_PARAMETER` | qsc tests selected policy explicitly | P8, P13 |
| noncanonical_parameter_order | Reject | Reason `REJECT_QSC_HS_NONCANONICAL_ORDER` | qsc rejects before output | P9, P13 |
| malformed_parameter_length | Reject | Reason `REJECT_QSC_HS_MALFORMED_LENGTH` | qsc rejects underflow/overflow/trailing bytes | P10, P13, P14 |
| inconsistent_protocol_version_suite_id | Reject | Reason `REJECT_QSC_HS_TUPLE_UNSUPPORTED` | qsc rejects tuple namespace mismatch | P4, P5, P13 |
| replayed_A1_or_A2_with_suite_context | Reject duplicate/replay before mutation | Reason `REJECT_QSC_HS_REPLAY` | qsc preserves existing replay no-mutation behavior | P13, P14 |
| valid_suite2_with_transcript_binding | Accept | Oracle computes transcript context label | qsc proves byte-exact binding in accepted path | P2, P15 |
| transcript_binding_mismatch | Reject | Oracle detects label mismatch | qsc rejects mismatch with no mutation | P2, P6, P13 |
| key_schedule_context_mismatch | Reject or STOP if implementation cannot represent it | Oracle computes selected key context label | qsc rejects mismatch before accepted state | P3, P13 |

If a future directive authorizes unknown noncritical ignore semantics, it must
also require transcript coverage for ignored bytes and update this design by
decision. NA-0308 selects reject as the default.

## Refimpl oracle requirements

Future refimpl oracle work should:

- parse the qsc suite-id vector schema;
- validate canonical parameter encoding, including block length, parameter id
  order, duplicate handling, flags, value lengths, and required suite context;
- compute an expected transcript/context label from the canonical block when
  safe for a test oracle;
- compute a selected key-schedule context label or hash placeholder without
  exposing secret material;
- validate accepted and rejected outcomes;
- expose deterministic reason labels without raw secret-bearing input dumps;
- assert `mutation_expected=no`, `recv_commit_expected=no`, and
  `secret_leak_expected=no` for reject vectors; and
- run as a bounded test/harness with no dependency or production behavior
  change unless separately authorized.

## qsc harness requirements

Future qsc harness work should:

- construct future `QHSM` v2 frames or negotiated-parameter fixtures only after
  implementation or explicit test-seam scope is authorized;
- prove the valid path produces established Suite-2 state;
- prove every invalid path rejects before accepted state mutation;
- prove invalid paths produce no `recv_commit`, qsp unpack output, plaintext,
  or post-reject handshake response;
- assert deterministic reason labels;
- scan output for route token, passphrase marker, plaintext, sentinel, panic,
  and backtrace leakage;
- test compatibility mode and suite-id-required mode separately;
- test old-parser/new-frame behavior if an old parser is available;
- test new-parser/old-frame behavior after the new parser exists; and
- assert no panic/backtrace on all reject paths.

## Formal/model requirements

Future formal/model work should:

- define state-machine inputs for qsc suite-id parameters;
- model A1/B1/A2 frame roles and compatibility policy;
- model transcript context and key-schedule context as explicit required
  state variables;
- assert no downgrade path from suite-id-required mode to legacy mode;
- assert no mutation on unsupported, downgraded, stripped, mismatched,
  duplicate, unknown, noncanonical, malformed, replayed, and legacy-required
  cases;
- assert no output flags on invalid cases; and
- assert accepted state only after a canonical valid sequence.

## Fail-closed reject/no-mutation taxonomy

| Reject class | Required reason label | Mutation boundary | Output boundary | Leak boundary |
| --- | --- | --- | --- | --- |
| Unsupported suite id | `REJECT_QSC_HS_SUITE_UNSUPPORTED` | Parse-local scratch only | No B1/A2, no `recv_commit` | No raw tuple dump |
| Downgraded suite id | `REJECT_QSC_HS_DOWNGRADE` | No accepted state mutation | No response after reject | No sentinel/plaintext |
| Missing or stripped suite context | `REJECT_QSC_HS_SUITE_MISSING` | No accepted state mutation | No response after reject | No raw frame dump |
| A1/B1/A2 mismatch | `REJECT_QSC_HS_CONTEXT_MISMATCH` | No accepted initiator/responder state mutation | No commit/output after mismatch | No peer secret material |
| Duplicate parameter | `REJECT_QSC_HS_DUPLICATE_PARAMETER` | Parse-local scratch only | No response | No raw value dump |
| Unknown critical parameter | `REJECT_QSC_HS_UNKNOWN_CRITICAL` | Parse-local scratch only | No response | No raw value dump |
| Unknown noncritical parameter | `REJECT_QSC_HS_UNKNOWN_PARAMETER` | Parse-local scratch only | No response | No raw value dump |
| Noncanonical order | `REJECT_QSC_HS_NONCANONICAL_ORDER` | Parse-local scratch only | No response | No raw value dump |
| Malformed length | `REJECT_QSC_HS_MALFORMED_LENGTH` | Parse-local scratch only | No response | No raw byte dump |
| Legacy in required mode | `REJECT_QSC_HS_LEGACY_REQUIRED` | No accepted qsp session | No B1/A2 or `recv_commit` | No overclaim |
| Replay with suite context | `REJECT_QSC_HS_REPLAY` | No duplicate pending or accepted state | No duplicate output | No sentinel/plaintext |

## Coverage matrix

| Item | Vector category | Refimpl oracle | qsc harness | Future artifact | Status | Risk | Needed next action |
| --- | --- | --- | --- | --- | --- | --- | --- |
| P1 canonical suite context | valid_v2_suite2_parameter_block | canonical parser | valid v2 path | `formal/model_qsc_handshake_suite_id_bounded.py` | READY_FOR_MODEL | Medium | Add executable model first |
| P2 transcript context | valid_suite2_with_transcript_binding, transcript_binding_mismatch | context label calculator | transcript binding harness | formal model then vectors | READY_FOR_MODEL | High | Model required context before vectors |
| P3 key context | key_schedule_context_mismatch | key context label placeholder | key context harness | formal model then vectors | READY_FOR_MODEL | High | Model STOP condition and required variable |
| P4 unsupported suite | unsupported_suite_id | deterministic reason | no mutation/output | formal model, vectors, qsc tests | READY_FOR_MODEL | Medium | Model then vector schema |
| P5 downgrade/stripped | downgraded_suite_id, stripped_suite_id_parameter | deterministic reason | no fallback | formal model, vectors, qsc tests | READY_FOR_MODEL | High | Model no fallback |
| P6 mismatch | mismatched_suite_id_A1_B1, mismatched_suite_id_B1_A2 | mismatch reason | initiator/responder no commit | formal model, vectors, qsc tests | READY_FOR_MODEL | High | Model frame sequence states |
| P7 duplicate | duplicate_suite_id_parameter | duplicate reason | parse-local reject | formal model, vectors | READY_FOR_MODEL | Medium | Model canonical parameter set |
| P8 unknown parameter | unknown_critical_parameter, unknown_noncritical_parameter | unknown reason | parse-local reject | formal model, vectors | READY_FOR_MODEL | Medium | Model selected reject policy |
| P9 order | noncanonical_parameter_order | order reason | parse-local reject | formal model, vectors | READY_FOR_MODEL | Medium | Model strict order |
| P10 length | malformed_parameter_length | length reason | parse-local reject | formal model, vectors | READY_FOR_MODEL | Medium | Model bounded length class |
| P11 legacy required | legacy_v1_rejected_in_suite_required_mode | legacy reason | required-mode reject | formal model, vectors | READY_FOR_MODEL | High | Model policy separation |
| P12 compatibility | legacy_v1_compatibility_allowed | compatibility-only label | separate qsc compatibility harness | formal model, vectors | READY_FOR_MODEL | High | Model no implicit downgrade |
| P13 no output | all reject categories | expected output flags | no `recv_commit`/B1/A2 | formal model, qsc tests | READY_FOR_MODEL | High | Model reject output flags |
| P14 no leak | all reject categories | no raw dump | output scan | refimpl/qsc tests | READY_FOR_VECTOR | Medium | Define sentinel fields in vector schema |
| P15 accepted Suite-2 | valid v2 categories | selected output | persisted Suite-2 state | model, qsc implementation tests | READY_FOR_MODEL | Medium | Model accepted state equivalence |
| Vector schema | all categories | future parser | future fixture loader | `inputs/suite2/vectors/...na0310.json` | READY_FOR_VECTOR | Medium | Add after model property lane |
| Refimpl oracle | all categories | future bounded oracle | n/a | `tools/refimpl/.../tests/na0310...rs` | DESIGN_ONLY | Medium | Wait for model and schema |
| qsc harness | all categories | compare oracle output | future v2 `QHSM` tests | `qsl/qsl-client/qsc/tests/...` | DESIGN_ONLY | High | Wait for implementation authorization |

## Selected successor

Selected exact successor:

NA-0309 -- qsc Handshake Suite-ID Formal Model Properties

Successor type: formal/model executable lane.

This is selected over a vector/refimpl lane because the formal model can define
the state-machine and policy boundary without constructing byte fixtures that
could imply an implemented qsc parser. It is selected over implementation
authorization because no executable model yet freezes the no-mutation,
compatibility, transcript-context, and key-context properties. It is selected
over blocker continuation because no design blocker remains after NA-0308; the
next missing artifact is executable model coverage.

## Future NA-0309 likely files

Likely files for NA-0309, if explicitly authorized:

- `formal/model_qsc_handshake_suite_id_bounded.py`
- `formal/run_model_checks.py`
- `formal/README.md`
- `docs/governance/evidence/NA-0309_qsc_handshake_suite_id_formal_model_properties.md`
- `tests/NA-0309_qsc_handshake_suite_id_formal_model_properties_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

NA-0309 should not implement qsc runtime, QHSM wire-format, production
handshake, key schedule, QSP wire-format, refimpl oracle, or vector pack work
unless its live scope explicitly authorizes a narrower addition.

## Future NA-0309 expected markers

Expected future model markers:

- `NA0309_QSC_SUITE_ID_MODEL_CANONICAL_CONTEXT_OK`
- `NA0309_QSC_SUITE_ID_MODEL_TRANSCRIPT_CONTEXT_OK`
- `NA0309_QSC_SUITE_ID_MODEL_KDF_CONTEXT_OK`
- `NA0309_QSC_SUITE_ID_MODEL_LEGACY_REQUIRED_REJECT_OK`
- `NA0309_QSC_SUITE_ID_MODEL_COMPATIBILITY_POLICY_OK`
- `NA0309_QSC_SUITE_ID_MODEL_REJECT_NO_MUTATION_OK`
- `NA0309_QSC_SUITE_ID_MODEL_REJECT_NO_OUTPUT_OK`
- `NA0309_NO_QSC_RUNTIME_IMPLEMENTATION_OK`
- `NA0309_CLAIM_BOUNDARY_OK`

## Stop conditions

Future work must STOP if:

- it would implement `QHSM` schema or production qsc behavior before an
  explicit implementation directive;
- it would change crypto state-machine, key schedule, production handshake, or
  QSP wire-format behavior without exact authorization;
- it would treat v1 legacy compatibility as explicit suite-id admission;
- it would silently fallback from suite-id-required mode to legacy mode;
- it would present persisted Suite-2 state as explicit admission evidence;
- it would leave transcript or key-schedule context binding ambiguous before
  implementation;
- it cannot model or vectorize downgrade, stripped, mismatch, duplicate,
  unknown, noncanonical, malformed, replay, or legacy-required cases;
- it would hide known qsc suite-id readiness gaps;
- it would change dependencies, workflows, branch protection, public-safety
  configuration, service implementation paths, website/external repo, README,
  START_HERE, or docs/public outside explicit scope; or
- required CI or public-safety fails conclusively.

## Claim boundaries

Allowed claims:

- NA-0308 records a design package for future qsc handshake suite-id model and
  vector work.
- Current qsc `QHSM` v1 frames lack an explicit suite-id field.
- Future explicit qsc handshake suite-id admission remains unimplemented.
- NA-0309 should add executable formal/model properties before vectors or qsc
  implementation.

Forbidden or unsupported claims:

- qsc suite-id wire-format behavior is implemented by NA-0308.
- design vectors are runtime behavior.
- the model design is completed executable proof.
- persisted Suite-2 state is explicit qsc handshake suite-id admission
  evidence.
- external review is finished.
- the system provides anonymity, metadata-free behavior, or untraceability.
- the system is ready for unrestricted deployment.

## No implementation change proof

NA-0308 changes only governance, evidence, testplan, traceability, and rolling
journal documents. It does not change:

- qsc runtime source under `qsl/qsl-client/qsc/src/**`;
- qsc runtime tests under `qsl/qsl-client/qsc/tests/**`;
- formal model implementation files under `formal/**`;
- vector input files under `inputs/**`;
- refimpl source or tests under `tools/refimpl/**`;
- QSP/refimpl/protocol-core implementation source;
- crypto state-machine, key schedule, production handshake implementation, or
  QSP wire-format implementation;
- Cargo manifests or locks;
- workflows or scripts;
- qsl-server, qsl-attachments, qsc-desktop, apps, website/external website,
  README, START_HERE, docs/public, branch protection, or public-safety
  configuration.

## Backup-plan impact statement

No backup-plan update is required. NA-0308 adds only qsl-protocol governance
evidence and testplan files under existing durable repository paths and updates
existing governance logs. It creates no non-rebuildable artifact outside the
current repository evidence scope.

## Next recommendation

After NA-0308 merges and public-safety is green, close NA-0308 and restore
exactly one READY successor:

NA-0309 -- qsc Handshake Suite-ID Formal Model Properties

NA-0309 should add executable bounded formal/model properties while preserving
the no-implementation boundary for qsc runtime, QHSM wire-format, production
handshake, key schedule, and QSP wire-format behavior.
