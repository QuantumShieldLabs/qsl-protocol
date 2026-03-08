# Contributing

## Scope
This repository is in freeze mode for public-release preparation. Contributions are limited to:
- Documentation and governance updates
- Vector/test artifacts
- Public-release scrub controls

Protocol or implementation changes are out of scope unless explicitly authorized.

## Workflow
1) Open an issue or discussion describing the change.
2) Follow canonical public/release posture docs: `docs/public/INDEX.md`.
3) Keep changes minimal and scoped; avoid unrelated refactors.
4) Use docs hygiene guardrails for docs moves/renames and evidence capture.

## Local checks
Run the smallest relevant checks for your change. Examples:
- `python3 tools/goal_lint.py` (requires a prepared PR event payload)
- `./scripts/ci/metadata_conformance_smoke.sh`

## Code of conduct
Behavior expectations are defined in `CODE_OF_CONDUCT.md`.
