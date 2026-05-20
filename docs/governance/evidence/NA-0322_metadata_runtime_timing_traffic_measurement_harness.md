Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-20

# NA-0322 Metadata Runtime Timing and Traffic-Shape Measurement Harness

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0322 adds a bounded qshield embedded relay/demo measurement harness for
observable timing and traffic-shape surfaces selected by NA-0321. The harness
records sender cadence, receiver cadence, queue cadence, ack/commit timing,
invalid retry cadence, padding bucket classes, and ordering/correlation
classification into a secret-safe JSONL artifact.

This is measurement evidence only. It does not implement runtime timing
mitigation, jitter, batching, cover traffic, send scheduling, receive
scheduling, transport padding, qsl-server behavior, or qsl-attachments
behavior. It does not claim anonymity, metadata-free behavior,
untraceability, production readiness, public internet readiness, external
review completion, hidden timing metadata, or hidden traffic shape.

Selected successor:

`NA-0323 -- Metadata Runtime Timing and Traffic-Shape Instrumentation / Mitigation Design Plan`

## Live NA-0322 Scope

The live queue item is `NA-0322 -- Metadata Runtime Timing and Traffic-Shape
Measurement Harness`, status `READY`, with goals G1 through G5.

Allowed work:

- build a bounded qshield/demo measurement harness or record an exact blocker;
- record observable sender cadence, receiver cadence, queue cadence, retry
  cadence, size/padding classes, and ordering/correlation signals;
- keep artifacts deterministic, bounded, and secret-safe;
- preserve qshield embedded relay/demo boundaries versus qsl-server and
  qsl-attachments production behavior;
- select the exact NA-0323 successor.

Forbidden work:

- runtime timing mitigation, jitter, batching, cover traffic, send scheduling,
  receive scheduling, transport padding, or service deployment behavior;
- qshield runtime source changes, qsl-server changes, qsl-attachments changes,
  qsc/qsp/protocol/crypto/key-schedule changes, dependency changes, workflow
  changes, website changes, README changes, START_HERE changes, or
  branch-protection/public-safety configuration changes;
- any unsupported production, public-internet, external-review, anonymity,
  metadata-free, untraceable, timing-hidden, or traffic-hidden claim.

## Inherited NA-0321 Threat Model

NA-0321 established that timing metadata and traffic shape remain observable.
The highest-confidence executable proof before this lane was bounded to the
qshield embedded relay/demo path:

- NA-0318 proved candidate fetch plus explicit ack/commit after local
  verification.
- NA-0319 proved bounded opaque candidate handles and default-padding buckets.
- NA-0320 proved selected sanitized-error and retention/purge behavior.

NA-0321 also recorded that qsl-server and qsl-attachments production timing
and traffic-shape behavior remain future-gated and cannot be inferred from
qshield embedded relay/demo evidence.

## Sources Inspected

- `NEXT_ACTIONS.md` live NA-0322 entry.
- `tests/NA-0321_closeout_restore_na0322_testplan.md`.
- `docs/governance/evidence/NA-0321_metadata_runtime_timing_traffic_shape_threat_model.md`.
- `tests/NA-0321_metadata_runtime_timing_traffic_shape_threat_model_testplan.md`.
- `docs/governance/evidence/NA-0320_metadata_runtime_sanitized_errors_retention_purge_harness.md`.
- `docs/governance/evidence/NA-0319_metadata_runtime_identifier_default_padding_harness.md`.
- `docs/governance/evidence/NA-0318_metadata_runtime_qshield_ack_commit_poll_implementation_harness.md`.
- `apps/qshield-cli/tests/na_0320_metadata_runtime_sanitized_retention.rs`.
- `apps/qshield-cli/tests/na_0319_metadata_runtime_identifier_padding.rs`.
- `apps/qshield-cli/tests/na_0318_qshield_ack_commit.rs`.
- `apps/qshield-cli/src/commands/relay.rs`.
- `apps/qshield-cli/src/commands/recv.rs`.
- `apps/qshield-cli/src/commands/send.rs`.
- `scripts/ci/demo_cli_smoke.sh`.
- `scripts/ci/demo_adversarial_stress.sh`.
- `scripts/ci/demo_soak_repeated_run.sh`.
- `TRACEABILITY.md` and `DECISIONS.md`.

## Measurement Harness Summary or Blocker

