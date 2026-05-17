Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-17
Replaces: n/a
Superseded-By: n/a

Goals: G1, G2, G3, G4, G5

# NA-0309 qsc Handshake Suite-ID Formal Model Properties

Directive: QSL-DIR-2026-05-17-117 / NA-0309

## Executive summary

NA-0309 adds an executable bounded formal/model artifact for future explicit
qsc handshake suite-id semantics. The model is implemented in
`formal/model_qsc_handshake_suite_id_bounded.py` and is run by
`formal/run_model_checks.py`.

This lane does not implement a qsc runtime parser, QHSM wire-format change,
QSP wire-format change, production handshake behavior, crypto state-machine
change, key schedule change, dependency change, service change, website change,
or public-safety configuration change.

Selected successor:

NA-0310 -- qsc Handshake Suite-ID Vector Schema and Refimpl Oracle

## Live NA-0309 scope

Live `NEXT_ACTIONS.md` authorizes executable bounded formal/model properties
for qsc handshake suite-id canonical context, transcript context, key-schedule
context, legacy required-mode reject, compatibility policy, reject/no-mutation,
and reject/no-output behavior.

Live scope forbids qsc runtime, QHSM wire-format, production handshake, key
schedule, QSP wire-format, vector pack, refimpl oracle, dependency, workflow,
website, service implementation, docs/public, README, START_HERE,
branch-protection, and public-safety configuration drift unless a future
directive authorizes exact files and semantics.

The live scope permits this formal/model implementation.

## Inherited NA-0308 design

NA-0308 selected executable formal/model properties as the smallest safe next
lane before vector/refimpl or qsc parameter-block implementation work.

Inherited requirements:

- future explicit qsc suite-id admission uses a version-gated QHSM v2
  negotiated-parameter block;
- QHSM v1 frames have no explicit suite-id semantics;
- v1 compatibility acceptance is explicit and not suite-id admission evidence;
- suite-id-required mode rejects legacy, missing, stripped, mismatched,
  duplicate, unknown, noncanonical, malformed, and inconsistent tuple inputs;
- A1/B1/A2 must carry byte-exact canonical parameter-block context;
- transcript context and key context must include that context before accepted
  state; and
- persisted Suite-2 session state remains supporting evidence only.

## Sources inspected

- `NEXT_ACTIONS.md` NA-0309 entry
- `tests/NA-0308_closeout_restore_na0309_testplan.md`
- `docs/governance/evidence/NA-0308_qsc_handshake_suite_id_formal_vector_design.md`
- `tests/NA-0308_qsc_handshake_suite_id_formal_vector_design_testplan.md`
- `docs/governance/evidence/NA-0307_qsc_handshake_suite_id_compatibility_transcript_design.md`
- `formal/README.md`
- `formal/model_scka_bounded.py`
- `formal/model_suite2_negotiation_bounded.py`
- `formal/run_model_checks.py`
- `qsl/qsl-client/qsc/src/handshake/mod.rs`
- `qsl/qsl-client/qsc/src/protocol_state/mod.rs`
- `qsl/qsl-client/qsc/tests/na_0302_suite2_negotiation_cross_surface.rs`
- `qsl/qsl-client/qsc/tests/na_0303_handshake_activation_negotiation.rs`
- `qsl/qsl-client/qsc/tests/na_0304_handshake_suite_id_negotiation.rs`
- `tools/refimpl/quantumshield_refimpl/src/qsp/handshake.rs`
- `tools/refimpl/quantumshield_refimpl/src/qsp/types.rs`
- `inputs/suite2/vectors/README.md`
- `TRACEABILITY.md`
- `DECISIONS.md`

## Existing formal/model coverage

Existing executable formal coverage before NA-0309:

- `formal/model_scka_bounded.py`: bounded SCKA ADV monotonicity, one-time CTXT
  consumption, tombstones, reject/no-state-change, and transactional commit.
- `formal/model_suite2_negotiation_bounded.py`: bounded Suite-2 required
  negotiation downgrade, capability-commitment mismatch, transcript-suite
  mismatch, and no-mutation on reject.
- `formal/run_model_checks.py`: CI-friendly runner for those models.

## New model file

New file:

- `formal/model_qsc_handshake_suite_id_bounded.py`

Runner update:

- `formal/run_model_checks.py` now runs the NA-0309 model after preserving the
  existing SCKA and Suite-2 negotiation checks.

README update:

- `formal/README.md` documents the NA-0309 model scope and limits.

## Model assumptions

- The model describes future qsc suite-id semantics only.
- The model is bounded and finite-state.
- The model treats QHSM v2 parameter-block bytes as abstract canonical tuples.
- Transcript and key-context binding are explicit abstract context equality
  checks, not cryptographic security proofs.
