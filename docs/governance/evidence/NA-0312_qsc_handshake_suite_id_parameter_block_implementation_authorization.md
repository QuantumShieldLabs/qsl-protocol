Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-18
Replaces: n/a
Superseded-By: n/a

Goals: G1, G2, G3, G4, G5

# NA-0312 qsc Handshake Suite-ID Parameter-Block Implementation Authorization

Directive: QSL-DIR-2026-05-18-120 / NA-0312

## Executive summary

NA-0312 authorizes the exact boundary for a future qsc handshake suite-id
parameter-block implementation lane. It does not implement that parameter
block, does not change qsc runtime behavior, does not change `QHSM` or QSP
production wire behavior, does not change the production handshake, does not
change crypto state-machine behavior, does not change dependencies, and does
not upgrade any public readiness or privacy claim.

Decision:

- Future qsc implementation may proceed only as the bounded NA-0313 lane named
  below.
- The next lane must use the existing qsc CLI/relay harness as the transport
  surface, but it must add a shared production parser/admission path before any
  explicit suite-id admission claim can be made.
- The implementation must be confined to the exact qsc files, tests, evidence,
  and governance paths listed in this document.
- Compatibility mode must be explicit and separate from explicit suite-id
  admission.
- Suite-required mode must reject legacy or stripped suite context
  fail-closed.
- Transcript and qsc handshake key-context binding are required before
  accepted session state is committed.
- If any requirement needs broader protocol, QSP, refimpl, formal, vector,
  dependency, workflow, service, website, or key-schedule scope, the future
  lane must stop and open a narrower blocker or refinement lane.

Selected successor:

NA-0313 -- qsc Handshake Suite-ID Parameter-Block Implementation Harness

Metadata runtime identifier/default-padding work remains a near-term priority.
Because NA-0312 can freeze the qsc implementation boundary safely and the qsc
explicit-admission gap is on the current critical path, metadata runtime
transition is recommended immediately after the NA-0313 implementation/harness
milestone unless NA-0313 stops on a prerequisite blocker.

## Live NA-0312 scope

Live `NEXT_ACTIONS.md` records NA-0312 as the sole READY item:

- title: qsc Handshake Suite-ID Parameter-Block Implementation Authorization;
- status: READY;
- objective: decide exact allowed files, semantics, tests, markers, and stop
  conditions before any parameter-block implementation may proceed;
- must protect: no unsupported production, public-internet, external-review, or
  anonymity claims; no silent protocol or crypto semantic changes; executable
  evidence or exact prerequisite stop; no qsc runtime, `QHSM` wire-format,
  production handshake, key schedule, QSP wire-format, or production suite-id
  field implementation unless a live directive explicitly authorizes exact
  files and semantics; visible suite-id admission limitations; visible metadata
  runtime agenda;
- expected deliverables: exact implementation-authorization decision; exact
  allowed and forbidden future file lists; marker/test plan; stop conditions;
- acceptance: NA-0311 selected successor remains visible, no NA-0312
  implementation is smuggled into authorization, required CI and public-safety
  are green, and exactly one READY item remains NA-0312.

The live scope matches this directive. NA-0312 remains authorization and
governance evidence only.

## Inherited NA-0311 conclusion

NA-0311 concluded:

- Current qsc `QHSM` A1/B1/A2 frames are strict fixed-layout v1 frames.
- Current qsc CLI/relay tests can prove current v1 frame flow, persisted
  Suite-2 session-state tuple, current version/type/malformed rejects,
  no-mutation boundaries, no `recv_commit` on reject, no output on reject, and
  no secret leak for current surfaces.
- Current qsc cannot directly consume NA-0310 explicit suite-id
  parameter-block vectors because no `QHSM` v2 parser or parameter-block
  admission path exists.
- A test-only fixture builder alone would prove only synthetic bytes or generic
  length/version rejection, not real qsc suite-id admission.
- The next safe lane is parameter-block implementation authorization.

NA-0312 carries that conclusion forward. Persisted Suite-2 session state remains
supporting state evidence only; it is not explicit suite-id admission proof.

## Sources inspected

