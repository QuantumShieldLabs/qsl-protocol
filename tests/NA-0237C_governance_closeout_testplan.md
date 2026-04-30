Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-28

# NA-0237C Governance Closeout Testplan

Goals: G3, G4

## Objective

Close `NA-0237C` from already-merged PR `#715` evidence and restore `NA-0237B` as the sole READY queue item without touching PR `#713`, PR `#708`, workflows, scripts, dependencies, runtime code, protocol semantics, or branch-protection policy.

## Protected Invariant

- Protected: queue discipline and public-safety repair closeout after #715.
- Must never happen: #713 or #708 advances while NA-0237C remains READY, or public-safety is weakened to force governance forward.
- Required behavior: close NA-0237C only through explicit governance evidence, restore exactly one successor READY item, and leave runtime/dependency/KT branches untouched.

## Scope Guard

Allowed changed paths for this closeout are:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `tests/NA-0237C_governance_closeout_testplan.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden changed paths include `.github/**`, `scripts/**`, `Cargo.toml`, `Cargo.lock`, `qsp/**`, `qsc/**`, `qsl/**`, `qsl-client/**`, `apps/**`, `tools/refimpl/**`, `qsc-desktop/**`, `qsl-server/**`, `qsl-attachments/**`, `website/**`, and all runtime, protocol, crypto, demo, dependency, or public-safety implementation paths.

## Queue Parser Proof

Required post-patch proof:

```text
READY_COUNT 1
READY NA-0237B
NA-0237C DONE
```

## PR #715 Merge Evidence

- PR `#715` state must be `MERGED`.
- PR `#715` merge commit must be `2abcee236e23aba1655a2f7155f01adcf2d604cb`.
- `origin/main` must contain that merge before this closeout branch is created.

## PR #713 / #708 Unchanged Evidence

- PR `#713` must remain open at head `e4032d3906f594b9ca931bb7fe7f3e6f3db9c357`.
- PR `#708` must remain open at head `7f54ea7ab4ae7347af4655183dfb24188cf1a8ce`.
- This directive must not push to, rebase, merge, or otherwise mutate either branch.

## Local Validation Commands

```bash
git diff --name-only origin/main...HEAD
git status --porcelain=v1 --branch
git diff --check
scripts/ci/classify_ci_scope.sh NEXT_ACTIONS.md DECISIONS.md TRACEABILITY.md tests/NA-0237C_governance_closeout_testplan.md docs/ops/ROLLING_OPERATIONS_JOURNAL.md
python3 tools/goal_lint.py --event-file <synthetic-pull-request-event>
git ls-files 'tests/*.md' | wc -l
git ls-files 'tests/**/*.md' | wc -l
git ls-files 'docs/*.md' | wc -l
git ls-files 'docs/**/*.md' | wc -l
```

Also run the manual relative markdown link-integrity check from `AGENTS.md` and a deterministic queue parser over `NEXT_ACTIONS.md`.

## Required CI Context Expectations

All required contexts for the closeout PR must be present and accepted by GitHub branch protection before merge:

- `ci-4a`
- `ci-4b`
- `ci-4c`
- `ci-4d`
- `ci-4d-dur`
- `demo-cli-build`
- `demo-cli-smoke`
- `formal-scka-model`
- `goal-lint`
- `metadata-conformance-smoke`
- `suite2-vectors`
- `CodeQL`
- `macos-qsc-qshield-build`
- `public-safety`

All listed contexts must conclude `success` except CodeQL, which may be accepted as neutral only if GitHub treats it as satisfied on the validated PR head.
