# Contributing

## Scope
This repository is in freeze mode for public-release preparation. Contributions are limited to:
- Documentation and governance updates
- Vector/test artifacts
- Public-release scrub controls

Protocol or implementation changes are out of scope unless explicitly authorized.

## Workflow
1) Open an issue or discussion describing the change.
2) Follow the public-release runbook: `docs/public/PUBLIC_RELEASE_RUNBOOK.md`.
3) Keep changes minimal and scoped; avoid unrelated refactors.
4) Use the allowlist/export process for public-release artifacts.

## Local checks
Run the smallest relevant checks for your change. Examples:
- `python3 tools/goal_lint.py` (requires a prepared PR event payload)
- `./scripts/ci/metadata_conformance_smoke.sh`

## Code of conduct
If a code of conduct is adopted, it will be listed in the repository root.
