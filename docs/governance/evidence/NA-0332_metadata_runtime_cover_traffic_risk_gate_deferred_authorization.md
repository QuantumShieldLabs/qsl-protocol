Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-22

# NA-0332 Metadata Runtime Cover Traffic Risk Gate and Deferred Authorization

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0332 completes a design/governance risk gate for metadata-runtime cover
traffic after NA-0331 delivered bounded qshield embedded relay/demo batching.

The decision is to keep cover traffic deferred and to require an exact
cost/quota/retention prerequisite lane before any cover-traffic prototype can
be authorized. The selected successor is:

`NA-0333 -- Metadata Runtime Cover Traffic Cost / Quota / Retention Prerequisite Plan`

This lane does not implement cover traffic, a cover-traffic prototype, runtime
timing mitigation, batching changes, jitter changes, retry-cadence changes,
qshield runtime changes, qsl-server behavior, qsl-attachments behavior,
protocol behavior, crypto behavior, workflow behavior, or dependency changes.

Cover traffic remains high-risk and high-cost. It may reduce selected timing
or volume distinguishers only after explicit bounded design and proof, but it
can also amplify abuse, denial-of-service, bandwidth, storage, backup,
retention, logging, and deployment risks. This evidence does not claim that
timing metadata or traffic shape is hidden. It does not claim anonymity. It
does not claim metadata-free behavior. It does not claim untraceable behavior,
production readiness, public-internet readiness, or external-review completion.

## Live NA-0332 Scope

Live `NEXT_ACTIONS.md` entry:

- `NA-0332 -- Metadata Runtime Cover Traffic Risk Gate and Deferred Authorization Plan`
- Status: READY
- Goals: G1, G2, G3, G4, G5
- Objective: execute the next timing/traffic-shape mitigation/design lane
  selected by NA-0331 evidence: a cover-traffic risk gate and deferred
  authorization plan, or stop on an exact prerequisite.

Allowed work used by this lane:

- produce a cover-traffic risk gate and deferred authorization plan;
- review inherited NA-0331 batching proof, NA-0329 bounded-jitter proof, and
  NA-0327 retry-cadence proof;
- analyze abuse, DoS, cost, bandwidth, storage, latency, reliability, backup,
  retention, operations, deployment, and external-review sensitivity;
- compare qshield embedded relay/demo experiments with qsl-server and
  qsl-attachments production surfaces;
- select one exact NA-0333 successor;
- update governance evidence, testplan, decisions, traceability, and the
  rolling operations journal.

Forbidden work preserved:

- cover traffic implementation or prototype;
- batching, bounded-jitter, retry-cadence, broad scheduling, send scheduling,
  receive scheduling, transport padding, runtime timing mitigation, or
  production timing implementation;
- qshield runtime source changes;
- qsl-server or qsl-attachments changes;
- qsc/qsp/protocol/crypto/key-schedule changes;
- Cargo or dependency changes;
- workflow, website, README, START_HERE, docs/public, branch-protection, or
  public-safety configuration changes;
- claims that timing metadata or traffic shape is hidden;
- prohibited anonymity, metadata-free, untraceable, production-readiness,
  public-internet-readiness, or external-review-complete claims.

## Inherited NA-0331 Batching Proof

NA-0331 D-0644 implemented only bounded qshield embedded relay/demo batching:

- policy name `qshield_demo_batching_v1`;
- opt-in `QSHIELD_DEMO_BATCHING=1`;
- deterministic test mode `QSHIELD_DEMO_BATCHING_TEST_MODE=1`;
- maximum send, receive candidate, and ack batch size of four members;
- effective send-side wait of 0 ms inside the previously authorized 750 ms cap;
- send-batch all-or-nothing validation;
- receive candidate cap and fail-closed invalid receive behavior;
- ack-batch all-or-nothing validation for locally verified ack IDs only;
- valid single-message compatibility;
- deterministic valid batch delivery and ordering preservation;
- invalid batch no remote delete before local verification;
- no accepted local state or plaintext output on invalid receive;
- retry-cadence and bounded-jitter preservation;
- secret-safe artifact evidence.

NA-0331 did not implement cover traffic, qsl-server production batching,
qsl-attachments production batching/object timing, broad queue scheduling,
transport padding expansion, protocol/crypto/qsc/qsp/key-schedule behavior, or
public/privacy readiness claims.

