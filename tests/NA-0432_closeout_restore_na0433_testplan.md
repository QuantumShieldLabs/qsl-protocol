Goals: G1, G2, G3, G4, G5

# NA-0432 Closeout / Restore NA-0433 Testplan

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-06-06

## Scope

This testplan validates the closeout-only governance patch that marks NA-0432
DONE and restores `NA-0433 -- QSL qsc Provider Error Path / No-Mutation
Findings Triage Authorization Plan` as the sole READY successor.

Allowed mutation paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0432_closeout_restore_na0433_testplan.md`

Forbidden mutation paths include runtime, crypto, dependency, Cargo, lockfile,
workflow, script, executable test, fuzz target, vector, qsl-server,
qsl-attachments, qshield runtime, website, public docs, README, START_HERE,
qwork/qstart/qresume/qshell, backup, qsl-backup, backup status, backup plan,
rollback, and `/backup/qsl` paths.

## Preconditions

- PR #1134 is merged.
- Post-merge public-safety is green on `1e8036473f6c`.
- Post-merge `qsc-adversarial-smoke` is green on `1e8036473f6c`.
- NA-0432 is READY before closeout.
- D-0852 exists once before closeout.
- D-0853 is absent before closeout.

## Queue and Decision Proof

```bash
python3 scripts/ci/qsl_evidence_helper.py queue \
  --select NA-0433 --select NA-0432 --select NA-0431 \
  --select NA-0430 --select NA-0429

python3 scripts/ci/qsl_evidence_helper.py decisions \
  --select D-0852 --select D-0853 --select D-0854
```

Expected:

- `READY_COUNT 1`
- `READY NA-0433`
- `NA-0432 DONE`
- `NA-0431 DONE`
- `NA-0430 DONE`
- `NA-0429 BLOCKED`
- `D-0852 1`
- `D-0853 1`
- `D-0854 0`
- `DUPLICATE_COUNT 0`

## Scope Guard

```bash
python3 scripts/ci/qsl_evidence_helper.py scope-guard \
  --base origin/main --head HEAD \
  --allowed NEXT_ACTIONS.md \
  --allowed DECISIONS.md \
  --allowed TRACEABILITY.md \
  --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md \
  --allowed tests/NA-0432_closeout_restore_na0433_testplan.md
```

Expected:

- `CHANGED_PATH_COUNT 5`
- each path is allowed
- `FORBIDDEN_COUNT 0`

## Hygiene Checks

```bash
git diff --check origin/main..HEAD
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
python3 scripts/ci/qsl_evidence_helper.py pr-body-preflight \
  --file <closeout-pr-body> --scan-overclaims
scripts/audit/run_goal_lint_pr.sh <PR_NUMBER>
```

Expected:

- no diff whitespace errors
- `TOTAL_MISSING 0`
- `SECRET_FINDING_COUNT 0`
- `MISSING_FIELD_COUNT 0`
- `PROHIBITED_PHRASE_COUNT 0`
- goal-lint passes

## Dependency and Public-Safety Checks

```bash
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
python3 scripts/ci/qsl_evidence_helper.py checks-summary \
  --pr <PR_NUMBER> --report-only --allow-codeql-neutral
python3 scripts/ci/qsl_evidence_helper.py public-safety-status \
  --sha <merge-sha> --report-only
```

Expected:

- root cargo audit passes
- nested qsc fuzz lock audit passes
- PR public-safety passes before merge
- post-merge public-safety passes before future work
- CodeQL neutral may be accepted only under the helper's explicit
  `--allow-codeql-neutral` classifier

## Public Claim Boundary

The closeout is internal governance only. It is not production readiness. It is
not public-internet readiness. It is not external-review completion. It is not
crypto completeness. It is not side-channel freedom. It is not bug-free status.
It is not vulnerability-free status. It is not perfect-crypto proof. Cargo
audit green remains dependency-health evidence only.

## Acceptance Criteria

- NA-0432 is DONE.
- NA-0433 is the sole READY item.
- D-0853 exists once.
- D-0854 remains absent.
- No runtime, crypto, dependency, test, vector, workflow, service, public,
  backup, restore, qwork, qsl-backup, status, plan, rollback, README,
  START_HERE, website, or public-claim mutation occurs.
- Public-safety is green before merge and after merge.
