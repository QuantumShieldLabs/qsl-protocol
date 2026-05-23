Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-23

# NA-0345 Metadata Runtime qsl-server Integration Boundary Plan

## Executive summary

NA-0345 records the qsl-server integration boundary after NA-0344 delivered
bounded qsl-attachments service-local size-class proof.

Result: `QSL_SERVER_SOURCE_AUTHORITY_COMPLETE_BOUNDARY_PLAN_RECORDED`.

The strongest local qsl-server source found is
`/srv/qbuild/work/NA-0237D/qsl-server` at `3f28d7433e74`. Read-only remote
evidence showed live remote `main` also at `3f28d7433e74`, viewer permission
`ADMIN`, protected `main`, strict required check `rust`, no open PRs, and latest
listed `main` CI success. That is sufficient source/authority evidence for a
future implementation-authorization planning lane.

This plan does not implement qsl-server, qsl-attachments, qshield runtime,
qsc/qsp/protocol behavior, deployment, public ingress, timing mitigation,
traffic-shape mitigation, or public-copy changes. The selected successor is:

`NA-0346 -- Metadata Runtime qsl-server Integration Implementation Authorization Plan`

## Live NA-0345 scope

The live `NEXT_ACTIONS.md` entry on `origin/main` required NA-0345 to:

- execute the qsl-server integration boundary planning lane selected by
  NA-0344, or stop with exact prerequisite evidence;
- re-verify qsl-server source freshness, mutation authority, CI authority,
  branch protection, open PR state, and required checks from live remote;
- map qsl-server integration boundaries after qsl-attachments size-class
  handling, including routing, storage, timing, transport, and public ingress;
- decide the exact next successor;
- keep qsl-attachments PR #37 and qsl-protocol PR #950 evidence linked without
  stronger privacy or readiness claims.

The live scope forbade:

- unsupported production, public-internet, external-review, anonymity,
  metadata-free, or untraceable claims;
- any claim that attachment size, timing metadata, or traffic shape is hidden
  unless exact future evidence proves it;
- presenting qsl-attachments service-local proof as qsl-server production
  proof;
- presenting qshield embedded relay/demo proof as production proof;
- qsl-server mutation until a future exact directive defines files, CI,
  rollback, deploy, backup, qsl-attachments integration, and public-claim
  boundaries.

## Inherited NA-0344 qsl-attachments service-local proof

NA-0344 merged qsl-attachments PR #37 as `96b9352bd63e` and qsl-protocol
companion PR #950 as `e269b9a94159`. qsl-protocol closeout PR #951 then
restored NA-0345 as READY at `013d3525ff24`.

Inherited qsl-attachments proof:

- opt-in policy `qsl_attachments_production_size_class_v1`;
- qshield-compatible small classes through `8192` bytes;
- 8 KiB classes through 1 MiB and 1 MiB classes through the configured max;
- invalid-config and oversize rejection;
- malformed persisted metadata rejection during startup reconciliation;
- valid small, medium, and large object proof;
- retention/purge proof;
- cold full-root backup boundary proof;
- no-secret-artifact proof;
- qsl-server boundary proof;
- qshield demo small-class compatibility proof.

Boundary preserved: this is qsl-attachments service-local evidence. It is not
qsl-server integration proof, public-ingress proof, deployment proof, timing
proof, traffic-shape proof, external-review proof, or a claim that attachment
size or all metadata is hidden.

## Sources inspected

qsl-protocol sources inspected:

- `NEXT_ACTIONS.md`;
- `DECISIONS.md`;
- `TRACEABILITY.md`;
- `tests/NA-0344_closeout_restore_na0345_testplan.md`;
- `docs/governance/evidence/NA-0344_metadata_runtime_qsl_attachments_production_size_class_implementation_harness.md`;
- `tests/NA-0344_metadata_runtime_qsl_attachments_production_size_class_implementation_harness_testplan.md`;
- `docs/governance/evidence/NA-0343_metadata_runtime_qsl_attachments_production_size_class_implementation_authorization.md`;
- `docs/governance/evidence/NA-0342_metadata_runtime_qsl_attachments_source_authority_blocker_resolution.md`;
- `docs/governance/evidence/NA-0339_metadata_runtime_qshield_demo_attachment_size_class_harness.md`;
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.

