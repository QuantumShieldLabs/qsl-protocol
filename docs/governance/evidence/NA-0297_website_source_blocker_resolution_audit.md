Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-16
Replaces: n/a
Superseded-By: n/a

# NA-0297 Website Source Blocker Resolution Audit

## Executive Summary

NA-0297 expanded the read-only website source, deployment, claim-scan, and
link-scan verification started by NA-0296. It did not mutate any website,
external website repository, DNS, hosting, deployment setting, public post,
form, source branch, protocol implementation, runtime implementation, service
implementation, workflow, script, Cargo file, dependency, branch protection, or
public-safety configuration.

Result: `OPERATOR_ACTION_REQUIRED`.

The official organization-linked `quantumshieldlabs.org` surface has strong
historical and live clues for a private `Tebbens4832/QuantumShield` source and
Cloudflare Pages deployment path. However, the current directive is read-only
and forbids private/login-gated source inspection, so the private source,
current edit authority, current preview path, and rollback procedure are not
independently verifiable here.

The `quantumshieldlabs.dev` surface remains a broader company/product static
site served from a Hostinger/LiteSpeed surface. Its exact source repository,
branch, build, preview, deployment, and rollback path remain unverified. The
public `mbennett-labs/qsl` repository is still only a source candidate: it has
Vite/React project files, but it does not verify as the current live source for
either official website surface.

Claim/link scan readiness is `PARTIAL_READY_NEEDS_SOURCE`. Live public text and
qsl-protocol evidence links can be inventoried now, but source-level scans,
build proof, preview proof, and rollback proof require an operator-provided
source/deploy/authority bundle.

## Scope and Non-Goals

In scope:

- read-only qsl-protocol handoff and blocker inventory;
- read-only official website and public repository discovery;
- read-only public HTTP header, robots, sitemap, and asset-path inspection;
- source, branch, build, preview, deploy, rollback, claim-scan, and link-scan
  classification;
- operator-action plan for a future NA-0298 lane;
- qsl-protocol governance/readiness audit, testplan, handoff reference,
  decision, traceability, and journal updates.

Out of scope:

- website or external website repository mutation;
- website PR creation, merge, deployment, preview mutation, DNS, hosting, or
  settings changes;
- private/login-gated source inspection;
- public posting, social action, form submission, image/video generation, or
  publishing;
- qsl-protocol runtime, protocol, crypto, state-machine, demo, service,
  qsc-desktop, qsl-server, or qsl-attachments implementation changes;
- `.github/**`, `scripts/**`, Cargo manifests, lockfiles, dependency updates,
  branch-protection changes, or public-safety configuration changes;
- production-readiness, public-internet-readiness, external-review-complete,
  anonymity, metadata-free, untraceable, quantum-proof, unbreakable,
  guaranteed-secure, website-updated, or implementation-complete claims.

## Sources Inspected

Local qsl-protocol sources:

- `NEXT_ACTIONS.md`
- `docs/governance/evidence/NA-0296_website_source_verification_readiness_audit.md`
- `tests/NA-0296_website_source_verification_readiness_testplan.md`
- `docs/governance/evidence/NA-0295_website_landing_evidence_visuals_plan.md`
- `docs/public/WEBSITE_IMPLEMENTATION_HANDOFF.md`
- `docs/public/WEBSITE_CLAIM_MATRIX.md`
- `docs/public/WEBSITE_UPDATE_PLAN.md`
- `docs/public/EXTERNAL_WEBSITE_IMPLEMENTATION_DIRECTIVE.md`
- `docs/public/PUBLIC_ATTENTION_AND_VISIBILITY_STRATEGY.md`
- `docs/public/INDEX.md`
- `README.md`
- `START_HERE.md`
- `TRACEABILITY.md`
- `DECISIONS.md`
- targeted `docs/governance/evidence/**`, `docs/demo/**`, and
  `docs/design/**` searches.

Read-only public and archived evidence sources:

- `https://github.com/QuantumShieldLabs`
- `https://github.com/QuantumShieldLabs/qsl-protocol`
- `https://github.com/QuantumShieldLabs/qsl-server`
- `https://github.com/QuantumShieldLabs/qsl-attachments`
- `https://quantumshieldlabs.org/`
- `https://quantumshieldlabs.org/robots.txt`
- `https://quantumshieldlabs.org/sitemap.xml`
- `https://quantumshieldlabs.dev/`
- `https://quantumshieldlabs.dev/robots.txt`
- `https://quantumshieldlabs.dev/sitemap.xml`
- `https://github.com/mbennett-labs/qsl`
- local response archive references for NA-0255 D049 and D050, which recorded
  historical external website PR #19 evidence for `Tebbens4832/QuantumShield`.

