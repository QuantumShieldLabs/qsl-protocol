Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-14
Replaces:
Superseded-By:

# Public Attention and Visibility Strategy

Goals: G1, G2, G3, G4, G5

This document is a strategy and audit output for NA-0290A. It does not
change website copy, README copy, START_HERE copy, protocol behavior, service
behavior, runtime behavior, dependencies, workflows, or public-safety
configuration.

## Executive summary

QSL has a stronger public story than its current visibility surfaces make
obvious. The safest public position is also the most distinctive one:

> Post-quantum messaging needs evidence, not slogans. QSL is building that
> evidence in public.

The project should become more memorable by leading with testable evidence,
fail-closed behavior, explicit limitations, and reviewer-ready proof paths. The
strategy should make the work easier to discuss without softening gaps around
metadata phase-2, service hardening, external review, and deployment readiness.

## The public challenge

Post-quantum messaging attracts inflated claims. A public reader may expect
simple assurances, but QSL's strongest asset is the opposite: the repository
names what is proven, what is only a demo, what remains incomplete, and what
reviewers should inspect next.

The public challenge is to make that discipline visible in the first ten
seconds. Today, a motivated reviewer can find the evidence trail, but a casual
reader may not immediately understand why the project is interesting, what
problem it is taking on, or how to share it without accidentally overstating it.

## The project hook

Primary hook:

> Post-quantum messaging needs evidence, not slogans. QSL is building that
> evidence in public.

Short variants:

- "A public evidence trail for post-quantum messaging."
- "Fail closed first, publish the proof, name the gaps."
- "A research-stage secure messaging stack that treats limitations as part of
  the evidence."

Use the primary hook when introducing QSL to broad technical audiences. Use the
short variants as supporting lines, not replacements for the evidence boundary.

## Core narrative

One-sentence hook:

QSL is a research-stage post-quantum messaging effort that publishes its
security evidence, demo limits, and remaining gaps in the open instead of
asking the public to trust slogans.

One-paragraph public explanation:

Most post-quantum security messaging starts with algorithm labels. QSL starts
with behavior that can be checked: negotiation should fail closed, downgrade
attempts should be rejected, demo flows should be reproducible, and gaps such
as metadata phase-2, external review, and service hardening should remain
visible. The public work is not a deployment approval; it is an evidence path
that lets reviewers, contributors, and supporters see what has been proven and
what still needs work.

Technical but accessible explanation:

QSL combines protocol specifications, conformance vectors, model checks, demo
acceptance criteria, and governance traceability so reviewers can inspect both
the intended security behavior and the current proof. Its public posture should
emphasize deterministic fail-closed behavior, downgrade and replay boundaries,
opaque attachment and relay service separation, explicit metadata limitations,
and reproducible non-production demos.

Reviewer explanation:

QSL is useful to review because evidence is organized around claims. Reviewers
can start from the [release readiness evidence map](RELEASE_READINESS_EVIDENCE_MAP.md),
the [external review package](EXTERNAL_REVIEW_PACKAGE.md), the
[demo acceptance criteria](../demo/DEMO_ACCEPTANCE_CRITERIA.md), and the
governance traceability records. A review should not treat the package as a
completed external review; it should use the package to identify findings,
dispositions, required vectors, and remaining proof gaps.

Contributor/supporter explanation:

QSL needs contributors and supporters who value testable security posture. The
best first contributions are evidence navigation, negative tests, reproducible
demo proof, documentation that clarifies limitations, and reviewer feedback
that turns broad claims into precise acceptance criteria.

## Message pillars