qsl-server local paths inspected read-only:

- `/srv/qbuild/work/NA-0237D/qsl-server`;
- `/srv/qbuild/work/NA-0237C/qsl-server`;
- `/srv/qbuild/work/NA-0237/qsl-server`;
- `/srv/qbuild/work/NA-0237A/qsl-server`;
- `/srv/qbuild/work/NA-0237B/qsl-server`.

qsl-server source files and docs inspected read-only included:

- `.github/workflows/ci.yml`;
- `Cargo.toml`;
- `src/lib.rs`;
- `src/main.rs`;
- `README.md`;
- `docs/server/DOC-SRV-001_Deployment_Hardening_Contract_v1.0.0_DRAFT.md`;
- `docs/server/DOC-SRV-003_Relay_Inbox_Contract_v1.0.0_DRAFT.md`;
- `docs/server/DOC-SRV-004_Relay_Auth_And_Hardening_Contract_v1.0.0_DRAFT.md`;
- `docs/server/DOC-SRV-005_Route_Token_API_Shape_Review_v1.0.0_DRAFT.md`;
- `packaging/runbook_ubuntu.md`;
- `packaging/systemd/relay.env.example`;
- `packaging/caddy/Caddyfile.example`;
- current qsl-server tests for config, auth reject, idempotency, rate/global
  caps, route lifecycle TTL, abuse/queue, and log-redaction boundaries.

No qsl-server checkout was fetched, cloned, checked out, branched, committed,
pushed, merged, rebased, dependency-installed, built, tested, deployed, or
mutated.

## qsl-server source/authority/local availability inventory

| Field | Result |
| --- | --- |
| selected local path | `/srv/qbuild/work/NA-0237D/qsl-server` |
| exists | yes |
| repository | `QuantumShieldLabs/qsl-server` |
| remote URL | `https://github.com/QuantumShieldLabs/qsl-server.git` |
| current HEAD | `3f28d7433e74` |
| current branch/ref | `main` |
| detached | no |
| local worktree clean | yes |
| local `origin/main` present | yes, `3f28d7433e74` |
| live remote `main` | `3f28d7433e74` |
| remote freshness known | yes |
| viewer permission | `ADMIN` |
| branch protection known | yes |
| required checks known | yes, strict `rust` |
| force pushes | disabled |
| deletions | disabled |
| admins enforced | enabled |
| open PRs known | yes, none listed |
| latest listed `main` CI | `ci` success on `3f28d7433e74` |
| build/test commands known | yes |
| CI entrypoints known | yes |
| secrets/env required | yes for optional relay token; exact future deployment values required before deployment |
| deploy/rollback docs known | yes, packaging/runbook and scripts exist |
| qsl-attachments integration points known | no direct implementation point found; future integration must define it |
| backup impact known | partial; deploy artifacts/backups are documented, production data/artifact scope needs future review |
| conclusion | `COMPLETE_SOURCE_AUTHORITY` for future authorization planning |

Other local qsl-server checkouts were clean historical `main` worktrees at
`0826ffa4d6f3`. They are not selected because the newer NA-0237D worktree
matches live remote `main`.

## qsl-server integration threat/value model

Future qsl-server integration might need to handle:

- qsl-attachments object-size metadata flow or opaque pass-through;
- upload/fetch routing or attachment-object handoff;
- public ingress exposure through reverse proxy or edge controls;
- service logs, including route identifiers, message identifiers, sizes, and
  reject reasons;
- timing observation from push/pull cadence and queue behavior;
- traffic-shape observation from request count, response size, status cadence,
  queue depth, and public-ingress logs;
- rate limits, quotas, and route caps;
- storage/proxy behavior for in-memory relay state and any future attachment
  object handoff;
- qsl-attachments API compatibility and failure-mode mapping.

It does not solve:

- endpoint compromise;
- production log leakage unless exact redaction and operational checks prove
  the deployment behavior;
- route/contact relationship leakage;
- content/key compromise;
- public-internet anonymity;
- broad metadata-free behavior;
- hiding attachment size, timing metadata, traffic shape, or all metadata.

Hazards:

- public ingress abuse and cost growth;
- route-token, auth-header, or message-id leakage in logs and support bundles;
- routing compatibility drift between qsl-server and qsl-attachments;
- storage amplification if attachment objects or metadata enter qsl-server;
- retention/purge mismatch across qsl-server and qsl-attachments;
- backup growth and restore ambiguity;
- deployment and rollback drift;
- stronger public-claim wording than the evidence supports.

Evidence needed before implementation:

- exact qsl-server branch/base/allowed files;
- exact qsl-attachments integration contract;
- exact routing, storage, retention, purge, and backup semantics;
- qsl-server local and CI checks;
- deploy/rollback non-deployment or deployment boundary;
- secrets/env and log-redaction boundary;
- public-ingress model;
- public-claim and external-review boundary.

## qsl-server integration boundary design

| Boundary | Classification | Required future evidence |
| --- | --- | --- |
| qsl-server source/authority | `READY_FOR_FUTURE_AUTHORIZATION` | refresh selected source, live remote, permission, protection, and CI before mutation |
| qsl-attachments integration prerequisite | `REQUIRED_BEFORE_IMPLEMENTATION` | define whether qsl-server passes through opaque objects or calls qsl-attachments APIs |
| API/route boundary | `REQUIRED_BEFORE_IMPLEMENTATION` | exact route/API contract, reject codes, no-mutation behavior, compatibility rules, and route-token handling |
| storage/proxy boundary | `REQUIRED_BEFORE_IMPLEMENTATION` | prove whether qsl-server remains memory-only relay or stores/proxies attachment metadata/objects |
| timing/traffic-shape boundary | `FUTURE_GATE` | no stronger claim without timing/cadence/traffic evidence and public-ingress logs |
| rate-limit/quota boundary | `REQUIRED_BEFORE_IMPLEMENTATION` | bind qsl-server limits to attachment-object handoff and abuse/cost ceilings |
| retention/purge boundary | `REQUIRED_BEFORE_IMPLEMENTATION` | align qsl-server route TTL/delete-on-delivery with qsl-attachments retention/purge |
| backup boundary | `REQUIRED_BEFORE_IMPLEMENTATION` | define source, build artifacts, deployment config, backups, and any data roots |
| deploy/rollback boundary | `REQUIRED_BEFORE_IMPLEMENTATION` | exact install/update/rollback path or explicit no-deploy boundary |
| secrets/env boundary | `REQUIRED_BEFORE_IMPLEMENTATION` | relay token, qsl-attachments endpoint/config, env examples, redaction checks |
| monitoring/logging boundary | `REQUIRED_BEFORE_IMPLEMENTATION` | prove no raw route token, auth header, payload, object locator, resume token, or secret appears in logs |
| public-claim boundary | `FUTURE_GATE` | no stronger public copy without implementation, service, deployment, monitoring, and review evidence |
| external-review boundary | `FUTURE_GATE` | review-sensitive; external review remains incomplete unless future evidence proves completion |
| test/CI boundary | `READY_FOR_FUTURE_AUTHORIZATION` | qsl-server `rust` plus qsl-protocol companion checks are known but must be refreshed |
| qshield demo reference boundary | `READY_FOR_FUTURE_AUTHORIZATION` | demo evidence may remain an oracle/reference only, not production proof |

## qsl-attachments compatibility boundary

qsl-attachments PR #37 service-local size-class proof is a prerequisite for any
future qsl-server integration lane. qsl-server integration must not change
qsl-attachments behavior without separate cross-repo authorization.

The read-only qsl-server source inspection did not find a current direct
qsl-attachments integration implementation. Therefore a future authorization
lane must choose one of these exact designs before mutation:

- qsl-server treats qsl-attachments objects and metadata as opaque external
  state and only forwards bounded references or ciphertext relay payloads; or
- qsl-server integrates with qsl-attachments through a named service contract
  with explicit failure, retention, purge, backup, logging, secrets/env, and
  compatibility tests.

Until that future proof exists, qsl-attachments production evidence remains
service-local and qsl-server integration remains unimplemented.

## Storage / retention / purge / backup / deploy / rollback / secrets model

Future implementation prerequisites:

- service lifecycle: exact start/stop/restart behavior, bind address, reverse
  proxy boundary, auth mode, health checks, and failure behavior;
- request lifecycle: validation order for auth, route token, body size, pull
  max, qsl-attachments handoff, queue mutation, and response mapping;
