# NA-0072 Repo Housekeeping Plan

## Scope
- Docs and repo-structure cleanup only. No protocol/client/demo code changes.
- Allowed areas: docs/**, tests/** (plan + harness docs), .github/** (PR template dedupe), START_HERE*.md, TRACEABILITY.md.

## Invariants
- No duplicate canonical artifacts (single source of truth).
- Reference integrity: no docs/scripts point at removed paths.
- CI lanes remain green after cleanup.

## Candidates
- PR template dedupe: keep .github/PULL_REQUEST_TEMPLATE.md; remove duplicate lowercase file.
- Harness dedupe: test-harness/4b vs tests/harness/4b (only if safe).
- START_HERE_2.md: archive deprecated entry point.

## Do-not-touch
- inputs/** artifacts and any CI-required evidence.
- tools/**, qsl/**, apps/**, formal/**, scripts/** (code semantics).

## Executed (2026-01-27)
- PR template dedupe:
  - Removed: .github/pull_request_template.md
  - Kept: .github/PULL_REQUEST_TEMPLATE.md
  - Evidence: repo-wide rg scan shows only uppercase path referenced.
- START_HERE_2.md archived:
  - Moved to docs/archive/START_HERE_2.md with tombstone banner.
  - References updated in docs/DOCS_MAP.md and docs/master/DOC-CTRL-001_... to point to archived path.

## Deferred / Blocked
- Harness dedupe (test-harness/4b): deferred.
  - Reason: README.md references test-harness/ (README is out of scope for NA-0072 execution).
  - CI/scripts use tests/harness/4b; cannot remove test-harness/ without updating README.

## Verification checklist
- rg reference scans before/after for removed paths.
- actionlint .github/workflows
- CI checks green on PR.

## Evidence (commands run)
- rg -n "PULL_REQUEST_TEMPLATE|pull_request_template" -S .
- rg -n "START_HERE_2\.md" -S .
- rg -n "(test-harness/|tests/harness/)" -S .
