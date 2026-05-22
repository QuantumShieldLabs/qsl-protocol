Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-22

# NA-0335 Metadata Runtime qshield Demo Cover Traffic Prototype Harness

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0335 implements the bounded qshield embedded relay/demo cover-traffic
prototype harness authorized by NA-0334. The implementation is opt-in through
`QSHIELD_DEMO_COVER_TRAFFIC=1`, deterministic under
`QSHIELD_DEMO_COVER_TRAFFIC_TEST_MODE=1`, and limited to qshield demo relay and
receive behavior.

The prototype supports the three authorized modes only:

- synthetic local cover;
- active-session cover;
- batch-fill cover.

It rejects or leaves unimplemented fixed-rate cover, qsl-server production
relay cover, qsl-attachments production object cover, public-internet cover,
transport padding expansion, and stronger privacy/readiness claims.

The executable harness proves policy gating, caps, retention and purge,
backup/ops boundary, abuse boundary, secret-free artifacts, real-message
priority, no recursive cover generation, batching/retry/jitter compatibility,
qshield demo boundary, service-production boundary, and prohibited-claim
boundaries.

## Live NA-0335 Scope

Live `NEXT_ACTIONS.md` entry:

- `NA-0335 -- Metadata Runtime qshield Demo Cover Traffic Prototype Implementation Harness`
- Status: READY
- Goals: G1, G2, G3, G4, G5
- Objective: execute the bounded qshield demo-only cover-traffic prototype
  implementation harness selected by NA-0334, or stop on an exact prerequisite.

Allowed implementation scope used:

- `apps/qshield-cli/src/commands/recv.rs`
- `apps/qshield-cli/src/commands/relay.rs`
- `apps/qshield-cli/src/relay_client.rs`
- `apps/qshield-cli/tests/na_0335_metadata_runtime_cover_traffic_prototype.rs`
- this evidence file
- `tests/NA-0335_metadata_runtime_qshield_demo_cover_traffic_prototype_harness_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden scope preserved:

- no qsl-server or qsl-attachments changes;
- no qsc, qsp, protocol, crypto, state-machine, or key-schedule changes;
- no Cargo/dependency, workflow, branch-protection, public-safety, website,
  README, START_HERE, docs/public, qsc-desktop, formal, input, tools/refimpl,
  or production-service changes.

## Inherited NA-0334 Authorization

NA-0334 D-0650 authorized one future bounded qshield demo-only cover-traffic
prototype implementation harness. The authorized modes were synthetic local
cover, active-session cover, and batch-fill cover inside the qshield embedded
relay/demo surface.

NA-0334 rejected or deferred fixed-rate cover, qsl-server production relay
cover, qsl-attachments production object cover, public-internet cover, and
stronger privacy/readiness claims.

## Inherited NA-0333 Caps

The implementation inherits these caps:

- payload per cover item: 8192 bytes;
- cover items per minute: 4;
- cover items per hour: 32;
- cover items per run/local day: 64;
- cover payload per run: 512 KiB;
- estimated request bytes per run: about 1 MiB;
- queued cover items: 16 global and 4 per route;
- retained cover artifacts: 4 per run and 1 MiB total;
- abort before generation when a cap would be exceeded;
- abort before generation when `/srv/qbuild` free disk is below 10 GiB.

The implementation checks the disk floor before cover generation. Deterministic
tests use an env override for the disk watermark so CI is not host-layout
dependent.

## Sources Inspected

- `NEXT_ACTIONS.md`
- `GOALS.md`
- `PROJECT_CHARTER.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `CHECKLIST_PROTOCOL_CHANGE.md`
- `tests/NA-0334_closeout_restore_na0335_testplan.md`
- `docs/governance/evidence/NA-0334_metadata_runtime_qshield_demo_cover_traffic_prototype_authorization.md`
- `tests/NA-0334_metadata_runtime_qshield_demo_cover_traffic_prototype_authorization_testplan.md`
- `docs/governance/evidence/NA-0333_metadata_runtime_cover_traffic_cost_quota_retention_prerequisite_plan.md`
- `docs/governance/evidence/NA-0332_metadata_runtime_cover_traffic_risk_gate_deferred_authorization.md`
- `docs/governance/evidence/NA-0331_metadata_runtime_qshield_demo_batching_harness.md`
- `apps/qshield-cli/src/commands/send.rs`
- `apps/qshield-cli/src/commands/recv.rs`
- `apps/qshield-cli/src/commands/relay.rs`
- `apps/qshield-cli/src/relay_client.rs`
- qshield NA-0318, NA-0319, NA-0320, NA-0322, NA-0324, NA-0327, NA-0329, and NA-0331 harnesses.

