Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-17
Replaces: n/a
Superseded-By: n/a

# NA-0307 qsc Handshake Suite-ID Compatibility and Transcript Binding Design

Directive: QSL-DIR-2026-05-17-115 / NA-0307

## Executive summary

NA-0307 is a design-only lane. It records the compatibility, transcript-binding,
key-schedule, reject-taxonomy, formal/model, vector/refimpl, and qsc harness
requirements for future explicit qsc handshake suite-id semantics. It does not
implement a qsc suite-id field, does not change `QHSM` or QSP frame schema,
does not change qsc runtime source, does not change protocol or crypto
behavior, and does not change dependencies.

Selected design posture:

- future explicit qsc handshake suite-id semantics use a version-gated,
  bounded `QHSM` negotiated-parameter block;
- legacy `QHSM` v1 frames are never explicit suite-id admission evidence;
- a future suite-id-required mode rejects legacy frames fail-closed;
- a future compatibility mode may accept legacy v1 only when explicitly named,
  policy-gated, and excluded from suite-id admission claims;
- the canonical Suite-2 tuple bytes must be transcript-bound and must feed the
  future qsc handshake key-schedule context before any accepted session state;
  and
- implementation remains blocked until model/vector design freezes the exact
  bytes and expected rejects.

Selected successor:

NA-0308 -- qsc Handshake Suite-ID Formal Model and Vector Design

## Live NA-0307 scope

Live `NEXT_ACTIONS.md` authorizes bounded design, governance, and testplan
artifacts for qsc handshake suite-id compatibility and transcript binding. It
forbids implementation by default:

- no protocol or wire semantic change;
- no crypto state-machine, key schedule, or production handshake implementation
  change;
- no qsc suite-id implementation unless a live directive explicitly authorizes
  exact files and semantics;
- no dependency, workflow, website, service implementation, docs/public,
  README, START_HERE, branch-protection, or public-safety configuration drift;
  and
- exactly one READY item remains NA-0307 until a separate closeout.

The live scope matches this directive. NA-0307 is a no-implementation design
lane.

## Inherited NA-0306 direction

NA-0306 selected a future no-implementation design refinement lane after
finding that current qsc `QHSM` A1/B1/A2 frames have no explicit suite-id
field. NA-0306 recommended a version-gated, bounded `QHSM`
negotiated-parameter block carrying the Suite-2 `protocol_version` /
`suite_id` tuple, but required NA-0307 to freeze compatibility,
transcript-binding, key-schedule input, version-gating, reject taxonomy,
formal/model, vector/refimpl, and qsc harness requirements before any
production schema implementation.

Inherited blockers:

- unsupported suite-id admission input cannot be represented in current qsc
  handshake frames;
- downgrade-like suite-id admission input cannot be represented in current qsc
  handshake frames;
- malformed suite-id admission input cannot be represented in current qsc
  handshake frames; and
- persisted Suite-2 state after successful activation is useful state evidence,
  but is not explicit suite-id admission evidence.

## Sources inspected

- `NEXT_ACTIONS.md` NA-0307 entry
- `tests/NA-0306_closeout_restore_na0307_testplan.md`
- `docs/governance/evidence/NA-0306_qsc_handshake_suite_id_wire_format_authorization_plan.md`
- `tests/NA-0306_qsc_handshake_suite_id_wire_format_authorization_testplan.md`
- `docs/governance/evidence/NA-0305_qsc_handshake_suite_id_seam_authorization_plan.md`
- `docs/governance/evidence/NA-0304_qsc_handshake_suite_id_negotiation_harness.md`
- `docs/governance/evidence/NA-0303_qsc_handshake_activation_negotiation_harness.md`
- `docs/governance/evidence/NA-0302_suite2_negotiation_vector_qsc_cross_surface_harness.md`
- `docs/governance/evidence/NA-0301_suite2_negotiation_downgrade_harness.md`
- `docs/governance/evidence/NA-0300_core_replay_reject_no_mutation_harness.md`
- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/src/main.rs`
- `qsl/qsl-client/qsc/src/protocol_state/mod.rs`
- `qsl/qsl-client/qsc/tests/na_0304_handshake_suite_id_negotiation.rs`
- `tools/refimpl/quantumshield_refimpl/src/qsp/types.rs`
- `tools/refimpl/quantumshield_refimpl/src/qsp/handshake.rs`
- `tools/refimpl/quantumshield_refimpl/src/qsp/ratchet.rs`
- `tools/refimpl/quantumshield_refimpl/src/suite2/establish.rs`
- `tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs`
- `tools/refimpl/quantumshield_refimpl/src/suite2/state.rs`
- `formal/README.md`
- `formal/model_suite2_negotiation_bounded.py`
- `inputs/suite2/vectors/qshield_suite2_negotiation_vectors_na0302.json`
- `inputs/suite2/vectors/README.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `GOALS.md`
- `PROJECT_CHARTER.md`
- `CHECKLIST_PROTOCOL_CHANGE.md`

