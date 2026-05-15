Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-15
Replaces: n/a
Superseded-By: n/a

# NA-0290 Metadata Phase-2 Identifier Rotation and Padding Defaults Design

## Executive Summary

NA-0290 defines the first metadata phase-2 design path for identifier rotation /
opaque handle policy and padding-default policy. It is design and governance
evidence only.

The current repository already proves bounded metadata-minimization behavior
for the non-production qshield demo and selected qsl-server/qsl-attachments
service-hardening lanes. That evidence is useful, but it does not prove
metadata phase-2 completion. Stable peer identifiers, route-visible handles,
contact graph shape, timing, size, deployment metadata, and local-state metadata
remain observable or future-gated.

The next executable lane should implement a deterministic metadata phase-2
harness that proves rotating opaque delivery handles and a named padding
default profile without claiming anonymity, metadata-free messaging,
untraceability, production readiness, public internet readiness, or external
review completion.

## Scope and Non-Goals

In scope:

- inventory current identifier, handle, padding, error, and retention surfaces;
- design future identifier rotation / opaque handle behavior;
- design future padding default behavior;
- define future executable harness requirements;
- preserve current public claim boundaries;
- recommend a successor executable lane.

Out of scope:

- protocol, wire, crypto, auth, negotiation, or state-machine changes;
- qsp protocol-core changes;
- qsc/qsl runtime changes;
- qshield demo implementation changes;
- qsl-server or qsl-attachments implementation changes;
- qsc-desktop changes;
- website, README, START_HERE, workflow, script, Cargo, lockfile, dependency,
  branch-protection, or public-safety changes;
- public-copy implementation;
- external review execution or completion;
- no claim of anonymity, metadata-free messaging, untraceability, production
  readiness, or public internet readiness.

## Current Metadata Phase-2 Baseline

Classification key:

- `PROVEN_EXECUTABLE`: local or CI-backed executable proof exists for the
  bounded behavior.
- `DOCS_ONLY`: the behavior or boundary is documented, but stronger behavior is
  not directly proven.
- `NOT_READY`: current evidence does not support the claim.
- `FUTURE_GATE`: a future lane must add proof before the claim can change.
- `OUT_OF_SCOPE`: the claim or mitigation is outside this evidence lane.

Current baseline:

| Surface | Current evidence | Classification | Boundary |
| --- | --- | --- | --- |
| Demo loopback default | `scripts/ci/metadata_conformance_smoke.sh`, `scripts/ci/demo_cli_smoke.sh`, DOC-G5-003 | `PROVEN_EXECUTABLE` | Non-production qshield demo only. |
| Demo relay authorization | Metadata/demo smoke, token-gated register/send/poll/bundle/consume/establish paths | `PROVEN_EXECUTABLE` | Authorization proof is demo-local; it is not production auth proof. |
| Demo stable peer IDs | DOC-G5-001/002/003, qshield register/send/poll surfaces | `DOCS_ONLY` as known leakage; `NOT_READY` for rotation | Peer IDs are stable and relay-visible today. |
| Demo establish replay record | `peer_id`, `bundle_id`, `session_id_hex`, `dh_init`, and `pq_init_ss` replay fingerprint in demo smoke | `PROVEN_EXECUTABLE` for replay reject | It depends on stable inputs today; rotation must preserve deterministic replay proof. |
| Demo optional padding | qshield `--padding-enable`, bucket validation, `pad_len` / `bucket` metadata, metadata smoke | `PROVEN_EXECUTABLE` for optional validation; `NOT_READY` for default policy | Padding is off by default and explicit when enabled. |
| QSE envelope bucket mode | `tools/refimpl/quantumshield_refimpl/src/qse/envelope.rs`, `qse_bucket_confidentiality.rs` | `PROVEN_EXECUTABLE` for refimpl bucket primitives | Not wired into a release default or metadata phase-2 profile. |
| Sanitized demo errors | NA-0244 evidence and metadata smoke | `PROVEN_EXECUTABLE` for selected rejects | Broader error normalization is future work. |
| qsl-server route lifecycle/rate/cap evidence | NA-0273, NA-0275, NA-0277, NA-0280, NA-0281, NA-0287 | `PROVEN_EXECUTABLE` for local hardening; `FUTURE_GATE` for deployment metadata | Service proof is local and does not prove metadata phase-2 completion. |
| qsl-attachments capability/retention/quota/recovery evidence | NA-0274, NA-0282, NA-0283, NA-0284, NA-0286, NA-0287 | `PROVEN_EXECUTABLE` for local service boundaries; `FUTURE_GATE` for deployment metadata | Capabilities remain authz secrets, not public delivery handles. |
| Contact graph hiding | DOC-G5-001/002/004, NA-0288 | `NOT_READY` | Relay/operator-visible communication relationships remain visible. |
| Timing, batching, jitter, cover traffic | DOC-G5-001/003, NA-0288 | `NOT_READY` | No batching, jitter, or cover traffic mitigation is proven. |
| Metadata phase-2 completion | NA-0288, release evidence map | `NOT_READY` | Identifier rotation, padding defaults, retention/purge, and broader errors remain future-gated. |

