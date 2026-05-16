Goals: G1, G3, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-16
Replaces: n/a
Superseded-By: n/a

# Website Implementation Handoff

Directive: QSL-DIR-2026-05-06-037 / NA-0251

## Status And Scope

This is a handoff package only. It prepares safe, evidence-bounded instructions for a later external website repository implementation lane.

This document does not edit the external website repository, does not edit any qsl-protocol website implementation source, and does not authorize production, runtime, protocol, crypto, demo, service, public-safety, workflow, branch-protection, or Cargo changes.

The implementation target for a future lane is the external website repository. That future lane must re-check live website source, branch state, public evidence links, and public copy before editing.

## NA-0295 Landing Page and Evidence Visuals Addendum

[NA-0295 website landing page handoff and evidence visuals plan](../governance/evidence/NA-0295_website_landing_evidence_visuals_plan.md)
updates this handoff with a concrete landing-page information architecture,
section-by-section copy seeds, evidence links, visual/storyboard guidance,
claim-matrix replacements, and future implementation stop conditions.

NA-0295 is planning only. It does not implement website copy, mutate the live
website, mutate an external website repository, deploy a site, generate media
assets, or authorize stronger public claims. Future website work must first
verify the exact website source and must keep production readiness, public
internet readiness, external review completion, anonymity, metadata-free
messaging, untraceability, and metadata phase-2 completion as visible open
gates.

## NA-0296 Source Verification Readiness Addendum

[NA-0296 website source verification readiness audit](../governance/evidence/NA-0296_website_source_verification_readiness_audit.md)
classifies the current website state as `PARTIAL_READY_SOURCE_UNVERIFIED`.

Read-only public inspection found two official or official-looking public
surfaces:

- `quantumshieldlabs.org` is linked from the official `QuantumShieldLabs`
  GitHub organization profile and presents a research-stage public evidence
  trail for QSL protocol.
- `quantumshieldlabs.dev` presents broader company/product and adjacent project
  copy that must remain separated from qsl-protocol release evidence.

The exact website source repository, source branch, build command, deployment
target, preview flow, and rollback path remain unverified. A future lane must
resolve those blockers before editing or deploying website copy. This addendum
does not claim that the website has been updated and does not authorize website
implementation.

## NA-0297 Source Blocker Resolution Addendum

[NA-0297 website source blocker resolution audit](../governance/evidence/NA-0297_website_source_blocker_resolution_audit.md)
classifies the current website/source state as `OPERATOR_ACTION_REQUIRED` and
the claim/link scan state as `PARTIAL_READY_NEEDS_SOURCE`.

Read-only follow-up found:

- `quantumshieldlabs.org` remains the official organization-linked public QSL
  evidence surface. Live structured data and historical NA-0255 response
  evidence point at `Tebbens4832/QuantumShield` and Cloudflare Pages, but that
  source is private in current GitHub metadata and was not source-inspected in
  this directive.
- `quantumshieldlabs.dev` remains a broader company/product static surface
  served through Hostinger/LiteSpeed. Its exact source repository, branch,
  build command, deployment target, preview flow, and rollback path remain
  unverified.
- `mbennett-labs/qsl` remains only an unconfirmed public source candidate.

Before any website implementation lane, the operator must provide the exact
target surface, source repo, branch, build command, preview/staging path,
deployment target, rollback process, edit authority, and whether Codex may open
a website PR or run a read-only local build. This addendum does not claim that
the website has been updated, does not authorize website implementation, and
does not authorize stronger public claims.

## Source-Of-Truth Artifacts

Primary public-claim sources:

- [Website claim matrix](WEBSITE_CLAIM_MATRIX.md)
- [Website update plan](WEBSITE_UPDATE_PLAN.md)
- [NA-0295 website landing page handoff and evidence visuals plan](../governance/evidence/NA-0295_website_landing_evidence_visuals_plan.md)
- [NA-0296 website source verification readiness audit](../governance/evidence/NA-0296_website_source_verification_readiness_audit.md)
- [NA-0297 website source blocker resolution audit](../governance/evidence/NA-0297_website_source_blocker_resolution_audit.md)
- [Suite-2 Triple-Ratchet claim boundary](SUITE2_TRIPLE_RATCHET_CLAIM_BOUNDARY.md)
- [External review package](EXTERNAL_REVIEW_PACKAGE.md)
- [Release-readiness evidence map](RELEASE_READINESS_EVIDENCE_MAP.md)

