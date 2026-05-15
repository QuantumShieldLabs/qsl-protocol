Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-14
Replaces:
Superseded-By:

# NA-0290A Public Attention and Visibility Audit

Goals: G1, G2, G3, G4, G5

This audit supports [PUBLIC_ATTENTION_AND_VISIBILITY_STRATEGY.md](../../public/PUBLIC_ATTENTION_AND_VISIBILITY_STRATEGY.md).
It is read-only with respect to public surfaces. It does not implement website,
README, START_HERE, protocol, service, runtime, dependency, workflow, or
branch-protection changes.

## Sources inspected

Local qsl-protocol sources inspected:

- [README.md](../../../README.md)
- [START_HERE.md](../../../START_HERE.md)
- [SUPPORT.md](../../../SUPPORT.md)
- [CONTRIBUTING.md](../../../CONTRIBUTING.md)
- [NEXT_ACTIONS.md](../../../NEXT_ACTIONS.md)
- [DECISIONS.md](../../../DECISIONS.md)
- [TRACEABILITY.md](../../../TRACEABILITY.md)
- [docs/public/INDEX.md](../../public/INDEX.md)
- [docs/public/EXTERNAL_REVIEW_PACKAGE.md](../../public/EXTERNAL_REVIEW_PACKAGE.md)
- [docs/public/RELEASE_READINESS_EVIDENCE_MAP.md](../../public/RELEASE_READINESS_EVIDENCE_MAP.md)
- [docs/public/WEBSITE_CLAIM_MATRIX.md](../../public/WEBSITE_CLAIM_MATRIX.md)
- [docs/public/WEBSITE_UPDATE_PLAN.md](../../public/WEBSITE_UPDATE_PLAN.md)
- [docs/public/WEBSITE_IMPLEMENTATION_HANDOFF.md](../../public/WEBSITE_IMPLEMENTATION_HANDOFF.md)
- [docs/public/EXTERNAL_WEBSITE_IMPLEMENTATION_DIRECTIVE.md](../../public/EXTERNAL_WEBSITE_IMPLEMENTATION_DIRECTIVE.md)
- [docs/demo/DEMO_ACCEPTANCE_CRITERIA.md](../../demo/DEMO_ACCEPTANCE_CRITERIA.md)
- targeted `rg` review of `docs/design/**` and `docs/governance/evidence/**`
  for public posture, website, claim-boundary, demo, service, and metadata
  evidence.

Public web and repository sources inspected read-only:

- QuantumShieldLabs GitHub organization: <https://github.com/QuantumShieldLabs>
- qsl-protocol public GitHub page: <https://github.com/QuantumShieldLabs/qsl-protocol>
- qsl-server public GitHub page: <https://github.com/QuantumShieldLabs/qsl-server>
- qsl-attachments public GitHub page: <https://github.com/QuantumShieldLabs/qsl-attachments>
- qsl-server public README, START_HERE, and docs under `docs/server/**` via
  GitHub API read-only.
- qsl-attachments public README, START_HERE, and docs under `docs/**` via
  GitHub API read-only.
- Quantum Shield Labs website discovered in existing docs:
  <https://quantumshieldlabs.dev/>
- Quantum Shield Labs public website result inspected by web module:
  <https://quantumshieldlabs.org/>

Website access notes:

- `quantumshieldlabs.dev` rendered public content to the web module.
- `quantumshieldlabs.org` exposed a public page title to the web module, but no
  body text was available in the extracted page view during this audit.
- A host-local unauthenticated fetch to `quantumshieldlabs.org` returned HTTP
  403, so this audit records the website as live-looking but body-content
  inaccessible from the local host. No forms, logins, dashboards, or private
  material were accessed.

## Website audit result

The prior website audit and handoff docs already identify the most important
brand-confusion risk: external company products and live service claims can sit
near QSL protocol references, which can make protocol readers infer stronger
release status than the qsl-protocol evidence supports.

Current `quantumshieldlabs.dev` public content remains broader than the QSL
protocol repository. It emphasizes company services, AI agent security,
consulting, and live external tools. That can be useful for attention, but it
must not be reused as QSL protocol proof. Any future website refresh should
separate company/product traction from QSL protocol evidence and add a visible
route to the qsl-protocol evidence map.

