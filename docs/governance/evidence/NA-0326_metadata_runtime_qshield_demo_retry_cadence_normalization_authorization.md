Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-20

# NA-0326 Metadata Runtime qshield Demo Retry Cadence Normalization Authorization

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0326 authorizes a future bounded qshield embedded relay/demo retry-cadence
normalization implementation harness. The authorization is design-only. This
lane does not implement retry-cadence normalization, timing mitigation, jitter,
batching, cover traffic, queue scheduling, send scheduling, receive scheduling,
transport padding, service behavior, protocol behavior, crypto behavior, or
dependency changes.

The authorized future successor is:

`NA-0327 -- Metadata Runtime qshield Demo Retry Cadence Normalization Implementation Harness`

The future lane is safe to authorize because NA-0322 measured repeated invalid
receive retry cadence, NA-0324 instrumented invalid retry and queue cadence
classes, and NA-0325 selected retry cadence normalization as the narrowest
next qshield embedded relay/demo mitigation candidate. The future lane must
remain qshield embedded relay/demo-only unless a later directive expands scope.
It must not claim that timing metadata or traffic shape is hidden, and it must
not present local/demo proof as qsl-server or qsl-attachments production proof.

## Live NA-0326 Scope

The live queue item is `NA-0326 -- Metadata Runtime qshield Demo Retry Cadence
Normalization Authorization Plan`, status `READY`, with goals G1 through G5.

Allowed work:

- produce an authorization/design plan for qshield embedded relay/demo retry
  cadence normalization;
- define exact future invalid retry cadence semantics;
- define abuse, DoS, latency, compatibility, and reversibility bounds;
- define future implementation files and validation markers;
- decide whether NA-0327 can be a future implementation harness;
- preserve the qshield embedded relay/demo versus qsl-server/qsl-attachments
  production boundary;
- avoid privacy, production, external-review, and readiness overclaims.

Forbidden work:

- runtime retry-cadence normalization implementation;
- runtime timing mitigation, jitter, batching, cover traffic, queue scheduling,
  send scheduling, receive scheduling, or transport padding;
- qshield runtime implementation changes;
- qsl-server or qsl-attachments changes;
- qsc/qsp/protocol/crypto/key-schedule changes;
- dependency, Cargo, workflow, website, README, START_HERE, branch-protection,
  or public-safety configuration changes;
- claims that timing metadata or traffic shape is hidden;
- prohibited claims: anonymity, metadata-free, untraceable,
  production-readiness, public-internet-readiness, or
  external-review-complete.

## Inherited NA-0325 Option Matrix

NA-0325 ranked fixed intervals, bounded jitter, batching, cover traffic, queue
drain scheduling, retry cadence normalization, padding bucket expansion,
attachment-size class handling, local-demo mitigation, qsl-server production
mitigation, qsl-attachments production mitigation, and staged mitigation.

Retry cadence normalization ranked first for the next authorization lane
because it is narrower than fixed intervals, batching, cover traffic,
production service timing, and attachment-size class handling. It is grounded
directly in inherited invalid retry measurement and instrumentation evidence.
NA-0325 rejected direct cover traffic as the first implementation because of
cost, abuse, DoS, dummy-state, and external-review risk. It rejected direct
qsl-server or qsl-attachments production timing changes because those require
cross-repo service-owned evidence.

## Inherited NA-0324 Instrumentation Evidence

NA-0324 added a bounded qshield embedded relay/demo instrumentation harness.
The harness starts real qshield embedded relay processes, drives real relay JSON
requests, runs real invalid receive attempts, writes a bounded JSONL trace
artifact under temporary qbuild storage, validates the schema, and scans the
artifact for sensitive values observed during the run.

The inherited trace classes include send, candidate fetch, local verify,
ack/commit, invalid retry, output classification, queue cadence,
padding/size-class, ordering/correlation, retry-count, and boundary events.
The harness emits `TRACE_ARTIFACT_SECRET_FINDING_COUNT 0` and records the
artifact class as instrumentation measurement, not mitigation.

