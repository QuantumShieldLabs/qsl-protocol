Goals: G4

Status: Archive
Owner: QSL governance
Last-Updated: 2026-04-27
Replaces: n/a
Superseded-By: n/a

# NA-0237D Self-Repair Bootstrap Evidence

## Summary

`NA-0237D` is complete from already-merged implementation state.

Merged PR: `#717`
Merge commit SHA: `cbf812a33ff0`
Prior main SHA: `750947d55e2c`
Final PR head SHA: `1e3a8c6a12a4`
Merged at: `2026-04-28T03:56:23Z`

## Merged-State Proof

- PR `#717` is `MERGED` on refreshed GitHub truth.
- The merge commit is `cbf812a33ff0`.
- Parent 1 is prior `main` `750947d55e2c`.
- Parent 2 is final PR head `1e3a8c6a12a4`.
- The merged result is a normal merge commit, not a squash or rebase.

## Exact Implementation Outcome On Main

Refreshed `main` contains exactly the expected `NA-0237D` bootstrap-repair paths:

- `.github/workflows/public-ci.yml`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `scripts/ci/public_safety_gate.py`
- `tests/NA-0237D_public_safety_self_repair_bootstrap_testplan.md`

This merged implementation/evidence outcome remains in-scope:

- The repaired `public-ci` / `public_safety_gate.py` seam keeps `public-safety` as the unchanged required protected context.
- The bootstrap path stays limited to the sanctioned workflow-only self-repair surface.
- Dependency/runtime PRs still fail closed on live advisories or red latest-main health.
- No additional runtime, manifest, lockfile, qsl-server, qsl-attachments, website/public-runtime, or GitHub-settings changes were part of the merged `NA-0237D` result.

## PR #715 Re-Evaluation Proof

Before `#717` merged, PR `#715` was blocked because its workflow-only repair branch could not satisfy its own `advisories` / `public-safety` path while latest `main` was still vulnerable.

After `#717` merged, PR `#715` was re-evaluated on the same unchanged head:

- PR `#715` head before rerun: `019e0385a5a9`
- PR `#715` head after rerun: `019e0385a5a9`
- Fresh PR-side `public-ci` run: `25033084702`
- Fresh `classify-public-ci-scope` job: `73318789030`
- Fresh `advisories` job: `73318798486`
- Fresh `public-safety` job: `73319045120`

This proves the old workflow-self-repair bootstrap deadlock is gone:

- A new PR-side suite attached to the unchanged `#715` head after the merged bootstrap rule was present on `main`.
- The lane no longer fails because the workflow-only repair cannot be evaluated at all.
- `#715` is now evaluable on its own merits.

## Current Post-Deadlock Failure Basis

The fresh PR-side suite on `#715` is still red, but for branch-local reasons rather than bootstrap deadlock:

- `advisories` still fails on `RUSTSEC-2026-0104` for `rustls-webpki 0.103.12`.
- `public-safety` then fails at `Require advisories success`.

The remaining truthful work therefore returns to `NA-0237C`: resume or supersede the bounded workflow/script recursion-repair lane from refreshed `main` without widening scope.

## Closeout Scope

This closeout PR is governance-only and introduces no workflow, runtime, dependency, or settings mutations. It archives durable merged evidence, marks `NA-0237D` `DONE`, records that the old bootstrap deadlock is cleared on `main`, and restores `NA-0237C` as the next truthful sole READY item.
