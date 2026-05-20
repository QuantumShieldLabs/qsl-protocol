Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-20

# NA-0325 Metadata Runtime Timing and Traffic-Shape Mitigation Option Matrix

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0325 ranks future metadata-runtime timing and traffic-shape mitigation
options after NA-0324 delivered bounded qshield embedded relay/demo
instrumentation evidence. This lane is analysis and governance evidence only.
It does not implement runtime mitigation, instrumentation, jitter, batching,
cover traffic, queue scheduling, send scheduling, receive scheduling, transport
padding, qsl-server behavior, qsl-attachments behavior, protocol changes,
crypto changes, or dependency changes.

The safest next authorization/design successor is:

`NA-0326 -- Metadata Runtime qshield Demo Retry Cadence Normalization Authorization Plan`

Rationale: retry cadence normalization is narrower than fixed intervals,
batching, cover traffic, production service timing, or attachment-size class
handling; it is directly grounded in NA-0322/NA-0324 invalid retry cadence
evidence; and it can be authorized first as a qshield embedded relay/demo-only
design lane without presenting local/demo evidence as production proof.

## Live NA-0325 Scope

The live queue item is `NA-0325 -- Metadata Runtime Timing and Traffic-Shape
Mitigation Option Matrix`, status `READY`, with goals G1 through G5.

Allowed work:

- build a mitigation option matrix grounded in NA-0324 instrumentation
  evidence;
- separate measurement findings from proposed future mitigation;
- analyze risk, cost, abuse surface, deployability, and claim boundaries;
- preserve qshield embedded relay/demo versus qsl-server/qsl-attachments
  production boundaries;
- select an exact NA-0326 authorization/design successor.

Forbidden work:

- runtime timing mitigation, jitter, batching, cover traffic, queue scheduling,
  send scheduling, receive scheduling, transport padding, service deployment,
  or instrumentation implementation;
- qshield runtime source changes;
- qsl-server or qsl-attachments changes;
- qsc/qsp/protocol/crypto/key-schedule changes;
- dependency, workflow, website, README, START_HERE, branch-protection, or
  public-safety configuration changes;
- unsupported claims that timing metadata or traffic shape is hidden.

## Inherited NA-0324 Instrumentation Evidence

NA-0324 added `apps/qshield-cli/tests/na_0324_metadata_runtime_timing_traffic_instrumentation.rs`.
The harness starts real qshield embedded relay processes, drives real relay
JSON requests, runs real invalid `qshield recv` attempts, records a bounded
JSONL trace artifact under `/srv/qbuild/tmp`, validates the trace schema, scans
for sensitive values observed during the run, and emits boundary markers.

The instrumentation records:

- send start/complete event classes;
- candidate fetch start/complete event classes;
- local verify start/complete event classes;
- ack/commit start/complete event classes;
- invalid retry start/complete event classes;
- output classification start/complete event classes;
- queue-depth classes;
- padding-size classes;
- retry-count classes;
- ordering/correlation classes;
- qshield embedded relay/demo boundary;
- `TRACE_ARTIFACT_SECRET_FINDING_COUNT 0`.

NA-0324 does not prove that timing metadata is hidden. It does not prove that
traffic shape is hidden. It does not implement mitigation. It does not prove
qsl-server or qsl-attachments production timing behavior.

## Inherited NA-0323 Mitigation Design Context

NA-0323 compared fixed intervals, bounded jitter, batching, cover traffic,
queue drain scheduling, retry cadence normalization, padding bucket expansion,
attachment-size class handling, local-demo-only mitigation, and production
service mitigation. It selected instrumentation first because NA-0322 proved
measurement feasibility but not enough event vocabulary for a runtime
authorization decision.

The NA-0323 recommendation carries forward here:

- improve evidence before mitigation;
- prefer local/demo-bounded authorization before service behavior changes;
- keep production service timing cross-repo and future-gated;
- require abuse/cost review before high-cost options such as cover traffic;
- never describe a proposed mitigation as current behavior.

