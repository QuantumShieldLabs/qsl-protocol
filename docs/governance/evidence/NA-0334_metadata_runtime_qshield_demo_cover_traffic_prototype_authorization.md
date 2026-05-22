Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-22

# NA-0334 Metadata Runtime qshield Demo Cover Traffic Prototype Authorization

Goals: G1, G2, G3, G4, G5

## Executive Summary

NA-0334 authorizes one future bounded qshield demo-only cover-traffic
prototype implementation harness as NA-0335, with strict cost, quota,
retention, purge, backup, abuse, deployment, CI, and operator-stop boundaries.

The authorized future prototype is limited to qshield embedded relay/demo
experiments that can exercise synthetic local cover, active-session cover, and
batch-fill cover inside the NA-0333 caps. The authorization does not include
fixed-rate cover, qsl-server production relay cover, qsl-attachments
production object cover, public-internet behavior, or stronger public/privacy
claims.

NA-0334 is authorization/design only. It implements no cover traffic, no cover
traffic prototype, no runtime timing mitigation, no batching change, no jitter
change, no retry-cadence change, no qshield runtime change, no qsl-server
change, no qsl-attachments change, no qsc/qsp/protocol/crypto/key-schedule
change, no dependency change, no workflow change, no website/public-doc
change, and no branch-protection or public-safety configuration change.

Selected successor:

`NA-0335 -- Metadata Runtime qshield Demo Cover Traffic Prototype Implementation Harness`

NA-0335 must remain bounded to the exact future files, caps, markers, and stop
conditions below. It must stop before implementation if those boundaries cannot
be met without runtime or claim drift outside the authorized qshield demo-only
surface.

## Live NA-0334 Scope

Live `NEXT_ACTIONS.md` entry:

- `NA-0334 -- Metadata Runtime qshield Demo Cover Traffic Prototype Authorization Plan`
- Status: READY
- Goals: G1, G2, G3, G4, G5
- Objective: execute the next metadata-runtime timing/traffic-shape lane
  selected by NA-0333: a qshield demo-only cover-traffic prototype
  authorization plan, or stop on an exact prerequisite.

Allowed work used by this lane:

- decide whether to authorize, defer, or reject a bounded qshield demo-only
  cover-traffic prototype;
- map NA-0333 cost, quota, retention, purge, backup, abuse/DoS, and deployment
  prerequisites to any future prototype lane;
- define exact qshield demo versus qsl-server/qsl-attachments production
  boundaries;
- define a future marker and stop-condition plan;
- select one exact NA-0335 successor;
- update governance evidence, testplan, decisions, traceability, and the
  rolling operations journal.

Forbidden work preserved:

- cover traffic implementation or prototype implementation in NA-0334;
- runtime timing mitigation, batching, bounded jitter, retry-cadence, broad
  queue scheduling, send scheduling, receive scheduling, or transport padding
  implementation;
- qshield runtime source changes in NA-0334;
- qsl-server or qsl-attachments changes;
- qsc/qsp/protocol/crypto/key-schedule changes;
- Cargo or dependency changes;
- workflow, website, README, START_HERE, docs/public, qsc-desktop,
  branch-protection, or public-safety configuration changes;
- claims that timing metadata or traffic shape is hidden;
- anonymity, metadata-free, untraceable, production-readiness,
  public-internet-readiness, or external-review-complete claims.

The live scope matches the directive and supports this authorization-only
evidence patch.

## Inherited NA-0333 Prerequisite Plan

NA-0333 D-0648 defined the prerequisite boundaries required before any future
cover-traffic prototype or production cover-traffic lane can be authorized.

Inherited caps for any future qshield demo prototype:

- maximum cover payload size per item: 8192 bytes;
- maximum cover items per minute: 4;
- maximum cover items per hour: 32;
- maximum cover items per run or local day: 64;
- maximum cover payload bytes per run: 512 KiB;
- maximum estimated request bytes per run: about 1 MiB;
- maximum queued cover items: 16 global and 4 per route;
- maximum retained cover artifacts: 4 files per run and 1 MiB total;
- deterministic test-mode cap: 8 cover items, 64 KiB payload, and 128 KiB
  estimated request bytes;
- operator stop threshold: abort before cover generation if any estimate would
  exceed a cap or if available disk under `/srv/qbuild` falls below 10 GiB.

