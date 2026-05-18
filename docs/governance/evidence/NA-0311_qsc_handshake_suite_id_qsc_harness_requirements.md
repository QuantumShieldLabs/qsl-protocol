Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-18
Replaces: n/a
Superseded-By: n/a

Goals: G1, G2, G3, G4, G5

# NA-0311 qsc Handshake Suite-ID qsc Harness Requirements

Directive: QSL-DIR-2026-05-18-119 / NA-0311

## Executive summary

NA-0311 records qsc harness requirements, test-seam analysis, fixture mapping,
and implementation-authorization prerequisites for future executable evidence
of explicit qsc handshake suite-id semantics.

Inputs inherited from earlier lanes:

- NA-0309 bounded model properties in `formal/model_qsc_handshake_suite_id_bounded.py`.
- NA-0310 deterministic vectors in `inputs/suite2/qsc_handshake_suite_id_vectors_na0310.json`.
- NA-0310 refimpl oracle in
  `tools/refimpl/quantumshield_refimpl/tests/na_0310_qsc_suite_id_vector_oracle.rs`.

Conclusion:

- The current qsc `QHSM` A1/B1/A2 frames are strict fixed-layout v1 frames.
- The existing qsc CLI/relay tests can prove current Suite-2 session-state
  persistence, version/type/malformed frame rejection, and no-mutation/no-output
  boundaries for current frames.
- The existing qsc surface cannot directly consume the NA-0310 explicit
  parameter-block vectors because qsc has no `QHSM` v2 parser or explicit
  suite-id parameter block.
- A test-only fixture builder alone would exercise synthetic bytes or generic
  length rejects, not real qsc admission behavior.
- The selected successor is:

NA-0312 -- qsc Handshake Suite-ID Parameter-Block Implementation Authorization

This lane makes no qsc runtime, QHSM/QSP production wire-format, production
handshake, crypto state-machine, key-schedule, dependency, service, website,
README, START_HERE, workflow, branch-protection, or public-safety change.

## Live NA-0311 scope

Live `NEXT_ACTIONS.md` authorizes a qsc harness requirements and test-seam plan
derived from NA-0310 vectors, NA-0310 refimpl oracle expectations, and NA-0309
model properties. It requires exact inventory of existing qsc/QHSM seams,
future files/tests, implementation-authorization stop points, and governance
evidence.

Live scope forbids production qsc suite-id implementation unless a live
directive explicitly authorizes exact files and semantics. It also forbids
runtime, wire-format, production handshake, key-schedule, crypto state-machine,
dependency, service, website, README, START_HERE, workflow, branch-protection,
and public-safety changes.

The live scope matches this directive. NA-0311 is a planning and requirements
lane only.

## Inherited NA-0310 vectors/refimpl oracle

NA-0310 provided 20 deterministic vector categories:

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

The refimpl oracle validates schema metadata, unique vector IDs, category
coverage, deterministic reason labels, model-property references, accepted
Suite-2 tuple expectations, legacy compatibility separation, reject
no-mutation/no-output/no-leak expectations, and future-gated qsc harness
expectations.

Every qsc harness expectation remains `future_gate`; NA-0310 does not claim
current qsc runtime behavior.

## Sources inspected

- `NEXT_ACTIONS.md` NA-0311 and NA-0310 entries
- `GOALS.md`
- `PROJECT_CHARTER.md`
- `CHECKLIST_PROTOCOL_CHANGE.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `tests/NA-0310_closeout_restore_na0311_testplan.md`
- `docs/governance/evidence/NA-0310_qsc_handshake_suite_id_vector_refimpl_oracle.md`
- `tests/NA-0310_qsc_handshake_suite_id_vector_refimpl_oracle_testplan.md`
- `inputs/suite2/qsc_handshake_suite_id_vectors_na0310.json`
- `tools/refimpl/quantumshield_refimpl/tests/na_0310_qsc_suite_id_vector_oracle.rs`
- `formal/model_qsc_handshake_suite_id_bounded.py`
- `formal/run_model_checks.py`
- `formal/README.md`
- `docs/governance/evidence/NA-0305_qsc_handshake_suite_id_seam_authorization_plan.md`
- `docs/governance/evidence/NA-0306_qsc_handshake_suite_id_wire_format_authorization_plan.md`
- `docs/governance/evidence/NA-0307_qsc_handshake_suite_id_compatibility_transcript_design.md`
- `docs/governance/evidence/NA-0309_qsc_handshake_suite_id_formal_model_properties.md`
- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/src/protocol_state/mod.rs`
- `qsl/qsl-client/qsc/tests/common/mod.rs`
- `qsl/qsl-client/qsc/tests/na_0302_suite2_negotiation_cross_surface.rs`
- `qsl/qsl-client/qsc/tests/na_0303_handshake_activation_negotiation.rs`
- `qsl/qsl-client/qsc/tests/na_0304_handshake_suite_id_negotiation.rs`