- object lifecycle: whether qsl-server stores no object data, stores only
  transient relay items, or creates qsl-attachments-backed object state;
- qsl-attachments handoff lifecycle: exact create/upload/commit/fetch/delete or
  opaque-forwarding semantics;
- retention duration: align route TTL, delete-on-delivery, qsl-attachments
  retention, and any handoff state;
- purge trigger: exact access-triggered, scheduled, operator, rollback, and
  failed-handoff cleanup rules;
- stale cleanup: deterministic stale route/object cleanup without resurrecting
  purged state;
- failed upload/fetch cleanup: no accepted state and no leaked artifact on
  reject paths;
- backup inclusion/exclusion: source, release artifact, env file, deploy
  metadata, qsl-server backup directory, qsl-attachments data roots, logs, and
  response/evidence files;
- log redaction: no payload, raw route token, auth header, raw object locator,
  resume token, plaintext, raw key, passphrase, or unredacted secret;
- artifact redaction: no secret-dependent fixtures or retained sensitive
  outputs;
- monitoring and alert thresholds: request counts, bounded response codes,
  queue depth, size limit hits, route cap hits, rate-limit hits, storage growth,
  purge failures, and backup growth;
- operator runbook: install, update, rollback, token rotation, qsl-attachments
  handoff, public-ingress config, and verification commands;
- rollback: exact restore of previous binary/config plus compatibility of
  qsl-attachments object state;
- migration/compatibility: whether legacy qsl-server queued items and
  qsl-attachments objects remain readable after upgrade and after rollback;
- abuse/cost threshold: per-route, global, body-size, object-size, rate,
  retention, and backup-growth ceilings;
- secrets/env variables: `RELAY_TOKEN`, qsl-attachments endpoint/config,
  storage roots, backup roots, TLS/proxy assumptions, and operator-only values;
- deployment/non-deployment boundary: future implementation authorization may
  still be no-deploy unless a later directive explicitly authorizes deployment.

Because qsl-server has complete source authority but no current qsl-attachments
integration implementation, all integration-specific lifecycle fields remain
`REQUIRED_BEFORE_IMPLEMENTATION`.

## Public-ingress / timing / traffic-shape boundary

Public ingress remains unproven for the integration lane. Existing qsl-server
docs describe loopback binding behind a reverse proxy and public TLS
termination at the proxy. That is useful planning evidence, not proof of a
specific deployed public service.

Timing and traffic-shape gaps remain explicit:

- no claim may imply attachment sizes are hidden;
- no claim may imply timing metadata is hidden;
- no claim may imply traffic shape is hidden;
- no claim may imply all metadata is hidden;
- no claim may imply metadata-free, anonymity, or untraceable behavior;
- no claim may imply production readiness, public-internet readiness, or
  external-review completion.

Any stronger claim requires implementation evidence, service evidence,
deployment evidence, monitoring/log evidence, and review evidence.

## External-review sensitivity

qsl-server integration is review-sensitive because it touches routing,
operators' public ingress posture, service logs, retention, backup, and
qsl-attachments handoff behavior. NA-0345 does not update website/public docs
and does not claim external review is complete.

## Future validation / marker / verification-bundle plan

Future positive markers if NA-0346 authorization succeeds:

- `NA0346_QSL_SERVER_SOURCE_AUTHORITY_OK`
- `NA0346_QSL_SERVER_INTEGRATION_AUTHORIZATION_OK`
- `NA0346_QSL_ATTACHMENTS_INTEGRATION_CONTRACT_OK`
- `NA0346_QSL_SERVER_ROUTE_BOUNDARY_OK`
- `NA0346_QSL_SERVER_STORAGE_BOUNDARY_OK`
- `NA0346_QSL_SERVER_RETENTION_PURGE_BOUNDARY_OK`
- `NA0346_QSL_SERVER_BACKUP_BOUNDARY_OK`
- `NA0346_QSL_SERVER_SECRET_ENV_BOUNDARY_OK`
- `NA0346_QSL_SERVER_DEPLOY_ROLLBACK_BOUNDARY_OK`
- `NA0346_QSL_SERVER_PUBLIC_INGRESS_BOUNDARY_OK`
- `NA0346_QSHIELD_DEMO_REFERENCE_BOUNDARY_OK`
- `NA0346_NO_ATTACHMENT_SIZE_HIDDEN_CLAIM_OK`
- `NA0346_NO_TIMING_HIDDEN_CLAIM_OK`
- `NA0346_NO_TRAFFIC_SHAPE_HIDDEN_CLAIM_OK`
- `NA0346_NO_METADATA_FREE_CLAIM_OK`

