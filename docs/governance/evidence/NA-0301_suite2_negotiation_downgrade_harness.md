Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-16
Replaces: n/a
Superseded-By: n/a

# NA-0301 Suite-2 Negotiation Downgrade Harness

Directive: QSL-DIR-2026-05-16-109 / NA-0301

## Executive Summary

NA-0301 adds a bounded executable refimpl integration harness for Suite-2
negotiation and downgrade-like reject behavior. The harness preserves protocol
and crypto semantics: it changes no implementation source, wire-format code,
key schedule code, handshake code, dependency file, workflow, service, desktop,
website, README, START_HERE, or public-safety configuration.

The test proves a valid Suite-2 negotiation/control path accepts and mutates
accepted state, then proves unsupported suite, downgrade-like version,
unsupported version, unsupported algorithm fixture, unsupported flag/parameter,
and malformed negotiation/wire inputs reject deterministically without mutating
accepted state. Reject attempts are panic-guarded and checked for sentinel leak
absence. The harness also cross-checks existing Suite-2 downgrade and transcript
vectors.

## Live NA-0301 Scope

Live `NEXT_ACTIONS.md` authorizes executable negotiation/downgrade/refimpl
harnesses under `tools/refimpl/**`, deterministic fixture inputs under
`inputs/**` if needed, focused qsc tests only if explicitly needed, and evidence
updates in governance/testplan/decision/traceability/journal files.

Protected boundaries:

- no protocol, wire, key-schedule, handshake, downgrade, replay, or crypto-state
  semantic drift;
- no dependency changes;
- no service, website, qsc-desktop, README, START_HERE, workflow, branch
  protection, or public-safety configuration changes;
- no unsupported production, public-internet, external-review, anonymity,
  metadata-free, or untraceable claims;
- executable tests or exact fail-closed stop.

## Selected Surface

Selected surface:

- `tools/refimpl/quantumshield_refimpl/tests/na_0301_suite2_negotiation_downgrade.rs`

Reason:

- It can reuse the direct Suite-2 wire receive surface proven by NA-0300.
- It can compare `Suite2SessionState::snapshot_bytes()` before and after
  rejects.
- It can evaluate existing Suite-2 downgrade/transcript vectors without adding
  dependencies or implementation code.
- It keeps qsl-server, qsl-attachments, qsc-desktop, website, workflow, Cargo,
  and runtime protocol-source paths untouched.

## Changed Files

