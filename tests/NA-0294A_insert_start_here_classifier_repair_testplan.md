Goals: G1, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-15

# NA-0294A Insert START_HERE Classifier Repair Test Plan

## Objective

Insert `NA-0294A - START_HERE Docs-Only Classifier Repair` ahead of `NA-0294` so the root-doc classifier mismatch can be repaired before public README/START_HERE navigation work resumes.

## Protected Invariants

- Exactly one READY item exists after the insertion.
- `NA-0294A` is the sole READY item.
- `NA-0294` remains preserved as the successor after NA-0294A closeout.
- D-0561 exists once and D-0562 remains absent.
- The insertion does not alter classifier behavior, workflows, branch protection, public-safety configuration, Cargo/dependencies, protocol/runtime/crypto/service paths, website/external website files, README, START_HERE, or docs/public content.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0294A_insert_start_here_classifier_repair_testplan.md`

## Forbidden Scope

- `scripts/ci/**`
- `.github/**`
- `README.md`
- `START_HERE.md`
- `docs/public/**`
- `Cargo.toml`
- `Cargo.lock`
- `qsp/**`
- `qsc/**`
- `qsl/**`
- `qsl-client/**`
- `apps/**`
- `tools/**`
- `inputs/**`
- `formal/**`
- `qsl-server/**`
- `qsl-attachments/**`
- `qsc-desktop/**`
- `website/**`

## Local Validation Commands

```bash
git diff --check origin/main...HEAD
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main \
  --allowed NEXT_ACTIONS.md \
  --allowed DECISIONS.md \
  --allowed TRACEABILITY.md \
  --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md \
  --allowed tests/NA-0294A_insert_start_here_classifier_repair_testplan.md
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
bash scripts/ci/classify_ci_scope.sh START_HERE.md
bash scripts/ci/classify_ci_scope.sh README.md Cargo.toml
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/run_model_checks.py
```

Goal-lint must run through the repo-local PR workflow or a synthetic pull-request event after the branch is committed.

## CI Expectations

- Required checks attach and complete successfully.
- `public-safety` remains required and green.
- Docs/governance-only cost control may skip heavy suites where the existing classifier permits it, but the insertion must not weaken classifier or public-safety behavior.

## Successor Handoff

After this insertion merges and post-merge public-safety is green, implement `NA-0294A` classifier repair. Do not start `NA-0294` public navigation work until NA-0294A repair and closeout are complete.