Inherited allowed later-consideration modes:

- qshield demo synthetic local cover;
- qshield demo active-session cover;
- qshield demo batch-fill cover.

Inherited rejected or gated modes:

- fixed-rate cover is rejected for the current phase;
- attachment-size cover is rejected for qsl-protocol-only work;
- qsl-server production relay cover is cross-repo gated;
- qsl-attachments production object cover is cross-repo gated.

NA-0333 selected this NA-0334 authorization lane because the caps are concrete
enough to evaluate a narrow qshield demo-only implementation harness without
touching production services.

## Inherited NA-0332 Risk Gate

NA-0332 D-0646 established the cover-traffic risk gate:

- cover traffic remained deferred;
- direct cover-traffic implementation was rejected for the current phase;
- any qshield-demo prototype needed exact cost, quota, retention, purge,
  backup, abuse, and deployment boundaries first;
- qsl-server production cover traffic requires cross-repo authorization;
- qsl-attachments production object cover requires cross-repo authorization;
- timing metadata and traffic shape are not claimed hidden;
- anonymity, metadata-free behavior, untraceable behavior, production
  readiness, public-internet readiness, and external-review completion remain
  unsupported.

NA-0332 also recorded why cover traffic is sensitive: it can add bandwidth,
storage, backup, retention, logging, quota, abuse, DoS, deployment, rollback,
and operator-monitoring costs if implemented before resource controls are
explicit.

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
- `tests/NA-0333_closeout_restore_na0334_testplan.md`
- `docs/governance/evidence/NA-0333_metadata_runtime_cover_traffic_cost_quota_retention_prerequisite_plan.md`
- `tests/NA-0333_metadata_runtime_cover_traffic_cost_quota_retention_prerequisite_testplan.md`
- `docs/governance/evidence/NA-0332_metadata_runtime_cover_traffic_risk_gate_deferred_authorization.md`
- `docs/governance/evidence/NA-0331_metadata_runtime_qshield_demo_batching_harness.md`
- `apps/qshield-cli/src/commands/send.rs`
- `apps/qshield-cli/src/commands/recv.rs`
- `apps/qshield-cli/src/commands/relay.rs`
- `apps/qshield-cli/src/relay_client.rs`
- `apps/qshield-cli/tests/na_0331_metadata_runtime_batching.rs`
- recent metadata-runtime governance/evidence, decisions, traceability, and
  qshield harness outputs.

Search coverage included cover traffic, prototype, dummy traffic, synthetic
cover, batch-fill, active-session cover, quota, retention, backup, abuse, DoS,
cost, deployment, qshield demo, qsl-server, qsl-attachments, production,
timing hidden, traffic hidden, metadata-free, anonymity, untraceable,
`FUTURE_GATE`, and `NOT_READY`.

## Prototype Scope / Mode Decision

Decision: authorize a future NA-0335 qshield demo-only cover-traffic prototype
implementation harness if it inherits the NA-0333 caps exactly or narrows them.

Authorized future modes:

1. qshield demo synthetic local cover.
   - May create bounded local/demo cover candidates to test classification,
     quota accounting, purge, retention, artifact, and operator-stop behavior.
   - Must not require service deployment or public-internet behavior.

2. qshield demo active-session cover.
   - May create bounded cover only while a local/demo session has real
     activity.
   - Must not turn idle periods into fixed-rate traffic and must not generate
     recursive cover.

3. qshield demo batch-fill cover.
   - May fill qshield demo batches up to the existing NA-0331 maximum of four
     members.
   - Must preserve send/receive/ack validation, no remote delete before local
     verification, retry-cadence bounds, bounded-jitter bounds, and all
     NA-0333 cover caps.

Rejected or deferred modes:

4. qshield demo fixed-rate cover: rejected for NA-0335 because continuous
   cover is higher cost, easier to overclaim, and not necessary for the first
   bounded prototype.
5. qsl-server production relay cover: cross-repo gated and not authorized.
6. qsl-attachments production object cover: cross-repo gated and not
   authorized.
7. no prototype / defer: rejected as the immediate successor because NA-0333
   made the qshield demo-only caps concrete enough for one tightly bounded
   harness.

Rationale:

- qshield demo synthetic, active-session, and batch-fill modes provide useful
  executable evidence about classification, quota, purge, and artifact safety;
