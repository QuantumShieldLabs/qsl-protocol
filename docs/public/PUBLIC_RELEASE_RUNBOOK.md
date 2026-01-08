# PUBLIC RELEASE RUNBOOK (AUTHORITATIVE ORDER)

## Current state
- Freeze mode active (no new NA promotions).
- Baseline tag: post-audit-pre-public-scrub-20260105.

## Operating rules
- One active directive at a time.
- One PR per directive; do not mix steps.
- Newer directives supersede older ones.
- No CI retrigger loops: rerun checks only after a concrete change and at most once per change.

## Ordered phases

### Step 1 (CURRENT) — Public guardrails
Scope: allowlist policy, denylist scan, CI gate additions.

Definition of done:
- Allowlist policy documented and enforced (explicitly enumerated public-safe paths).
- Denylist scan is scripted and CI-gated.
- CI guardrail runs on governance/docs PRs without expanding scope.
- Go/No-Go: all guardrail checks pass on a clean main; no drift from allowlist.

### Step 2 — Inventory + scrub
Scope: redact sensitive content, ensure doc completeness.

Definition of done:
- All allowlisted paths reviewed and scrubbed.
- Redaction log recorded (what changed and why).
- Doc completeness checks pass (no missing required public docs).
- Go/No-Go: no remaining sensitive content in allowlisted paths.

### Step 3 — License + repo hygiene
Scope: LICENSE/NOTICE/SECURITY/CONTRIBUTING/README alignment.

Definition of done:
- License decision recorded and files added (LICENSE, NOTICE if needed).
- SECURITY.md and CONTRIBUTING.md in place with contact and disclosure flow.
- README reflects public posture, build/test steps, and status.
- Go/No-Go: legal and disclosure checklist completed.

### Step 4 — Dry-run public export + cutover
Scope: export, validate, then cut public repo (including community health files, templates, CODEOWNERS, checklists, and minimal public CI).

Definition of done:
- Dry-run export succeeds with allowlist-only content.
- CI passes in the public mirror configuration.
- Public repo created with tags and release notes.
- Go/No-Go: final security review sign-off and release checklist complete.

High-confidence credential scan exclusion:
- .github/workflows/public-ci.yml is excluded because it intentionally contains the detection regex used in public CI.
- Any additional exclusions require an explicit decision and PR review.
- Preferred exclusion form (portable): use ripgrep glob exclude, e.g. `rg -g '!.github/workflows/public-ci.yml' <pattern> .`.
