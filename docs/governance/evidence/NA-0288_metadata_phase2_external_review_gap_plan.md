Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-14
Replaces: n/a
Superseded-By: n/a

# NA-0288 Metadata Phase-2 and External Review Readiness Gap Plan

## Executive Summary

NA-0288 maps the remaining metadata phase-2 and external-review readiness gaps
after the public demo, qsl-server, and qsl-attachments evidence lanes matured.

This is planning and evidence classification only. It does not implement
protocol, crypto, runtime, service, desktop, website, workflow, script, Cargo,
dependency, branch-protection, or public-safety changes.

Current evidence is useful but bounded:

- the qshield demo has executable proof for loopback defaults, authenticated
  relay access, selected sanitized rejects, optional padding validation,
  one-command demo behavior, KT-negative demo proof, attachment demo proof, and
  repeated-run/stress evidence;
- qsl-server and qsl-attachments have local executable service-hardening
  evidence that improves production-gate planning, logging boundaries, and
  opaque-ciphertext boundaries;
- these proofs do not prove anonymity, metadata-free messaging, untraceability,
  production service operation, public internet readiness, or external review
  completion.

The next truthful work is to refresh the external review package and claim
boundaries with this gap map, then create separate executable lanes for the
highest-priority metadata phase-2 evidence gaps.

## Scope and Non-Goals

In scope:

- classify metadata phase-2 evidence and gaps;
- classify external review package readiness and gaps;
- link demo/service evidence to the claims it actually supports;
- define prohibited wording and future evidence gates;
- identify next evidence lanes without implementing them.

Out of scope:

- protocol, wire, crypto, auth, negotiation, or state-machine changes;
- qsp protocol-core changes;
- qsc/qsl runtime changes;
- qsl-server or qsl-attachments implementation changes;
- qsc-desktop changes;
- website or external website changes;
- workflow, script, Cargo, lockfile, dependency, branch-protection, or
  public-safety changes;
- public claim upgrades;
- external review completion.

## Classification Key

- `PROVEN_EXECUTABLE`: executable local or CI-backed proof exists for the stated
  bounded behavior.
- `DOCS_ONLY`: the boundary is documented, but no direct executable proof exists
  for a stronger claim.
- `NOT_READY`: the evidence does not support the claim.
- `FUTURE_GATE`: a future lane must add proof before the claim can change.
- `OUT_OF_SCOPE`: the claim or mitigation is outside the current QSL evidence
  lane.

## Metadata Phase-2 Readiness Matrix

| Category | Evidence source | Classification | Remaining gap | Next evidence needed |
| --- | --- | --- | --- | --- |
| Demo loopback default and public-bind guard | `scripts/ci/metadata_conformance_smoke.sh`, `docs/privacy/DOC-G5-003_Envelope_Transport_Profile_v0.1.0_DRAFT.md` | `PROVEN_EXECUTABLE` | Covers the qshield demo relay only. | Keep as baseline regression; do not treat as public internet service proof. |
| Demo relay token requirement | `scripts/ci/metadata_conformance_smoke.sh`, `scripts/ci/demo_cli_smoke.sh`, NA-0244 evidence | `PROVEN_EXECUTABLE` | Covers selected demo endpoints and token-shape rejects; not a production auth proof. | Extend future service/deployment proof to proxy/access logs and deployment auth policy. |
| Sanitized demo errors | NA-0244 evidence, `scripts/ci/metadata_conformance_smoke.sh` | `PROVEN_EXECUTABLE` | Selected malformed JSON, content-type, auth, and padding/config rejects only. | Broaden error-normalization coverage to all public demo/service-visible reject families. |
| Optional padding validation | `docs/privacy/DOC-G5-003_Envelope_Transport_Profile_v0.1.0_DRAFT.md`, NA-0244 evidence | `PROVEN_EXECUTABLE` for validation; `NOT_READY` for default policy | Padding exists as explicit optional demo behavior; default policy, profile tuning, and cross-surface costs remain open. | Define padding defaults, profile negotiation/UX, test vectors, and cost/size leakage analysis. |
| Stable demo identifiers | `docs/privacy/DOC-G5-001_Metadata_Threat_Model_v1.0.0_DRAFT.md`, `DOC-G5-002` | `DOCS_ONLY` as known leakage; `NOT_READY` for rotation | Stable peer IDs remain observable to relay/operator surfaces. | Design and prove rotating or opaque handles, replay boundaries, migration behavior, and operator/debug ergonomics. |
| Contact graph hiding | `DOC-G5-001`, `DOC-G5-002`, `DOC-G5-004` | `NOT_READY` | Relay/operator-visible communication relationships remain visible in current demo/service surfaces. | Define what can be minimized without claiming anonymity; add measurable linkability reduction tests if approved. |
| Timing and polling leakage | `DOC-G5-001`, `DOC-G5-003`, `formal/README.md` scope limits | `NOT_READY` | No batching, jitter, cover traffic, or timing-analysis resistance is proven. | Design bounded batching/jitter knobs and tests, or keep as explicit residual leakage. |
| Size leakage beyond optional buckets | `DOC-G5-002`, `DOC-G5-003`, NA-0244 evidence | `FUTURE_GATE` | Optional buckets do not prove a release-ready size-hiding profile. | Add profile-level bucket selection, default/off behavior, regression vectors, and reviewer-facing leakage notes. |
| Local state and journal metadata | `DOC-G5-004`, qsc local-state tests referenced in `TRACEABILITY.md` | `DOCS_ONLY` plus bounded executable support in earlier lanes | Local stores still contain peer/session/timeline/attachment metadata needed for truthful operation. | Create focused phase-2 local-state minimization inventory and tests for passive-output and evidence-artifact boundaries. |
| qsl-server route and queue metadata | NA-0273, NA-0275, NA-0277, NA-0280, NA-0281, NA-0287 evidence | `PROVEN_EXECUTABLE` for local hardening; `FUTURE_GATE` for deployment metadata | Route timing, queue/backpressure, route-token presence, and access-log/proxy metadata remain deployment risks. | Deployment-profile proof for proxy/access logs, metrics labels, source-IP policy, and retained support bundles. |
| qsl-attachments session/object metadata | NA-0274, NA-0282, NA-0283, NA-0284, NA-0286, NA-0287 evidence | `PROVEN_EXECUTABLE` for local opaque-ciphertext/logging boundaries; `FUTURE_GATE` for deployment metadata | Session/object journals, locator refs, capability scope, disk/backup artifacts, and metrics labels need deployment proof. | Deployment and backup/restore operations drills with secret-safe logs, metrics, and retained artifacts. |
| Public internet metadata behavior | NA-0287 service production-gate map | `NOT_READY` | No public ingress, TLS/proxy, firewall/ACL, source-IP, or public abuse metadata proof exists. | Public-ingress metadata evidence only if separately authorized after service deployment-profile proof. |
| Metadata external review | NA-0250 package, this plan | `FUTURE_GATE` | No external reviewer findings or disposition are recorded for metadata phase-2. | Include residual metadata matrix and test commands in a refreshed review package, then record reviewer findings separately. |
| Anonymity / metadata-free / untraceable claims | `README.md`, public evidence docs, this plan | `OUT_OF_SCOPE` and `NOT_READY` as claims | QSL is not an anonymity network and does not eliminate metadata. | Do not pursue claim changes without a separate approved program scope; keep public copy conservative. |