- each selected mode can stay inside the embedded relay/demo surface already
  used by NA-0318 through NA-0331;
- the first harness can be deterministic and CI-bounded;
- production-service relevance remains limited, so the result must not be used
  as qsl-server or qsl-attachments proof;
- claim safety remains manageable because the authorization explicitly forbids
  stronger privacy/readiness language.

## Future Implementation Boundary

Future NA-0335 allowed qshield files only if the future directive confirms this
same boundary:

- `apps/qshield-cli/src/commands/send.rs`
- `apps/qshield-cli/src/commands/recv.rs`
- `apps/qshield-cli/src/commands/relay.rs`
- `apps/qshield-cli/src/relay_client.rs`
- `apps/qshield-cli/tests/na_0335_metadata_runtime_cover_traffic_prototype.rs`
- optional qshield test helper paths only if already established and named by
  the future directive.

Potential future fixture or script paths only if exact future scope permits:

- `inputs/metadata_runtime/cover_traffic_prototype_fixture_v1.json`
- `scripts/ci/metadata_runtime_cover_traffic_prototype_harness.sh`

Forbidden future files without new authorization:

- `qsl-server/**`
- `qsl-attachments/**`
- `qsc/**`
- `qsp/**`
- protocol, crypto, state-machine, or key-schedule paths;
- `Cargo.toml` or `Cargo.lock`;
- `.github/**`;
- `website/**`;
- `README.md`;
- `START_HERE.md`;
- `qsc-desktop/**`;
- branch-protection or public-safety configuration.

Future implementation proof requirements:

- no real user message corrupted;
- cover items are locally distinguishable without secret leaks;
- cover items obey quota and cost caps;
- cover items purge by TTL, cap, failure, or run end;
- backup impact remains bounded and recorded;
- cover artifacts and logs are secret-free;
- batching, retry, and jitter semantics remain bounded;
- qsl-server and qsl-attachments production boundary remains explicit;
- no claim says timing metadata is hidden;
- no claim says traffic shape is hidden;
- no claim says cover traffic exists outside the authorized qshield demo
  prototype.

NA-0334 does not create these files and does not implement NA-0335.

## Future Cost / Quota / Retention / Abuse Boundary

| Scope | Cap | Enforced by | Future test | Stop condition | Backup impact | Claim boundary |
| --- | ---: | --- | --- | --- | --- | --- |
| Payload per cover item | 8192 bytes max | pre-generation estimate and relay reject | oversize cover rejects before enqueue | planned item exceeds cap | no retained payload beyond bounded artifacts | size cap only |
| Cover items per run or local day | 64 max | run ledger/counter | 65th item rejects | counter would exceed cap | no durable growth beyond cap | resource cap only |
| Cover items per minute | 4 max | local time-window counter | fifth item in minute rejects | window full | no backup effect | not a timing-hiding proof |
| Cover items per hour | 32 max | local time-window counter | 33rd item in hour rejects | window full | no backup effect | not a traffic-shape proof |
| Cover payload per run | 512 KiB max | byte estimate and byte counter | byte boundary rejects | payload estimate exceeds cap | bounded temporary artifacts | payload cap only |
| Estimated request bytes per run | about 1 MiB max | conservative HTTP/JSON estimator | request estimate boundary rejects | estimate exceeds cap | no service backup impact | cost cap only |
| Queued cover items global | 16 max | relay/demo queue accounting | 17th queued cover rejects | queue would exceed cap | no durable service queue | local/demo only |
| Queued cover items per route | 4 max | route counter | fifth route cover rejects | route cover queue full | no durable service queue | local/demo only |
| Retained cover artifacts | 4 files max | artifact ledger | fifth artifact purges or rejects | artifact cap would exceed | 1 MiB retained total max | evidence cap only |
| Retained bytes | 1 MiB max | artifact byte counter | retained-byte boundary rejects | retained bytes exceed cap | no backup-plan update for qsl-protocol-only temp proof | not service storage proof |
| TTL | 10 minutes max | cleanup task or run-end purge | stale cover purged | stale item exceeds TTL | no restore requirement | lifecycle cap only |
| Purge triggers | ack/delete, failed verification, stale TTL, cap exhaustion, operator stop, run end, serialization failure, redaction failure | local cleanup procedure | each trigger removes only cover-classified artifacts | cleanup cannot prove cover-only deletion | stop and recommend backup review if durable artifacts appear | no claim of hidden metadata |
| Low disk threshold | `/srv/qbuild` free disk below 10 GiB | pre-generation disk check | forced low-threshold test or mocked estimator | abort before generation | backup-plan update if new durable location is needed | ops safety only |
| Abuse stop | any cap, recursive cover, invalid cover, public/service target, or secret artifact finding | fail-closed runtime and harness checks | abuse scenarios reject without real-message mutation | abuse path cannot be bounded | no service backup impact | not public-internet proof |
| Deterministic test mode | 8 cover items, 64 KiB payload, 128 KiB request estimate, no real sleeps beyond existing qshield bounded waits | explicit test-mode env or fixture | marker proof | test mode missing in CI | small temp artifacts only | CI proof only |
| Production default | zero production cover | absence of service calls and forbidden path guard | no qsl-server/qsl-attachments path touched | any production-service behavior needed | stop for cross-repo authorization | not production proof |
| Public-internet default | zero public-internet behavior | loopback/demo-only checks | non-loopback/default rejects preserved | public target needed | stop for new authorization | not public readiness |