Repo evidence sources:

- [GOALS.md](../../GOALS.md)
- [ROADMAP.md](../../ROADMAP.md)
- [TRACEABILITY.md](../../TRACEABILITY.md)
- [DECISIONS.md](../../DECISIONS.md)
- [Demo acceptance criteria](../demo/DEMO_ACCEPTANCE_CRITERIA.md)
- [KT-negative public demo readiness](../demo/KT_NEGATIVE_PUBLIC_DEMO_READINESS.md)
- [Attachment public demo readiness](../demo/ATTACHMENT_PUBLIC_DEMO_READINESS.md)
- [Conformance vector prioritization](../conformance/CONFORMANCE_VECTOR_PRIORITIZATION.md)
- [Metadata threat model](../privacy/DOC-G5-001_Metadata_Threat_Model_v1.0.0_DRAFT.md)
- [Envelope transport profile](../privacy/DOC-G5-003_Envelope_Transport_Profile_v0.1.0_DRAFT.md)
- [NA-0245 website truthfulness audit](../governance/evidence/NA-0245_website_truthfulness_audit.md)
- [NA-0250 external review release-readiness audit](../governance/evidence/NA-0250_external_review_release_readiness_audit.md)

## Page-By-Page Checklist

### Homepage

- Add a visible QSL protocol status band near any product or security headline.
- State that QSL protocol is research-stage and non-production.
- State that external live products under the same brand do not prove QSL protocol release readiness.
- Replace any broad "not demos / not prototypes" wording with product-scoped language that excludes QSL protocol and its demo.
- Add last-verified dates to counters, milestones, and roadmap-style cards.
- Link to the public evidence section before readers reach external live-product claims.

### Project / Protocol Status Section

- Add a standalone QSL protocol section.
- Include Goals G1, G3, and G5 in plain language, with G4 release-gate context where useful.
- Link GOALS.md, ROADMAP.md, DECISIONS.md, TRACEABILITY.md, release-readiness evidence, and external review package.
- State that release readiness is not claimed until all G1-G5 gates, conformance evidence, formal/model checks, demo criteria, and public-safety gates support it.
- Avoid implying qsl-server, qsl-attachments, QuantumShield API, CrawDaddy, SELARIX, or consulting work is protocol release proof.

### Demo / GUI Section

- State that the demo is a non-production acceptance surface.
- Mention the current local proof shape: peer init, relay authorization, establish, send, receive/decrypt, bounded negative rejects, demo-only KT-negative verifier proof, and demo-only encrypted attachment descriptor/fetch/decrypt/integrity proof.
- Link demo acceptance criteria and the external review package.
- For QSC desktop, state guided prototype readiness only.
- Keep keychain-backed active operations, handshake/session-establish UI, production attachment service claims, and production GUI claims out of scope unless a later evidence lane proves them.

### Metadata / Privacy Section

- Use metadata minimization wording only.
- State that the current demo profile is not an anonymity system.
- State that stable identifiers, timing, size, relay-visible metadata, and IP-level metadata remain observable.
- Link the metadata threat model and envelope/transport profile.
- Keep website form privacy/legal claims separate from QSL protocol metadata-minimization claims.

### Suite-2 / Triple-Ratchet Wording Section

- Use "Suite-2 / Triple-Ratchet-style" with research-stage and release-gated qualifiers.
- Explain that QSL combines classical and PQ message-key material in a hybrid per-message design, backed by repo evidence for covered paths.
- Do not claim equivalence to Signal production protocol, Signal formal proofs, or a proven true Triple Ratchet.
- Do not say "quantum-proof" or "production-ready Triple Ratchet."

