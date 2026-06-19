Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-19
Goals: G1, G2, G3, G4, G5

# NA-0501 Closeout and NA-0502 Restoration Testplan

## Objective

Verify that NA-0501 is closed only after the NA-0501 evidence PR merges and post-merge public-safety is green, then restore the selected NA-0502 successor as the sole READY item without implementing NA-0502.

## Scope under test

Allowed closeout changed paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0501_closeout_restore_na0502_testplan.md`

Forbidden closeout changes:

- qsc source, qsc tests, qsc fuzz, qsc Cargo files, root Cargo files, dependencies, lockfiles, workflows, scripts, helpers, corpus/vector/input files, formal models, refimpl files, services, public docs, backup paths, qsl-backup, qwork, qstart, or qresume.

## Required proof

- PR #1274 merged.
- PR #1274 merge commit is recorded.
- post-merge public-safety for the PR #1274 merge commit completed success.
- D-0991 exists once.
- D-0992 is added once.
- NA-0501 is marked DONE.
- NA-0502 is restored as the single READY item.
- NA-0502 exact allowed and forbidden scope matches D-0991.
- no NA-0502 implementation is performed by closeout.

## Validation commands

Run before the closeout PR:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
bash scripts/ci/classify_ci_scope.sh NEXT_ACTIONS.md DECISIONS.md TRACEABILITY.md docs/ops/ROLLING_OPERATIONS_JOURNAL.md tests/NA-0501_closeout_restore_na0502_testplan.md
```

Also run exact five-path closeout scope proof, added-line overclaim scan, PR body preflight, and goal-lint for the closeout PR.

## Pass criteria

- Changed paths are exactly within the five allowed closeout paths.
- READY_COUNT is 1.
- READY item is NA-0502.
- D-0991 exists exactly once.
- D-0992 exists exactly once.
- no implementation mutation is present.
- no qsc source/test/fuzz/Cargo mutation is present.
- no corpus/vector/input mutation is present.
- no workflow/script/helper/dependency/lockfile mutation is present.
- no formal/refimpl/service/public/backup mutation is present.
- no public-readiness claim is introduced.
- no production-readiness claim is introduced.
- no external-review-complete claim is introduced.
- no crypto-complete claim is introduced.
- no secret-material-complete claim is introduced.
- no zeroization-complete claim is introduced.
- no memory-erasure-complete claim is introduced.
- no side-channel-free claim is introduced.
- no vulnerability-free claim is introduced.
- no bug-free claim is introduced.
- no perfect-crypto claim is introduced.
