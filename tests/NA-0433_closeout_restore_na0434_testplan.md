Status: Supporting
Owner: QSL governance
Last-Updated: 2026-06-06

# NA-0433 Closeout and NA-0434 Restoration Testplan

Goals: G1, G2, G3, G4, G5

## Purpose

Verify that NA-0433 is closed only after its evidence PR merged and post-merge
public-safety completed success, and that NA-0434 is restored as the sole READY
successor without implementing NA-0434.

## Scope

Allowed closeout mutation paths:

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0433_closeout_restore_na0434_testplan.md`

Forbidden closeout mutation paths include runtime, crypto, dependency, Cargo,
lockfile, workflow, script, executable test, fuzz target, vector, qsl-server,
qsl-attachments, qshield runtime, website, public docs, README, START_HERE,
qwork/qstart/qresume/qshell, qsl-backup, backup status, backup plan, rollback
subtree, and `/backup/qsl` paths.

## Preconditions

- PR #1136 is MERGED.
- PR #1136 merge commit is `55383fa9a953`.
- Post-merge public-safety completed success on `55383fa9a953`.
- Post-merge qsc-adversarial-smoke completed success on `55383fa9a953`.
- Queue before closeout reports READY_COUNT 1 and READY NA-0433.
- Decisions before closeout report D-0854 once, D-0855 absent, and duplicate
  count zero.

## Expected Queue Outcome

- NA-0433 is DONE.
- NA-0434 is READY.
- READY_COUNT remains 1.
- NA-0432 remains DONE.
- NA-0431 remains DONE.
- NA-0430 remains DONE.
- NA-0429 remains BLOCKED.

## Expected Decision Outcome

- D-0855 exists once.
- D-0854 remains once.
- D-0853 remains once.
- D-0856 remains absent until future NA-0434 work.
- Duplicate decision count remains zero.

## Restored NA-0434 Scope

Selected successor:

`NA-0434 -- QSL qsc Provider Error Path / No-Mutation Test Implementation Harness`

Exact future qsc test path:

`qsl/qsl-client/qsc/tests/handshake_provider_error_no_mutation.rs`

Future NA-0434 may add or modify only that exact test file plus:

- `docs/governance/evidence/NA-0434_qsl_qsc_provider_error_path_no_mutation_test_implementation_harness.md`
- `tests/NA-0434_qsl_qsc_provider_error_path_no_mutation_test_implementation_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

NA-0434 must stop if exact tests require runtime, crypto, dependency, Cargo,
lockfile, workflow, fuzz target, vector, service, backup, qwork-tool, or public
surface changes.

## Public Claim Boundary

This closeout is internal governance evidence only. It is not production
readiness, public-internet readiness, external-review completion,
crypto-complete proof, side-channel-free proof, bug-free proof,
vulnerability-free proof, perfect-crypto proof, backup completion,
restore proof, off-host backup completion, disaster recovery completion,
metadata-free behavior, anonymity, or untraceability evidence.

Cargo audit output remains dependency-health evidence only.

## Validation Commands

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py queue --select NA-0434 --select NA-0433 --select NA-0432 --select NA-0431 --select NA-0430 --select NA-0429
python3 scripts/ci/qsl_evidence_helper.py decisions --select D-0853 --select D-0854 --select D-0855 --select D-0856
python3 scripts/ci/qsl_evidence_helper.py link-check --root .
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main --paths \
  NEXT_ACTIONS.md \
  DECISIONS.md \
  TRACEABILITY.md \
  docs/ops/ROLLING_OPERATIONS_JOURNAL.md \
  tests/NA-0433_closeout_restore_na0434_testplan.md
```

Scope guard:

```bash
python3 scripts/ci/qsl_evidence_helper.py scope-guard \
  --base origin/main \
  --head HEAD \
  --allowed NEXT_ACTIONS.md \
  --allowed DECISIONS.md \
  --allowed TRACEABILITY.md \
  --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md \
  --allowed tests/NA-0433_closeout_restore_na0434_testplan.md
```

Classifier:

```bash
bash scripts/ci/classify_ci_scope.sh \
  NEXT_ACTIONS.md \
  DECISIONS.md \
  TRACEABILITY.md \
  docs/ops/ROLLING_OPERATIONS_JOURNAL.md \
  tests/NA-0433_closeout_restore_na0434_testplan.md
```

PR body preflight and goal-lint must pass with a standalone line:

```md
Goals: G1, G2, G3, G4, G5
```

Dependency and health validation:

```bash
cargo audit --deny warnings
cargo audit --deny warnings --file qsl/qsl-client/qsc/fuzz/Cargo.lock
cargo tree -i rustls-webpki --locked
cargo tree -i ml-kem --locked || true
cargo tree -i pqcrypto-mlkem --locked || true
cargo tree -i pqcrypto-traits --locked || true
cargo tree -i pqcrypto-internals --locked || true
cargo fmt --check
```

## Required Markers

- `NA0433_CLOSEOUT_PR1136_MERGED_OK`
- `NA0433_CLOSEOUT_POST_MERGE_PUBLIC_SAFETY_OK`
- `NA0433_CLOSEOUT_POST_MERGE_QSC_ADVERSARIAL_OK`
- `NA0433_DONE_OK`
- `NA0434_READY_OK`
- `NA0434_EXACT_TEST_PATH_RESTORED_OK`
- `NA0434_NO_RUNTIME_CHANGE_OK`
- `NA0434_NO_CRYPTO_CHANGE_OK`
- `NA0434_NO_DEPENDENCY_CHANGE_OK`
- `NA0434_NO_WORKFLOW_CHANGE_OK`
- `NA0434_NO_PUBLIC_OVERCLAIM_OK`
- `NA0434_NO_BACKUP_RESTORE_OK`
- `NA0433_D0855_DECISION_OK`
- `NA0433_ONE_READY_INVARIANT_OK`

## Acceptance Criteria

- PR #1136 merge and post-merge public-safety are recorded.
- NA-0433 is DONE.
- NA-0434 is READY and includes the exact authorized test path.
- D-0855 exists once.
- D-0856 is absent.
- No NA-0434 implementation occurs in this closeout.
- Changed paths are limited to the five allowed closeout paths.
- public-safety is green before merge and after merge.
- exactly one READY item remains.
