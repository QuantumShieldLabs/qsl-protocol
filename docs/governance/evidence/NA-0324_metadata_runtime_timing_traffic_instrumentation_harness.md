Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-20

# NA-0324 Metadata Runtime Timing and Traffic-Shape Instrumentation Harness

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0324 adds a bounded qshield embedded relay/demo instrumentation harness for
metadata-runtime timing and traffic-shape measurement. The harness is executable
test evidence only. It records trace-safe start/complete events around send,
candidate fetch, local verify, ack/commit, invalid retry, output
classification, queue cadence, padding/size classes, and ordering/correlation
classes.

This lane does not implement runtime mitigation. It does not add jitter,
batching, cover traffic, queue scheduling, send scheduling, receive scheduling,
transport padding, qsl-server behavior, qsl-attachments behavior, protocol
changes, crypto changes, or dependency changes.

Selected successor:

`NA-0325 -- Metadata Runtime Timing and Traffic-Shape Mitigation Option Matrix`

## Live NA-0324 Scope

The live queue item is `NA-0324 -- Metadata Runtime Timing and Traffic-Shape
Instrumentation Harness`, status `READY`, with goals G1 through G5.

Allowed work:

- build bounded qshield embedded relay/demo instrumentation evidence, or stop
  on an exact prerequisite;
- use secret-safe trace artifact schema for relative monotonic timings and
  classified event data;
- emit markers for event timing capture, queue cadence, padding size classes,
  invalid retry cadence, no-secret trace artifacts, and measurement boundaries;
- preserve the qshield embedded relay/demo versus qsl-server/qsl-attachments
  production boundary.

Forbidden work:

- timing mitigation, jitter, batching, cover traffic, queue scheduling, send
  scheduling, receive scheduling, transport padding, or service deployment
  implementation;
- qshield runtime source changes;
- qsl-server or qsl-attachments changes;
- qsc/qsp/protocol/crypto/key-schedule changes;
- dependency, workflow, website, README, START_HERE, branch-protection, or
  public-safety configuration changes;
- unsupported claims that timing metadata or traffic shape is hidden.

## Inherited NA-0323 Design

NA-0323 selected test-only qshield demo instrumentation as the safest next
step after NA-0322 measurement. The design required:

- relative monotonic timing or deterministic sequence fields;
- stable coarse event names;
- queue depth, bucket, retry, and correlation classes;
- bounded JSONL artifacts under temporary qbuild storage;
- direct scans for route tokens, raw handles, ack/candidate IDs, plaintext,
  padding sentinels, passphrases, raw key material, panic/backtrace text, and
  sensitive absolute paths;
- explicit wording that instrumentation measures observable behavior and is not
  mitigation.

## Inherited NA-0322 Measurement Evidence

NA-0322 proved that qshield embedded relay/demo timing and traffic-shape
measurement can be executed without qshield runtime source changes. It measured
explicit send cadence, candidate fetch cadence, queue depth snapshots, valid
ack/commit timing, invalid receive retry cadence, padding bucket classes, and
ordering/correlation visibility.

NA-0322 did not show that timing metadata is hidden, that traffic shape is
hidden, that mitigation exists, or that qsl-server/qsl-attachments production
timing is proven.

## Sources Inspected

- `NEXT_ACTIONS.md` live NA-0324 entry.
- `tests/NA-0323_closeout_restore_na0324_testplan.md`.
- `docs/governance/evidence/NA-0323_metadata_runtime_timing_traffic_shape_instrumentation_mitigation_design.md`.
- `tests/NA-0323_metadata_runtime_timing_traffic_shape_instrumentation_mitigation_design_testplan.md`.
- `docs/governance/evidence/NA-0322_metadata_runtime_timing_traffic_measurement_harness.md`.
- `apps/qshield-cli/tests/na_0322_metadata_runtime_timing_traffic_measurement.rs`.
- `apps/qshield-cli/tests/na_0320_metadata_runtime_sanitized_retention.rs`.
- `apps/qshield-cli/tests/na_0319_metadata_runtime_identifier_padding.rs`.
- `apps/qshield-cli/tests/na_0318_qshield_ack_commit.rs`.
- `apps/qshield-cli/src/commands/relay.rs`.
- `apps/qshield-cli/src/commands/recv.rs`.
- `apps/qshield-cli/src/commands/send.rs`.
- `scripts/ci/demo_adversarial_stress.sh`.
- `scripts/ci/demo_soak_repeated_run.sh`.
- `TRACEABILITY.md` and `DECISIONS.md`.

## Instrumentation Harness Summary or Blocker

No blocker was found for bounded qshield embedded relay/demo instrumentation.
No runtime hook, service change, dependency change, or workflow change was
required.

Added executable harness:

- `apps/qshield-cli/tests/na_0324_metadata_runtime_timing_traffic_instrumentation.rs`

The harness starts real qshield embedded relay processes, drives real relay JSON
requests, runs real invalid `qshield recv` attempts, records a bounded JSONL
trace artifact under `/srv/qbuild/tmp`, validates the trace schema, scans the
trace for sensitive values observed during the run, and emits all NA-0324
instrumentation and boundary markers.

## Trace Artifact Schema

Runtime artifact path shape:

- default: `/srv/qbuild/tmp/NA-0324_metadata_runtime_timing_traffic_instrumentation_<run>/na0324_metadata_runtime_timing_traffic_instrumentation.jsonl`
- fallback: process temp directory if `/srv/qbuild/tmp` cannot be created;
- override: `NA0324_ARTIFACT_DIR`.

Each JSONL event contains:

- `schema_version`: `qsl.na0324.metadata_runtime_timing_traffic_instrumentation.v1`
- `run_id`: `na0324-qshield-demo-instrumentation-bounded`
- `event_id`: deterministic event sequence label;
- `sequence`;
- `relative_ms`;
- `event`;
- `duration_class`;
- `phase_class`;
- `queue_depth_class`;
- `padding_size_class`;
- `retry_count_class`;
- `correlation_class`;
- `boundary`: `qshield_embedded_relay_demo_only`;
- `artifact_class`: `instrumentation_measurement_not_mitigation`.

Raw route tokens, raw handles, raw candidate/ack IDs, plaintext, raw output,
passphrases, raw key material, and local sensitive paths are not recorded.

## Trace Artifact Safety

The harness scans trace artifacts and command output for:

- route token sentinel;
- raw handle sentinel;
- candidate/ack sentinel and concrete ack IDs observed during the run;
- plaintext sentinel;
- padding sentinel;
- passphrase sentinel;
- raw key sentinel;
- panic/backtrace/Python traceback markers;
- sensitive absolute path prefixes.

Targeted run:

- command: `cargo +stable test -p qshield-cli --locked --test na_0324_metadata_runtime_timing_traffic_instrumentation -- --test-threads=1 --nocapture`
- result: pass, 2 tests.
- artifact path: `/srv/qbuild/tmp/NA-0324_metadata_runtime_timing_traffic_instrumentation_96447-1779306367832377510/na0324_metadata_runtime_timing_traffic_instrumentation.jsonl`
- emitted result: `TRACE_ARTIFACT_SECRET_FINDING_COUNT 0`.

## Event Timing Capture

The trace records relative monotonic event positions plus duration classes for
operation completion. The required event labels are present:

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
- `output_write_start`
- `output_write_complete`

Marker:

- `NA0324_QSHIELD_EVENT_TIMING_CAPTURE_OK`

## Queue Cadence Instrumentation

The harness records queue classes before send, after send, before/after
candidate fetch, before/after ack, and before/after invalid retry. Queue
classes are coarse (`empty`, `single`, `small_bounded`, or explicit
not-observed labels) and do not include raw queue contents.

Marker:

- `NA0324_QUEUE_CADENCE_INSTRUMENTATION_OK`

## Padding/Size Class Instrumentation

The harness sends two bounded padded samples through the embedded relay and
records only bucket classes (`bucket_512`, `bucket_1024`) and coarse observable
size class data. It does not record raw plaintext or raw plaintext length.

Marker:

- `NA0324_PADDING_SIZE_CLASS_INSTRUMENTATION_OK`

## Invalid Retry Instrumentation

The harness queues a malformed message for a demo session, runs two invalid
`qshield recv` attempts, records first/repeat retry classes, proves the same
candidate remains queued, and classifies sanitized output without recording raw
output.

Marker:

- `NA0324_INVALID_RETRY_INSTRUMENTATION_OK`

## Ordering/Correlation Instrumentation

The harness records front-of-queue and ack-order visibility as qshield
embedded relay/demo correlation classes. It does not claim contact-graph,
route, endpoint, service, or ordering-correlation resistance.

## Harness Markers

Emitted markers:

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

## Measurement Versus Mitigation Boundary

Instrumentation records observable timing and traffic-shape classes. It is not
runtime mitigation. No jitter, batching, cover traffic, scheduling, transport
padding, retry normalization, queue-drain smoothing, or service behavior is
implemented here.

## qshield Embedded Relay/Demo Boundary

This evidence is bounded to qshield embedded relay/demo behavior under local
test control. It exercises qshield demo relay endpoints and qshield CLI receive
behavior only.

## qsl-server/qsl-attachments Production Boundary

qsl-server production relay timing is not proven. qsl-attachments production
upload/fetch timing, descriptor/object size timing, capability use timing,
retention side effects, backup artifacts, and service logs are not proven. Any
service timing instrumentation requires a separate cross-repo authorization
lane.

## Limitations

- The harness runs on local loopback and does not model public internet,
  proxy/CDN, mobile, desktop shell, or deployment timing.
- The artifact records classes and relative event sequence for bounded samples,
  not statistically meaningful latency distributions.
- It does not implement or validate mitigation behavior.
- It does not prove production service timing for qsl-server or
  qsl-attachments.

## Selected Successor

`NA-0325 -- Metadata Runtime Timing and Traffic-Shape Mitigation Option Matrix`

## Backup-Plan Impact Statement

No backup-plan update is required. Tracked evidence remains under the
qsl-protocol worktree in `/srv/qbuild/work`, and runtime trace artifacts remain
temporary under `/srv/qbuild/tmp`. No durable non-rebuildable artifact was
introduced outside current backup scope.

## Next Recommendation

Run NA-0325 as a mitigation option matrix before any runtime mitigation
authorization. The matrix should compare fixed intervals, bounded jitter,
batching, cover traffic, queue-drain scheduling, retry cadence normalization,
padding bucket expansion, and service-specific timing lanes while preserving
the qshield demo versus production-service boundary.