The private `Tebbens4832/QuantumShield` repository was not inspected for source
content in this lane. Its private status is treated as an operator-action
blocker, not as implementation authorization.

## Local Blocker Baseline

Live NA-0297 queue scope:

- status: `READY`;
- objective: resolve source, branch, build, preview, deployment, rollback,
  link-scan, and claim-scan blockers before any website implementation lane is
  authorized;
- allowed: docs-only blocker-resolution evidence;
- forbidden: website/external repo edits and implementation claims;
- acceptance: exactly one READY item remains, no website implementation occurs,
  and required CI/public-safety stay green.

Inherited NA-0296 blockers:

- exact website source repo not verified;
- exact source branch not verified;
- build command and package manager state not verified;
- deployment target and deployment authority not verified;
- preview/staging flow not verified;
- rollback path not verified;
- source-level link scan not verified;
- source-level overclaim/claim scan not verified;
- `.org` public copy had stale open-gap wording relative to later qsl-protocol
  evidence;
- `.dev` public copy needs stronger separation between company/product claims
  and qsl-protocol release evidence.

Local source candidates:

- `Tebbens4832/QuantumShield`: historical NA-0255 evidence and live `.org`
  structured data point at this repository, but it is private and cannot be
  independently source-inspected under this directive.
- `mbennett-labs/qsl`: public Vite/React candidate with `project/package.json`,
  `project/vite.config.ts`, and TypeScript React source, but it does not verify
  as the current live source for `.org` or `.dev`.
- `QuantumShieldLabs/*`: official public org repositories are `.github`,
  `qsl-protocol`, `qsl-server`, and `qsl-attachments`; none is a public website
  source repository.

Implementation can not be authorized by this directive.

## Website / Source Discovery Expansion

### QuantumShieldLabs GitHub Organization

- URL: `https://github.com/QuantumShieldLabs`
- Official website link: `https://www.quantumshieldlabs.org/`
- Public repositories observed: `.github`, `qsl-protocol`, `qsl-server`,
  `qsl-attachments`.
- Source repo evidence: no public official organization repository appears to
  be the current website source.
- Branch/build/deploy evidence: not present in public org metadata.
- Classification: `SOURCE_NOT_FOUND` for public official organization source.

### qsl-protocol Repository

- URL: `https://github.com/QuantumShieldLabs/qsl-protocol`
- Homepage metadata: empty.
- Role: public specifications, conformance vectors, and research-stage
  reference implementations.
- Source repo evidence: not a website source.
- Claim/link scan feasibility: strong qsl-protocol evidence source for future
  website links and claim boundaries.
- Classification: `NOT_APPLICABLE` as a website source.

### `quantumshieldlabs.org`

- URL: `https://quantumshieldlabs.org/`
- Title: `QuantumShield Labs | Public Post-Quantum Messaging Research`
- Officiality: linked from the official `QuantumShieldLabs` GitHub
  organization profile.
- Public HTTP clues: Cloudflare-served static app surface; current HTML loads
  hashed `/assets/index-*.js` and `/assets/index-*.css` files; robots and
  sitemap are public.
- Source clue: live structured data includes `https://github.com/Tebbens4832/QuantumShield`
  and official QSL public repos in `sameAs`.
- Historical source/deploy clue: NA-0255 D049/D050 archives recorded external
  website PR #19 in `Tebbens4832/QuantumShield`, `npm run build`, Cloudflare
  preview checks, merge to `main`, and automatic Cloudflare Pages deployment to
  production `.org`.
- Current source blocker: `Tebbens4832/QuantumShield` is private under current
  GitHub metadata, and this directive forbids private/login-gated source
  inspection.
- Branch evidence: historical and metadata clues point to `main`, but current
  branch state cannot be independently source-inspected here.
- Build evidence: historical D049/D050 evidence says Vite build via `npm run
  build`; current source package state cannot be inspected here.
- Deploy/hosting evidence: historical D050 evidence and current headers point
  to Cloudflare/Cloudflare Pages behavior; current deployment settings and
  rollback controls are not verifiable here.