No blocker was found for a bounded qshield embedded relay/demo measurement
harness. No runtime hook or runtime source change was required.

The added executable harness is:

- `apps/qshield-cli/tests/na_0322_metadata_runtime_timing_traffic_measurement.rs`

It starts real qshield embedded relay processes, drives real relay JSON
requests, runs a real invalid `qshield recv`, measures elapsed monotonic
durations around each operation, writes a bounded JSONL artifact under a temp
artifact directory, scans that artifact for configured forbidden values, and
emits the NA-0322 measurement and boundary markers.

## Measurement Artifact Schema

Runtime artifact path shape:

- default local path: `/srv/qbuild/tmp/NA-0322_metadata_runtime_timing_traffic_measurement_<run>/na0322_metadata_runtime_timing_traffic_measurement.jsonl`
- fallback path when `/srv/qbuild/tmp` is unavailable: process temp directory.
- override: `NA0322_ARTIFACT_DIR`.

Each JSONL event contains:

- `schema_version`: `qsl.na0322.metadata_runtime_timing_traffic_measurement.v1`
- `run_id`: stable bounded run label.
- `sequence`: monotonic event sequence.
- `relative_ms`: relative elapsed milliseconds from harness start.
- `event`: coarse event label.
- `duration_us`: monotonic measured duration around the local/demo operation.
- `artifact_class`: `measurement_not_mitigation`.
- `boundary`: `qshield_embedded_relay_demo_only`.
- coarse surface/classification fields, such as queue depth, candidate count,
  bucket class, result class, and boundary classification.

The artifact intentionally does not record raw relay tokens, raw ack IDs, raw
candidate values, raw route handles, plaintext, raw output, or raw local paths.

## Measurement Artifact Safety

The harness scans the artifact for:

- route token sentinel;
- raw handle sentinel;
- candidate ack sentinel and concrete ack IDs observed during the run;
- plaintext sentinel;
- padding sentinel;
- passphrase sentinel;
- raw key sentinel;
- panic/backtrace/Python traceback markers.

Targeted run evidence:

- command: `cargo +stable test -p qshield-cli --locked --test na_0322_metadata_runtime_timing_traffic_measurement -- --test-threads=1 --nocapture`
- result: pass, 2 tests.
- emitted result: `ARTIFACT_SECRET_FINDING_COUNT 0`.

## Sender Cadence Measurement

The harness records elapsed duration around real relay `/send` operations. It
measures one explicit single-send event followed by a short explicit burst of
additional sends and records the result as `explicit_command_burst`.

Marker:

- `NA0322_SENDER_CADENCE_MEASURED_OK`

## Receiver Cadence Measurement

The harness records elapsed duration around real relay `/poll-candidate`
candidate fetch operations and real invalid `qshield recv` commands. The
receiver cadence classification is explicit local command timing, not an idle
polling or production receive schedule.

Marker:

- `NA0322_RECEIVER_CADENCE_MEASURED_OK`

## Queue Cadence Measurement

The harness records queue-depth observations from bounded candidate fetches:

- queue depth before send: zero candidates;
- after burst sends: three candidates;
- repeated candidate fetch: same front candidate remains queued;
- after valid ack: one candidate is deleted and the remaining candidate order is
  preserved.

Marker:

- `NA0322_QUEUE_CADENCE_MEASURED_OK`

## Ack/Commit Timing Measurement

The harness records elapsed duration around a real relay `/ack` request and
then confirms one matching candidate was deleted while the remaining candidate
stayed queued.

Marker:

- `NA0322_ACK_COMMIT_TIMING_MEASURED_OK`

## Invalid Retry Cadence Measurement

The harness queues a malformed message for a store with a bounded demo session,
runs invalid `qshield recv` twice, records both durations, proves both attempts
fail with the same coarse output, and proves the same candidate remains queued
after each reject.

Marker:

- `NA0322_INVALID_RETRY_CADENCE_BOUNDED_OK`

## Padding/Size Distribution Measurement

The harness sends two bounded padded payload classes through the real relay and
records only bucket classes:

- `bucket_512`
- `bucket_1024`

It confirms relay candidates carry those bucket classes and does not record raw
plaintext or raw plaintext size.

Marker:

- `NA0322_PADDING_SIZE_DISTRIBUTION_OK`

## Ordering/Correlation Classification

