Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-17
Replaces: n/a
Superseded-By: n/a

# NA-0306 qsc Handshake Suite-ID Wire-Format Authorization Plan

Directive: QSL-DIR-2026-05-17-114 / NA-0306

## Executive summary

NA-0306 is a planning and authorization lane only. It does not implement a qsc
suite-id field, does not change `QHSM` or QSP frame schema, does not change
qsc runtime source, does not change protocol or crypto behavior, and does not
change dependencies.

The evidence supports a future no-implementation design refinement lane before
any implementation lane:

NA-0307 -- qsc Handshake Suite-ID Compatibility and Transcript Binding Design

Recommended wire-format direction for that design lane: a version-gated,
bounded `QHSM` negotiated-parameter block that carries the Suite-2
`protocol_version` / `suite_id` tuple and any explicitly authorized future
negotiated parameters. That recommendation is not an implementation
authorization. NA-0307 must freeze compatibility, transcript binding,
key-schedule posture, reject taxonomy, and vector/model requirements before any
production wire-format implementation may be attempted.

## Live NA-0306 scope

Live `NEXT_ACTIONS.md` authorizes design/authorization evidence for the qsc
`QHSM` suite-id field or an exact blocker continuation. It requires explicit
implementation go/no-go and future proof markers.

Protected boundaries:

- no unsupported production, public-internet, external-review, anonymity,
  metadata-free, or untraceable claims;
- no silent protocol or crypto semantic changes;
- no qsc suite-id implementation unless a live directive explicitly authorizes
  exact files and semantics;
- no dependency, workflow, website, service implementation, docs/public,
  README, START_HERE, branch-protection, or public-safety configuration drift;
  and
- exactly one READY item remains NA-0306 until a separate closeout.

## Inherited NA-0305 decision

NA-0305 selected NA-0306 after NA-0304 proved that current qsc `QHSM`
A1/B1/A2 frames expose no explicit suite-id field. The inherited blocker is:

- unsupported suite-id admission input cannot be represented in current qsc
  handshake frames;
- downgrade-like suite-id admission input cannot be represented in current qsc
  handshake frames;
- malformed suite-id admission input cannot be represented in current qsc
  handshake frames; and
- persisted Suite-2 state after successful handshake activation is not explicit
  suite-id admission proof.

## Sources inspected

- `NEXT_ACTIONS.md` NA-0306 entry
- `docs/governance/evidence/NA-0305_qsc_handshake_suite_id_seam_authorization_plan.md`
- `tests/NA-0305_qsc_handshake_suite_id_seam_authorization_testplan.md`
- `tests/NA-0305_closeout_restore_na0306_testplan.md`
- `docs/governance/evidence/NA-0304_qsc_handshake_suite_id_negotiation_harness.md`
- `qsl/qsl-client/qsc/tests/na_0304_handshake_suite_id_negotiation.rs`
- `docs/governance/evidence/NA-0303_qsc_handshake_activation_negotiation_harness.md`
- `docs/governance/evidence/NA-0302_suite2_negotiation_vector_qsc_cross_surface_harness.md`
- `docs/governance/evidence/NA-0301_suite2_negotiation_downgrade_harness.md`
- `docs/governance/evidence/NA-0300_core_replay_reject_no_mutation_harness.md`
- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `tools/refimpl/quantumshield_refimpl/src/qsp/types.rs`
- `tools/refimpl/quantumshield_refimpl/src/qsp/handshake.rs`
- `tools/refimpl/quantumshield_refimpl/src/qsp/ratchet.rs`
- `tools/refimpl/quantumshield_refimpl/src/suite2/establish.rs`
- `tools/refimpl/quantumshield_refimpl/src/suite2/types.rs`
- `docs/canonical/DOC-CAN-003_QSP_Suite-2_True_Triple_Ratchet_v5.0.0_DRAFT.md`
- `docs/spec-closure/DOC-SCL-002_Shared_Schemas_Error_Reason_Code_Registry_v1.0_DRAFT.md`
- `formal/README.md`
- `formal/model_suite2_negotiation_bounded.py`
- `TRACEABILITY.md`
- `DECISIONS.md`

Discovery note: the directive listed generic top-level `qsp/**` and `qsc/**`
targets, but this checkout has no top-level `qsp` or `qsc` directories. The
actual qsc path is `qsl/qsl-client/qsc/**`; QSP/refimpl surfaces live under
`tools/refimpl/quantumshield_refimpl/src/qsp/**`.

## Current QHSM/QSP wire-format summary

Current qsc `QHSM` frames are private fixed-layout frames in
`qsl/qsl-client/qsc/src/handshake/mod.rs`:

- A1 / init: `QHSM` magic, `HS_VERSION`, `HS_TYPE_INIT`, 16-byte session id,
  KEM public key, signature public key, and 32-byte DH public key.