## Sources Inspected

- `NEXT_ACTIONS.md` live NA-0325 entry.
- `tests/NA-0324_closeout_restore_na0325_testplan.md`.
- `docs/governance/evidence/NA-0324_metadata_runtime_timing_traffic_instrumentation_harness.md`.
- `tests/NA-0324_metadata_runtime_timing_traffic_instrumentation_harness_testplan.md`.
- `docs/governance/evidence/NA-0323_metadata_runtime_timing_traffic_shape_instrumentation_mitigation_design.md`.
- `docs/governance/evidence/NA-0322_metadata_runtime_timing_traffic_measurement_harness.md`.
- `docs/governance/evidence/NA-0321_metadata_runtime_timing_traffic_shape_threat_model.md`.
- `apps/qshield-cli/tests/na_0324_metadata_runtime_timing_traffic_instrumentation.rs`.
- `apps/qshield-cli/tests/na_0322_metadata_runtime_timing_traffic_measurement.rs`.
- `TRACEABILITY.md`.
- `DECISIONS.md`.

Search coverage included mitigation, option matrix, jitter, batching, cover
traffic, queue drain, retry cadence, padding bucket, attachment size,
scheduling, cadence, traffic shape, timing, measurement, instrumentation,
abuse, DoS, latency, bandwidth, cost, production, qsl-server, qsl-attachments,
external review, hidden, metadata-free, anonymity, untraceable, `FUTURE_GATE`,
and `NOT_READY`.

## Mitigation Option Inventory

