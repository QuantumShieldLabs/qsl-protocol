Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-21

# NA-0331 Metadata Runtime qshield Demo Batching Harness

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0331 implements the bounded qshield embedded relay/demo batching harness
authorized by NA-0330. The implementation is opt-in through
`QSHIELD_DEMO_BATCHING=1`, deterministic under
`QSHIELD_DEMO_BATCHING_TEST_MODE=1`, capped at four members, and limited to the
qshield local/demo relay plus qshield send/receive client surface.

The harness proves valid single-message behavior remains unchanged, valid
batch delivery is deterministic, batch ordering is preserved, invalid send and
ack batches fail closed all-or-nothing, invalid receive candidates create no
remote delete, no accepted local state, and no plaintext output, retry-cadence
and bounded-jitter caps remain intact, and batch artifacts are secret-safe.

This evidence is local/demo only. It does not claim that timing metadata or
traffic shape is hidden. It does not claim anonymity, metadata-free behavior,
untraceable behavior, production readiness, public-internet readiness, or
external-review completion.

## Live NA-0331 Scope

Live `NEXT_ACTIONS.md` entry:

- `NA-0331 -- Metadata Runtime qshield Demo Batching Implementation Harness`
- Status: READY
- Goals: G1, G2, G3, G4, G5
- Objective: execute the batching implementation/design/blocker lane selected
  by NA-0330 evidence, or stop on an exact prerequisite.

Allowed implementation scope used by this lane:

- `apps/qshield-cli/src/commands/send.rs`
- `apps/qshield-cli/src/commands/recv.rs`
- `apps/qshield-cli/src/commands/relay.rs`
- `apps/qshield-cli/src/relay_client.rs`
- `apps/qshield-cli/tests/na_0331_metadata_runtime_batching.rs`
- this evidence file
- `tests/NA-0331_metadata_runtime_qshield_demo_batching_harness_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden scope preserved:

- no qsl-server changes;
- no qsl-attachments changes;
- no qsc/qsp/protocol/crypto/key-schedule changes;
- no dependency, Cargo, workflow, branch-protection, public-safety, website,
  README, START_HERE, docs/public, formal, input, tools/refimpl, or
  qsc-desktop changes;
- no cover traffic, transport padding expansion, broad scheduling, production
  service timing, or NA-0332 implementation.

## Inherited NA-0330 Authorization

NA-0330 D-0642 authorized only a future bounded qshield embedded relay/demo
batching implementation harness with:

- policy name `qshield_demo_batching_v1`;
- opt-in switch `QSHIELD_DEMO_BATCHING=1`;
- deterministic test mode `QSHIELD_DEMO_BATCHING_TEST_MODE=1`;
- maximum batch size 4;
- maximum send-side batch window 750 ms;
- receive candidate batches at most 4;
- ack batches at most 4 locally verified ack IDs;
- send-batch all-or-nothing validation before mutation;
- receive processing in relay order, failing closed on invalid members;
- ack-batch all-or-nothing validation before deletion;
- no unsupported production, public-internet, external-review, anonymity,
  metadata-free, untraceable, timing-hidden, or traffic-shape-hidden claim.

## Inherited NA-0329 Bounded-Jitter Proof

NA-0329 D-0640 added opt-in qshield embedded relay/demo bounded jitter through
`QSHIELD_DEMO_BOUNDED_JITTER=1` and deterministic test mode through
`QSHIELD_DEMO_BOUNDED_JITTER_TEST_MODE=1`. The cap remains 0-250 ms jitter and
2250 ms composed retry plus jitter for invalid/empty-poll classes. NA-0331
does not change these constants or their ledger semantics.

## Inherited NA-0327 Retry-Cadence Proof

NA-0327 D-0636 added the opt-in qshield demo retry ledger with four invalid
candidate attempts per 60 second local window and retry classes 0 ms, 500 ms,
1000 ms, and 2000 ms. NA-0331 composes with that ledger and does not increase
the retry cap.

## Sources Inspected

- `NEXT_ACTIONS.md`
- `GOALS.md`
- `PROJECT_CHARTER.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `CHECKLIST_PROTOCOL_CHANGE.md`
- `tests/NA-0330_closeout_restore_na0331_testplan.md`
- `docs/governance/evidence/NA-0330_metadata_runtime_qshield_demo_batching_authorization.md`
- `tests/NA-0330_metadata_runtime_qshield_demo_batching_authorization_testplan.md`
- `docs/governance/evidence/NA-0329_metadata_runtime_qshield_demo_bounded_jitter_harness.md`
- `docs/governance/evidence/NA-0327_metadata_runtime_qshield_demo_retry_cadence_normalization_harness.md`
- `apps/qshield-cli/src/commands/send.rs`
- `apps/qshield-cli/src/commands/recv.rs`
- `apps/qshield-cli/src/commands/relay.rs`
- `apps/qshield-cli/src/relay_client.rs`
- qshield NA-0318, NA-0319, NA-0320, NA-0322, NA-0324, NA-0327, and NA-0329 harnesses

