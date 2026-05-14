Goals: G1, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-13
Replaces: n/a
Superseded-By: n/a

# QSL Server and Attachments Production-Boundary Plan

## Executive summary

NA-0269 defines the boundary between current demo evidence and future
production-capable service work for `qsl-server` and `qsl-attachments`.

Current evidence is useful, but it is not a production claim. The public demo
proves bounded, non-production `qshield` relay and attachment behavior. Sibling
repo evidence shows useful hardening work already exists for a transport-only
relay and an opaque encrypted attachment service. Before either service can be
treated as production-capable, the project needs executable service harnesses,
abuse and negative tests, deployment review, log/secret-leak proof, persistence
and cleanup proof, and external review readiness.

This plan makes those gates explicit. It does not implement production
hardening, change service code, change protocol or crypto behavior, change
website copy, change workflows, or update dependencies.

## Current evidence baseline

The current baseline comes from:

- `NEXT_ACTIONS.md` NA-0269, which authorizes planning only.
- `docs/demo/ATTACHMENT_PUBLIC_DEMO_READINESS.md`, which proves a
  non-production `qshield` attachment demo path.
- `docs/demo/CROSS_HOST_PRIVATE_NETWORK_SOAK.md`, which proves bounded
  cross-host private-network demo soak for `qshield`.
- `docs/public/RELEASE_READINESS_EVIDENCE_MAP.md`, which keeps release and
  production-service claims conservative.
- `docs/design/DOC-ATT-001_Signal_Class_Attachment_Architecture_Program_v0.1.0_DRAFT.md`,
  which separates the message/control plane from an opaque attachment plane.
- `docs/canonical/DOC-CAN-005_QSP_Attachment_Descriptor_and_Control_Plane_v0.1.0_DRAFT.md`,
  `DOC-CAN-006`, and `DOC-CAN-007`, which define descriptor, service, and
  attachment-encryption-context contracts.
- Read-only sibling worktree inventory under `/srv/qbuild/work/NA-0237D/`.

The current baseline proves bounded behavior in specific surfaces. It does not
prove production operation for a public relay or attachment service.

## qsl-server role and current boundary

`qsl-server` is a transport-only relay. It stores and forwards opaque payloads
and must not parse protocol messages, perform cryptography, or inspect payload
contents.

Read-only local inventory found the sibling repo at:

- `/srv/qbuild/work/NA-0237D/qsl-server`
- local HEAD: `0826ffa4d6f3`
- language/build system: Rust/Cargo

Evidence observed there:

- README states the transport-only relay invariant and safe loopback default.
- Canonical push/pull route-token carriage is header-based; legacy path-token
  routes are retired.
- Runtime limits exist for body size and queue depth.
- Optional bearer-token auth exists through `RELAY_TOKEN`.
- Tests cover relay smoke, auth rejects, no payload logging, and route-token
  redaction.
- Deployment docs and packaging exist for systemd, Caddy, install/update, and
  audit/verify scripts.

Current boundary:

- Useful relay hardening evidence exists, but NA-0269 did not build, test, or
  mutate the sibling repo.
- The qsl-protocol public demo evidence still uses bounded demo surfaces.
- Production exposure, multi-host operations, monitoring, incident response,
  dependency posture, and external review are future gates.

## qsl-attachments role and current boundary

`qsl-attachments` is the opaque encrypted attachment plane. It stores
ciphertext parts and committed ciphertext objects, persists local metadata and
session journals, and keeps plaintext plus attachment decrypt context off
service surfaces.

Read-only local inventory found the sibling repo at:

- `/srv/qbuild/work/NA-0237D/qsl-attachments`
- local HEAD: `1e1ae272a4cb`
- language/build system: Rust/Cargo

Evidence observed there:

- README maps the runtime to qsl-protocol `DOC-CAN-005`, `DOC-CAN-006`, and
  `DOC-CAN-007`.
- The current service is single-node and local-disk based.
- Opaque ciphertext parts and committed objects live under a local storage root.
- Session/object metadata is persisted in JSON journals.
- Request authorization is resource-scoped: resume tokens authorize one upload
  session, fetch capabilities authorize one committed object, and neither is a
  service account identity.
- Operational docs cover constrained-host validation, reference deployment,
  retention, quota, restart/recovery, and secret-safe logging boundaries.
- Tests cover the service contract and several validation evidence lanes.

Current boundary:

- The service has stronger contract and deployment evidence than the current
  qshield demo surface, but NA-0269 did not execute the sibling repo tests.
- The current implementation remains single-node local disk with local journals.
- Multi-node storage, backup/restore proof beyond the documented boundary,
  production observability, public exposure review, and external review remain
  future gates.

## What current demo evidence proves

Current qsl-protocol evidence proves:

- Bounded non-production qshield demo send/receive/decrypt behavior.
- Demo relay authentication and selected negative rejects.
- A demo-only attachment descriptor plus encrypted attachment payload path.
- Descriptor-bound ciphertext integrity rejection before receiver output write.
- Opaque relay entries for demo descriptor and payload wires.
- No checked token, sentinel plaintext, or panic markers in retained demo
  evidence.
- Three bounded private-network cross-host qshield demo runs with artifact and
  runtime separation.
- Public evidence maps and safe wording keep these claims bounded.

## What current demo evidence does not prove

Current demo evidence does not prove:

- qsl-server production relay operation.
- qsl-attachments production service operation.
- Public internet exposure safety.
- Production service authentication and authorization.
- Abuse resistance under untrusted client populations.
- Multi-tenant isolation.
- Long-running retention, cleanup, backup, restore, and incident response.
- Production logging, monitoring, and alerting.
- External review completion.
- Website or marketing approval for production service claims.

## Threat model categories

Future service hardening must explicitly cover:

- Authentication and authorization.
- Token/session management.
- Relay abuse and spam.
- Replay and idempotency.
- Attachment descriptor integrity.
- Opaque ciphertext storage.
- Metadata minimization limits.
- Persistence and retention.
- Deletion and cleanup semantics.
- Rate limiting and quotas.
- Observability and audit logging without secret leakage.
- Operational secrets management.
- TLS and network exposure.
- DoS and resource exhaustion.
- Cross-host/private-network versus public internet exposure.
- Multi-tenant isolation, if introduced.
- Deployment and configuration drift.
- Backup and restore.
- Incident response.
- Dependency and advisory handling.
- External review prerequisites.

## Hardening domains

Future implementation lanes should decompose hardening into these domains:

- Service auth.
- Storage.
- Network.
- Logging.
- Monitoring.
- Rate limiting.
- Config and secrets.
- API contract.
- Negative tests.
- Soak and stress.
- Cross-host tests.
- Production deployment review.
- External audit.

## Required production gates

Do not refresh public claims toward service production capability until all
applicable gates are executable and green:

- Executable qsl-server test harness.
- Executable qsl-attachments test harness.
- Opaque-ciphertext retention proof.
- Reject/no-mutation tests.
- Auth and rate-limit abuse tests.
- Secret/log leak tests.
- Persistence and cleanup tests.
- Deployment config review.
- Dependency/audit green.
- External review readiness.
- Documentation/public claim update after proof.

## Non-goals

NA-0269 does not:

- Implement qsl-server hardening.
- Implement qsl-attachments hardening.
- Change protocol, wire, crypto, auth, or state-machine semantics.
- Change qsp protocol-core, qsc/qsl runtime, qsc-desktop, website, external
  website, workflows, branch protection, public-safety, Cargo metadata, or
  dependencies.
- Authorize production deployment.
- Authorize public internet exposure.
- Convert demo evidence into production service evidence.

## Acceptance criteria for future production-capable claims

A future production-capable service claim needs at least:

- Exact service version, commit, deployment profile, and configuration profile.
- Executable qsl-server and qsl-attachments service harnesses.
- Negative tests proving rejects do not mutate protected state.
- Abuse tests for auth, replay, malformed input, queue/object limits, and rate
  limits.
- Opaque-ciphertext proof showing no plaintext or decrypt-context storage on
  service surfaces.
- Secret/log leak scans over service logs, proxy logs, command output, and
  retained evidence.
- Persistence, cleanup, restart, and backup/restore proof aligned to the
  declared deployment profile.
- Explicit network exposure review for TLS, ingress, firewall/security-group
  posture, and header preservation.
- Dependency/advisory audit green.
- External review package refreshed with exact commands and residual gaps.
- Public wording updated only after the evidence exists.

## Recommended NA sequence after planning

1. qsl-server read-only code audit and test-harness design.
2. qsl-attachments read-only code audit and test-harness design.
3. qsl-server auth/rate-limit/reject test harness.
4. qsl-attachments opaque ciphertext fetch/decrypt/integrity test harness.
5. Service logging/no-secret-leak audit.
6. Production deployment config review.
7. External review handoff refresh.
8. Website/public claim refresh only after evidence.

## NA-0270 qsl-server handoff

NA-0270 now provides the qsl-server read-only audit and first hardening harness
design at
`docs/governance/evidence/NA-0270_qsl_server_readonly_audit_test_harness_design.md`.
It records no qsl-server implementation change and preserves this plan's
non-production boundary.

## NA-0273 qsl-server harness evidence handoff

NA-0273 now records the first executable qsl-server auth/reject/logging
harness evidence at
`../governance/evidence/NA-0273_qsl_server_auth_reject_logging_harness.md`.
It also records the prerequisite qsl-server dependency advisory remediation.
The harness is local/loopback evidence and does not approve public exposure,
production relay operation, qsl-attachments service operation, or production
deployment. Remaining production gates in this plan stay future work.