Discovery note: the directive listed generic top-level `qsp/**` and `qsc/**`
targets, but this checkout has no top-level `qsp` or `qsc` directories. The
actual qsc path is `qsl/qsl-client/qsc/**`; QSP/refimpl surfaces live under
`tools/refimpl/quantumshield_refimpl/src/qsp/**`.

## Current QHSM/QSP wire-format summary

Current qsc `QHSM` frames are private fixed-layout frames in
`qsl/qsl-client/qsc/src/handshake/mod.rs`:

- A1 / init: `QHSM` magic, `HS_VERSION=1`, `HS_TYPE_INIT`, 16-byte session id,
  KEM public key, signature public key, and 32-byte DH public key.
- B1 / response: `QHSM` magic, `HS_VERSION=1`, `HS_TYPE_RESP`, 16-byte session
  id, KEM ciphertext, transcript MAC, signature public key, signature, and
  32-byte DH public key.
- A2 / confirm: `QHSM` magic, `HS_VERSION=1`, `HS_TYPE_CONFIRM`, 16-byte
  session id, confirm MAC, and signature.

The parser is strict-length. Appended suite-id bytes are rejected as generic
frame-length errors before suite policy could run.

Current QSP/refimpl handshake and message surfaces already carry explicit
`protocol_version` and `suite_id` fields. QSP decode rejects unsupported values,
and QSP transcripts include encoded handshake structures. That supports the
general Suite-2 negotiation model, but it does not prove qsc `QHSM` handshake
admission because qsc A1/B1/A2 do not expose those fields today.

## Current session-state suite summary

Current qsc handshake activation supplies `SUITE2_PROTOCOL_VERSION` (`0x0500`)
and `SUITE2_SUITE_ID` (`0x0002`) internally when `hs_build_session` calls
`init_from_base_handshake`. NA-0304 confirms persisted qsc session state carries
the Suite-2 tuple after successful activation.

That state evidence remains useful for accepted-state inspection. It must not
be used as explicit suite-id admission evidence, unsupported suite reject
evidence, downgrade-like suite reject evidence, or malformed suite reject
evidence.

## Compatibility posture analysis

| Option | Classification | Rationale | Future proof requirements |
| --- | --- | --- | --- |
| Version-gated `QHSM` v2 negotiated-parameter block | RECOMMENDED | Gives a clean parser boundary. Old v1 frames remain fixed-layout. New explicit semantics appear only behind a new frame version and bounded canonical block. | Formal/model design, vector pack, qsc parser/admission harness, no-mutation rejects, and branch policy proving no silent fallback. |
| `QHSM` v1 fixed-field append | REJECTED | Current strict-length v1 parser would reject appended bytes. Changing v1 length semantics risks parser divergence and ambiguous legacy handling. | Only reconsider with a separate compatibility proof that old/new parser behavior cannot be confused. |
| New top-level QSP frame schema for qsc handshake | REJECTED for this lane | qsc `QHSM` is the missing admission surface; changing QSP instead would not prove qsc A1/B1/A2 admission and would widen scope. | Separate QSP wire-format directive and canonical spec changes would be required. |
| Dual-mode parser that auto-detects legacy vs explicit frames | RISKY | Auto-detection creates downgrade and parser-divergence risk if an attacker can strip or alter the parameter block and trigger legacy accept. | Must prove explicit policy gating, transcript separation, no silent fallback, and deterministic old/new parse rules. |
| New parser accepts v1 only in named compatibility mode | POSSIBLE | Useful for migration if explicitly configured or negotiated outside the suite-id-required lane. It cannot count as explicit suite-id admission. | Tests must label compatibility accepts separately, deny suite-id admission claims, and reject v1 when explicit mode is required. |
| Suite-id-required mode rejects all v1 frames | RECOMMENDED | This is the safest posture whenever explicit Suite-2 admission is required. Missing tuple, stripped tuple, and legacy frame all fail closed. | qsc harness must prove reject before accepted session mutation, outbound B1/A2, `recv_commit`, or secret-bearing output. |
| Out-of-band or persisted-state assertion | REJECTED | It does not authenticate qsc handshake admission and does not stop tuple stripping at A1/B1/A2. | None as an admission solution; state inspection remains supporting evidence only. |