The `quantumshieldlabs.org` page was discoverable and appeared QSL-specific by
title, but body content was not available to this audit. A future website lane
must re-audit the live site and verified source repository before changing
copy.

## qsl-protocol audit

First 10-second impression:

- Strength: the public GitHub page and README already state that QSL is
  research-stage, public for review, and not a deployment approval.
- Gap: the most memorable line is present but not yet packaged as the primary
  shareable hook across all public surfaces.
- Gap: a casual reader can miss the emotional reason to care before entering a
  dense evidence map.

Public problem statement:

- Strength: README explains that post-quantum messaging needs more than
  algorithm labels and must prove negotiation, replay, metadata, demo, and
  service boundaries.
- Gap: the public problem could be made more repeatable in one sentence.

Why now / urgency:

- Strength: QSL avoids unsupported countdown-style urgency.
- Gap: urgency is mostly implicit. Future copy can say the migration window
  makes evidence-building urgent, while leaving external timeline claims to
  separately sourced website work.

Memorable project identity:

- Strength: "evidence in public" is distinctive and safe.
- Gap: QSL does not yet consistently package itself as the public evidence
  trail for post-quantum messaging.

What makes QSL different:

- Strength: traceability, decisions, model checks, vectors, demo criteria, and
  limitation tracking are unusually visible.
- Gap: this differentiation is buried across several documents.

Evidence story:

- Strength: release map, review package, demo criteria, traceability, and
  decisions give a credible reviewer path.
- Gap: there is no simple "claim -> evidence -> gap" public landing view yet.

Demo story:

- Strength: demo docs explicitly preserve non-production boundaries and include
  negative-path proof.
- Gap: the demo story needs a short public transcript or evidence receipt that
  can be shared without implying deployment approval.

Contributor/supporter pathway:

- Strength: CONTRIBUTING and SUPPORT exist.
- Gap: they are process-oriented and do not yet give public-visibility
  contributors a clear first task list.

Reviewer pathway:

- Strength: the external review package and release readiness map are strong
  starting points.
- Gap: public copy should make reviewer output concrete: findings, missing
  vectors, ambiguity reports, and claim-boundary issues.

Media/social shareability:

- Strength: the project has a safe public hook.
- Gap: it lacks shareable visuals, a safe quote bank, a short demo transcript,
  and a "what not to say" public writer guide.

Repo landing-page clarity:

- Strength: the README is honest and careful.
- Gap: carefulness reads as dense technical posture before it reads as a
  memorable public narrative.

Public screenshots/visuals status:

- Strength: desktop and demo evidence exists in prior lanes.
- Gap: there is no public screenshot/diagram set designed for explaining the
  evidence story.

Public claim safety:

- Strength: current qsl-protocol public docs consistently preserve
  non-production and limitation wording.
- Gap: public attention work needs reusable safe-copy patterns so future
  writers do not improvise stronger claims.

Attention without overclaiming:

- Strength: the safest attention angle is the strongest one: evidence,
  fail-closed behavior, and visible limitations.
- Gap: future implementation must keep hooks separated from claims and require
  evidence links for every high-level statement.

## qsl-server audit

The qsl-server public page and README present it as a transport-only relay for
QSL demos. The service boundary is clear: opaque payload forwarding/storage,
no protocol parsing, no crypto, deterministic errors, bounded limits, optional
relay auth, and no secret or payload logging.

Docs under `docs/server/**` strengthen the operational story: deployment
hardening, systemd hardening, relay inbox semantics, auth/hardening, and
route-token shape review all reinforce fail-closed and operator-safety
boundaries.

Visibility strengths:

- The technical invariants are clear and concrete.
- The repo is useful evidence that service boundaries are being narrowed
  before stronger deployment claims.

Attention gaps:

- The repo is mostly operator-facing. It does not connect the relay boundary to
  the public QSL narrative in a concise way.
- Public readers may not understand why a boring transport-only relay is a
  trust-building feature.

Safe opportunity:

- Later public copy can describe qsl-server as an example of "narrow service
  boundaries before stronger claims" while avoiding deployment approval.

## qsl-attachments audit