### External Products / CrawDaddy / SELARIX Separation Section

- Label CrawDaddy, SELARIX, QuantumShield API, BTC Battle, crypto-scanner, risk calculator, and consulting pages as external products/services or adjacent offerings unless a current qsl-protocol evidence source says otherwise.
- Add a standard boundary note: "External product/service. Does not prove QSL protocol production readiness."
- Do not use external product uptime, dashboards, scanners, payment flows, or agent status as QSL protocol evidence.
- Do not conflate QuantumShield API with qsl-server or qsl-attachments evidence.

### Healthcare / PQC Consulting Section

- Treat healthcare PQC readiness, risk calculators, playbooks, and consulting as service claims, not qsl-protocol proof.
- Use "assessment", "planning", "migration roadmap", and "readiness review" unless certification evidence exists.
- Keep NIST/deadline wording source-linked and precise.
- Remove or refresh stale availability windows.
- Do not imply qsl-protocol is certified, deployed, or production-ready for healthcare.

### Evidence Links Section

- Add an evidence page or section with date-stamped links to repo evidence.
- Separate evidence groups: protocol goals, release posture, decisions, traceability, demo evidence, metadata evidence, Suite-2 claim boundary, external review package, and public-safety status.
- Use short SHA references when evidence needs a commit identifier.
- Do not paste raw secrets, endpoint tokens, auth headers, or long-hex dumps.

### Roadmap / Release-Readiness Section

- State that current release readiness is partial and conservative.
- Link the release-readiness evidence map.
- Keep open gaps visible: external cryptographic review completion, production KT deployment, live qshield KT evidence ingestion, cross-host/private-network attachment proof, production relay/service hardening, qsl-attachments hardening, metadata phase-2, and production desktop release readiness.
- Avoid "deployment-ready", "production release", or "ready for production users" until a later release gate authorizes it.

## Safe Copy Snippets

Use these as copy seeds. The website implementation lane may make them shorter, but must not make them stronger.

Research-stage QSL protocol:

> QSL protocol is a research-stage, non-production protocol and demo system. Public repository evidence tracks its goals, decisions, conformance priorities, demo acceptance, and CI gates.

Triple-Ratchet-style Suite-2 wording:

> QSL Suite-2 is a research-stage Triple-Ratchet-style hybrid messaging design. Current evidence is release-gated and does not claim production readiness or equivalence to any production messaging protocol.

Demo / non-production posture:

> The current demo is a non-production acceptance surface for local, inspectable protocol-adjacent behavior. It proves selected positive and fail-closed reject flows; it is not deployment readiness.

Metadata-minimization baseline:

> QSL has a metadata minimization lane. The current demo profile is not an anonymity system, and timing, size, stable identifiers, relay-visible metadata, and IP-level metadata remain observable.

Public evidence links:

> Evidence links: goals, roadmap, decisions, traceability, demo acceptance criteria, conformance priorities, metadata profile, Suite-2 claim boundary, release-readiness map, and external review package.

## Prohibited Copy Snippets

Do not use these phrases for QSL protocol, Suite-2, the demo, or the GUI unless a later authoritative release decision explicitly changes the boundary:

- "production-ready"
- "proven true Triple Ratchet"
- "quantum-proof"
- "metadata-free"
- "anonymity"
- "anonymous messaging"
- "untraceable communications"
- "deployment-ready"
- "production deployment ready"
- "not demos / not prototypes" when referring to QSL protocol, QSL demo, Suite-2, or QSC desktop prototype
- "QSL protocol is shipped in production"
- "external review complete"
- "production attachment ready"
- "production relay ready"

## Evidence Link Map

