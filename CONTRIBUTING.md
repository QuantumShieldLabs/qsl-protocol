# Contributing

Goals: G1, G2, G3, G4, G5

Revision: 2026-10-13. Read [the project workflow](PROJECT_WORKFLOW.md).

Follow PROJECT_WORKFLOW.md for task scope, safety, independent review and hardening gates. This guide adds component-local check references and contributor conduct.

## Local checks
Run the smallest relevant checks for your change. Examples:
- `python3 tools/goal_lint.py` (requires a prepared PR event payload)
- `./scripts/ci/metadata_conformance_smoke.sh`

## Code of conduct
Behavior expectations are defined in `CODE_OF_CONDUCT.md`.