The qsl-attachments public page and README present it as the runtime home for
the opaque encrypted attachment plane. It names qsl-protocol as canonical for
attachment semantics and states the current single-node local-disk runtime
posture. It also records no plaintext attachment handling on service surfaces,
no capability-like secrets in canonical URLs, and explicit limitations around
deployment automation, multi-node storage, authn/authz scope, and durability.

Docs under `docs/**` add operational hardening, reference deployment,
authn/authz policy-subject, and durability/recovery boundaries. The current
docs are unusually honest about local-disk topology, backup/restore limits,
resource-scoped capabilities, and fail-closed recovery.

Visibility strengths:

- The attachment service has concrete evidence and clear boundaries.
- The docs make limitations visible instead of smoothing them away.

Attention gaps:

- The public story is hard to summarize quickly.
- The strongest message, "opaque encrypted attachments with named durability
  and policy-subject limits," needs translation for non-specialist readers.

Safe opportunity:

- Later public copy can use qsl-attachments to demonstrate the discipline of
  naming service boundaries before claiming broader readiness.

## Current public attention gaps

- The first-viewport narrative is safer than most security projects but not yet
  sharp enough to be easily repeated.
- Existing proof is spread across many docs and can feel like a maze.
- The project lacks a compact claim-to-evidence visual.
- The demo lacks a concise public transcript that includes both success and
  reject behavior.
- There is no public safe-copy bank for maintainers, supporters, or writers.
- Contributor on-ramps are not yet framed as evidence-strengthening work.
- Website/company surfaces can create brand conflation with QSL protocol proof
  unless separated carefully.

## Current strengths

- Research-stage and non-production boundaries are already explicit.
- The project uses decisions, traceability, testplans, and evidence docs as
  first-class public proof.
- The external review package and release readiness map are strong reviewer
  starting points.
- Fail-closed and no-mutation behavior are recurring proof themes.
- Service boundaries are being narrowed before stronger deployment claims.
- Limitations around metadata phase-2, external review, and service readiness
  are visible.

## Risks of over-hype

- Turning "post-quantum-first" into a perfect-security claim.
- Letting demo success imply deployment approval.
- Letting a prepared review package imply completed external review.
- Letting metadata minimization wording imply anonymity, metadata elimination,
  or untraceability.
- Letting company website product traction imply QSL protocol release status.
- Using service-hardening evidence as public internet deployment approval.
- Using lock/shield imagery or slogans that imply more proof than exists.

## Safe opportunities

- Lead with evidence discipline instead of assurance language.
- Make visible limitations part of the public identity.
- Treat fail-closed rejects as public proof moments.
- Create claim-to-evidence navigation for reviewers and writers.
- Use service boundary diagrams to explain why less service behavior can be a
  security strength.
- Invite contributors to strengthen proof, not add broad features first.
- Invite reviewers to attack claims through the evidence map.

## Recommended implementation sequence

1. Proceed with NA-0290 after NA-0290A closeout unless the Director explicitly
   inserts another public lane.
2. Later public-visibility implementation should start with repository
   navigation and safe-copy polish because it has the lowest operational risk.
3. Add public evidence visuals after the claim-to-evidence structure is stable.
4. Refresh external website handoff only after re-auditing live website content
   and verifying the deployment repository.
5. Build demo media only from current demo proof and explicit non-production
   boundaries.
6. Broader outreach should wait until the external review package and metadata
   phase-2 roadmap are aligned with the current queue.

## No-implementation-change proof

This NA-0290A strategy packet changes only:

- `docs/public/PUBLIC_ATTENTION_AND_VISIBILITY_STRATEGY.md`
- `docs/governance/evidence/NA-0290A_public_attention_visibility_audit.md`
- `tests/NA-0290A_public_attention_visibility_strategy_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

This packet does not change:

- `README.md`
- `START_HERE.md`
- website or external website files
- qsl-protocol runtime, protocol, crypto, demo, or service code
- qsl-server implementation files
- qsl-attachments implementation files
- qsc-desktop files
- `.github/**`
- `scripts/**`
- `Cargo.toml` or `Cargo.lock`
- branch-protection settings
- public-safety configuration
- dependencies
- branch state
