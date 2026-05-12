Goals: G1, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-12
Replaces: n/a
Superseded-By: n/a

# NA-0269 Server / Attachment Production-Boundary Plan Audit

Directive: QSL-DIR-2026-05-11-070 / NA-0269

## Objective

Produce a planning-only production-boundary hardening plan for qsl-server and
qsl-attachments without implementing production hardening, changing service
code, changing protocol or crypto semantics, changing website copy, changing CI
configuration, changing branch protection, or changing dependencies.

## Sources inspected

qsl-protocol sources:

- `NEXT_ACTIONS.md` NA-0269 entry.
- `docs/public/RELEASE_READINESS_EVIDENCE_MAP.md`.
- `docs/demo/ATTACHMENT_PUBLIC_DEMO_READINESS.md`.
- `docs/demo/CROSS_HOST_PRIVATE_NETWORK_SOAK.md`.
- `docs/design/DOC-ATT-001_Signal_Class_Attachment_Architecture_Program_v0.1.0_DRAFT.md`.
- `docs/design/DOC-ATT-002_qsl-attachments_Deployment_and_Operational_Hardening_Contract_v0.1.0_DRAFT.md`.
- `docs/canonical/DOC-CAN-005_QSP_Attachment_Descriptor_and_Control_Plane_v0.1.0_DRAFT.md`.
- `docs/canonical/DOC-CAN-006_QATT_Attachment_Service_Contract_v0.1.0_DRAFT.md`.
- `docs/canonical/DOC-CAN-007_QATT_Attachment_Encryption_Context_and_Part_Cipher_v0.1.0_DRAFT.md`.
- `docs/governance/evidence/NA-0260_attachment_demo_readiness_audit.md`.
- `docs/governance/evidence/NA-0268_cross_host_private_network_soak_audit.md`.
- `TRACEABILITY.md`.
- `DECISIONS.md`.
- Relevant `tests/NA-*attachment*`, `tests/NA-*relay*`, and demo testplans.

Sibling/local repo inventory:

- `/home/victor/work/qsl/qsl-server`: not present.
- `/home/victor/work/qsl/qsl-attachments`: not present.
- `/home/victor/work/qsl/qsc-desktop`: not present as a standalone sibling.
- `/srv/qbuild/work/NA-0237D/qsl-server`: present, read-only inspected.
- `/srv/qbuild/work/NA-0237D/qsl-attachments`: present, read-only inspected.
- Older `/srv/qbuild/work/NA-0237*` qsl-server and qsl-attachments sibling
  worktrees exist with the same observed local HEADs as the NA-0237D copies.

## qsl-server inventory

Path:

- `/srv/qbuild/work/NA-0237D/qsl-server`

Status:

- Clean local sibling worktree.
- Local HEAD: `0826ffa4d6f3`.
- Language/build system: Rust/Cargo.

Observed role:

- Transport-only relay for QSL demos and relay-backed flows.
- Stores and forwards opaque payloads.
- Does not implement protocol parsing, crypto, or attachment semantics.

Evidence found:

- README states transport-only and no payload/secret logging invariants.
- The default bind is loopback, with explicit opt-in needed for broader bind.
- Header-based route-token carriage is canonical; legacy path-token routes are
  retired.
- Body-size and queue-depth limits exist.
- Optional bearer-token auth gate exists through `RELAY_TOKEN`.
- Tests cover smoke behavior, auth rejects, route-token behavior, and safe
  logging/redaction properties.
- Deployment docs and scripts cover systemd, Caddy, install/update, checksum
  update, audit, and remote verify workflows.

Known production gaps:

- NA-0269 did not run the sibling repo tests or build.
- In-app timeout coverage remains a documented follow-on in server deployment
  hardening docs.
- Public internet exposure requires deployment-specific TLS, ACL, proxy, and
  log-review proof.
- Abuse, spam, rate-limit, incident response, operational monitoring, and
  external review gates need executable service evidence.

## qsl-attachments inventory

Path:

- `/srv/qbuild/work/NA-0237D/qsl-attachments`

Status:

- Clean local sibling worktree.
- Local HEAD: `1e1ae272a4cb`.
- Language/build system: Rust/Cargo.

Observed role:

- Opaque encrypted attachment service/runtime.
- Stores ciphertext parts and committed ciphertext objects on local disk.
- Persists session/object metadata as JSON journals.
- Keeps plaintext attachment content and decrypt context off service surfaces.

Evidence found:

- README maps runtime behavior to qsl-protocol `DOC-CAN-005`, `DOC-CAN-006`,
  and `DOC-CAN-007`.
- Current posture is single-node local-disk runtime.
- Service auth is resource-capability based: resume tokens authorize one upload
  session, fetch capabilities authorize one committed object.
- Operator policy subject remains deployment-scoped, not per-user or
  multi-tenant.
