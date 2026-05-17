Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-17
Replaces: n/a
Superseded-By: n/a

# NA-0302 Suite-2 Negotiation Vector and qsc Cross-Surface Harness

Directive: QSL-DIR-2026-05-17-110 / NA-0302

## Executive Summary

NA-0302 adds a dedicated Suite-2 negotiation vector pack and executable
test-only proof across two surfaces:

- a refimpl integration test that consumes the NA-0302 negotiation vectors; and
- a qsc integration test that injects unsupported/downgrade/malformed Suite-2
  wire through the existing relay receive path and proves fail-closed behavior
  without persisted Suite-2 session mutation.

The lane changes no protocol, wire-format, crypto state-machine, handshake, key
schedule, dependency, workflow, service implementation, desktop, website,
README, START_HERE, docs/public, branch-protection, or public-safety
configuration.

## Live NA-0302 Scope

Live `NEXT_ACTIONS.md` authorizes executable vector or cross-surface hardening
proof, evidence/testplan, decision, and traceability updates. It explicitly
forbids protocol and crypto semantic changes by default and requires a stop if
a bug fix would require those changes.

Protected boundaries:

- no unsupported production/public-internet/external-review/anonymity claims;
- no silent protocol or crypto semantic changes;
- executable proof or exact prerequisite stop;
- no dependency, workflow, website, service implementation, docs/public,
  README, START_HERE, branch-protection, or public-safety configuration drift.

## Selected Surfaces

Selected vector surface:

- `inputs/suite2/vectors/qshield_suite2_negotiation_vectors_na0302.json`

Selected refimpl proof:

- `tools/refimpl/quantumshield_refimpl/tests/na_0302_suite2_negotiation_vectors.rs`

Selected qsc cross-surface proof:

- `qsl/qsl-client/qsc/tests/na_0302_suite2_negotiation_cross_surface.rs`

Reason:

- the vector pack is schema-valid under the existing Suite-2 vector schema and
  stays in the established `inputs/suite2/vectors/**` surface;
- the refimpl test evaluates dedicated valid and negative negotiation vectors
  without runtime implementation changes;
- the qsc test uses existing public CLI, relay, QSE envelope, and stored
  Suite-2 session behavior to prove analogous reject behavior through the qsc
  receive surface.

## Changed Files

- `inputs/suite2/vectors/qshield_suite2_negotiation_vectors_na0302.json`
- `tools/refimpl/quantumshield_refimpl/tests/na_0302_suite2_negotiation_vectors.rs`
- `qsl/qsl-client/qsc/tests/na_0302_suite2_negotiation_cross_surface.rs`
- `docs/governance/evidence/NA-0302_suite2_negotiation_vector_qsc_cross_surface_harness.md`
- `tests/NA-0302_suite2_negotiation_vector_qsc_cross_surface_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Vector Schema

The NA-0302 vector file uses `QSHIELD-P4-VECTOR-SET-1` and is covered by the
existing `schemas/qshield.phase4.vector_set.schema.v1.json` validator.

Marker:

- `NA0302_VECTOR_SCHEMA_OK`

## Valid Suite-2 Vector Proof

The valid vector selects:

- protocol version `0x0500`;
- suite id `0x0002`;
- `ML-KEM-768`;
- `ML-DSA-65+Ed25519`;
- `KDF_HYBRID_KMAC256_SHA512`;
- zero unsupported flags.

The refimpl harness accepts the vector and mutates accepted negotiation state
only for that valid vector.

Marker:

- `NA0302_VALID_SUITE2_VECTOR_OK`

## Unsupported Suite Vector Proof

The vector `NA0302-S2-NEGOTIATION-UNSUPPORTED-SUITE-0001` selects suite
`0x9999` while Suite-2 is required. The refimpl vector harness rejects it with
`REJECT_S2_SUITE_MISMATCH`, repeats the reject deterministically, and proves
accepted state unchanged.

Marker:

- `NA0302_UNSUPPORTED_SUITE_VECTOR_REJECT_OK`

## Downgrade / Version Vector Proof

The vector `NA0302-S2-NEGOTIATION-DOWNGRADE-VERSION-0001` selects the
Suite-1-family version/suite pair `0x0403` / `0x0001` while both peers support
Suite-2. The refimpl vector harness rejects it with `REJECT_S2_DOWNGRADE`,
repeats the reject deterministically, and proves accepted state unchanged.

Marker:

- `NA0302_DOWNGRADE_VECTOR_REJECT_OK`

## Unsupported Parameter / Flag Vector Proof

The vector pack includes:

- `NA0302-S2-NEGOTIATION-UNSUPPORTED-ALGORITHM-0001`, rejected with
  `REJECT_S2_ALGORITHM_UNSUPPORTED`; and
- `NA0302-S2-NEGOTIATION-UNSUPPORTED-FLAGS-0001`, rejected with
  `REJECT_S2_PARSE_FLAGS`.

The marker is emitted only after the unsupported algorithm and unsupported flag
category is exercised without accepted-state mutation.

Marker:

- `NA0302_UNSUPPORTED_PARAMETER_VECTOR_REJECT_OK`

## Malformed Negotiation Vector Proof

The vector `NA0302-S2-NEGOTIATION-MALFORMED-0001` carries only a malformed
sentinel input. The refimpl vector harness rejects it with
`REJECT_S2_MALFORMED_NEGOTIATION`, repeats the reject deterministically, and
proves the sentinel is not echoed in reject text.

Marker:

- `NA0302_MALFORMED_NEGOTIATION_VECTOR_REJECT_OK`

## Refimpl / Vector Proof

The refimpl harness loads the dedicated NA-0302 vector file, verifies the root
schema identity and protocol tuple, accepts the valid vector, rejects every
negative vector with the expected reason code, repeats each reject, and compares
accepted-state snapshots before and after rejects.

Harness command:

```bash
cargo +stable test -p quantumshield_refimpl --locked --test na_0302_suite2_negotiation_vectors -- --test-threads=1 --nocapture
```

## qsc Cross-Surface Proof

The qsc harness is implemented. It:

1. initializes a test-only qsc vault and peer route;
2. sends one valid Suite-2 message through the existing relay/QSE surface;
3. snapshots the encrypted persisted Suite-2 session state;
4. mutates the queued QSE payload into unsupported suite, downgrade-like
   version/suite, unsupported flags, and malformed payload variants;
5. runs qsc receive through the normal CLI surface for each variant; and
6. proves fail-closed `qsp_verify_failed`, no `recv_commit`, no plaintext file,
   no sentinel leak, no panic/backtrace, and unchanged persisted session state.

Marker:

- `NA0302_QSC_CROSS_SURFACE_OK`

## No-Mutation Proof

Refimpl proof:

- every negative vector is evaluated twice from the same accepted state and the
  accepted negotiation state remains byte-for-byte equivalent by value.

qsc proof:

- the encrypted qsc Suite-2 session blob is decrypted by the test harness before
  and after each rejected receive, and `Suite2SessionState::snapshot_bytes()`
  remains unchanged.

Marker:

- `NA0302_VECTOR_NO_MUTATION_ON_REJECT_OK`

## No Panic / Backtrace Proof

Refimpl proof:

- every vector evaluation is wrapped with `catch_unwind`;
- reject text is scanned for panic/backtrace wording.

qsc proof:

- receive output is scanned for panic/backtrace wording after each adversarial
  queued payload.

Marker:

- `NA0302_NO_PANIC_OK`

## No Secret / Plaintext Leak Proof

Refimpl proof:

- malformed negotiation sentinel text is not echoed in reject output.

qsc proof:

- the plaintext sentinel, malformed sentinel, and route token are not present in
  command output on rejected receive paths.

Marker:

- `NA0302_NO_SECRET_LEAK_OK`

## Commands Run

Preflight:

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
cargo +stable build -p refimpl_actor --locked
cargo +stable test -p quantumshield_refimpl --locked --test na_0300_core_replay_reject_no_mutation -- --test-threads=1 --nocapture
cargo +stable test -p quantumshield_refimpl --locked --test na_0301_suite2_negotiation_downgrade -- --test-threads=1 --nocapture
cargo +stable test -p quantumshield_refimpl --locked -- --test-threads=1
python3 scripts/ci/validate_suite2_vectors.py
python3 scripts/ci/run_suite2_downgrade_vectors.py --actor /srv/qbuild/cache/targets/qsl-protocol/debug/refimpl_actor
scripts/ci/run_suite2_transcript_vectors.py --actor /srv/qbuild/cache/targets/qsl-protocol/debug/refimpl_actor
cargo +stable build -p qshield-cli --locked
cargo +stable test -p qshield-cli --locked -- --test-threads=1
```

