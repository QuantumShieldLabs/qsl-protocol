Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-14

# NA-0287 Service Production-Gate Evidence Map

Goals: G1, G3, G4, G5

## Executive Summary

NA-0287 maps the current `qsl-server` and `qsl-attachments` service-hardening
evidence into production-gate boundaries and deployment-boundary planning.

This is an evidence and governance map only. It does not implement service
behavior, change qsl-protocol runtime/protocol/crypto behavior, change
qsl-server, change qsl-attachments, update workflows, update dependencies,
change public-safety configuration, deploy a service, expose a service to the
public internet, or make a production-readiness claim.

The current service evidence is meaningful but bounded:

- `qsl-server` has local executable proof for the transport-only relay API,
  auth and route-token rejects, reject no-mutation, payload and queue limits,
  current `x-msg-id` semantics, fail-closed config parsing for selected knobs,
  local in-app rate/global route caps, and route TTL/retention cleanup.
- `qsl-attachments` has local executable proof for opaque-ciphertext handling,
  malformed JSON and reject taxonomy, capability scope and abuse, retention and
  cleanup, disk/quota pressure, same-root recovery, stopped/quiesced full-root
  restore, fail-closed partial restore boundaries, and secret-safe local
  logging/audit surfaces.
- Neither service has completed production deployment proof, public internet
  exposure proof, TLS/proxy proof, operational runbook proof, observability and
  alerting proof, long-running operations proof, incident-response proof, or
  external review completion proof.

## Scope and Non-Goals

In scope:

- classify qsl-server and qsl-attachments evidence by category;
- separate executable proof from docs-only or future-gated planning;
- define conservative deployment and public-claim boundaries;
- identify required gates before any future production service claim changes;
- preserve known gaps and recommended successor lanes.

Out of scope:

- qsl-server implementation changes;
- qsl-attachments implementation changes;
- qsl-protocol runtime, protocol, wire, crypto, auth, or state-machine changes;
- qsc/qsl runtime changes;
- qsc-desktop changes;
- website or external website changes;
- workflow, script, Cargo, dependency, branch-protection, or public-safety
  configuration changes;
- production deployment or public internet exposure;
- claims that production service operation, public internet operation,
  production backup/restore operation, external review completion, metadata
  elimination, anonymity, untraceability, quantum-proof status, or a proven true
  Triple Ratchet have been achieved.

## Classification Key

- `PROVEN_EXECUTABLE`: executable local or CI-backed evidence exists for the
  stated bounded behavior.
- `DOCS_ONLY`: the boundary is documented, but executable proof for the
  stronger operational claim does not exist.
- `NOT_READY`: the category is a missing service/operations gate and should
  not be treated as satisfied.
- `OUT_OF_SCOPE`: not part of this service production-gate map.
- `FUTURE_GATE`: future work must prove the category before a stronger claim
  can change.

## qsl-server Evidence Matrix

