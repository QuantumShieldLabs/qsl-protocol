Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-17
Replaces: n/a
Superseded-By: n/a

# NA-0304 qsc Handshake Suite-ID Negotiation Harness

Directive: QSL-DIR-2026-05-17-112 / NA-0304

## Executive summary

NA-0304 adds a test-only qsc harness that records the current suite-id
handshake boundary without changing runtime, protocol, wire, crypto
state-machine, key schedule, or dependency behavior.

The harness proves:

- a valid qsc `QHSM` handshake still commits persisted Suite-2 session state
  with protocol version `0x0500` and suite id `0x0002`;
- `QHSM` A1, B1, and A2 frames expose magic, frame version, type, session id,
  and cryptographic material, but no explicit suite-id negotiation field;
- unsupported, downgrade-like, and malformed suite-id admission inputs cannot
  be expressed at the qsc handshake frame surface without adding a protocol or
  runtime seam; and
- no `recv_commit`, panic/backtrace text, route token, passphrase-env marker,
  or plaintext/sentinel content is emitted by the blocker harness.

This is bounded executable blocker evidence. It is not external review, not a
production/public-internet readiness statement, and not a claim that qsc
handshake suite-id admission is fully proven.

## Live NA-0304 scope

Live `NEXT_ACTIONS.md` authorizes qsc handshake suite-id negotiation visibility
through existing public/test APIs or an authorized test-only seam, and requires
exact prerequisite blocker evidence if no such seam exists.

Protected boundaries:

- no silent protocol or crypto semantic changes;
- no production handshake implementation change;
- no QSP wire-format or key schedule change;
- no dependency, workflow, service, website, docs/public, README, START_HERE,
  branch-protection, or public-safety configuration drift; and
- exactly one READY item remains NA-0304 until a separate closeout.

## Selected surface or blocker

Selected surface:

- `qsl/qsl-client/qsc/tests/na_0304_handshake_suite_id_negotiation.rs`

The harness uses only existing qsc CLI commands, the existing qsc in-process
relay test helper, and test-local session-state inspection:

- `handshake init`;
- `handshake poll`;
- identity/contact/relay setup commands;
- raw relay queue inspection for `QHSM` frames; and
- test-local qsc session blob decryption to confirm the accepted Suite-2 tuple.

Selected blocker:

- qsc `QHSM` handshake frames do not carry an explicit suite-id field.
- qsc `hs_build_session` passes `SUITE2_PROTOCOL_VERSION` and
  `SUITE2_SUITE_ID` constants into `init_from_base_handshake`.
- Therefore the current qsc handshake surface proves Suite-2 session
  commitment after successful activation, but it cannot accept injected
  unsupported, downgrade-like, or malformed suite-id negotiation values at
  handshake admission without an implementation seam.

## Changed files