| Option | What it addresses | What it does not address | Required future files | Runtime/service scope | Abuse/DoS risk | Latency impact | Bandwidth/storage impact | Correctness risk | CI/test burden | Backup impact | External-review sensitivity | Claim boundary | Recommendation |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Fixed interval sender cadence | Smooths explicit send-command bursts within a configured local profile. | Does not hide total volume, endpoints, queue state, size buckets, or long-term activity. | Future qshield demo authorization plan, runtime scheduling tests, evidence/testplan. | qshield demo first; production requires service lane. | Medium: adversary can queue pressure into fixed windows. | Medium to high; sends wait for slots. | Low unless idle sends are added. | Medium: ordering, deadlines, and fail-closed rejects must survive. | Moderate: deterministic slot-bound tests. | None expected if tracked files stay under repo. | Medium: production rollout needs review. | Future partial mitigation only; not current behavior. | Defer. |
| Fixed interval receiver polling cadence | Smooths explicit poll/fetch cadence and may reduce user-action timing linkage in a bounded profile. | Does not hide queue existence, ack timing, message volume, or production access logs. | Future qshield demo authorization plan and receive/poll tests. | qshield demo only first; service polling needs cross-repo scope. | Medium: polling can amplify load. | Medium; receive may wait for poll slot. | Low to medium if idle polling is enabled. | Medium: ack-after-verify and invalid no-delete must remain intact. | Moderate to high. | None expected. | Medium. | Poll cadence remains observable. | Defer. |
| Bounded jitter on send/receive | Reduces exact cadence fingerprinting for specific events. | Does not hide bursts, total volume, endpoint metadata, size classes, or long-term patterns. | Future qshield demo jitter authorization plan, bound tests, no-mutation tests. | qshield demo only first. | Medium: attackers can force delay queues. | Low to medium if bounds are small. | None directly. | Medium: randomness, retry bounds, and deterministic CI assertions are sensitive. | Moderate. | None expected. | Medium: needs careful review before production. | Jitter is partial and bounded; timing remains observable. | Future-gated after retry cadence. |
| Batching by count | Groups sends/receives until a count threshold. | Does not hide batch size, batch windows, total volume, or bucket distribution. | Future batching authorization plan, ordering/flush/ack tests. | qshield demo only first; production separately authorized. | Medium to high: queue buildup and forced flush pressure. | Medium to high; waits for threshold or fallback. | Low unless padding/cover is combined. | High: ordering, ack, and retention semantics can drift. | High. | None expected. | Medium to high. | Batch behavior is not broad traffic-shape hiding. | Defer. |
| Batching by time window | Groups events inside a time window. | Does not hide window boundaries, batch count, idle/active periods, or size classes. | Future batching authorization plan and time-window tests. | qshield demo only first. | Medium: adversary can hold windows open or force flushes. | Medium to high; waits for window. | Low unless padding/cover is combined. | High: deadlines, retries, and ack semantics are sensitive. | High. | None expected. | Medium to high. | Future bounded mitigation only. | Defer. |
| Queue drain scheduling | Smooths candidate fetch, ack/delete, and drain-order cadence. | Does not hide queue existence, route visibility, or total volume. | Future queue-drain authorization plan, fairness/starvation tests. | qshield demo only first; production relay requires cross-repo scope. | High: unfair scheduling can starve valid messages or favor abuse. | Medium. | None directly. | High: ack-after-verify, invalid no-delete, stale ack, and retention are sensitive. | High. | None expected. | High for production. | Queue shape remains observable. | Defer. |
| Retry cadence normalization | Coarsens timing differences across invalid retries and selected retry-after classes. | Does not hide failures, abuse volume, endpoint metadata, valid-message timing, or production service timing. | Future qshield demo retry-cadence authorization plan, invalid retry compatibility tests, evidence/testplan. | qshield embedded relay/demo first. | Medium: repeated invalid work can be forced if caps are weak. | Low to medium; only selected retry/reject paths should be delayed or normalized. | None directly. | Medium: validation must stay fail-closed and no retry hint may reveal exact state. | Moderate and directly grounded in NA-0322/NA-0324 invalid retry evidence. | None expected. | Medium. | Partial future mitigation; invalid rejects remain visible. | Recommend as NA-0326 authorization/design successor. |
| Padding bucket expansion | Reduces exact size leakage by coarsening bucket choices. | Does not hide timing, volume, endpoints, directionality, or bucket membership. | Future padding authorization plan, bucket config tests, artifact scans. | qshield demo first; production attachment buckets separately authorized. | Medium: bandwidth/storage amplification can be abused. | None to low. | Medium; more bytes per message/object. | Medium: invalid bucket/profile rejects must stay fail-closed. | Moderate. | None expected for qsl-protocol docs; production artifacts may require review. | Medium. | Padding is not traffic-shape hiding. | Future-gated. |
| Attachment-size class handling | Coarsens descriptor/object size classes for attachment flows. | Does not hide upload/fetch timing, capabilities, endpoint metadata, service logs, or object access patterns. | qsl-attachments cross-repo authorization plan, upload/fetch/retention/quota tests. | qsl-attachments production/service lane required. | High: storage, bandwidth, quota, and retention pressure. | Low to medium. | High. | High: capability, retention, recovery, and backup behavior are sensitive. | High across repos. | Could require backup review for durable service evidence. | High. | qsl-protocol local evidence is not attachment production proof. | Cross-repo required. |
| Local-demo-only mitigation | Lets future mitigations be tried under bounded qshield demo labels before service claims. | Does not prove production, public-internet, qsl-server, or qsl-attachments behavior. | Future qshield demo authorization plan, evidence/testplan, markers. | qshield embedded relay/demo only. | Medium if local results are overgeneralized. | Depends on selected mitigation. | Depends on selected mitigation. | Medium; must not change service contracts. | Moderate. | None expected. | Medium. | Local/demo-only evidence must stay local/demo-only. | Recommend as boundary for NA-0326. |
| qsl-server production relay timing mitigation | Addresses real relay timing, logs, queue, rate-limit, deployment, proxy, and public-internet surfaces. | Does not address qsl-attachments object timing or endpoint compromise. | qsl-server cross-repo authorization, service harness, deployment/log redaction plan. | qsl-server repo required. | High: operational abuse, cost, and rate-limit interactions. | Medium to high. | Medium if padding/cover is involved. | High: auth, retention, and rate limits are sensitive. | High. | May require backup-plan review if service artifacts move outside covered paths. | High. | Not proven or authorized by NA-0325. | Cross-repo required. |
| qsl-attachments production object timing/size mitigation | Addresses attachment upload/fetch/object-size timing and storage classes. | Does not address relay send/poll cadence or IP-level metadata. | qsl-attachments cross-repo authorization, object-size class harness, quota/retention tests. | qsl-attachments repo required. | High: storage/bandwidth amplification and object abuse. | Medium. | High. | High: capability and recovery semantics are sensitive. | High. | May require backup-plan review for durable service artifacts. | High. | Not qsl-protocol qshield proof. | Cross-repo required. |
| Cover traffic | Reduces idle-versus-active distinction if costed and shaped correctly. | Does not hide endpoints from network observers and can become fingerprintable. | Separate cover-traffic authorization, cost model, abuse controls, dummy-state tests, external review plan. | Runtime/service behavior; not qshield docs-only. | Very high: quota, storage, bandwidth, and abuse amplification. | Variable; may add scheduling delay. | Very high recurring cost. | Very high: dummy traffic must not mutate real state or receipts. | Very high. | Likely requires backup/cost review for durable artifacts. | Very high; external review recommended before implementation. | Cover traffic is not anonymity and is not authorized here. | Reject as first implementation; future-gated. |
| Hybrid staged mitigation: measurement -> instrumentation -> local-demo mitigation -> service authorization | Reduces sequencing risk by preserving evidence before runtime changes. | Does not itself reduce metadata until later authorized implementation. | Current NA-0325 evidence, future NA-0326 authorization, later implementation and service lanes. | qshield demo first, production later. | Low now; future risk depends on selected option. | None now. | None now. | Low now; future lanes isolate risk. | Low now; future lanes add tests. | None expected now. | Low now; high-risk options still gated. | Analysis/design only now. | Recommend as program sequence. |

