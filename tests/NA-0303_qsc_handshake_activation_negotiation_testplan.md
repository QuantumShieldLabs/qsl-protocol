# NA-0303 qsc Handshake Activation Negotiation Testplan

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-17

Goals: G1, G2, G3, G4, G5

## Objective

Add bounded executable qsc proof for handshake activation/admission behavior
using existing test seams, without changing protocol, wire, crypto, handshake,
key schedule, dependency, workflow, service, desktop, website, or public docs
semantics.

## Protected invariants

- Valid qsc handshake activation creates accepted qsp session state only at the
  expected stages.
- Unsupported or downgrade-like handshake/admission input fails closed.
- Malformed handshake/admission input fails closed.
- Inactive or unauthorized activation/admission fails closed.
- Duplicate/replayed pending-stage admission input fails closed or is a
  deterministic no-op under current semantics.
- Rejected admission inputs do not create or mutate accepted qsp session state.
- Rejected admission inputs do not emit `recv_commit`, qsp output, or plaintext
  artifacts.
- Reject paths do not panic and do not print backtraces.
- Reject paths do not leak route tokens, passphrase-env names, or malformed
  sentinel strings.

## Allowed scope

- `qsl/qsl-client/qsc/tests/na_0303_handshake_activation_negotiation.rs`
- `docs/governance/evidence/NA-0303_qsc_handshake_activation_negotiation_harness.md`
- `tests/NA-0303_qsc_handshake_activation_negotiation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

- qsc runtime implementation under `qsl/qsl-client/qsc/src/**`
- qsp protocol-core implementation paths
- crypto state-machine implementation paths
- handshake/key schedule implementation paths
- QSP wire-format implementation paths
- `Cargo.toml`
- `Cargo.lock`
- `.github/**`
- qsl-server implementation
- qsl-attachments implementation
- qsc-desktop
- website or external website sources
- `README.md`
- `START_HERE.md`
- `docs/public/**`

## qsc activation/admission requirements

The harness must run qsc `handshake init` and `handshake poll` through the
existing relay test server and prove:

- A1 is emitted by the initiator;
- B1 is emitted by the responder without accepted session commit;
- A2 is emitted by the initiator after B1 admission;
- responder session commit happens only after A2 admission; and
- expected `handshake_complete` markers appear.

## Unsupported suite/version requirements

The qsc handshake frame has no explicit suite-id field. The harness must record
that limitation and must still test the supported seam by mutating the
handshake-frame version to an unsupported value. The reject must create no
session state and no outbound response.

## Downgrade requirements

The harness must mutate the handshake-frame version to a lower
downgrade-like value and prove fail-closed rejection with no accepted-state
mutation and no output commit.

## Malformed input requirements

The harness must inject a malformed sentinel in place of a `QHSM` frame and
prove reject, no session, no response, no commit, and no sentinel echo.

## Inactive/unauthorized requirements

The harness must attempt handshake activation without authenticated identity
pins and prove fail-closed reject, no queued A1, and no session state.

## Replay/duplicate requirements

The harness must replay a previously accepted A1 while responder pending state
is waiting for A2. The replay must reject or be a deterministic no-op without
creating accepted qsp session state, without mutating the pending vault blob,
and without emitting a second B1.

## No-mutation requirements

Rejected inputs must leave qsp session blobs absent or byte-identical, and the
duplicate pending-stage replay must preserve the encrypted vault blob.

## No recv_commit/output requirements

Rejected admission outputs must not contain:

- `event=recv_commit`;
- `event=qsp_unpack ok=true`; or
- `event=handshake_complete`.

## No panic/leak requirements

The harness must scan command output for panic/backtrace text, route tokens,
passphrase-env names, and malformed sentinels.

## Blocked-seam handling if applicable

If a future directive needs suite-id negotiation proof directly inside qsc
handshake activation/admission, it must first add an authorized test-only seam
or select a surface that carries explicit suite-id negotiation data. NA-0303
records that qsc `QHSM` frames currently expose frame version/type but no
suite-id field.

## Required local checks

```bash
cargo +stable fmt --check
cargo +stable test -p qsc --locked --test na_0303_handshake_activation_negotiation -- --test-threads=1 --nocapture
cargo +stable test -p qsc --locked --test na_0302_suite2_negotiation_cross_surface -- --test-threads=1 --nocapture
cargo +stable test -p quantumshield_refimpl --locked --test na_0302_suite2_negotiation_vectors -- --test-threads=1 --nocapture
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/run_model_checks.py
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --allowed qsl/qsl-client/qsc/tests/na_0303_handshake_activation_negotiation.rs --allowed docs/governance/evidence/NA-0303_qsc_handshake_activation_negotiation_harness.md --allowed tests/NA-0303_qsc_handshake_activation_negotiation_testplan.md --allowed DECISIONS.md --allowed TRACEABILITY.md --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
```

## CI expectations

- Required PR checks must attach and complete successfully.
- `public-safety` remains required and green.
- Because the changed path set includes a qsc integration test, qsc-relevant
  suites may run and must not be bypassed.

## Successor handoff

NA-0303 remains READY until its harness PR merges and post-merge public-safety
is green. The recommended successor is NA-0304 as a narrow qsc handshake
negotiation seam or next test-only proof lane; NA-0304 is not implemented by
this testplan.