Search terms included: qsc harness, test seam, seam, QHSM, suite-id,
suite_id, suiteid, parameter block, vector, refimpl, oracle, frame, parser,
builder, admission, activation, compatibility, legacy, transcript, key context,
no mutation, recv_commit, output, reason label, future_gate, and harness
expectation.

## Current qsc harness/test architecture

Current qsc harnesses use the qsc CLI, a local in-process relay test server,
isolated temp config roots, mock passphrase vaults, route tokens, output-marker
assertions, session-state decrypt/restore helpers, and direct inspection of
relay-carried `QHSM` frame bytes.

Relevant current tests:

- `qsl/qsl-client/qsc/tests/send_commit.rs`
- `qsl/qsl-client/qsc/tests/na_0302_suite2_negotiation_cross_surface.rs`
- `qsl/qsl-client/qsc/tests/na_0303_handshake_activation_negotiation.rs`
- `qsl/qsl-client/qsc/tests/na_0304_handshake_suite_id_negotiation.rs`

Current `QHSM` frame codec:

- A1: `QHSM`, `HS_VERSION=1`, `HS_TYPE_INIT`, session id, KEM public key,
  signature public key, DH public key.
- B1: `QHSM`, `HS_VERSION=1`, `HS_TYPE_RESP`, session id, KEM ciphertext,
  transcript MAC, signature public key, signature, DH public key.
- A2: `QHSM`, `HS_VERSION=1`, `HS_TYPE_CONFIRM`, session id, confirm MAC,
  signature.

Current qsc parser behavior:

- The parser is strict-length and private to `qsl/qsl-client/qsc/src/handshake/mod.rs`.
- Appended parameter bytes reject as generic decode failure.
- Version/type mutations exercise current frame admission, not explicit
  suite-id admission.
- Accepted session state is built by internal Suite-2 constants after the base
  handshake succeeds.

## qsc seam option analysis

| Option | Status | Likely future files | Production behavior change | QHSM/QSP wire-format change | qsc runtime change | Admission coverage | Needs prior implementation authorization | Risk / recommendation |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Existing qsc CLI/relay seam | PARTIAL | Existing qsc tests plus future test | No by itself | No by itself | No by itself | Proves current v1 frame flow, session-state tuple, and current rejects; cannot consume v2 vectors directly | Yes for explicit suite-id admission | Keep as harness transport, but do not claim explicit suite-id admission |
| Existing qsc test helper seam | PARTIAL | `qsl/qsl-client/qsc/tests/common/mod.rs`; future qsc test | No by itself | No by itself | No by itself | Can capture and mutate relay items; cannot call private v2 parser that does not exist | Yes | Useful support only |
| New test-only fixture builder | BLOCKED | Future qsc tests; possibly test helper | No if bytes are synthetic only | No if parser unchanged | No if parser unchanged | Existing parser turns v2-like bytes into generic length/version rejects | Yes if shared parser/runtime is needed | Reject as sufficient evidence by itself |
| Parser-only seam | BLOCKED | Future `handshake/mod.rs` parser helpers and tests if authorized | Yes if production parser is shared | Yes for accepted v2 frame schema | Yes | Could validate parameter blocks and no-mutation boundaries once implemented | Yes | Requires parameter-block implementation authorization first |
| Runtime implementation seam | FEASIBLE after authorization | `qsl/qsl-client/qsc/src/handshake/mod.rs`; qsc tests; governance/spec traceability | Yes | Yes for `QHSM` v2; QSP changes only if explicitly authorized later | Yes | Can prove valid v2 admission, reject taxonomy, no mutation, no output, no leak | Yes | Select authorization successor |
| Formal/vector/refimpl-only continuation | PARTIAL | `formal/**`, `inputs/**`, `tools/refimpl/**`, docs/tests | No | No | No | Strengthens abstract/fixture evidence but not qsc runtime admission | No if planning only | Useful adjunct, not the next critical path |

