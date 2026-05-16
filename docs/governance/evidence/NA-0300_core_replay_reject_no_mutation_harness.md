Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-16
Replaces: n/a
Superseded-By: n/a

# NA-0300 Core Replay Reject No-Mutation Harness

Directive: QSL-DIR-2026-05-16-108 / NA-0300

## Executive Summary

NA-0300 adds a focused executable refimpl harness for Suite-2 wire receive
reject behavior. The harness proves a valid control receive still mutates
accepted state, then proves replay, malformed input, downgrade-like version
input, and unsupported flags reject deterministically without mutating the
accepted session snapshot.

The dependency-health prerequisite packets were skipped because refreshed live
`origin/main` already had `rustls-webpki v0.103.13` and `cargo audit --deny
warnings` passed. No NA-0300A queue insertion, Cargo change, or dependency
remediation PR was needed.

## Live NA-0300 Scope

Live `NEXT_ACTIONS.md` authorizes NA-0300 as the Core Protocol Replay / Reject
/ No-Mutation Adversarial Harness with these boundaries:

- no wire or behavior change by default;
- no crypto state-machine change by default;
- executable harness/tests are preferred over planning-only evidence;
- `tools/refimpl/**` may be used for executable adversarial harnesses if the
  directive explicitly authorizes it;
- qsc tests may be used only if explicitly authorized;
- evidence, testplan, `DECISIONS.md`, `TRACEABILITY.md`, and the rolling
  journal may be updated.

## Selected Surface

Selected surface:

- `tools/refimpl/quantumshield_refimpl/tests/na_0300_core_replay_reject_no_mutation.rs`

Reason:

- it exercises the protocol-core Suite-2 wire receive boundary directly;
- it can compare `Suite2SessionState::snapshot_bytes()` before and after reject
  attempts;
- it uses public refimpl APIs without changing implementation code;
- it can emit directive-required markers under `--nocapture`;
- it avoids qsl-server, qsl-attachments, qsc-desktop, website, workflow,
  dependency, and runtime protocol-source changes.

## Changed Files

- `tools/refimpl/quantumshield_refimpl/tests/na_0300_core_replay_reject_no_mutation.rs`
- `docs/governance/evidence/NA-0300_core_replay_reject_no_mutation_harness.md`
- `tests/NA-0300_core_replay_reject_no_mutation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Harness Design

The harness builds a deterministic Suite-2 session with matching send/receive
keys, sends one valid wire message, and receives it through
`recv_wire_canon`. That establishes the positive control: accepted input
advances the session snapshot.

After the control receive, the harness repeatedly applies adversarial input to
the same accepted state through a transaction-style wrapper that only commits
returned receive state on success. For each rejected input, the test asserts:

- the exact reject code is stable;
- two repeated attempts return identical `RefimplError` values;
- the accepted session snapshot is unchanged after each reject;
- the error rendering does not include the plaintext sentinel;
- the attempt does not panic.

## Replay / Duplicate Proof

The valid wire message is replayed after it has already been accepted. The
replay rejects with `REJECT_S2_REPLAY`, the accepted snapshot remains unchanged,
and repeated replay attempts are deterministic.

Marker:

- `NA0300_REPLAY_REJECT_OK`

## Malformed / Invalid Proof

The harness feeds a malformed byte string containing the plaintext sentinel as
wire input. It rejects at the Suite-2 parse prefix boundary with
`REJECT_S2_PARSE_PREFIX`, does not mutate accepted state, and does not echo the
sentinel in the error surface.

Marker:

- `NA0300_MALFORMED_REJECT_OK`

## Unsupported / Downgrade Proof

The harness mutates a valid Suite-2 wire envelope to use a downgrade-like
protocol version. It rejects with `REJECT_S2_PARSE_PREFIX` without mutating the
accepted state.

The harness also mutates a valid wire header to set an unsupported high flag
bit. It rejects with `REJECT_S2_PARSE_FLAGS` without mutating the accepted
state.

## No-Mutation Proof

For every reject case, the harness compares the pre-reject
`Suite2SessionState::snapshot_bytes()` value to the post-reject snapshot after
the first and second attempt.

Marker:

- `NA0300_NO_MUTATION_ON_REJECT_OK`

## No Panic / Backtrace Proof

Each adversarial receive attempt is wrapped in `catch_unwind`. A panic fails the
test. Error text is also checked for panic/backtrace wording.

Marker:

- `NA0300_NO_PANIC_OK`

## No Secret / Plaintext Leak Proof

The control message plaintext uses the sentinel
`NA0300_PLAINTEXT_SENTINEL_DO_NOT_ECHO`. Reject error text for replay,
malformed, downgrade-like, and unsupported-flag input is checked so the
sentinel is not echoed.

Marker:

- `NA0300_NO_SECRET_LEAK_OK`

## Commands Run

```bash
cargo +stable test -p quantumshield_refimpl --locked --test na_0300_core_replay_reject_no_mutation -- --test-threads=1 --nocapture
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
cargo +stable build -p qshield-cli --locked
cargo +stable test -p qshield-cli --locked -- --test-threads=1
cargo +stable test -p quantumshield_refimpl --locked -- --test-threads=1
cargo fmt --check
```

## Artifacts

- Demo adversarial stress artifacts:
  `/srv/qbuild/tmp/NA-0262_demo_adversarial_stress_artifacts_20260516T213513Z`
- Demo soak artifacts:
  `/srv/qbuild/tmp/NA-0266_demo_soak_repeated_run_artifacts_20260516T213520Z`
- Sanitized retention harness artifacts:
  `/srv/qbuild/tmp/NA-0293_metadata_phase2_sanitized_retention.YbqY99`

## Limitations

- This is bounded executable harness proof, not a complete cryptographic proof.
- It covers the selected Suite-2 wire receive refimpl surface, not every qsc or
  service path.
- It does not claim external review completion.
- It does not establish production readiness or public-internet readiness.
- It does not establish anonymity, metadata-free messaging, or untraceability.

## No Protocol / Crypto Implementation Change Proof

NA-0300 changes only one refimpl integration test plus governance evidence. It
does not edit:

- `tools/refimpl/quantumshield_refimpl/src/**`;
- `qsp/**`;
- `qsc/**` implementation source;
- `qsl/**` implementation source;
- `Cargo.toml` or `Cargo.lock`;
- `.github/**`;
- qsl-server or qsl-attachments implementation paths;
- qsc-desktop;
- website or external website paths.

## Next Recommendation

After the NA-0300 PR merges and post-merge public-safety is green, close out
NA-0300 and restore the next executable hardening lane, NA-0301. The likely
successor should extend the same proof boundary toward a broader Suite-2
negotiation/downgrade expansion harness or refimpl/vector differential
consistency harness without changing protocol semantics unless a dedicated
future fix lane authorizes it.
