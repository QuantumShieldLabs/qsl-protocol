Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-20

# NA-0323 Metadata Runtime Timing and Traffic-Shape Instrumentation Mitigation Design

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0323 turns the NA-0321 threat model and NA-0322 qshield embedded
relay/demo measurement evidence into a design plan for future instrumentation
and mitigation work. This lane is design only. It does not implement runtime
instrumentation, timing jitter, batching, cover traffic, send scheduling,
receive scheduling, queue scheduling, transport padding, service deployment
behavior, qshield runtime changes, qsl-server changes, qsl-attachments changes,
qsc/qsp/protocol/crypto changes, or dependency changes.

The immediate successor should be:

`NA-0324 -- Metadata Runtime Timing and Traffic-Shape Instrumentation Harness`

Rationale: NA-0322 proved that bounded qshield embedded relay/demo timing and
traffic-shape measurement is feasible from tests without user-facing runtime
changes. The next safest step is to improve trace vocabulary, artifact schema,
and event coverage before authorizing any mitigation implementation.

## Live NA-0323 Scope

The live queue item is `NA-0323 -- Metadata Runtime Timing and Traffic-Shape
Instrumentation / Mitigation Design Plan`, status `READY`, with goals G1
through G5.

Allowed work:

- plan the next metadata-runtime timing and traffic-shape lane selected by
  NA-0322;
- design instrumentation and mitigation strategy from bounded qshield embedded
  relay/demo measurement evidence;
- carry forward the NA-0321 threat model and NA-0322 measurement boundaries;
- separate qshield embedded relay/demo options from qsl-server and
  qsl-attachments production options;
- define a later validation plan and select an exact NA-0324 successor.

Forbidden work:

- runtime timing, jitter, batching, cover traffic, send scheduling, receive
  scheduling, queue scheduling, transport padding, or service deployment
  implementation;
- qshield runtime implementation changes;
- qsl-server or qsl-attachments implementation changes;
- qsc/qsp/protocol/crypto/key-schedule implementation changes;
- dependency, workflow, website, README, START_HERE, branch-protection, or
  public-safety configuration changes;
- claims of hidden timing metadata, hidden traffic shape, anonymity,
  metadata-free behavior, untraceability, production readiness, public internet
  readiness, or external review completion.

Runtime implementation is not authorized by this lane.

## Inherited NA-0321 Threat Model

NA-0321 established that timing metadata and traffic shape remain observable
for the current metadata-runtime surfaces. The threat model includes passive
local observers, passive network observers, relay observers, malicious queued
message injectors in the bounded demo context, log/artifact observers, public
claim observers, and future production service observers.

Key inherited risks:

- sender cadence can reveal explicit command timing and bursts;
- receiver cadence can reveal candidate fetch, ack, and invalid retry timing;
- queue depth and front-of-queue behavior can reveal queue shape and ordering;
- padding buckets reduce exact plaintext-size exposure but do not hide traffic
  shape;
- retry and rate-limit classes can reveal abuse and load behavior;
- qsl-server and qsl-attachments production timing are not proven by qshield
  embedded relay/demo evidence;
- deployment, proxy, CDN, mobile, desktop, public internet, log, and support
  bundle timing remain future gates.

## Inherited NA-0322 Measurement Evidence

NA-0322 added `apps/qshield-cli/tests/na_0322_metadata_runtime_timing_traffic_measurement.rs`.
The harness starts real qshield embedded relay processes, drives real relay JSON
requests, runs real invalid `qshield recv` commands, captures monotonic elapsed
timing around selected operations, writes bounded JSONL measurement artifacts,
and scans artifacts for configured secret/sentinel values.

NA-0322 measured or classified:

- explicit sender cadence around relay `/send`;
- candidate fetch and invalid receive timing;
- queue-depth observations before and after send/fetch/ack;
- valid ack/commit delete timing;
- repeated invalid receive cadence and retention;
- padding bucket classes for bounded samples;
- front-of-queue and ack-order visibility;
- qshield embedded relay/demo boundary;
- qsl-server and qsl-attachments production timing as not proven.