- Operational docs cover constrained-host validation, reference deployment,
  durability/recovery, retention, quota, secret hygiene, and policy-subject
  boundaries.
- Tests and evidence files cover service contract, runtime faithfulness,
  operational hardening, constrained host validation, reference deployment, and
  stress/soak/chaos evidence.

Known production gaps:

- NA-0269 did not run sibling repo tests or build.
- The service remains single-node local disk.
- Multi-node durability, storage backend migration, backup/restore breadth,
  production monitoring, and external review remain future gates.
- Broader auth beyond resource capabilities is reserved/undefined unless a
  future contract defines it.
- Public exposure and ingress/TLS/log redaction need exact deployment proof.

## Current evidence baseline

qsl-protocol currently proves:

- Non-production qshield demo relay and message flow evidence.
- Non-production qshield attachment descriptor, encrypted payload,
  fetch/decrypt, integrity-reject, and no checked secret/plaintext leakage
  proof.
- Cross-host private-network qshield demo soak across a trusted private
  network, with runtime/artifact separation and cleanup proof.
- Metadata and public-evidence maps that keep production and release claims
  conservative.

qsl-protocol does not currently prove:

- qsl-server production relay operation.
- qsl-attachments production service operation.
- Public internet exposure safety.
- Multi-tenant isolation.
- Production observability, incident response, backup/restore, or external
  review completion.

## Known gaps

- No executable NA-0269 service harness was added.
- No qsl-server implementation changed.
- No qsl-attachments implementation changed.
- No protocol, crypto, or state-machine behavior changed.
- No website or external website wording changed.
- No dependency or workflow changed.
- Production gaps remain explicitly visible in the public plan.

## Extended read-only audit observations

### qsl-server

Proven issue:

- Medium severity documentation/contract consistency issue: qsl-server
  `docs/server/DOC-SRV-003_Relay_Inbox_Contract_v1.0.0_DRAFT.md` and
  `docs/server/DOC-SRV-004_Relay_Auth_And_Hardening_Contract_v1.0.0_DRAFT.md`
  name the queue-full error as `ERR_QUEUE_FULL`, while the observed
  implementation and README use `ERR_OVERLOADED`. This is not fixed here
  because qsl-server implementation and sibling-repo docs are out of scope.

Recommendations, not proven bugs:

- Add an executable qsl-server harness that tests optional auth enabled and
  disabled modes, missing/wrong bearer token, missing route token, malformed
  route token, oversize body, empty body, queue overflow, pull empty, and
  no-payload/no-token logging.
- Decide whether duplicate message ids are intentionally accepted or should
  become an idempotency/replay gate before any stronger relay claim.
- Add deployment-profile tests for timeout behavior at the proxy or in-app
  layer, depending on the final production contract.

### qsl-attachments

Proven issue:

- None found during NA-0269 read-only audit.

Recommendations, not proven bugs:

- Add an executable qsl-attachments harness that covers create, upload,
  identical replay, mismatched replay reject/no-mutation, status, commit,
  abort, fetch, range fetch, expiry, cleanup, disk pressure, restart/recovery,
  secret/log leak scans, and capability scope boundaries.
- Review startup failure handling in the future service hardening lane so
  invalid config, bind, and serve failures produce operator-safe evidence
  rather than relying on panic-style process termination.

### qsl-protocol evidence

Proven issue:

- None found in current qsl-protocol evidence for this planning scope.

Recommendations:

- Keep current public evidence wording conservative until service harnesses
  exist.
- Treat metadata phase-2, external review, and service deployment review as
  independent gates, not implied follow-ons from demo success.

## Plan summary

The plan added at
`docs/public/QSL_SERVER_ATTACHMENTS_PRODUCTION_BOUNDARY_PLAN.md` defines:

- Current evidence baseline.
- qsl-server role and boundary.
- qsl-attachments role and boundary.
- What current demo evidence proves.
- What current demo evidence does not prove.
- Threat model categories.
- Hardening domains.
- Required production gates.
- Non-goals.
- Acceptance criteria for future production-capable claims.
- Recommended future NA sequence.
- Safe and prohibited public wording.
- Stop conditions for future implementation.

## Why no implementation changes were made

NA-0269 is a docs/governance planning lane. Implementing qsl-server or
qsl-attachments hardening here would violate the active READY scope, blur
planning with production service implementation, and risk claiming more than the
current evidence proves.

## Recommended successor

Recommended successor:

- `NA-0270 - qsl-server Read-Only Code Audit and Test-Harness Design`.

Reason:

- qsl-server is the narrower transport boundary.
- A read-only audit and harness design can convert this plan into executable
  service evidence without changing relay implementation prematurely.
- The lane should preserve transport-only relay semantics, no production
  service claim, no public exposure, no website update, and no protocol/crypto
  change.