Exact compatibility posture:

- Future explicit qsc suite-id semantics require `QHSM` `HS_VERSION=2`.
- `HS_VERSION=1` frames carry no explicit suite-id semantics.
- Old parsers are expected to reject v2 frames by version or strict length; a
  future lane must test this where feasible.
- New parsers may accept v1 frames only in an explicitly named compatibility
  mode. Such acceptance is not suite-id admission evidence.
- New parsers in suite-id-required mode must reject v1 frames before accepted
  session mutation or outbound handshake response.
- Automatic fallback from v2 to v1 is forbidden.
- Lack of the required suite tuple in a v2 frame is reject, not compatibility.

## Canonical parameter-block posture

Future `QHSM` v2 frames must carry a bounded canonical parameter block in A1,
B1, and A2. NA-0307 fixes the intended shape for NA-0308 modeling and vectors;
implementation remains deferred.

Recommended block:

- `param_block_len`: `u16` big-endian, byte length after this field, maximum
  64 bytes.
- Repeated parameters sorted by strictly increasing `param_id`.
- Parameter header: `param_id: u16`, `flags: u8`, `value_len: u16`, followed by
  `value_len` bytes.
- `flags` bit 0 means critical; all other flag bits must be zero.
- Required parameter `0x0001` is critical suite context with `value_len=4` and
  value `protocol_version: u16 || suite_id: u16`.
- Suite-2 value is `protocol_version=0x0500`, `suite_id=0x0002`.

Canonical rejects:

- duplicate parameter id;
- non-increasing parameter id order;
- missing required suite context;
- suite context with a noncritical flag;
- unknown critical parameter;
- unknown noncritical parameter unless a later directive explicitly authorizes
  ignore semantics with transcript coverage;
- nonzero reserved flags;
- malformed block length, underflow, overflow, or trailing bytes;
- alternative integer widths or encodings for the same tuple; and
- total block length above the selected bound.

## Transcript-binding posture analysis

| Posture | Classification | Rationale | Future proof required |
| --- | --- | --- | --- |
| Bind byte-exact canonical parameter blocks into qsc A1/B1/A2 transcript | RECOMMENDED | Prevents stripping, mismatch, and parser-normalization ambiguity from reaching accepted session state. | Vectors must show valid same-bytes accept and stripped/mismatched/duplicate/malformed reject. |
| Bind only semantic tuple after parsing | RISKY | Parser normalization differences could let distinct encodings share a semantic tuple while signatures/MACs cover different bytes. | Would require a stronger canonicalization proof; not selected. |
| Do not bind suite tuple because state already stores Suite-2 constants | REJECTED | Persisted state is post-admission and does not authenticate what the peer saw during A1/B1/A2. | None; this would hide the known gap. |
| Defer transcript-binding decision to implementation | REJECTED | Implementation without a frozen binding decision would risk protocol drift and downgrade ambiguity. | Not acceptable for a future implementation lane. |

Transcript-binding decision:

- The future suite context parameter and the complete canonical parameter-block
  bytes must be included in the qsc handshake transcript before any B1 MAC,
  B1 signature, A2 confirm MAC, A2 signature, or accepted qsp session state.
- A1 proposes the canonical block.
- B1 must echo the byte-exact A1 canonical block.
- A2 must confirm the byte-exact A1 canonical block.
- The authoritative accepted suite context is the A1 canonical block only when
  B1 and A2 echo/confirm it byte-for-byte.
- Any omission or alteration in B1/A2 rejects before state mutation and before
  outbound post-reject handshake output.

## Key-schedule posture analysis

| Posture | Classification | Rationale | Future proof required |
| --- | --- | --- | --- |
| Feed canonical suite context into future qsc handshake KDF/context inputs | RECOMMENDED | Makes accepted key material context-separated by the explicit suite tuple and parameter bytes, not only by post-admission state. | KDF/context vectors, model assumptions, qsc harnesses, and explicit no-mutation rejects before code. |
| Transcript-only binding, no KDF/context input | POSSIBLE but weaker | Transcript MAC/confirm would detect peer mismatch, but future key material would not be directly context-separated by suite policy. | Must prove transcript hash is consumed everywhere needed. Not selected as primary posture. |
| Leave key-schedule posture ambiguous until implementation | REJECTED | A future implementation would be unable to truthfully claim downgrade resistance without knowing KDF context semantics. | Not acceptable; future lane must STOP if this cannot be modeled. |
| Reject explicit suite-id schema entirely | REJECTED | Current blocker requires an explicit admission surface for qsc suite semantics; rejecting the schema would preserve the gap. | Only acceptable if a future canonical decision removes the qsc admission requirement. |

