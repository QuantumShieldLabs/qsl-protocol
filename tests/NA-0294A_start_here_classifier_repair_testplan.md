Goals: G1, G3, G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-15

# NA-0294A START_HERE Classifier Repair Test Plan

## Objective

Repair CI classifier/root-doc policy so `START_HERE.md` is treated as docs/front-door governance scope while ambiguous, runtime, workflow, script, Cargo, qsp/qsc/qsl, app/tool/input/formal, service, desktop, website, and mixed docs+runtime paths remain non-docs-only.

## Protected Invariants

- `START_HERE.md` classifies docs-only.
- README remains docs-only.
- `docs/public/**` remains docs-only.
- Governance docs and testplan markdown remain docs-only.
- Empty classifier input remains runtime-critical.
- Runtime, workflow/script, Cargo, dependency, qsp/qsc/qsl, qsl-client, app/tool/input/formal, service, desktop, website, unknown, and mixed docs+runtime paths remain non-docs-only.
- Branch protection and public-safety configuration remain unchanged.

## Allowed Scope

- `scripts/ci/classify_ci_scope.sh`
- `scripts/ci/public_safety_gate.py`
- `scripts/ci/qsl_evidence_helper.py`
- `docs/governance/evidence/NA-0294A_start_here_classifier_repair.md`
- `tests/NA-0294A_start_here_classifier_repair_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

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
- branch-protection or public-safety configuration

## Positive Classifier Cases

```bash
bash scripts/ci/classify_ci_scope.sh START_HERE.md
bash scripts/ci/classify_ci_scope.sh README.md START_HERE.md docs/public/INDEX.md docs/public/RELEASE_READINESS_EVIDENCE_MAP.md docs/public/EXTERNAL_REVIEW_PACKAGE.md docs/governance/evidence/NA-0294_public_evidence_navigation_refresh_audit.md tests/NA-0294_public_evidence_navigation_refresh_testplan.md DECISIONS.md TRACEABILITY.md docs/ops/ROLLING_OPERATIONS_JOURNAL.md
python3 scripts/ci/public_safety_gate.py selftest-full-suite-cost-control
```

Expected result: docs-only classification.

## Negative Classifier Cases

```bash
bash scripts/ci/classify_ci_scope.sh
bash scripts/ci/classify_ci_scope.sh Cargo.toml
bash scripts/ci/classify_ci_scope.sh Cargo.lock
bash scripts/ci/classify_ci_scope.sh scripts/ci/classify_ci_scope.sh
bash scripts/ci/classify_ci_scope.sh .github/workflows/public-ci.yml
bash scripts/ci/classify_ci_scope.sh qsp/example
bash scripts/ci/classify_ci_scope.sh qsc/example
bash scripts/ci/classify_ci_scope.sh qsl/example
bash scripts/ci/classify_ci_scope.sh qsl-client/example
bash scripts/ci/classify_ci_scope.sh apps/example
bash scripts/ci/classify_ci_scope.sh tools/example
bash scripts/ci/classify_ci_scope.sh inputs/example
bash scripts/ci/classify_ci_scope.sh formal/example
bash scripts/ci/classify_ci_scope.sh qsl-server/example
bash scripts/ci/classify_ci_scope.sh qsl-attachments/example
bash scripts/ci/classify_ci_scope.sh qsc-desktop/example
bash scripts/ci/classify_ci_scope.sh website/example
```

Expected result: non-docs-only classification.

## Mixed-Path Fail-Closed Cases

```bash
bash scripts/ci/classify_ci_scope.sh README.md Cargo.toml
bash scripts/ci/classify_ci_scope.sh START_HERE.md qsl/example
```

Expected result: non-docs-only runtime-critical classification.

## Selftest and Local Validation Commands

```bash
bash -n scripts/ci/classify_ci_scope.sh
python3 -m py_compile scripts/ci/public_safety_gate.py scripts/ci/qsl_evidence_helper.py
python3 scripts/ci/public_safety_gate.py selftest-full-suite-cost-control
python3 scripts/ci/public_safety_gate.py selftest-advisories-resilience
git diff --check origin/main...HEAD
python3 scripts/ci/qsl_evidence_helper.py queue --select NA-0294A --select NA-0294
python3 scripts/ci/qsl_evidence_helper.py decisions --select D-0561 --select D-0562 --select D-0563
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main \
  --allowed scripts/ci/classify_ci_scope.sh \
  --allowed scripts/ci/public_safety_gate.py \
  --allowed scripts/ci/qsl_evidence_helper.py \
  --allowed docs/governance/evidence/NA-0294A_start_here_classifier_repair.md \
  --allowed tests/NA-0294A_start_here_classifier_repair_testplan.md \
  --allowed DECISIONS.md \
  --allowed TRACEABILITY.md \
  --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/run_model_checks.py
```

Goal-lint must run through the repo-local PR workflow or a synthetic pull-request event after the branch is committed.

## CI Expectations

- Required checks attach and complete successfully.
- `public-safety` remains required and green.
- No workflow, branch-protection, public-safety configuration, Cargo, dependency, runtime, service, protocol, crypto, website, README, START_HERE content, or docs/public content changes are introduced.

## Successor Handoff

After this repair merges and post-merge public-safety is green, close out NA-0294A and restore NA-0294 as READY. Do not implement NA-0294 in the classifier repair PR.