NA-0324 does not prove that timing metadata is hidden. It does not prove that
traffic shape is hidden. It does not implement retry normalization or any other
mitigation. It does not prove qsl-server or qsl-attachments production timing.

## Sources Inspected

- `NEXT_ACTIONS.md` live NA-0326 entry.
- `tests/NA-0325_closeout_restore_na0326_testplan.md`.
- `docs/governance/evidence/NA-0325_metadata_runtime_timing_traffic_mitigation_option_matrix.md`.
- `tests/NA-0325_metadata_runtime_timing_traffic_mitigation_option_matrix_testplan.md`.
- `docs/governance/evidence/NA-0324_metadata_runtime_timing_traffic_instrumentation_harness.md`.
- `docs/governance/evidence/NA-0323_metadata_runtime_timing_traffic_shape_instrumentation_mitigation_design.md`.
- `docs/governance/evidence/NA-0322_metadata_runtime_timing_traffic_measurement_harness.md`.
- `apps/qshield-cli/tests/na_0324_metadata_runtime_timing_traffic_instrumentation.rs`.
- `apps/qshield-cli/tests/na_0322_metadata_runtime_timing_traffic_measurement.rs`.
- `apps/qshield-cli/tests/na_0320_metadata_runtime_sanitized_retention.rs`.
- `apps/qshield-cli/tests/na_0319_metadata_runtime_identifier_padding.rs`.
- `apps/qshield-cli/tests/na_0318_qshield_ack_commit.rs`.
- `TRACEABILITY.md`.
- `DECISIONS.md`.

Search coverage included retry cadence, normalization, invalid retry, retry
interval, retry count, backoff, cadence, mitigation, authorization, qshield
demo, abuse, DoS, latency, compatibility, measurement, instrumentation,
service timing, qsl-server, qsl-attachments, production, hidden,
metadata-free, anonymity, untraceable, `FUTURE_GATE`, and `NOT_READY`.

## Retry-Cadence Semantic Design

The future implementation harness is authorized only for qshield embedded
relay/demo retry paths. It must use a named local/demo policy and must expose
the policy in deterministic tests without changing protocol, crypto, service,
or production semantics.

Future policy constants:

- policy name: `qshield_demo_retry_cadence_v1`;
- deterministic test mode: required for NA-0327;
- retry classes: `invalid_candidate`, `empty_poll`, `stale_ack`, and
  `duplicate_ack`;
- first valid message receive: no added normalization delay;
- first valid ack/commit after local verification: no added normalization
  delay;
- first invalid local verification attempt: may complete immediately but must
  enter the local demo retry ledger;
- repeated invalid candidate attempts inside a 60 second local window: apply
  deterministic bounded backoff of 500 ms, 1000 ms, then 2000 ms;
- maximum normalized delay: 2000 ms;
- maximum invalid candidate processing attempts for the same candidate class in
  the 60 second local window: 4, including the first attempt;
- attempts beyond the cap: fail closed with the same coarse invalid-retry class
  and no remote ack/delete, no accepted local state, and no user plaintext
  output;
- empty queue polling inside a 60 second local window: coarsen repeated empty
  polls with the same 500 ms, 1000 ms, 2000 ms capped sequence, then stay at
  the cap;
- stale ack retry: one request result only, deterministic fail-closed, no
  retry loop;
- duplicate valid ack: first ack succeeds after verification; any duplicate
  ack is stale and fail-closed;
- retry ledger scope: local qshield demo/test-only state keyed by coarse
  route/session/candidate class using secret-safe local identifiers; no raw
  route token, raw ack ID, plaintext, key material, or raw candidate body may
  be written to evidence artifacts;
- production state scope: none in qsl-server or qsl-attachments.

What a future implementation would change:

- add qshield embedded relay/demo retry policy state for selected retry paths;
- normalize repeated invalid candidate and repeated empty poll cadence within
  bounded deterministic test windows;
- add a qshield test harness proving the retry bounds and compatibility
  invariants;
- emit NA-0327 markers listed below.

What a future implementation must not change:

- valid message delivery semantics;
- ack/commit only after local verification;
- candidate retention before local verification succeeds;
- stale ack fail-closed behavior;
- duplicate ack behavior after the first valid ack;
- sanitized error output;
- local no-output/no-accepted-state behavior on invalid receive;
- qsl-server production behavior;
- qsl-attachments production behavior;
- qsc/qsp/protocol/crypto/key-schedule behavior;
- Cargo dependencies, workflows, public-safety, or branch protection.

## Future Implementation Boundary

Future allowed qshield files for NA-0327, if separately directed:

- `apps/qshield-cli/src/commands/recv.rs`
- `apps/qshield-cli/src/commands/relay.rs`
- `apps/qshield-cli/src/relay_client.rs`
- `apps/qshield-cli/tests/na_0327_metadata_runtime_retry_cadence_normalization.rs`
- existing qshield test helper code if needed and already local to qshield
  tests.

Future forbidden files:

- `qsl-server/**`
- `qsl-attachments/**`
- `qsc/**`
- `qsp/**`
- protocol, crypto, or key-schedule implementation paths;
- `Cargo.toml`
- `Cargo.lock`
- `.github/**`
- `website/**`
- `README.md`
- `START_HERE.md`
- `qsc-desktop/**`
- branch-protection or public-safety configuration.

Future proof requirements:

- valid message delivery still succeeds;
- invalid retry cadence is bounded;
- repeated invalid candidate fetches are normalized or bounded;
- stale ack retry is deterministic and fail-closed;
- no candidate is remotely deleted before local verification succeeds;
- invalid retry creates no accepted state and no plaintext output;
- retry artifacts and logs contain no secrets;
- no claim says timing metadata is hidden;
- no claim says traffic shape is hidden;
- qsl-server and qsl-attachments production boundaries remain explicit.

## Abuse/DoS/Latency/Compatibility Matrix