Required behavior coverage by option:

- Valid v2 suite-id admission: only runtime/parser implementation seam can
  prove this for qsc.
- Unsupported suite-id reject: blocked until qsc accepts explicit parameter
  inputs.
- Downgrade/stripped suite-id reject: blocked until qsc has v2 parameter-block
  semantics and suite-required mode.
- Malformed parameter reject: blocked until qsc has a parameter-block parser.
- No mutation, no `recv_commit`, no output, no leak: current tests prove this
  for existing mutation surfaces; future v2 suite-id vectors need qsc parser
  and runtime admission coverage.

## Selected future seam or blocker

Selected result:

`NA0311_QSC_TEST_SEAM_BLOCKED`

The existing CLI/relay harness should remain the future transport seam, but it
is not sufficient without a shared qsc `QHSM` v2 parser/runtime admission
surface. The next lane should authorize the parameter-block implementation
scope, files, tests, and stop conditions before code changes.

Selected successor:

NA-0312 -- qsc Handshake Suite-ID Parameter-Block Implementation Authorization

## Harness input requirements

A future qsc harness must consume or derive from:

- NA-0310 vector JSON.
- Future `QHSM` v2 A1/B1/A2 frame fixtures or an equivalent shared parser
  fixture that uses the production parser code.
- Explicit compatibility-mode configuration.
- Explicit suite-required-mode configuration.
- Passphrase, route-token, peer identity, and sentinel fixtures.
- Isolated temp config roots, relay queues, output capture, and session-state
  paths.
- Stable expected reason labels.

The harness must reject any fixture that cannot be traced to a vector category,
model property, or explicit implementation-authorization decision.

## Accepted path requirements

The future accepted-path harness must prove:

- A valid v2 Suite-2 parameter-block vector establishes the expected qsc
  session-state tuple.
- B1 echoes and A2 confirms the byte-exact canonical A1 parameter block.
- Transcript context includes the canonical suite context before accepted
  state.
- Key context includes the canonical suite context before accepted state.
- Legacy v1 compatibility is accepted only in explicit compatibility mode and
  is labeled separately from explicit suite-id admission.
- Accepted paths do not leak passphrases, route tokens, plaintext sentinels,
  vector sentinels, stack traces, backtraces, or raw key material.

## Reject path requirements

The future reject-path harness must prove these cases reject before accepted
session state:

- legacy v1 in suite-required mode;
- unsupported `suite_id`;
- downgraded tuple;
- stripped or missing suite context;
- mismatched A1/B1 suite context;
- mismatched B1/A2 suite context;
- duplicate suite parameter;
- unknown critical parameter;
- unknown noncritical parameter unless a future directive explicitly authorizes
  ignore semantics with transcript coverage;
- noncanonical parameter order;
- malformed length, trailing bytes, underflow, overflow, or over-bound block;
- inconsistent `protocol_version` / `suite_id` namespace;
- transcript binding mismatch;
- key-context mismatch or missing required key context;
- replayed A1 with suite context; and
- replayed A2 with suite context, or a deterministic no-op if a future design
  explicitly permits that behavior.

## No-mutation requirements

For every reject vector, the future qsc harness must prove:

- no accepted session-state file is created or changed;
- no pending state is promoted to accepted state;
- no outbound B1 or A2 is emitted after a reject that should stop before that
  stage;
- no receive timeline, peer-confirmed state, or send-ready state advances;
- no temp output artifact survives reject; and
- repeated reject is deterministic.

## No-output requirements

For every reject vector, the future qsc harness must prove:

- no `event=recv_commit`;
- no decrypted plaintext output;
- no qsp unpack success output;
- no file/attachment completion output;
- no positive handshake-complete marker; and
- no partial output artifact remains after cleanup.

## No-leak requirements

For every accepted and reject path, the future qsc harness must scan stdout,
stderr, temp artifacts, and relevant state roots for:

- mock vault passphrase;
- route tokens;
- plaintext sentinel;
- vector secret sentinel;
- panic text;
- stack backtrace;
- raw KEM, DH, AEAD, chain-key, root-key, or skipped-message key material.

## Future markers

The future qsc harness or authorization lane must emit or require these markers
when the corresponding evidence exists:

- `NA0311_QSC_HARNESS_REQUIREMENTS_OK`
- `NA0311_QSC_TEST_SEAM_BLOCKED`
- `NA0311_QSC_VALID_V2_EXPECTATION_OK`
- `NA0311_QSC_LEGACY_COMPAT_EXPECTATION_OK`
- `NA0311_QSC_REQUIRED_MODE_REJECT_EXPECTATION_OK`
- `NA0311_QSC_UNSUPPORTED_SUITE_REJECT_EXPECTATION_OK`
- `NA0311_QSC_DOWNGRADE_REJECT_EXPECTATION_OK`
- `NA0311_QSC_MALFORMED_REJECT_EXPECTATION_OK`
- `NA0311_QSC_NO_MUTATION_EXPECTATION_OK`
- `NA0311_QSC_NO_OUTPUT_EXPECTATION_OK`
- `NA0311_QSC_NO_LEAK_EXPECTATION_OK`

If a later implementation lane identifies a safe shared parser seam before
runtime behavior changes, it may use `NA0311_QSC_TEST_SEAM_IDENTIFIED_OK`, but
NA-0311 does not find that seam in the current code.

## Implementation authorization requirements

### If a future test-only seam is proposed

A future directive must prove all of these before implementation:

- exact files are authorized;
- helper API is named and bounded;
- the helper consumes NA-0310 vector categories;
- production behavior is untouched, or every shared production parser call is
  explicitly authorized;
- tests prove synthetic fixtures are not described as qsc admission behavior;
- markers include `NA0311_QSC_TEST_SEAM_IDENTIFIED_OK`; and
- stop conditions reject generic length/version failures as insufficient for
  suite-id admission evidence.

Likely test-only files, if this path becomes viable:

- `qsl/qsl-client/qsc/tests/common/mod.rs`
- `qsl/qsl-client/qsc/tests/na_0312_handshake_suite_id_parameter_block.rs`
- `tests/NA-0312_*_testplan.md`

Current result: not sufficient.

### If qsc runtime / QHSM v2 implementation is required

A future directive must explicitly authorize:

- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- future qsc integration tests for NA-0312/NA-0313 as named by that directive
- governance evidence, testplan, decision, traceability, and rolling journal
  files
- any canonical spec/design file if the directive classifies the change as
  protocol-significant

Required prerequisites:

- D-0601 is present and accepted.
- NA-0307 compatibility/transcript/key-context posture remains authoritative or
  is updated by a new decision.
- NA-0310 vectors remain valid or are updated in the same authorized lane.
- Transcript binding and key-context binding are implemented before accepted
  state.
- Compatibility mode and suite-required mode are explicit.
- Reject reason labels remain deterministic.
- No dependency update is hidden in the implementation lane.

External review consideration:

- The parameter-block implementation authorization should record that external
  review remains a future readiness gate for broad claims. It must not imply
  that implementation authorization equals external review.

## Coverage matrix