- `tools/refimpl/quantumshield_refimpl/tests/na_0301_suite2_negotiation_downgrade.rs`
- `docs/governance/evidence/NA-0301_suite2_negotiation_downgrade_harness.md`
- `tests/NA-0301_suite2_negotiation_downgrade_harness_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Harness Design

The harness has three bounded proof layers:

1. A local negotiation fixture evaluator for Suite-2 required policy,
   transcript/AD agreement, and algorithm selection.
2. A refimpl Suite-2 wire receive path using `send_wire_canon` and
   `recv_wire_canon` with transaction-style state commit only on success.
3. Existing vector consistency checks for:
   - `inputs/suite2/vectors/qshield_suite2_downgrade_vectors_v1.json`
   - `inputs/suite2/vectors/qshield_suite2_transcript_vectors_v1.json`

For every reject, the harness asserts:

- exact reason code;
- deterministic repeated rejection;
- no accepted negotiation/session state mutation;
- no panic/backtrace;
- no plaintext or negotiation sentinel leak in reject text.

## Valid Suite-2 Control Proof

The valid local negotiation fixture accepts Suite-2
`protocol_version=0x0500`, `suite_id=0x0002`, and canonical local algorithm
names. The valid wire control path sends and receives one Suite-2 message using
the existing refimpl wire functions. Both accepted paths change their accepted
state snapshots.

Marker:

- `NA0301_SUITE2_CONTROL_OK`

## Unsupported Suite Reject Proof

The local negotiation fixture rejects unsupported suite `0x9999` with
`REJECT_S2_SUITE_MISMATCH`. The wire fixture mutates the Suite-2 envelope suite
field and rejects at the parse prefix boundary without accepted-state mutation.

Marker:

- `NA0301_UNSUPPORTED_SUITE_REJECT_OK`

## Downgrade / Version Reject Proof

The local negotiation fixture rejects a Suite-1-like fallback
`protocol_version=0x0403`, `suite_id=0x0001` with `REJECT_S2_DOWNGRADE`. It also
rejects an unsupported future version fixture with
`REJECT_S2_VERSION_UNSUPPORTED`. The wire fixture mutates the envelope version
and suite fields and rejects without accepted-state mutation.

Marker:

- `NA0301_DOWNGRADE_REJECT_OK`

## Unsupported Parameter / Flag Reject Proof

The harness mutates a valid wire header to set an unsupported high flag bit and
asserts deterministic `REJECT_S2_PARSE_FLAGS` without accepted-state mutation.
The local negotiation fixture also rejects an unsupported KEM algorithm label
with `REJECT_S2_ALGORITHM_UNSUPPORTED`. That algorithm check is fixture-based
because the current wire vectors do not carry a standalone algorithm identifier.

Marker:

- `NA0301_UNSUPPORTED_PARAMETER_REJECT_OK`

## Malformed Negotiation Reject Proof

The harness feeds a malformed sentinel byte string as wire input. It rejects
with `REJECT_S2_PARSE_PREFIX`, remains deterministic on retry, does not mutate
accepted state, and does not echo the sentinel in reject text.

Marker:

- `NA0301_MALFORMED_NEGOTIATION_REJECT_OK`

## No-Mutation Proof

The harness stores the accepted negotiation state and accepted
`Suite2SessionState::snapshot_bytes()` after the valid control path. After all
reject cases, it verifies both accepted snapshots are unchanged.

Marker:

- `NA0301_NO_MUTATION_ON_REJECT_OK`

## No Panic / Backtrace Proof

Each adversarial wire receive is wrapped in `catch_unwind`; any panic fails the
test. Reject text is also scanned for panic/backtrace wording.

Marker:

- `NA0301_NO_PANIC_OK`

## No Secret / Plaintext Leak Proof

The valid wire plaintext contains `NA0301_PLAINTEXT_SENTINEL_DO_NOT_ECHO`; the
local negotiation fixture carries `NA0301_NEGOTIATION_SECRET_SENTINEL_DO_NOT_ECHO`.
Reject text is checked so neither sentinel appears.

Marker:

- `NA0301_NO_SECRET_LEAK_OK`

## Refimpl / Vector Consistency Proof

The harness reads and evaluates existing vector files:

- downgrade vectors: `CAT-S2-DOWNGRADE-001`, 5 vectors;
- transcript vectors: `CAT-S2-TRANSCRIPT-001`, 4 vectors.

The downgrade evaluator mirrors the current actor/vector policy for required
Suite-2 selection, downgrade rejection, suite mismatch rejection, peer
unsupported rejection, and AD mismatch rejection. The transcript evaluator uses
the refimpl `binding` helpers to recompute `pq_bind`, `ad_hdr`, and `ad_body`.

Marker:

- `NA0301_VECTOR_CONSISTENCY_OK`

## Commands Run

Pre-implementation and implementation validation included:

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
cargo +stable test -p quantumshield_refimpl --locked --test na_0300_core_replay_reject_no_mutation -- --test-threads=1 --nocapture
cargo +stable build -p refimpl_actor --locked
cargo +stable test -p quantumshield_refimpl --locked -- --test-threads=1
python3 scripts/ci/run_suite2_downgrade_vectors.py --actor /srv/qbuild/cache/targets/qsl-protocol/debug/refimpl_actor
scripts/ci/run_suite2_transcript_vectors.py --actor /srv/qbuild/cache/targets/qsl-protocol/debug/refimpl_actor
python3 scripts/ci/validate_suite2_vectors.py
cargo +stable test -p quantumshield_refimpl --locked --test na_0301_suite2_negotiation_downgrade -- --test-threads=1 --nocapture
```

## Artifacts

- Demo adversarial stress artifacts:
  `/srv/qbuild/tmp/NA-0262_demo_adversarial_stress_artifacts_20260517T003212Z`
- Sanitized retention harness artifacts:
  `/srv/qbuild/tmp/NA-0293_metadata_phase2_sanitized_retention.thvKIm`
- Vector reports are generated under ignored `artifacts/suite2/**` by the
  existing vector helper scripts.

## Limitations

- This is bounded executable harness proof, not full cryptographic proof.
- The local unsupported-algorithm check is fixture-based because current
  Suite-2 negotiation vectors do not expose a standalone algorithm identifier.
- The harness covers the selected refimpl and vector surfaces, not every qsc,
  service, desktop, or production deployment path.
- External review completion, production readiness, public-internet readiness,
  anonymity, metadata-free messaging, and untraceability remain unclaimed.

## No Protocol / Crypto Implementation Change Proof

NA-0301 changes one refimpl integration test plus governance evidence. It does
not edit:

- `tools/refimpl/quantumshield_refimpl/src/**`;
- `qsp/**`;
- `qsl/qsl-client/qsc/src/**`;
- `Cargo.toml` or `Cargo.lock`;
- `.github/**`;
- qsl-server or qsl-attachments implementation paths;
- qsc-desktop;
- website or external website paths;
- README or START_HERE.

## Next Recommendation

After this PR merges and post-merge public-safety is green, close out NA-0301
and restore NA-0302 as the next executable hardening lane. NA-0302 should keep
protocol/crypto implementation changes forbidden unless a future directive
names a concrete bug and authorizes a narrow fix lane.
