Goals: G1, G3, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-07
Replaces: n/a
Superseded-By: n/a

# External Website Implementation Directive

Directive: QSL-DIR-2026-05-07-042 / NA-0253

## Status And Scope

This is an operator-ready implementation planning package only. It does not edit
the external website repository, does not edit qsl-protocol website
implementation source, and does not authorize production deployment.

This qsl-protocol lane prepares evidence-boundary instructions for a future
external website repository lane. That future lane must independently verify the
live website source, branch, build pipeline, public evidence links, and rendered
copy before editing.

Forbidden in this planning lane:

- external website repository edits
- qsl-protocol website implementation edits
- protocol, runtime, crypto, auth, state-machine, demo, service, qsl-server,
  qsl-attachments, qsc-desktop, app, tool, input, formal, workflow,
  public-safety, branch-protection, Cargo, or script changes
- production-readiness, proven true Triple Ratchet, quantum-proof, anonymity,
  metadata-free, or deployment-ready claims

## External Website Repository Prerequisites

- Exact external website repository: unknown from the qsl-protocol evidence
  inspected in this lane. The future operator must identify and verify the
  repository that deploys `quantumshieldlabs.dev` before editing.
- Local clone/worktree requirement: create a clean local clone or worktree of
  that verified website repository. Record `pwd`, branch, `git status`, remotes,
  default branch, current SHA, and deploy pipeline before edits.
- Branch naming: use a bounded branch such as
  `qsl-protocol-evidence-boundary-copy`.
- Production deployment: do not deploy to production without normal review,
  screenshots, link checks, and explicit operator approval.
- Backup/snapshot/rollback: record the pre-edit commit, current deployment
  identifier if available, and rollback command or hosting-provider rollback
  path before publishing any copy change.

## Page-By-Page Implementation Tasks

### Homepage

- Add a visible QSL protocol status band before or adjacent to live-product
  sections.
- State that QSL protocol is research-stage and non-production.
- State that live external products under the same brand do not prove QSL
  protocol production readiness.
- Replace broad "not demos / not prototypes" wording with product-scoped
  language that excludes QSL protocol, QSL demo, and QSC desktop prototype.
- Add last-verified dates to counters, timelines, and roadmap cards.
- Link the evidence page or evidence section before readers reach live-product
  proof claims.

### Protocol Status

- Add a standalone QSL protocol status section.
- Summarize G1, G3, and G5 in public-safe terms, with G4 as the release gate.
- Link GOALS, ROADMAP, DECISIONS, TRACEABILITY, release-readiness evidence,
  and external-review package artifacts.
- State that release readiness is not claimed until G1-G5 gates, conformance
  evidence, formal/model checks, demo criteria, and public-safety gates support
  it.
- Do not present qsl-server, qsl-attachments, QuantumShield API, CrawDaddy,
  SELARIX, BTC Battle, crypto-scanner, risk calculator, healthcare consulting,
  or playbook claims as QSL protocol release proof.

### Demo / GUI

- State that the demo is a non-production acceptance surface.
- Describe only the current bounded proof shape: local loopback peers, explicit
  relay authorization, establish, send, receive/decrypt, and bounded negative
  rejects.
- Link demo acceptance criteria and the external review package.
- For QSC desktop, state guided prototype readiness only.
- Keep keychain-backed active operations, handshake/session-establish UI,
  attachments UI, native package proof, and production GUI claims out of scope
  unless a later evidence lane proves them.

### Metadata / Privacy

- Use "metadata minimization" wording only.
- State that the current demo profile is not an anonymity system.
- State that stable identifiers, timing, size, relay-visible metadata, and
  IP-level metadata remain observable.
- Link the metadata threat model and envelope/transport profile.
- Keep website form privacy/legal claims separate from QSL protocol
  metadata-minimization claims.

### Suite-2 / Triple-Ratchet Wording

- Use "Suite-2 / Triple-Ratchet-style" with research-stage and release-gated
  qualifiers.
- State that QSL combines classical and PQ message-key material in a hybrid
  per-message design for covered evidence paths.
- Do not claim equivalence to Signal production protocol, Signal formal proofs,
  or a proven true Triple Ratchet.
- Do not say "quantum-proof" or "production-ready Triple Ratchet."

### External Product Separation

- Label CrawDaddy, SELARIX, QuantumShield API, BTC Battle, crypto-scanner, risk
  calculator, playbook, and consulting pages as external products/services or
  adjacent offerings unless a current qsl-protocol evidence source says
  otherwise.
- Add a standard boundary note: "External product/service. Does not prove QSL
  protocol production readiness."
- Do not use external product uptime, dashboards, scanners, payment flows, or
  agent status as QSL protocol evidence.
- Do not conflate QuantumShield API with qsl-server or qsl-attachments evidence.

### Healthcare / PQC Consulting

- Treat healthcare PQC readiness, risk calculators, playbooks, and consulting
  as service claims, not qsl-protocol proof.
- Use "assessment", "planning", "migration roadmap", and "readiness review"
  unless certification evidence exists.