Key-schedule decision:

- Future explicit qsc suite-id implementation must feed a negotiated context
  derived from the byte-exact canonical parameter block into qsc handshake KDF
  inputs before accepted session state is committed.
- The negotiated context must cover at least `protocol_version`, `suite_id`,
  `param_block_len`, parameter ids, flags, value lengths, and values.
- NA-0308 must choose the exact derivation shape for vectors. Acceptable
  candidate: `suite_context_hash = KMAC/SHA-family context over
  "QSC.HS.SUITECTX" || canonical_param_block`, then include that value in the
  data for future `QSC.HS.PQ`, `QSC.HS.DHINIT`, `QSC.HS.TRANSCRIPT`,
  `QSC.HS.TRANSCRIPT.H`, `QSC.HS.CONFIRM`, and `QSC.HS.A2` derivations.
- No NA-0307 code change implements this. Any later implementation lane must
  STOP if it cannot include the selected context without explicit
  key-schedule authorization, vectors, and model updates.

## Reject taxonomy and migration requirements

| Case | Expected behavior | Mutation boundary | Output boundary | Log/secret boundary | Vector/model/qsc requirement |
| --- | --- | --- | --- | --- | --- |
| Unsupported `suite_id` | Reject fail-closed. | No accepted qsp session, no pending accept advancement beyond parse-local scratch. | No B1/A2 or `recv_commit` caused by reject. | Deterministic reason; no route token, passphrase marker, plaintext, secret, panic, or backtrace. | Positive unsupported-suite vector, model reject, qsc harness. |
| Downgraded `suite_id` | Reject when Suite-2 is required or mutually supported. | Same no accepted-state mutation. | Same no output artifact. | Same no secret leak. | Downgrade vector/model/harness. |
| Missing `suite_id` when required | Reject v2 missing required suite context. | Same no accepted-state mutation. | Same no output artifact. | Same no secret leak. | Missing-parameter vector/model/harness. |
| Suite context stripped from frame | Reject as missing tuple or transcript mismatch. | Same no accepted-state mutation. | Same no output artifact. | Same no secret leak. | Stripped A1/B1/A2 vector/model/harness. |
| Mismatched suite across A1/B1/A2 | Reject at first mismatch. | Reject before accepting initiator/responder state. | No confirm/output after mismatch. | Deterministic mismatch reason without peer secret material. | Mismatch vector/model/harness. |
| Duplicate suite parameter | Reject noncanonical block. | Parse-local only. | No response/output. | No raw parameter dump. | Duplicate vector/model/harness. |
| Unknown critical parameter | Reject. | Parse-local only. | No response/output. | Reason may say unknown critical without dumping value. | Unknown-critical vector/model/harness. |
| Unknown noncritical parameter | Reject unless a later directive authorizes ignore semantics. | Parse-local only. | No response/output. | No raw parameter dump. | Unknown-parameter vector/model/harness. |
| Noncanonical parameter order | Reject. | Parse-local only. | No response/output. | Deterministic noncanonical reason. | Ordering vector/model/harness. |
| Invalid parameter length | Reject underflow/overflow/trailing bytes. | Parse-local only. | No response/output. | No raw byte dump. | Length vector/model/harness. |
| Inconsistent `protocol_version` and `suite_id` | Reject unsupported tuple in its namespace. | Parse-local only. | No response/output. | Deterministic tuple reason. | Tuple vector/model/harness. |
| Legacy frame in suite-id-required mode | Reject v1 as missing explicit semantics. | No accepted qsp session. | No B1/A2 or `recv_commit`. | No overclaim; reason may name legacy frame. | Legacy-required vector/model/harness. |
| New frame presented to old parser | Old parser rejects by version or length. | No accepted state in old parser. | No response/output. | Existing generic reject remains secret-safe. | Compatibility proof where feasible. |

Migration requirements:

- Roll out refimpl/vector/formal design before qsc implementation.
- Keep v1 compatibility and v2 suite-required semantics separate in tests,
  markers, and docs.