What NA-0322 proves:

- bounded qshield embedded relay/demo measurement can be performed without
  qshield runtime source changes;
- the artifact schema can record coarse timing and traffic-shape classes while
  avoiding raw route tokens, raw ack IDs, raw candidates, plaintext, raw local
  paths, and panic/backtrace text;
- the current qshield embedded relay/demo surfaces expose observable cadence,
  queue shape, ordering, and size bucket classes.

What NA-0322 does not prove:

- timing metadata is hidden;
- traffic shape is hidden;
- padding alone hides traffic shape;
- mitigation exists;
- qsl-server production timing is proven;
- qsl-attachments production upload/fetch timing is proven;
- public internet or deployment timing is proven;
- contact graph, IP-level, route/session, or ordering-correlation resistance
  is proven.

## Sources Inspected

- `NEXT_ACTIONS.md` live NA-0323 entry.
- `tests/NA-0322_closeout_restore_na0323_testplan.md`.
- `docs/governance/evidence/NA-0322_metadata_runtime_timing_traffic_measurement_harness.md`.
- `tests/NA-0322_metadata_runtime_timing_traffic_measurement_harness_testplan.md`.
- `docs/governance/evidence/NA-0321_metadata_runtime_timing_traffic_shape_threat_model.md`.
- `apps/qshield-cli/tests/na_0322_metadata_runtime_timing_traffic_measurement.rs`.
- `apps/qshield-cli/tests/na_0320_metadata_runtime_sanitized_retention.rs`.
- `apps/qshield-cli/tests/na_0319_metadata_runtime_identifier_padding.rs`.
- `apps/qshield-cli/tests/na_0318_qshield_ack_commit.rs`.
- `apps/qshield-cli/src/commands/relay.rs`.
- `apps/qshield-cli/src/commands/recv.rs`.
- `apps/qshield-cli/src/commands/send.rs`.
- `scripts/ci/demo_adversarial_stress.sh`.
- `scripts/ci/demo_soak_repeated_run.sh`.
- `docs/governance/evidence/**` targeted timing/traffic-shape search.
- `TRACEABILITY.md`.
- `DECISIONS.md`.

## Instrumentation Design Options

| Option | Future files | Pros | Risks | Artifact safety | CI cost | Backup impact | Claim boundary | Recommendation |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| qshield demo timing instrumentation | `apps/qshield-cli/tests/na_0324_metadata_runtime_timing_traffic_instrumentation.rs`; future evidence/testplan | Extends NA-0322 with richer event labels around send, candidate fetch, ack, invalid retry, and output using monotonic relative timings. | Can overfit local loopback timing or imply mitigation if wording drifts. | Record event class, relative monotonic timing, duration class, and result class only; redact raw tokens, ack IDs, candidates, plaintext, and paths. | Moderate; test-only Rust harness can stay bounded. | None for tracked files under qsl-protocol; runtime artifacts remain temporary. | qshield embedded relay/demo measurement only; not production proof. | Recommended as NA-0324 core. |
| qshield queue cadence instrumentation | Same future qshield test plus optional future trace schema doc | Records queue depth class, candidate state transitions, fetch/no-delete, ack/delete, and drain order. | Exact queue depths may reveal sensitive load if copied into durable artifacts. | Use coarse queue depth class and transition class; avoid raw queue internals. | Moderate under bounded sample counts. | None expected. | Queue observability evidence, not queue-shape hiding. | Recommended. |
| padding/size instrumentation | Same future qshield test and future schema text | Records bucket class, padded length class, overhead class, and observable size class without raw plaintext size. | Bad schema could record sensitive exact lengths. | Store only configured bucket class and coarse length class; avoid raw plaintext and sensitive exact length. | Low to moderate. | None expected. | Padding evidence is not a traffic-shape hiding claim. | Recommended. |
| invalid retry instrumentation | Same future qshield test | Records retry count class, repeated reject cadence, same-output class, and retention state. | Can create expensive or abusive loops if unbounded. | Hard cap attempts, durations, output capture, and artifact bytes; record output class only. | Low if attempts remain small. | None expected. | Invalid retry measurement only; no retry normalization implemented. | Recommended. |
| script-based instrumentation | Future `scripts/ci/metadata_runtime_timing_traffic_instrumentation_harness.sh` only if separately authorized | Can wrap existing commands without Rust code and reuse demo smoke/stress/soak artifacts. | CI shell dependency constraints are strict; shell timing can be noisy; script path is not authorized in NA-0323. | Use POSIX shell plus coreutils/grep/awk/sed only in `scripts/ci`; redact paths and secrets. | Variable; can be cheap if sampled. | None for tracked repo files; temp artifacts under qbuild tmp. | Script evidence remains local/demo measurement. | Defer until a future lane explicitly authorizes scripts. |
| test-only Rust instrumentation | `apps/qshield-cli/tests/na_0324_metadata_runtime_timing_traffic_instrumentation.rs` | Uses `Instant`, direct assertions, typed JSON records, and existing qshield test patterns; no user-facing runtime changes. | Test harness must not become a de facto runtime policy. | Enforce artifact schema and forbidden-value scan in the test. | Moderate but bounded. | None expected. | Test-only proof; not runtime instrumentation. | Best immediate path. |
| service instrumentation | Future qsl-server/qsl-attachments cross-repo lanes | Needed for production relay and attachment timing truth. | Cross-repo scope, service logs, deployment data, support bundles, and privacy review are higher risk. | Requires separate artifact taxonomy and production-log redaction policy. | Higher; may require separate CI/service harnesses. | Could require backup review if durable service artifacts move outside current scope. | Not proven in qsl-protocol NA-0323. | Reject for immediate NA-0324; require cross-repo authorization later. |