| Scenario | Risk | Proposed bound | Future test | Failure mode | Stop condition | Compatibility impact | Claim boundary |
| --- | --- | --- | --- | --- | --- | --- | --- |
| One invalid queued message | Single malformed candidate can expose a reject timing class. | First reject may be immediate; local ledger records the invalid candidate class. | Queue one invalid candidate and verify no ack/delete, no state, no output. | Candidate deleted, accepted, or leaked. | Any delete before verify or accepted state on invalid. | Preserves existing invalid-retain behavior. | Reject remains visible. |
| Repeated invalid queued message | Attacker can force repeated local verify work and cadence fingerprinting. | 500 ms, 1000 ms, 2000 ms capped backoff; 4 processing attempts per candidate class per 60 seconds. | Repeat invalid receive attempts and assert bounded normalized classes. | Unlimited work or exact failure-specific timing. | Backoff/cap absent or validation bypassed. | Adds bounded delay only on repeated invalid path. | Partial future mitigation only. |
| Repeated empty polling | Empty queue polling can reveal active user polling cadence and amplify load. | Same capped backoff sequence for repeated empty poll class. | Poll empty queue repeatedly and assert bounded cadence classes. | Busy loop or unbounded relay traffic. | Empty poll normalization missing or unbounded. | Adds bounded delay only on repeated empty result. | Empty queue state remains observable. |
| Stale ack replay | Replayed ack can create retry loops or delete wrong candidate. | One fail-closed stale result; no retry loop. | Replay a prior ack after successful delete. | Wrong deletion or loop. | Any state mutation or retry loop on stale ack. | Preserves NA-0318 stale ack fail-closed semantics. | Ack failure remains visible. |
| Duplicate valid ack | Duplicate ack after a valid commit can drift into idempotent success. | First valid ack succeeds once; duplicate is stale/fail-closed. | Ack once, repeat same ack, assert remaining queue unchanged. | Duplicate success or wrong delete. | Duplicate valid ack accepted. | Preserves one-delete semantics. | No production proof. |
| Slow valid receiver | Normalization could delay valid delivery. | No added delay on first valid candidate or verified ack. | Valid send/receive/ack path succeeds without retry delay. | Valid delivery waits behind retry policy. | Any first-valid delay not explicitly authorized. | Valid path stays compatible. | Timing still observable. |
| Multiple candidates | Invalid front candidate can starve valid later candidate. | Do not delete invalid before verify; cap repeated invalid work; leave future skip policy out of scope. | Queue invalid plus valid candidates; assert no invalid delete and no valid state corruption. | Delete, reorder, or accept wrong candidate. | Any candidate deletion/reordering beyond scope. | Preserves current front-candidate retention boundary. | Queue shape remains observable. |
| Attachment candidate flow | Attachment metadata can inherit retry cadence overclaims. | No qsl-attachments behavior in NA-0327; qshield demo attachment proof remains local. | qshield attachment demo smoke stays green, no service claim. | Attachment service timing claim from qshield proof. | Any qsl-attachments path change or production claim. | No service compatibility change. | qsl-attachments timing future-gated. |
| Local demo stress | Adversarial local loop can amplify work. | Bounded attempts, capped delays, no secret artifacts, stress smoke green. | Baseline demo adversarial stress plus NA-0327 retry harness. | Unbounded CPU/network or panic. | Stress failure with retry root cause. | Local demo remains bounded. | Not production readiness. |
| qsl-server equivalent | Production relay queues/logs/rate limits are different. | No qsl-server implementation; cross-repo lane required. | Future qsl-server authorization only. | qshield proof presented as production service proof. | Any qsl-server path change or production timing claim. | None in qsl-protocol. | Production timing unproven. |
| qsl-attachments equivalent | Object storage, quotas, retention, and upload/fetch timing are different. | No qsl-attachments implementation; cross-repo lane required. | Future qsl-attachments authorization only. | qshield proof presented as attachment service proof. | Any qsl-attachments path change or production timing claim. | None in qsl-protocol. | Attachment production timing unproven. |

## Future Validation/Marker Plan

NA-0327 marker candidates:

- `NA0327_RETRY_CADENCE_AUTHORIZATION_OK`
- `NA0327_RETRY_NORMALIZATION_POLICY_OK`
- `NA0327_INVALID_RETRY_BOUNDED_OK`
- `NA0327_EMPTY_POLL_RETRY_BOUNDED_OK`
- `NA0327_STALE_ACK_RETRY_FAIL_CLOSED_OK`
- `NA0327_VALID_ACK_ONCE_OK`
- `NA0327_NO_REMOTE_DELETE_BEFORE_VERIFY_OK`
- `NA0327_NO_ACCEPTED_STATE_ON_INVALID_RETRY_OK`
- `NA0327_NO_OUTPUT_ON_INVALID_RETRY_OK`
- `NA0327_NO_SECRET_RETRY_ARTIFACT_OK`
- `NA0327_QSHIELD_DEMO_BOUNDARY_OK`
- `NA0327_SERVICE_PRODUCTION_BOUNDARY_OK`
- `NA0327_NO_TIMING_HIDDEN_CLAIM_OK`
- `NA0327_NO_TRAFFIC_SHAPE_HIDDEN_CLAIM_OK`
- `NA0327_NO_METADATA_FREE_CLAIM_OK`

The future harness should also preserve inherited NA-0318 through NA-0324
markers or equivalent assertions for ack-after-verify, invalid retention,
sanitized errors, padding/identifier rejects, measurement boundaries, and
instrumentation-not-mitigation.

## Production Boundary

### qshield Embedded Relay/Demo