| NA-0310 vector category | NA-0309 model property | Refimpl oracle assertion | Required qsc fixture | Required seam | Current status | Expected future artifact | Risk | Next action |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| valid_v2_suite2_parameter_block | P1/P11/P18 | accepted explicit suite | A1/B1/A2 v2 canonical block | QHSM v2 parser/runtime | PROVEN_MODEL; PROVEN_VECTOR; PROVEN_REFIMPL_ORACLE; BLOCKED_ON_QSC_RUNTIME; BLOCKED_ON_WIRE_FORMAT | qsc accepted-path harness | State evidence overclaimed as admission | Parameter-block authorization |
| legacy_v1_compatibility_allowed | P2/P19 | compatibility accept only | v1 A1/B1/A2 with compatibility mode | policy-gated qsc harness | PROVEN_MODEL; PROVEN_VECTOR; PROVEN_REFIMPL_ORACLE; READY_FOR_IMPLEMENTATION_AUTHORIZATION | compatibility harness | Compatibility mislabeled as admission | Keep separate result label |
| legacy_v1_rejected_in_suite_required_mode | P3/P12 | legacy required reject | v1 A1 in suite-required mode | policy-gated qsc harness | PROVEN_MODEL; PROVEN_VECTOR; PROVEN_REFIMPL_ORACLE; BLOCKED_ON_QSC_RUNTIME | required-mode reject test | Silent fallback | Authorize required mode |
| unsupported_suite_id | P4/P12/P20 | unsupported suite reject | v2 unsupported tuple | QHSM v2 parser/runtime | PROVEN_MODEL; PROVEN_VECTOR; PROVEN_REFIMPL_ORACLE; BLOCKED_ON_WIRE_FORMAT | unsupported-suite reject test | Generic decode overclaim | Authorize parser |
| downgraded_suite_id | P5/P19/P20 | downgrade reject | v2 legacy tuple | QHSM v2 parser/runtime | PROVEN_MODEL; PROVEN_VECTOR; PROVEN_REFIMPL_ORACLE; BLOCKED_ON_WIRE_FORMAT | downgrade reject test | Fallback ambiguity | Authorize parser and mode |
| stripped_suite_id_parameter | P6/P12 | missing suite reject | v2 block missing suite param | QHSM v2 parser/runtime | PROVEN_MODEL; PROVEN_VECTOR; PROVEN_REFIMPL_ORACLE; BLOCKED_ON_WIRE_FORMAT | stripped/missing reject test | Stripping accepted as v1 | Authorize no fallback |
| mismatched_suite_id_A1_B1 | P7/P12 | context mismatch | A1 Suite-2, B1 mismatch | shared transcript parser | PROVEN_MODEL; PROVEN_VECTOR; PROVEN_REFIMPL_ORACLE; BLOCKED_ON_QSC_RUNTIME | initiator mismatch test | Commit before mismatch reject | Bind transcript before commit |
| mismatched_suite_id_B1_A2 | P8/P12 | context mismatch | B1 Suite-2, A2 mismatch | shared transcript parser | PROVEN_MODEL; PROVEN_VECTOR; PROVEN_REFIMPL_ORACLE; BLOCKED_ON_QSC_RUNTIME | responder mismatch test | Responder commit before reject | Bind confirm before commit |
| duplicate_suite_id_parameter | P9/P12 | duplicate reject | v2 duplicate param | parameter parser | PROVEN_MODEL; PROVEN_VECTOR; PROVEN_REFIMPL_ORACLE; BLOCKED_ON_WIRE_FORMAT | duplicate reject test | Parser ambiguity | Canonical parser |
| unknown_critical_parameter | P10/P12 | unknown critical reject | v2 unknown critical | parameter parser | PROVEN_MODEL; PROVEN_VECTOR; PROVEN_REFIMPL_ORACLE; BLOCKED_ON_WIRE_FORMAT | unknown-critical reject test | Value leak in logs | Redacted reason |
| unknown_noncritical_parameter | P12/P20 | reject default | v2 unknown noncritical | parameter parser | PROVEN_VECTOR; PROVEN_REFIMPL_ORACLE; FUTURE_GATE | policy test | Silent ignore drift | Reject unless later authorized |
| noncanonical_parameter_order | P11/P12 | noncanonical reject | v2 out-of-order params | parameter parser | PROVEN_MODEL; PROVEN_VECTOR; PROVEN_REFIMPL_ORACLE; BLOCKED_ON_WIRE_FORMAT | order reject test | Alternate encodings | Strict canonicalization |
| malformed_parameter_length | P12/P20 | malformed reject | v2 malformed length | parameter parser | PROVEN_MODEL; PROVEN_VECTOR; PROVEN_REFIMPL_ORACLE; BLOCKED_ON_WIRE_FORMAT | length reject test | Panic or byte dump | Bounds-first parser |
| inconsistent_protocol_version_suite_id | P13/P12 | tuple reject | v2 inconsistent namespace tuple | parameter parser | PROVEN_MODEL; PROVEN_VECTOR; PROVEN_REFIMPL_ORACLE; BLOCKED_ON_WIRE_FORMAT | tuple reject test | Namespace confusion | Tuple registry decision |
| replayed_A1_with_suite_context | P12/P16/P17 | replay reject | repeated A1 with suite context | runtime replay harness | PROVEN_VECTOR; PROVEN_REFIMPL_ORACLE; FUTURE_GATE | replay A1 test | Duplicate pending state | Deterministic no mutation |
| replayed_A2_with_suite_context | P12/P16/P17 | replay reject | repeated A2 with suite context | runtime replay harness | PROVEN_VECTOR; PROVEN_REFIMPL_ORACLE; FUTURE_GATE | replay A2 test | Duplicate accepted state | Deterministic no mutation/no-op |
| valid_suite2_with_transcript_binding | P1/P14/P18 | transcript label accept | v2 canonical block with transcript hash | transcript-aware runtime | PROVEN_MODEL; PROVEN_VECTOR; PROVEN_REFIMPL_ORACLE; BLOCKED_ON_QSC_RUNTIME | transcript accept test | Missing byte binding | Authorize transcript binding |
| transcript_binding_mismatch | P14/P16 | transcript reject | v2 tuple with wrong transcript context | transcript-aware runtime | PROVEN_MODEL; PROVEN_VECTOR; PROVEN_REFIMPL_ORACLE; BLOCKED_ON_QSC_RUNTIME | transcript mismatch test | State commit before detection | Reject before store |
| key_schedule_context_mismatch | P15/P17 | key context reject | v2 tuple with wrong key context | key-context runtime | PROVEN_MODEL; PROVEN_VECTOR; PROVEN_REFIMPL_ORACLE; BLOCKED_ON_QSC_RUNTIME | key-context mismatch test | KDF context ambiguity | Authorize key-context binding |
| missing_key_context_in_required_mode | P15/P16 | missing key context reject | v2 tuple without required key context | key-context runtime | PROVEN_MODEL; PROVEN_VECTOR; PROVEN_REFIMPL_ORACLE; BLOCKED_ON_QSC_RUNTIME | missing key-context test | Accepted state without binding | Stop if key context absent |