## NA-0271 qsl-attachments handoff

NA-0271 now provides the qsl-attachments read-only audit and first hardening
harness design at
`docs/governance/evidence/NA-0271_qsl_attachments_readonly_audit_test_harness_design.md`.
It records no qsl-attachments implementation change and preserves this plan's
opaque-ciphertext and non-production boundaries. Future service implementation
remains gated on separate executable harness lanes and required CI evidence.

## NA-0274 qsl-attachments harness evidence handoff

NA-0274 now records the first executable qsl-attachments malformed JSON /
reject-taxonomy harness evidence at
`../governance/evidence/NA-0274_qsl_attachments_reject_taxonomy_harness.md`.
The qsl-attachments implementation change is limited to a test-backed Axum
JSON extractor reject mapping that returns sanitized canonical `reason_code`
bodies. It does not change qsl-protocol runtime behavior, qsl-server behavior,
deployment posture, workflow configuration, dependency files, or this plan's
opaque-ciphertext boundary. Remaining production gates in this plan stay future
work.

## NA-0275 qsl-server x-msg-id semantics handoff

NA-0275 now records executable qsl-server `x-msg-id` / duplicate-message
semantics evidence at
`../governance/evidence/NA-0275_qsl_server_idempotency_semantics_harness.md`.
The selected current semantics are that `x-msg-id` is a client-supplied
message identifier, not an idempotency key, and duplicate supplied IDs enqueue
separate FIFO messages when accepted. The qsl-server implementation was not
changed. This evidence does not approve public exposure, production relay
operation, qsl-attachments service operation, or production deployment.
Remaining production gates in this plan stay future work.

## NA-0276 qsl-server invalid config semantics handoff

NA-0276 now records executable qsl-server invalid configuration semantics
evidence at
`../governance/evidence/NA-0276_qsl_server_invalid_config_semantics_harness.md`.
The selected semantics are that missing size/depth config uses defaults,
non-numeric or zero size/depth config fails startup, above-ceiling values are
explicitly capped, invalid port/bind inputs fail closed, and relay-token auth
mode remains explicit. The qsl-server implementation change is limited to the
test-backed startup config parsing repair. This evidence does not approve
public exposure, production relay operation, qsl-attachments service
operation, or production deployment. Remaining production gates in this plan
stay future work.

## NA-0277 qsl-server abuse/rate/queue evidence handoff

NA-0277 now records executable qsl-server abuse/rate/queue harness evidence at
`../governance/evidence/NA-0277_qsl_server_abuse_rate_queue_harness.md`.
The current proven semantics are per-route queue caps, deterministic
`429 ERR_OVERLOADED`, no enqueue on overload, route isolation under pressure,
body/auth rejects without queue mutation, and no route-token/auth/payload
logging under pressure. The same evidence keeps the current gaps explicit: no
in-app rate limiting and no global route-count cap are implemented. This
evidence does not approve public exposure, production relay operation,
qsl-attachments service operation, or production deployment. Remaining
production gates in this plan stay future work.

## NA-0279 qsl-server rate/global-cap design handoff

NA-0279 now records the qsl-server rate-limit and global route-cap design plus
NA-0280 executable harness plan at
`../governance/evidence/NA-0279_qsl_server_rate_global_cap_design.md`.
It is planning evidence only: qsl-server implementation and tests are
unchanged, rate limiting and global route-count caps remain unimplemented, and
future hardening remains gated on executable qsl-server harness proof plus
required CI. This evidence does not approve public exposure, production relay
operation, qsl-attachments service operation, or production deployment.
Remaining production gates in this plan stay future work.

## NA-0280 qsl-server rate/global-cap harness handoff

NA-0280 now records executable qsl-server rate-limit and global route-cap
harness evidence at
`../governance/evidence/NA-0280_qsl_server_rate_global_cap_harness.md`.
The selected qsl-server semantics are bounded live route slots, accepted-push
route creation only when the cap allows, unknown pulls that do not create route
slots, delete-on-empty route slot release after drain, bounded local in-app
per-route push token buckets, `429 ERR_ROUTE_CAP`, and
`429 ERR_RATE_LIMITED`. The qsl-server implementation change is limited to
that local test-backed harness and minimal service behavior. This evidence
does not approve public exposure, production relay operation, qsl-attachments
service operation, or production deployment. Reverse proxy / edge rate
limiting and remaining production gates in this plan stay future work.

## NA-0281 qsl-server route lifecycle / TTL / retention handoff