| Category | Evidence source | PR / NA reference | Classification | Remaining gate |
| --- | --- | --- | --- | --- |
| API contract / docs consistency | `README.md`, `docs/server/**`, local loopback API harnesses, retired legacy-route tests | NA-0270, NA-0272, NA-0273, qsl-server PRs #47/#49 | `PROVEN_EXECUTABLE` | Prove the same API contract under the selected deployment profile, including proxy/header preservation and retained access-log behavior. |
| Auth and bearer/route-token handling | Optional `RELAY_TOKEN` bearer auth, `X-QSL-Route-Token`, wrong/missing auth reject tests | NA-0273, qsl-server PR #49 | `PROVEN_EXECUTABLE` | Add deployment secret-management proof and proxy/header handling proof before any public service claim. |
| Reject / no-mutation behavior | Auth, route-token, oversized body, queue-full, bad pull `max`, and retired legacy-route no-mutation tests | NA-0273, NA-0275, NA-0277, NA-0280, NA-0281 | `PROVEN_EXECUTABLE` | Prove the same invariant under deployment logs, proxy paths, and long-running service operation. |
| Payload/body limits | `MAX_BODY_BYTES` parsing and effective body-limit tests | NA-0276, qsl-server PR #51 | `PROVEN_EXECUTABLE` | Add deployment configuration profile evidence and capacity sizing evidence. |
| Queue depth / overload | Per-route FIFO queue cap, `ERR_OVERLOADED`, drain behavior, route isolation | NA-0273, NA-0277, qsl-server PRs #49/#52 | `PROVEN_EXECUTABLE` | Add memory-sizing and long-running overload evidence for the selected host profile. |
| Rate limiting and global route caps | `MAX_ROUTE_COUNT`, `PUSH_RATE_BURST`, `PUSH_RATE_REFILL_PER_SEC`, route-slot release, `ERR_ROUTE_CAP`, `ERR_RATE_LIMITED` | NA-0279 design, NA-0280 harness, qsl-server PR #53 | `PROVEN_EXECUTABLE` | Edge/proxy rate policy, process-wide/auth-mode/source-IP/pull limits, and multi-host abuse posture remain future gates. |
| Route lifecycle / TTL / retention | `ROUTE_IDLE_TTL_MS`, access-triggered cleanup, stale-message discard, route slot/rate bucket release | NA-0281, qsl-server PR #54 | `PROVEN_EXECUTABLE` | Long-running cleanup soak, operational retention runbook, and persistence/backup posture remain future gates. |
| x-msg-id semantics | Duplicate IDs are accepted as separate FIFO entries; generated IDs remain non-empty; IDs are non-secret metadata | NA-0275, qsl-server PR #50 | `PROVEN_EXECUTABLE` | Any future idempotency, deduplication, or ID redaction change needs a separate semantic decision and tests. |
| Logging / no-secret behavior | Local captured app logs exclude raw route tokens, bearer tokens, auth headers, and payload sentinels; `x-msg-id` may appear as documented metadata | NA-0273, NA-0275, NA-0277, NA-0280, NA-0281 | `PROVEN_EXECUTABLE` | Reverse proxy, access-log, metrics-label, tracing, support-bundle, and retained artifact proof remains future work. |
| Config fail-closed behavior | Invalid size/depth/port/bind/TTL config rejects deterministically; selected above-ceiling values cap explicitly | NA-0276, NA-0280, NA-0281 | `PROVEN_EXECUTABLE` | Full deployment config matrix, secrets handling, and drift detection remain future gates. |
| Local loopback harness coverage | Focused and full qsl-server locked test suites, clippy, audit, and CI `rust` checks for service hardening PRs | NA-0273, NA-0275, NA-0276, NA-0277, NA-0280, NA-0281 | `PROVEN_EXECUTABLE` | Cross-host and public-ingress service harnesses remain future work. |
| Deployment assumptions | README and `docs/server/**` describe loopback default, explicit public bind, systemd/Caddy guidance, deployment verification, and production-boundary warnings | NA-0270, NA-0272, NA-0269 public plan | `DOCS_ONLY` | Executable deployment-profile validation, operator runbook drills, and drift checks are still required. |
| TLS/proxy/public exposure boundaries | qsl-server is HTTP-only and expects upstream TLS/proxy when exposed; public exposure is not approved by current evidence | NA-0270, NA-0269 public plan, current README | `FUTURE_GATE` | TLS termination, firewall/ACL, proxy header preservation, access-log redaction, and public internet abuse testing must be proven before public exposure. |
| Observability / health / metrics gaps | No dedicated health/metrics production observability proof is present in the audited hardening lanes | NA-0270 gap list, NA-0281 remaining gates | `NOT_READY` | Define and prove health, metrics, alerting, dashboards, SLOs, and secret-safe labels before a service operation claim. |
| Operational runbook gaps | Deployment docs exist, but incident response, rotation, backup, restore, rollback, and long-running operations drills are not proven | NA-0269 public plan, qsl-server docs | `DOCS_ONLY` | Execute runbook drills and record evidence for the selected deployment profile. |
| Dependency/advisory posture | qsl-server dependency remediation and repeated `cargo audit --deny warnings` passes are recorded | NA-0273, later qsl-server lanes | `PROVEN_EXECUTABLE` | Keep advisory scans green on the exact future service version and deployment artifact. |
| External review readiness | Evidence is organized but external review remains incomplete | NA-0250 external review package, this map | `FUTURE_GATE` | Refresh review package with service commits, deployment profile, residual gaps, and reviewer outcomes before claiming review completion. |

## qsl-server Production-Gate Gaps

qsl-server still lacks executable production service operation proof for:

- selected public or private deployment profile, exact service version, config,
  host shape, TLS/proxy topology, firewall/ACL posture, and header behavior;
- proxy/access log and metrics/tracing proof that route tokens, bearer values,
  payload bytes, or secret-bearing URL material are not retained;
