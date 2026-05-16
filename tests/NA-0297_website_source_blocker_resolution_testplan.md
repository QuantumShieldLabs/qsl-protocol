Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-16
Replaces: n/a
Superseded-By: n/a

# NA-0297 Website Source Blocker Resolution Testplan

## Objective

Verify that NA-0297 records a read-only website source/deploy follow-up and
implementation-blocker resolution plan before any website implementation is
authorized.

## Protected Invariants

- Website/source discovery does not authorize website mutation.
- Website audit does not equal live-site update.
- Source clues do not equal edit authority.
- A private or login-gated source repository is an operator-action blocker
  unless explicitly authorized in a later directive.
- External review package existence does not equal external review completion.
- Metadata minimization evidence does not equal anonymity or metadata-free
  messaging.
- Service hardening evidence does not equal production service readiness.
- Public internet readiness remains NOT_READY unless separately proven.
- All NOT_READY gates remain visible.

## Allowed Scope

- `docs/governance/evidence/NA-0297_website_source_blocker_resolution_audit.md`
- `tests/NA-0297_website_source_blocker_resolution_testplan.md`
- `docs/public/WEBSITE_IMPLEMENTATION_HANDOFF.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

- website or external website repository mutation;
- website PR creation, merge, deployment, DNS, hosting, settings, forms,
  comments, or public posts;
- private/login-gated source inspection without later explicit authorization;
- `README.md`, `START_HERE.md`, `.github/**`, `scripts/**`, `Cargo.toml`,
  `Cargo.lock`, dependency updates, branch-protection changes, or public-safety
  configuration changes;
- qsl-protocol runtime, protocol, crypto, demo, service, qsc-desktop,
  qsl-server, qsl-attachments, qsp, qsc, qsl, qsl-client, apps, tools, inputs,
  or formal implementation changes.

## Web / Source Read-Only Audit Requirements

- Inspect official public website surfaces discovered from local docs and
  official GitHub metadata.
- Inspect official public GitHub organization/repository metadata.
- Inspect only public pages, public repositories, public HTTP headers, public
  robots/sitemap files, and public page assets.
- Do not submit forms, comments, social posts, or settings changes.
- Do not inspect private/login-gated source content.
- Record titles, URLs, visible sections, source clues, hosting clues, claim
  language, and remaining blockers.

## Source / Deploy Verification Requirements

The audit must state whether each item is verified or unverified:

- website surface;
- source repository;
- source branch;
- build system and build command;
- deployment target;
- preview/staging flow;
- rollback path;
- edit authority;
- pre-deploy link scan;
- pre-deploy claim scan.

If exact source, branch, build, preview, deploy, rollback, and authority are
not all verified, the audit must not recommend implementation.

## Claim / Link Scan Readiness Requirements

The audit must classify scan readiness as one of:

- `READY_FOR_FUTURE_SOURCE_SCAN`
- `PARTIAL_READY_NEEDS_SOURCE`
- `NOT_READY_SOURCE_UNKNOWN`
- `NOT_READY_CLAIM_RISK`

The audit must identify:

- live public claim inventory feasibility;
- local evidence link inventory feasibility;
- source-level scan blockers;
- future source scan commands when known;
- live public claim risks that need rewrite or evidence-boundary labels.

## Operator-Action / Blocker Plan Requirements

If source/deploy/authority cannot be fully verified, the audit must list exact
operator inputs needed:

- source repo URL;
- branch and base SHA;
- build command;
- preview/staging command or hosting preview mechanism;
- deployment target;
- rollback process;
- edit authority;
- whether Codex may open a website PR later;
- whether Codex may run a read-only local source build.

The audit must keep production edits, live deployment, and unverified source
mutation forbidden.

## Link / Leak / Overclaim Expectations

- All new relative links must resolve.
- Evidence text must avoid raw secrets, auth headers, endpoint tokens, and
  unnecessary long-hex dumps.
- High-risk phrases are allowed only when negated, marked prohibited, marked
  NOT_READY, or used as scan evidence with clear caveat.
- The patch must not introduce unsupported production-readiness,
  public-internet-readiness, external-review-complete, metadata-free,
  anonymity, untraceable, quantum-proof, unbreakable, guaranteed-secure,
  website-updated, source-verified, deploy-ready, or implementation-complete
  claims.

## CI Expectations

Required local validation:

- clean worktree/status proof;
- diff name-only and diff stat proof;
- `git diff --check`;
- direct overclaim phrase scan over changed lines;
- `python3 scripts/ci/qsl_evidence_helper.py queue`;
- `python3 scripts/ci/qsl_evidence_helper.py decisions`;
- `python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main`
  with the exact allowed path set;
- `python3 scripts/ci/qsl_evidence_helper.py link-check`;
- `python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main`;
- `cargo audit --deny warnings`;
- `cargo tree -i rustls-webpki --locked`;
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`;
- `python3 formal/run_model_checks.py` when present;
- goal-lint;
- classifier proof for the changed path set.

Required PR validation:

- public-safety is required before PR creation;
- required checks complete normally before merge;
- no admin bypass, direct push, squash, rebase, branch deletion, or
  delete-branch flag is used.

## Future Implementation Gate

A future website implementation directive must start from verified source,
branch, build, preview, deploy, rollback, link-scan, claim-scan, and authority
evidence. If any of those are unknown, the future lane must remain a
blocker-resolution or operator-action lane and must not edit or deploy the
website.