- Rejected cases do not mutate modeled accepted state, emit output, emit
  recv_commit, or leak the sentinel.
- Legacy compatibility mode is explicit and does not create explicit suite-id
  admission evidence.

## Model properties

| ID | Property | Status |
| --- | --- | --- |
| P1 | valid v2 Suite-2 parameter block accepts | PROVEN_MODEL |
| P2 | valid legacy v1 accepts only in explicit compatibility mode | PROVEN_MODEL |
| P3 | legacy v1 rejects in suite-required mode | PROVEN_MODEL |
| P4 | unsupported suite_id rejects before mutation | PROVEN_MODEL |
| P5 | downgraded suite_id rejects before mutation | PROVEN_MODEL |
| P6 | stripped suite_id rejects before mutation | PROVEN_MODEL |
| P7 | mismatched A1/B1 suite_id rejects before mutation | PROVEN_MODEL |
| P8 | mismatched B1/A2 suite_id rejects before mutation | PROVEN_MODEL |
| P9 | duplicate suite_id rejects before mutation | PROVEN_MODEL |
| P10 | unknown critical parameter rejects before mutation | PROVEN_MODEL |
| P11 | noncanonical parameter order rejects before mutation | PROVEN_MODEL |
| P12 | malformed length rejects before mutation | PROVEN_MODEL |
| P13 | inconsistent protocol_version/suite_id rejects before mutation | PROVEN_MODEL |
| P14 | transcript binding missing or mismatched rejects before mutation | PROVEN_MODEL |
| P15 | key context missing or mismatched rejects before mutation | PROVEN_MODEL |
| P16 | rejected cases emit no output or recv_commit | PROVEN_MODEL |
| P17 | rejected cases have no secret/sentinel leak | PROVEN_MODEL |
| P18 | accepted valid cases preserve Suite-2 semantics | PROVEN_MODEL |
| P19 | no downgrade path from suite-required mode to compatibility mode | PROVEN_MODEL |
| P20 | deterministic reason label for every reject | PROVEN_MODEL |

## Scenario counts

Direct model output:

- scenarios explored: 18
- accepts: 2
- rejects: 16
- no-mutation assertions: 16
- no-output assertions: 16
- no-leak assertions: 16
- no-downgrade assertions: 16
- deterministic reason labels: 16

The two accepts are the valid v2 Suite-2 path and explicit legacy
compatibility path. The compatibility accept is not explicit suite-id admission
evidence.

## Markers

The model emits:

- `NA0309_MODEL_VALID_V2_SUITE2_OK`
- `NA0309_MODEL_LEGACY_COMPATIBILITY_OK`
- `NA0309_MODEL_LEGACY_REQUIRED_REJECT_OK`
- `NA0309_MODEL_UNSUPPORTED_SUITE_REJECT_OK`
- `NA0309_MODEL_DOWNGRADE_REJECT_OK`
- `NA0309_MODEL_STRIPPED_SUITE_REJECT_OK`
- `NA0309_MODEL_MISMATCH_REJECT_OK`
- `NA0309_MODEL_DUPLICATE_REJECT_OK`
- `NA0309_MODEL_UNKNOWN_CRITICAL_REJECT_OK`
- `NA0309_MODEL_NONCANONICAL_REJECT_OK`
- `NA0309_MODEL_MALFORMED_REJECT_OK`
- `NA0309_MODEL_TRANSCRIPT_BINDING_OK`
- `NA0309_MODEL_KEY_CONTEXT_OK`
- `NA0309_MODEL_NO_MUTATION_ON_REJECT_OK`
- `NA0309_MODEL_NO_OUTPUT_ON_REJECT_OK`
- `NA0309_MODEL_NO_SECRET_LEAK_OK`
- `NA0309_MODEL_NO_DOWNGRADE_PATH_OK`
- `NA0309_MODEL_REASON_LABELS_OK`
- `NA0309_QSC_HANDSHAKE_SUITE_ID_FORMAL_MODEL_OK`

## No-mutation proof

For every reject, the model snapshots accepted-state fields before evaluation
and asserts the post-reject snapshot is identical. Reject accounting is not
persisted into the modeled accepted state.

## No-output proof

Every reject outcome asserts `output_emitted == False` and
`recv_commit == False`. Accepted paths may emit output; reject paths cannot.

## No-secret-leak proof

Every reject outcome asserts `secret_leak == False`. Sentinel-bearing malformed
and unknown-critical scenarios also assert the sentinel string is not included
in deterministic reason labels.

## No-downgrade-path proof

Suite-required scenarios assert that reject outcomes retain suite-required
mode and never mutate into legacy compatibility mode. Legacy v1 acceptance is
exercised only by the explicit compatibility-mode scenario.

