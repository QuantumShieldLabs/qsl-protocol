Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-16
Replaces: n/a
Superseded-By: n/a

# NA-0298 Website Operator Action Blocker Resolution

## Executive Summary

NA-0298 checked whether an operator-provided website source, deployment, and
authority bundle exists for the public QSL website surfaces. It does not.

Bundle classification: `OPERATOR_BUNDLE_INCOMPLETE`.

Implementation cannot proceed from the available evidence. The repo still has
only public surface facts and unverified source/deploy clues:

- `quantumshieldlabs.org` remains the official organization-linked public QSL
  evidence surface. It is Cloudflare-served and its live HTML references
  `Tebbens4832/QuantumShield`, but that source is not publicly inspectable and
  no current operator authority, branch/base SHA, preview flow, deploy path, or
  rollback process is verified here.
- `quantumshieldlabs.dev` remains a broader company/product surface. It is
  served through LiteSpeed/Hostinger-style public headers and links to
  `mbennett-labs` public repositories, but no exact source repository, branch,
  build, deploy, preview, rollback, or edit authority is verified here.
- `mbennett-labs/qsl` remains an unconfirmed public source candidate. It has
  Vite/React project files, but it is not verified as the live source for
  `.org` or `.dev`.

NA-0298 therefore creates a precise operator-action request packet and leaves
website mutation blocked. It does not edit a website, create a website PR,
deploy, mutate DNS/hosting, submit forms, post publicly, generate media, or
change qsl-protocol runtime/protocol/service implementation paths.

## Scope and Non-Goals

In scope:

- read-only qsl-protocol handoff search;
- read-only public website and GitHub recheck;
- exact required operator bundle matrix;
- operator-action request packet and copy-paste response template;
- qsl-protocol governance evidence, testplan, handoff reference, decision,
  traceability, and rolling-journal updates.

Out of scope:

- website or external website repository mutation;
- website PR creation, merge, deployment, DNS, hosting, settings, form
  submission, comments, public posts, or social actions;
- private/login-gated source inspection;
- image/video generation or publishing;
- qsl-protocol runtime, protocol, crypto, state-machine, demo, service,
  qsc-desktop, qsl-server, qsl-attachments, qsp, qsc, qsl-client, apps, tools,
  inputs, or formal implementation changes;
- README, START_HERE, `.github/**`, `scripts/**`, Cargo/dependency, workflow,
  branch-protection, or public-safety configuration changes;
- production-readiness, public-internet-readiness, external-review-complete,
  anonymity, metadata-free, untraceable, quantum-proof, unbreakable,
  guaranteed-secure, website-updated, source-verified, deploy-ready, or
  implementation-complete claims.

## Sources Inspected

Local qsl-protocol sources:

- `NEXT_ACTIONS.md`
- [NA-0297 website source blocker resolution audit](NA-0297_website_source_blocker_resolution_audit.md)
- [NA-0296 website source verification readiness audit](NA-0296_website_source_verification_readiness_audit.md)
- [NA-0295 website landing evidence visuals plan](NA-0295_website_landing_evidence_visuals_plan.md)
- [Website implementation handoff](../../public/WEBSITE_IMPLEMENTATION_HANDOFF.md)
- [Website claim matrix](../../public/WEBSITE_CLAIM_MATRIX.md)
- [Website update plan](../../public/WEBSITE_UPDATE_PLAN.md)
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `README.md`
- `START_HERE.md`
- `TRACEABILITY.md`
- `DECISIONS.md`

Read-only public surfaces and repositories:

- `https://quantumshieldlabs.org/`
- `https://quantumshieldlabs.org/robots.txt`
- `https://quantumshieldlabs.org/sitemap.xml`
- `https://quantumshieldlabs.dev/`
- `https://quantumshieldlabs.dev/robots.txt`
- `https://quantumshieldlabs.dev/sitemap.xml`
- `https://github.com/QuantumShieldLabs`
- `https://github.com/QuantumShieldLabs/qsl-protocol`
- `https://github.com/QuantumShieldLabs/qsl-server`
- `https://github.com/QuantumShieldLabs/qsl-attachments`
- `https://github.com/mbennett-labs/qsl`
- public unauthenticated GitHub metadata for
  `https://github.com/Tebbens4832/QuantumShield`, which returned not found.

No private source, dashboard, hosting console, DNS panel, form, settings page,
or authenticated-only website data was inspected.

## Operator Bundle Discovery Matrix

