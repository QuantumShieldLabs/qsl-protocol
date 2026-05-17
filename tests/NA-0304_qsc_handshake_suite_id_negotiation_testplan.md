Status: Supporting
Owner: qsl-protocol maintainers
Last-Updated: 2026-05-17
Replaces: n/a
Superseded-By: n/a

# NA-0304 qsc Handshake Suite-ID Negotiation Testplan

## Objective

Add or use an authorized test-only qsc handshake suite-id seam if one exists.
If no such seam exists, record executable blocker evidence without changing
runtime, protocol, wire, crypto state-machine, key schedule, or dependency
behavior.

## Protected invariants

- No silent protocol semantic change.
- No silent crypto state-machine change.
- No production handshake implementation change.
- No QSP wire-format or key schedule change.
- No dependency, workflow, service, website, docs/public, README, START_HERE,
  branch-protection, or public-safety configuration drift.
- Unsupported, downgrade-like, and malformed suite-id admission gaps must remain
  visible if no qsc handshake input seam exists.
- Rejected inputs must not be normalized as success.
- No panic/backtrace or secret/plaintext/sentinel leakage is accepted.

## Allowed scope

- `qsl/qsl-client/qsc/tests/na_0304_handshake_suite_id_negotiation.rs`
- `docs/governance/evidence/NA-0304_qsc_handshake_suite_id_negotiation_harness.md`
- `tests/NA-0304_qsc_handshake_suite_id_negotiation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

- qsc runtime source under `qsl/qsl-client/qsc/src/**`
- QSP/refimpl/protocol-core implementation source
- crypto state-machine implementation
- production handshake implementation
- key schedule implementation
- QSP wire-format implementation
- `Cargo.toml` or `Cargo.lock`
- `.github/**`
- `scripts/**`
- `qsl-server/**`
- `qsl-attachments/**`
- `qsc-desktop/**`
- website or external website paths
- README, START_HERE, docs/public
- branch-protection or public-safety configuration

## Suite-id seam requirements

The harness must inspect the current qsc handshake surface and classify whether
an explicit suite-id field is available in qsc `QHSM` handshake admission. If
the field is absent, the harness must emit blocker markers rather than
pretending unsupported/downgrade/malformed suite-id admission was tested.

## Valid suite-id admission requirements

If feasible, a valid qsc handshake must commit Suite-2 session state. The
NA-0304 blocker harness must at least prove the accepted session state carries
protocol version `0x0500` and suite id `0x0002` after the valid qsc handshake.

## Unsupported suite-id requirements

If an explicit qsc handshake suite-id input exists, unsupported suite-id input
must reject deterministically with no accepted-state mutation and no
`recv_commit`/output artifact. If no input seam exists, blocker evidence must
state that exact reason.

## Downgrade suite-id requirements

If an explicit qsc handshake suite-id input exists, downgrade-like suite-id
input must reject deterministically with no accepted-state mutation and no
`recv_commit`/output artifact. If no input seam exists, blocker evidence must
state that exact reason.

## Malformed suite-id requirements

If an explicit qsc handshake suite-id input exists, malformed suite-id input
must reject deterministically, must not echo sentinels, and must not mutate
accepted session state. If no input seam exists, blocker evidence must state
that exact reason.

## No-mutation requirements

Rejected qsc handshake suite-id admission inputs must not create qsp session
state or mutate accepted qsc session state. If such inputs cannot be expressed,
the blocker evidence must preserve the gap.

## No recv_commit/output requirements

Rejected qsc handshake suite-id admission inputs must not produce
`event=recv_commit`, `event=qsp_unpack ok=true`, plaintext files, or qsp output
artifacts. If such inputs cannot be expressed, the blocker evidence must
preserve the gap.

## No panic/leak requirements

The harness must fail if panic/backtrace wording, route tokens, passphrase-env
markers, plaintext payloads, or sentinel strings appear in output.

## Blocked-seam handling

Required blocker markers:

- `NA0304_QSC_SUITE_ID_SEAM_BLOCKED`
- `NA0304_NO_IMPLEMENTATION_CHANGE_OK`
- `NA0304_BLOCKER_EVIDENCE_OK`

Additional accepted blocker-context markers:

- `NA0304_QSC_HANDSHAKE_SESSION_SUITE2_STATE_OK`
- `NA0304_QSC_QHSM_NO_EXPLICIT_SUITE_ID_FIELD_OK`

## Required local checks

- `cargo +stable fmt --check`
- `cargo +stable test -p qsc --locked --test na_0304_handshake_suite_id_negotiation -- --test-threads=1 --nocapture`
- `cargo +stable test -p qsc --locked --test na_0303_handshake_activation_negotiation -- --test-threads=1 --nocapture`
- `cargo +stable test -p qsc --locked --test na_0302_suite2_negotiation_cross_surface -- --test-threads=1 --nocapture`
- `cargo +stable test -p quantumshield_refimpl --locked --test na_0302_suite2_negotiation_vectors -- --test-threads=1 --nocapture`
- `cargo +stable test -p quantumshield_refimpl --locked --test na_0301_suite2_negotiation_downgrade -- --test-threads=1 --nocapture`
- `cargo +stable test -p quantumshield_refimpl --locked --test na_0300_core_replay_reject_no_mutation -- --test-threads=1 --nocapture`
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/run_model_checks.py`
- metadata/demo smoke and baseline stress checks when feasible
- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --allowed qsl/qsl-client/qsc/tests/na_0304_handshake_suite_id_negotiation.rs docs/governance/evidence/NA-0304_qsc_handshake_suite_id_negotiation_harness.md tests/NA-0304_qsc_handshake_suite_id_negotiation_testplan.md DECISIONS.md TRACEABILITY.md docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- goal-lint using a PR body containing `Goals: G1, G2, G3, G4, G5`

## CI expectations

Required CI and public-safety must complete normally. Because the patch adds a
qsc integration test, qsc-related suites may run and must pass or be accepted
only when they report explicit intentional skip behavior.

## Successor handoff

If NA-0304 records the missing qsc handshake suite-id seam, the recommended
successor is NA-0305: a narrow planning lane for qsc handshake suite-id seam
authorization. It must not implement NA-0305 during NA-0304 closeout.