Instrumentation requirements for the selected successor:

- capture relative monotonic timings only;
- use stable event names and schema versions;
- record classed queue depth, bucket, retry, and result values;
- keep artifacts bounded and secret-safe;
- scan artifacts for tokens, raw handles, ack IDs, plaintext, sentinels,
  panic/backtrace text, and long secret-like dumps;
- emit markers only for measured surfaces;
- state that instrumentation measures observable metadata and is not
  mitigation.

## Mitigation Design Options

| Option | Threat addressed | What it does not address | Complexity | Abuse/DoS risk | Latency/cost impact | Correctness risk | Metadata claim risk | Test markers | Stop conditions | External review |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Fixed-interval send/receive scheduling | Reduces direct event-to-send or poll timing correlation within a bounded profile. | Does not hide IP metadata, total volume, queue depth over time, or size classes. | Medium for demo; high for production services. | Queue growth and delayed abuse response. | Higher latency; possible idle work. | Ack/commit and retry ordering must remain fail-closed. | High if described as hiding all timing. | Future `NA0324_FIXED_INTERVAL_DESIGN_BOUNDARY_OK`. | Stop if queue bounds, retries, or user deadlines are undefined. | Recommended before runtime rollout. |
| Bounded jitter | Reduces exact cadence fingerprinting. | Does not hide bursts, volume, endpoint metadata, or long-term patterns. | Medium. | Attackers can exploit jitter queues to amplify delay. | Adds bounded latency and test nondeterminism. | Must preserve deterministic bounds and fail-closed state transitions. | High if called timing hiding. | Future `NA0324_JITTER_DESIGN_BOUNDARY_OK`. | Stop if randomness source, bounds, and replay interactions are unclear. | Recommended. |
| Batching | Reduces per-message timing linkage by grouping sends or receives. | Does not hide batch count, batch window, total volume, or size bucket distribution. | Medium to high. | Queue buildup, delayed delivery, and batch flushing abuse. | Latency increases by batch window. | Message ordering, ack/commit, and retention must stay correct. | Medium to high. | Future `NA0324_BATCHING_DESIGN_BOUNDARY_OK`. | Stop if ordering and ack semantics are not fully specified. | Recommended. |
| Cover traffic | Obscures idle versus active periods in a configured profile. | Does not hide endpoints from the network and can be distinguishable if poorly shaped. | High. | Very high bandwidth, storage, quota, and abuse cost. | High recurring cost. | Must avoid fake state that mutates real queues or receipts. | Very high; easy to overclaim. | Future `NA0324_COVER_TRAFFIC_DESIGN_BOUNDARY_OK`. | Stop without cost model, abuse controls, and external review. | Required before implementation. |
| Queue drain scheduling | Smooths receive/ack/delete cadence and front-of-queue drain shape. | Does not hide queue existence or total traffic volume. | Medium. | Queue starvation or prioritized abuse if unfair. | Adds receive latency. | Ack-after-verify and retention invariants are sensitive. | Medium. | Future `NA0324_QUEUE_DRAIN_DESIGN_BOUNDARY_OK`. | Stop if valid ack/delete or invalid no-delete behavior changes. | Recommended. |
| Retry cadence normalization | Reduces distinguishability between invalid retry families and rate/capacity retries. | Does not hide existence of failures or overall abuse volume. | Medium. | Attackers can force repeated normalized work. | Adds delay to some rejects. | Must not weaken fail-closed rejects or leak state through retry hints. | Medium. | Future `NA0324_RETRY_CADENCE_DESIGN_BOUNDARY_OK`. | Stop if validation is bypassed or retry hints reveal exact state. | Recommended. |
| Padding bucket expansion | Reduces exact size class leakage for messages or attachments. | Does not hide timing, volume, directionality, or bucket membership. | Low to medium for demo; higher for attachments. | Storage and bandwidth amplification. | Higher bytes per message/object. | Bucket config must reject invalid profiles fail-closed. | Medium if padding is overclaimed. | Future `NA0324_PADDING_BUCKET_EXPANSION_BOUNDARY_OK`. | Stop if exact plaintext length or attachment size leaks into artifacts. | Review recommended for production defaults. |
| Attachment-size class handling | Reduces descriptor/object size correlation across attachment flows. | Does not hide upload/fetch timing, capability use, endpoint metadata, or access logs. | High across qsl-attachments. | High storage/bandwidth overhead and object-retention pressure. | Potentially high. | Capability, retention, recovery, and partial restore behavior are sensitive. | High. | Future `NA0324_ATTACHMENT_SIZE_CLASS_BOUNDARY_OK`. | Stop without qsl-attachments authorization. | Required. |
| Local-demo-only mitigation | Allows safe experiments with visible boundaries. | Does not prove production or public internet behavior. | Medium. | Can drift into public claims if not labeled. | Bounded. | Must not change production-facing contracts. | High if demo is marketed as production. | Future `NA0324_LOCAL_DEMO_MITIGATION_BOUNDARY_OK`. | Stop if demo mitigation is described as production proof. | Recommended before public claims. |
| qsl-server/qsl-attachments production mitigation | Addresses service timing, logs, deployment, queue, attachment, and public internet surfaces. | Does not address endpoint compromise or all network metadata. | Very high. | High operational and cost risk. | High. | Service auth, retention, backup, and abuse controls are sensitive. | Very high. | Future service-lane markers only. | Stop without cross-repo authorization and external review plan. | Required. |