## Risk / Cost / Abuse / Compatibility Matrix

| Option | Metadata benefit | Timing benefit | Traffic-shape benefit | Abuse risk | DoS risk | Latency cost | Bandwidth cost | Storage cost | Reliability risk | Correctness risk | Implementation scope | Testing scope | External review sensitivity | Production boundary | Claim boundary | Recommended successor relation |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Fixed interval sender cadence | Medium | Medium | Low to medium | Medium | Medium | Medium/high | Low | None | Medium | Medium | Runtime scheduling | Slot/bound/queue tests | Medium | qshield demo first; service later | Partial future mitigation only | Later authorization |
| Fixed interval receiver polling cadence | Medium | Medium | Low | Medium | Medium/high | Medium | Low/medium | None | Medium | Medium | Runtime polling | Poll/ack/no-delete tests | Medium | qshield demo first | Polls remain observable | Later authorization |
| Bounded jitter | Medium | Medium | Low | Medium | Medium | Low/medium | None | None | Medium | Medium | Runtime timing bounds | Range/no-mutation tests | Medium | qshield demo first | Timing remains observable | Future-gated after retry cadence |
| Batching by count | Medium | Medium | Medium | Medium/high | High | Medium/high | Low | Queue growth | High | High | Runtime queues | Flush/order/ack tests | Medium/high | qshield demo first | Batches remain observable | Defer |
| Batching by time window | Medium | Medium | Medium | Medium/high | High | Medium/high | Low | Queue growth | High | High | Runtime queues/timers | Window/order/ack tests | Medium/high | qshield demo first | Windows remain observable | Defer |
| Queue drain scheduling | Medium | Medium | Medium | High | High | Medium | None | Queue growth | High | High | Runtime receive/ack | Fairness/starvation/no-delete tests | High | qshield demo first; qsl-server later | Queue remains observable | Defer |
| Retry cadence normalization | Medium for invalid/retry paths | Medium for retry paths | Low | Medium | Medium | Low/medium | None | None | Medium | Medium | Runtime reject/retry cadence | Invalid retry, bounds, compatibility tests | Medium | qshield demo only for first successor | Rejects and failures remain visible | Select NA-0326 |
| Padding bucket expansion | Medium for size classes | None | Medium for size buckets | Medium | Medium | Low | Medium | Medium | Low/medium | Medium | Runtime padding config | Bucket/reject/artifact tests | Medium | qshield demo first; attachments later | Padding is partial only | Future-gated |
| Attachment-size class handling | Medium/high for object size | Low | Medium/high | High | High | Medium | High | High | High | High | qsl-attachments service | Upload/fetch/retention/quota tests | High | Cross-repo required | qsl-protocol does not prove service behavior | Cross-repo required |
| Local-demo-only mitigation | Depends on selected option | Depends on selected option | Depends on selected option | Medium | Medium | Bounded | Bounded | Bounded | Medium | Medium | qshield demo only | Marker and boundary tests | Medium | Not production proof | Local/demo only | Boundary for NA-0326 |
| qsl-server production relay mitigation | High for relay service | High for service timing | Medium/high | High | High | Medium/high | Medium/high | Medium | High | High | qsl-server repo | Service CI/deployment/log tests | High | Cross-repo required | Not authorized here | Cross-repo required |
| qsl-attachments production object mitigation | High for attachment size/timing | Medium | High for object classes | High | High | Medium | High | High | High | High | qsl-attachments repo | Service object/quota tests | High | Cross-repo required | Not authorized here | Cross-repo required |
| Cover traffic | Potentially high if carefully shaped | Medium/high | High | Very high | Very high | Variable | Very high | Very high | Very high | Very high | Runtime/service behavior | Cost/quota/dummy-state tests | Very high | Separate authorization required | Not anonymity; not authorized here | Reject as first lane |
| Hybrid staged mitigation | Governance benefit now | None now | None now | Low now | Low now | None now | None now | None now | Low now | Low now | Planning now, runtime later | Evidence and marker plans | Low now | Preserves boundaries | Analysis only now | Program sequence |