- B1 / response: `QHSM` magic, `HS_VERSION`, `HS_TYPE_RESP`, 16-byte session
  id, KEM ciphertext, transcript MAC, signature public key, signature, and
  32-byte DH public key.
- A2 / confirm: `QHSM` magic, `HS_VERSION`, `HS_TYPE_CONFIRM`, 16-byte session
  id, confirm MAC, and signature.

The parser is strict-length. Extra bytes appended to A1, B1, or A2 reject as a
generic frame length error before any suite-id policy can run.

Current refimpl/QSP handshake and message surfaces do carry explicit
`protocol_version` and `suite_id` fields. That proves the canonical QSP/refimpl
wire surfaces can express suite mismatch, downgrade-like, and AD mismatch
inputs. It does not prove qsc `QHSM` handshake admission because current qsc
does not expose those fields in A1/B1/A2.

## Current session-state suite summary

Current qsc handshake activation passes `SUITE2_PROTOCOL_VERSION` (`0x0500`)
and `SUITE2_SUITE_ID` (`0x0002`) constants into
`init_from_base_handshake` after the base handshake succeeds. NA-0304's qsc
harness decrypts persisted qsc session blobs and confirms both send and recv
states carry the Suite-2 tuple after successful activation.

That state proof remains useful, but it is post-admission state proof. It must
not be described as unsupported, downgrade-like, or malformed suite-id
admission proof.

## Wire-format option analysis

| Option | Feasibility | Likely future files | Production behavior change | Wire-format change | Key schedule / transcript posture | Compatibility impact | Reject taxonomy | Test requirements | Risk of overclaim | Recommendation |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1. `QHSM` fixed-field explicit `suite_id` | FEASIBLE, but incomplete alone | `qsl/qsl-client/qsc/src/handshake/mod.rs`, qsc tests, canonical/design docs, vectors | Yes | Yes | Must be transcript-bound; key-schedule effect must be decided before implementation | Legacy v1 frames need version-gated accept/reject policy | unsupported suite, downgrade-like suite, mismatch across frames, malformed length | A1/B1/A2 positive and negative admission harnesses | Medium if field is added without compatibility/key-schedule design | Do not implement next; keep as a simpler fallback if NA-0307 rejects a parameter block |
| 2. `QHSM` negotiated parameter block | FEASIBLE and preferred design direction | design doc first; later qsc handshake source/tests, vectors, formal/model updates if authorized | Yes when implemented | Yes | Must be canonical and transcript-bound; key-schedule effect must be explicitly decided or deferred with a stop | Cleanest version-gated migration path; legacy v1 can remain a separate explicit policy | unsupported, downgrade-like, duplicate, unknown, malformed, noncanonical, mismatch, missing required parameter | deterministic valid/negative qsc harnesses, refimpl/vector/model alignment | Low if staged as design before implementation | Select as recommended future wire-format shape, but require NA-0307 design before code |
| 3. `QHSM` extension/TLV block | PARTIAL | same as option 2 plus extension registry docs | Yes when implemented | Yes | Must define critical/noncritical handling and transcript binding | More extensible, but larger ambiguity surface | duplicate tag, unknown critical/noncritical, order, length, canonical encoding | broader parser/adversarial matrix | Medium to high because extension policy can hide downgrade gaps | Reject for next lane; too much parser surface before basic suite tuple is frozen |
| 4. Out-of-band or post-handshake suite assertion | BLOCKED for admission proof | session-state docs/tests only | No wire change if out-of-band only | No qsc handshake wire change | Does not authenticate admission unless separately bound | Keeps legacy frames but does not solve the missing field | cannot express qsc handshake suite-id admission rejects | only post-state checks | High | Reject as sufficient proof |
| 5. Test-only seam only | PARTIAL / BLOCKED for real admission | qsc tests or test helpers | No if tests only; yes if parser changes | No if tests only | Proves synthetic behavior only unless production parser uses same path | Avoids migration, but cannot prove real admission semantics | generic length reject unless production schema exists | synthetic negative tests only | High | Reject as the next lane by itself |
| 6. Formal/model-first design | PARTIAL and useful | `formal/**` and design/evidence docs if later authorized | No if model only | No | Can model reject/no-mutation and transcript assumptions, not parser reality | No direct migration answer | abstract reject taxonomy | model checks plus later vectors | Medium if called implementation proof | Use as an NA-0307 requirement or follow-on, not as a replacement for wire design |

## Selected option

Selected recommended wire-format direction:

Option 2 -- a version-gated, bounded `QHSM` negotiated-parameter block.

Selected successor type:

- no-implementation design refinement lane.

Selected exact successor:

NA-0307 -- qsc Handshake Suite-ID Compatibility and Transcript Binding Design

Rationale:

- current qsc `QHSM` frames lack the input surface needed for explicit
  suite-id admission proof;