Search coverage included batching, batch window, batch size, ordering, partial
invalid, ack batch, retry cadence, bounded jitter, remote delete, accepted
state, output, artifact, secret, qshield demo, qsl-server, qsl-attachments,
production, timing-hidden, traffic-hidden, metadata-free, untraceable,
`FUTURE_GATE`, and `NOT_READY`.

## Implementation Summary or Blocker

No implementation blocker was found inside the allowed qshield surface.

Implemented:

- `relay_client.rs` adds `SendBatchRequest`, `SendBatchMember`, and
  `AckBatchRequest`.
- `relay.rs` adds env-gated `/send-batch` and `/ack-batch` endpoints.
- `send.rs` uses a one-member `/send-batch` request only when
  `QSHIELD_DEMO_BATCHING=1`.
- `recv.rs` caps candidate polling at four when batching is enabled and uses
  `/ack-batch` only after local verification succeeds for the candidate batch.
- `na_0331_metadata_runtime_batching.rs` proves the required markers.

## Batching Policy

- Policy name: `qshield_demo_batching_v1`.
- Opt-in: `QSHIELD_DEMO_BATCHING=1`.
- Deterministic test mode: `QSHIELD_DEMO_BATCHING_TEST_MODE=1`.
- Maximum send batch size: 4.
- Maximum receive candidate batch size: 4.
- Maximum ack batch size: 4 verified ack IDs.
- Maximum send-side batch window: 750 ms. The first implementation adds no
  real send wait or local staging, so the effective wait is 0 ms and remains
  inside the cap.
- Empty send and ack batches reject.
- Send and ack batches validate all members before mutation.

## Deterministic Test Mode

The NA-0331 harness starts the qshield relay with both batching env vars set.
The relay response records `test_mode: true`, and the implementation adds no
batch sleeps. Tests use deterministic request order and fixed max sizes, so CI
does not depend on timing races or wall-clock waits.

## Future / Actual Implementation Boundary

Actual NA-0331 behavior:

- qshield embedded relay/demo `/send-batch`;
- qshield embedded relay/demo `/ack-batch`;
- env-gated qshield one-member send through `/send-batch`;
- env-gated qshield receive candidate cap and verified-ID ack batching.

Not implemented:

- qsl-server production batching or timing behavior;
- qsl-attachments batching, upload/fetch timing, object timing, or retention behavior;
- cover traffic;
- transport padding expansion;
- broad queue scheduling;
- protocol wire, crypto, key schedule, qsc, or qsp behavior.

## Abuse / DoS / Latency / Compatibility Behavior

- Batch size is capped at four.
- The existing relay body, total queue, per-recipient queue, and per-token queue
  caps are checked before send-batch mutation.
- Send-batch invalid padding metadata, empty batch, over-size batch, queue-full,
  or quota-full conditions reject before enqueueing any member.
- Ack-batch empty, over-size, malformed, duplicate, stale, missing, or wrong
  recipient IDs reject before deleting any candidate.
- Receive batching does not add sleeps and does not add post-verification delay
  before ack/delete beyond one verified ack-batch request.
- Default single-message `/send` and `/ack` behavior is unchanged when batching
  is not enabled.

## Valid Single-Message Proof

Harness: `apps/qshield-cli/tests/na_0331_metadata_runtime_batching.rs`

The single-message test starts the relay without batching, sends one raw qshield
demo message through `/send`, verifies repeated `/poll-candidate` does not
delete it, a normal `/ack` deletes exactly that one candidate, and `/send-batch`
is unavailable while batching is disabled.