Search coverage included cover traffic, prototype, synthetic cover,
active-session cover, batch-fill cover, fixed-rate cover, quota, retention,
purge, backup, abuse, DoS, cost, qshield demo, qsl-server, qsl-attachments,
production, timing hidden, traffic hidden, metadata-free, anonymity,
untraceable, `FUTURE_GATE`, and `NOT_READY`.

## Implementation Summary or Blocker

No implementation blocker was found inside the authorized qshield surface.

Implemented:

- `relay.rs` adds env-gated `/cover-traffic`, `/cover-traffic/status`, and
  `/cover-traffic/purge` demo endpoints.
- `relay.rs` tags cover candidates separately from user candidates, enforces
  NA-0333 caps before generation, stores only bounded in-memory artifact
  summaries, prioritizes real candidates before cover candidates, and prevents
  legacy destructive poll from returning cover as plaintext.
- `recv.rs` recognizes tagged cover candidates and acks them without sending
  them to the actor or printing them as user plaintext.
- `relay_client.rs` accepts the relay `cover` boolean on candidate responses.
- `na_0335_metadata_runtime_cover_traffic_prototype.rs` proves required
  markers and artifact safety.

## Cover Traffic Prototype Policy

- Policy: `qshield_demo_cover_traffic_v1`.
- Opt-in gate: `QSHIELD_DEMO_COVER_TRAFFIC=1`.
- Disabled behavior: `/cover-traffic` returns `cover traffic disabled`.
- Deterministic test mode: `QSHIELD_DEMO_COVER_TRAFFIC_TEST_MODE=1`.
- Disk floor override for tests:
  `QSHIELD_DEMO_COVER_TRAFFIC_DISK_FREE_BYTES`.
- Test clock field accepted only in deterministic test mode: `test_now_ms`.

## Deterministic Test Mode

The harness starts a local qshield demo relay with test mode enabled and no
long sleeps. Cover payload bytes are deterministic in test mode. The harness
uses fixed local timestamps to exercise minute-window and artifact-retention
boundaries without waiting.

## Future / Actual Implementation Boundary

Actual implementation:

- qshield embedded relay/demo cover endpoint;
- qshield embedded relay/demo cover status and purge endpoints;
- qshield receive-side cover candidate skip/ack path;
- in-memory cover artifact summaries under retention caps.

Not implemented:

- fixed-rate cover;
- production qsl-server relay cover;
- qsl-attachments production object cover;
- public-internet cover;
- transport padding expansion;
- protocol, crypto, qsc, or qsp behavior.

## Cost / Quota / Retention / Backup / Abuse Behavior

The relay rejects cover generation before mutation if:

- payload/item exceeds 8192 bytes;
- 4/minute, 32/hour, 64/run, 512 KiB/run, or about 1 MiB request/run caps would
  be exceeded;
- queued cover would exceed 16 global or 4 per route;
- existing relay total/per-recipient/per-token queue caps would be exceeded;
- retained artifact bytes would exceed 1 MiB;
- the `/srv/qbuild` disk floor check is below 10 GiB or cannot be proven.

Retained artifacts are in-memory summaries only: mode, payload length, hashed
route tag, artifact tag, timestamp, and retained byte estimate. Before adding
new summaries, the relay purges oldest retained summaries as needed to keep the
retained count at or below four.