Mitigation recommendation: do not implement mitigation in NA-0323 or in the
immediate closeout. First run an instrumentation harness lane that improves
evidence quality. After that, choose a mitigation option matrix or a narrowly
authorized demo-only mitigation prototype if the evidence supports it.

## Risk Abuse Cost Compatibility Matrix

| Option | Security benefit | Metadata benefit | Reliability risk | Cost | Implementation scope | Test scope | Claim boundary | Recommendation |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Test-only qshield instrumentation | Improves evidence and catches drift. | Shows what remains observable. | Low. | Low to moderate CI time. | Future qshield test only. | Artifact schema, no-secret scan, markers. | Measurement, not mitigation. | Do next. |
| Script-based command instrumentation | Broader command workload coverage. | Adds smoke/stress/soak timing samples. | Medium shell fragility. | Low to moderate. | Future script only if authorized. | POSIX shell artifact checks. | Local/demo only. | Defer. |
| Fixed interval | Reduces direct timing linkage. | Smooths cadence within bounds. | Queue growth and delivery delay. | Moderate. | Runtime scheduling. | Deterministic bound and queue tests. | Does not hide all timing. | Design after instrumentation. |
| Bounded jitter | Reduces precise fingerprints. | Adds timing uncertainty. | Nondeterminism and delay. | Low to moderate. | Runtime scheduling. | Bound/range/no-mutation tests. | Jitter is partial mitigation. | Design after instrumentation. |
| Batching | Reduces per-message correlation. | Groups observable events. | Ordering and ack risk. | Latency. | Runtime send/receive queues. | Batch flush, ordering, ack tests. | Batch windows remain observable. | Needs detailed design. |
| Cover traffic | Reduces idle/active distinction. | Can obscure traffic presence if shaped well. | High abuse and capacity risk. | High ongoing cost. | Runtime/service behavior. | Cost, quota, dummy-state, no-secret tests. | Not anonymity. | Not immediate. |
| Queue drain scheduling | Smooths receive/ack cadence. | Reduces queue-shape spikes. | Starvation or stale candidates. | Latency. | Runtime receive/ack behavior. | Ack-after-verify and no-delete tests. | Queue still observable. | Needs design. |
| Retry normalization | Reduces reject-family timing differences. | Coarsens retry signals. | Abuse loops. | Delay on rejects. | Runtime reject paths. | Invalid/retry bounded tests. | Fail-closed rejects remain. | Needs design. |
| Padding expansion | Reduces exact size leakage. | Coarser size buckets. | Overhead and config rejects. | Bandwidth/storage. | Runtime padding config. | Bucket and reject tests. | Does not hide traffic shape. | Needs evidence. |
| Attachment-size class handling | Reduces object-size linkage. | Coarser attachment classes. | Service complexity. | High storage/bandwidth. | qsl-attachments cross-repo. | Upload/fetch/retention tests. | Not qsl-protocol proof. | Cross-repo later. |

