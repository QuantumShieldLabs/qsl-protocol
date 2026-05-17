Status: Supporting
Owner: qsl-protocol maintainers
Last-Updated: 2026-05-17
Replaces: n/a
Superseded-By: n/a

Goals: G1, G2, G3, G4, G5

# NA-0307 qsc Handshake Suite-ID Compatibility and Transcript Testplan

## Objective

Produce an evidence-bound compatibility and transcript-binding design for
future explicit qsc handshake suite-id semantics without implementing a qsc
suite-id field, `QHSM` schema change, QSP wire-format change, crypto
state-machine change, key schedule change, dependency change, or production
handshake implementation change in NA-0307.

## Protected invariants

- No qsc suite-id wire-format implementation in NA-0307.
- No qsc suite-id test seam implementation in NA-0307.
- No silent protocol semantic change.
- No silent crypto state-machine change.
- No production handshake implementation change.
- No key schedule implementation change.
- No QSP wire-format implementation change.
- No dependency, workflow, service, website, docs/public, README, START_HERE,
  branch-protection, or public-safety configuration drift.
- The missing explicit qsc handshake suite-id admission evidence remains
  visible.
- Persisted Suite-2 state is not presented as explicit suite-id admission
  evidence.
- No claim is introduced that external review is finished, anonymity is
  provided, metadata-free behavior exists, untraceability is provided, or the
  system is ready for unrestricted deployment.

## Allowed scope

- `docs/governance/evidence/NA-0307_qsc_handshake_suite_id_compatibility_transcript_design.md`
- `tests/NA-0307_qsc_handshake_suite_id_compatibility_transcript_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden scope

- qsc runtime source under `qsl/qsl-client/qsc/src/**`
- qsc runtime tests under `qsl/qsl-client/qsc/tests/**`
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

## Prior design review requirements

The evidence must review NA-0304 through NA-0306 and preserve these findings:

- valid qsc handshake activation persists Suite-2 state with protocol version
  `0x0500` and suite id `0x0002`;
- qsc `QHSM` A1/B1/A2 frames have no explicit suite-id field;
- appending suite-id bytes to current frames proves only generic length reject;
- unsupported, downgraded, malformed, stripped, and mismatched qsc handshake
  suite-id admission inputs remain blocked by the missing field; and
- the missing direct admission evidence remains visible.

## Compatibility analysis requirements

The evidence must classify compatibility options as RECOMMENDED, POSSIBLE,
RISKY, or REJECTED and cover:

- legacy `QHSM` frame behavior;
- suite-id-required mode reject behavior;
- compatibility mode boundaries;
- version-gating and old/new parser expectations;
- canonical parameter-block ordering, duplicate handling, unknown parameter
  handling, and length limits;
- downgrade stripping and fallback prevention;
- rollout/migration requirements; and
- compatibility risks such as interop ambiguity, parser divergence, transcript
  mismatch, and operational rollback.

## Transcript-binding analysis requirements

The evidence must decide:

- whether suite context is transcript-bound;
- which A1/B1/A2 copy is authoritative;
- whether B1/A2 echo or confirm byte-exact canonical parameter blocks;
- how B1/A2 omission or alteration rejects;
- what formal/model properties must be added; and
- what vectors and qsc harnesses must assert.

## Key-schedule posture requirements

The evidence must decide or explicitly defer with a STOP condition:

- whether future suite context feeds qsc handshake KDF/context inputs;
- which future derivation surfaces need a selected context shape;
- what vectors must cover the selected posture; and
- when a future implementation lane must STOP.

NA-0307 must not implement any key schedule change.

## Reject taxonomy requirements

The evidence must define fail-closed behavior, mutation boundary, output
boundary, log/secret boundary, vector requirement, formal/model requirement,
and qsc harness requirement for:

- unsupported suite id;
- downgraded suite id;
- missing suite id when required;
- suite id stripped from frame;
- mismatched suite id across frames;
- duplicate suite parameter;
- unknown critical parameter;
- unknown parameter under the selected policy;
- noncanonical parameter order;
- invalid parameter length;
- inconsistent protocol version and suite id;
- legacy frame presented in suite-id-required mode; and
- new frame presented to an old parser.

## Successor-selection requirements

The selected successor must be exact. If implementation would require
unresolved compatibility, transcript-binding, key-schedule, model, or vector
decisions, the successor must not be an implementation lane.

Selected successor:

NA-0308 -- qsc Handshake Suite-ID Formal Model and Vector Design

## Claim-boundary requirements

The evidence must explicitly reject:

- treating generic frame-length rejects as suite-id-specific admission
  behavior;
- treating persisted Suite-2 state as explicit qsc handshake suite-id
  admission evidence;
- hiding the missing suite-id seam;
- smuggling a production wire/schema change into NA-0307;
- implying production deployment readiness;
- implying public-internet suitability;
- implying external review completion;
- implying anonymity, metadata-free behavior, or untraceability; and
- claiming qsc suite-id implementation work in NA-0307.

## Required local checks

- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/run_model_checks.py`
- `scripts/ci/metadata_conformance_smoke.sh`
- `scripts/ci/metadata_phase2_identifier_padding_harness.sh`
- `scripts/ci/metadata_phase2_sanitized_errors_retention_harness.sh`
- `scripts/ci/demo_cli_smoke.sh`
- `DEMO_STRESS_PROFILE=baseline scripts/ci/demo_adversarial_stress.sh`
- `DEMO_SOAK_RUNS=3 scripts/ci/demo_soak_repeated_run.sh`
- `cargo +stable test -p qsc --locked --test na_0304_handshake_suite_id_negotiation -- --test-threads=1 --nocapture`
- `cargo +stable test -p qsc --locked --test na_0303_handshake_activation_negotiation -- --test-threads=1 --nocapture`
- `cargo +stable test -p qsc --locked --test na_0302_suite2_negotiation_cross_surface -- --test-threads=1 --nocapture`
- `cargo +stable test -p quantumshield_refimpl --locked --test na_0301_suite2_negotiation_downgrade -- --test-threads=1 --nocapture`
- `cargo +stable test -p quantumshield_refimpl --locked --test na_0300_core_replay_reject_no_mutation -- --test-threads=1 --nocapture`
- `cargo +stable build -p qshield-cli --locked`
- `cargo +stable test -p qshield-cli --locked -- --test-threads=1`
- `git diff --check`
- direct overclaim phrase scan over changed lines
- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --allowed docs/governance/evidence/NA-0307_qsc_handshake_suite_id_compatibility_transcript_design.md --allowed tests/NA-0307_qsc_handshake_suite_id_compatibility_transcript_testplan.md --allowed DECISIONS.md --allowed TRACEABILITY.md --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `python3 scripts/ci/qsl_evidence_helper.py link-check`
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`
- goal-lint using a PR body containing `Goals: G1, G2, G3, G4, G5`
- `bash scripts/ci/classify_ci_scope.sh <changed_paths>`

## CI expectations

Required CI and public-safety must complete normally. Because NA-0307 is
governance/testplan-only, full-suite cost-control skips are acceptable only
when classifier and public-safety jobs report intentional skip behavior.

## Successor handoff

After the NA-0307 PR merges and post-merge public-safety is green, a separate
closeout may mark NA-0307 DONE and restore exactly one READY successor:

NA-0308 -- qsc Handshake Suite-ID Formal Model and Vector Design

The closeout must not implement NA-0308.
