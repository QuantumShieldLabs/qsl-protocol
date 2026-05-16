Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-16
Replaces: n/a
Superseded-By: n/a

# NA-0296 Website Source Verification and Claim-Safe Implementation Readiness Audit

## Executive Summary

NA-0296 performed a read-only website/source verification and claim-boundary
audit before any website implementation work. The audit found two public QSL
website surfaces:

- `https://quantumshieldlabs.org/` is linked from the official
  `QuantumShieldLabs` GitHub organization profile and presents a research-stage
  public evidence trail for QSL protocol.
- `https://quantumshieldlabs.dev/` presents a broader company/product website
  with consulting, product, and adjacent project copy.

The exact website source repository, branch, build path, deployment path,
preview path, and rollback path were not publicly verified. No public repository
under the official `QuantumShieldLabs` GitHub organization appears to be the
website source. A public `mbennett-labs/qsl` repository was inspected as a
source candidate, but its content does not match either current live website
surface and is not verified as live source.

Readiness classification: `PARTIAL_READY_SOURCE_UNVERIFIED`.

NA-0297 should remain a source-verification and implementation-blocker
resolution lane unless a later directive identifies and authorizes the exact
website source and deploy path. This audit does not change the live website,
does not update website copy, does not create a website PR, and does not
authorize stronger public claims.

## Scope And Non-Goals

In scope:

- Read-only local handoff inventory.
- Read-only official public website and source discovery.
- Read-only live claim-boundary audit.
- Website source/build/deploy readiness classification.
- Future claim-safe implementation preconditions.
- qsl-protocol audit, testplan, handoff reference, decision, traceability, and
  journal updates.

Out of scope:

- Website or external website repository mutation.
- Website deployment, DNS, hosting, settings, forms, comments, or public posts.
- Website source commits, branches, PRs, screenshots, images, or videos.
- qsl-protocol runtime, protocol, crypto, service, demo, qsc-desktop,
  qsl-server, or qsl-attachments implementation changes.
- `.github`, script, Cargo, dependency, branch-protection, or public-safety
  configuration changes.
- Production-readiness, public-internet-readiness, external-review-complete,
  anonymity, metadata-free, untraceable, quantum-proof, unbreakable, or
  guaranteed-secure claims.

## Sources Inspected

Local qsl-protocol sources:

- `docs/public/WEBSITE_IMPLEMENTATION_HANDOFF.md`
- `docs/public/WEBSITE_CLAIM_MATRIX.md`
- `docs/public/WEBSITE_UPDATE_PLAN.md`
- `docs/governance/evidence/NA-0295_website_landing_evidence_visuals_plan.md`
- `tests/NA-0295_website_landing_evidence_visuals_testplan.md`
- `README.md`
- `START_HERE.md`
- `docs/public/INDEX.md`
- `docs/public/PUBLIC_ATTENTION_AND_VISIBILITY_STRATEGY.md`
- `docs/public/RELEASE_READINESS_EVIDENCE_MAP.md`
- `docs/public/EXTERNAL_REVIEW_PACKAGE.md`
- `docs/governance/evidence/NA-0294_public_evidence_navigation_refresh_audit.md`
- `docs/governance/evidence/NA-0287_service_production_gate_evidence_map.md`
- `docs/governance/evidence/NA-0253_external_website_implementation_planning_audit.md`
- `docs/public/EXTERNAL_WEBSITE_IMPLEMENTATION_DIRECTIVE.md`
- `TRACEABILITY.md`
- `DECISIONS.md`
- `NEXT_ACTIONS.md`
- Targeted `docs/demo/**`, `docs/design/**`, and
  `docs/governance/evidence/**` searches.

Public read-only sources:

- `https://github.com/QuantumShieldLabs`
- `https://github.com/QuantumShieldLabs/qsl-protocol`
- `https://github.com/QuantumShieldLabs/qsl-server`
- `https://github.com/QuantumShieldLabs/qsl-attachments`
- `https://quantumshieldlabs.org/`
- `https://quantumshieldlabs.dev/`
- `https://github.com/mbennett-labs`
- `https://github.com/mbennett-labs/qsl`
- Public GitHub and web searches for `quantumshieldlabs.dev`,
  `quantumshieldlabs.org`, live title strings, and website-source clues.

Private or login-gated content was not inspected. A structured-data repository
reference on the `.org` site did not resolve as a public web page and is not
treated as verified source evidence.

## Local Handoff Baseline

Known website URLs from local docs:

- `https://quantumshieldlabs.dev/`
- `https://quantumshieldlabs.org/`

Known source candidates from local docs:

- No exact live website source repository, branch, build command, deployment
  path, preview path, or rollback path is documented as verified.
- Prior handoff material requires a future lane to re-check live website source,
  branch state, public evidence links, and public copy before editing.

Known claim-matrix and handoff posture:

- `WEBSITE_CLAIM_MATRIX.md` records high-risk public claims and safe
  replacements.