Cover traffic is high risk and high cost because it creates recurring traffic
that consumes bandwidth, queue slots, storage, service capacity, operator
attention, and abuse budget even when no real message exists. It also creates a
correctness hazard: dummy traffic must never mutate real queues, receipts,
message state, attachment state, or user-visible delivery truth. It should not
be the first implementation unless a separate authorization supplies a cost
model, quota policy, abuse controls, dummy-state invariants, and external-review
plan.

qsl-server and qsl-attachments production mitigation require cross-repo
authorization because their timing surfaces include service queues, access
logs, deployment topology, reverse proxies, object storage, retention, backup,
quota, and support-bundle behavior that qsl-protocol qshield local/demo tests
do not exercise.

qshield local/demo mitigation cannot be presented as production proof because
it runs inside bounded local test control and does not prove public-internet,
hosted service, proxy, mobile, desktop, qsl-server, or qsl-attachments timing.

Mitigation does not equal metadata-free behavior. Even a future mitigation may
only reduce selected observable correlations inside a stated profile. Endpoint
metadata, total volume, directionality, long-term patterns, service logs,
deployment metadata, and attachment/object behavior can remain visible.

## Candidate Prioritization

Ranking scale: 5 is best for safe first authorization; 1 is weakest.

| Candidate | Security value | Metadata-reduction value | Minimality | Testability | Abuse resistance | Reversibility | Compatibility | CI cost | Claim safety | External-review readiness | Total | Ranking |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| Retry cadence normalization | 4 | 3 | 4 | 4 | 3 | 4 | 4 | 4 | 4 | 3 | 37 | 1 |
| Padding bucket expansion | 3 | 3 | 4 | 4 | 3 | 4 | 3 | 3 | 3 | 3 | 33 | 2 |
| Bounded jitter | 4 | 3 | 3 | 3 | 3 | 4 | 3 | 3 | 3 | 3 | 32 | 3 |
| Fixed interval sender cadence | 3 | 3 | 3 | 3 | 3 | 3 | 3 | 3 | 3 | 3 | 30 | 4 |
| Fixed interval receiver polling cadence | 3 | 3 | 3 | 3 | 2 | 3 | 3 | 3 | 3 | 3 | 29 | 5 |
| Queue drain scheduling | 4 | 3 | 2 | 3 | 2 | 3 | 2 | 2 | 3 | 2 | 26 | 6 |
| Batching by time window | 3 | 3 | 2 | 3 | 2 | 3 | 2 | 2 | 3 | 2 | 25 | 7 |
| Batching by count | 3 | 3 | 2 | 3 | 2 | 3 | 2 | 2 | 3 | 2 | 25 | 7 |
| qsl-server production mitigation | 4 | 4 | 1 | 2 | 1 | 2 | 2 | 1 | 2 | 1 | 20 | Cross-repo |
| qsl-attachments production mitigation | 4 | 4 | 1 | 2 | 1 | 2 | 2 | 1 | 2 | 1 | 20 | Cross-repo |
| Attachment-size class handling | 4 | 4 | 1 | 2 | 1 | 2 | 2 | 1 | 2 | 1 | 20 | Cross-repo |
| Cover traffic | 4 | 4 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 1 | 16 | Reject as first lane |