| Website topic | Required evidence link | Safe interpretation |
| --- | --- | --- |
| QSL protocol goals | [GOALS.md](../../GOALS.md) | Release gates and non-regression requirements, not release approval. |
| Current posture | [ROADMAP.md](../../ROADMAP.md) | Research-stage and non-production posture. |
| Decisions | [DECISIONS.md](../../DECISIONS.md) | Governance record, including D-0469 after NA-0251. |
| Traceability | [TRACEABILITY.md](../../TRACEABILITY.md) | Mapping from goals to specs, modules, tests, and evidence. |
| Claim matrix | [WEBSITE_CLAIM_MATRIX.md](WEBSITE_CLAIM_MATRIX.md) | Public claim risks and required website copy boundaries. |
| Update plan | [WEBSITE_UPDATE_PLAN.md](WEBSITE_UPDATE_PLAN.md) | Site information architecture and implementation backlog. |
| Suite-2 wording | [SUITE2_TRIPLE_RATCHET_CLAIM_BOUNDARY.md](SUITE2_TRIPLE_RATCHET_CLAIM_BOUNDARY.md) | Research-stage Triple-Ratchet-style wording only. |
| External review | [EXTERNAL_REVIEW_PACKAGE.md](EXTERNAL_REVIEW_PACKAGE.md) | Reviewer package, not production approval. |
| Release readiness | [RELEASE_READINESS_EVIDENCE_MAP.md](RELEASE_READINESS_EVIDENCE_MAP.md) | Partial/proven/not-ready status by goal and surface. |
| Demo | [DEMO_ACCEPTANCE_CRITERIA.md](../demo/DEMO_ACCEPTANCE_CRITERIA.md) | Non-production acceptance surface. |
| KT-negative demo | [KT_NEGATIVE_PUBLIC_DEMO_READINESS.md](../demo/KT_NEGATIVE_PUBLIC_DEMO_READINESS.md) | Non-production verifier/vector proof only; not production KT deployment. |
| Attachment demo | [ATTACHMENT_PUBLIC_DEMO_READINESS.md](../demo/ATTACHMENT_PUBLIC_DEMO_READINESS.md) | Non-production encrypted descriptor/payload proof only; not qsl-server or qsl-attachments production readiness. |
| Conformance | [CONFORMANCE_VECTOR_PRIORITIZATION.md](../conformance/CONFORMANCE_VECTOR_PRIORITIZATION.md) | Priority vectors/tests before release claims. |
| Metadata threat model | [DOC-G5-001](../privacy/DOC-G5-001_Metadata_Threat_Model_v1.0.0_DRAFT.md) | Not anonymity; residual metadata remains. |
| Envelope/transport profile | [DOC-G5-003](../privacy/DOC-G5-003_Envelope_Transport_Profile_v0.1.0_DRAFT.md) | Demo transport profile and residual leakage. |
| Website audit | [NA-0245 audit](../governance/evidence/NA-0245_website_truthfulness_audit.md) | No website implementation; audit only. |
| Release audit | [NA-0250 audit](../governance/evidence/NA-0250_external_review_release_readiness_audit.md) | No production approval; package only. |

## External Repo Implementation Checklist

- Re-check the external website repo branch, default branch, and working tree before editing.
- Add the QSL protocol status band before or adjacent to live-product sections.
- Add product/service taxonomy labels and boundary notes.
- Add or update the public evidence page/section.
- Replace broad "not demos / not prototypes" language with scoped external-product language.
- Add metadata-minimization copy with explicit no-anonymity and no-metadata-elimination boundaries.
- Add Suite-2 / Triple-Ratchet-style copy using research-stage qualifiers.
- Separate CrawDaddy, SELARIX, QuantumShield API, BTC Battle, crypto-scanner, risk calculator, healthcare consulting, and playbook from qsl-protocol release evidence.
- Refresh stale timeline, availability, and counter wording with last-verified dates.
- Run static overclaim phrase scans.
- Run link checks for all public evidence links.
- Capture screenshots of homepage status band, product taxonomy labels, evidence links, metadata/privacy section, and Suite-2 wording.
- Include rollback notes for every copy change or feature flag used in the website repo.

## Suggested External Website PR

Suggested title:

`Add QSL protocol evidence-boundary copy and public evidence links`

Suggested body:

```md
Goals: G1, G3, G5

Impact:
- Adds visible QSL protocol research-stage / non-production status.
- Separates external product/service claims from qsl-protocol evidence.
- Adds evidence links for goals, roadmap, decisions, traceability, demo, metadata, Suite-2 claim boundary, release-readiness map, and external-review package.

No-regression:
- No production-readiness claim.
- No proven true Triple Ratchet claim.
- No anonymity or metadata-elimination claim.
- No qsl-server, qsl-attachments, CrawDaddy, SELARIX, QuantumShield API, or consulting conflation with qsl-protocol evidence.

Tests/Vectors:
- Static overclaim phrase scan.
- Link check for public evidence URLs.
- Screenshot review of homepage status band, product taxonomy labels, evidence section, metadata/privacy copy, and Suite-2 wording.
- Claim-matrix checklist review for MUST_FIX rows.
```

## Future External Website Directive Template

```md
Directive: Public Website Evidence-Boundary Implementation

Allowed repo:
- External website repo only.

Allowed scope:
- Website content/components/styles needed for QSL protocol status, product taxonomy labels, evidence links, metadata/privacy boundary copy, Suite-2 claim boundary copy, and stale timeline/counter fixes.

Forbidden scope:
- qsl-protocol repo edits unless separately authorized.
- Protocol/runtime/crypto/demo/service implementation changes.
- Branch-protection/public-safety changes.
- Production-readiness, proven true Triple Ratchet, anonymity, metadata-free, quantum-proof, or deployment-ready claims.

Required sources:
- qsl-protocol docs/public/WEBSITE_IMPLEMENTATION_HANDOFF.md
- qsl-protocol docs/public/WEBSITE_CLAIM_MATRIX.md
- qsl-protocol docs/public/WEBSITE_UPDATE_PLAN.md
- qsl-protocol docs/public/SUITE2_TRIPLE_RATCHET_CLAIM_BOUNDARY.md
- qsl-protocol docs/public/EXTERNAL_REVIEW_PACKAGE.md
- qsl-protocol docs/public/RELEASE_READINESS_EVIDENCE_MAP.md

Validation:
- Static overclaim phrase scan.
- Link check.
- Screenshot review.
- Claim-matrix MUST_FIX checklist.
- Scope guard proving only website repo files changed.
```

## Static Overclaim Phrase Scan Recommendations

Run scans over rendered source, markdown, content files, and generated HTML when practical. Treat matches as review blockers unless they are in a prohibited-phrases list, a test fixture, or a historical audit context.

Recommended phrases:

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
- `production attachment ready`
- `qsl-attachments production ready`
- `production relay`

Recommended boundary phrases that should exist at least once:

- `research-stage`
- `non-production`
- `metadata minimization`
- `not an anonymity system`
- `external product`
- `does not prove QSL protocol production readiness`
- `release-gated`

## Rollback / Verification Checklist

- Confirm the external website PR touches only website repository files.
- Confirm no qsl-protocol files changed in the website implementation lane unless separately authorized.
- Confirm no public-safety, branch-protection, workflow, Cargo, protocol, runtime, crypto, demo, or service settings changed.
- Run the static overclaim phrase scan.
- Run a public link check for evidence links.
- Review screenshots for the homepage, product/service cards, QSL protocol status, metadata/privacy copy, Suite-2 wording, and evidence links.
- Compare against the MUST_FIX rows in the claim matrix.
- Confirm prohibited phrases appear only in blocked-copy lists or audit/test contexts.
- Confirm rollback plan identifies the website commit or deployment that can be reverted.

## Known Uncertainties

- The external website source branch and deployment pipeline must be re-verified in the future website lane.
- Live public website copy may have changed since the NA-0245 audit.
- NIST/deadline language depends on current external source status and must be rechecked during website implementation.
- External products and services may have their own evidence or operating claims outside qsl-protocol; those claims must not be imported as QSL protocol proof without separate evidence.
- Production KT deployment, live qshield KT evidence ingestion, cross-host/private-network attachment proof, production relay hardening, qsl-attachments hardening, production desktop release readiness, metadata phase-2, and external cryptographic review completion remain open release-readiness gaps.
