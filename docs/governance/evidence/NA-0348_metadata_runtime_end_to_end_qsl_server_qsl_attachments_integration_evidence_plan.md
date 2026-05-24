Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-24

# NA-0348 Metadata Runtime End-to-End qsl-server / qsl-attachments Integration Evidence Plan

Goals: G1, G2, G3, G4, G5

## Executive summary

NA-0348 records the end-to-end qsl-server / qsl-attachments evidence plan after
the bounded service-local proofs from qsl-server PR #55 and qsl-attachments PR
#37.

Result: `NA0348_END_TO_END_EVIDENCE_PLAN_RECORDED`.

Read-only refresh found no source, authority, CI, or open-PR blocker that would
force a blocker successor. qsl-server remote `main` is `b194a95b19a7`, PR #55
is merged, required `rust` CI is green, branch protection is present, and no
open qsl-server PRs were listed. qsl-attachments remote `main` is
`96b9352bd63e`, PR #37 is merged, required `rust` CI is green, branch
protection is present, and no open qsl-attachments PRs were listed.

Selected successor:

`NA-0349 -- Metadata Runtime End-to-End qsl-server / qsl-attachments Integration Implementation Harness`

NA-0348 is an evidence-planning lane only. It does not implement qsl-server,
qsl-attachments, qshield runtime, qsc, qsp, protocol, crypto, deployment,
public ingress, website copy, or public claims.

## Live NA-0348 scope

The live `NEXT_ACTIONS.md` entry requires NA-0348 to:

- refresh qsl-server and qsl-attachments source freshness, mutation authority,
  CI authority, branch protection, open PR state, required checks, and SHA
  status from live state;
- define the exact end-to-end qsl-server / qsl-attachments integration evidence
  plan, including route/API, storage/proxy, quota/rate, retention/purge,
  backup, deploy/rollback, secrets/env, public-ingress, CI, artifact, and
  public-claim boundaries;
- decide whether end-to-end integration evidence can be authorized next or
  whether blocker resolution, backup/deploy/rollback hardening, public-claim
  audit, or external-review gap audit must come first;
- preserve all public-claim constraints and avoid stronger privacy/readiness
  language.

Protected boundaries:

- no unsupported production, public-internet, external-review, anonymity,
  metadata-free, or untraceable claim;
- no claim that attachment size, timing metadata, traffic shape, or all
  metadata is hidden;
- qsl-server/qsl-attachments production boundary remains explicit;
- qsl-attachments PR #37 remains service-local prerequisite evidence only;
- qshield embedded relay/demo proof remains reference/oracle evidence only;
- no NA-0348 implementation is authorized by NA-0347 closeout.

## Inherited NA-0347 qsl-server integration proof

qsl-server PR #55 merged as `b194a95b19a7` from validated head
`e8b2de95426c`. It changed only:

- `tests/qsl_attachments_integration_contract.rs`

The harness proves the conservative qsl-server integration boundary:

- qsl-server accepts and returns qsl-attachments-shaped opaque bytes without
  parsing, storing durable qsl-attachments objects, or calling qsl-attachments
  APIs;
- missing route-token rejects before mutation;
- route isolation and exact byte delivery are preserved;
- body-size, route-cap, and rate controls fail closed;
- idle route TTL purges queued data;
- test listener is loopback only;
- route-token, bearer-token, and payload sentinels do not appear in captured
  logs;
- public-claim phrase scans reject unsupported claims.

Markers inherited from the qsl-server harness include:

- `NA0347_QSL_SERVER_ROUTE_BOUNDARY_OK`
- `NA0347_QSL_SERVER_STORAGE_BOUNDARY_OK`
- `NA0347_QSL_SERVER_QUOTA_BOUNDARY_OK`
- `NA0347_QSL_SERVER_RETENTION_PURGE_BOUNDARY_OK`
- `NA0347_QSL_SERVER_BACKUP_BOUNDARY_OK`
- `NA0347_QSL_SERVER_SECRET_ENV_BOUNDARY_OK`
- `NA0347_QSL_SERVER_DEPLOY_ROLLBACK_BOUNDARY_OK`
- `NA0347_QSL_SERVER_PUBLIC_INGRESS_BOUNDARY_OK`
- `NA0347_QSL_ATTACHMENTS_SERVICE_LOCAL_BOUNDARY_OK`
- `NA0347_QSHIELD_DEMO_REFERENCE_BOUNDARY_OK`

Limitations:

- no qsl-attachments API is exercised by qsl-server;
- no deployed qsl-server instance is exercised;
- no public-ingress path is exercised;
- no monitoring, alerting, backup restore, deploy, or rollback system is
  exercised;
- no claim is made that attachment size, timing metadata, traffic shape, or all
  metadata is hidden.

## Inherited NA-0344 qsl-attachments service-local proof

qsl-attachments PR #37 merged as `96b9352bd63e` from validated head
`7e6d82570b7d`. It changed:

- `src/lib.rs`
- `tests/production_size_class_policy.rs`

The service-local harness proves:

- opt-in policy `qsl_attachments_production_size_class_v1`;
- deterministic size-class table with qshield-compatible small-class prefix;
- invalid config and oversize rejects;
- malformed size-class metadata reject during startup reconciliation;
- valid small, medium, and large object handling;
- exact opaque ciphertext fetch behavior;
- retention/purge behavior;
- cold full-root backup/restore behavior in the harness;
- no raw fetch capability or resume token in object JSON;
- qsl-server route path is not part of the qsl-attachments service-local API.

Limitations:

- no qsl-server handoff is exercised;
- no public qsl-attachments deployment is exercised;
- no production backup system is exercised;
- no service-to-service auth, monitoring, alerting, ingress, deploy, or
  rollback path is exercised;
- the size-class policy does not hide exact size from every observer and does
  not hide timing metadata or traffic shape.

## qshield demo reference proof

The qshield embedded relay/demo lanes remain reference/oracle evidence only.
NA-0339 proved an opt-in qshield demo attachment ciphertext object size-class
harness with policy `qshield_demo_attachment_size_class_v1`. The descriptor
remains separately encrypted and metadata-bearing, including exact unpadded
ciphertext length and hash for receive-side validation.

Limitations:

- qshield embedded relay/demo proof is not qsl-server production proof;
- qshield embedded relay/demo proof is not qsl-attachments production proof;
- qshield embedded relay/demo proof does not authorize public claims;
- qshield embedded relay/demo proof does not prove production deploy, backup,
  monitoring, public ingress, or external review.

## Source / authority / CI refresh

### qsl-server

| Field | Result |
| --- | --- |
| repository | `QuantumShieldLabs/qsl-server` |
| default branch | `main` |
| remote `HEAD` / `main` | `b194a95b19a7` |
| PR #55 | merged, merge `b194a95b19a7` |
| local path inspected | `/srv/qbuild/work/NA-0237D/qsl-server` |
| local worktree | clean at PR head `e8b2de95426c` |
| local note | validated PR-head worktree; future mutation must refresh from live `main` |
| viewer permission | `ADMIN` |
| branch protection | present |
| required check | strict `rust` |
| force pushes | disabled |
| deletions | disabled |
| admins enforced | enabled |
| open PRs | none listed |
| latest listed `main` CI | `ci` success on `b194a95b19a7` |
| source classification | `FRESH_SOURCE` for live remote/default source |
| authority classification | `COMPLETE_AUTHORITY` |
| CI classification | `COMPLETE_CI` |

### qsl-attachments

| Field | Result |
| --- | --- |
| repository | `QuantumShieldLabs/qsl-attachments` |
| default branch | `main` |
| remote `HEAD` / `main` | `96b9352bd63e` |
| PR #37 | merged, merge `96b9352bd63e` |
| local path inspected | `/srv/qbuild/work/NA-0237D/qsl-attachments` |
| local worktree | clean detached HEAD at `96b9352bd63e` |
| viewer permission | `ADMIN` |
| branch protection | present |
| required check | strict `rust` |
| force pushes | disabled |
| deletions | disabled |
| admins enforced | not enabled on current protection |
| open PRs | none listed |
| latest listed `main` CI | `rust` success on `96b9352bd63e` |
| source classification | `FRESH_SOURCE` |
| authority classification | `COMPLETE_AUTHORITY` |
| CI classification | `COMPLETE_CI` |

## Inherited proof inventory