Any future NA-0335 implementation must drop or refuse cover before real
messages are affected. Recursive cover generation is forbidden. Invalid cover
interactions must count against the same or stricter budgets as existing
qshield demo invalid-candidate behavior.

## Future Validation / Marker Plan

Required future NA-0335 markers if the prototype is implemented:

- `NA0335_COVER_TRAFFIC_PROTOTYPE_AUTHORIZATION_OK`
- `NA0335_QSHIELD_DEMO_COVER_TRAFFIC_POLICY_OK`
- `NA0335_COVER_ITEM_QUOTA_BOUNDARY_OK`
- `NA0335_COVER_ITEM_RETENTION_BOUNDARY_OK`
- `NA0335_COVER_ITEM_PURGE_BOUNDARY_OK`
- `NA0335_COVER_ITEM_BACKUP_BOUNDARY_OK`
- `NA0335_COVER_ITEM_ABUSE_BOUNDARY_OK`
- `NA0335_COVER_ITEM_SECRET_FREE_ARTIFACT_OK`
- `NA0335_BATCHING_RETRY_JITTER_STILL_BOUNDED_OK`
- `NA0335_NO_PRODUCTION_COVER_TRAFFIC_OK`
- `NA0335_QSHIELD_DEMO_BOUNDARY_OK`
- `NA0335_SERVICE_PRODUCTION_BOUNDARY_OK`
- `NA0335_NO_TIMING_HIDDEN_CLAIM_OK`
- `NA0335_NO_TRAFFIC_SHAPE_HIDDEN_CLAIM_OK`
- `NA0335_NO_METADATA_FREE_CLAIM_OK`
- `NA0335_NO_ANONYMITY_CLAIM_OK`

If NA-0335 stops instead of implementing, it must emit or document:

- `NA0335_COVER_TRAFFIC_DEFERRED_GATE_OK`
- `NA0335_NO_COVER_TRAFFIC_IMPLEMENTATION_OK`
- `NA0335_QSHIELD_DEMO_BOUNDARY_OK`
- `NA0335_SERVICE_PRODUCTION_BOUNDARY_OK`
- `NA0335_NO_TIMING_HIDDEN_CLAIM_OK`
- `NA0335_NO_TRAFFIC_SHAPE_HIDDEN_CLAIM_OK`
- `NA0335_NO_METADATA_FREE_CLAIM_OK`
- `NA0335_NO_ANONYMITY_CLAIM_OK`

Required future negative proof:

- oversize cover item rejects before enqueue;
- item, byte, minute, hour, run, route, global queue, retained artifact, and
  low-disk boundaries reject before generation or enqueue;
- invalid cover does not delete or mutate real messages;
- cover ack/delete and purge cannot remove unverified real candidates;
- logs and artifacts have secret-finding count zero;
- default behavior stays no-cover unless explicit qshield demo opt-in is set;
- qsl-server and qsl-attachments paths remain untouched.

## Production Boundary

qshield embedded relay/demo cover traffic is local/demo evidence only. It may
study synthetic local cover shape, active-session cover boundaries, batch-fill
cover boundaries, marker safety, cost accounting, quotas, retention, purge,
and artifact redaction. It does not prove production service behavior.