- `WEBSITE_UPDATE_PLAN.md` records website copy and information architecture
  backlog items, including a QSL protocol status band, product taxonomy labels,
  evidence links, metadata disclaimers, and visible open gates.
- NA-0295 created planning-only landing page and evidence-visual guidance. It
  did not implement the website and left source verification as the next gate.

Local handoff gaps:

- Exact source repo is still not verified.
- Exact source branch is still not verified.
- Build system for the live site is still not verified.
- Deployment target and deployment permissions are still not verified.
- Preview/staging flow is still not verified.
- Link/claim scan commands for the live website source are not verified.
- Rollback and stop conditions for the website deploy path are not verified.

## Website / Source Discovery Results

### `quantumshieldlabs.org`

- URL inspected: `https://quantumshieldlabs.org/`
- Title observed: `QuantumShield Labs | Public Post-Quantum Messaging Research`
- Officiality signal: the official `QuantumShieldLabs` GitHub organization
  profile lists this site as its website.
- Visible posture: public proof trail for research-stage post-quantum messaging,
  drafts, reference implementation evidence, demo criteria, release gates, and
  review boundaries.
- Visible top-level sections: home, progress/proof, demos, security posture,
  resources, and evidence navigation.
- QSL protocol mention: yes.
- Claim alignment: broadly aligned with QSL evidence-bound public posture.
- Stale/mismatch risk: some live open-gap wording still lists native package,
  KT-negative demo, and attachment demo proof as open even though later
  qsl-protocol evidence has advanced those areas. This needs a rewrite before
  any public implementation lane claims current evidence alignment.
- Source repo/branch/build/deploy: not publicly verified.
- Deployment clue: built static application assets are visible, but this is not
  enough to identify source or deployment ownership.

### `quantumshieldlabs.dev`

- URL inspected: `https://quantumshieldlabs.dev/`
- Title observed: `Quantum Shield Labs - Post-Quantum Security for the AI Age`
- Officiality signal: brand-adjacent public site with Quantum Shield Labs
  company/product copy and public GitHub links.
- Visible posture: broader consulting/product website with healthcare,
  autonomous agents, blockchain/security tooling, dashboards, calculators, and
  external product links.
- QSL protocol mention: not the primary public evidence surface in the inspected
  live page.
- Claim alignment: mixed. Company/product claims may be acceptable only when
  clearly separated from qsl-protocol release evidence.
- High-risk area: broad live-product wording, healthcare/PQC service copy, and
  external project claims can be mistaken for qsl-protocol release evidence
  unless future copy adds clear boundaries.
- Source repo/branch/build/deploy: not publicly verified.
- Deployment clue: response headers indicate a hosted static site surface, but
  this does not verify source repository, branch, or deployment authority.

### Official GitHub Organization

- Organization inspected: `https://github.com/QuantumShieldLabs`
- Public repositories observed: `.github`, `qsl-protocol`, `qsl-server`,
  `qsl-attachments`.
- Website source result: no public official organization repository inspected
  appears to be the live website source.
- qsl-protocol repo homepage: no verified website-source metadata.

### Public Source Candidate: `mbennett-labs/qsl`

- URL inspected: `https://github.com/mbennett-labs/qsl`
- Default branch: `main`
- Build clues: public Vite/React project files are present.
- Verification result: `NOT_VERIFIED_AS_LIVE_SOURCE`.
- Reason: inspected public source content does not match the current
  `quantumshieldlabs.org` or `quantumshieldlabs.dev` live surfaces.

## Live Claim-Boundary Audit

### `quantumshieldlabs.org`

| Claim area | Classification | Finding |
| --- | --- | --- |
| Research-stage public proof trail | SAFE | Aligned with current QSL evidence posture. |
| Public AGPL / reference implementation evidence | SAFE_WITH_EVIDENCE_LINK | Safe when linked to repository evidence and release maps. |
| Production hardening as separate roadmap work | SAFE | Preserves non-production boundary. |
| External review not complete / open review boundary | SAFE | Preserves external-review boundary. |
| Native package, KT-negative demo, attachment demo still open | NEEDS_REWRITE | Appears stale relative to later qsl-protocol evidence; should be refreshed without implying production readiness. |
| Metadata minimization / deployment-dependent metadata claims | SAFE_WITH_EVIDENCE_LINK | Safe only with links to metadata threat model and residual metadata boundaries. |
| Source/build/deploy path | UNVERIFIED | Source is not publicly verified. |

### `quantumshieldlabs.dev`

| Claim area | Classification | Finding |
| --- | --- | --- |
| Broad company/product security positioning | SAFE_WITH_EVIDENCE_LINK | Safe only as company/service copy and not as qsl-protocol proof. |
| Live projects or deployed tools | NEEDS_REWRITE | Must be scoped to external products/services and separated from qsl-protocol release evidence. |
| Healthcare PQC consulting/readiness | UNVERIFIED | Requires source-linked service evidence and must not imply qsl-protocol certification or deployment. |
| External products such as dashboards, calculators, or scanners | SAFE_WITH_EVIDENCE_LINK | Safe only with explicit external-product labels and non-qsl-protocol boundary notes. |
| Metadata/anonymity posture | NEEDS_REWRITE | Future copy must say metadata minimization only and must not imply anonymity or metadata elimination. |
| Source/build/deploy path | UNVERIFIED | Source is not publicly verified. |