Backup impact: no new durable evidence or runtime artifact location is created
outside qsl-protocol tracked files. Runtime cover artifacts are in-memory relay
state and temporary test stores under system temp paths. No backup-plan update
is required.

## Synthetic Local Cover Proof

Harness test:
`allowed_cover_modes_are_tagged_bounded_and_real_messages_have_priority`

Proof:

- queues one synthetic local cover item;
- verifies the candidate is tagged as cover;
- verifies payload length is bounded and hex-encoded;
- emits `NA0335_SYNTHETIC_LOCAL_COVER_OK`.

## Active-Session Cover Proof

Harness test:
`allowed_cover_modes_are_tagged_bounded_and_real_messages_have_priority`

Proof:

- queues one active-session cover item with a source peer;
- rejects active-session mode without a source peer;
- verifies the candidate is tagged as cover;
- emits `NA0335_ACTIVE_SESSION_COVER_OK`.

## Batch-Fill Cover Proof

Harness tests:

- `allowed_cover_modes_are_tagged_bounded_and_real_messages_have_priority`
- `cover_traffic_coexists_with_batching_retry_and_jitter_demo_policy`

Proof:

- queues two batch-fill cover items under the per-route cap;
- verifies qshield demo batching still accepts a valid two-member batch;
- verifies real batch candidates are returned before cover candidates;
- emits `NA0335_BATCH_FILL_COVER_OK` and
  `NA0335_BATCHING_RETRY_JITTER_STILL_BOUNDED_OK`.

## Real-Message Priority Proof

Candidate polling now orders non-cover candidates before cover candidates while
preserving each class order. The harness queues a real demo message plus cover
items and verifies the real message is returned first.

Marker: `NA0335_REAL_MESSAGE_PRIORITY_OK`

## No Recursive Cover-Generation Proof

Cover is generated only by an explicit authenticated `/cover-traffic` request.
Polling, acking, receiving, purging, and batching do not create new cover
items. The harness checks queue counts are stable across repeated status reads.

Marker: `NA0335_NO_RECURSIVE_COVER_GENERATION_OK`

## Batching / Retry / Jitter Preservation Proof

The cover prototype does not modify NA-0327 retry-cadence or NA-0329
bounded-jitter ledger logic. It also does not modify NA-0331 send-batch or
ack-batch semantics. The NA-0335 coexistence test runs cover and batching
toggles together and verifies the batching policy remains
`qshield_demo_batching_v1`.

Required validation also reruns the NA-0331, NA-0329, and NA-0327 harnesses.

## Artifact / Log Safety Proof

The harness scans cover status, purge responses, and receive output for route
token sentinel, raw handle sentinel, candidate/ack sentinel, plaintext
sentinel, padding sentinel, passphrase sentinel, raw key sentinel, panic,
backtrace, and sensitive absolute path fragments.

Harness output:

- `COVER_ARTIFACT_SECRET_FINDING_COUNT 0`
- `COVER_ARTIFACT_SIZE_WITHIN_CAP_OK`
- `COVER_ARTIFACT_COUNT_WITHIN_CAP_OK`

## Harness Markers

The NA-0335 harness emits:

- `NA0335_COVER_TRAFFIC_PROTOTYPE_AUTHORIZATION_OK`
- `NA0335_QSHIELD_DEMO_COVER_TRAFFIC_POLICY_OK`
- `NA0335_SYNTHETIC_LOCAL_COVER_OK`
- `NA0335_ACTIVE_SESSION_COVER_OK`
- `NA0335_BATCH_FILL_COVER_OK`
- `NA0335_COVER_ITEM_QUOTA_BOUNDARY_OK`
- `NA0335_COVER_ITEM_RETENTION_BOUNDARY_OK`
- `NA0335_COVER_ITEM_PURGE_BOUNDARY_OK`
- `NA0335_COVER_ITEM_BACKUP_BOUNDARY_OK`
- `NA0335_COVER_ITEM_ABUSE_BOUNDARY_OK`
- `NA0335_COVER_ITEM_SECRET_FREE_ARTIFACT_OK`
- `NA0335_REAL_MESSAGE_PRIORITY_OK`
- `NA0335_NO_RECURSIVE_COVER_GENERATION_OK`
- `NA0335_BATCHING_RETRY_JITTER_STILL_BOUNDED_OK`
- `NA0335_NO_PRODUCTION_COVER_TRAFFIC_OK`
- `NA0335_QSHIELD_DEMO_BOUNDARY_OK`
- `NA0335_SERVICE_PRODUCTION_BOUNDARY_OK`
- `NA0335_NO_TIMING_HIDDEN_CLAIM_OK`
- `NA0335_NO_TRAFFIC_SHAPE_HIDDEN_CLAIM_OK`
- `NA0335_NO_METADATA_FREE_CLAIM_OK`
- `NA0335_NO_ANONYMITY_CLAIM_OK`
- `NA0335_METADATA_RUNTIME_COVER_TRAFFIC_PROTOTYPE_OK`

No required marker remains blocked.

## qshield Embedded Relay / Demo Boundary

This evidence is limited to qshield embedded relay/demo behavior. It proves a
bounded local/demo prototype and does not prove production service cover
behavior.

## qsl-server / qsl-attachments Production Boundary

qsl-server production relay cover remains unimplemented and unproven.
qsl-attachments production object cover remains unimplemented and unproven.
Both remain cross-repo-gated.

## Claim Boundaries

This lane does not claim:

- timing metadata is hidden;
- traffic shape is hidden;
- metadata-free behavior;
- anonymity;
- untraceable behavior;
- production readiness;
- public-internet readiness;
- external review completion.

## Recovered Failures

Recovered failure 1:

- failing command:
  `cargo +stable test -p qshield-cli --locked --test na_0335_metadata_runtime_cover_traffic_prototype -- --test-threads=1 --nocapture`
- classification: recoverable in-scope local compile failure with understood
  cause.
- cause: local test variable shadowed the `status()` helper.
- corrective action: renamed the local variable.
- final result: targeted NA-0335 harness passed.

Recovered failure 2:

- failing command:
  `cargo +stable test -p qshield-cli --locked --test na_0335_metadata_runtime_cover_traffic_prototype -- --test-threads=1 --nocapture`
- classification: recoverable in-scope local test fixture failure with
  understood cause.
- cause: a deterministic retention fixture used exactly the 60 second
  minute-window boundary, which is intentionally still inside the window.
- corrective action: moved the fixture timestamp one millisecond outside the
  window.
- final result: targeted NA-0335 harness passed.

## Selected Successor

Selected successor:

`NA-0336 -- Metadata Runtime Padding Bucket Expansion Authorization Plan`

Rationale: NA-0335 succeeded inside the qshield embedded relay/demo boundary,
leaving qsl-server/qsl-attachments production cover gated. The next safest
metadata-runtime lane is an authorization plan for padding bucket expansion,
not production cover, service timing, or further cover blocker resolution.

Rejected successors:

- `NA-0336 -- Metadata Runtime qshield Demo Cover Traffic Prototype Blocker Resolution`
  because no implementation blocker remains.
- `NA-0336 -- Metadata Runtime Service Timing Cross-Repo Authorization`
  because NA-0335 did not create a service-production proof and padding bucket
  planning is the safer next local metadata lane.

## Backup-Plan Impact Statement

No backup-plan update is required. Tracked changes stay under the qsl-protocol
worktree covered by `/srv/qbuild/work`. Runtime cover artifacts are in-memory
relay state and deterministic test temp data, not durable evidence locations.

## Next Recommendation

After merge and green public-safety, close NA-0335 and restore exactly one
READY item:

`NA-0336 -- Metadata Runtime Padding Bucket Expansion Authorization Plan`