- adding a fixed suite-id field directly would still leave compatibility,
  transcript-binding, and key-schedule questions unresolved;
- a bounded parameter block gives the smallest extensible shape for the Suite-2
  tuple and future negotiated values, but it must be frozen before code; and
- implementation before the transcript/key-schedule decision would risk either
  silent behavior drift or an overclaim that qsc suite-id admission is proven.

## Required future semantics

Any future wire-format implementation lane must be preceded by NA-0307 design
approval and must define these semantics exactly.

Required tuple:

- `protocol_version`: `u16`, big-endian, namespace value `0x0500` for Suite-2.
- `suite_id`: `u16`, big-endian, value `0x0002` for Suite-2 within the
  `protocol_version=0x0500` namespace.
- Suite id must always be interpreted inside its protocol-version namespace.

Recommended placement for future design:

- A1 carries the proposed negotiated tuple in a canonical parameter block.
- B1 echoes and confirms the exact tuple and canonical parameter bytes.
- A2 confirms the exact tuple and canonical parameter bytes.
- All three frames must reject mismatch before any accepted qsp session state
  or output commit is produced.

Canonical parameter-block requirements:

- bounded total length with a fixed maximum chosen in NA-0307;
- deterministic byte order and canonical integer widths;
- unique parameter identifiers if identifiers are used;
- no duplicate parameters;
- no alternative encodings for the same semantic value;
- unknown parameters reject unless NA-0307 defines an explicit criticality rule;
- missing required tuple parameters reject;
- malformed length, trailing bytes, underflow, overflow, or noncanonical
  encoding reject; and
- accepted canonical bytes are stable enough for vectors.

Transcript and key-schedule posture:

- suite tuple and canonical parameter bytes must be transcript-bound before any
  session state is committed;
- B1 and A2 MAC/signature coverage must include the same canonical bytes or a
  transcript hash derived from them;
- NA-0307 must explicitly decide whether the suite tuple also enters qsc
  handshake KDF inputs such as `QSC.HS.PQ`, `QSC.HS.DHINIT`,
  `QSC.HS.TRANSCRIPT`, `QSC.HS.CONFIRM`, and `QSC.HS.A2`;
- no implementation lane may proceed if that key-schedule decision remains
  ambiguous; and
- NA-0306 does not authorize any key schedule change.

Reject taxonomy:

- unsupported `protocol_version` reject;
- unsupported `suite_id` for a supported protocol version reject;
- downgrade-like tuple reject when Suite-2 is required;
- tuple mismatch across A1/B1/A2 reject;
- transcript or confirmation mismatch reject;
- duplicate/ambiguous field reject;
- unknown required/critical parameter reject;
- malformed length/canonical encoding reject;
- legacy-frame admission reject when explicit suite semantics are required;
- replay/duplicate pending-state reject remains fail-closed; and
- every reject must prove no accepted qsp session mutation, no `recv_commit`,
  no plaintext/output artifact, no panic/backtrace, and no secret or sentinel
  leakage.

Compatibility and migration posture:

- the future schema must be version-gated; current `HS_VERSION=1` frames have
  no explicit suite semantics;
- legacy v1 frames must not be silently treated as explicit Suite-2 admission
  proof;
- if a compatibility mode accepts v1 frames, it must be explicitly named,
  policy-gated, and excluded from explicit suite-id admission claims;
- if policy requires explicit Suite-2 admission, v1 frames must fail closed; and
- migration must not silently fall back from Suite-2 or from the explicit
  parameter-block schema.

Formal/model requirements:

- model unsupported, downgrade-like, malformed, duplicate, and mismatch
  parameter cases;
- model no-mutation on reject;
- model transcript/key-schedule assumptions selected by NA-0307;
- keep claim boundaries explicit if the model abstracts parser bytes; and
- run model checks in CI before implementation claims depend on them.

Vector/refimpl requirements:

- add canonical positive vectors for A1/B1/A2 with the selected parameter block;
- add negative vectors for unsupported suite, downgrade-like tuple, mismatch
  across frames, unknown/duplicate/malformed parameters, and legacy-v1 policy;
- keep QSP/refimpl vectors aligned with qsc `QHSM` semantics only where they
  share actual fields; and
- do not use QSP/refimpl suite fields as proof of qsc `QHSM` admission without
  qsc-specific harness evidence.

qsc harness requirements:

- valid explicit Suite-2 qsc handshake succeeds only with matching A1/B1/A2
  tuple and canonical parameter block;
- unsupported, downgrade-like, malformed, duplicate, unknown, and mismatch
  parameter inputs reject deterministically;
- rejected inputs do not create or mutate qsp session files or pending state;
- rejected inputs emit no B1/A2 or qsp output when rejection happens before
  response emission;
- no `recv_commit`, plaintext, route token, passphrase-env marker, panic, or
  backtrace appears in output; and
