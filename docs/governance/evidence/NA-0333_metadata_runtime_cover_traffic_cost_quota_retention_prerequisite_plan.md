Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-22

# NA-0333 Metadata Runtime Cover Traffic Cost Quota Retention Prerequisite Plan

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0333 defines the prerequisite plan required before any future cover-traffic
prototype or production cover-traffic lane can be authorized.

The plan keeps NA-0333 planning-only. It implements no cover traffic, no cover
traffic prototype, no runtime timing mitigation, no batching change, no jitter
change, no retry-cadence change, no qshield runtime change, no qsl-server
change, no qsl-attachments change, no qsc/qsp/protocol/crypto/key-schedule
change, no dependency change, no workflow change, no website/public-doc
change, and no branch-protection or public-safety configuration change.

The decision is:

- a future qshield embedded relay/demo cover-traffic prototype may be
  considered only after a separate authorization plan proves the exact local
  demo scope, caps, markers, validation bundle, and stop conditions;
- qsl-server production cover traffic remains cross-repo gated;
- qsl-attachments production object cover remains cross-repo gated;
- fixed-rate and attachment-object cover are rejected for the current QSL
  phase;
- all public/privacy claims remain conservative.

Selected successor:

`NA-0334 -- Metadata Runtime qshield Demo Cover Traffic Prototype Authorization Plan`

NA-0334 is an authorization-plan successor, not an implementation successor.

## Live NA-0333 Scope

Live `NEXT_ACTIONS.md` entry:

- `NA-0333 -- Metadata Runtime Cover Traffic Cost / Quota / Retention Prerequisite Plan`
- Status: READY
- Goals: G1, G2, G3, G4, G5
- Objective: execute the next metadata-runtime timing/traffic-shape lane
  selected by NA-0332: a cover-traffic cost, quota, retention, purge, backup,
  abuse, and deployment prerequisite plan, or stop on an exact prerequisite.

Allowed work used by this lane:

- define cost, bandwidth, storage, quota, abuse/DoS, retention, purge, backup,
  operations, deployment, qshield-demo, and service-production prerequisites;
- review inherited NA-0332, NA-0331, NA-0330, NA-0329, and NA-0327 evidence;
- decide whether a bounded qshield-demo cover-traffic prototype can be
  considered later, must remain deferred, or must be rejected;
- select one exact NA-0334 successor;
- update governance evidence, testplan, decisions, traceability, and the
  rolling operations journal.

Forbidden work preserved:

- cover traffic implementation or prototype implementation;
- runtime timing mitigation, batching, bounded jitter, retry-cadence, broad
  queue scheduling, send scheduling, receive scheduling, or transport padding
  implementation;
- qshield runtime source changes;
- qsl-server or qsl-attachments changes;
- qsc/qsp/protocol/crypto/key-schedule changes;
- Cargo or dependency changes;
- workflow, website, README, START_HERE, docs/public, qsc-desktop,
  branch-protection, or public-safety configuration changes;
- claims that timing metadata or traffic shape is hidden;
- anonymity, metadata-free, untraceable, production-readiness,
  public-internet-readiness, or external-review-complete claims.

The live scope matches this directive and does not conflict with the
planning-only evidence patch.

## Inherited NA-0332 Risk Gate

NA-0332 D-0646 established the current cover-traffic gate:

- cover traffic remains deferred;
- direct cover-traffic implementation is rejected for the current phase;
- direct qshield-demo prototype authorization is deferred until this
  cost/quota/retention prerequisite plan exists;
- qsl-server production cover traffic requires cross-repo authorization;
- qsl-attachments production object cover requires cross-repo authorization;
- fixed-rate and object cover are high-cost/high-risk;
- timing metadata and traffic shape are not claimed hidden;
- anonymity, metadata-free behavior, untraceable behavior, production
  readiness, public-internet readiness, and external-review completion remain
  unsupported.

NA-0332 selected this exact NA-0333 prerequisite lane because cover traffic can
amplify bandwidth, storage, backup, retention, logging, quota, abuse, DoS,
deployment, rollback, and operator-monitoring costs if implemented before the
resource model is explicit.

## Inherited NA-0331 Batching Proof

NA-0331 D-0644 implemented only bounded qshield embedded relay/demo batching:

- policy `qshield_demo_batching_v1`;
- opt-in `QSHIELD_DEMO_BATCHING=1`;
- deterministic test mode `QSHIELD_DEMO_BATCHING_TEST_MODE=1`;
- maximum send, receive-candidate, and ack batch size of four;
- send and ack batches validate all members before mutation;
- receive invalid candidates fail closed without remote delete before local
  verification;