## Identifier / Opaque Handle Surface Inventory

Current identifier and handle surfaces:

| Identifier or handle | Where observed | Current stability | Current proof | Classification |
| --- | --- | --- | --- | --- |
| qshield peer ID | `/register`, `/send.to`, `/send.from`, `/poll.id`, `/bundle/<id>`, `/consume.id`, local state | Stable until operator uses another store/id | Metadata/demo smoke and DOC-G5 inventory | `DOCS_ONLY` leakage; `NOT_READY` rotation |
| qshield bundle ID | Bundle `id` field; establish identity binding | Stable for registered bundle and must match requested peer | Metadata smoke proves missing/mismatch rejects | `PROVEN_EXECUTABLE` for binding; `FUTURE_GATE` for opaque replacement |
| qshield session ID | Local session state and establish replay record | Deterministic from demo inputs today | Demo smoke and refimpl tests | `PROVEN_EXECUTABLE` for current replay/no-mutation paths; `FUTURE_GATE` for handle interaction |
| Establish replay fingerprint inputs | `peer_id`, `bundle_id`, `session_id_hex`, `dh_init`, `pq_init_ss` | Stable for the demo session | `scripts/ci/demo_cli_smoke.sh`, `metadata_conformance_smoke.sh` | `PROVEN_EXECUTABLE` |
| Relay bearer token | Authorization header and config/env | Secret-bearing auth token, not a delivery identifier | Metadata smoke no-leak checks | `PROVEN_EXECUTABLE` for no-leak checks; `OUT_OF_SCOPE` as rotating metadata handle |
| qsl-server route token | Canonical header carriage in service evidence | Secret-bearing route capability | Service evidence and DOC-G5-004 | `PROVEN_EXECUTABLE` for selected local hardening; `OUT_OF_SCOPE` as public handle |
| qsl-server route slot / message ID | Service-local queue/idempotency evidence | Service-local operational state | NA-0275/NA-0280/NA-0281/NA-0287 | `PROVEN_EXECUTABLE` locally; `FUTURE_GATE` for deployment metadata |
| qsl-attachments resume/fetch capability | Header/body capability carriage | Secret-bearing resource capability | NA-0284 and service evidence | `PROVEN_EXECUTABLE` locally; `OUT_OF_SCOPE` as user-visible rotating handle |
| qsl-attachments session/object/locator handles | Service journals, audit handles, descriptor refs | Resource scoped; some shortened audit handles exist in service evidence | NA-0282/0284/0286/0287 | `PROVEN_EXECUTABLE` locally; `FUTURE_GATE` for deployment metadata |
| qsc contact labels/device IDs/fingerprints | Local qsc stores and operator surfaces | Stable local trust/routing metadata | TRACEABILITY rows and DOC-G5-004 | `DOCS_ONLY` plus bounded executable support; `FUTURE_GATE` for minimization |

## Identifier Rotation Design

### Design rule

The future phase-2 lane should introduce an explicit opaque delivery handle
layer for the demo/harness surface instead of rotating cryptographic identity
material silently.