| Source | Proven | Evidence | Limitations |
| --- | --- | --- | --- |
| qsl-server PR #55 | Opaque qsl-attachments-shaped payload relay through qsl-server route/API; fail-closed route-token, quota/rate, retention/purge, loopback-only, log-redaction, and claim-scan bounds | `tests/qsl_attachments_integration_contract.rs`, qsl-server `ci` success | Does not call qsl-attachments, does not deploy, does not prove public ingress, monitoring, backup restore, or cross-service production behavior |
| qsl-attachments PR #37 | Opt-in production size-class service-local policy, exact opaque ciphertext storage/fetch, retention/purge, cold full-root backup harness, malformed metadata reject | `src/lib.rs`, `tests/production_size_class_policy.rs`, qsl-attachments `rust` success | Does not exercise qsl-server handoff, public ingress, production backup, monitoring, deploy, or rollback |
| qshield demo NA-0339 | Demo-only attachment ciphertext object size-class reference and receive-side verification | `apps/qshield-cli/tests/na_0339_metadata_runtime_attachment_size_class.rs` | Not production service proof; descriptor remains metadata-bearing |
| qsl-protocol governance PRs #950/#956 | Service-local evidence captured with claim boundaries and successor sequencing | NA-0344 and NA-0347 evidence/testplans, D-0670, D-0676 | Governance evidence is not runtime implementation |

## End-to-end threat / value model

True end-to-end qsl-server / qsl-attachments evidence should prove a bounded
client-to-service path in which qsl-server and qsl-attachments source versions,
auth boundaries, object lifecycle, route/API handoff, opaque payload treatment,
retention/purge behavior, backup/restore assumptions, logs/redaction, and
operator-visible status are tested together.

Observer surfaces that must remain explicit:

- qsl-server route identifiers, route-token auth outcomes, queue timing, retry
  cadence, and HTTP status behavior;
- qsl-attachments session/object identifiers, object size-class metadata,
  retention class, part count, and fetch/commit timing;
- network timing, traffic shape, request count, response status, and body-size
  class observations;
- logs, monitoring events, alert content, backup manifests, deployment
  commands, rollback paths, and operator runbook output.

Fail-closed value:

- missing, malformed, mismatched, expired, over-quota, over-size, unauthorized,
  stale, or partially restored objects must reject without accepted state drift;
- service logs must not include raw route tokens, bearer tokens, resume tokens,
  fetch capabilities, payload bytes, raw keys, or passphrases;
- deployment or public-ingress prerequisites must stop before claim expansion.

## End-to-end evidence gap matrix

| Area | Proven by current evidence? | Evidence source | Remaining gap | Risk | Future proof required | Status | Successor relation |
| --- | --- | --- | --- | --- | --- | --- | --- |
| source/authority freshness | Partial yes | live read-only refresh | future mutation must refresh again | stale base branch or authority drift | repeat source/authority/CI before mutation | ready | NA-0349 start gate |
| qsl-server route/API | Service-local yes | qsl-server PR #55 | no qsl-attachments live handoff | route proof may miss service coupling | end-to-end route/API harness | ready | NA-0349 |
| qsl-attachments object lifecycle | Service-local yes | qsl-attachments PR #37 | no qsl-server-origin object lifecycle | lifecycle drift at handoff | create/commit/fetch/purge across both services | ready | NA-0349 |
| qsl-server to qsl-attachments handoff | No | boundary docs only | no direct contract execution | integration mismatch | cross-service contract harness or exact stub boundary | ready | NA-0349 |
| object-size class propagation or opacity | Partial | qsl-attachments PR #37, qshield demo | no qsl-server observation proof | overclaim or leakage ambiguity | verify size-class metadata handling and visible observations | ready | NA-0349 |
| descriptor/ciphertext treatment | Partial | qsl-attachments and qshield demo | no qsl-server-visible descriptor/ciphertext flow | parsing or logging drift | exact descriptor/ciphertext pass-through and log scan | ready | NA-0349 |
| route-token/auth behavior | qsl-server yes | qsl-server PR #55 | no paired service auth policy | auth mismatch or secret leakage | route-token plus attachment capability negative tests | ready | NA-0349 |
| quota/rate/max body | qsl-server and qsl-attachments separately | PR #55, PR #37 | no combined cost envelope | unbounded cross-service resource path | combined max body, object max, rate, quota rejects | ready | NA-0349 |
| retention/purge consistency | separately yes | PR #55, PR #37 | no cross-service purge ordering proof | stale object or stale route reference | paired expiry/purge/late-fetch tests | ready | NA-0349 |
| backup/restore consistency | qsl-attachments harness only | PR #37 | no qsl-server queue plus qsl-attachments restore proof | inconsistent restore/runbook | cold restore boundary and explicit unsupported cases | ready | NA-0349 requirement |
| logs/redaction | partial | PR #55, PR #37 | no cross-service transcript scan | secret or payload leak | combined log/event artifact scan | ready | NA-0349 requirement |
| monitoring/alerts | No | none | no alert schema or operator status proof | hidden failures | bounded monitoring/logging evidence or explicit stop | ready | NA-0349 requirement |
| deploy/rollback | No | planning docs only | no deployment path exercised | unsafe rollout assumptions | deploy/rollback non-deploy boundary or prerequisite stop | ready with gate | NA-0349 requirement |
| public ingress | No | qsl-server loopback proof only | no public ingress behavior | public exposure overclaim | loopback-only proof or explicit public-ingress prerequisite | ready with gate | NA-0349 requirement |
| external review | No | governance caveats | review not complete | claim drift | external-review readiness remains future | not blocker for harness | deferred |
| website/public claims | No mutation | governance caveats | public copy must not change | claim drift | no public copy update unless future directive | not blocker | deferred |
| qshield demo reference | bounded yes | NA-0339 | not production proof | confusing demo with service proof | reference-only boundary marker | ready | NA-0349 marker |
| qsc/suite-id unaffected | yes for NA-0348 | scope proof | future implementation must avoid qsc drift | protocol scope creep | scope guard | ready | NA-0349 guard |
| CI/advisories/cost-control | yes for qsl-protocol | public-safety, cargo audit | future service CI must be refreshed | stale required checks | required qsl-server/qsl-attachments CI green | ready | NA-0349 gate |
| D132/history/backup operations | yes for NA-0348 | local guards | D132 cleanup not authorized | accidental cleanup | preserve bundle and record no backup-plan update | ready | NA-0349 guard |