The harness classifies the qshield embedded relay/demo ordering surface as
front-of-queue candidate order and ack order visible within the relay/demo
boundary. It records that recipient queue and ack order remain observable. It
does not claim contact-graph, route, or ordering-correlation resistance.

Marker:

- `NA0322_ORDERING_CORRELATION_CLASSIFIED_OK`

## Harness Markers

Measurement markers emitted:

- `NA0322_TIMING_SURFACE_INVENTORY_OK`
- `NA0322_TRAFFIC_SHAPE_THREAT_MODEL_OK`
- `NA0322_QSHIELD_DEMO_TIMING_MEASUREMENT_OK`
- `NA0322_SENDER_CADENCE_MEASURED_OK`
- `NA0322_RECEIVER_CADENCE_MEASURED_OK`
- `NA0322_QUEUE_CADENCE_MEASURED_OK`
- `NA0322_ACK_COMMIT_TIMING_MEASURED_OK`
- `NA0322_INVALID_RETRY_CADENCE_BOUNDED_OK`
- `NA0322_PADDING_SIZE_DISTRIBUTION_OK`
- `NA0322_ORDERING_CORRELATION_CLASSIFIED_OK`
- `NA0322_NO_SECRET_TIMING_ARTIFACT_OK`
- `NA0322_MEASUREMENT_NOT_MITIGATION_OK`
- `NA0322_METADATA_TIMING_MEASUREMENT_HARNESS_OK`

Boundary markers emitted:

- `NA0322_QSHIELD_DEMO_BOUNDARY_OK`
- `NA0322_QSL_SERVER_TIMING_NOT_PROVEN_OK`
- `NA0322_QSL_ATTACHMENTS_TIMING_NOT_PROVEN_OK`
- `NA0322_NO_METADATA_FREE_CLAIM_OK`

## Metadata Claim Boundary

The measurement records observable timing and traffic-shape classes. It does
not claim timing metadata is hidden. It does not claim traffic shape is hidden.
It does not claim anonymity, metadata-free behavior, or untraceability.

## qshield Embedded Relay/Demo Boundary

All executable measurement in this lane is bounded to qshield embedded relay
and demo CLI behavior. The harness measures local loopback relay operations and
a local invalid `qshield recv` path. It is not a public-internet, deployment,
or production-service measurement.

## qsl-server/qsl-attachments Production Boundary

No qsl-server or qsl-attachments implementation changed. This evidence does
not prove qsl-server production relay timing, qsl-attachments production
upload/fetch timing, service access-log behavior, deployment timing, or public
internet behavior. Those require a separately authorized service timing lane.

## Measurement Versus Mitigation Boundary

The artifact class is `measurement_not_mitigation`. NA-0322 does not implement
or configure jitter, batching, cover traffic, send scheduling, receive
scheduling, transport padding, or service traffic shaping. Observable timing
and traffic-shape gaps remain visible.

## Limitations

- The harness measures bounded local/demo operations only.
- It does not measure qsl-server or qsl-attachments production behavior.
- It does not measure public internet timing, IP-level metadata, access logs,
  proxy logs, or deployment telemetry.
- It does not define or implement mitigation policy.
- It records bucket classes for bounded samples, not a complete traffic
  distribution for every product surface.

## Selected Successor

Selected successor:

`NA-0323 -- Metadata Runtime Timing and Traffic-Shape Instrumentation / Mitigation Design Plan`

Rationale: NA-0322 now provides bounded qshield/demo measurement evidence, so
the next precise lane should design instrumentation and mitigation strategy
without jumping directly to runtime jitter, batching, cover traffic, or service
changes.

Rejected successors:

- qsl-server/qsl-attachments service timing cross-repo authorization as the
  immediate successor, because the local qshield/demo measurement harness
  succeeded and should first feed an instrumentation/mitigation design plan.
- immediate mitigation implementation, because NA-0322 is measurement evidence
  only and no mitigation policy has been authorized.

## Backup-Plan Impact Statement

No backup-plan update is required. Durable tracked evidence stays under the
qsl-protocol worktree in `/srv/qbuild/work`. Runtime measurement artifacts are
temporary bounded files under `/srv/qbuild/tmp` or the process temp directory
and are not durable governance state.

## Next Recommendation

Run NA-0323 as an instrumentation and mitigation design plan that uses the
NA-0322 event vocabulary to decide what future runtime hooks, if any, are
needed before any timing/traffic-shape mitigation implementation is attempted.