| Pillar | Public framing | Required boundary |
| --- | --- | --- |
| Evidence over slogans | Post-quantum messaging needs evidence, not slogans. | Never imply the evidence is complete. |
| Fail-closed behavior | Fail-closed behavior is a testable promise. | Tie the statement to specific tests, vectors, or model checks. |
| Published limitations | QSL publishes limitations instead of hiding them. | Keep metadata phase-2, service hardening, and external review gaps visible. |
| Non-production demo | The demo is real evidence, not a release approval. | Keep demo wording explicitly non-production. |
| Service hardening path | Services are hardened before stronger deployment claims. | Do not present server or attachment work as deployment approval. |
| Metadata honesty | Metadata and external-review gaps are named. | Do not imply anonymity, metadata elimination, or completed review. |
| Reviewer-first governance | Decisions and traceability are part of the public proof. | Do not let governance records substitute for executable proof. |

## Audience map

| Audience | What they need first | Safe engagement path |
| --- | --- | --- |
| Security reviewers | Evidence map, claim boundaries, negative-test inventory. | Invite findings against specific claims and gaps. |
| Cryptographers | Protocol docs, vectors, model checks, terminology boundaries. | Ask for review of assumptions and missing proofs. |
| Open-source developers | Build path, focused issues, tests, docs map. | Offer small evidence-navigation and harness tasks. |
| Privacy/security community | Honest metadata posture and fail-closed demos. | Show what is visible, what is minimized, and what is not complete. |
| Journalists/technical writers | Clear problem statement and safe quote bank. | Provide non-hype copy plus explicit "do not claim" notes. |
| Funders/supporters | Why the work matters and what support unlocks. | Map funding to evidence milestones, not vague security promises. |
| Potential contributors | Concrete next tasks and contribution boundaries. | Point to tests, docs navigation, demo proof, and reviewer feedback loops. |

## Attention hooks

Each hook is proposed copy for later implementation. None is implemented by
NA-0290A.

| Hook | Safe wording | Audience | Overclaim risk | Evidence dependency | Timing |
| --- | --- | --- | --- | --- | --- |
| Evidence not slogans | "Post-quantum messaging needs evidence, not slogans." | broad technical public | Sounds complete if isolated | Evidence map and review package links | Phase 1 |
| Fail-closed receipt | "Show me the reject path." | security reviewers | May imply all rejects are covered | Negative tests and model checks | Phase 2 |
| Public limitations | "The gaps are part of the release notes." | privacy/security community | Could sound defensive without proof | Release readiness map | Phase 1 |
| Demo with boundaries | "Try the demo, then read what it does not prove." | developers | Demo may be mistaken for deployment proof | Demo acceptance criteria | Phase 2 |
| Reviewer path | "Start with the claim, follow the evidence." | cryptographers/reviewers | Could imply review is complete | External review package | Phase 1 |
| Metadata honesty | "Metadata work is named, not waved away." | privacy community | Could be misread as anonymity | Metadata phase-2 plan and future harness | Phase 3 after NA-0290 work |
| Service boundary | "Opaque relay, explicit service gaps." | infra/security operators | Could imply deployment approval | Server/attachment production-boundary map | Phase 2 |
| Red-team checklist | "Break the claim, not the slogan." | testers/researchers | Can invite broad claims without scope | Claim-boundary checklist | Phase 4 |
| Proof timeline | "What is proven, what is planned, what is blocked." | supporters | Timeline can drift | Date-stamped evidence map | Phase 2 |
| Visual evidence trail | "One diagram from public claim to test." | journalists/developers | Diagram can oversimplify | Traceability rows and testplan links | Phase 2 |
| Limitation-first launch | "A launch page that leads with what is not proven." | technical writers | May sound negative without narrative | Public posture docs | Phase 3 |
| Contributor ladder | "Small contributions that strengthen public evidence." | contributors | Could attract broad feature requests | Issues and contribution guide | Phase 1 |
| Demo transcript | "A 60-second fail-closed walkthrough." | social/video audience | Could overfit to one path | Demo smoke and reject evidence | Phase 4 |
| External review invite | "Review the package; do not assume approval." | reviewers | May be mistaken for completed review | Review package and findings process | Phase 5 |

## Website strategy

Hero proposal:

- Headline: "Post-quantum messaging needs evidence, not slogans."
- Supporting line: "QSL is a research-stage public evidence trail for
  fail-closed secure messaging behavior, reproducible demos, and named gaps."