The handle should be:

- scoped to a peer/contact and relay route;
- short-lived by epoch or session policy;
- unlinkable to casual relay inspection beyond the active epoch, within the
  limits of timing, IP, token, and traffic-shape residuals;
- deterministic under a test-only seed or fixture so CI can prove rotation,
  replay rejects, and no-mutation behavior;
- separate from relay bearer tokens, qsl-server route tokens, and
  qsl-attachments capabilities.

### Rotation triggers

Recommended future triggers:

- session start or re-establish event;
- explicit handle epoch rollover;
- explicit operator reset / rotate command in a harness profile;
- relay route re-registration;
- attachment operation only when a separate descriptor or locator handle is
  required for that resource.

Triggers to keep future-gated:

- traffic-volume-based rotation;
- time-based production rotation;
- batching or jitter-driven rotation;
- cross-device or multi-relay handle migration.

### What should never rotate silently

The future lane must not silently rotate:

- long-term cryptographic identity keys or trust fingerprints;
- canonical decision IDs, evidence IDs, and test vector category IDs;
- replay-protection inputs without preserving deterministic replay reject
  semantics;
- relay bearer tokens or qsl-server route tokens as if they were public
  metadata handles;
- qsl-attachments fetch/resume capabilities as if they were non-secret
  delivery handles;
- local audit evidence needed to reproduce a test failure.

Identity rotation is a separate trust and authentication problem. Opaque handle
rotation must not make peer identity ambiguous or hide no-mutation failures.

### Deterministic audit hooks

Future executable proof should include:

- a fixed test profile name for metadata phase-2 handle behavior;
- a deterministic test-only handle derivation fixture or seeded generator;
- stable handle epoch labels in test output that do not expose secret material;
- a replay cache test proving old handle replay rejects after rotation;
- a no-mutation test proving rejected old or mismatched handles do not consume
  bundles, mutate session state, or drain queued messages;
- a migration/compatibility test proving pre-rotation demo behavior remains
  explicit when the phase-2 profile is not selected.

### Interactions

Replay protection:

- replay records should bind the accepted handle epoch to the underlying
  session/bundle evidence;
- stale handles must reject deterministically;
- replay cache migration must be auditable and no-mutation preserving.

No-mutation rejects:

- stale, unknown, malformed, wrong-contact, and wrong-route handles must reject
  before queue/session/bundle mutation;
- reject bodies must remain sanitized and must not reveal secret tokens or raw
  stable identifiers beyond what the harness explicitly marks as public test
  evidence.

Demo reproducibility:

- CI must be able to reproduce handle values or at least handle equality/
  inequality relations deterministically;
- random production-like handle material may be simulated by a fixture, but the
  fixture must not be confused with production randomness.

Service route tokens and attachment capabilities:

- route tokens and capabilities remain authorization secrets;
- the opaque delivery handle may reference a route/resource, but it must not be
  accepted as proof of authorization;
- service-local short audit handles can inform the shape, but qsl-server and
  qsl-attachments implementation changes need separate authorization.

External review evidence:

- reviewer-facing docs should classify rotation proof as harness evidence until
  a later implementation lane proves broader surfaces;
- package existence, design existence, and harness proof still do not complete
  external review.

## Padding Surface Inventory

Current padding surfaces:

| Padding surface | Current behavior | Classification | Gap |
| --- | --- | --- | --- |
| qshield config | `padding_enabled` defaults false; buckets are configured by `--padding-enable --padding-buckets` | `PROVEN_EXECUTABLE` for config validation | No default padding policy. |
| qshield send | When enabled, pads the wire bytes to the smallest configured bucket and sends `pad_len` plus `bucket` metadata | `PROVEN_EXECUTABLE` for optional behavior | Bucket choice leaks bucket class and is not a phase-2 default. |
| qshield relay validation | Rejects inconsistent `pad_len` / `bucket` metadata | `PROVEN_EXECUTABLE` | Broader raw ciphertext shape validation is out of current proof. |
| qshield receive | Verifies bucket size, strips `pad_len`, and passes unpadded wire to actor | `PROVEN_EXECUTABLE` | No phase-2 default or profile negotiation. |
| QSE envelope profile | `Standard` 1024, `Enhanced` 2048, `Private` 4096 minimum envelope profiles in refimpl | `PROVEN_EXECUTABLE` for primitives | Not a default transport policy. |
| Attachment demo messages | Demo attachment descriptor and payload messages use qshield send path; attachment service itself remains opaque-ciphertext | `PROVEN_EXECUTABLE` for demo path only | No production attachment padding policy. |
| qsl-server/qsl-attachments service surfaces | Local hardening evidence records sizes/limits/quotas but no global padding default | `FUTURE_GATE` | Deployment and service padding require separate scope. |