No live claim audit evidence supports a production-readiness,
public-internet-readiness, external-review-complete, anonymity, metadata-free,
untraceable, quantum-proof, unbreakable, or guaranteed-secure website claim for
qsl-protocol.

## Website Source / Build / Deploy Readiness Classification

Classification: `PARTIAL_READY_SOURCE_UNVERIFIED`.

Reason:

- Official and official-looking public surfaces were identified.
- Claim-safe public posture is partly documented locally.
- The `.org` site is broadly aligned with QSL evidence posture but has stale
  public gap wording that needs refresh.
- The `.dev` site needs stricter separation between external product/service
  claims and qsl-protocol evidence.
- The exact source repo, branch, build command, deployment path, preview flow,
  rollback path, and pre-deploy scan commands are still unverified.

This is not `READY_FOR_CLAIM_SAFE_IMPLEMENTATION` because source and deployment
authority are not proven.

## Claim-Safe Implementation Preconditions

Before a future implementation lane may edit website copy or source, it must:

1. Identify the exact website surface to edit.
2. Verify the exact source repository and default/edit branch.
3. Verify build commands and package manager lockfile state.
4. Verify deployment target, preview/staging flow, and rollback procedure.
5. Confirm edit authority without admin bypass or branch-protection weakening.
6. Run source-level link checks for all evidence links.
7. Run source-level overclaim scans before any deploy.
8. Preserve visible NOT_READY boundaries for production service readiness,
   public internet readiness, external review completion, metadata phase-2,
   anonymity, metadata elimination, and untraceability.
9. Add a QSL protocol status/evidence section before adjacent external-product
   claims can be mistaken as qsl-protocol proof.
10. Refresh stale `.org` open-gap wording to match current qsl-protocol
    evidence while preserving non-production and external-review boundaries.
11. Separate `.dev` external product/service claims from qsl-protocol release
    evidence.
12. Define STOP conditions for source mismatch, broken evidence links,
    unsupported claims, failed preview scans, or deployment uncertainty.

## Evidence Link Requirements

Future website work must link to stable public evidence instead of relying on
unsupported marketing claims:

- `GOALS.md`
- `PROJECT_CHARTER.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/public/INDEX.md`
- `docs/public/RELEASE_READINESS_EVIDENCE_MAP.md`
- `docs/public/EXTERNAL_REVIEW_PACKAGE.md`
- `docs/demo/DEMO_ACCEPTANCE_CRITERIA.md`
- `docs/demo/KT_NEGATIVE_PUBLIC_DEMO_READINESS.md`
- `docs/demo/ATTACHMENT_PUBLIC_DEMO_READINESS.md`
- `docs/public/SUITE2_TRIPLE_RATCHET_CLAIM_BOUNDARY.md`
- `docs/privacy/DOC-G5-001_Metadata_Threat_Model_v1.0.0_DRAFT.md`
- `docs/privacy/DOC-G5-003_Envelope_Transport_Profile_v0.1.0_DRAFT.md`
- This audit and its paired testplan.

## Future Test / Scan Requirements

A future website implementation lane should include:

- Source repo and branch proof.
- Clean working tree proof for the website source repo.
- Build command proof.
- Preview or static output proof.
- Link-check proof for all public evidence links.
- Overclaim phrase scan proof.
- Claim matrix pass/fail table.
- Evidence-link existence proof.
- No-form-submission proof.
- No-production-deploy proof unless a later directive explicitly authorizes it.
- Rollback or no-deploy stop condition proof.

## Recommended NA-0297 Shape

Recommended successor:

`NA-0297 - Website Source Verification Follow-Up and Implementation Blocker Resolution`

Recommended objective:

Resolve the source/deploy verification blockers and stale live-claim deltas
before any website implementation lane is authorized.

NA-0297 should not be an implementation lane unless a later directive first
verifies the exact source repository, branch, build, preview, deployment, and
rollback path.

## What Was Not Changed

- No website or external website repository was changed.
- No live site was changed.
- No website deployment was performed.
- No DNS, hosting, settings, forms, comments, or public posts were changed.
- No qsl-protocol runtime, protocol, crypto, service, demo, qsc-desktop,
  qsl-server, or qsl-attachments implementation path was changed.
- No workflow, script, Cargo, dependency, branch-protection, or public-safety
  configuration was changed.

## No Website Mutation Proof

All website and external source discovery was read-only. The qsl-protocol patch
is limited to governance/readiness evidence, a testplan, a handoff reference,
decision/traceability entries, and the rolling operations journal.

## No Implementation Change Proof

This audit does not change executable implementation paths. It records readiness
state, claim boundaries, source gaps, and preconditions for a future lane.