## Production Boundary Analysis

### qshield Embedded Relay/Demo

qshield embedded relay/demo instrumentation can produce local proof for
specific send, candidate fetch, ack, invalid receive, queue, retry, and padding
surfaces. It can safely drive the same event vocabulary as NA-0322 with richer
artifact classes. It remains local loopback/demo evidence.

### qsl-server

qsl-server production relay timing is not proven by NA-0321, NA-0322, or
NA-0323. Production relay timing, queue behavior, access logs, proxy logs,
deployment timing, rate limiting, support bundles, and public internet behavior
require a separate cross-repo authorization lane.

### qsl-attachments

qsl-attachments production upload/fetch timing, descriptor/object size
correlation, capability use timing, retention side effects, backup artifacts,
and service logs are not proven here. Attachment-size class handling and
attachment traffic-shape mitigation require separate qsl-attachments scope.

### Public Internet, Deployment, Desktop, and Mobile

Public internet timing, CDN/proxy behavior, firewall/ACL posture, mobile
backgrounding, desktop shell timing, external website behavior, and production
operator bundles remain out of scope or future-gated. No production readiness
or public internet readiness is claimed.

## Future Marker Plan

Instrumentation markers for NA-0324:

- `NA0324_TIMING_INSTRUMENTATION_PLAN_OK`
- `NA0324_QSHIELD_DEMO_TRACE_ARTIFACT_SCHEMA_OK`
- `NA0324_QSHIELD_EVENT_TIMING_CAPTURE_OK`
- `NA0324_QUEUE_CADENCE_INSTRUMENTATION_OK`
- `NA0324_PADDING_SIZE_CLASS_INSTRUMENTATION_OK`
- `NA0324_INVALID_RETRY_INSTRUMENTATION_OK`
- `NA0324_NO_SECRET_TRACE_ARTIFACT_OK`

Mitigation-design markers for a later design lane:

- `NA0324_MITIGATION_OPTION_MATRIX_OK`
- `NA0324_JITTER_DESIGN_BOUNDARY_OK`
- `NA0324_BATCHING_DESIGN_BOUNDARY_OK`
- `NA0324_COVER_TRAFFIC_DESIGN_BOUNDARY_OK`
- `NA0324_MEASUREMENT_BEFORE_MITIGATION_OK`
- `NA0324_NO_TIMING_HIDDEN_CLAIM_OK`
- `NA0324_NO_METADATA_FREE_CLAIM_OK`

If NA-0324 implements an instrumentation harness, future allowed files should
be limited to a qshield test harness plus governance evidence/testplan and
ordinary decision/traceability/journal updates. Any script, input schema,
runtime source, service, workflow, dependency, qsc/qsp/protocol/crypto,
qsl-server, qsl-attachments, website, README, or START_HERE change should
require exact future scope.

Future instrumentation stop conditions:

- raw secrets, route tokens, ack IDs, candidate values, plaintext, raw local
  paths, or secret-like dumps would be recorded;
- runtime instrumentation is needed but not authorized;
- qsl-server or qsl-attachments work is needed but not cross-repo authorized;
- measurement is presented as mitigation;
- local/demo evidence is presented as production proof;
- timing metadata or traffic shape is described as hidden without exact later
  proof;
- required CI or public-safety fails conclusively.

## Selected Successor

Selected successor:

`NA-0324 -- Metadata Runtime Timing and Traffic-Shape Instrumentation Harness`

Rationale:

- NA-0322 already shows test-only qshield embedded relay/demo measurement is
  feasible without user-facing runtime changes.
- The highest-value next step is better event coverage, schema discipline,
  queue/padding/retry classing, and no-secret artifact proof.
- Mitigation options are identified but need stronger instrumentation evidence
  before runtime implementation can be authorized truthfully.

## Rejected Alternatives

- `NA-0324 -- Metadata Runtime Timing and Traffic-Shape Mitigation Option Matrix`
  is rejected as the immediate successor because the instrumentation path is
  feasible and provides better evidence for a later matrix.
- `NA-0324 -- Metadata Runtime qshield Demo Timing Instrumentation Harness` is
  rejected as too narrow; the future harness should cover timing and
  traffic-shape instrumentation while still implementing only bounded
  qshield/demo proof unless a later directive expands scope.
- `NA-0324 -- Metadata Runtime Service Timing Cross-Repo Authorization` is
  rejected as immediate successor because qshield/demo instrumentation can
  improve evidence before service timing work.
- `NA-0324 -- Metadata Runtime Timing and Traffic-Shape Mitigation Blocker
  Resolution` is rejected because no blocker prevents the bounded qshield/demo
  instrumentation harness successor.

## Claim Boundaries

NA-0323 does not claim:

- runtime mitigation was implemented;
- timing metadata is hidden;
- traffic shape is hidden;
- anonymity;
- metadata-free behavior;
- untraceability;
- production readiness;
- public internet readiness;
- external review completion;
- qsl-server production timing proof;
- qsl-attachments production timing proof.

Safe future wording must say that instrumentation measures observable metadata
and that mitigation options are future work until separately authorized and
implemented with exact executable evidence.

## Backup-Plan Impact Statement

No backup-plan update is required. Durable changes stay inside the
qsl-protocol worktree under `/srv/qbuild/work`, which is already inside the
qbuild backup scope. No new durable evidence location, source root, excluded
backup path, or non-rebuildable artifact outside current backup scope is
introduced.

## Next Recommendation

After this design PR merges and post-merge public-safety is green, close out
NA-0323 and restore:

`NA-0324 -- Metadata Runtime Timing and Traffic-Shape Instrumentation Harness`

The closeout must not implement NA-0324. The future NA-0324 lane should add a
bounded qshield embedded relay/demo instrumentation harness, preserve the
measurement-versus-mitigation boundary, and stop before runtime or service
instrumentation if the exact future scope does not authorize it.