## Selected successor

Selected successor:

NA-0312 -- qsc Handshake Suite-ID Parameter-Block Implementation Authorization

Rationale:

- NA-0309 and NA-0310 already provide model/vector/oracle evidence.
- The current qsc runtime lacks the explicit `QHSM` v2 input surface needed for
  direct qsc admission evidence.
- A test-only builder would not be sufficient unless it uses an authorized
  shared parser/runtime path.
- The next safe lane is not implementation itself; it is authorization that
  freezes exact files, semantics, tests, markers, and stop conditions for a
  future parameter-block implementation lane.

Rejected successors:

- qsc Handshake Suite-ID Test-Seam Implementation Authorization: rejected
  because no sufficient test-only seam exists in the current code.
- qsc Handshake Suite-ID qsc Harness Implementation: premature before
  parameter-block implementation authorization.
- qsc Handshake Suite-ID qsc Harness Blocker Resolution: not needed because
  the blocker is understood; it is missing authorized parser/runtime surface.
- qsc Handshake Suite-ID Vector/Model Refinement: useful later only if
  implementation authorization changes semantics.
- Metadata Runtime Identifier and Default Padding Transition Plan: important,
  but the suite-id queue remains on a critical path.

## Metadata-reduction agenda statement

Metadata runtime identifier/default-padding work remains a near-term priority.
Because the explicit qsc suite-id admission lane is still on a critical path,
NA-0312 should stay in the qsc authorization sequence. A metadata runtime
identifier/default-padding transition lane should be inserted immediately after
the next qsc authorization milestone unless the live queue provides a stronger
security blocker.

NA-0311 does not implement metadata runtime behavior.

## No qsc runtime/protocol/wire implementation change proof

NA-0311 changes only governance, evidence, testplan, traceability, decision,
and rolling journal files. It does not change:

- `qsl/qsl-client/qsc/src/**`
- `qsl/qsl-client/qsc/tests/**`
- QHSM/QSP production wire-format implementation paths
- production handshake implementation
- crypto state-machine implementation
- key schedule implementation
- `Cargo.toml`
- `Cargo.lock`
- `.github/**`
- qsl-server implementation
- qsl-attachments implementation
- qsc-desktop implementation
- website or external website sources
- `README.md`
- `START_HERE.md`
- branch-protection or public-safety configuration

## Backup-plan impact statement

No backup-plan update is required. NA-0311 changes only tracked qsl-protocol
governance, evidence, testplan, traceability, decision, and rolling journal
files under `/srv/qbuild/work`, which is already in the local backup scope. It
creates no non-rebuildable artifact outside the repository worktree and does
not move evidence roots, response roots, source roots, or excluded backup
paths.

## Next recommendation

After this PR merges and post-merge public-safety is green, close out NA-0311
and restore exactly one READY successor:

NA-0312 -- qsc Handshake Suite-ID Parameter-Block Implementation Authorization

The NA-0312 directive should authorize no implementation until it has frozen
exact files, parser/runtime semantics, harness requirements, CI markers, and
stop conditions.
