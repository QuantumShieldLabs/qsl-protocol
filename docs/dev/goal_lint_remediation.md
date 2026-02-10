# Goal-Lint Remediation: Stale PR Body Event

## Symptom

`goal-lint` fails after editing the PR body to add or fix the `Goals:` line.

## Why This Happens

The workflow rerun can read an older `pull_request` event payload where the body does not yet include the latest edit.

## Deterministic Remediation

1. Confirm the PR body has a top-level line like `Goals: G2, G5`.
2. Close the PR.
3. Reopen the PR.
4. Wait for a new `pull_request` event run (new run id / timestamp).
5. Re-check `goal-lint` and merge only after required checks are green.

## Notes

- Do not push new commits for this remediation.
- Do not bypass protections.
- Keep merge strategy consistent with repository policy.