- `GOALS.md`
- `PROJECT_CHARTER.md`
- `CHECKLIST_PROTOCOL_CHANGE.md`
- `docs/dev/DOC-DEV-003_Assistant_Operating_Rules_v1.0.0_DRAFT.md`
- `NEXT_ACTIONS.md` NA-0312 entry
- `DECISIONS.md`
- `TRACEABILITY.md`
- `tests/NA-0311_closeout_restore_na0312_testplan.md`
- `docs/governance/evidence/NA-0311_qsc_handshake_suite_id_qsc_harness_requirements.md`
- `tests/NA-0311_qsc_handshake_suite_id_qsc_harness_requirements_testplan.md`
- `docs/governance/evidence/NA-0310_qsc_handshake_suite_id_vector_refimpl_oracle.md`
- `tests/NA-0310_qsc_handshake_suite_id_vector_refimpl_oracle_testplan.md`
- `inputs/suite2/qsc_handshake_suite_id_vectors_na0310.json`
- `tools/refimpl/quantumshield_refimpl/tests/na_0310_qsc_suite_id_vector_oracle.rs`
- `formal/model_qsc_handshake_suite_id_bounded.py`
- `formal/run_model_checks.py`
- `docs/governance/evidence/NA-0307_qsc_handshake_suite_id_compatibility_transcript_design.md`
- `docs/governance/evidence/NA-0308_qsc_handshake_suite_id_formal_vector_design.md`
- `docs/governance/evidence/NA-0309_qsc_handshake_suite_id_formal_model_properties.md`
- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/src/cmd/mod.rs`
- `qsl/qsl-client/qsc/src/main.rs`
- `qsl/qsl-client/qsc/src/protocol_state/mod.rs`
- `qsl/qsl-client/qsc/tests/common/mod.rs`
- `qsl/qsl-client/qsc/tests/na_0302_suite2_negotiation_cross_surface.rs`
- `qsl/qsl-client/qsc/tests/na_0303_handshake_activation_negotiation.rs`
- `qsl/qsl-client/qsc/tests/na_0304_handshake_suite_id_negotiation.rs`
- `docs/governance/evidence/NA-0288_metadata_phase2_external_review_gap_plan.md`
- `docs/governance/evidence/NA-0290_metadata_phase2_identifier_padding_design.md`
- `docs/governance/evidence/NA-0291_metadata_phase2_identifier_padding_harness.md`
- `docs/governance/evidence/NA-0292_metadata_phase2_sanitized_errors_retention_design.md`
- `docs/governance/evidence/NA-0293_metadata_phase2_sanitized_errors_retention_harness.md`
- `docs/public/RELEASE_READINESS_EVIDENCE_MAP.md` read-only
- `docs/public/EXTERNAL_REVIEW_PACKAGE.md` read-only

Search terms included: qsc harness, implementation authorization, parameter
block, QHSM v2, QHSM, suite-id, suite_id, suiteid, parser, encoder, decoder,
frame, A1, B1, A2, protocol_version, negotiated parameter, compatibility mode,
suite-required mode, transcript, key context, no mutation, recv_commit, output,
reason label, implementation, authorized files, stop condition, metadata,
identifier, padding, and runtime.

## Current qsc runtime/frame surfaces

Current qsc `QHSM` handshake code lives in
`qsl/qsl-client/qsc/src/handshake/mod.rs`.

Current structs and codec helpers:

- `HsInit`: session id, KEM public key, signature public key, DH public key.
- `HsResp`: session id, KEM ciphertext, transcript MAC, signature public key,
  signature, DH public key.
- `HsConfirm`: session id, confirm MAC, signature.
- `hs_encode_init` / `hs_decode_init`.
- `hs_encode_resp` / `hs_decode_resp`.
- `hs_encode_confirm` / `hs_decode_confirm`.
- `HS_MAGIC = QHSM`, `HS_VERSION = 1`, and frame types 1/2/3 for A1/B1/A2.

Current transcript and qsc handshake key-context helpers:

- `hs_transcript_mac`
- `hs_transcript_hash`
- `hs_pq_init_ss`
- `hs_dh_init_from_shared`
- `hs_confirm_key`
- `hs_confirm_mac`
- `hs_sig_msg_b1`
- `hs_sig_msg_a2`
- `hs_build_session`

Current admission state and mutation/output paths:

- `perform_handshake_init_with_route` builds and sends A1 and stores pending
  initiator state.
- `perform_handshake_poll_with_tokens` consumes A1/B1/A2, validates identity
  pins, builds sessions, stores pending responder state, stores accepted qsp
  session state, clears pending state, and emits handshake markers.
- `qsp_session_store` / `qsp_session_load` live in
  `qsl/qsl-client/qsc/src/protocol_state/mod.rs` and should remain unchanged
  for NA-0313 unless a future directive stops and separately authorizes that
  session-store scope.
- CLI plumbing for `handshake init`, `handshake poll`, and `handshake status`
  is split across `qsl/qsl-client/qsc/src/cmd/mod.rs` and
  `qsl/qsl-client/qsc/src/main.rs`.

Current tests:

- `qsl/qsl-client/qsc/tests/common/mod.rs` provides command/vault/relay helpers.
- `qsl/qsl-client/qsc/tests/na_0303_handshake_activation_negotiation.rs` proves
  current v1 version/malformed/inactive/replay reject properties.
- `qsl/qsl-client/qsc/tests/na_0304_handshake_suite_id_negotiation.rs` proves
  persisted Suite-2 state and the absence of explicit suite-id fields in
  current v1 `QHSM` frames.

## Implementation boundary inventory

| Surface | Current file / type / function | Future touch? | Test-only helper sufficient? | QSP docs/design first? | Expected tests | Risk |
| --- | --- | --- | --- | --- | --- | --- |
| QHSM frame structs | `handshake/mod.rs` `HsInit`, `HsResp`, `HsConfirm` | Yes, only to carry version-gated v2 canonical parameter block state | No | No QSP change; QHSM v2 decision/governance must accompany | valid v2 accept; legacy compat; legacy required reject | Field drift or legacy overclaim |
| Parser/decoder | `hs_decode_init`, `hs_decode_resp`, `hs_decode_confirm` | Yes, bounded v1/v2 parser split and parameter-block parser | No | No QSP change; stop if canonical spec conflict appears | unsupported, downgraded, stripped, duplicate, unknown, noncanonical, malformed, inconsistent rejects | Generic decode errors mislabeled as suite-id evidence |
| Encoder/builder | `hs_encode_init`, `hs_encode_resp`, `hs_encode_confirm` | Yes, bounded v2 encoder that emits canonical block and v1 encoder only in explicit compatibility mode | No | No QSP change | canonical encoding, echo/confirm byte equality | Noncanonical alternate encodings |
| Handshake/admission state | `perform_handshake_init_with_route`, `perform_handshake_poll_with_tokens` | Yes, only for suite-required/compatibility policy, reject ordering, and context checks before accepted state | No | No QSP change | no mutation, no output, no B1/A2 after reject where forbidden | State commit before reject |
| Compatibility / suite-required mode | `cmd/mod.rs`, `main.rs`, `handshake/mod.rs` | Yes, only to pass an explicit mode into handshake init/poll; hidden test flag or explicit config must not silently change defaults | No | No QSP change | legacy compat accept; legacy required reject; stripped suite reject | Silent fallback |
| Session-state write path | `protocol_state/mod.rs` `qsp_session_store` | No by default | Not needed | No | assert unchanged store semantics through tests | Store semantics churn |
| recv_commit/output path | qsc send/receive and transport markers | No by default | Existing assertions sufficient | No | no `recv_commit`, no qsp unpack success, no plaintext/file output on reject | Output before reject |
| Logging/error path | `emit_marker` call sites in `handshake/mod.rs` | Yes, deterministic reason labels only; no raw parameter bytes or secrets | No | No | reason labels and leak scans | Secret or route-token leak |
| Tests/harness | `tests/common/mod.rs`; future NA-0313 qsc test | Yes | Helpers are support only, not sufficient evidence alone | No | NA-0310 vector-driven qsc harness and NA-0300..NA-0304 regressions | Synthetic fixture overclaim |

## Authorization decision

Selected outcome: Outcome A.

NA-0312 authorizes a future qsc implementation lane:

NA-0313 -- qsc Handshake Suite-ID Parameter-Block Implementation Harness

Rationale:

- The missing implementation boundary is understood: qsc lacks a shared
  `QHSM` v2 parser/runtime admission surface.
- The exact files needed for a narrow implementation/harness lane can be
  frozen safely.
- NA-0307, NA-0309, NA-0310, and NA-0311 already freeze compatibility posture,
  transcript/key-context requirements, model properties, vector categories,
  refimpl oracle checks, and qsc harness expectations.
- A parser-only lane would still need the same production parser boundary to
  avoid synthetic evidence, so a bounded implementation/harness lane is a more
  truthful next step than another planning loop.

Rejected outcomes:

- Outcome B, parser/vector harness first: rejected because the parser would
  need to be the shared production parser to prove admission, and NA-0312 can
  bound that parser in the same NA-0313 implementation/harness lane.
- Outcome C, metadata runtime immediately: rejected for NA-0313 because the
  qsc explicit-admission gap remains on the current critical path and can now
  be addressed with a bounded file set. Metadata runtime remains the next
  recommended insertion after the NA-0313 milestone.
- Outcome D, blocker continuation: rejected because the blocker is no longer
  ambiguous. The stop conditions below cover cases where future implementation
  discovers broader scope.

## Future allowed files

NA-0313 may touch only these implementation files:

- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/src/cmd/mod.rs`
- `qsl/qsl-client/qsc/src/main.rs`