Marker:

- `NA0331_VALID_SINGLE_MESSAGE_UNCHANGED_OK`

## Valid Batch Proof

The valid batch test starts the relay with batching enabled, sends four members
through `/send-batch`, verifies response policy, maximum size, maximum window,
and deterministic test-mode metadata, polls four candidates, and ack-batches
the verified candidates in two groups.

Markers:

- `NA0331_BATCH_POLICY_OK`
- `NA0331_DETERMINISTIC_TEST_BATCHING_OK`
- `NA0331_VALID_BATCH_DELIVERY_OK`

## Ordering Proof

The valid batch test verifies candidate order exactly matches request order:
`aa`, `bb`, `cc`, `dd`. After acking the first two candidates, the remaining
queue preserves `cc`, `dd`.

Marker:

- `NA0331_BATCH_ORDERING_PRESERVED_OK`

## Invalid Batch Proof

The invalid send-batch test includes one valid member and one invalid padding
metadata member. The relay rejects the whole batch and the recipient queue
remains empty.

The invalid ack-batch test combines a valid ack ID with a stale ack ID, then a
duplicate ack ID. Both reject without deleting either queued candidate.

Marker:

- `NA0331_INVALID_BATCH_MEMBER_NO_REMOTE_DELETE_OK`

## Partial Invalid Batch Proof

Partial invalid send and ack batches reject all-or-nothing. The invalid receive
test queues an invalid first member plus a later candidate, runs batched
receive, and proves both candidates remain queued after every invalid attempt.

Marker:

- `NA0331_PARTIAL_INVALID_BATCH_FAIL_CLOSED_OK`

## Retry-Cadence and Jitter Preservation Proof

The invalid receive test runs batched receive under:

- `QSHIELD_DEMO_BATCHING=1`
- `QSHIELD_DEMO_BATCHING_TEST_MODE=1`
- `QSHIELD_DEMO_RETRY_CADENCE=1`
- `QSHIELD_DEMO_RETRY_CADENCE_TEST_MODE=1`
- `QSHIELD_DEMO_BOUNDED_JITTER=1`
- `QSHIELD_DEMO_BOUNDED_JITTER_TEST_MODE=1`

It verifies retry classes 0 ms, 500 ms, 1000 ms, and 2000 ms, verifies the
fifth attempt fails closed at the existing cap, verifies jitter is at or below
250 ms, and verifies composed retry plus jitter stays at or below 2250 ms.

Markers:

- `NA0331_RETRY_CADENCE_STILL_BOUNDED_OK`
- `NA0331_BOUNDED_JITTER_STILL_BOUNDED_OK`

## No Remote Delete Before Verify Proof

The invalid receive test records the first two candidate ack IDs before batched
receive and proves both remain queued after each invalid attempt and after the
cap failure. The invalid ack-batch test also proves stale and duplicate
ack-batches delete nothing.

Marker:

- `NA0331_INVALID_BATCH_MEMBER_NO_REMOTE_DELETE_OK`

## No Accepted State / Output Proof

The invalid receive test snapshots `state.json`, runs batched invalid receive
attempts, and confirms the state bytes are unchanged. It also scans combined
stdout/stderr for sender output and plaintext sentinels.

Markers:

- `NA0331_NO_ACCEPTED_STATE_ON_INVALID_BATCH_OK`
- `NA0331_NO_OUTPUT_ON_INVALID_BATCH_OK`

## Artifact / Log Safety Proof

The harness scans invalid receive output and the retry/jitter ledger for route
token sentinel, raw handle sentinel, candidate/ack sentinel, actual relay
token, actual ack IDs, plaintext sentinel, padding sentinel, passphrase
sentinel, raw key sentinel, panic/backtrace text, and sensitive local path
prefixes.

Result:

- `BATCH_ARTIFACT_SECRET_FINDING_COUNT 0`
- `NA0331_NO_SECRET_BATCH_ARTIFACT_OK`

## Harness Markers

The executable harness declares and emits:

- `NA0331_BATCHING_AUTHORIZATION_OK`
- `NA0331_BATCH_POLICY_OK`
- `NA0331_DETERMINISTIC_TEST_BATCHING_OK`
- `NA0331_VALID_SINGLE_MESSAGE_UNCHANGED_OK`
- `NA0331_VALID_BATCH_DELIVERY_OK`
- `NA0331_BATCH_ORDERING_PRESERVED_OK`
- `NA0331_INVALID_BATCH_MEMBER_NO_REMOTE_DELETE_OK`
- `NA0331_PARTIAL_INVALID_BATCH_FAIL_CLOSED_OK`
- `NA0331_RETRY_CADENCE_STILL_BOUNDED_OK`
- `NA0331_BOUNDED_JITTER_STILL_BOUNDED_OK`
- `NA0331_NO_ACCEPTED_STATE_ON_INVALID_BATCH_OK`
- `NA0331_NO_OUTPUT_ON_INVALID_BATCH_OK`
- `NA0331_NO_SECRET_BATCH_ARTIFACT_OK`
- `NA0331_QSHIELD_DEMO_BOUNDARY_OK`
- `NA0331_SERVICE_PRODUCTION_BOUNDARY_OK`
- `NA0331_NO_TIMING_HIDDEN_CLAIM_OK`
- `NA0331_NO_TRAFFIC_SHAPE_HIDDEN_CLAIM_OK`
- `NA0331_NO_METADATA_FREE_CLAIM_OK`
- `NA0331_METADATA_RUNTIME_BATCHING_OK`

No required NA-0331 marker is knowingly omitted.

## qshield Embedded Relay / Demo Boundary

This proof is limited to the qshield embedded relay/demo send, receive, relay,
and relay-client surfaces. The proof is local/demo evidence only.

Marker:

- `NA0331_QSHIELD_DEMO_BOUNDARY_OK`

## qsl-server / qsl-attachments Production Boundary

qsl-server production batching and production timing remain unproven and
cross-repo-gated. qsl-attachments batching, object timing, quota, retention,
and upload/fetch timing remain unproven and cross-repo-gated. No qsl-server or
qsl-attachments file is changed.

Marker:

- `NA0331_SERVICE_PRODUCTION_BOUNDARY_OK`

## Claim Boundaries

NA-0331 does not claim:

- timing metadata is hidden;
- traffic shape is hidden;
- metadata-free behavior;
- anonymity;
- untraceable behavior;
- production readiness;
- public-internet readiness;
- external-review completion;
- qshield local/demo proof is qsl-server or qsl-attachments production proof.

Markers:

- `NA0331_NO_TIMING_HIDDEN_CLAIM_OK`
- `NA0331_NO_TRAFFIC_SHAPE_HIDDEN_CLAIM_OK`
- `NA0331_NO_METADATA_FREE_CLAIM_OK`

## Selected Successor

Selected successor:

`NA-0332 -- Metadata Runtime Cover Traffic Risk Gate and Deferred Authorization Plan`

Rationale:

- NA-0331 completed the qshield demo batching implementation/harness without a
  blocker;
- retry cadence, bounded jitter, and bounded batching are now executable
  qshield demo-local controls;
- NA-0325 ranked cover traffic as the next higher-risk timing/traffic-shape
  lane after lower-cost qshield demo controls;
- cover traffic needs a risk gate and deferred authorization plan before any
  implementation attempt.

Rejected successors:

- qshield demo batching blocker resolution, because NA-0331 completed;
- qsl-server production batching, because production service timing remains
  cross-repo-gated;
- qsl-attachments timing, because attachment object behavior remains a
  separate cross-repo service concern;
- direct cover traffic implementation, because cost, abuse, storage, bandwidth,
  and claim-safety risks require a risk gate first.

## Backup-Plan Impact Statement

No backup-plan update is required. Tracked changes remain inside the
qsl-protocol worktree under `/srv/qbuild/work`, already covered by the qbuild
backup scope. Runtime test artifacts are temporary per-test directories under
system temp storage and are removed by the harness. No durable evidence
location outside current backup scope is introduced. The D132 preservation
bundle is not deleted or modified.

## Next Recommendation

Merge NA-0331 only after required local validation and required PR checks pass.
After merge and green post-merge public-safety, close NA-0331 and restore
exactly one successor:

`NA-0332 -- Metadata Runtime Cover Traffic Risk Gate and Deferred Authorization Plan`
