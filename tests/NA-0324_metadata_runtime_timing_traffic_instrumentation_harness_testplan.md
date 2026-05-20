Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-20

# NA-0324 Metadata Runtime Timing and Traffic-Shape Instrumentation Harness Testplan

Goals: G1, G2, G3, G4, G5

## Objective

Validate that NA-0324 adds bounded qshield embedded relay/demo instrumentation
for metadata-runtime timing and traffic-shape measurement without implementing
runtime mitigation.

## Protected Invariants

- Instrumentation remains measurement evidence, not mitigation.
- Trace artifacts do not include route tokens, raw handles, raw candidate/ack
  IDs, plaintext sentinels, padding sentinels, passphrases, raw key material,
  panic/backtrace text, or sensitive absolute paths.
- Timing metadata and traffic shape are not claimed hidden.
- qshield embedded relay/demo evidence remains distinct from qsl-server and
  qsl-attachments production behavior.
- qsl-server and qsl-attachments production timing remain unproven.
- No anonymity, metadata-free, untraceable, production-readiness,
  public-internet-readiness, or external-review-complete claim is introduced.

## Allowed Scope

- `apps/qshield-cli/tests/na_0324_metadata_runtime_timing_traffic_instrumentation.rs`
- `docs/governance/evidence/NA-0324_metadata_runtime_timing_traffic_instrumentation_harness.md`
- `tests/NA-0324_metadata_runtime_timing_traffic_instrumentation_harness_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

- Runtime mitigation implementation.
- Jitter, batching, cover traffic, queue scheduling, send scheduling, receive
  scheduling, transport padding, retry normalization, or service deployment
  behavior.
- qshield runtime source changes.
- qsl-server or qsl-attachments changes.
- qsc/qsp/protocol/crypto/key-schedule changes.
- Dependency, Cargo, workflow, website, README, START_HERE,
  branch-protection, or public-safety configuration changes.
- NA-0325 implementation.

## Prior Design Review Requirements

The evidence must review and preserve:

- live NA-0324 scope;
- inherited NA-0323 design;
- inherited NA-0322 measurement evidence;
- feasible qshield embedded relay/demo instrumentation surfaces;
- the runtime-change boundary;
- qsl-server and qsl-attachments production timing boundaries;
- stop conditions.

## Trace Artifact Schema Requirements

Trace events must include:

- `schema_version`;
- `run_id`;
- `event_id`;
- monotonic relative timestamp or deterministic sequence;
- coarse event label;
- `phase_class`;
- `queue_depth_class`;
- `padding_size_class`;
- `retry_count_class`;
- `correlation_class`;
- `boundary`;
- no raw secrets.

## Artifact Safety Requirements

The harness must scan trace artifacts for:

- route token sentinel;
- raw handle sentinel;
- candidate/ack sentinel and concrete sensitive ack IDs observed during the run;
- plaintext sentinel;
- padding sentinel;
- passphrase sentinel;
- raw key sentinel;
- panic/backtrace/Python traceback markers;
- sensitive absolute path prefixes.

`TRACE_ARTIFACT_SECRET_FINDING_COUNT 0` is required.

## Harness Marker Requirements

The executable harness should emit:

- `NA0324_TIMING_INSTRUMENTATION_PLAN_OK`
- `NA0324_QSHIELD_DEMO_TRACE_ARTIFACT_SCHEMA_OK`
- `NA0324_QSHIELD_EVENT_TIMING_CAPTURE_OK`
- `NA0324_QUEUE_CADENCE_INSTRUMENTATION_OK`
- `NA0324_PADDING_SIZE_CLASS_INSTRUMENTATION_OK`
- `NA0324_INVALID_RETRY_INSTRUMENTATION_OK`
- `NA0324_NO_SECRET_TRACE_ARTIFACT_OK`
- `NA0324_INSTRUMENTATION_NOT_MITIGATION_OK`
- `NA0324_MEASUREMENT_BEFORE_MITIGATION_OK`
- `NA0324_QSHIELD_DEMO_BOUNDARY_OK`
- `NA0324_QSL_SERVER_TIMING_NOT_PROVEN_OK`
- `NA0324_QSL_ATTACHMENTS_TIMING_NOT_PROVEN_OK`
- `NA0324_NO_TIMING_HIDDEN_CLAIM_OK`
- `NA0324_NO_TRAFFIC_SHAPE_HIDDEN_CLAIM_OK`
- `NA0324_NO_METADATA_FREE_CLAIM_OK`
- `NA0324_METADATA_TIMING_INSTRUMENTATION_HARNESS_OK`

If any marker cannot be truthfully emitted, the evidence must record the exact
blocker and select an exact successor.

## Event Timing Capture Requirements

The trace must include these coarse labels when feasible:

- `send_start`
- `send_complete`
- `candidate_fetch_start`
- `candidate_fetch_complete`
- `local_verify_start`
- `local_verify_complete`
- `ack_start`
- `ack_complete`
- `invalid_retry_start`
- `invalid_retry_complete`
- `output_write_start` or equivalent output classification start
- `output_write_complete` or equivalent output classification completion

## Queue Cadence Instrumentation Requirements

The harness must record queue classes around send, fetch, ack, and invalid
retry without recording raw queue contents or raw candidate values.

## Padding/Size Class Instrumentation Requirements

The harness must record only configured bucket or size classes and must not
record raw plaintext, raw plaintext length, or padding sentinels.

## Invalid Retry Instrumentation Requirements

The harness must run bounded invalid receive attempts, classify first/repeat
retry cadence, prove the candidate remains queued, and record sanitized output
classes without raw output.

## Measurement-Versus-Mitigation Requirements

Evidence and PR text must state that instrumentation is measurement evidence
only and that runtime timing mitigation was not implemented.

## Successor-Selection Requirements

If bounded qshield/demo instrumentation succeeds, select:

`NA-0325 -- Metadata Runtime Timing and Traffic-Shape Mitigation Option Matrix`

If runtime hooks or service instrumentation are required first, select an exact
authorization or blocker-resolution successor. Do not implement NA-0325.

## Claim-Boundary Requirements

Evidence and PR text must state:

- no anonymity claim;
- no metadata-free claim;
- no untraceable claim;
- no production/public-internet readiness claim;
- no external-review-complete claim;
- no claim that timing metadata is hidden;
- no claim that traffic shape is hidden;
- no claim that runtime mitigation was implemented;
- all runtime gaps remain visible;
- qsl-server/qsl-attachments production boundaries remain explicit.

## Backup-Impact Requirements

Record whether durable evidence or artifacts were created outside current
backup scope. Expected result: no backup-plan update required if tracked
evidence remains under `/srv/qbuild/work` and runtime trace artifacts remain
temporary under `/srv/qbuild/tmp`.

## Required Local Checks

- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo fmt --check`
- `cargo +stable test -p qshield-cli --locked --test na_0324_metadata_runtime_timing_traffic_instrumentation -- --test-threads=1 --nocapture`
- `cargo +stable test -p qshield-cli --locked --test na_0322_metadata_runtime_timing_traffic_measurement -- --test-threads=1`
- `cargo +stable test -p qshield-cli --locked --test na_0320_metadata_runtime_sanitized_retention -- --test-threads=1`
- `cargo +stable test -p qshield-cli --locked --test na_0319_metadata_runtime_identifier_padding -- --test-threads=1`
- `cargo +stable test -p qshield-cli --locked --test na_0318_qshield_ack_commit -- --test-threads=1`
- `cargo +stable test -p qshield-cli --locked -- --test-threads=1`
- `cargo +stable build -p qshield-cli --locked`
- `bash scripts/ci/demo_cli_smoke.sh`
- `DEMO_STRESS_PROFILE=baseline bash scripts/ci/demo_adversarial_stress.sh`
- `DEMO_SOAK_RUNS=3 bash scripts/ci/demo_soak_repeated_run.sh` if feasible
- `bash scripts/ci/metadata_runtime_identifier_padding_harness_plan.sh`
- `bash scripts/ci/metadata_phase2_identifier_padding_harness.sh`
- `bash scripts/ci/metadata_phase2_sanitized_errors_retention_harness.sh`
- `bash scripts/ci/metadata_conformance_smoke.sh`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `python3 formal/model_qsc_handshake_suite_id_bounded.py`
- `python3 formal/run_model_checks.py`
- JSON parse for `inputs/suite2/qsc_handshake_suite_id_vectors_na0310.json`
- targeted refimpl NA-0310 oracle test
- full refimpl tests if feasible
- qsc NA-0313 harness if directly runnable
- `python3 scripts/ci/qsl_evidence_helper.py queue`
- `python3 scripts/ci/qsl_evidence_helper.py decisions`
- scope guard, link-check, leak-scan, goal-lint, overclaim scan, and classifier proof

## CI Expectations

Required checks must pass before merge, including `public-safety`. Cost-control
skips are acceptable only when CI reports them as skipped and `public-safety`
remains green.

## Successor Handoff

After NA-0324 implementation merges and post-merge public-safety is green, a
separate closeout may mark NA-0324 DONE and restore exactly one READY
successor:

`NA-0325 -- Metadata Runtime Timing and Traffic-Shape Mitigation Option Matrix`