Restrictions:

- `handshake/mod.rs` changes are limited to `QHSM` v2 parameter-block
  parser/encoder/admission logic, explicit compatibility/suite-required mode,
  canonical suite-context transcript binding, qsc handshake key-context
  binding, deterministic reason labels, and reject/no-mutation/no-output
  ordering.
- `cmd/mod.rs` and `main.rs` changes are limited to explicit handshake
  suite-mode plumbing for the harness and operators. Defaults must not silently
  weaken suite-required posture or hide compatibility mode.
- No other qsc source file may change unless NA-0313 stops and receives a new
  authorization decision.

NA-0313 may add or update only these qsc test files:

- `qsl/qsl-client/qsc/tests/common/mod.rs`
- `qsl/qsl-client/qsc/tests/na_0313_handshake_suite_id_parameter_block.rs`

NA-0313 must reuse these existing artifacts as read-only inputs unless it stops
for a model/vector/refimpl mismatch:

- `inputs/suite2/qsc_handshake_suite_id_vectors_na0310.json`
- `tools/refimpl/quantumshield_refimpl/tests/na_0310_qsc_suite_id_vector_oracle.rs`
- `formal/model_qsc_handshake_suite_id_bounded.py`
- `formal/run_model_checks.py`
- `docs/governance/evidence/NA-0310_qsc_handshake_suite_id_vector_refimpl_oracle.md`
- `docs/governance/evidence/NA-0311_qsc_handshake_suite_id_qsc_harness_requirements.md`