Retry cadence normalization ranks first because it is the narrowest future
authorization candidate grounded in the inherited invalid retry evidence. It
targets a known measured surface, can stay qshield demo-only, can preserve
fail-closed validation, and has clearer test markers than broad jitter,
batching, queue scheduling, service timing, or cover traffic.

Bounded jitter and padding bucket expansion remain plausible future candidates,
but both should follow an authorization plan that first proves bounds, abuse
controls, and claim boundaries. Fixed intervals, batching, and queue drain
scheduling have larger ordering, latency, and queue-pressure risks. Production
service timing and attachment-size class handling are cross-repo lanes. Cover
traffic is rejected as an immediate successor because it has the highest cost
and claim-risk profile.

## Future Validation / Marker Plan

Shared NA-0326 markers:

- `NA0326_MITIGATION_OPTION_MATRIX_OK`
- `NA0326_SELECTED_MITIGATION_SCOPE_OK`
- `NA0326_MEASUREMENT_BEFORE_MITIGATION_OK`
- `NA0326_QSHIELD_DEMO_BOUNDARY_OK`
- `NA0326_SERVICE_PRODUCTION_BOUNDARY_OK`
- `NA0326_NO_TIMING_HIDDEN_CLAIM_OK`
- `NA0326_NO_TRAFFIC_SHAPE_HIDDEN_CLAIM_OK`
- `NA0326_NO_METADATA_FREE_CLAIM_OK`

Selected retry-cadence successor markers:

- `NA0326_RETRY_CADENCE_AUTHORIZATION_OK`
- `NA0326_RETRY_ABUSE_BOUNDARY_OK`
- `NA0326_INVALID_RETRY_COMPATIBILITY_OK`
- `NA0326_RETRY_LATENCY_BOUNDS_OK`
- `NA0326_RETRY_FAIL_CLOSED_VALIDATION_OK`
- `NA0326_RETRY_NO_STATE_WEAKENING_OK`

Deferred/high-risk marker candidates:

- bounded jitter: `NA0326_BOUNDED_JITTER_AUTHORIZATION_OK`,
  `NA0326_JITTER_ABUSE_RISK_REVIEW_OK`,
  `NA0326_JITTER_LATENCY_BOUNDS_OK`;
- batching: `NA0326_BATCHING_AUTHORIZATION_OK`,
  `NA0326_BATCHING_ORDERING_RISK_OK`,
  `NA0326_BATCHING_LATENCY_BOUNDS_OK`;
- service cross-repo: `NA0326_SERVICE_TIMING_CROSS_REPO_AUTHORIZATION_OK`;
- cover traffic: `NA0326_COVER_TRAFFIC_DEFERRED_GATE_OK`,
  `NA0326_COVER_TRAFFIC_COST_MODEL_REQUIRED_OK`.