## External Review Readiness Matrix

| Package element | Evidence source | Classification | Remaining gap | Next evidence needed |
| --- | --- | --- | --- | --- |
| External review package exists | `docs/public/EXTERNAL_REVIEW_PACKAGE.md`, NA-0250 evidence | `DOCS_ONLY` | Package exists, but recent NA-0265 through NA-0288 evidence needs alignment. | Refresh package references, command dates, service boundary rows, and metadata gap rows. |
| Release readiness evidence map exists | `docs/public/RELEASE_READINESS_EVIDENCE_MAP.md` | `DOCS_ONLY` | Current map is conservative but needs NA-0287/NA-0288 alignment. | Add gap-plan references and keep external review completion `NOT_READY`. |
| Canonical goals and traceability | `GOALS.md`, `TRACEABILITY.md`, `DECISIONS.md` | `DOCS_ONLY` with executable links | Traceability is broad but reviewer path is dense. | Create a concise reviewer index or curated route through canonical specs, vectors, and model limits. |
| Formal/model checks | `formal/README.md`, `python3 formal/run_model_checks.py` | `PROVEN_EXECUTABLE` for bounded models | Models do not prove full cryptographic secrecy, authentication, or production correctness. | State scope limits in the review package and add future model slices only with explicit claims. |
| Suite-2 vectors and refimpl tests | `inputs/suite2/vectors/**`, `tools/refimpl/**`, CI suite contexts | `PROVEN_EXECUTABLE` for covered categories | Reviewer reproduction across clean Linux/macOS remains uneven. | Refresh clean-host and cross-host reviewer commands with exact commit and prerequisites. |
| Public demo evidence | NA-0246, NA-0259, NA-0260, NA-0262, NA-0263, NA-0266 evidence | `PROVEN_EXECUTABLE` for non-production demo surfaces | Demo proof is not production relay/service or anonymity proof. | Keep demo rows explicitly bounded and separate from service production gates. |
| Service hardening evidence | NA-0270 through NA-0287 evidence | `PROVEN_EXECUTABLE` for local service-hardening categories; `NOT_READY` for production operation | Package does not yet integrate the latest qsl-server/qsl-attachments production-gate map. | Refresh package with NA-0287 classifications and deployment gaps. |
| Metadata phase-2 gap plan | This document | `DOCS_ONLY` | This plan maps gaps but does not implement phase-2 mitigations. | Promote separate executable metadata lanes for rotation, padding defaults, retention/purge, and error normalization. |
| Reviewer scope and questions | `docs/public/EXTERNAL_REVIEW_PACKAGE.md` | `DOCS_ONLY` | Questions exist, but no reviewer engagement, scope acceptance, or findings are recorded. | Add reviewer-ready scope sheet and findings/disposition template. |
| External reviewer findings | none in current tree | `NOT_READY` | No external cryptographic review completion evidence exists. | Record reviewer identity/scope, findings, fixes, residuals, and disposition in a later evidence lane. |
| Public claim boundary | `README.md`, `RELEASE_READINESS_EVIDENCE_MAP.md`, `EXTERNAL_REVIEW_PACKAGE.md`, Suite-2 claim boundary | `DOCS_ONLY` | Boundaries are clear, but stale dates and missing NA-0288 references can create reviewer confusion. | Align public docs, then run overclaim/link/leak/goal checks. |