| # | Required field | Classification | Evidence found | Blocker / note |
| --- | --- | --- | --- | --- |
| 1 | Exact target surface: `quantumshieldlabs.org`, `quantumshieldlabs.dev`, or both | `MISSING` | Local docs and live recheck identify both surfaces. | No operator selection says which surface is the future edit target. |
| 2 | Exact website source repository URL | `MISSING` | `.org` HTML references `Tebbens4832/QuantumShield`; `.dev` links to `mbennett-labs`; `mbennett-labs/qsl` is public. | These are clues/candidates, not an operator-provided verified source URL. |
| 3 | Exact branch and base SHA | `MISSING` | Historical and candidate clues mention `main`. | No verified current source branch or exact base SHA exists for the live target. |
| 4 | Package manager | `MISSING` | Historical `.org` and candidate `mbennett-labs/qsl` clues suggest npm/Vite. | No target source is verified, so package manager cannot be accepted. |
| 5 | Build command | `MISSING` | Historical `.org` evidence and `mbennett-labs/qsl` candidate show Vite build-style commands. | No verified target-source build command exists. |
| 6 | Preview/staging command or preview process | `MISSING` | Historical `.org` evidence mentioned Cloudflare preview behavior. | No current preview or staging command/process is verified. |
| 7 | Deployment target/provider/path | `PROVIDED_UNVERIFIED` | `.org` is Cloudflare-served; `.dev` is LiteSpeed/Hostinger-style. | Public headers do not prove source-to-deploy path, target project, or deploy authority. |
| 8 | Rollback process | `MISSING` | No current rollback process found. | Must be supplied by the operator before implementation. |
| 9 | Hosting/deployment authority | `MISSING` | No authority grant found. | Codex cannot assume hosting or deployment authority from public headers. |
| 10 | Whether Codex may open a PR in the website source repo later | `MISSING` | No explicit permission found. | Future website PR permission must be explicit. |
| 11 | Whether Codex may run a read-only local build later | `MISSING` | No explicit permission found. | Build permission must be explicit even if source is public. |
| 12 | Whether Codex may edit source only, or source plus deployment config, in a future directive | `MISSING` | No explicit edit scope found. | Future edit scope must be explicit and path-bounded. |
| 13 | Whether implementation requires user/operator approval before merge/deploy | `MISSING` | No approval rule found. | Merge/deploy approval gates must be stated before implementation. |
| 14 | Required environment variables or statement that none are needed for build/preview | `MISSING` | No build/preview environment statement found. | Public docs must not receive secret values; only variable names/status belong in the bundle. |
| 15 | Whether live deployment is forbidden unless separately authorized | `PROVIDED_VERIFIED` | Current directive and governance scope forbid deployment unless separately authorized. | This field is satisfied for the current lane, but future operator response should repeat it. |

## External Recheck Findings

| Target | Status | Officiality | Source/deploy clues | Claim risk clues | Fields resolved |
| --- | --- | --- | --- | --- | --- |
| `https://quantumshieldlabs.org/` | Inspected read-only. Title: `QuantumShield Labs | Public Post-Quantum Messaging Research`. | Official: linked from the `QuantumShieldLabs` GitHub organization profile. | Cloudflare-served static app; live assets under `/assets/`; HTML references `QuantumShieldLabs/qsl-protocol`, `QuantumShieldLabs/qsl-server`, and `Tebbens4832/QuantumShield`. | Direct phrase scan of HTML plus app asset found no matches for the high-risk direct terms in this lane's scan list. | Confirms official surface and public deploy clue only; no source authority field resolved. |
| `https://quantumshieldlabs.dev/` | Inspected read-only. Title: `Quantum Shield Labs - Post-Quantum Security for the AI Age`. | Brand/company surface, not the official GitHub organization website. | LiteSpeed/Hostinger-style public surface; local `css/*` and `js/*`; Formspree form action visible; links to `mbennett-labs` repos. | Direct phrase scan found `production-ready`, `not demos`, and `not prototypes` in company/product contexts requiring separation from qsl-protocol claims. | Confirms public surface and claim risk only; no source authority field resolved. |
| `https://github.com/QuantumShieldLabs` | Inspected read-only. | Official GitHub organization. | Lists only `.github`, `qsl-protocol`, `qsl-server`, and `qsl-attachments` as public repositories; website link is `.org`. | Org profile states current artifacts are research-grade until formal security review. | Confirms no public official website source repo. |
| `https://github.com/QuantumShieldLabs/qsl-protocol` | Inspected read-only. | Official public QSL protocol repo. | Public specs, vectors, and research-stage reference implementation; not website source. | README keeps production, public internet, review, anonymity, metadata-free, and untraceability gaps visible. | Not applicable as website source. |
| `https://github.com/QuantumShieldLabs/qsl-server` | Inspected read-only. | Official public relay repo. | Transport-only relay code; not website source. | Scope says no protocol or cryptographic behavior in relay. | Not applicable as website source. |
| `https://github.com/QuantumShieldLabs/qsl-attachments` | Inspected read-only. | Official public attachment repo. | Opaque encrypted attachment plane/runtime; not website source. | Operational posture remains single-node/local-disk with explicit limits. | Not applicable as website source. |
| `https://github.com/mbennett-labs/qsl` | Inspected read-only. | Unofficial/unconfirmed source candidate. | Default branch displays `main`; public `project/package.json` has Vite scripts: `dev`, `build`, `lint`, `preview`. | Does not prove live source for `.org` or `.dev`. | Candidate package/build clues only; no required field resolved. |
| `https://github.com/Tebbens4832/QuantumShield` | Public unauthenticated metadata checked read-only. | Unverified/private clue from `.org` HTML and historical NA-0255 evidence. | Public API returned not found. | Not inspected as private/login-gated source. | No required field resolved. |