- Keep NIST/deadline wording source-linked and precise.
- Remove or refresh stale availability windows.
- Do not imply qsl-protocol is certified, deployed, or production-ready for
  healthcare.

### Evidence Links

- Add an evidence page or section with date-stamped links to qsl-protocol
  evidence.
- Separate evidence groups: protocol goals, release posture, decisions,
  traceability, demo evidence, metadata evidence, Suite-2 claim boundary,
  external review package, release-readiness map, and public-safety status.
- Use short SHA references when evidence needs a commit identifier.
- Do not paste raw secrets, endpoint tokens, auth headers, or long-hex dumps.

### Roadmap / Release Readiness

- State that current release readiness is partial and conservative.
- Link the release-readiness evidence map.
- Keep open gaps visible: external cryptographic review completion, KT-negative
  demo readiness, attachment demo readiness, native desktop package proof,
  production relay/service hardening, qsl-attachments hardening, metadata
  phase 2, and cross-host reproducibility.
- Avoid "deployment-ready", "production release", or "ready for production
  users" until a later release gate authorizes it.

## Safe Copy Snippets

Use these as seeds. The website implementation lane may make them shorter, but
must not make them stronger.

Research-stage QSL protocol:

> QSL protocol is a research-stage, non-production protocol and demo system. Public repository evidence tracks its goals, decisions, conformance priorities, demo acceptance, and CI gates.

Suite-2 / Triple-Ratchet-style:

> QSL Suite-2 is a research-stage Triple-Ratchet-style hybrid messaging design. Current evidence is release-gated and does not claim production readiness or equivalence to any production messaging protocol.

Demo posture:

> The current demo is a non-production acceptance surface for local, inspectable protocol-adjacent behavior. It proves selected positive and fail-closed reject flows; it is not deployment readiness.

Metadata minimization:

> QSL has a metadata minimization lane. The current demo profile is not an anonymity system, and timing, size, stable identifiers, relay-visible metadata, and IP-level metadata remain observable.

External product boundary:

> External product/service. Does not prove QSL protocol production readiness.

## Prohibited Copy Snippets

Do not use these phrases for QSL protocol, Suite-2, the demo, or the GUI unless
a later authoritative release decision explicitly changes the boundary:

- "production-ready"
- "production ready"
- "proven true Triple Ratchet"
- "true Triple Ratchet"
- "quantum-proof"
- "metadata-free"
- "metadata eliminated"
- "anonymity"
- "anonymous messaging"
- "untraceable communications"
- "deployment-ready"
- "production deployment ready"
- "not demos / not prototypes" when referring to QSL protocol, QSL demo,
  Suite-2, or QSC desktop prototype
- "QSL protocol is shipped in production"
- "external review complete"
- "attachment demo ready"
- "production relay ready"

## Static Overclaim Scan List

Run scans over website source, markdown/content files, rendered static output,
and generated HTML when practical. Matches are review blockers unless they are
inside a prohibited-phrases list, test fixture, or historical audit context.

Scan terms:

- `production-ready`
- `production ready`
- `proven true Triple Ratchet`
- `true Triple Ratchet`
- `quantum-proof`
- `metadata-free`
- `metadata eliminated`
- `anonymous messaging`
- `anonymity`
- `untraceable`
- `deployment-ready`
- `not demos`
- `not prototypes`
- `production deployment`
- `external review complete`
- `attachment demo ready`
- `production relay`

Boundary terms that should appear at least once in rendered QSL protocol copy:

- `research-stage`
- `non-production`
- `metadata minimization`
- `not an anonymity system`
- `external product`
- `does not prove QSL protocol production readiness`
- `release-gated`

## Evidence Link Map

| Website topic | Required evidence link | Safe interpretation |
| --- | --- | --- |
| Goals and release gates | [GOALS.md](../../GOALS.md) | Goal and non-regression requirements, not release approval. |
| Current posture | [ROADMAP.md](../../ROADMAP.md) | Research-stage and non-production posture. |
| Governance decisions | [DECISIONS.md](../../DECISIONS.md) | Governance record through D-0473 after this lane. |
| Traceability | [TRACEABILITY.md](../../TRACEABILITY.md) | Goal-to-spec/module/test/evidence map. |
| Claim matrix | [WEBSITE_CLAIM_MATRIX.md](WEBSITE_CLAIM_MATRIX.md) | Public claim risks and required copy boundaries. |
| Update plan | [WEBSITE_UPDATE_PLAN.md](WEBSITE_UPDATE_PLAN.md) | Website information architecture and implementation backlog. |
| Suite-2 wording | [SUITE2_TRIPLE_RATCHET_CLAIM_BOUNDARY.md](SUITE2_TRIPLE_RATCHET_CLAIM_BOUNDARY.md) | Research-stage Triple-Ratchet-style wording only. |
| External review | [EXTERNAL_REVIEW_PACKAGE.md](EXTERNAL_REVIEW_PACKAGE.md) | Reviewer package, not production approval. |
| Release readiness | [RELEASE_READINESS_EVIDENCE_MAP.md](RELEASE_READINESS_EVIDENCE_MAP.md) | Partial/proven/not-ready status by goal and surface. |
| Demo | [DEMO_ACCEPTANCE_CRITERIA.md](../demo/DEMO_ACCEPTANCE_CRITERIA.md) | Non-production acceptance surface. |
| Conformance | [CONFORMANCE_VECTOR_PRIORITIZATION.md](../conformance/CONFORMANCE_VECTOR_PRIORITIZATION.md) | Priority tests/vectors before release claims. |
| Metadata threat model | [DOC-G5-001](../privacy/DOC-G5-001_Metadata_Threat_Model_v1.0.0_DRAFT.md) | Not anonymity; residual metadata remains. |
| Envelope/transport profile | [DOC-G5-003](../privacy/DOC-G5-003_Envelope_Transport_Profile_v0.1.0_DRAFT.md) | Demo transport profile and residual leakage. |
| Website audit | [NA-0245 audit](../governance/evidence/NA-0245_website_truthfulness_audit.md) | Audit only; no website implementation. |
| Handoff audit | [NA-0251 audit](../governance/evidence/NA-0251_website_implementation_handoff_audit.md) | Handoff only; no external repo edit. |

