# NA-0262A Public-Safety Full-Suite Cost-Control Test Plan

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-10

## Objective

Validate that docs/governance-only main pushes skip unnecessary Linux/macOS full-suite work while runtime/security/Cargo/workflow/code changes still require full suites and public-safety remains fail-closed.

## Protected Invariants

- `public-safety` remains required.
- Branch protection is unchanged.
- Docs/governance-only acceleration is path-classified.
- Empty, ambiguous, unknown, mixed, runtime, workflow, scripts/ci, Cargo, app, qsl-server, qsl-attachments, and qsc-desktop paths require full suites.
- No protocol/runtime/crypto/demo/service behavior changes occur.
- No qsl-server, qsl-attachments, qsc-desktop, website, external website, Cargo, or branch-protection changes occur.

## Scope Guard

Allowed changed paths:

- `.github/workflows/public-ci.yml`
- `.github/workflows/ci.yml`
- `.github/workflows/macos-build.yml`
- `scripts/ci/public_safety_gate.py`
- `docs/governance/evidence/NA-0262A_public_safety_full_suite_cost_control_audit.md`
- `tests/NA-0262A_public_safety_full_suite_cost_control_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden path classes include protocol/runtime/crypto/demo/service implementation paths, qsl-server, qsl-attachments, qsc-desktop, website, external website source, Cargo manifests/locks, branch-protection settings, and public-safety required-check removal.

Expected guard command:

```bash
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main \
  --allowed .github/workflows/public-ci.yml \
  --allowed .github/workflows/ci.yml \
  --allowed .github/workflows/macos-build.yml \
  --allowed scripts/ci/public_safety_gate.py \
  --allowed docs/governance/evidence/NA-0262A_public_safety_full_suite_cost_control_audit.md \
  --allowed tests/NA-0262A_public_safety_full_suite_cost_control_testplan.md \
  --allowed DECISIONS.md \
  --allowed TRACEABILITY.md \
  --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md
```

## Self-Test Proof

Run:

```bash
python3 scripts/ci/public_safety_gate.py selftest-full-suite-cost-control
```

Required passing fixtures:

- docs/governance-only closeout paths produce `docs_only` and skip full-suite wait/jobs.
- qsc runtime path requires full-suite wait/jobs.
- `apps/qshield-cli` path requires full-suite wait/jobs.
- `scripts/ci` path requires full-suite wait/jobs.
- `.github/workflows` path requires full-suite wait/jobs.
- `Cargo.toml` and `Cargo.lock` require full-suite wait/jobs.
- `qsl-server` path requires full-suite wait/jobs.
- `qsl-attachments` path requires full-suite wait/jobs.
- `qsc-desktop` path requires full-suite wait/jobs.
- mixed docs plus runtime paths require full-suite wait/jobs.
- unknown paths require full-suite wait/jobs.
- empty/ambiguous push scope requires full-suite wait/jobs.

## Workflow Parse Proof

Run a YAML load over changed workflow files:

```bash
python3 - <<'PY'
import pathlib, yaml
for path in [
    ".github/workflows/public-ci.yml",
    ".github/workflows/ci.yml",
    ".github/workflows/macos-build.yml",
]:
    yaml.safe_load(pathlib.Path(path).read_text())
    print(f"YAML_OK {path}")
PY
```

Expected:

- all changed workflows parse.

## Local Validation Bundle

Run:

```bash
git diff --check
python3 -m py_compile scripts/ci/public_safety_gate.py
python3 scripts/ci/public_safety_gate.py selftest-full-suite-cost-control
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --allowed .github/workflows/public-ci.yml --allowed .github/workflows/ci.yml --allowed .github/workflows/macos-build.yml --allowed scripts/ci/public_safety_gate.py --allowed docs/governance/evidence/NA-0262A_public_safety_full_suite_cost_control_audit.md --allowed tests/NA-0262A_public_safety_full_suite_cost_control_testplan.md --allowed DECISIONS.md --allowed TRACEABILITY.md --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
goal-lint
```

Expected:

- queue remains `READY_COUNT 1`, READY `NA-0262A`.
- D-0492 exists once and D-0493 is absent.
- no duplicate decision IDs exist.
- no forbidden scope appears.
- public-safety required checks remain unchanged.

## Post-Merge Proof

After merge:

- public-safety must be green on main.
- D-0492 must exist on main.
- READY must remain `NA-0262A`.
- cost-control behavior must be present on main.
- local smoke/selftest must show docs-only skip and runtime fail-closed behavior.