- Primary action: "Read the evidence map."
- Secondary action: "Try the non-production demo."
- Boundary line near the actions: "Not a deployment approval. External review
  and metadata phase-2 remain open."

Why this matters section:

- Explain that algorithm labels alone do not prove negotiation, replay,
  downgrade, metadata, or service behavior.
- Emphasize harvest-now-decrypt-later risk as motivation only when paired with
  source-backed wording in a future website lane.
- Tie urgency to evidence-building, not fear.

What is proven section:

- Use a short list of current evidence categories: fail-closed negotiation and
  downgrade paths, model checks, conformance vectors, demo acceptance, service
  boundary planning, and governance traceability.
- Link each item to a specific evidence document.

What is not proven yet section:

- Keep metadata phase-2, external review completion, service deployment gates,
  and broader release gates visible.
- Label this section as a strength: the project is showing the remaining work.

Try/demo/evidence section:

- Separate "try the demo" from "read the evidence."
- Keep demo copy non-production and reproducibility-focused.
- Include negative-path highlights, not only successful send/receive.

Reviewer section:

- Provide a route from claim to evidence to expected reviewer output.
- Ask reviewers to file findings, missing vectors, ambiguity reports, and
  claim-boundary issues.

Contributor/supporter section:

- Offer evidence-navigation tasks, negative-test tasks, documentation clarity
  tasks, and reproducibility tasks before feature requests.
- Tie support requests to concrete evidence gates.

Visual and diagram ideas:

- Claim-to-evidence map: public claim -> source doc -> test/vector/model ->
  current status.
- Fail-closed flow: input -> validation -> reject -> no mutation.
- "What is proven / what is not proven" split view.
- Demo receipt: command, checked marker, evidence link, known boundary.
- Service boundary diagram: qsl-protocol, qsl-server, qsl-attachments, and
  external website as separate surfaces.

Credibility guardrails:

- Every high-level statement must link to a current evidence source.
- Time-sensitive website copy must carry a last-verified date.
- Company/product claims must not be used as QSL protocol evidence.
- Website copy must not suggest external review completion, deployment
  approval, metadata elimination, or anonymous messaging.

## Repository strategy

README improvements to consider later:

- Move the primary hook near the first viewport while preserving current
  research-stage wording.
- Add a compact "what to share publicly" block.
- Add a "claim -> evidence" table for the top five public claims.
- Keep the current "what this is not" posture visible.

START_HERE improvements to consider later:

- Add a reviewer path, contributor path, and public-demo path.
- Keep the governance path for maintainers.
- Avoid turning START_HERE into a marketing page.

Evidence navigation improvements:

- Add a public evidence map landing section that groups proof by reader need:
  reviewer, demo user, contributor, supporter.
- Add "known gap" links next to each proof category.

Issue and feedback pathways:

- Add issue labels or templates later for claim-boundary feedback, evidence
  navigation, demo reproducibility, and reviewer findings.
- Keep security vulnerability reporting routed through the existing security
  process.

Contributor on-ramp:

- Publish small, bounded tasks that improve proof quality: link validation,
  gap wording, negative-test inventory, demo transcript capture, and evidence
  receipts.

Suggested repo topics, description, and social preview:

- Repository description candidate: "Research-stage post-quantum messaging
  protocol and demo with public evidence, fail-closed tests, and explicit
  limitations."
- Topics to consider later: `post-quantum`, `secure-messaging`,
  `fail-closed`, `protocol-evidence`, `cryptography-research`,
  `security-testing`.
- Social preview idea: a clean "evidence map" graphic, not a shield or lock
  that implies completed security.

Branch cleanup note:

- Prior read-only audit found stale non-main branches tied to closed or
  preserved PRs. Cleanup remains recommendation-only and must not happen
  without an explicit branch-governance directive.

## Demo/evidence storytelling strategy

60-second demo concept:

1. Start with the hook.
2. Run or show the non-production demo command.
3. Show one success marker.
4. Show one fail-closed reject marker.
5. End on the evidence map and the visible gap list.

Animated fail-closed flow:

- Present validation as the hero: invalid input stops before state mutation.
- Show "reject" as success of the safety property, not as a failure of the demo.

Evidence receipt graphic:

- Include command, commit or PR, status check, test marker, source document, and
  known boundary.

What is proven / not proven visual:

- Use two columns with equal visual weight.
- Avoid making gaps look like footnotes.

Red-team checklist:

- "Can I reproduce the proof?"
- "Does the reject path mutate state?"
- "Does the public claim link to evidence?"
- "Is the limitation explicit?"
- "Is this demo evidence being mistaken for deployment evidence?"

Blog/social post ideas:

- "Why fail-closed behavior is a public claim."
- "The difference between a post-quantum algorithm label and protocol
  evidence."
- "What a non-production demo can prove."
- "How to read QSL's evidence map."
- "The gaps we are not hiding."
- "How to review QSL without trusting the README."

Talk/lightning talk outline:

1. The problem: post-quantum messaging claims are easy to overstate.
2. The QSL approach: public evidence tied to claims.
3. One fail-closed example.
4. One limitation example.
5. How reviewers and contributors can help.

## Public content ideas

- A one-page public evidence map with date-stamped status.
- A reviewer quickstart that starts from claims, not repository structure.
- A demo transcript that includes one positive path and one reject path.
- A "known limitations" page written for non-specialists.
- A claim-boundary copy bank for maintainers and writers.
- A short comparison article: algorithm label vs protocol evidence.
- A visual explainer of relay and attachment boundaries.
- A contributor issue set for evidence navigation and link hygiene.

## Contributor/supporter call-to-action strategy

Primary contributor call:

"Help turn security claims into reproducible evidence."

Safe contribution categories:

- Reproduce a demo and report exact host details.
- Add or review negative-path evidence.
- Improve evidence navigation.
- Review public claim boundaries.
- Translate technical proof into accurate public explanation.
- Identify ambiguity in docs or traceability.

Supporter framing:

- Support buys time for tests, review, reproducible artifacts, and evidence
  navigation.
- Support does not buy stronger claims before evidence exists.

## Media/reviewer strategy

For media and technical writers:

- Provide a short approved summary, a "do not say" list, and links to evidence.
- Keep screenshots focused on evidence maps, demo receipts, and limitations.
- Avoid lock imagery or words that imply completed security.

For reviewers:

- Publish what output is useful: findings, missing vectors, ambiguity reports,
  proof gaps, and suggested acceptance criteria.
- Keep review status separate from review invitation.
- Track dispositions in governance before changing public posture.

## Visual/diagram ideas

- Evidence ladder: claim, source, test, status, gap.
- Fail-closed no-mutation path: invalid input, reject, state unchanged.
- Public surface map: website, qsl-protocol, qsl-server, qsl-attachments.
- Demo receipt: command, marker, CI, link, limitation.
- Service boundary map: protocol semantics stay in qsl-protocol; qsl-server
  remains transport-only; qsl-attachments remains opaque ciphertext-only.
- "Gaps we are naming" panel: metadata phase-2, external review, service
  deployment gates, broader release gates.

## Safe language and prohibited language

Approved language:

- "research-stage"
- "non-production demo"
- "public evidence trail"
- "fail-closed behavior with linked tests"
- "limitations are documented"
- "external review package prepared for review"
- "metadata phase-2 remains open"
- "service hardening remains evidence-gated"
- "not a deployment approval"

Prohibited language unless explicitly quoted as prohibited, negated, or marked
as future/unproven:

- "production-ready"
- "deployment-ready"
- "production relay ready"
- "production attachment ready"
- "public internet ready"
- "external review complete"
- "externally reviewed"
- "metadata-free"
- "anonymity"
- "anonymous messaging"
- "untraceable"
- "quantum-proof"
- "unbreakable"
- "military-grade"
- "guaranteed secure"
- "proven true Triple Ratchet"
- "ready for production"
- "release ready"