- Do not silently rollback from v2 to v1 after any v2 signal is observed.
- Do not count v1 accepts as explicit suite-id admission.
- Add operator-visible compatibility wording only if a future directive
  authorizes UI/docs surfaces; NA-0307 does not.

## Selected design posture

Selected compatibility posture:

- `QHSM` v2 negotiated-parameter block is the only future explicit qsc
  handshake suite-id admission shape selected by NA-0307.
- `QHSM` v1 remains legacy. It may be accepted only in an explicit
  compatibility mode and never as explicit suite-id admission.
- Suite-id-required mode rejects v1, missing tuple, stripped tuple, unknown
  parameter, duplicate tuple, malformed block, and any A1/B1/A2 mismatch.

Selected transcript posture:

- Byte-exact canonical parameter blocks are transcript-bound across A1/B1/A2.
- A1 is authoritative only when B1 and A2 echo/confirm the same bytes.

Selected key-schedule posture:

- Future implementation must bind the negotiated context into qsc handshake
  KDF/context derivations. NA-0308 must freeze the exact derivation and vectors.
- Future implementation must STOP if key-schedule binding remains ambiguous.

Rejected alternatives:

- v1 append or auto-detect because it creates parser ambiguity;
- persisted-state-only because it does not authenticate admission;
- implementation before vectors/model because it risks silent crypto or wire
  drift;
- QSP-only change because it does not solve the qsc `QHSM` admission gap.

## Future formal/model requirements

NA-0308 should add a bounded model for qsc handshake suite-id negotiation that
covers:

- v1 legacy accepted only in compatibility mode;
- v1 legacy rejected in suite-id-required mode;
- v2 valid tuple accepted only when A1/B1/A2 match byte-for-byte;
- unsupported tuple, downgraded tuple, missing tuple, stripped tuple, duplicate
  tuple, unknown critical parameter, unknown parameter, noncanonical order,
  invalid length, inconsistent tuple, and A1/B1/A2 mismatch rejects;
- no accepted-state mutation on every reject;
- no outbound response after reject where the state machine should abort;
- transcript-bound and KDF-context-bound negotiated context assumptions; and
- explicit model limits, especially that cryptographic secrecy and parser
  memory safety are not proven by the bounded model.

Likely files:

- `formal/model_qsc_handshake_suite_id_bounded.py`
- `formal/run_model_checks.py`
- `formal/README.md`
- `docs/governance/evidence/NA-0308_qsc_handshake_suite_id_formal_model_vector_design.md`
- `tests/NA-0308_qsc_handshake_suite_id_formal_model_vector_design_testplan.md`

## Future vector/refimpl requirements

NA-0308 should add design-level vector definitions before qsc implementation.
The vectors should be stable enough for later qsc and refimpl consumers.

Required vector classes:

- valid v2 Suite-2 tuple;
- unsupported suite id;
- downgraded suite tuple;
- missing suite tuple;
- stripped tuple from A1, B1, and A2;
- mismatched suite tuple across A1/B1/A2;
- duplicate suite parameter;
- malformed suite length;
- noncanonical parameter order;
- unknown critical parameter;
- unknown noncritical parameter under the selected reject policy;
- inconsistent protocol version and suite id; and
- legacy v1 in compatibility mode vs suite-id-required mode.

Likely files:

- `inputs/suite2/vectors/qshield_qsc_handshake_suite_id_vectors_na0308.json`
- `tools/refimpl/quantumshield_refimpl/tests/na_0308_qsc_handshake_suite_id_vectors.rs` only if NA-0308 explicitly authorizes a refimpl vector consumer without changing production semantics.
- `docs/governance/evidence/NA-0308_qsc_handshake_suite_id_formal_model_vector_design.md`

## Future qsc harness requirements

A later implementation or implementation-authorization lane must require qsc
harnesses to assert:

- valid v2 explicit Suite-2 admission succeeds only with byte-exact matching
  canonical blocks in A1/B1/A2;
- each reject case occurs before accepted qsp session mutation;
- no reject emits `event=recv_commit`, `event=qsp_unpack ok=true`, plaintext,
  or a post-reject handshake response;
- deterministic reject classification is stable enough for tests without
  leaking raw parameter bytes;
- no panic or backtrace is emitted;
- no route token, passphrase-env marker, secret, or plaintext sentinel is
  emitted; and
- legacy v1 compatibility accepts are labeled separately from explicit
  suite-id admission.