## Production Boundary

### qshield Embedded Relay/Demo

qshield embedded relay/demo evidence is local/demo proof only. A future
NA-0326 authorization may design retry cadence normalization for qshield
embedded relay/demo paths, but that would still not prove production service
timing.

### qsl-server

qsl-server production relay timing requires a cross-repo lane. That lane must
handle service queues, route-token/header behavior, rate/global caps,
deployment topology, reverse proxies, access logs, metrics, backup artifacts,
and public-internet exposure before any production timing statement changes.

### qsl-attachments

qsl-attachments production upload/fetch timing and object-size class behavior
require a cross-repo lane. That lane must handle capabilities, opaque object
storage, quotas, retention, recovery, support bundles, backup artifacts, and
service logs.

## External-Review Sensitivity

External review is recommended before high-cost or high-risk mitigation such as
cover traffic, production service timing, attachment-size class handling,
global padding default changes, or broad queue scheduling. External review is
not complete for these areas. NA-0325 does not claim any external review
completion.

The selected retry cadence authorization plan has moderate external-review
sensitivity because it can remain demo-local and design-only. It still needs
clear abuse controls, latency bounds, and fail-closed compatibility proof
before any implementation lane.

## Public Claim Boundary

NA-0325 permits only these claim-safe statements:

- the option matrix exists;
- the matrix is grounded in NA-0321 threat model, NA-0322 measurement,
  NA-0323 design, and NA-0324 instrumentation evidence;
- selected mitigation options are future candidates;
- retry cadence normalization is the safest next authorization/design
  successor;
- qshield embedded relay/demo evidence remains separate from qsl-server and
  qsl-attachments production behavior;
- qsl-server and qsl-attachments production timing remain unproven;
- timing metadata and traffic shape remain observable unless exact future
  evidence proves a narrower reduction.

NA-0325 does not permit a claim of anonymity, metadata-free behavior,
untraceability, production readiness, public-internet readiness, external
review completion, hidden timing metadata, hidden traffic shape, or implemented
mitigation.

## Selected Successor

`NA-0326 -- Metadata Runtime qshield Demo Retry Cadence Normalization Authorization Plan`

The successor should:

- stay design/authorization first;
- stay qshield embedded relay/demo-only unless a later directive expands scope;
- define exactly which invalid retry and retry-after classes may be normalized;
- define latency bounds and maximum retry work;
- preserve fail-closed validation and no-state/no-delete invariants;
- preserve artifact no-secret requirements;
- reject any production or public-claim expansion without cross-repo evidence;
- select a later implementation lane only if authorization evidence is green.

## Rejected Alternatives

- Direct cover traffic implementation: rejected because cost, abuse, DoS,
  dummy-state, and external-review requirements are too high for a first
  mitigation lane.
- Direct qsl-server/qsl-attachments production mitigation: rejected because
  production service timing requires cross-repo authorization and service-owned
  evidence.
- Direct bounded jitter implementation: deferred because retry cadence has a
  narrower evidence base and simpler compatibility boundary.
- Direct batching implementation: deferred because ordering, flush, ack, and
  latency semantics need design first.
- Claiming metadata-free behavior: rejected because current evidence proves
  observable timing and traffic-shape surfaces remain.

## Backup-Plan Impact Statement

No backup-plan update is required for NA-0325. Tracked evidence remains under
the qsl-protocol worktree inside `/srv/qbuild/work`, which is already in the
operator backup scope. NA-0325 creates no durable evidence location outside the
current backup scope and no non-rebuildable artifact outside the repo.

## Next Recommendation

Close NA-0325 after this matrix merges and post-merge public-safety is green,
then restore exactly one READY successor:

`NA-0326 -- Metadata Runtime qshield Demo Retry Cadence Normalization Authorization Plan`

NA-0326 should authorize only the retry-cadence normalization design boundary
and must not implement runtime mitigation unless a later exact directive
separately authorizes implementation.
