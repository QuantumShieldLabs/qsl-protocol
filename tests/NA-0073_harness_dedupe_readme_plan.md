# NA-0073 Harness Dedupe + README Alignment Plan

## Scope
- Docs/structure only: README.md + harness directories + reference integrity.
- No protocol/client/demo code changes.

## Candidate directories
- test-harness/4b/**
- tests/harness/4b/**

## Reference scans
- rg -n "(test-harness/|tests/harness/)" -S .
- rg -n "README.md" -S docs tests .

## Proposed change
- Select a single authoritative harness root.
- Remove or relocate the legacy harness directory.
- Update README.md references to the authoritative path.

## Verification checklist
- rg finds no references to removed harness path.
- CI required contexts pass.
- TRACEABILITY updated with PR links and plan evidence.

## Rollback
- Revert the PR to restore the removed directory and references if CI or reference integrity fails.