qsl-server production cover traffic requires cross-repo authorization before
any implementation, prototype, deployment, queue, quota, logging, monitoring,
rate-limit, rollback, or operator-cost behavior changes.

qsl-attachments production object cover requires cross-repo authorization
before any implementation, prototype, object creation, object padding, upload
cadence, fetch cadence, retention, purge, backup, or recovery behavior
changes.

Public-internet timing and traffic-shape behavior remains `FUTURE_GATE`.

## External-Review Sensitivity

Cover traffic is externally review sensitive because bounded local/demo cover
can be mistaken for broader privacy proof. External review is recommended
before any language stronger than bounded experimental mitigation for a
specific measured local/demo class.

Review should cover:

- threat model fit and non-goals;
- traffic-analysis assumptions;
- mode selection and excluded modes;
- cost and quota model;
- adversarial amplification model;
- service logging and operator visibility;
- retention, purge, backup, and restore economics;
- deployment and rollback plan;
- public claim wording.

No external-review-complete claim is made by NA-0334.

## Public Claim Boundary

Allowed wording:

- NA-0334 authorizes a future bounded qshield demo-only prototype harness;
- NA-0335 must inherit or narrow NA-0333 caps;
- qshield demo cover remains local/demo only;
- qsl-server and qsl-attachments production cover remain cross-repo gated;
- timing and traffic-shape gaps remain explicit;
- external review is recommended before stronger privacy language.

Prohibited wording:

- cover traffic is implemented by NA-0334;
- a cover traffic prototype exists after NA-0334;
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

`NA-0335 -- Metadata Runtime qshield Demo Cover Traffic Prototype Implementation Harness`

Rationale:

- NA-0333 made the prerequisite caps concrete enough for a tightly bounded
  qshield demo-only harness;
- NA-0334 narrows the authorized modes to synthetic local, active-session, and
  batch-fill cover only;
- fixed-rate and production cover remain excluded;
- future files and markers are exact enough to make scope guardable;
- a future implementation harness can produce executable evidence while
  keeping all production-service and public/privacy boundaries visible.

## Rejected Alternatives

- `NA-0335 -- Metadata Runtime Cover Traffic Prototype Blocker Resolution`:
  rejected as the immediate successor because no current blocker prevents the
  narrow qshield demo-only harness if NA-0333 caps are inherited exactly.
- `NA-0335 -- Metadata Runtime Padding Bucket Expansion Authorization Plan`:
  rejected as the immediate successor because it would not close the
  cover-traffic prototype question opened by NA-0332 and bounded by NA-0333.
- `NA-0335 -- Metadata Runtime Service Timing Cross-Repo Authorization`:
  deferred because production-service timing should not be promoted before the
  local/demo prototype proves classification, quota, purge, and artifact
  boundaries.
- `NA-0335 -- Metadata Runtime Cover Traffic Deferred Gate Closeout`: rejected
  because the narrow qshield demo-only implementation harness is now
  sufficiently bounded.
- Direct fixed-rate cover: rejected for cost, abuse, and overclaim risk.
- Direct qsl-server production cover: rejected for this qsl-protocol-only lane
  and cross-repo gated.
- Direct qsl-attachments production object cover: rejected for this
  qsl-protocol-only lane and cross-repo gated.
- Claiming metadata-free behavior: rejected as unsupported.

## Backup-Plan Impact Statement

NA-0334 changes only qsl-protocol governance/evidence/testplan/traceability
and journal files under `/srv/qbuild/work`.

No backup-plan update is required for NA-0334.

Future NA-0335 must revisit backup-plan impact if it creates durable
non-rebuildable artifacts, durable service logs, qsl-server storage growth,
qsl-attachments object growth, retained cover payloads, backup exclusions, or
durable evidence outside the current qbuild scope. Any such need is a stop
condition unless the future directive explicitly updates the backup plan.

## Next Recommendation

After NA-0334 merges and post-merge public-safety is green, close NA-0334 and
restore exactly one successor:

`NA-0335 -- Metadata Runtime qshield Demo Cover Traffic Prototype Implementation Harness`

NA-0335 must not implement production cover traffic, qsl-server behavior,
qsl-attachments behavior, fixed-rate cover, public-internet behavior, or any
unsupported public/privacy claim.
