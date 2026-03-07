# PR-0014 — Docs Cutover Accuracy Test Plan (DRAFT)

Goals: G4, G5

## Purpose
Validate that README.md and docs/INDEX.md accurately reflect the v0.2.0 public development cutover and key entry points.

## Scope
- README.md and docs/INDEX.md accuracy only
- No protocol semantics or vector content changes

## Validation steps
1. Confirm README shows latest tag v0.2.0-draft and retains DRAFT/not-audited language.
2. Confirm README repository contents map includes START_HERE, docs/, inputs/, tests/, harness, apps/qshield-cli, formal, schemas, scripts/ci.
3. Confirm docs/INDEX links to START_HERE.md, GOALS.md, PROJECT_CHARTER.md, and CHECKLIST_PROTOCOL_CHANGE.md.

## Expected results
- Links and labels are present and correct.
- No claim of production readiness.