No external recheck target authorizes website mutation, source repo mutation,
website PR creation, deployment, DNS/hosting changes, public posting, form
submission, or unsupported claim generation.

## Bundle Classification

Classification: `OPERATOR_BUNDLE_INCOMPLETE`.

Reason:

- the exact target surface has not been operator-selected;
- the exact source repository URL is not verified;
- branch/base SHA, package manager, build command, preview/staging process,
  deploy target/path, rollback, hosting/deploy authority, PR permission,
  read-only build permission, edit scope, approval rules, and build/preview
  environment status are missing or only public clues;
- no contradictory operator bundle exists; the issue is incompleteness, not
  conflict.

Implementation can proceed later only after an operator supplies and authorizes
the missing fields, and after a future directive separately permits the
specific website source action. The current blocker-resolution packet is not
implementation authorization.

## Missing and Contradictory Fields

Missing critical fields:

- exact future target surface;
- exact website source repository URL;
- exact branch and base SHA;
- package manager;
- build command;
- preview/staging command or process;
- current deployment target/provider/path sufficient to connect source to host;
- rollback process;
- hosting/deployment authority;
- future website PR permission;
- future read-only local build permission;
- future edit-scope permission;
- future merge/deploy approval requirement;
- required environment variable names/status, or explicit statement that none
  are needed for build/preview.

Contradictory fields: none found.

Unverified clues that must not be treated as authority:

- `.org` Cloudflare serving behavior;
- `.org` HTML reference to `Tebbens4832/QuantumShield`;
- historical NA-0255 PR/build/deploy evidence;
- `.dev` LiteSpeed/Hostinger-style serving behavior;
- `.dev` links to `mbennett-labs`;
- `mbennett-labs/qsl` Vite/React project files.

## Operator Action Request

Implementation is blocked because Codex does not have a verified source,
branch/base SHA, build/preview/deploy/rollback path, or authority grant for the
website surface. Guessing from public source clues could mutate the wrong
repository, scan the wrong branch, or imply deploy authority that has not been
granted.

Website mutation remains forbidden until a future directive supplies the bundle
and explicitly authorizes the exact next action. Source discovery does not
equal authority to edit. Historical deployment clues do not equal current
rollback proof. Public website text does not equal source ownership.

The operator should provide the following bundle. Do not paste secret values
into public docs or issue/PR bodies; provide only variable names, whether values
are required, and the secure channel/process for supplying secrets if a later
build/preview needs them.

Required fields:

1. exact target surface: `quantumshieldlabs.org`, `quantumshieldlabs.dev`, or
   both;
2. exact website source repository URL for each target surface;
3. exact branch and base SHA;
4. package manager;
5. build command;
6. preview/staging command or process;
7. deployment target/provider/path;
8. rollback process;
9. hosting/deployment authority;
10. whether Codex may open a PR in the website source repo later;
11. whether Codex may run a read-only local build later;
12. whether Codex may edit only source, or source plus deployment config, in a
    future directive;
13. whether website implementation requires user/operator approval before
    merge or deploy;
14. environment variable names/status, or a statement that no environment
    variables are needed for build/preview;
15. whether live deployment is forbidden unless separately authorized.

## Copy-Paste Operator Response Template

```md
Website operator source/deploy/authority bundle

Target surface:
- [ ] quantumshieldlabs.org
- [ ] quantumshieldlabs.dev
- [ ] both

Source repository:
- URL:
- Visibility: public/private
- Codex may inspect source later: yes/no

Branch/base:
- Branch:
- Base SHA:

Build:
- Package manager:
- Install command:
- Build command:
- Preview/staging command or process:

Deployment:
- Provider:
- Project/site/path:
- Deployment trigger: automatic/manual/other
- Rollback process:
- Hosting/deployment authority holder:

Codex permissions for a future directive:
- May open a website source PR: yes/no
- May run a read-only local build: yes/no
- May edit: source only / source plus deployment config / other:
- Merge approval required before merge: yes/no
- Deployment approval required before deploy: yes/no
- Live deployment forbidden unless separately authorized: yes/no

Environment:
- Build/preview environment variables required: none / listed below
- Variable names only, no secret values:
  - NAME: required/optional; value delivery channel/process:

Notes:
- Claim boundaries or copy constraints:
- Required reviewers/approvers:
- Additional stop conditions:
```

