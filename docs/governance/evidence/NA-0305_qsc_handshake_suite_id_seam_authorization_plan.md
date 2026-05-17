Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-17
Replaces: n/a
Superseded-By: n/a

# NA-0305 qsc Handshake Suite-ID Seam Authorization Plan

Directive: QSL-DIR-2026-05-17-113 / NA-0305

## Executive summary

NA-0305 is a planning and authorization lane only. It does not implement a qsc
suite-id seam, does not change qsc runtime source, does not change protocol or
crypto behavior, and does not change dependencies.

The smallest truthful next lane is:

NA-0306 -- qsc Handshake Suite-ID Wire-Format Change Authorization Plan

Reason: the current qsc `QHSM` A1/B1/A2 handshake frames have no explicit
suite-id field. Existing qsc tests can prove accepted Suite-2 session state and
can reject malformed or version/type-mutated frames, but they cannot express
unsupported, downgrade-like, or malformed suite-id admission inputs at the qsc
handshake admission surface. A test-only frame builder that appends suite-id
bytes would be rejected by the current strict parser as a length error, which
would not prove suite-id-specific admission behavior.

## Live NA-0305 scope

Live `NEXT_ACTIONS.md` authorizes a narrow authorization, design, and proof
boundary for an explicit qsc handshake suite-id seam. It forbids qsc suite-id
seam implementation during NA-0305 and preserves these boundaries:

- no protocol or wire semantic change by default;
- no crypto state-machine, key schedule, or production handshake
  implementation change;
- no dependency, workflow, website, service implementation, docs/public,
  README, START_HERE, branch-protection, or public-safety configuration drift;
- no unsupported public-internet, external-review, anonymity, metadata-free, or
  untraceable claims; and
- exactly one READY item remains NA-0305 until a separate closeout.

## Inherited NA-0304 blocker

NA-0304 proved the valid qsc handshake path persists Suite-2 session state with
protocol version `0x0500` and suite id `0x0002`. It also proved that qsc `QHSM`
A1, B1, and A2 frames expose magic, frame version, frame type, session id, KEM
material, signature material, MACs, signatures, and DH public keys, but no
explicit suite-id handshake admission field.

The blocker remains:

- unsupported suite-id admission input cannot be represented in the current
  qsc handshake frame;
- downgrade-like suite-id admission input cannot be represented in the current
  qsc handshake frame;
- malformed suite-id admission input cannot be represented in the current qsc
  handshake frame; and
- inferred Suite-2 state after successful activation is not explicit suite-id
  admission evidence.

## Sources inspected