## Future strategy options

| Option | Evaluation | Decision |
| --- | --- | --- |
| qsl-server-only end-to-end implementation harness | Feasible if it uses a deterministic qsl-attachments contract stub or fixture while proving qsl-server side of the handoff | recommended only if the future directive keeps stub boundaries explicit |
| paired qsl-server and qsl-attachments harness | Strongest behavioral evidence but higher coordination and CI cost | recommended if NA-0349 authorizes both repos and exact files |
| cross-repo integration test without production code changes | Good first implementation shape if existing APIs can be composed under tests | recommended as the preferred NA-0349 starting shape |
| contract vector or fixture | Useful to keep handoff deterministic and avoid secret/service dependencies | recommended as supporting evidence |
| backup/deploy/rollback prerequisite lane | Needed if NA-0349 discovers durable artifacts or deploy assumptions outside current backup scope | deferred unless NA-0349 hits that blocker |
| external-review readiness lane | Important before stronger public claims | deferred until executable service evidence exists |
| website/public-claim boundary lane | Useful only after evidence changes public-facing truth | deferred; no public docs change now |
| blocker continuation | Appropriate only if future refresh finds stale source, red CI, authority loss, or conflicting PRs | rejected for immediate successor because current refresh is green |

## Backup / deploy / rollback / secrets / monitoring plan

Future NA-0349 must record:

- qsl-server and qsl-attachments source roots, SHAs, branches, and merge
  strategy;
- whether test artifacts remain under `/srv/qbuild/work` or `/srv/qbuild/tmp`;
- whether any durable service data root, backup root, deploy config, or
  monitoring artifact is created outside current backup scope;
- exact non-secret environment variables and deterministic test credentials;
- explicit handling for route token, bearer token, resume token, fetch
  capability, object locator, attachment ID, descriptor, ciphertext, and
  payload sentinels;
- deploy boundary: no production deployment unless the future directive names
  the target, rollback, backup, monitoring, secrets, and public-ingress gates;
- rollback boundary: source revert/test-harness removal unless deploy behavior
  is separately authorized;
- monitoring/logging boundary: no raw tokens, payload bytes, raw keys,
  passphrases, or secret-bearing URLs in logs or artifacts;
- operator runbook boundary: unsupported states must be visible and fail closed.

NA-0348 creates only qsl-protocol governance/evidence files under the existing
qbuild worktree. No backup-plan update is required now.

## Public-ingress / timing / traffic-shape boundary

Current evidence does not prove public-ingress service behavior. The qsl-server
harness binds to loopback only. Future public-ingress work must be a separate
gate unless NA-0349 explicitly authorizes and proves a bounded non-public
integration harness.

Current evidence does not prove timing metadata is hidden. Current evidence
does not prove traffic shape is hidden. Current evidence does not prove
attachment size or all metadata is hidden. Size-class evidence only records a
bounded class policy and its visible limitations.

## External-review sensitivity

External review is not complete. End-to-end service integration is
review-sensitive because it combines route/API behavior, attachment object
lifecycle, auth, logging, retention, backup, and deployment assumptions. Any
stronger public or release claim requires service implementation evidence,
deployment evidence, monitoring/log evidence, backup/restore evidence, public
claim review, and external-review evidence.