NA-0302 focused harness commands:

```bash
python3 scripts/ci/validate_suite2_vectors.py
cargo +stable test -p quantumshield_refimpl --locked --test na_0302_suite2_negotiation_vectors -- --test-threads=1 --nocapture
cargo +stable test -p qsc --locked --test na_0302_suite2_negotiation_cross_surface -- --test-threads=1 --nocapture
```

## Artifacts

- Demo adversarial stress:
  `/srv/qbuild/tmp/NA-0262_demo_adversarial_stress_artifacts_20260517T034934Z`
- Demo repeated soak:
  `/srv/qbuild/tmp/NA-0266_demo_soak_repeated_run_artifacts_20260517T034939Z`
- Sanitized retention harness:
  `/srv/qbuild/tmp/NA-0293_metadata_phase2_sanitized_retention.JXat2A`
- Suite-2 schema report:
  `artifacts/suite2/vector_schema_report.json`

## Limitations

- This is bounded executable harness proof, not full cryptographic proof.
- The dedicated negotiation vector evaluator is a test harness over the current
  expected Suite-2 negotiation tuple and reason-code labels; it does not change
  runtime negotiation semantics.
- The qsc proof covers the existing qsc relay/QSE receive surface and persisted
  session state, not every service, desktop, or production deployment path.
- External review completion, production readiness, public-internet readiness,
  anonymity, metadata-free messaging, and untraceability remain unclaimed.

## No Protocol / Crypto Implementation Change Proof

NA-0302 does not edit:

- `tools/refimpl/quantumshield_refimpl/src/**`;
- `qsp/**`;
- `qsl/qsl-client/qsc/src/**`;
- `Cargo.toml` or `Cargo.lock`;
- `.github/**`;
- qsl-server or qsl-attachments implementation paths;
- qsc-desktop;
- website or external website paths;
- README or START_HERE.

The only qsc path changed is a Rust integration test under
`qsl/qsl-client/qsc/tests/**`.

## Next Recommendation

After this PR merges and post-merge public-safety is green, close out NA-0302
and restore NA-0303 as the next executable hardening lane. The recommended
NA-0303 focus is to extend qsc cross-surface negotiation hardening from the
current receive-path proof into a narrower handshake/activation prerequisite
audit or harness if live scope authorizes it, without changing protocol or
crypto semantics unless a future dedicated fix lane names the exact bug.
