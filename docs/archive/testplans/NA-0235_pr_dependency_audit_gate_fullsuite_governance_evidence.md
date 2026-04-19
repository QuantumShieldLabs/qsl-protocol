Goals: G4

Status: Archive
Owner: QSL governance
Last-Updated: 2026-04-19
Replaces: n/a
Superseded-By: n/a

# NA-0235 PR Dependency-Audit Gate + Full-Suite Governance Evidence

## Summary

`NA-0235` is complete from already-merged implementation state.

Merged PR: `#695`
Merge commit SHA: `f071bdae0c6a`
Prior main SHA: `569d21cfcb19`
Final PR head SHA: `6c0e3385d861`

## Merged-State Proof

- PR `#695` is `MERGED` on refreshed GitHub truth.
- The merge commit is `f071bdae0c6a4bf4f0d3a4be2331e9d5455840cb`.
- Parent 1 is prior `main` `569d21cfcb19673b43ff38ae968391b44f3efa68`.
- Parent 2 is final PR head `6c0e3385d8611181cbe858edbf87a6552d5d0a11`.
- The merged result is a normal merge commit, not a squash or rebase.

## Exact Implementation Outcome On Main

Refreshed `main` contains exactly the six expected `NA-0235` paths:

- `.github/workflows/public-ci.yml`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `scripts/ci/public_safety_gate.py`
- `tests/NA-0235_rolling_journal_entry_testplan.md`

This merged implementation/evidence outcome remains in-scope:

- The runtime-free workflow/governance repair is present on refreshed `main`.
- No additional runtime, manifest, lockfile, qsl-server, qsl-attachments, qsc-desktop, or website/public-runtime paths were part of the merged `NA-0235` change set.

## Post-Incident Branch-Protection Verification

Refreshed branch-protection truth after the manual GitHub UI remove/re-add of `public-safety` shows:

- `public-safety` is still in the required protected set.
- `public-safety` is still associated with GitHub Actions `app_id 15368`.
- The broader required protected set remains intact.
- `required_approving_review_count = 0`.
- `required_conversation_resolution = false`.
- `enforce_admins = true`.

Refreshed repository-setting truth shows:

- `allow_auto_merge = true`
- `allow_merge_commit = true`
- `delete_branch_on_merge = true`
- `mergeQueue(branch:"main") = null`

## Interpretation

- There is no evidence that the manual UI action bypassed or weakened protection.
- The merge landed as a normal merge commit on protected `main`.
- The most likely effect of the manual remove/re-add of `public-safety` was refreshing a stale required-check association so GitHub could reevaluate the already-green protected check set correctly.

## Closeout Scope

This closeout PR is governance-only and introduces no runtime changes. It archives durable merged evidence, marks `NA-0235` `DONE`, records the post-incident protection verification, and promotes `NA-0236` as the next truthful sole READY item.