- Preview/staging evidence: historical Cloudflare preview URLs existed for PR
  #19; no current preview path or authority is verifiable here.
- Rollback evidence: not verifiable here.
- Edit authority evidence: not verifiable here.
- Claim-scan feasibility: live app asset text is publicly scannable; source
  scan requires operator-authorized source access.
- Link-scan feasibility: sitemap and public evidence links are scannable; source
  output scan requires source/build proof.
- Classification: `OPERATOR_ACTION_REQUIRED`.

### `quantumshieldlabs.dev`

- URL: `https://quantumshieldlabs.dev/`
- Title: `Quantum Shield Labs - Post-Quantum Security for the AI Age`
- Officiality: brand-adjacent company/product site linked from public QSL
  materials, but not the official organization profile website.
- Public HTTP clues: Hostinger/LiteSpeed static HTML surface, public robots and
  sitemap, Tailwind CDN, local `css/*`, local `js/hero-particles.js`, and public
  form action to Formspree.
- Source repo evidence: no exact live source repository verified.
- Branch evidence: not verified.
- Build evidence: static HTML/CSS/JS visible, but no build pipeline verified.
- Deploy/hosting evidence: Hostinger/LiteSpeed headers visible; source-to-host
  deploy path not verified.
- Preview/staging evidence: not verified.
- Rollback evidence: not verified.
- Edit authority evidence: not verified.
- Claim-scan feasibility: live HTML is publicly scannable and has known
  high-risk copy; source scan requires source proof.
- Link-scan feasibility: live link inventory is possible; source/output scan
  requires source proof.
- Classification: `SOURCE_NOT_FOUND`.

### `mbennett-labs/qsl`

- URL: `https://github.com/mbennett-labs/qsl`
- Why considered: prior NA-0296 inspected it and live `.dev` links to the
  `mbennett-labs` profile.
- Default branch: `main`.
- Build system evidence: `project/package.json` has Vite commands
  `dev`, `build`, `lint`, and `preview`; `project/vite.config.ts` uses
  React.
- Source match: not proven to match current `.org` or `.dev` live surfaces.
- Deploy/hosting evidence: absent.
- Preview/rollback/edit authority evidence: absent.
- Classification: `SOURCE_CANDIDATE_UNCONFIRMED`.

## Source / Deploy Classification

Overall classification: `OPERATOR_ACTION_REQUIRED`.

Reason:

- no fully public official website source repository is available;
- `.org` has strong historical/private source and Cloudflare clues, but private
  source content, current authority, preview, and rollback cannot be verified in
  this lane;
- `.dev` has a visible static Hostinger surface, but no source repository,
  branch, build, preview, deploy, or rollback path is verified;
- `mbennett-labs/qsl` remains unconfirmed as live source;
- source discovery does not equal authority to mutate or deploy.

This is not `VERIFIED_SOURCE_READY_FOR_IMPLEMENTATION`.

## Claim / Link Scan Readiness

Claim/link scan readiness classification: `PARTIAL_READY_NEEDS_SOURCE`.

What is ready now:

- local qsl-protocol evidence links can be checked with the repository
  `link-check` helper;
- live `.org` and `.dev` public text/assets can be inventoried read-only;
- live `.dev` high-risk phrase scan is possible from static HTML;
- live `.org` high-risk phrase scan is possible from rendered asset text and
  public routes, with source-level confirmation deferred;
- future source scans have candidate commands from the historical `.org` Vite
  flow, if the operator authorizes that repository later.

Dry-run evidence:

- `.org` app asset phrase scan found no direct matches for the prohibited
  production-readiness, external-review-complete, metadata-free, anonymity,
  untraceable, quantum-proof, unbreakable, guaranteed-secure, website-updated,
  source-verified, or deploy-ready phrases in the direct scan term list.
- `.dev` live HTML scan found high-risk copy requiring review:
  `production-ready` once, `not demos` once, and `not prototypes` once.
  The `production-ready` match appears in NIST/PQC standards copy, not as direct
  qsl-protocol release evidence, but it still needs careful source-linked
  service-copy handling.
- `.dev` also retains the high-risk "Production infrastructure running right
  now" context for external products, which must remain separated from
  qsl-protocol evidence.

What is blocked:

- source-level scan over current website source;
- rendered build-output scan from a clean source build;
- preview URL scan from an authorized website PR;
- rollback proof;
- authority proof for Codex to open a future website PR.

## Operator-Action Requirements

