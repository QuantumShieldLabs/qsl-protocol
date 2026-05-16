Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-15

# NA-0294A Closeout and NA-0294 Restoration Test Plan

## Objective

Close NA-0294A after the START_HERE docs-only classifier repair and restore NA-0294 as the sole READY successor.

## Protected Invariants

- Exactly one READY item exists after the patch.
- NA-0294A is DONE.
- NA-0294 is READY.
- D-0563 exists exactly once and D-0564 is absent.
- START_HERE.md classifies as docs-only.
- The intended NA-0294 README/START_HERE/docs/public/governance/testplan bundle classifies as docs-only.
- Empty, ambiguous, runtime, workflow, script, Cargo, qsp/qsc/qsl, app/tool/input/formal, service/desktop/website, and mixed docs+runtime inputs remain non-docs-only.
- Public-safety remains required.
- Branch protection remains unchanged.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0294A_closeout_restore_na0294_testplan.md`

## Forbidden Scope

- `README.md`
- `START_HERE.md`
- `docs/public/**`
- `.github/**`
- `scripts/**`
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
- `qsc-desktop/**`
- `qsl-server/**`
- `qsl-attachments/**`
- `website/**`

## Validation Commands

```bash
git diff --name-only origin/main...HEAD
git diff --check
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
bash scripts/ci/classify_ci_scope.sh START_HERE.md
bash scripts/ci/classify_ci_scope.sh README.md START_HERE.md docs/public/INDEX.md docs/public/RELEASE_READINESS_EVIDENCE_MAP.md docs/public/EXTERNAL_REVIEW_PACKAGE.md docs/governance/evidence/NA-0294_public_evidence_navigation_refresh_audit.md tests/NA-0294_public_evidence_navigation_refresh_testplan.md DECISIONS.md TRACEABILITY.md docs/ops/ROLLING_OPERATIONS_JOURNAL.md
bash scripts/ci/classify_ci_scope.sh
bash scripts/ci/classify_ci_scope.sh Cargo.toml
bash scripts/ci/classify_ci_scope.sh README.md Cargo.toml
bash scripts/ci/classify_ci_scope.sh START_HERE.md qsl/example
bash scripts/ci/classify_ci_scope.sh scripts/ci/classify_ci_scope.sh
bash scripts/ci/classify_ci_scope.sh .github/workflows/public-ci.yml
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/run_model_checks.py
```

## Success Criteria

- The queue helper reports `READY_COUNT 1` and `READY NA-0294`.
- Decision helper reports no duplicates, latest decision D-0563, and no D-0564.
- Scope is limited to the allowed closeout paths.
- Link-check, leak-scan, dependency health, send_commit, formal/model checks, classifier proof, and required CI pass.
- No branch deletion, direct push, squash, rebase, admin bypass, public-safety weakening, or branch-protection change occurs.
