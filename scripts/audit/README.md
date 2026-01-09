# Local Audit Helpers

Goals: G4

This directory contains *local* helper scripts to make audits reproducible and fail-closed without
requiring an arbitrary git history depth (e.g., `HEAD~80`). The primary mode is **per-PR delta**
auditing using merge-commit parent diffs.

## Policy
- Audits are **fail-closed**: if a prerequisite is missing, stop and fix prerequisites.
- Audits must run on a **clean working tree**.
- Audit artifacts must be written **outside the repo** (default: `/tmp`).

## Scripts

### 1) Run goal-lint locally for a PR
Synthesizes the minimal `GITHUB_EVENT_PATH` payload required by `tools/goal_lint.py`.

```bash
scripts/audit/run_goal_lint_pr.sh 19
```

### 2) Generate a per-PR delta audit report
Produces a markdown report for one or more PRs.

```bash
scripts/audit/audit_pr_delta.sh --pr 19 --pr 20
# Output printed to stderr:
#   Wrote audit report: /tmp/audit_<timestamp>.md
```

To write to a specific path (must be **outside** the repo root):

```bash
scripts/audit/audit_pr_delta.sh --pr 19 -o /tmp/audit_pr19.md
```

## Notes
- For PRs merged via `--merge`, a two-parent merge commit exists and the scripts will use
  `MERGE^1..MERGE` for exact deltas.
- For squash/rebase merges, the scripts fall back to `baseRefOid...headRefOid`.