## Verification Checklist

- No production-ready or deployment-ready QSL protocol wording.
- No proven true Triple Ratchet wording.
- No quantum-proof wording.
- No metadata-free, metadata-elimination, anonymity, anonymous messaging, or
  untraceable communications wording.
- No external product/QSL protocol evidence conflation.
- No QuantumShield API/qsl-server/qsl-attachments conflation.
- All evidence links resolve.
- Static overclaim scan completed and reviewed.
- Screenshots or build-preview captures exist for homepage status band,
  product/service labels, evidence links, metadata/privacy copy, and Suite-2
  wording.
- Screenshot review confirms copy is visible and not hidden behind product
  cards, modals, or inaccessible navigation.

## Rollback Checklist

- Record pre-change website repository SHA.
- Record build artifact or deployment identifier before publish if available.
- Keep every changed route/component/content file listed in the PR body.
- Confirm the hosting provider can revert to the prior deployment.
- If a prohibited claim reaches preview or production, revert the website PR or
  deployment first, then repair copy in a new reviewed PR.
- Do not compensate for bad website copy by weakening qsl-protocol evidence,
  public-safety, or branch protection.

## Future External Website Directive Template

```md
Directive: Public Website Evidence-Boundary Implementation

Allowed repo:
- Verified external website repository that deploys quantumshieldlabs.dev.

Allowed scope:
- Website content/components/styles needed for QSL protocol status,
  product/service taxonomy labels, evidence links, metadata/privacy boundary
  copy, Suite-2 claim-boundary copy, and stale timeline/counter fixes.

Forbidden scope:
- qsl-protocol repo edits unless separately authorized.
- Protocol/runtime/crypto/demo/service implementation changes.
- Branch-protection/public-safety changes.
- Production-readiness, proven true Triple Ratchet, anonymity, metadata-free,
  quantum-proof, or deployment-ready claims.

Required sources:
- qsl-protocol docs/public/EXTERNAL_WEBSITE_IMPLEMENTATION_DIRECTIVE.md
- qsl-protocol docs/public/WEBSITE_IMPLEMENTATION_HANDOFF.md
- qsl-protocol docs/public/WEBSITE_CLAIM_MATRIX.md
- qsl-protocol docs/public/WEBSITE_UPDATE_PLAN.md
- qsl-protocol docs/public/SUITE2_TRIPLE_RATCHET_CLAIM_BOUNDARY.md
- qsl-protocol docs/public/EXTERNAL_REVIEW_PACKAGE.md
- qsl-protocol docs/public/RELEASE_READINESS_EVIDENCE_MAP.md

Validation:
- Static overclaim phrase scan.
- Public evidence link check.
- Screenshot or build-preview review.
- Claim-matrix MUST_FIX checklist.
- Scope proof showing only website repository files changed.
```

## Stop Conditions For Website Implementation

Stop the future external website implementation lane if:

- the exact website repository or deployment source cannot be verified
- the website worktree is dirty with unrelated local changes
- the implementation requires qsl-protocol repo edits not separately authorized
- the implementation would edit protocol/runtime/crypto/demo/service code
- a copy change would overclaim production readiness, proven true Triple
  Ratchet, quantum-proof security, anonymity, metadata elimination, or
  deployment readiness
- evidence links cannot be made to resolve without inventing unsupported claims
- the website build or preview cannot be verified
- rollback cannot be identified before deployment
- external product/service claims must be used as QSL protocol proof

## Known Uncertainties

- The exact external website repository and deployment pipeline are not recorded
  in the qsl-protocol artifacts inspected by this lane.
- Live public website copy may have changed since the NA-0245 audit.
- NIST/deadline language must be rechecked against current authoritative sources
  in the website implementation lane.
- External products may have independent evidence outside qsl-protocol; that
  evidence must not be imported as QSL protocol proof without a separate review.
- Native desktop package proof, KT-negative demo proof, attachment demo proof,
  production relay hardening, qsl-attachments hardening, metadata phase 2, and
  complete cross-host conformance reproduction remain open release-readiness
  gaps.