- source-IP, edge, auth-mode, process-wide, and pull-rate abuse handling;
- memory sizing and long-running overload/cleanup soak under realistic
  sustained traffic;
- operational health/metrics/alerting and incident-response runbooks;
- secrets rotation, deployment rollback, backup/restore, and disaster recovery
  drills;
- external review results over the exact service build and deployment profile.

## qsl-server Deployment Boundary

The current qsl-server evidence supports a local/test harness claim only:

- transport-only relay behavior is locally test-backed;
- app-level auth/reject/rate/TTL behavior is locally test-backed;
- logs captured by the local harness avoid raw route tokens, bearer values, and
  payload sentinels;
- qsl-server remains HTTP-only and does not itself prove TLS termination or
  public ingress safety.

The current evidence does not approve public exposure, production relay
operation, production deployment, managed operations, or external review
completion.

## qsl-server Claim Boundary

Allowed wording:

- qsl-server has executable local hardening evidence for the bounded
  transport-only relay behaviors listed in this map.
- qsl-server production service claims remain future-gated on deployment,
  observability, public-ingress, runbook, and external-review evidence.

Disallowed wording unless explicitly negated or listed as prohibited:

- qsl-server production ready;
- production relay ready;
- public internet ready;
- external review complete;
- metadata-free, anonymity, anonymous messaging, untraceable, quantum-proof, or
  proven true Triple Ratchet.

## qsl-server Next Recommended Evidence

Recommended future lanes:

1. qsl-server deployment-profile proof: exact host, systemd, TLS/proxy,
   firewall/ACL, access-log redaction, header preservation, and rollback
   evidence.
2. qsl-server observability proof: health/metrics/logging labels that are
   useful and secret-safe.
3. qsl-server long-running service soak: route TTL, rate/global caps, queue
   pressure, cleanup, and alert behavior under sustained traffic.
4. qsl-server public-ingress abuse gate only if separately authorized.

## qsl-attachments Evidence Matrix