NA-0313 may touch these governance/evidence paths:

- `docs/governance/evidence/NA-0313_qsc_handshake_suite_id_parameter_block_implementation_harness.md`
- `tests/NA-0313_qsc_handshake_suite_id_parameter_block_implementation_harness_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

If NA-0313 is closed out later, a separate closeout directive may touch only
queue/closeout governance files named by that later directive.

## Future forbidden files

NA-0313 must not touch:

- `Cargo.toml`
- `Cargo.lock`
- `.github/**`
- `qsl/qsl-client/qsc/src/protocol_state/mod.rs` unless a new directive first
  authorizes session-store semantics
- any other `qsl/qsl-client/qsc/src/**` path not listed above
- any qsc test file not listed above, except by explicit new directive
- `qsl-server/**`
- `qsl-attachments/**`
- `qsl/qsl-client/qsc-desktop/**`
- `website/**`
- external website repositories
- `README.md`
- `START_HERE.md`
- `docs/public/**`
- `apps/**`
- `tools/refimpl/quantumshield_refimpl/src/**`
- `tools/refimpl/quantumshield_refimpl/tests/**` unless the future lane stops
  for an oracle mismatch and a new directive authorizes refinement
- `inputs/**` unless the future lane stops for vector mismatch and a new
  directive authorizes refinement
- `formal/**` unless the future lane stops for model mismatch and a new
  directive authorizes refinement
- branch-protection or public-safety configuration
- dependency manifests, lockfiles, vendored dependency sources, or workflow
  definitions

No qsl-server, qsl-attachments, qsc-desktop, website, service, demo, QSP
wire-format, refimpl runtime, protocol-core, broader crypto state-machine, or
dependency change is authorized by NA-0312.

## Future parser/encoder requirements

NA-0313 must implement a bounded canonical `QHSM` v2 parameter block using the
NA-0307 posture unless it stops:

- `HS_VERSION=1` remains legacy fixed-layout and has no explicit suite-id
  semantics.
- `HS_VERSION=2` is the only explicit qsc suite-id admission version.
- A v2 frame carries `param_block_len` as a big-endian `u16` with maximum 64
  bytes.
- Parameters are strictly sorted by increasing `param_id`.
- Each parameter header is `param_id: u16`, `flags: u8`, `value_len: u16`,
  followed by `value_len` bytes.
- Flags bit 0 is critical; all other bits must be zero.
- Required parameter `0x0001` is critical suite context with `value_len=4` and
  value `protocol_version: u16 || suite_id: u16`.
- The only accepted explicit tuple for this lane is
  `protocol_version=0x0500`, `suite_id=0x0002`.
- Unknown critical parameters reject.
- Unknown noncritical parameters reject by default unless a future directive
  separately authorizes ignore semantics with transcript coverage.
- Duplicate, missing, stripped, noncanonical, malformed, trailing, underflow,
  overflow, inconsistent namespace, and over-bound blocks reject.
- Encoders must emit one canonical byte representation only.
- Decoders must not allocate or log attacker-controlled unbounded data.

## Future transcript/key-context requirements

NA-0313 must bind the byte-exact canonical parameter block before accepted
state:

- A1 proposes the canonical block.
- B1 must echo the byte-exact A1 canonical block.
- A2 must confirm the byte-exact A1 canonical block.
- Any omission or alteration in B1 or A2 rejects before accepted qsp session
  state is stored.
- `hs_transcript_mac`, `hs_transcript_hash`, B1 signature input, A2 confirm MAC,
  and A2 signature input must cover the canonical suite context.
- qsc handshake key-context derivations in `handshake/mod.rs` must include a
  deterministic suite-context value derived from the canonical parameter block
  before `hs_build_session` commits accepted state.
- Broader refimpl Suite-2 KDF, SCKA, AEAD, or QSP message key schedule files
  must not change in NA-0313. If the future implementation cannot preserve
  key-context binding within the allowed qsc handshake file, it must stop.

## Future compatibility-mode rules

NA-0313 must implement or expose explicit modes:

- `suite_required`: v2 canonical suite context required; v1 frames reject.
- `legacy_compat`: v1 frames may be accepted only when explicitly selected and
  must be labeled as compatibility accept, not explicit suite-id admission.

Rules:

- No automatic fallback from v2 to v1.
- Stripped or missing suite context in v2 rejects; it must not be interpreted
  as v1.
- Legacy compatibility accept must never emit the final
  `NA0313_QSC_SUITE_ID_PARAMETER_BLOCK_OK` marker by itself.
- Defaults must be fail-closed for any ambiguous mode. If changing the default
  would affect existing operator workflows, NA-0313 must stop and request a
  separate migration/compatibility directive.

## Future required markers

NA-0313 must emit or assert all of these markers only when backed by executable
evidence:

- `NA0313_QHSM_V2_PARAMETER_BLOCK_PARSE_OK`
- `NA0313_VALID_SUITE2_ACCEPT_OK`
- `NA0313_LEGACY_COMPAT_ACCEPT_OK`
- `NA0313_REQUIRED_MODE_LEGACY_REJECT_OK`
- `NA0313_UNSUPPORTED_SUITE_REJECT_OK`
- `NA0313_DOWNGRADE_SUITE_REJECT_OK`
- `NA0313_STRIPPED_SUITE_REJECT_OK`
- `NA0313_MISMATCH_SUITE_REJECT_OK`
- `NA0313_DUPLICATE_SUITE_REJECT_OK`
- `NA0313_UNKNOWN_CRITICAL_REJECT_OK`
- `NA0313_NONCANONICAL_REJECT_OK`
- `NA0313_MALFORMED_REJECT_OK`
- `NA0313_TRANSCRIPT_BINDING_OK`
- `NA0313_KEY_CONTEXT_BINDING_OK`
- `NA0313_NO_MUTATION_ON_REJECT_OK`
- `NA0313_NO_OUTPUT_ON_REJECT_OK`
- `NA0313_NO_SECRET_LEAK_OK`
- `NA0313_QSC_SUITE_ID_PARAMETER_BLOCK_OK`

## Future required tests

NA-0313 must include:

- vector-driven qsc harness coverage derived from
  `inputs/suite2/qsc_handshake_suite_id_vectors_na0310.json`;
- refimpl oracle cross-check:
  `cargo test -p quantumshield_refimpl --locked --test na_0310_qsc_suite_id_vector_oracle -- --test-threads=1`;
- model cross-check:
  `python3 formal/model_qsc_handshake_suite_id_bounded.py` and
  `python3 formal/run_model_checks.py`;
- valid v2 Suite-2 accept;
- legacy compatibility accept;
- suite-required legacy reject;
- unsupported suite reject;
- downgraded tuple reject;
- stripped suite context reject;
- A1/B1 mismatch reject;
- B1/A2 mismatch reject;
- duplicate suite parameter reject;
- unknown critical parameter reject;
- unknown noncritical default reject;
- noncanonical parameter order reject;
- malformed length/trailing/underflow/overflow reject;
- inconsistent protocol-version/suite-id tuple reject;
- transcript binding mismatch reject;
- key-context mismatch or missing key context reject;
- replayed A1 with suite context;
- replayed A2 with suite context, or a deterministic no-op if justified by
  explicit accepted-state replay semantics;
- no accepted session mutation on every reject;
- no `recv_commit`, qsp unpack success, plaintext output, file/attachment
  completion output, or positive handshake-complete marker on every reject;
- no mock vault passphrase, route token, vector sentinel, panic text,
  backtrace, raw KEM/DH/AEAD/chain-key/root-key/skipped-key material, or
  secret-like long hex in accepted or rejected output;
- regression tests for existing NA-0300, NA-0301, NA-0302, NA-0303, and
  NA-0304 harnesses;
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`;
- `cargo audit --deny warnings`;
- `cargo tree -i rustls-webpki --locked`;
- public-safety required and green before merge and after merge.

## Future stop conditions

NA-0313 must stop if any of these occur:

- `QHSM` v2 format ambiguity appears.
- Transcript or key-context binding cannot be implemented before accepted
  state within `handshake/mod.rs`.
- Legacy compatibility behavior is ambiguous or would silently downgrade.
- Current qsc parser cannot be changed without broad runtime churn outside the
  authorized qsc files.
- Session-state store changes are required.
- QSP wire-format changes are required.
- Refimpl runtime, formal model, or vector changes are required before the qsc
  implementation can be truthful.
- Broader key schedule or crypto state-machine files must change.
- A dependency or Cargo lockfile change is needed.
- Existing tests fail for security-relevant reasons.
- A direct qsc suite-id admission claim would rely only on persisted Suite-2
  state, synthetic fixtures, or generic length/version rejects.
- Output or logs expose a route token, passphrase, vector sentinel, raw secret,
  panic text, backtrace, or unsafe long-hex evidence.
- Metadata runtime agenda would be hidden.
- The change would introduce production, public-internet, external-review,
  anonymity, metadata-free, untraceable, quantum-proof, unbreakable,
  guaranteed-secure, or broad-readiness overclaims.

## Metadata runtime agenda decision

Metadata evidence reviewed:

- NA-0288 maps metadata phase-2 and external-review readiness gaps.
- NA-0290 designs identifier rotation / opaque-handle and padding-default
  policy.
- NA-0291 proves deterministic identifier/padding policy fixtures only.
- NA-0292 designs sanitized-error and retention/purge metadata policy.
- NA-0293 proves deterministic sanitized-error/retention policy fixtures only.
- Public release/readiness docs keep metadata phase-2 `NOT_READY`: runtime
  identifier rotation, runtime default padding, broader runtime sanitized-error
  behavior, production retention/purge behavior, timing, size, contact graph,
  IP-level metadata, deployment metadata, and external review remain open.

Decision:

- NA-0313 remains qsc implementation/harness because the exact qsc boundary can
  now be frozen and explicit qsc admission is the active critical-path gap.
- Metadata runtime identifier/default-padding transition should be inserted
  immediately after NA-0313 if NA-0313 merges cleanly and no qsc blocker needs
  the next slot.
- Exact recommended metadata lane title:
  Metadata Runtime Identifier and Default Padding Transition Plan.

Likely first metadata files for that future lane:

- `docs/governance/evidence/NA-03xx_metadata_runtime_identifier_padding_transition_plan.md`
- `tests/NA-03xx_metadata_runtime_identifier_padding_transition_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

NA-0312 does not implement metadata runtime behavior and does not claim runtime
metadata reduction is implemented.

## Coverage matrix

| Requirement | NA-0312 authorization | Future NA-0313 proof | Stop if |
| --- | --- | --- | --- |
| Valid v2 Suite-2 parameter block | Authorized within `handshake/mod.rs` | qsc v2 accepted-path test plus NA-0310 vector mapping | Accepted path lacks canonical parser or context binding |
| Legacy compatibility | Explicit mode only | separate legacy compat marker | Compatibility is counted as explicit admission |
| Suite-required legacy reject | Required | v1 frame rejects in suite-required mode | Default silently falls back |
| Unsupported suite | Required | unsupported tuple reject | Generic decode reject is overclaimed |
| Downgrade/stripped/mismatch | Required | A1/B1/A2 mismatch and stripped tests | State commits before reject |
| Duplicate/unknown/noncanonical/malformed | Required | parser canonicality tests | Parser accepts ambiguous encoding |
| Transcript binding | Required before state | transcript mismatch and accept tests | Binding cannot cover byte-exact block |
| Key-context binding | Required before state | key-context mismatch and accept tests | Needs broader key-schedule files |
| No mutation on reject | Required | session/pending/output snapshots | Store or pending state mutates |
| No output on reject | Required | stdout/stderr and state-artifact assertions | `recv_commit` or positive output appears |
| No secret leak | Required | leak scans over outputs/artifacts | secret-like evidence leaks |
| Metadata agenda | Required visible | next-lane recommendation | qsc work hides G5 runtime gaps |

## Selected successor

Selected successor:

NA-0313 -- qsc Handshake Suite-ID Parameter-Block Implementation Harness

Successor handoff:

- Implement only the bounded qsc parser/admission/mode/context/test surface
  authorized here.
- Reuse NA-0309 model, NA-0310 vectors/refimpl oracle, and NA-0311 harness
  requirements.
- Keep metadata runtime transition as the recommended immediate insertion after
  NA-0313 unless NA-0313 stops on a prerequisite blocker.

## No qsc runtime/protocol/wire implementation change proof

NA-0312 changes only:

- this evidence document;
- the NA-0312 testplan;
- `DECISIONS.md`;
- `TRACEABILITY.md`;
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.

NA-0312 does not change:

- `qsl/qsl-client/qsc/src/**`
- `qsl/qsl-client/qsc/tests/**`
- `qsl/qsl-client/qsc/Cargo.toml`
- `Cargo.toml`
- `Cargo.lock`
- `qsl-server/**`
- `qsl-attachments/**`
- `qsl/qsl-client/qsc-desktop/**`
- `apps/**`
- `tools/refimpl/**`
- `inputs/**`
- `formal/**`
- `docs/public/**`
- `README.md`
- `START_HERE.md`
- `.github/**`
- website or external website sources
- branch-protection or public-safety configuration

This document is authorization evidence only. It is not qsc runtime proof,
QHSM/QSP production wire-format proof, production handshake proof, crypto
state-machine proof, key-schedule implementation proof, metadata runtime proof,
external review proof, or production-readiness proof.

## Backup-plan impact statement

No backup-plan update is required. NA-0312 changes only tracked qsl-protocol
governance, evidence, testplan, traceability, decision, and rolling journal
files under `/srv/qbuild/work`, which is already within the local backup scope.
It creates no non-rebuildable artifact outside the repository worktree and
does not move evidence roots, response roots, source roots, or excluded backup
paths.

## Next recommendation

After this PR merges and post-merge public-safety is green, close out NA-0312
and restore exactly one READY successor:

NA-0313 -- qsc Handshake Suite-ID Parameter-Block Implementation Harness

If NA-0313 merges cleanly and proves explicit qsc suite-id admission, insert
the metadata runtime lane immediately after it:

Metadata Runtime Identifier and Default Padding Transition Plan