- `NEXT_ACTIONS.md` NA-0305 and NA-0304 entries
- `tests/NA-0304_closeout_restore_na0305_testplan.md`
- `docs/governance/evidence/NA-0304_qsc_handshake_suite_id_negotiation_harness.md`
- `tests/NA-0304_qsc_handshake_suite_id_negotiation_testplan.md`
- `qsl/qsl-client/qsc/tests/na_0304_handshake_suite_id_negotiation.rs`
- `docs/governance/evidence/NA-0303_qsc_handshake_activation_negotiation_harness.md`
- `qsl/qsl-client/qsc/tests/na_0303_handshake_activation_negotiation.rs`
- `docs/governance/evidence/NA-0302_suite2_negotiation_vector_qsc_cross_surface_harness.md`
- `docs/governance/evidence/NA-0301_suite2_negotiation_downgrade_harness.md`
- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/src/main.rs`
- `qsl/qsl-client/qsc/src/protocol_state/mod.rs`
- `tools/refimpl/quantumshield_refimpl/src/qsp/types.rs`
- `tools/refimpl/quantumshield_refimpl/src/qsp/handshake.rs`
- `tools/refimpl/quantumshield_refimpl/src/suite2/types.rs`
- `tools/refimpl/quantumshield_refimpl/src/suite2/state.rs`
- `formal/README.md`
- `formal/model_suite2_negotiation_bounded.py`
- `TRACEABILITY.md`
- `DECISIONS.md`

## Existing qsc handshake/session evidence

The qsc handshake source uses private fixed-layout message structs:

- `HsInit`: session id, KEM public key, signature public key, DH public key.
- `HsResp`: session id, KEM ciphertext, transcript MAC, signature public key,
  signature, DH public key.
- `HsConfirm`: session id, confirm MAC, signature.

The frame codec in `qsl/qsl-client/qsc/src/handshake/mod.rs` writes and reads:

- `QHSM` magic;
- `HS_VERSION`;
- `HS_TYPE_INIT`, `HS_TYPE_RESP`, or `HS_TYPE_CONFIRM`;
- session id; and
- the frame-specific cryptographic fields above.

The accepted Suite-2 tuple is supplied internally by `hs_build_session`, which
passes `SUITE2_PROTOCOL_VERSION` and `SUITE2_SUITE_ID` constants into
`init_from_base_handshake`.

NA-0304 rerun markers confirmed the current accepted-state boundary:

- `NA0304_QSC_HANDSHAKE_SESSION_SUITE2_STATE_OK`
- `NA0304_QSC_QHSM_NO_EXPLICIT_SUITE_ID_FIELD_OK`
- `NA0304_QSC_SUITE_ID_SEAM_BLOCKED`
- `NA0304_NO_IMPLEMENTATION_CHANGE_OK`
- `NA0304_BLOCKER_EVIDENCE_OK`

## Missing explicit suite-id admission field evidence

The current qsc `QHSM` parser is strict-length. Adding extra bytes for a
suite-id fixture at the test layer would cause a generic decode failure before
any suite-id-specific policy is reached. Mutating `HS_VERSION` or `HS_TYPE_*`
continues to prove only handshake frame version/type admission behavior.

The qsc receive path and refimpl/vector paths already expose suite-id values:

- NA-0301 covers refimpl negotiation and wire reject behavior.
- NA-0302 covers dedicated vectors and qsc receive-path suite-id mutation.
- NA-0303 covers qsc handshake activation/admission version, malformed, and
  replay rejects.
- NA-0304 covers qsc accepted Suite-2 session state and records the missing
  explicit handshake suite-id field.

None of those surfaces is an explicit qsc handshake suite-id admission field.

## Option analysis

| Option | Feasibility | Likely files | Production behavior change | Wire-format change | Test-only | Admission proof | Overclaim risk | Recommended successor shape |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 1. Test-only frame builder/parser helper | BLOCKED for real qsc admission | Future tests only would be insufficient; a source helper would touch `qsl/qsl-client/qsc/src/handshake/mod.rs` | No if tests only, yes if parser behavior changes | No if tests only, yes if accepted frame schema changes | Only for synthetic rejection | Generic length reject only, not suite-id admission | High if described as qsc handshake suite-id admission evidence | Do not select as executable seam without prior wire-format authorization |
| 2. Test-only mutated fixture around existing QHSM frames | PARTIAL and already covered | `qsl/qsl-client/qsc/tests/na_0303_*`, `qsl/qsl-client/qsc/tests/na_0304_*` | No | No | Yes | Version/type/malformed frame admission only | Medium if called suite-id behavior | Keep as existing supporting evidence only |
| 3. Production QHSM frame schema addition | FEASIBLE only after design authorization | Design docs first; later `qsl/qsl-client/qsc/src/handshake/mod.rs` and qsc tests if approved | Yes | Yes, qsc handshake wire/schema behavior | No | Yes, if designed and implemented later with negative tests | Low if staged honestly | Select NA-0306 as design/authorization lane |
| 4. qsc public API seam | BLOCKED | No existing CLI/API exposes handshake suite id | No existing change path | No existing change path | Not currently | No | High if state injection is treated as admission | Do not select unless a future audit finds a real API |
| 5. Refimpl/vector proof only | PARTIAL and already covered | Existing `tools/refimpl/**`, `inputs/suite2/**` | No | No | Yes | Refimpl/vector and qsc receive path, not qsc handshake | Medium | Keep as supporting evidence, not successor for qsc handshake admission |
| 6. Formal/model seam | PARTIAL | Existing `formal/**` plus docs if later authorized | No | No | Yes | Abstract negotiation properties only | Medium | Useful adjunct after frame semantics are authorized |

## Selected future lane

Selected successor:

NA-0306 -- qsc Handshake Suite-ID Wire-Format Change Authorization Plan

NA-0306 should be design/authorization only unless its live directive explicitly
authorizes a narrower implementation scope. Its job is to decide the exact
`QHSM` suite-id/capability field semantics, compatibility posture, transcript
binding expectations, fail-closed reject taxonomy, and test markers needed
before any qsc handshake schema implementation.

## Future NA-0306 likely files

Expected NA-0306 planning files:

- `docs/governance/evidence/NA-0306_qsc_handshake_suite_id_wire_format_authorization_plan.md`
- `tests/NA-0306_qsc_handshake_suite_id_wire_format_authorization_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- optional `docs/design/qsc_handshake_suite_id_wire_format_plan.md` if the live
  NA-0306 directive authorizes a design companion doc

Deferred implementation files for a later, separately authorized lane may
include:

- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/tests/na_0307_handshake_suite_id_admission.rs`
- canonical spec or traceability updates if the design classifies the change as
  protocol-significant

Those deferred files are not authorized by NA-0305.

## Future NA-0306 expected markers

NA-0306 should define design markers such as:

- `NA0306_QSC_QHSM_SUITE_ID_WIRE_FORMAT_AUTHORIZATION_OK`
- `NA0306_QSC_SUITE_ID_ADMISSION_TEST_REQUIREMENTS_OK`
- `NA0306_NO_IMPLEMENTATION_CHANGE_OK`
- `NA0306_CLAIM_BOUNDARY_OK`

A later executable lane, if authorized after NA-0306, should require markers for
unsupported suite-id reject, downgrade-like suite-id reject, malformed suite-id
reject, no accepted-state mutation on rejects, no `recv_commit` or qsp output
on rejects, no panic/backtrace, and no secret or sentinel leakage.

## Stop conditions

Future work must STOP if:

- it would add or alter qsc `QHSM` frame fields without explicit
  wire-format/protocol authorization;
- it would treat a generic decode length reject as suite-id-specific admission
  behavior;
- it would infer suite-id admission behavior only from persisted state;
- it would alter key schedule, SCKA, crypto state-machine, or production
  handshake semantics outside the authorized scope;
- it would change dependencies, workflows, branch protection, public-safety
  configuration, services, website, README, START_HERE, or docs/public without
  explicit scope; or
- required CI or public-safety fails conclusively.

## Claim boundaries

NA-0305 records a plan, not implementation.

Allowed claims:

- qsc currently persists Suite-2 session state after valid handshake activation.
- qsc currently lacks an explicit handshake suite-id admission field.
- explicit qsc handshake suite-id admission testing requires a future authorized
  schema/design decision or a newly discovered real public/test API.

Forbidden claims:

- qsc unsupported, downgrade-like, or malformed handshake suite-id admission is
  already tested directly.
- inferred Suite-2 state is explicit qsc handshake suite-id admission evidence.
- qsc is production-ready or public-internet ready.
- external review is complete.
- the system provides anonymity, metadata-free behavior, or untraceability.

## No implementation change proof

NA-0305 changes only governance, evidence, testplan, traceability, and rolling
journal documents. It does not change:

- qsc runtime source under `qsl/qsl-client/qsc/src/**`;
- qsc integration tests under `qsl/qsl-client/qsc/tests/**`;
- QSP/refimpl/protocol-core implementation source;
- crypto state-machine, key schedule, production handshake implementation, or
  QSP wire-format implementation;
- Cargo manifests or locks;
- workflows or scripts;
- qsl-server, qsl-attachments, qsc-desktop, apps, formal models, inputs, tools
  implementation paths, website/external website, README, START_HERE, docs/public,
  branch protection, or public-safety configuration.

## Next recommendation

Close NA-0305 after the planning PR merges and post-merge public-safety is
green, then restore exactly one READY successor:

NA-0306 -- qsc Handshake Suite-ID Wire-Format Change Authorization Plan

NA-0306 should remain a design/authorization lane. It should not implement the
suite-id frame/schema change unless a later live directive explicitly changes
that scope.