| Category | Evidence source | PR / NA reference | Classification | Remaining gate |
| --- | --- | --- | --- | --- |
| Opaque-ciphertext boundary | Service contract, reject-taxonomy harness, retention/recovery, quota, capability, and backup/restore harnesses | NA-0271, NA-0274, NA-0282, NA-0283, NA-0284, NA-0286 | `PROVEN_EXECUTABLE` | Prove the same boundary in deployment logs, proxy paths, backup artifacts, and retained operator bundles. |
| Malformed JSON and reject taxonomy | Sanitized Axum JSON rejection mapping, canonical `reason_code`, no persistence on malformed rejects | NA-0274, qsl-attachments PR #32 | `PROVEN_EXECUTABLE` | Expand to any future API shape/range/header taxonomy before stronger service claims. |
| Capability scope and abuse | Resource-scoped resume/fetch capabilities, wrong-resource rejects, duplicate/reuse semantics, abuse escalation, no unauthorized mutation/exposure | NA-0284, qsl-attachments PR #35 | `PROVEN_EXECUTABLE` | Broader authorization-service design, multi-tenant identity, cross-node abuse state, and one-time capability changes remain future gates. |
| Retention / cleanup / recovery | Request-path expiry cleanup, same-root restart recovery, coherent open-session and committed-object reconciliation, no resurrection for local cases | NA-0282, qsl-attachments PR #33 | `PROVEN_EXECUTABLE` | Background cleanup, operator cleanup commands, long-running retention soak, and production retention policy remain future gates. |
| Disk pressure / quota / abuse | Oversize quota, open-session quota, simulated disk headroom, no persistence on rejects, cleanup under pressure, bounded abuse loops | NA-0283, qsl-attachments PR #34 | `PROVEN_EXECUTABLE` | Real deployment disk monitoring, alerting, capacity planning, and host-level pressure drills remain future gates. |
| Backup / partial restore / transactional recovery | Stopped/quiesced full-root tempdir restore, coherent object/session recovery, unsupported partial restore fail-closed, mismatched metadata discard, no resurrection | NA-0285 plan, NA-0286 harness, qsl-attachments PR #36 | `PROVEN_EXECUTABLE` | This is local executable proof only; production backup/restore automation, snapshot discipline, restore drills, hot/live backup, and cross-node replication remain future gates. |
| No-secret / no-plaintext logging | Local logs, audit snapshots, recovery summaries, and error bodies exclude capabilities, descriptor sentinels, ciphertext sentinels, and plaintext sentinels | NA-0274, NA-0282, NA-0283, NA-0284, NA-0286 | `PROVEN_EXECUTABLE` | Proxy/access logs, metrics labels, tracing spans, backup artifacts, and retained support bundles need deployment proof. |
| Restart and same-root recovery | Startup reconciliation re-exposes only coherent open sessions and paired committed objects; incoherent artifacts are discarded fail-closed | NA-0282, NA-0285, NA-0286 | `PROVEN_EXECUTABLE` | Abrupt crash and power-loss durability, fsync discipline, durable transactions, and multi-root recovery remain future gates. |
| Partial restore boundary | Partial restore is unsupported; metadata-only, bytes-only, missing-part, orphan-part, mismatched locator, and mismatched length fixtures fail closed | NA-0285, NA-0286 | `PROVEN_EXECUTABLE` | Any future supported partial restore requires a separate policy, implementation, and executable tests. |
| Hot/live backup boundary | Hot/live backup is explicitly unsupported in the boundary plan and remains unimplemented | NA-0285, NA-0286 | `DOCS_ONLY` | A snapshot or transaction design plus executable proof is required before this boundary can change. |
| Public exposure / deployment assumptions | README and docs state single-node local-disk posture, operator-scoped deployment policy, loopback defaults, and deployment evidence limits | NA-0271, NA-0269 public plan, qsl-attachments docs | `FUTURE_GATE` | Public ingress, TLS/proxy, auth policy, quota policy, and abuse posture need exact deployment proof. |
| Operational backup/restore runbook gaps | Cold full-root plus matching config is the documented shape, but operator restore drills and automation are not proven | NA-0285, NA-0286 | `DOCS_ONLY` | Create and execute backup/restore runbooks against the exact deployment profile before any stronger claim. |
| Observability / health / metrics gaps | No production observability proof is present for health, metrics, alerting, or labels | NA-0271 gap list, this map | `NOT_READY` | Define and prove secret-safe health, metrics, dashboards, alert thresholds, and recovery signals. |
| Dependency/advisory posture | qsl-attachments hardening lanes repeatedly passed `cargo audit --deny warnings`, tests, and clippy | NA-0274, NA-0282, NA-0283, NA-0284, NA-0286 | `PROVEN_EXECUTABLE` | Keep advisory scans green on the exact deployed version and artifact. |
| External review readiness | Evidence is organized but external review remains incomplete | NA-0250 external review package, this map | `FUTURE_GATE` | Refresh review material with service commits, deployment evidence, residual gaps, and reviewer outcomes before claiming review completion. |

## qsl-attachments Production-Gate Gaps

qsl-attachments still lacks executable production service operation proof for:

- exact deployment profile, host shape, storage root, config, TLS/proxy, and
  firewall/ACL posture;
- deployment-level auth policy beyond the current operator-scoped deployment
  plus per-resource capability model;
- real disk capacity monitoring, quota alerting, restore drills, and incident
  response;
- production backup automation, stopped/quiesced snapshot procedure, restore
  runbook, and regular restore verification;
- hot/live backup support, partial restore support, cross-node replication, and
  object-store integration;
- health/metrics/alerting with secret-safe labels and retained artifacts;
- long-running retention/cleanup/disk/abuse soak on selected hosts;
- external review of the exact service build and deployment profile.

## qsl-attachments Deployment Boundary

The current qsl-attachments evidence supports a local/test harness claim only:

- opaque ciphertext storage/fetch behavior is locally test-backed;
- malformed JSON, capability rejects, retention/cleanup, quota/disk, and
  backup/restore fail-closed behavior are locally test-backed;
- stopped/quiesced full-root copy plus matching config is proven as a local
  tempdir recovery unit;
- partial restore is proven unsupported and fail-closed for the tested
  fixtures;
- hot/live backup, production backup automation, public exposure, and
  production service operation are not approved.

## qsl-attachments Claim Boundary

Allowed wording:

- qsl-attachments has executable local hardening evidence for the bounded
  opaque-ciphertext service behaviors listed in this map.
- qsl-attachments production service and backup/restore claims remain
  future-gated on deployment, runbook, observability, restore-drill,
  public-ingress, and external-review evidence.

Disallowed wording unless explicitly negated or listed as prohibited:

- production attachment ready;
- backup ready;
- restore ready;
- hot backup;
- live backup;
- public internet ready;
- external review complete;
- metadata-free, anonymity, anonymous messaging, untraceable, quantum-proof, or
  proven true Triple Ratchet.

## qsl-attachments Next Recommended Evidence

Recommended future lanes:

1. qsl-attachments deployment-profile proof: exact host, storage root,
   systemd, TLS/proxy, firewall/ACL, log redaction, and rollback evidence.
2. qsl-attachments observability proof: health/metrics/recovery signals with
   secret-safe labels and retention.
3. qsl-attachments backup/restore operations drill: stopped/quiesced full-root
   backup creation, integrity check, restore on a clean root, failure cases,
   and operator runbook evidence.
4. qsl-attachments long-running service soak: retention, cleanup, quota,
   abuse, restart, and recovery behavior under sustained use.

## Deployment Boundary Plan

### Local / Test Harness Scope

Local executable harnesses prove bounded service semantics under in-process or
loopback test conditions. They are valid evidence for the specific behavior
they exercise, not for production operation.

Current local/test scope includes:

- qsl-server loopback API, auth, rejects, config, rate/global cap, and TTL
  behavior;
- qsl-attachments in-process/loopback opaque-ciphertext, reject, capability,
  retention, disk/quota, and backup/restore behavior;
- focused and full Rust test suites, clippy, audit, and required sibling-repo
  CI checks for the associated PRs.

### Private-Network Proof Scope

Existing qsl-protocol demo evidence includes private-network demo proof, but
that is qshield demo evidence, not qsl-server or qsl-attachments production
service proof.

Before service claims can change, a future lane must define the exact private
deployment profile and prove service behavior there with retained logs and
secret-safe artifacts.

### Public Internet Exposure Boundary

Public internet service exposure is not authorized by this evidence.

Before any public internet service claim can change, future proof must cover:

- exact ingress host and DNS boundary;
- TLS termination and certificate renewal posture;
- firewall/security-group/ACL posture;
- reverse proxy request/response behavior and access-log redaction;
- auth, rate, quota, abuse, and overload behavior under untrusted clients;
- incident response and rollback;
- external review results or explicit residual review gaps.

### TLS / Proxy Boundary

qsl-server and qsl-attachments do not currently prove service-level TLS/proxy
operation in production. Any future deployment proof must show:

- TLS terminates in the declared layer;
- required headers are preserved and secret-bearing headers are not logged;
- canonical URLs do not place capability-like secrets in paths or query
  strings;
- proxy retries, buffering, body limits, timeouts, and connection limits do
  not weaken fail-closed service behavior.

### Observability / Metrics Boundary

Production observability remains `NOT_READY`.

Future proof must define and test health, metrics, logs, alerts, dashboards,
retention, and support bundle handling. Labels and event fields must avoid raw
route tokens, bearer values, capabilities, descriptors, ciphertext bytes,
plaintext, secret-bearing URLs, and long stable identifiers.

### Logging / Secret Boundary

Local harnesses prove application-level redaction for many paths. Deployment
proof remains future-gated for:

- proxy/access logs;
- tracing spans;
- metrics labels;
- systemd/journald retention;
- panic/error paths;
- support bundles;
- backup/restore artifacts.

### Backup / Restore Boundary

qsl-attachments has executable local proof for stopped/quiesced full-root copy
and fail-closed unsupported partial restore. That proof does not approve
production backup/restore operation.

Future backup/restore gates must prove:

- exact backup unit and matching config;
- quiescence or snapshot method;
- integrity and retention of the backup artifact;
- restore into a clean root;
- negative partial/mixed fixture behavior;
- operator runbook execution;
- no secret/plaintext leakage in backup, restore, logs, or evidence.

qsl-server backup/restore posture remains a separate future service operations
gate because the current relay state is local in-memory route state, not a
durable production queue.

### Rate / Abuse Boundary

qsl-server local in-app route and push-rate controls are proven. qsl-attachments
local quota/disk/capability abuse controls are proven. Deployment abuse
resistance remains future-gated for:

- edge/proxy and source-IP policy;
- aggregate/process-wide limits;
- multi-client or multi-tenant policy;
- alerting, operator action, and incident response;
- realistic sustained abuse tests.

### External Review Boundary

External review is incomplete. The current evidence can feed a future review
package, but review completion cannot be claimed until the package is refreshed
with exact service commits, deployment profile, residual gaps, test outputs,
reviewer scope, findings, and disposition.