## Example Safe Response Format

```md
Website operator source/deploy/authority bundle

Target surface:
- [x] quantumshieldlabs.org
- [ ] quantumshieldlabs.dev
- [ ] both

Source repository:
- URL: https://github.com/example-owner/example-website
- Visibility: private
- Codex may inspect source later: yes, in a separate directive

Branch/base:
- Branch: main
- Base SHA: abcdef123456

Build:
- Package manager: npm
- Install command: npm ci
- Build command: npm run build
- Preview/staging command or process: hosting provider creates PR preview

Deployment:
- Provider: example provider
- Project/site/path: example project name
- Deployment trigger: automatic on merge to main
- Rollback process: revert merge commit and redeploy previous production build
- Hosting/deployment authority holder: named operator/team

Codex permissions for a future directive:
- May open a website source PR: yes
- May run a read-only local build: yes
- May edit: source only
- Merge approval required before merge: yes
- Deployment approval required before deploy: yes
- Live deployment forbidden unless separately authorized: yes

Environment:
- Build/preview environment variables required: none
- Variable names only, no secret values: n/a

Notes:
- Keep research-stage, non-production, external-review, metadata, and service
  readiness boundaries visible.
```

The example above is format-only. It is not evidence for the QSL website.

## Future Use of Bundle

After a complete bundle is supplied, a future NA-0299 or successor directive may
use it to:

- verify the exact website source worktree and branch/base SHA;
- run an authorized read-only local build if permitted;
- run source-level link and overclaim scans;
- prepare a claim-safe website PR only if PR permission is explicit;
- keep deployment disabled unless deploy approval is separately explicit;
- record rollback and preview proof before any merge/deploy decision;
- preserve all qsl-protocol NOT_READY boundaries in public copy.

If the bundle remains incomplete, the next successor should continue
operator-action capture rather than implementation. If the bundle contains
conflicting data, the next successor should resolve the authority conflict
before implementation.

## What Remains Forbidden

Even after the bundle is supplied, the following remain forbidden unless a later
directive explicitly authorizes and scopes them:

- live deployment;
- secret exposure in public docs, comments, PR bodies, logs, or screenshots;
- unsupported production-readiness or public-internet-readiness claims;
- unsupported external-review-complete claims;
- anonymity, metadata-free, or untraceable messaging claims;
- quantum-proof, unbreakable, guaranteed-secure, military-grade, or similar
  hype claims;
- branch-protection, public-safety, workflow, script, Cargo, or dependency
  changes;
- qsl-protocol protocol/crypto/runtime/demo/service implementation changes;
- qsl-server, qsl-attachments, qsc-desktop, qsp, qsc, qsl-client, apps, tools,
  inputs, or formal implementation changes unless separately scoped.

## Successor Recommendation

Live NA-0298 acceptance requires exactly one READY item to remain NA-0298. This
packet therefore recommends leaving NA-0298 READY after Packet F.

If a later governance closeout is explicitly authorized with a successor, choose
based on the then-current bundle state:

- complete and verified bundle: promote a claim-safe website implementation
  readiness PR lane, still forbidding live deploy unless deploy approval is
  explicit;
- incomplete/missing bundle: promote `Operator Website Source Authority Bundle
  Required`, governance/action docs only, no website implementation;
- contradictory bundle: promote `Website Source Authority Conflict
  Resolution`, no website implementation.

## No Website Mutation Proof

NA-0298 used only local qsl-protocol docs, public unauthenticated website pages,
public robots/sitemap files, public headers, public GitHub pages, public raw
candidate-source files, and public unauthenticated GitHub metadata. It did not
edit, clone for mutation, push to, deploy, configure, submit forms on, or open
a PR against any website or external website repository.

## No Implementation Change Proof

The qsl-protocol patch is limited to governance/readiness evidence, testplan,
handoff reference, D-0572, traceability, and the rolling operations journal.
It changes no qsl-protocol runtime, protocol, crypto, state-machine, demo,
service, qsc-desktop, qsl-server, qsl-attachments, workflow, script, Cargo,
dependency, branch-protection, public-safety configuration, website, external
website repository, formal model, input vector, tool, app, README, or
START_HERE path.