## Inherited Retry-Cadence and Bounded-Jitter Proof

NA-0327 D-0636 added opt-in qshield embedded relay/demo retry-cadence
normalization:

- policy `qshield_demo_retry_cadence_v1`;
- invalid candidate attempts capped at four per 60 second local window;
- retry classes 0 ms, 500 ms, 1000 ms, and 2000 ms;
- empty poll bounded at the 2000 ms retry class;
- no remote candidate delete before local verification;
- no accepted local state and no plaintext output on invalid retry;
- secret-safe local retry ledger.

NA-0329 D-0640 added opt-in qshield embedded relay/demo bounded jitter:

- policy `qshield_demo_bounded_jitter_v1`;
- maximum selected jitter of 250 ms;
- retry plus jitter composed cap of 2250 ms;
- invalid-candidate retry and empty-poll jitter only;
- no post-verify ack/delete jitter in the first harness;
- retry-cadence cap preservation;
- no secret jitter artifacts.

Neither lane proves qsl-server or qsl-attachments production timing behavior.
Neither lane proves that timing metadata or traffic shape is hidden.

## Sources Inspected

- `NEXT_ACTIONS.md`
- `GOALS.md`
- `PROJECT_CHARTER.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `CHECKLIST_PROTOCOL_CHANGE.md`
- `docs/dev/DOC-DEV-003_Assistant_Operating_Rules_v1.0.0_DRAFT.md`
- `docs/ops/DOC-OPS-003_Rolling_Operations_Journal_Procedure_v0.1.0_DRAFT.md`
- `tests/NA-0331_closeout_restore_na0332_testplan.md`
- `docs/governance/evidence/NA-0331_metadata_runtime_qshield_demo_batching_harness.md`
- `tests/NA-0331_metadata_runtime_qshield_demo_batching_harness_testplan.md`
- `docs/governance/evidence/NA-0330_metadata_runtime_qshield_demo_batching_authorization.md`
- `docs/governance/evidence/NA-0329_metadata_runtime_qshield_demo_bounded_jitter_harness.md`
- `docs/governance/evidence/NA-0327_metadata_runtime_qshield_demo_retry_cadence_normalization_harness.md`
- `apps/qshield-cli/src/commands/send.rs`
- `apps/qshield-cli/src/commands/recv.rs`
- `apps/qshield-cli/src/commands/relay.rs`
- `apps/qshield-cli/src/relay_client.rs`
- `apps/qshield-cli/tests/na_0331_metadata_runtime_batching.rs`
- recent metadata-runtime governance/evidence, decisions, traceability, and
  testplans.

Search coverage included cover traffic, dummy traffic, dummy message,
synthetic traffic, decoy, padding, bandwidth, storage, DoS, abuse, cost,
deployment, backup, operations, retention, queue pressure, batching, bounded
jitter, retry cadence, qshield demo, qsl-server, qsl-attachments, production,
prohibited phrase families such as timing-hidden, traffic-hidden,
metadata-free, anonymity, untraceable, `FUTURE_GATE`, and `NOT_READY`.

## Cover Traffic Threat / Value Model

Cover traffic could address only selected observable patterns after strict
scope and cost controls exist:

- idle versus active timing observability;
- traffic-volume observability;
- queue-drain cadence observability;
- burst correlation across send, receive, and ack events;
- route/contact activity correlation in narrow profiles;
- attachment timing and object-size correlation in a separately authorized
  qsl-attachments lane.

Cover traffic does not solve:

- endpoint compromise;
- production logging or telemetry disclosure;
- route/contact relationship leakage from authenticated or operational state;
- content/key compromise;
- service operator visibility into allowed request counts, object counts, or
  quotas;
- public-internet deployment readiness;
- broad unsupported anonymity, metadata-free behavior, or untraceable
  behavior.

The value model is therefore conditional: any future cover traffic must be
presented as bounded mitigation for specific observable classes, not as a
privacy completeness claim.

## Candidate Cover-Traffic Modes

1. None/defer:
   - no new traffic, no new operational burden, and no new privacy claim;
   - leaves existing timing and traffic-shape gaps explicit.

2. qshield demo synthetic local dummy messages:
   - local/demo-only dummy candidates generated inside the embedded relay
     harness;
   - useful for studying validation, artifact safety, and operator text;
   - not production proof.

3. qshield demo fixed-rate dummy candidates:
   - local/demo-only fixed-rate dummy enqueue/poll behavior;
   - highest demo observability value, but risks CI/runtime cost and false
     confidence if not bounded tightly.

4. qshield demo opportunistic cover during active sessions:
   - dummy traffic only while real activity exists;
   - lower idle cost than fixed-rate cover, but still exposes active-session
     boundaries and may amplify bursts.

5. qshield demo batch-fill cover:
   - fill small local/demo batches up to a cap;
   - may compose with NA-0331 batching, but needs all-or-nothing validation
     and no remote delete before local verification.

6. qsl-server production relay cover:
   - server-side cover enqueue/pull behavior;
   - requires qsl-server cross-repo authorization, abuse controls, rate
     limits, quota accounting, logs, and deployment model.

7. qsl-attachments production object cover:
   - cover upload/object creation or object-size padding behavior;
   - requires qsl-attachments cross-repo authorization, disk quota, retention,
     cleanup, backup, and recovery proof.

## Risk / Cost / Abuse / Ops Matrix

| Option | Threat addressed | Threat not addressed | Abuse / DoS risk | Bandwidth / storage cost | Latency / CPU / memory cost | Backup / retention / purge impact | Logging / observability risk | Deployment complexity | qshield demo feasibility | qsl-server / qsl-attachments feasibility | External-review sensitivity | Claim boundary | Recommendation |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| No cover traffic | No new signal is added; preserves current bounded jitter/batching evidence | Idle/active timing, traffic volume, burst and queue cadence remain observable | Lowest | None | None | None | Existing gaps remain explicit | Lowest | Already true | Already true | Low | Must state gaps remain | Keep as current baseline |
| Demo-only synthetic cover | Exercises dummy validation and artifact boundaries | Production timing, endpoint compromise, service logs, route/contact leakage | Medium if dummy creation bypasses caps | Bounded if fixture-only; unbounded if looped | CI/runtime cost if sleeps or loops exist | Tempdir only if bounded; no durable backup impact | Dummy labels can create misleading evidence if logs are careless | Low to medium | Feasible after cost/quota plan | Not production proof | Medium | Local/demo only | Do not authorize until cost/quota/retention lane defines caps |
| Active-session cover only | May reduce exact active burst regularity | Idle/active boundary, volume, route/contact relationships | Medium; attackers can trigger active sessions | Scales with real traffic and attacker activity | Can increase queue pressure and delays | Requires purge and retention caps | Logs may reveal cover/real ratios | Medium | Feasible only as future prototype | Production requires cross-repo gates | Medium | Bounded active-session study only | Defer behind prerequisite lane |
| Fixed-rate cover | Targets idle/active timing and cadence | Endpoint compromise, logging, relationship leakage, content/key compromise | High; easy bandwidth amplification | High continuous bandwidth and storage risk | Can create constant CPU/network load and delivery delays | High backup and retention growth unless excluded/purged | Strong risk of overclaiming privacy | High | Expensive even in demo if not tiny | Not feasible without service quotas and deployment plan | High | Must never imply hidden timing | Reject for current phase; require explicit later authorization |
| Batch-fill cover | May reduce small-batch size distinguishers | Timing still observable; batch windows and volume caps visible | Medium; can force filler generation | Medium and tied to batch rate | Adds validation and scheduling complexity | Local staging/retention model required | Batch-fill logs can reveal empty capacity | Medium | Plausible later after NA-0331, but not now | Service feasibility requires quota and storage proof | Medium | Bounded batch-fill only | Defer behind cost/quota/retention prerequisite |
| Attachment-size cover | May reduce selected object-size classes | Upload timing, fetch timing, retention, route/contact leakage | High; uploads are costly | Very high storage and backup growth risk | Higher I/O and cleanup cost | Major retention, purge, backup, and recovery impact | Object counts and cleanup logs can reveal patterns | High | Not qshield-only | Requires qsl-attachments authorization | High | Size-class only, not metadata-free | Reject for qsl-protocol-only NA-0332; cross-repo-gated |
| Production relay cover | May reduce selected service relay timing/volume classes | Endpoint compromise, qsl-attachments object timing, logs, relationships | High; public callers can amplify cover | High service bandwidth and queue cost | Queue, memory, and rate-limit pressure | Requires server retention and backup policy | Service logs can expose cover patterns | High | Not demo scope | Requires qsl-server authorization | High | Production proof only after service evidence | Not authorized |
| Production attachment cover | May reduce selected attachment object-size/timing classes | Relay timing, endpoint compromise, logs, relationships | High; disk amplification | Very high disk, egress, backup cost | I/O and cleanup pressure | Major backup and partial-restore implications | Storage logs/object metadata reveal activity | High | Not demo scope | Requires qsl-attachments authorization | High | Object-profile only | Not authorized |
| Adversarial cover amplification | None; this is a misuse case | All privacy threats remain | Critical if quotas absent | Potentially unbounded | Queue and resource exhaustion | Backup growth and cleanup failures | Abuse logs may become sensitive | High | Must be modeled before prototype | Must be modeled before production | High | Failure scenario only | NA-0333 must model this first |
| Quota exhaustion | None; this is a failure mode | All privacy threats remain | High | Depends on quota design | Can starve real messages | Retention/purge must fail closed | Logs may reveal user activity | Medium to high | Must be fixture-mode only | Requires service-specific proof | High | Failure scenario only | NA-0333 must define quota accounting |
| Backup growth | None; this is an ops risk | All privacy threats remain | Medium to high | High durable-data risk | Restore and audit complexity | Backup plan may need change | Backup evidence can leak patterns | High | Avoid durable demo artifacts | Requires backup policy | High | Ops risk only | NA-0333 must define backup boundary |
| Public-internet deployment | Could only help after service proof | Does not solve broad metadata or endpoint risks | Critical | High | High | High | High | Highest | Not applicable | Requires separate governance and service lanes | Highest | No readiness claim | Not authorized |

## Defer / Reject / Authorize Decision

NA-0332 decides:

1. Cover traffic remains deferred.
2. Direct cover-traffic implementation is rejected for the current QSL phase.
3. A cost/quota/retention prerequisite lane is required before any bounded
   qshield demo-only cover-traffic prototype authorization can be considered.
4. A qshield demo-only prototype may be considered only after the prerequisite
   lane defines exact traffic caps, dummy object shape, quota accounting,
   retention/purge behavior, artifact handling, marker names, and stop
   conditions.
5. qsl-server and qsl-attachments production cover traffic require separate
   cross-repo service authorization before any production prototype.

Why:

- fixed-rate or object cover can create large continuous bandwidth, storage,
  backup, and cleanup burdens;
- active-session and batch-fill cover can be adversarially amplified;
- qshield demo experiments can produce useful local evidence but can also
  create misleading privacy claims if service boundaries are not explicit;
- production relay/object cover changes service economics and operations;
- stronger public privacy language would require external review and service
  evidence that do not exist in NA-0332.

## Future Validation / Marker Plan

Selected NA-0333 prerequisite markers:

- `NA0333_COVER_TRAFFIC_COST_QUOTA_PLAN_OK`
- `NA0333_BANDWIDTH_STORAGE_BOUNDARY_OK`
- `NA0333_BACKUP_IMPACT_BOUNDARY_OK`
- `NA0333_ABUSE_DOS_BOUNDARY_OK`
- `NA0333_COVER_TRAFFIC_COST_MODEL_REQUIRED_OK`
- `NA0333_COVER_TRAFFIC_QUOTA_MODEL_REQUIRED_OK`
- `NA0333_COVER_TRAFFIC_RETENTION_MODEL_REQUIRED_OK`
- `NA0333_NO_COVER_TRAFFIC_IMPLEMENTATION_OK`
- `NA0333_QSHIELD_DEMO_BOUNDARY_OK`
- `NA0333_SERVICE_PRODUCTION_BOUNDARY_OK`
- `NA0333_NO_TIMING_HIDDEN_CLAIM_OK`
- `NA0333_NO_TRAFFIC_SHAPE_HIDDEN_CLAIM_OK`
- `NA0333_NO_METADATA_FREE_CLAIM_OK`
- `NA0333_NO_ANONYMITY_CLAIM_OK`

Future NA-0333 must define:

- cost model inputs and output units;
- maximum allowable local/demo dummy count and duration;
- bandwidth and storage upper bounds;
- quota and rate-limit accounting for real versus cover traffic;
- retention and purge behavior for cover artifacts;
- backup-impact boundary and whether backup-plan updates are required;
- explicit service-production stop conditions;
- markers that prove no implementation occurred in the prerequisite lane.

## Production Boundary

qshield embedded relay/demo cover traffic would be local/demo evidence only. It
could study synthetic dummy shape, marker safety, and fail-closed validation
inside the embedded relay harness, but it would not prove qsl-server or
qsl-attachments production timing behavior.

qsl-server production cover traffic requires cross-repo authorization because
it would affect service queues, per-route and global quota, rate limits,
logging, deployment, and operator costs.

qsl-attachments production object cover requires cross-repo authorization
because it would affect object counts, disk pressure, upload/fetch cadence,
retention, purge, backup/restore, and recovery behavior.

Public-internet timing and traffic-shape posture remains future-gated.

## External-Review Sensitivity

Cover traffic is externally review sensitive because it is easy to overstate.
Even a correctly bounded implementation would require independent review
before any stronger privacy claim is made. Review should include:

- threat model fit and non-goals;
- traffic-analysis assumptions;
- adversarial amplification model;
- quota/rate-limit design;
- service logging and operator visibility;
- bandwidth, storage, retention, purge, backup, and restore economics;
- deployment and rollback plan;
- claim wording.

External review is recommended before any language stronger than bounded
experimental mitigation for a specific measured class.

## Public Claim Boundary

Allowed wording:

- cover traffic is deferred behind explicit risk and cost gates;
- current qshield batching, retry cadence, and bounded jitter are local/demo
  evidence only;
- qsl-server and qsl-attachments production timing remain unproven and
  cross-repo-gated;
- timing and traffic-shape gaps remain visible.

Prohibited wording:

- cover traffic is implemented;
- timing metadata is hidden;
- traffic shape is hidden;
- prohibited metadata-free behavior claim;
- prohibited anonymity claim;
- prohibited untraceable behavior claim;
- production readiness;
- public-internet readiness;
- external-review completion.

## Selected Successor

Selected:

`NA-0333 -- Metadata Runtime Cover Traffic Cost / Quota / Retention Prerequisite Plan`

Rationale:

- it directly addresses the largest blocker before any cover-traffic prototype:
  cost, quota, retention, purge, backup, and abuse boundaries;
- it keeps implementation deferred;
- it gives future qshield demo prototype authorization concrete stop/go
  criteria;
- it avoids prematurely opening qsl-server or qsl-attachments production
  behavior;
- it preserves conservative public/privacy claim boundaries.

## Rejected Alternatives

- Direct cover traffic implementation: rejected because abuse, DoS, cost,
  quota, retention, backup, deployment, and external-review boundaries are not
  yet proven.
- Direct qshield demo cover-traffic prototype authorization: deferred until
  NA-0333 defines exact caps and marker requirements.
- Direct qsl-server production cover: rejected for this repo-local lane and
  cross-repo-gated.
- Direct qsl-attachments production object cover: rejected for this
  repo-local lane and cross-repo-gated.
- Fixed-rate cover in the current phase: rejected because continuous traffic
  can create high cost and misleading privacy impressions.
- Padding bucket expansion as the immediate NA-0333 successor: rejected as the
  immediate successor because it is lower risk but does not close the
  cover-traffic cost/quota prerequisite identified by this gate.
- Claiming metadata-free behavior: rejected as unsupported.

## Backup-Plan Impact Statement

NA-0332 changes only qsl-protocol governance/evidence/testplan/journal files
under `/srv/qbuild/work`. It creates no durable evidence location outside the
current qbuild working tree and no non-rebuildable runtime artifacts outside
the existing backup scope.

No backup-plan update is required for NA-0332.

Future NA-0333 must revisit backup-plan impact if it proposes durable
cover-traffic artifacts, service logs, object stores, retained dummy payloads,
or backup exclusions.

## Next Recommendation

Close NA-0332 only after this risk gate merges and post-merge public-safety is
green. Then restore exactly one successor:

`NA-0333 -- Metadata Runtime Cover Traffic Cost / Quota / Retention Prerequisite Plan`

NA-0333 must remain prerequisite/design-only unless its live directive
explicitly authorizes a narrower implementation lane later. It must not
implement cover traffic.
