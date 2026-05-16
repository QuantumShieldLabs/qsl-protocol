Goals: G1, G2, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-16
Replaces: n/a
Superseded-By: n/a

# NA-0296 Website Source Verification Readiness Testplan

## Objective

Verify that NA-0296 records a read-only website/source discovery and
claim-boundary readiness audit before any website implementation work, and that
all unsupported public claims remain blocked.

## Protected Invariants

- Website source verification does not equal website implementation.
- Website audit does not equal live-site update.
- Website handoff does not equal production readiness.
- External review package existence does not equal external review completion.
- Metadata minimization evidence does not equal anonymity or metadata-free
  messaging.
- Service hardening evidence does not equal production service readiness.
- Public internet readiness remains NOT_READY unless separately proven.
- All NOT_READY gates remain visible.

## Allowed Scope

- `docs/governance/evidence/NA-0296_website_source_verification_readiness_audit.md`
- `tests/NA-0296_website_source_verification_readiness_testplan.md`
- `docs/public/WEBSITE_IMPLEMENTATION_HANDOFF.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

- Website or external website repository mutation.
- Website deployment, DNS, hosting, settings, forms, comments, or public posts.
- `.github/**`, `scripts/**`, `Cargo.toml`, `Cargo.lock`, dependency updates,
  branch-protection changes, or public-safety configuration changes.
- qsl-protocol runtime, protocol, crypto, demo, service, qsc-desktop,
  qsl-server, qsl-attachments, qsp, qsc, qsl, apps, tools, inputs, or formal
  implementation changes.

## Web Read-Only Audit Requirements

- Inspect official public website surfaces discovered from local docs and
  official GitHub metadata.
- Inspect official public GitHub organization/repository metadata.
- Inspect only public pages and public repositories.
- Do not inspect private/login-gated content.
- Do not submit forms, comments, or public posts.
- Record titles, URLs, visible sections, claim language, source clues, and
  source verification gaps.

## Source Verification Requirements

The audit must state whether each item is verified or unverified:

- website surface
- source repository
- branch
- build system
- deployment target
- preview/staging flow
- rollback path
- pre-deploy link scan
- pre-deploy claim scan

If exact source cannot be verified, the classification must remain blocked or
partial and must not infer ownership.

## Live Claim-Boundary Requirements

The audit must classify visible claims as one of:

- `SAFE`
- `SAFE_WITH_EVIDENCE_LINK`
- `NEEDS_REWRITE`
- `PROHIBITED`
- `UNVERIFIED`
- `NOT_APPLICABLE`

The audit must flag or block unsupported claims about:

- production readiness
- public internet readiness
- external review completion
- metadata-free messaging
- anonymity
- anonymous messaging
- untraceability
- quantum-proof security
- unbreakable security
- military-grade security
- guaranteed secure behavior
- proven true Triple Ratchet equivalence
- live website updates

## Readiness Classification Requirements

The audit must select one readiness classification and explain why:

- `READY_FOR_CLAIM_SAFE_IMPLEMENTATION`
- `PARTIAL_READY_SOURCE_UNVERIFIED`
- `NOT_READY_SOURCE_UNKNOWN`
- `NOT_READY_CLAIM_RISK`
- `NOT_READY_DEPLOYMENT_PATH_UNKNOWN`

If source repository, branch, build, and deployment are not all verified, the
audit must not classify the lane as ready for implementation.

## Link / Leak / Overclaim Expectations

- All new relative links must resolve.
- Evidence text must avoid raw secrets, auth headers, endpoint tokens, and
  unnecessary long-hex dumps.
- High-risk phrases are allowed only when negated, marked prohibited, marked
  NOT_READY, or used as examples of what not to claim.
- The audit must preserve visible gaps instead of hiding them.

## CI Expectations

Required local validation:

- clean worktree/status proof
- diff name-only and diff stat proof
- `git diff --check`
- queue proof
- decision proof with D-0568 once and no duplicate decision IDs
- scope guard over the exact allowed path set
- link check
- added-content leak scan
- direct overclaim scan
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- formal/model checks when present
- classifier proof for the changed path set

Required PR validation:

- public-safety is required before PR creation.
- Required checks complete normally before merge.
- No admin bypass, direct push, squash, rebase, branch deletion, or
  delete-branch flag is used.

## Future Implementation Gate

A future website implementation directive must start from verified source,
branch, build, preview, deploy, rollback, link-scan, and claim-scan evidence.
If any of those are unknown, the future lane must remain a blocker-resolution or
source-verification lane and must not edit or deploy the website.