Before NA-0298 can become an implementation lane, the operator must provide:

1. exact website surface to edit: `.org`, `.dev`, or both;
2. exact source repository URL for each target surface;
3. whether any source repository is private and whether Codex may inspect it;
4. default/edit branch and expected base SHA;
5. package manager and build command;
6. preview/staging command or hosting preview mechanism;
7. deployment target and whether deployment is automatic or manual;
8. rollback procedure, including prior deployment or commit rollback path;
9. edit authority: whether Codex may open a website PR later;
10. whether Codex may run a read-only local build of that source;
11. required link-check and overclaim-scan commands for that source tree;
12. any human preview approval requirement before merge or deploy.

What Codex can do without operator action:

- maintain qsl-protocol handoff docs and claim matrices;
- prepare source-agnostic scan checklists;
- map live public claims and links;
- prepare implementation copy bundles that stay evidence-bound;
- recommend blocker-resolution or claim rewrite sequencing.

What remains forbidden:

- production edit or deployment;
- unverified repository mutation;
- private/login-gated source inspection without explicit authorization;
- public form submissions or social/public posts;
- qsl-protocol runtime/protocol/service changes;
- unsupported production, public-internet, external-review-complete,
  anonymity, metadata-free, untraceable, or website-updated claims.

## Decision Tree for NA-0298

- If the operator provides exact source, branch, build, preview, deploy,
  rollback, and authority proof for `.org`, then NA-0298 may be a claim-safe
  website PR lane for `.org` only, with no live deploy unless explicitly
  authorized.
- If the operator provides exact source/deploy/authority proof for `.dev`, then
  NA-0298 may be a claim-rewrite implementation lane for `.dev` only, with
  external-product and qsl-protocol boundaries enforced.
- If source/deploy/authority remains unverified, NA-0298 must remain a website
  source operator-action and implementation-blocker lane.
- If live claims are judged materially risky before source authority is proven,
  NA-0298 should first be a website claim rewrite plan and source verification
  follow-up, not implementation.

Recommended NA-0298 successor if closeout runs now:

`NA-0298 - Website Source Operator Action and Implementation Blocker Resolution`

## Future Implementation Preconditions

A future implementation directive must prove all of the following before edits:

- clean source worktree for the verified website repository;
- source repo URL, default/edit branch, and base SHA;
- current deployment target and hosting provider path;
- package manager lockfile and build command;
- preview/staging command or automatic preview mechanism;
- source-level overclaim scan;
- source/build-output link scan;
- rendered page check for target routes;
- human preview approval if required by operator;
- rollback path;
- no unsupported claim expansion;
- no deployment unless the directive explicitly authorizes deployment.

Candidate future commands after source authorization:

```bash
git status --porcelain=v1 --branch
npm ci
npm run build
npm run preview -- --host 127.0.0.1
node scripts/scan-overclaims.mjs
rg -n -i "production-ready|production ready|deployment-ready|external review complete|metadata-free|anonymity|anonymous messaging|untraceable|quantum-proof|unbreakable|military-grade|guaranteed secure|proven true Triple Ratchet|ready for production|website updated|source verified|deploy ready" src public index.html dist
```

Exact commands remain blocked until the operator verifies the source tree.

## What Remains NOT_READY

- exact `.org` current source inspection under this directive;
- exact `.org` current edit authority;
- exact `.org` current preview and rollback path;
- exact `.dev` source repository;
- exact `.dev` branch/build/preview/deploy/rollback path;
- source-level website link scan;
- source-level website overclaim scan;
- live-site update;
- production readiness;
- public internet service readiness;
- external review completion;
- anonymity, metadata-free messaging, or untraceability;
- production qsl-server or qsl-attachments readiness;
- production desktop release readiness.

## No Website Mutation Proof

This lane used only read-only local files, public web pages, public HTTP
headers, public robots/sitemap files, public GitHub metadata, public source
candidate metadata, and local response archives. It did not edit, clone for
mutation, push to, deploy, configure, or open a PR against any website or
external website repository.

## No Implementation Change Proof

The qsl-protocol patch is limited to governance/readiness evidence, testplan,
handoff reference, D-0570, traceability, and the rolling operations journal.
It changes no qsl-protocol runtime, protocol, crypto, state-machine, demo,
service, qsc-desktop, qsl-server, qsl-attachments, workflow, script, Cargo,
dependency, branch-protection, or public-safety configuration path.