## Future validation / marker / verification-bundle plan

Candidate NA-0349 markers:

- `NA0349_END_TO_END_SOURCE_AUTHORITY_OK`
- `NA0349_QSL_SERVER_MAIN_PROOF_OK`
- `NA0349_QSL_ATTACHMENTS_MAIN_PROOF_OK`
- `NA0349_QSL_SERVER_QSL_ATTACHMENTS_CONTRACT_OK`
- `NA0349_ROUTE_API_BOUNDARY_OK`
- `NA0349_ATTACHMENT_OBJECT_LIFECYCLE_OK`
- `NA0349_SIZE_CLASS_FLOW_BOUNDARY_OK`
- `NA0349_OPAQUE_PAYLOAD_BOUNDARY_OK`
- `NA0349_ROUTE_TOKEN_AUTH_BOUNDARY_OK`
- `NA0349_QUOTA_RATE_BOUNDARY_OK`
- `NA0349_RETENTION_PURGE_CONSISTENCY_OK`
- `NA0349_BACKUP_RESTORE_BOUNDARY_OK`
- `NA0349_LOG_REDACTION_BOUNDARY_OK`
- `NA0349_MONITORING_BOUNDARY_OK`
- `NA0349_DEPLOY_ROLLBACK_BOUNDARY_OK`
- `NA0349_PUBLIC_INGRESS_BOUNDARY_OK`
- `NA0349_QSHIELD_DEMO_REFERENCE_BOUNDARY_OK`
- `NA0349_NO_ATTACHMENT_SIZE_HIDDEN_CLAIM_OK`
- `NA0349_NO_TIMING_HIDDEN_CLAIM_OK`
- `NA0349_NO_TRAFFIC_SHAPE_HIDDEN_CLAIM_OK`
- `NA0349_NO_METADATA_FREE_CLAIM_OK`

If implementation is blocked, NA-0349 should instead emit blocker markers that
name the exact failed source, authority, CI, backup, deploy, rollback, secrets,
monitoring, public-ingress, or claim-boundary prerequisite.

## Public claim boundary

NA-0348 does not update public docs or website copy. Future evidence must not
state or imply:

- attachment size is hidden;
- timing metadata is hidden;
- traffic shape is hidden;
- all metadata is hidden;
- the system is metadata-free, anonymous, or untraceable;
- production readiness or public-internet readiness is achieved;
- external review is complete;
- padding hides all metadata.

Service-local qsl-server and qsl-attachments evidence is not enough for public
claims. qshield demo evidence remains reference/oracle evidence only.

## Selected successor

Selected:

`NA-0349 -- Metadata Runtime End-to-End qsl-server / qsl-attachments Integration Implementation Harness`

Rationale:

- qsl-server and qsl-attachments live source/authority/CI refresh is green;
- no blocking open PRs were listed;
- qsl-server PR #55 and qsl-attachments PR #37 provide the immediate
  service-local prerequisites;
- the next truthful step is executable end-to-end integration evidence, with
  explicit stop gates if future refresh changes.

## Rejected alternatives

- `NA-0349 -- Metadata Runtime End-to-End Integration Blocker Resolution`:
  rejected because current refresh found no live blocker.
- `NA-0349 -- Metadata Runtime Production Backup / Deploy / Rollback
  Prerequisite Plan`: deferred because implementation evidence can first prove
  or stop on exact backup/deploy/rollback gates.
- `NA-0349 -- Metadata Runtime External Review Readiness Gap Audit`: deferred
  until cross-service evidence exists.
- `NA-0349 -- Metadata Runtime Website / Public Claim Boundary Audit`: deferred
  because NA-0348 makes no public-doc or website claim change.

## Backup-plan impact statement

No backup-plan update is required for NA-0348. Changed files are qsl-protocol
governance/evidence/testplan files under `/srv/qbuild/work`, which is already
within the current qbuild working area. Future NA-0349 must stop or update the
backup plan if it creates non-rebuildable service artifacts, durable evidence,
service data roots, deploy configuration, monitoring exports, or backup roots
outside current backup scope.

## Next recommendation

Proceed to NA-0349 only after Packet N merges and closeout restores the exact
successor. NA-0349 should begin with a fresh source/authority/CI refresh for
both service repos and should prefer a cross-repo integration test that proves
the handoff without production deployment. If either repo has stale source,
red required CI, insufficient authority, or conflicting open PRs, NA-0349 must
stop and select the exact blocker path.
