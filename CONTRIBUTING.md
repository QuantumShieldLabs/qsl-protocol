# Contributing

Thanks for your interest in contributing to QSL.

## How to propose changes
1) Open an issue to describe the change and scope.
2) Submit a focused PR that references the issue.
3) Keep changes minimal and grounded in the existing specifications.

## Documentation standards
- Prefer small, reviewable PRs.
- Cite relevant decisions or traceability entries when updating specs.
- Avoid speculative claims that cannot be supported by current artifacts.

## Prohibited content
Do not include secrets, credentials, internal endpoints, or non-public operational details.

## Local checks
Run the smallest relevant checks for your change (examples):
- `python3 tools/goal_lint.py` (requires a prepared PR event payload)
- `./scripts/ci/run_4b.sh`