Safe substitutions:

| Unsafe or ambiguous wording | Safer substitution |
| --- | --- |
| "production-ready protocol" | "research-stage protocol with public evidence and open gates" |
| "externally reviewed" | "prepared for external review; completion evidence is not present yet" |
| "metadata-free messaging" | "metadata phase-2 work remains open and explicitly tracked" |
| "anonymous messaging" | "privacy-relevant metadata boundaries are documented and still incomplete" |
| "quantum-proof security" | "post-quantum-first research-stage design with evidence-linked claims" |
| "unbreakable encryption" | "bounded claims tied to tests, vectors, and reviewable assumptions" |

Examples of what not to say:

- "QSL is production-ready."
- "QSL has completed external review."
- "QSL provides metadata-free anonymous messaging."
- "QSL is quantum-proof and guaranteed secure."

Examples of stronger safe public copy:

- "QSL is not asking for trust in slogans. It publishes its current proof and
  its open gaps."
- "The demo is useful because it shows both success and reject behavior, while
  staying explicitly non-production."
- "External review is invited through a prepared package; completion still
  requires reviewer findings, dispositions, and recorded acceptance."
- "Metadata phase-2 is a visible work item, not a hidden footnote."

## Implementation roadmap

Phase 1: README / START_HERE / repo polish

- Add the primary hook and a compact safe-share block.
- Add claim-to-evidence navigation.
- Add issue/feedback routing for claim-boundary and evidence-navigation
  feedback.
- Evidence gate: current release readiness map, external review package, and
  claim-boundary scan must pass.

Phase 2: public docs navigation and evidence visuals

- Build a public evidence navigation page and simple evidence diagrams.
- Add demo receipt templates.
- Evidence gate: link-check, leak-scan, and no-overclaim scan must pass.

Phase 3: website / landing page handoff

- Update the external website handoff with current sources and a page-by-page
  implementation plan.
- Keep company/product pages separated from QSL protocol proof.
- Evidence gate: live website source must be re-audited before edits.

Phase 4: demo media kit

- Produce 60-second demo transcript, fail-closed animation, and evidence
  receipt graphic.
- Evidence gate: demo smoke and negative-path proof must be current.

Phase 5: outreach after external review package alignment

- Invite reviewer and contributor attention around specific claims and gaps.
- Evidence gate: review package, findings workflow, and response process must
  be current; do not imply review completion.

## Evidence gates before implementation

Before any public copy implementation:

- Confirm the target surface and allowed paths in a separate directive.
- Run a claim-boundary scan for prohibited phrases.
- Link every high-level claim to current evidence.
- Keep metadata phase-2 and external review status visible.
- Keep website/company product claims separated from QSL protocol proof.
- Run link-check and leak-scan.
- Keep public-safety required and green.
- Do not change protocol, crypto, runtime, service, dependency, workflow, or
  branch-protection semantics in a public-copy lane.

## What not to claim

Do not claim:

- production readiness, deployment approval, or public internet service
  readiness;
- completed external review;
- metadata elimination, anonymity, or untraceability;
- perfect security, guaranteed security, or quantum-proof outcomes;
- service hardening or backup/restore readiness beyond the evidence map;
- that company products or websites prove QSL protocol release status;
- that a demo proves more than the demo acceptance criteria say it proves.

## Immediate next recommended implementation NA

After NA-0290A closeout restores NA-0290, the queue should proceed with NA-0290
unless the Director explicitly inserts another public-visibility lane. The next
best public-visibility implementation lane, when authorized, is:

`NA-PUBLIC-001 - Public Evidence Navigation and README/START_HERE Hook Refresh`

Recommended scope for that later lane:

- Update README and START_HERE only after a separate directive.
- Add a small public evidence navigation section or page.
- Add a safe-share copy bank and claim-to-evidence table.
- Keep the external website as a later handoff item.
- Preserve all claim boundaries in this strategy.