## Metadata Threat / Claim Boundaries

What is proven:

- bounded demo loopback defaults and explicit unsafe public-bind acknowledgement;
- token-gated demo relay endpoints and selected fail-closed demo rejects;
- selected sanitized error bodies and no-secret leakage in demo negative tests;
- optional padding metadata validation on the demo relay;
- local qsl-server hardening for selected transport-only relay behaviors;
- local qsl-attachments hardening for opaque-ciphertext, capability, retention,
  quota, recovery, and secret-safe local logging behaviors.

What is not proven:

- anonymity;
- metadata-free messaging;
- untraceability;
- traffic-analysis resistance;
- contact graph hiding;
- IP/location hiding;
- production service metadata posture;
- public internet metadata posture;
- deployment proxy/access-log/metrics metadata posture;
- external review completion.

What must not be claimed:

- QSL is anonymous.
- QSL is metadata-free or metadata-eliminated.
- QSL is untraceable.
- QSL is traffic-analysis resistant.
- Current service evidence proves public internet readiness.
- Current service evidence proves production readiness.
- A review package exists, therefore external review is complete.

## External Review Boundary

The external review package exists and is suitable as a starting point for
reviewer orientation. Package existence is not review completion.

External cryptographic review is not complete unless a later evidence lane
records reviewer scope, reviewer findings, disposition, residual risk, and
updated public claim boundaries. No such completion evidence exists in the
current tree.

## Service Evidence Relationship

qsl-server and qsl-attachments hardening evidence helps the production-boundary
story by proving local service invariants and by making deployment gates more
specific. It does not prove metadata-free messaging, anonymity, untraceability,
production operation, public internet exposure safety, or external review
completion.

## Demo Evidence Relationship

Public demo evidence helps reproducibility and shows bounded non-production
behavior for the qshield demo. It does not prove production relay/service
operation, metadata-free messaging, anonymity, untraceability, full traffic
analysis resistance, or external review completion.

## Next Recommended Evidence Lanes

1. Refresh the external review package and release evidence map with this gap
   plan, recent service evidence, exact current commands, stale-reference
   cleanup, and claim-boundary alignment.
2. Define and test metadata phase-2 identifier rotation or opaque-handle
   behavior for the demo/transport surface.
3. Define padding default policy and size-leakage evidence, including explicit
   cost and UX boundaries.
4. Expand sanitized-error and retention/purge evidence across the demo and
   service-visible boundaries.
5. Add deployment-profile metadata proof for qsl-server and qsl-attachments
   only after deployment-profile lanes are explicitly authorized.
6. Record external reviewer findings and dispositions in a later evidence lane.

## Prohibited Wording List

The following phrases must not appear as affirmative claims:

- production-ready
- deployment-ready
- production relay ready
- production attachment ready
- public internet ready
- external review complete
- externally reviewed
- review complete
- metadata-free
- anonymity
- anonymous messaging
- untraceable
- quantum-proof
- proven true Triple Ratchet

These phrases may appear only when explicitly negated, explicitly described as
future or unproven, or listed in a prohibited-wording section.

## Explicit No-Anonymity Claim

NA-0288 does not claim anonymity. The current metadata posture remains bounded
metadata minimization with explicit residual leakage.

## Explicit No-Metadata-Free Claim

NA-0288 does not claim metadata-free or metadata-eliminated messaging. Timing,
size, stable identifiers, relay-visible metadata, local-state metadata, and
deployment metadata remain visible or future-gated.

## Explicit No-Untraceable Claim

NA-0288 does not claim untraceability or traffic-analysis resistance. Contact
graph, timing, size, IP/location, route, relay, and attachment metadata gaps
remain explicit.

## Explicit No-External-Review-Complete Claim

NA-0288 does not claim external review completion. The package exists and can
be refreshed for reviewers, but no completed external review findings or
disposition are recorded.

## Explicit No-Production-Readiness Claim

NA-0288 does not claim production readiness, deployment readiness, production
relay readiness, production attachment readiness, public internet readiness, or
production backup/restore readiness.

## Not Implemented In NA-0288

NA-0288 does not implement:

- identifier rotation;
- padding default changes;
- batching, jitter, cover traffic, or timing mitigations;
- retention/purge behavior;
- service deployment behavior;
- qsl-server or qsl-attachments changes;
- qsc/qshield/qsl-client changes;
- protocol, wire, crypto, auth, or state-machine changes;
- external review execution;
- website or public-claim upgrades.