## Limitations

- This is a bounded model, not a complete protocol proof.
- This is not a cryptographic proof, AEAD proof, parser memory-safety proof, or
  qsc runtime proof.
- No QHSM/QSP wire-format implementation is added.
- No qsc suite-id production field is added.
- No vector schema or refimpl oracle is implemented in NA-0309.
- Current qsc persisted Suite-2 state remains supporting state evidence only;
  it is not explicit qsc handshake suite-id admission evidence.

## Coverage matrix

| Requirement | Current status | Future artifact | Risk | Next action |
| --- | --- | --- | --- | --- |
| Valid v2 Suite-2 parameter context | PROVEN_MODEL | qsc suite-id vector + refimpl oracle | Future bytes not frozen as fixture | NA-0310 vector schema/refimpl |
| Legacy v1 compatibility allowed only by policy | PROVEN_MODEL | compatibility vectors | Compatibility accept overclaimed as admission | Keep separate expected result |
| Legacy v1 required-mode reject | PROVEN_MODEL | qsc harness once v2 parser exists | Silent fallback | Vector + qsc reject harness |
| Unsupported suite id reject | PROVEN_MODEL | vector category + oracle reason | No runtime injection surface yet | NA-0310 fixture/oracle |
| Downgraded suite id reject | PROVEN_MODEL | vector category + oracle reason | Downgrade ambiguity | NA-0310 fixture/oracle |
| Stripped suite id reject | PROVEN_MODEL | A1/B1/A2 stripped vectors | Stripping not representable today | NA-0310 fixture/oracle |
| A1/B1 and B1/A2 mismatch reject | PROVEN_MODEL | mismatch vectors | Echo/confirm drift | NA-0310 fixture/oracle |
| Duplicate/unknown/noncanonical/malformed reject | PROVEN_MODEL | malformed-block vectors | Parser divergence | NA-0310 fixture/oracle |
| Transcript context binding | PROVEN_MODEL | context-label vector fields | Runtime transcript path not implemented | NA-0310 expected context fields |
| Key context binding | PROVEN_MODEL | key-context vector fields | Future KDF input ambiguity | NA-0310 expected context fields |
| No output / no recv_commit on reject | PROVEN_MODEL | qsc harness when parser exists | Output before reject | Future qsc harness lane |
| No secret leak on reject | PROVEN_MODEL | qsc harness leak scans | Secret/log exposure | Future qsc harness lane |
| Refimpl oracle | READY_FOR_REFIMPL | oracle parser/checker | No oracle yet | NA-0310 |
| qsc runtime harness | READY_FOR_QSC_HARNESS | QHSM v2 harness after authorization | No runtime field yet | Later explicit qsc lane |
| QHSM/QSP implementation | FUTURE_GATE | separate implementation authorization | Wire/runtime drift | Not NA-0309 |

## Selected successor

Selected successor:

NA-0310 -- qsc Handshake Suite-ID Vector Schema and Refimpl Oracle

Rationale: the bounded model now freezes the abstract accept/reject,
compatibility, transcript/key-context, no-mutation, no-output, no-leak, and
reason-label boundaries. The next narrow lane should turn those model
properties into deterministic vector schema and refimpl oracle expectations
before qsc runtime or QHSM parameter-block implementation is authorized.

Rejected successors:

- qsc harness requirements and test seam plan: useful, but vector/refimpl
  oracle should freeze fixture semantics first.
- implementation authorization: premature because no vector schema or oracle
  exists yet.
- model blocker resolution: not needed because the executable model succeeded.

## No runtime/protocol/crypto implementation change proof

Changed implementation-model files are limited to `formal/**`. The patch does
not touch qsc runtime source, qsc tests, QSP/refimpl implementation, crypto
state machines, production handshake implementation, key schedule, `Cargo.toml`,
`Cargo.lock`, workflows, qsl-server, qsl-attachments, qsc-desktop, website,
README, START_HERE, docs/public, branch protection, or public-safety
configuration.

## Backup-plan impact statement

No backup-plan update is required. All new artifacts are under the qsl-protocol
worktree in formal, docs/governance/evidence, tests, and governance files
already covered by `/srv/qbuild/work`. No new source root, response root,
excluded backup path, or non-rebuildable artifact outside current backup scope
is introduced.

## Next recommendation

After NA-0309 merges and closes out, execute:

NA-0310 -- qsc Handshake Suite-ID Vector Schema and Refimpl Oracle

The next lane should add vector schema, vector categories, deterministic oracle
reason labels, expected transcript/key-context fields, and oracle validation
without implementing qsc runtime or QHSM/QSP production wire changes.
