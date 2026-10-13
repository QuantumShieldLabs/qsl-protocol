# Contributing

Goals: G1, G2, G3, G4, G5

Revision: 2026-10-13. Read [the project workflow](PROJECT_WORKFLOW.md).

Implement one connected scoped change; run relevant targeted local checks and preserve all required CI before human merge. Log every actual shortcut in the one HARDENING.md ledger with cut/risk/mandatory trigger. Sensitive interfaces are independently reviewed before dependent use. An already scoped task does not need a duplicate issue or new routine approval. Report exact source, tests and limitations; never publish credentials/private raw evidence.

## Local checks
Run the smallest relevant checks for your change. Examples:
- `python3 tools/goal_lint.py` (requires a prepared PR event payload)
- `./scripts/ci/metadata_conformance_smoke.sh`

## Code of conduct
Behavior expectations are defined in `CODE_OF_CONDUCT.md`.