The future NA-0327 implementation harness may change only qshield embedded
relay/demo retry behavior and only under a separate exact directive. Local
qshield evidence can prove selected demo retry bounds, not production service
timing.

### qsl-server

qsl-server production retry-cadence normalization requires a separate cross-repo
lane. That lane must handle service queues, auth, rate/global caps, logs,
metrics, deployment topology, reverse proxies, backup artifacts, support
bundles, and public-internet exposure before any production timing statement
changes.

### qsl-attachments

qsl-attachments production retry, retention, upload/fetch timing, and object
size behavior require a separate cross-repo lane. That lane must handle
capabilities, opaque object storage, quotas, cleanup, recovery, support
bundles, backup artifacts, and service logs.

## External-Review Sensitivity

External review is recommended before stronger timing or traffic-shape claims,
production service mitigation, attachment-size class handling, broad queue
scheduling, cover traffic, or public-internet statements. NA-0326 does not
claim external review completion.

The selected qshield demo retry-cadence implementation harness has moderate
external-review sensitivity because it can remain local/demo-only and bounded,
but the claim boundary must remain conservative.

## Public Claim Boundary

NA-0326 permits only these claim-safe statements:

- the retry-cadence authorization plan exists;
- the plan is grounded in NA-0322 measurement, NA-0324 instrumentation, and
  NA-0325 option-matrix evidence;
- retry cadence normalization is authorized only for a future qshield
  embedded relay/demo implementation harness;
- the future implementation must prove bounds, no-delete-before-verify,
  no-accepted-state/no-output on invalid retry, and no secret artifacts;
- qsl-server and qsl-attachments production timing remain unproven and
  future-gated.

NA-0326 does not permit a claim of anonymity, metadata-free behavior,
untraceability, production readiness, public-internet readiness, external
review completion, hidden timing metadata, hidden traffic shape, or implemented
mitigation.

## Selected Successor

`NA-0327 -- Metadata Runtime qshield Demo Retry Cadence Normalization Implementation Harness`

Rationale:

- the inherited evidence identifies invalid retry cadence as a real measured
  qshield embedded relay/demo surface;
- the future semantics above are exact enough to test without changing service
  or protocol semantics;
- abuse and DoS bounds are explicit;
- latency impact is limited to repeated invalid and repeated empty-poll
  classes;
- compatibility invariants from NA-0318 through NA-0320 remain protected;
- the future implementation files can be bounded to qshield demo paths plus a
  qshield test harness.

## Rejected Alternatives

- Direct implementation in NA-0326: rejected because this lane is
  authorization/design only.
- Another detailed-design successor: rejected because the policy constants,
  file boundary, abuse matrix, and marker plan are precise enough for a future
  implementation harness.
- qsl-server production retry cadence as NA-0327: rejected because production
  service timing requires cross-repo authorization.
- qsl-attachments production retry/retention timing as NA-0327: rejected
  because attachment service timing requires cross-repo authorization.
- Cover traffic: rejected as first implementation because it requires cost,
  quota, dummy-state, abuse, and external-review gates.
- Claiming metadata-free behavior: rejected because current evidence proves
  observable timing and traffic-shape surfaces remain.

## Backup-Plan Impact Statement

No backup-plan update is required for NA-0326. Tracked evidence remains under
the qsl-protocol worktree inside `/srv/qbuild/work`, which is already in the
operator backup scope. NA-0326 creates no durable evidence location outside the
current backup scope and no non-rebuildable artifact outside the repo. The D132
preservation bundle and stash are not deleted or modified.

## Next Recommendation

Merge this authorization plan, keep NA-0326 READY until the closeout PR, then
restore exactly one successor:

`NA-0327 -- Metadata Runtime qshield Demo Retry Cadence Normalization Implementation Harness`

NA-0327 must not be implemented by NA-0326 closeout. The future NA-0327
directive should implement only the bounded qshield demo retry-cadence harness
or stop on an exact prerequisite.