NA-0281 now records executable qsl-server route lifecycle / TTL / retention
harness evidence at
`../governance/evidence/NA-0281_qsl_server_route_lifecycle_ttl_retention_harness.md`.
The selected qsl-server semantics add `ROUTE_IDLE_TTL_MS` as a deterministic
idle route TTL, with access-triggered cleanup on canonical push/pull after
request validation. Expired routes discard queued messages, release route
capacity, and release per-route rate accounting; drain-to-empty release and
NA-0280 rate/global-cap semantics remain preserved. The qsl-server
implementation change is limited to that local test-backed harness and minimal
service behavior. This evidence does not approve public exposure, production
relay operation, qsl-attachments service operation, or production deployment.
qsl-attachments retention / cleanup / recovery proof and remaining production
gates in this plan stay future work.

## NA-0282 qsl-attachments retention / cleanup / recovery handoff

NA-0282 now records executable qsl-attachments retention / cleanup / recovery
harness evidence at
`../governance/evidence/NA-0282_qsl_attachments_retention_cleanup_recovery_harness.md`.
The selected qsl-attachments semantics are request-path expiry cleanup,
same-root startup recovery of only coherent open sessions and paired committed
objects, fail-closed discard of incoherent artifacts, preservation of unexpired
committed objects, and secret-safe cleanup/recovery output. The qsl-attachments
implementation was not changed. This evidence does not approve public
exposure, production relay operation, qsl-attachments service operation, or
production deployment. Disk pressure / quota / abuse proof and remaining
production gates in this plan stay future work.

## NA-0283 qsl-attachments disk pressure / quota / abuse handoff

NA-0283 now records executable qsl-attachments disk pressure / quota / abuse
harness evidence at
`../governance/evidence/NA-0283_qsl_attachments_disk_pressure_quota_abuse_harness.md`.
The selected qsl-attachments semantics are deterministic quota and simulated
disk-headroom rejects, no unexpected new persistence on rejected writes, commit
disk rejects that preserve only the pre-existing committable session/part
contract, request-path cleanup under pressure that preserves unexpired
committed objects, fail-closed startup discard of partial artifacts, bounded
abuse loops, resource-scoped capabilities, opaque-ciphertext handling, and
secret-safe pressure logs. The qsl-attachments implementation was not changed.
This evidence does not approve public exposure, production relay operation,
qsl-attachments service operation, or production deployment. Capability scope /
abuse / logging depth and remaining production gates in this plan stay future
work.

## NA-0284 qsl-attachments capability scope / abuse / logging handoff

NA-0284 now records executable qsl-attachments capability scope / abuse /
logging harness evidence at
`../governance/evidence/NA-0284_qsl_attachments_capability_scope_abuse_logging_harness.md`.
The selected qsl-attachments semantics are resource-scoped resume tokens and
fetch capabilities that are reusable only inside their resource scope while
valid; commit, abort, session expiry, and object expiry invalidate the relevant
capability; wrong-resource and malformed capability attempts fail closed with
canonical reason codes and bounded abuse escalation; unauthorized operations do
not mutate another session/object or expose ciphertext/plaintext; logs and
error bodies do not leak capabilities, descriptors, ciphertext, or plaintext;
and the service remains opaque-ciphertext only. The qsl-attachments service
source was not changed. This evidence does not approve public exposure,
production relay operation, qsl-attachments service operation, or production
deployment. Backup / partial restore / transactional recovery boundaries and
remaining production gates in this plan stay future work.

## Safe public wording

Use wording like:

```text
The current public demo includes bounded, non-production relay and attachment
evidence. qsl-server and qsl-attachments have explicit production-boundary
hardening requirements, and production service claims remain gated on future
executable service evidence and review.
```

Also safe:

```text
The attachment proof stores opaque encrypted demo wires in the demo relay and
rejects tampered ciphertext before receiver output is written. This is demo
evidence, not production service approval.
```

## Prohibited public wording

Do not say:

```text
production-ready
deployment-ready
production relay ready
production attachment ready
qsl-server production ready
qsl-attachments production ready
external review complete
metadata-free
anonymity
untraceable
quantum-proof
proven true Triple Ratchet
```

Those phrases are prohibited unless they are explicitly negated, listed as
future/unproven, or appear inside this prohibited wording section.

## Rollback/stop conditions for future implementation

Future implementation lanes must stop if:

- The planned change would weaken fail-closed behavior.
- A reject path mutates protected state.
- Service logs or retained artifacts expose plaintext, secret-bearing tokens,
  decrypt context, raw route tokens, or secret-bearing URL material.
- A public exposure change is needed without explicit deployment authority.
- qsl-server starts parsing protocol or attachment semantics.
- qsl-attachments starts parsing plaintext attachment content or message-plane
  transcript semantics.
- A change requires protocol, wire, crypto, or state-machine semantics outside
  the declared lane.
- Dependency, advisory, or required CI gates fail without a bounded, truthful
  recovery.
- The evidence would require hiding known gaps or making a stronger claim than
  the tests prove.
