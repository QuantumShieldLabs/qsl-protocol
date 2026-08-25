# Contributing

## Scope
This repository is under active development and is governed by a directive workflow: work lands
through authorized lanes, not through unsolicited implementation PRs. The contributions that are
most useful from outside, and that need no prior authorization to propose, are:
- Negative tests and reproduction notes
- Vector/test artifacts
- Claim-boundary review — anywhere the documentation says more than the code earns
- Documentation and governance corrections

Protocol or implementation changes are directed through the lane workflow; open an issue first
rather than a PR, and expect the change to be scheduled as a lane.

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