- valid single-message behavior remains unchanged;
- retry-cadence and bounded-jitter caps remain intact;
- batch artifacts are secret-safe.

That proof is local/demo only. It is not qsl-server or qsl-attachments
production timing proof. It does not implement or authorize cover traffic.

## Sources Inspected

- `NEXT_ACTIONS.md`
- `GOALS.md`
- `PROJECT_CHARTER.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `CHECKLIST_PROTOCOL_CHANGE.md`
- `docs/dev/DOC-DEV-003_Assistant_Operating_Rules_v1.0.0_DRAFT.md`
- `docs/ops/DOC-OPS-003_Rolling_Operations_Journal_Procedure_v0.1.0_DRAFT.md`
- `docs/ops/TEMPLATE_Rolling_Operations_Journal_v0.1.0.md`
- `tests/NA-0332_closeout_restore_na0333_testplan.md`
- `docs/governance/evidence/NA-0332_metadata_runtime_cover_traffic_risk_gate_deferred_authorization.md`
- `tests/NA-0332_metadata_runtime_cover_traffic_risk_gate_deferred_authorization_testplan.md`
- `docs/governance/evidence/NA-0331_metadata_runtime_qshield_demo_batching_harness.md`
- `docs/governance/evidence/NA-0330_metadata_runtime_qshield_demo_batching_authorization.md`
- `docs/governance/evidence/NA-0329_metadata_runtime_qshield_demo_bounded_jitter_harness.md`
- `docs/governance/evidence/NA-0327_metadata_runtime_qshield_demo_retry_cadence_normalization_harness.md`
- `apps/qshield-cli/tests/na_0331_metadata_runtime_batching.rs`
- qshield NA-0318, NA-0319, NA-0320, NA-0322, NA-0324, NA-0327, NA-0329,
  and NA-0331 harness outputs from the local preflight bundle.

Search coverage included cover traffic, dummy traffic, quota, rate limit,
retention, purge, backup, bandwidth, storage, cost, abuse, DoS, amplification,
deployment, production, qshield demo, qsl-server, qsl-attachments, public
internet, timing hidden, traffic hidden, metadata-free, anonymity,
untraceable, `FUTURE_GATE`, and `NOT_READY`.

## Cost / Bandwidth / Storage Model

Any future qshield-demo cover-traffic prototype must start with a static cost
estimate before it can enqueue or send cover items. If the estimate exceeds
the applicable cap, the future prototype must fail closed before generating
cover traffic.

Shared units:

- `cover_item`: one synthetic local/demo candidate or filler member.
- `cover_payload_bytes`: bytes carried as the synthetic local/demo message
  body before HTTP and JSON overhead.
- `cover_request_bytes`: conservative HTTP plus JSON request/response estimate.
- `cover_storage_bytes`: local queue, ledger, and artifact bytes retained
  after a run.

Global qshield-demo envelope for any future prototype authorization:

- maximum cover payload size per item: 8192 bytes, matching the largest
  currently tested qshield demo padding bucket unless a future lane narrows it;
- maximum cover items per minute: 4;
- maximum cover items per hour: 32;
- maximum cover items per run or local day: 64;
- maximum cover payload bytes per run: 512 KiB;
- maximum estimated request bytes per run: 1 MiB;
- maximum queued cover items: 16 global and 4 per route;
- maximum retained cover artifacts: 4 files per run and 1 MiB total;
- deterministic test-mode cap: 8 cover items, 64 KiB payload, 128 KiB
  estimated request bytes, and no real sleeps longer than existing bounded
  qshield test waits;
- operator stop threshold: abort before cover generation if the static
  estimate exceeds any cap or if available disk under `/srv/qbuild` falls
  below 10 GiB.

Cost model by mode:

| Mode | Per-minute cap | Per-hour cap | Per-run or local-day cap | Storage cap | Budget class | Decision |
| --- | ---: | ---: | ---: | ---: | --- | --- |
| No cover traffic | 0 | 0 | 0 | 0 | None | Current baseline |
| qshield-demo synthetic local cover | 4 items / 32 KiB payload | 16 items / 128 KiB payload | 32 items / 256 KiB payload | 512 KiB | Low | Can be considered by NA-0334 authorization plan only |
| Active-session cover only | 4 items / 32 KiB payload | 32 items / 256 KiB payload | 64 items / 512 KiB payload | 1 MiB | Low-to-medium | Can be considered only if tied to real local/demo activity and capped |
| Batch-fill cover | 4 items / 32 KiB payload | 32 items / 256 KiB payload | 64 items / 512 KiB payload | 1 MiB | Low-to-medium | Can be considered only inside NA-0331 batch size 4 and no recursive fill |
| Fixed-rate qshield-demo cover | Stop by default | Stop by default | Stop by default | Stop by default | High | Rejected for current phase |
| Attachment-size cover | Stop by default | Stop by default | Stop by default | Stop by default | High | Rejected for qsl-protocol-only current phase |
| qsl-server production relay cover | Stop by default | Stop by default | Stop by default | Stop by default | High | Cross-repo gated |
| qsl-attachments production object cover | Stop by default | Stop by default | Stop by default | Stop by default | High | Cross-repo gated |

Cost dimensions required for any future authorization:

- bandwidth ingress and egress estimates;
- relay queue item count and byte count;
- local qshield store and ledger byte growth;
- attachment object-store impact, which must be zero for a qshield-demo-only
  prototype;
- CPU/memory estimate for cover generation, validation, polling, and cleanup;
- queue-depth impact and real-message starvation analysis;
- backup growth and exclusion/inclusion decision;
- log/artifact growth and redaction plan;
- operator monitoring and stop-threshold plan.

Unsupported claim boundary:

- cost bounds are resource controls only;
- they do not prove that timing metadata is hidden;
- they do not prove that traffic shape is hidden;
- they do not prove anonymity, metadata-free behavior, untraceable behavior,
  production readiness, public-internet readiness, or external-review
  completion.

## Quota / Abuse / DoS Model

Any future qshield-demo cover-traffic prototype must account cover items
against stricter quotas than real local/demo messages and must drop or refuse
cover before real messages are affected.

Required quota controls:

- per-run cap: no more than 64 cover items and 1 MiB estimated request bytes;
- per-route cap: no more than 8 cover items per route per run and no more
  than 4 queued cover items per route;
- per-session cap: no more than 16 cover items per qshield demo session;
- per-time-window cap: no more than 4 cover items per minute and 32 per hour;
- global queued cap: no more than 16 cover items across the embedded relay;
- retained-artifact cap: no more than 4 cover artifacts and 1 MiB total;
- deterministic test-mode caps: no more than 8 cover items and 128 KiB
  estimated request bytes;
- abuse stop condition: fail closed before cover generation if a request would
  exceed any cover, route, session, queue, storage, or artifact cap.

Required scenario handling:

- Honest local demo cover: generated only under opt-in qshield-demo settings,
  uses deterministic test mode in CI, and cleans up at run end.
- Malicious queued-message injector: cover never bypasses existing relay body,
  total queue, per-recipient queue, or per-token queue caps.
- Malicious cover-trigger amplification: one cover item must never trigger
  another cover item; recursive cover is forbidden.
- Repeated invalid cover interactions: invalid cover attempts count against
  the same or stricter retry/invalid-attempt budgets used by existing qshield
  demo retry evidence.
- Quota exhaustion: real local/demo messages take priority; cover is dropped
  or refused first with a coarse local error.
- Relay queue exhaustion: cover generation aborts before enqueue if the queue
  lacks capacity for all planned real and cover items.
- Backup growth: future cover artifacts remain temporary and rebuildable
  unless a later backup plan explicitly changes scope.
- Attachment storage growth: qshield-demo prototype must create zero
  qsl-attachments objects.
- Service deployment pressure: qshield-demo evidence cannot be used as
  qsl-server or qsl-attachments deployment proof.
- Public-internet abuse: not authorized; any public network or service lane
  must stop for cross-repo authorization.
- Log/artifact growth: logs and artifacts record aggregate counts and marker
  names only, not route tokens, raw handles, raw ack IDs, plaintext,
  passphrases, key material, long hex dumps, or sensitive endpoint fragments.
- Operator runaway cost: static estimate and runtime counters both abort
  before exceeding the operator stop threshold.

The no-production-service default is mandatory: qshield-demo cover traffic, if
later authorized, remains local/demo only and must not call qsl-server or
qsl-attachments production paths.

## Retention / Purge / Backup / Ops Model

Future cover item lifecycle:

1. Plan: compute static cost/quota/retention estimate.
2. Authorize locally: require opt-in qshield-demo settings and deterministic
   test mode for CI.
3. Generate: create only bounded synthetic local/demo cover items.
4. Queue: enqueue only if all route, session, global, and storage caps remain
   within bounds.
5. Deliver or drop: cover may be consumed only through local/demo harness
   semantics; cover is dropped before real messages when capacity is tight.
6. Verify/ack or reject: local verification and ack/delete ordering must
   preserve the NA-0318 through NA-0331 no-remote-delete-before-verify
   boundary.
7. Purge: remove cover items at ack/delete, failed verification, stale TTL, or
   run end.
8. Record: retain only secret-safe aggregate markers and bounded evidence.

Real item versus cover item distinction:

- the future implementation must maintain an internal secret-safe cover flag
  or equivalent local classification;
- the distinction must not be exposed as plaintext to users;
- the distinction must not require logging route tokens, raw peer handles, raw
  ack IDs, plaintext, passphrases, key material, or long hex values;
- if a future implementation cannot distinguish real and cover items without
  leaking sensitive state, it must stop before implementation.

Required purge triggers:

- successful ack/delete;
- failed local verification;
- stale cover TTL, capped at 10 minutes for qshield-demo prototype work unless
  a future lane narrows it;
- route/session cap exhaustion;
- operator stop threshold;
- run end;
- parse/serialization failure while writing local cover state;
- any failed redaction or artifact-scan check.

Required cleanup behavior:

- stale cover cleanup must be deterministic and bounded;
- failed cover cleanup must not delete unverified real candidates;
- cleanup errors must fail closed with a coarse local error;
- rollback requires disabling the opt-in cover setting and deleting only
  cover-classified local/demo artifacts;
- no backup restore procedure may be required for qshield-demo cover artifacts.

Backup impact:

- NA-0333 changes only qsl-protocol governance/testplan/journal files under
  `/srv/qbuild/work`;
- no new backup-plan update is required for NA-0333;
- future cover-traffic implementation may require a backup-plan update if it
  creates durable non-rebuildable artifacts, durable service logs, retained
  dummy payloads, new backup exclusions, qsl-server storage growth,
  qsl-attachments object growth, or durable artifacts outside the current
  qbuild scope.

Operations requirements:

- pre-run static cost estimate;
- runtime counters for cover items, bytes, queues, artifacts, and cleanup;
- operator stop thresholds documented in evidence and test output;
- secret-safe log redaction;
- artifact redaction and scan;
- cleanup procedure;
- rollback procedure;
- cross-repo stop for any qsl-server or qsl-attachments production behavior.

## Prototype Authorization Decision

Decision:

- A future qshield-demo-only cover-traffic prototype may be authorized later,
  but only through a separate authorization-plan lane.
- NA-0334 should be that authorization-plan lane.
- NA-0334 must still be free to stop, defer, or reject the prototype if exact
  implementation files, caps, marker proof, CI cost, or claim boundaries are
  not concrete enough.
- Production cover traffic remains deferred and cross-repo gated.
- Fixed-rate cover and attachment-object cover remain rejected for the current
  QSL phase.

Rationale:

- the prerequisite caps above are concrete enough to make a narrow qshield-demo
  authorization plan useful;
- qshield-demo cover can exercise dummy validation, artifact safety, quota
  accounting, and operator text without service deployment behavior;
- the resource caps keep the first possible prototype small enough for local
  CI proof;
- production-service relevance remains limited, so qsl-server and
  qsl-attachments behavior cannot be inferred from qshield-demo evidence;
- public claim risk stays controlled because the successor is still an
  authorization plan and must preserve all prohibited-claim boundaries.

## Future Validation / Marker Plan

Required NA-0334 markers for the selected successor:

- `NA0334_QSHIELD_DEMO_BOUNDARY_OK`
- `NA0334_SERVICE_PRODUCTION_BOUNDARY_OK`
- `NA0334_NO_TIMING_HIDDEN_CLAIM_OK`
- `NA0334_NO_TRAFFIC_SHAPE_HIDDEN_CLAIM_OK`
- `NA0334_NO_METADATA_FREE_CLAIM_OK`
- `NA0334_NO_ANONYMITY_CLAIM_OK`
- `NA0334_COVER_TRAFFIC_PROTOTYPE_AUTHORIZATION_OK`
- `NA0334_COVER_TRAFFIC_COST_BOUNDARY_OK`
- `NA0334_COVER_TRAFFIC_QUOTA_BOUNDARY_OK`
- `NA0334_COVER_TRAFFIC_RETENTION_BOUNDARY_OK`
- `NA0334_COVER_TRAFFIC_BACKUP_BOUNDARY_OK`
- `NA0334_COVER_TRAFFIC_ABUSE_BOUNDARY_OK`
- `NA0334_NO_PRODUCTION_COVER_TRAFFIC_OK`
- `NA0334_NO_COVER_TRAFFIC_IMPLEMENTATION_OK`

NA-0334 must also restate the NA-0333 prerequisite caps or narrow them. It
must not widen caps, scope, service surfaces, claim language, backup impact,
or implementation files without an explicit future directive.

## Production Boundary

qshield embedded relay/demo cover traffic remains local/demo only. It may
study synthetic local cover shape, marker safety, cost accounting, quotas,
retention, purge, and artifact redaction, but it is not production-service
proof.

qsl-server production cover traffic requires cross-repo authorization before
any implementation, prototype, deployment, queue, quota, logging, monitoring,
rate-limit, or rollback behavior changes.

qsl-attachments production object cover requires cross-repo authorization
before any implementation, prototype, object creation, object padding, upload
cadence, fetch cadence, retention, purge, backup, or recovery behavior
changes.

Public-internet timing and traffic-shape behavior remains `FUTURE_GATE`.

## External-Review Sensitivity

Cover traffic is externally review sensitive. It can make evidence look
stronger than it is and can create large cost or abuse risk. External review
is recommended before any language stronger than bounded experimental
mitigation for a specific measured local/demo class.

Review should cover:

- threat model fit and non-goals;
- traffic-analysis assumptions;
- cost and quota model;
- adversarial amplification model;
- service logging and operator visibility;
- retention, purge, backup, and restore economics;
- deployment and rollback plan;
- public claim wording.

No external-review-complete claim is made by NA-0333.

## Public Claim Boundary

Allowed wording:

- cover traffic is deferred behind explicit cost, quota, retention, backup,
  abuse, and deployment gates;
- a qshield-demo-only authorization plan is the next selected step;
- qsl-server and qsl-attachments production cover traffic remain
  cross-repo-gated;
- timing and traffic-shape gaps remain explicit.

Prohibited wording:

- cover traffic is implemented;
- a cover traffic prototype exists;
- timing metadata is hidden;
- traffic shape is hidden;
- metadata-free behavior;
- anonymity;
- untraceable behavior;
- production readiness;
- public-internet readiness;
- external-review completion.

## Selected Successor

Selected:

`NA-0334 -- Metadata Runtime qshield Demo Cover Traffic Prototype Authorization Plan`

Rationale:

- it is the smallest successor that uses the NA-0333 prerequisite plan without
  implementing cover traffic immediately;
- it keeps qshield embedded relay/demo evidence separate from production
  service behavior;
- it preserves the option to stop before implementation if exact caps or
  marker proof are not satisfactory;
- it keeps qsl-server and qsl-attachments production cover traffic deferred;
- it avoids replacing this cover-traffic prerequisite with a lower-risk but
  different padding bucket lane before the cover-traffic decision is
  authorization-ready.

## Rejected Alternatives

- Direct cover traffic implementation: rejected because NA-0333 is
  prerequisite planning only.
- Direct qshield-demo cover traffic prototype implementation: rejected because
  NA-0334 must first authorize exact files, caps, markers, and validation.
- Direct qsl-server production cover: rejected for this qsl-protocol-only lane
  and cross-repo gated.
- Direct qsl-attachments production object cover: rejected for this
  qsl-protocol-only lane and cross-repo gated.
- Fixed-rate cover in the current phase: rejected because continuous cover can
  create misleading evidence and high cost even in demo form.
- Attachment-size cover in the current phase: rejected because object storage,
  retention, purge, backup, and recovery costs belong in a separate
  qsl-attachments lane.
- Padding bucket expansion as immediate NA-0334: rejected as the immediate
  successor because it does not close the cover-traffic authorization question
  opened by NA-0332 and bounded by NA-0333.
- Service timing cross-repo authorization as immediate NA-0334: deferred
  because qshield-demo authorization can be evaluated locally first without
  service changes.
- Claiming metadata-free behavior: rejected as unsupported.

## Backup-Plan Impact Statement

NA-0333 changes only qsl-protocol governance/evidence/testplan/traceability
and journal files under `/srv/qbuild/work`.

No backup-plan update is required for NA-0333.

Future cover-traffic implementation or production-service lanes may require a
backup-plan update if they create durable non-rebuildable artifacts, durable
service logs, qsl-server storage growth, qsl-attachments object growth,
retained dummy payloads, backup exclusions, or durable evidence outside the
current qbuild scope.

## Next Recommendation

After NA-0333 merges and post-merge public-safety is green, close NA-0333 and
restore exactly one successor:

`NA-0334 -- Metadata Runtime qshield Demo Cover Traffic Prototype Authorization Plan`

NA-0334 must remain authorization/design only unless its future live directive
explicitly authorizes implementation. NA-0334 must not implement production
cover traffic, qsl-server behavior, or qsl-attachments behavior.