- accepted persisted state continues to carry `0x0500` / `0x0002`.

Demo and service boundary impact:

- no qsl-server or qsl-attachments implementation change is implied;
- no qsc-desktop or website change is implied;
- demo evidence may be updated only after a separately authorized
  implementation lane exists; and
- public docs must keep the current not-ready and external-review boundaries.

## Future NA-0307 likely files

Likely NA-0307 design files:

- `docs/design/qsc_handshake_suite_id_compatibility_transcript_binding.md`
- `docs/governance/evidence/NA-0307_qsc_handshake_suite_id_compatibility_transcript_binding_design.md`
- `tests/NA-0307_qsc_handshake_suite_id_compatibility_transcript_binding_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Future implementation files, not authorized by NA-0306:

- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/tests/na_0308_handshake_suite_id_wire_format.rs`
- `formal/**` only if an explicit model lane authorizes it
- `inputs/suite2/**` only if an explicit vector lane authorizes it
- canonical spec updates only if the design classifies the qsc `QHSM` change as
  protocol-significant beyond the local qsc handshake surface

## Future NA-0307 expected markers

NA-0307 should define design markers such as:

- `NA0307_QSC_SUITE_ID_COMPATIBILITY_DESIGN_OK`
- `NA0307_QSC_SUITE_ID_TRANSCRIPT_BINDING_DECISION_OK`
- `NA0307_QSC_SUITE_ID_KEY_SCHEDULE_POSTURE_OK`
- `NA0307_QSC_SUITE_ID_REJECT_TAXONOMY_OK`
- `NA0307_NO_IMPLEMENTATION_CHANGE_OK`

A later implementation lane should require markers such as:

- `NA0308_QSC_EXPLICIT_SUITE_ID_HANDSHAKE_OK`
- `NA0308_UNSUPPORTED_SUITE_ID_REJECT_OK`
- `NA0308_DOWNGRADE_SUITE_ID_REJECT_OK`
- `NA0308_MALFORMED_SUITE_ID_REJECT_OK`
- `NA0308_QSC_SUITE_ID_MISMATCH_REJECT_OK`
- `NA0308_QSC_NO_MUTATION_ON_REJECT_OK`
- `NA0308_QSC_NO_RECV_COMMIT_ON_REJECT_OK`

## Stop conditions

Future work must STOP if:

- it would implement `QHSM` frame/schema changes before NA-0307 freezes
  compatibility and transcript/key-schedule posture;
- it would treat v1 legacy frames as explicit suite-id admission proof;
- it would rely on persisted Suite-2 state as direct admission proof;
- it would treat generic length rejects from appended bytes as
  suite-id-specific rejects;
- it would change crypto state machines, key schedule, or production handshake
  semantics without explicit authorization;
- it would weaken fail-closed rejection, no-mutation behavior, or
  public-safety;
- it would change dependencies, workflows, branch protection, services,
  website, README, START_HERE, or docs/public outside an explicit scope; or
- required local validation or protected CI fails conclusively.

## Claim boundaries

Allowed claims:

- current qsc `QHSM` handshake frames lack an explicit suite-id field;
- current qsc successful handshakes persist Suite-2 session state;
- explicit qsc handshake suite-id admission proof requires a future authorized
  wire-format or design/test seam; and
- NA-0306 selected a no-implementation NA-0307 design successor.

Forbidden claims:

- unsupported, downgrade-like, or malformed qsc handshake suite-id admission is
  already tested directly;
- persisted Suite-2 state is explicit suite-id admission proof;
- a test-only appended field proves real admission semantics;
- qsc is production-ready or public-internet ready;
- external review is complete;
- the system provides anonymity, metadata-free behavior, or untraceability; or
- the future parameter block is implementation-ready without NA-0307 decisions.

## No implementation change proof

NA-0306 changes only governance, evidence, testplan, traceability, and rolling
journal documents. It does not change:

- qsc runtime source under `qsl/qsl-client/qsc/src/**`;
- qsc runtime tests under `qsl/qsl-client/qsc/tests/**`;
- QSP/refimpl/protocol-core implementation source;
- crypto state machines, key schedules, production handshake implementation,
  or QSP wire-format implementation;
- Cargo manifests or locks;
- workflows or scripts;
- qsl-server, qsl-attachments, qsc-desktop, apps, formal models, inputs,
  tools/refimpl, service implementation paths, website/external website,
  README, START_HERE, docs/public, branch protection, or public-safety
  configuration.

## Next recommendation

After the NA-0306 PR merges and post-merge public-safety is green, close out
NA-0306 and restore exactly one READY successor:

NA-0307 -- qsc Handshake Suite-ID Compatibility and Transcript Binding Design

NA-0307 must remain a no-implementation design lane unless a future live
directive explicitly changes scope.