## Padding Defaults Design

### Design rule

The future phase-2 executable lane should define a named metadata phase-2 demo
profile where padding is enabled by default inside that profile, while the
current baseline remains explicit and unchanged until implementation is
authorized.

This design does not turn padding on today.

### Optional versus default

Current state:

- qshield padding is optional and off by default;
- a user/test must pass `--padding-enable` plus bucket sizes;
- QSE bucket mode exists as a refimpl primitive but is not a default transport
  policy.

Future state to prove:

- a named phase-2 harness profile enables padding without requiring each test to
  restate the option;
- explicit opt-out, if allowed, is visible in config and evidence output;
- the baseline non-phase-2 demo profile remains truthful and does not pretend
  padding is active.

### Bucket policy to consider

Recommended first harness bucket set:

- demo transport buckets: 512, 1024, 2048, 4096 bytes;
- QSE profile alignment: Standard 1024, Enhanced 2048, Private 4096;
- reject any ciphertext larger than the largest configured bucket unless the
  future lane explicitly defines a larger bucket or fail-closed fallback.

The future lane should measure size overhead for representative small, medium,
and attachment-descriptor messages. It should report overhead as evidence, not
as a privacy guarantee.

### Deterministic audit hooks

Future executable proof should include:

- fixed plaintext sizes that land in different buckets;
- exact `wire_len == bucket` checks for queued relay messages;
- exact `pad_len == bucket - ciphertext_len` checks where deterministic
  ciphertext length is available;
- malformed bucket metadata rejects with no queue mutation;
- receive-side rejection for bucket mismatch and pad length overflow;
- compatibility proof that old optional padding tests still pass;
- QSE bucket-mode regression proof remains green.

### Interactions

Envelope / metadata conformance smoke:

- metadata smoke should prove default phase-2 profile padding, optional
  baseline padding, malformed padding rejects, and no-secret error bodies.

Demo CLI smoke:

- demo smoke should keep a non-production boundary marker and add a phase-2
  padding receipt only when the future harness profile is selected.

External review package:

- reviewer docs should state that padding defaults are harness evidence until a
  later release lane proves broader operation.

Performance and size overhead:

- bucket defaults trade bandwidth for reduced exact-size leakage;
- overhead must be measured and visible;
- future text must not claim full size hiding, traffic-analysis resistance, or metadata-free messaging from padding alone.

Public claims:

- safe wording is limited to evidence-bound phrasing such as "padding default
  harness evidence exists" after the future lane lands;
- do not claim anonymity, metadata-free messaging, untraceability, or traffic
  analysis resistance.

## Sanitized Error / Retention / Purge Adjacency

Sanitized errors:

- current executable proof covers selected demo malformed JSON, content type,
  auth, padding config, padding metadata, and service-local reject cases;
- broader error normalization belongs in a later executable lane unless it is
  needed as a direct prerequisite for the identifier/padding harness.

Retention and purge:

- qsl-server route TTL/retention and qsl-attachments retention/cleanup/recovery
  have local executable evidence;
- a public metadata phase-2 retention/purge policy for demo/service-visible
  metadata is not implemented by NA-0290;
- retention/purge should remain a separate successor lane unless the NA-0291
  harness discovers a prerequisite stop.

## Future Executable Harness Plan

The recommended successor lane is:

`NA-0291 - Metadata Phase-2 Identifier Rotation and Padding Defaults Executable Harness`

Minimum future proof:

1. Add a named phase-2 harness profile for the qshield demo or a prerequisite
   stop if the current surface cannot host it truthfully.
2. Prove rotating opaque delivery handles with deterministic test fixtures.
3. Prove stale/wrong/malformed handle rejects are fail-closed and no-mutation.
4. Prove replay protection remains deterministic across handle rotation.
5. Prove padding defaults inside the phase-2 profile with stable bucket checks.
6. Prove malformed padding metadata and receive-side padding mismatches reject
   without leaking secrets.
7. Preserve existing metadata smoke, demo smoke, QSE bucket primitive tests,
   qsc send_commit, formal/model checks, advisory health, link/leak checks, and
   public-safety.
8. Keep public docs conservative until executable evidence lands.

## Public Claim Boundary

NA-0290 preserves these boundaries:

- no anonymity claim;
- no metadata-free claim;
- no untraceable claim;
- no production-readiness claim;
- no public-internet-readiness claim;
- no external-review-complete claim;
- no claim that metadata phase-2 is complete;
- no claim that identifier rotation is implemented;
- no claim that padding defaults are implemented.

Service-hardening evidence and demo evidence remain useful, but neither proves
anonymity, metadata-free messaging, or untraceability, and neither proves
production operation, public internet exposure safety, or external review
completion.

## Safe Future Public Wording

Safe future wording, before NA-0291 implementation:

- "metadata phase-2 design work is underway"
- "identifier/padding evidence is future-gated"
- "not anonymity"
- "not metadata-free"
- "not untraceable"

Safe future wording, only after executable proof lands:

- "QSL has bounded harness evidence for rotating opaque delivery handles in the
  named phase-2 demo profile."
- "QSL has bounded harness evidence for default padding buckets in the named
  phase-2 demo profile."
- "These harnesses reduce selected stable-id and exact-size exposure in the
  tested profile; they do not remove timing, IP, relay-observer, deployment, or
  local-state metadata."

## Prohibited Wording

The following phrases must not appear as affirmative claims:

- prohibited phrase: production-ready
- prohibited phrase: deployment-ready
- prohibited phrase: production relay ready
- prohibited phrase: production attachment ready
- prohibited phrase: public internet ready
- prohibited phrase: external review complete
- prohibited phrase: externally reviewed
- prohibited phrase: review complete
- prohibited phrase: metadata-free
- prohibited phrase: anonymity
- prohibited phrase: anonymous messaging
- prohibited phrase: untraceable
- prohibited phrase: quantum-proof
- prohibited phrase: proven true Triple Ratchet
- prohibited phrase: traffic-analysis resistant

They may appear only when explicitly negated, explicitly marked `NOT_READY`,
listed as prohibited wording, or described as future/unproven.

## Relationship to NA-0290A Public Visibility Strategy

NA-0290A recommends making QSL more memorable by leading with evidence,
fail-closed behavior, and named gaps. NA-0290 applies that guidance to metadata
phase-2 by turning "metadata honesty" into an executable-lane design rather than
public copy.

This document does not implement NA-0290A website, README, START_HERE, social,
or public-copy recommendations.

## What Is Not Implemented in NA-0290

NA-0290 does not implement:

- identifier rotation;
- opaque delivery handles;
- padding defaults;
- new padding buckets;
- batching, jitter, or cover traffic;
- contact graph hiding;
- retention/purge policy;
- broader sanitized-error coverage;
- qshield demo code changes;
- qsl-server or qsl-attachments code changes;
- qsc/qsl runtime changes;
- protocol, wire, crypto, auth, negotiation, or state-machine changes;
- website, README, START_HERE, workflow, script, Cargo, dependency,
  branch-protection, or public-safety changes.

## Recommended Successor Lane

Restore exactly one successor after NA-0290 closeout:

`NA-0291 - Metadata Phase-2 Identifier Rotation and Padding Defaults Executable Harness`

Recommended objective:

Implement the first executable metadata phase-2 harness for identifier rotation
/ opaque handle policy and padding-default behavior designed here, without claiming anonymity, metadata-free messaging, untraceability, production readiness, public internet readiness, or external review completion.