Likely later files if explicitly authorized after NA-0308:

- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/tests/na_0309_handshake_suite_id_parameter_block.rs`
- `tests/NA-0309_qsc_handshake_suite_id_parameter_block_implementation_testplan.md`

Those files are not authorized by NA-0307.

## Future NA-0308 likely files

Recommended NA-0308 files:

- `formal/model_qsc_handshake_suite_id_bounded.py`
- `formal/run_model_checks.py`
- `formal/README.md`
- `inputs/suite2/vectors/qshield_qsc_handshake_suite_id_vectors_na0308.json`
- `docs/governance/evidence/NA-0308_qsc_handshake_suite_id_formal_model_vector_design.md`
- `tests/NA-0308_qsc_handshake_suite_id_formal_model_vector_design_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

NA-0308 should not implement qsc `QHSM` parser/runtime changes unless its live
directive explicitly changes scope after model/vector design is complete.

## Future NA-0308 expected markers

Expected design/model/vector markers:

- `NA0308_QSC_HANDSHAKE_SUITE_ID_MODEL_OK`
- `NA0308_QSC_HANDSHAKE_SUITE_ID_VECTOR_DESIGN_OK`
- `NA0308_QSC_HANDSHAKE_SUITE_ID_COMPATIBILITY_POLICY_OK`
- `NA0308_QSC_HANDSHAKE_SUITE_ID_TRANSCRIPT_BINDING_OK`
- `NA0308_QSC_HANDSHAKE_SUITE_ID_KDF_CONTEXT_OK`
- `NA0308_NO_QSC_RUNTIME_IMPLEMENTATION_OK`
- `NA0308_CLAIM_BOUNDARY_OK`

Later implementation markers should be selected by the future implementation
authorization lane, not NA-0307.

## Stop conditions

Future work must STOP if:

- it would implement qsc `QHSM` schema changes before NA-0308 freezes formal
  and vector design;
- it would accept v1 legacy frames as explicit suite-id admission evidence;
- it would silently fall back from v2 explicit semantics to v1 compatibility;
- it would leave transcript binding or KDF context binding ambiguous before
  implementation;
- it would present persisted Suite-2 state as explicit admission evidence;
- it would alter key schedule, SCKA, crypto state-machine, production
  handshake semantics, QSP wire format, dependencies, workflows, branch
  protection, public-safety configuration, services, website, README,
  START_HERE, or docs/public outside explicit scope;
- formal/model or vector requirements cannot represent downgrade, stripped,
  mismatch, duplicate, malformed, or legacy-mode cases; or
- required CI or public-safety fails conclusively.

## Claim boundaries

Allowed claims:

- NA-0307 selects future compatibility, transcript-binding, and key-schedule
  posture for explicit qsc handshake suite-id semantics.
- Current qsc `QHSM` v1 frames lack an explicit suite-id field.
- Future explicit qsc handshake suite-id admission remains unimplemented.
- NA-0308 should model and vectorize the selected posture before implementation
  authorization.

Forbidden or unsupported claims:

- qsc unsupported, downgraded, malformed, stripped, or mismatched handshake
  suite-id admission is already tested directly.
- Persisted Suite-2 state is explicit qsc handshake suite-id admission evidence.
- NA-0307 implements qsc suite-id wire-format behavior.
- External review is finished.
- The system provides anonymity, metadata-free behavior, or untraceability.
- The system is ready for unrestricted deployment.

## No implementation change proof

NA-0307 changes only governance, evidence, testplan, traceability, and rolling
journal documents. It does not change:

- qsc runtime source under `qsl/qsl-client/qsc/src/**`;
- qsc runtime tests under `qsl/qsl-client/qsc/tests/**`;
- QSP/refimpl/protocol-core implementation source;
- crypto state-machine, key schedule, production handshake implementation, or
  QSP wire-format implementation;
- Cargo manifests or locks;
- workflows or scripts;
- qsl-server, qsl-attachments, qsc-desktop, apps, formal models, inputs, tools
  implementation paths, website/external website, README, START_HERE,
  docs/public, branch protection, or public-safety configuration.

## Next recommendation

After NA-0307 merges and public-safety is green, close NA-0307 and restore
exactly one READY successor:

NA-0308 -- qsc Handshake Suite-ID Formal Model and Vector Design

NA-0308 should remain no-implementation by default. Its job is to convert this
design into executable model and vector requirements so a later implementation
authorization lane can name exact qsc files and tests without ambiguity.