## Production Gates

### Required Before Production Service Claim

- Exact qsl-server and qsl-attachments commits, artifacts, config, deployment
  profile, and operating assumptions.
- Green dependency/advisory posture on those exact artifacts.
- Executable deployment-profile tests for auth, reject no-mutation, rate/quota,
  retention/cleanup, logging, restart, backup/restore, observability, and
  rollback.
- Secret-safe logs, metrics, traces, backup artifacts, and support bundles.
- Operational runbooks for deploy, rollback, rotate, backup, restore, incident
  response, and evidence capture.
- External review package and explicit residual gap disposition.

### Required Before Public Internet Service Claim

- All production service gates above.
- TLS/proxy/firewall/DNS/ACL evidence for public ingress.
- Public-ingress abuse, malformed traffic, overload, and rate/quota proof.
- Access-log and metrics-label redaction proof.
- Incident response and emergency rollback proof.
- Review of public wording after evidence exists.

### Required Before Production Relay Claim

- qsl-server deployment-profile proof over the exact relay host/config.
- qsl-server auth, route-token, body, queue, rate/global-cap, TTL, and
  no-mutation behavior under that deployment.
- Proxy/header/access-log proof.
- Observability and operations runbooks.
- External review or explicit residual review status.

### Required Before Production Attachment Service Claim

- qsl-attachments deployment-profile proof over exact host/storage/config.
- Opaque-ciphertext, capability, reject, retention, quota/disk, backup/restore,
  restart, logging, and observability proof under that deployment.
- Backup/restore drill evidence and incident response.
- External review or explicit residual review status.

### Required Before Backup / Restore Readiness Claim

- qsl-attachments stopped/quiesced backup procedure or snapshot design;
- backup artifact integrity and storage policy;
- restore drill on a clean root using matching service config;
- negative partial/mixed restore fixtures;
- no secret/plaintext leakage in restore logs, summaries, and evidence;
- operator runbook and recovery-time expectations;
- explicit unsupported hot/live backup status and partial restore status.

## What Is Proven Now

- qsl-server local executable hardening evidence through auth/reject/logging,
  `x-msg-id`, config, queue pressure, rate/global cap, and route TTL/retention
  lanes.
- qsl-attachments local executable hardening evidence through malformed JSON,
  reject taxonomy, retention/cleanup/recovery, disk/quota/abuse, capability
  scope/abuse/logging, and backup/partial-restore/transactional-recovery lanes.
- qsl-protocol evidence documents and public boundary plan preserve
  non-production posture.
- Required public-safety remains a qsl-protocol protection gate.

## What Is Not Proven

- Production service operation.
- Public internet service operation.
- Production relay operation.
- Production attachment service operation.
- Production backup/restore operation.
- Hot/live backup support.
- Partial restore support beyond fail-closed unsupported fixtures.
- TLS/proxy/firewall/DNS/ACL safety.
- Production observability/metrics/alerting.
- Operator runbooks and incident response.
- External review completion.
- Metadata elimination, anonymity, untraceability, quantum-proof status, or a
  proven true Triple Ratchet.

## Prohibited Wording

The following phrases must not appear as affirmative claims:

- production-ready
- deployment-ready
- production relay ready
- qsl-server production ready
- production attachment ready
- public internet ready
- backup ready
- restore ready
- hot backup
- live backup
- external review complete
- metadata-free
- anonymity
- anonymous messaging
- untraceable
- quantum-proof
- proven true Triple Ratchet

They may appear only when explicitly negated, explicitly future/unproven, or
listed as prohibited wording.

## Suggested Next Implementation / Evidence Lanes

Recommended ranked successors:

1. `NA-0288 — Metadata Phase-2 and External Review Readiness Gap Plan`.
   This should map metadata phase-2 and external-review readiness gaps without
   changing protocol, service, website, or production claims.
2. qsl-server deployment-profile and observability proof.
3. qsl-attachments deployment-profile and backup/restore operations drill.
4. Public-ingress service proof only if separately authorized.

## Explicit No-Production-Readiness Claim

NA-0287 does not claim production readiness, deployment readiness, public
internet readiness, production relay readiness, production attachment service
readiness, production backup/restore readiness, or public service readiness.

## Explicit No External-Review-Complete Claim

NA-0287 does not claim external review completion. It only maps service
evidence and the remaining gates that a future external review package must
cover.
