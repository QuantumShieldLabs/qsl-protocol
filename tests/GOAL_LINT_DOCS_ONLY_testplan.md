# Goal-Lint Policy — Docs-Only Exception Test Plan (DRAFT)

Goals: G4, G5

## Purpose
Document that documentation-only PRs (README/docs/.github templates) are exempt from the tests/vectors/harness requirement, while protocol/code changes remain fail-closed.

## Scope
- Applies only to docs-only changes (README.md, docs/**, docs/INDEX.md, docs/CHECKLIST_*.md, .github/ISSUE_TEMPLATE/**, .github/PULL_REQUEST_TEMPLATE.md)
- Does not change governance requirements (DECISIONS/TRACEABILITY remain required when core paths are touched)

## Validation steps
1. Open a PR that touches only README.md and docs/INDEX.md with proper PR body Goals.
2. Confirm goal-lint does NOT require tests/vectors/harness changes.
3. Open a PR that touches protocol/code paths (e.g., tools/, src/, specs/, or docs/ with non-doc-only paths) without tests/vectors updates.
4. Confirm goal-lint FAILS with the tests/vectors/harness requirement.

## Expected results
- Docs-only PRs pass without tests/vectors/harness changes.
- Protocol/code PRs still require tests/vectors/harness updates.