Future blocker markers if source, authority, CI, qsl-attachments integration,
deploy/rollback, backup, secrets/env, logging, or public-ingress evidence
regresses:

- `NA0346_BLOCKED_QSL_SERVER_SOURCE_AUTHORITY`
- `NA0346_BLOCKED_QSL_ATTACHMENTS_CONTRACT`
- `NA0346_BLOCKED_QSL_SERVER_CI_AUTHORITY`
- `NA0346_BLOCKED_QSL_SERVER_DEPLOY_ROLLBACK`
- `NA0346_BLOCKED_QSL_SERVER_BACKUP`
- `NA0346_BLOCKED_QSL_SERVER_SECRET_ENV`
- `NA0346_BLOCKED_QSL_SERVER_PUBLIC_INGRESS`
- `NA0346_BLOCKED_PUBLIC_CLAIM_BOUNDARY`

## Public claim boundary

Allowed wording:

- qsl-server source/authority is complete enough for future authorization
  planning as of this evidence;
- qsl-attachments size-class proof is service-local;
- qshield embedded relay/demo evidence is reference/oracle only;
- qsl-server integration remains unimplemented until a future exact directive.

Prohibited wording:

- qsl-server integration is implemented by NA-0345;
- qsl-attachments service-local proof is qsl-server proof;
- qshield demo proof is production proof;
- attachment size, timing metadata, traffic shape, all metadata, anonymity,
  metadata-free behavior, or untraceable behavior is achieved;
- production readiness, public-internet readiness, or external-review
  completion is achieved.

## Selected successor

Selected successor:

`NA-0346 -- Metadata Runtime qsl-server Integration Implementation Authorization Plan`

Rationale:

- qsl-server source exists locally and strongest local source matches live
  remote `main`;
- GitHub permission, branch protection, required check, open PR state, and
  latest listed CI are known;
- the blocker is no longer source/authority absence;
- the next safe step is an authorization plan that defines exact future
  mutation files, branch/base, qsl-server tests, qsl-protocol companion scope,
  qsl-attachments integration contract, deploy/rollback, backup, secrets/env,
  public-ingress, logging, and public-claim boundaries.

## Rejected alternatives

- `NA-0346 -- Metadata Runtime qsl-server Source / Authority Bundle`: rejected
  as immediate successor because NA-0345 refreshed complete source/authority
  evidence for planning.
- `NA-0346 -- Metadata Runtime qsl-server Integration Blocker Resolution`:
  rejected as immediate successor because no current source/authority blocker
  remains; implementation-specific prerequisites can be handled in an
  authorization plan.
- `NA-0346 -- Metadata Runtime External Review Readiness Gap Audit`: rejected
  as immediate successor because integration authorization boundaries must be
  defined before review-readiness can be audited truthfully.
- `NA-0346 -- Metadata Runtime Website / Public Claim Boundary Audit`:
  rejected as immediate successor because public claims remain unchanged and
  the next evidence gap is integration authorization.
- `NA-0346 -- Metadata Runtime Service Timing Cross-Repo Authorization`:
  rejected as immediate successor because timing is one future gate inside the
  broader qsl-server integration authorization surface.

## Backup-plan impact statement

No backup-plan update is required for NA-0345 because this PR changes only
qsl-protocol governance, evidence, testplan, traceability, and operations
journal files under `/srv/qbuild/work`.

Future qsl-server implementation, deployment validation, qsl-attachments
integration, durable artifact capture, production data roots, deployment env
files, release artifacts, logs, or backup directories must re-check backup
scope before mutation or evidence capture.

## Next recommendation

Execute NA-0346 as a qsl-server integration implementation-authorization plan.
It should refresh qsl-server source/authority and then either authorize an exact
future implementation lane or stop with a precise blocker if any qsl-server,
qsl-attachments, deploy/rollback, backup, secrets/env, logging, public-ingress,
or public-claim prerequisite regresses.