- `qsl/qsl-client/qsc/tests/na_0304_handshake_suite_id_negotiation.rs`
- `docs/governance/evidence/NA-0304_qsc_handshake_suite_id_negotiation_harness.md`
- `tests/NA-0304_qsc_handshake_suite_id_negotiation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Suite-id seam discovery

Discovery found these relevant surfaces:

- qsc `QHSM` frame codec in `qsl/qsl-client/qsc/src/handshake/mod.rs`;
- qsc CLI/relay handshake harness from NA-0303;
- qsc receive-path Suite-2 negotiation harness from NA-0302;
- refimpl negotiation/vector harnesses from NA-0301 and NA-0302; and
- Suite-2 constants in `tools/refimpl/quantumshield_refimpl/src/suite2/types.rs`.

The explicit suite-id field exists in refimpl Suite-2 vectors, QSP message
types, Suite-2 wire receive payloads, and persisted Suite-2 session state. It
does not exist as an input field in qsc `QHSM` handshake A1/B1/A2 frames.

## Valid suite-id admission proof if feasible

Partial qsc proof is feasible and covered:

1. Alice emits A1.
2. Bob consumes A1 and emits B1 without committing an accepted qsp session.
3. Alice consumes B1, stores an initiator qsp session, and emits A2.
4. Bob consumes A2 and stores a responder qsp session.
5. The harness decrypts both qsc session blobs and asserts both send and recv
   states carry protocol version `0x0500` and suite id `0x0002`.

Marker:

- `NA0304_QSC_HANDSHAKE_SESSION_SUITE2_STATE_OK`

Limitation:

- This proves accepted session-state tuple after qsc handshake activation. It
  does not prove explicit suite-id negotiation at qsc handshake admission,
  because no suite-id input field exists in the frame.

## Unsupported suite-id reject proof if feasible

Blocked for qsc handshake admission.

Reason:

- The `QHSM` A1/B1/A2 frame layout has no explicit suite-id field to mutate.
- Mutating frame version or type is already covered by NA-0303, but that is not
  an explicit suite-id admission input.
- Mutating Suite-2 receive wire is already covered by NA-0302, but that is the
  post-handshake receive surface, not qsc handshake admission.

## Downgrade suite-id reject proof if feasible

Blocked for qsc handshake admission for the same reason: no handshake suite-id
input exists to set to a Suite-1-like or lower-suite value. NA-0301/NA-0302
remain the bounded refimpl/vector and qsc receive-path downgrade evidence.

## Malformed suite-id reject proof if feasible

Blocked for qsc handshake admission because malformed suite-id content has no
dedicated handshake field. NA-0303 still covers malformed `QHSM` admission
bytes, and NA-0302 covers malformed Suite-2 receive-path payloads.

## No-mutation proof or blocker

The blocker harness confirms the successful valid handshake follows the same
state transition ordering as NA-0303:

- Bob has no qsp session after B1 emission.
- Alice has a qsp session only after consuming B1.
- Bob has a qsp session only after consuming A2.

Rejected explicit suite-id admission no-mutation proof is blocked because no
explicit suite-id reject input can be expressed at the qsc handshake frame
surface.

## No recv_commit/output proof or blocker

The NA-0304 harness asserts that the qsc handshake blocker path does not emit
`event=recv_commit`. Rejected explicit suite-id output proof is blocked by the
missing input seam.

## No panic/backtrace proof

The harness scans combined command output for panic/backtrace markers and fails
if found.

Marker:

- `NA0304_BLOCKER_EVIDENCE_OK`

## No secret/plaintext/sentinel leak proof

The harness scans combined command output for both route tokens and the
passphrase-env marker and fails if any are emitted. No plaintext payload is
created by this handshake-only harness.

## Blocker evidence if seam absent

The executable blocker markers are:

- `NA0304_QSC_QHSM_NO_EXPLICIT_SUITE_ID_FIELD_OK`
- `NA0304_QSC_SUITE_ID_SEAM_BLOCKED`
- `NA0304_NO_IMPLEMENTATION_CHANGE_OK`
- `NA0304_BLOCKER_EVIDENCE_OK`

Source evidence:

- `hs_encode_init` writes `QHSM`, `HS_VERSION`, `HS_TYPE_INIT`, session id,
  KEM public key, signature public key, and DH public key.
- `hs_encode_resp` writes `QHSM`, `HS_VERSION`, `HS_TYPE_RESP`, session id,
  KEM ciphertext, transcript MAC, signature public key, signature, and DH
  public key.
- `hs_encode_confirm` writes `QHSM`, `HS_VERSION`, `HS_TYPE_CONFIRM`, session
  id, confirm MAC, and signature.
- `hs_build_session` supplies the Suite-2 tuple internally from constants.

## Commands run

Preflight and focused harness:

```bash
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/run_model_checks.py
scripts/ci/metadata_conformance_smoke.sh
scripts/ci/metadata_phase2_identifier_padding_harness.sh
scripts/ci/metadata_phase2_sanitized_errors_retention_harness.sh
scripts/ci/demo_cli_smoke.sh
DEMO_STRESS_PROFILE=baseline scripts/ci/demo_adversarial_stress.sh
DEMO_SOAK_RUNS=3 scripts/ci/demo_soak_repeated_run.sh
cargo +stable test -p quantumshield_refimpl --locked --test na_0300_core_replay_reject_no_mutation -- --test-threads=1 --nocapture
cargo +stable test -p quantumshield_refimpl --locked --test na_0301_suite2_negotiation_downgrade -- --test-threads=1 --nocapture
cargo +stable test -p quantumshield_refimpl --locked --test na_0302_suite2_negotiation_vectors -- --test-threads=1 --nocapture
cargo +stable test -p qsc --locked --test na_0302_suite2_negotiation_cross_surface -- --test-threads=1 --nocapture
cargo +stable test -p qsc --locked --test na_0303_handshake_activation_negotiation -- --test-threads=1 --nocapture
cargo +stable test -p qsc --locked --test qsp_protocol_gate -- --test-threads=1
cargo +stable test -p quantumshield_refimpl --locked -- --test-threads=1
cargo +stable build -p qshield-cli --locked
cargo +stable test -p qshield-cli --locked -- --test-threads=1
cargo +stable fmt --check
cargo +stable test -p qsc --locked --test na_0304_handshake_suite_id_negotiation -- --test-threads=1 --nocapture
```

## Artifacts

- Demo adversarial stress:
  `/srv/qbuild/tmp/NA-0262_demo_adversarial_stress_artifacts_20260517T143134Z`
- Demo repeated soak:
  `/srv/qbuild/tmp/NA-0266_demo_soak_repeated_run_artifacts_20260517T143138Z`
- Sanitized retention harness:
  `/srv/qbuild/tmp/NA-0293_metadata_phase2_sanitized_retention.q5rBft`

## Limitations

- qsc `QHSM` handshake frames do not expose an explicit Suite-2 suite-id
  negotiation field.
- The NA-0304 qsc harness proves valid Suite-2 session-state commitment and the
  missing explicit field. It does not prove unsupported/downgrade/malformed
  suite-id admission at qsc handshake admission.
- Refimpl/vector and qsc receive-path suite-id reject evidence remains covered
  by NA-0301 and NA-0302.
- This is not external review and not broad readiness evidence.

## No protocol/crypto implementation change proof

The implementation patch is test/governance only. It does not change qsc
runtime source under `qsl/qsl-client/qsc/src/**`, QSP/refimpl source, crypto
state machines, key schedule code, production handshake implementation, QSP
wire-format implementation, Cargo manifests, workflows, qsl-server,
qsl-attachments, qsc-desktop, website/external website, README, START_HERE, or
docs/public.

## Next recommendation

Select NA-0305 as a narrow qsc handshake suite-id seam planning lane. It should
decide whether adding an explicit suite-id/capability field to the qsc
handshake admission path is authorized, and it must keep protocol/wire/crypto
semantic changes out of scope unless a future directive explicitly authorizes
that implementation work with tests/vectors.
