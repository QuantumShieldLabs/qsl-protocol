Goals: G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-11
Replaces: n/a
Superseded-By: n/a

# NA-0267 CI Advisories Fetch Resilience Testplan

## Objective

Validate bounded cargo-audit retry/classification behavior for transient
external RustSec advisory database fetch failures while preserving fail-closed
handling for real advisories, warnings, unsupported output, malformed logs, and
unknown local/tool failures.

## Protected Invariants

- Real vulnerabilities fail closed.
- cargo-audit warnings fail closed.
- Unknown cargo-audit failures fail closed.
- Unsupported or malformed logs fail closed.
- Transient advisory database fetch failures are retried only within a bounded
  budget.
- A transient classification does not create a green result unless a later
  cargo-audit attempt succeeds.
- `public-safety` remains required.
- Branch protection is unchanged.
- No Cargo dependency files change.
- No protocol/runtime/crypto/demo implementation path changes.

## Allowed Scope

- `.github/workflows/public-ci.yml`
- `scripts/ci/public_safety_gate.py`
- `docs/governance/evidence/NA-0267_ci_advisories_fetch_resilience_audit.md`
- `tests/NA-0267_ci_advisories_fetch_resilience_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

## Forbidden Scope

- `Cargo.toml`
- `Cargo.lock`
- protocol/runtime/crypto/demo implementation paths
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
- branch-protection settings
- production service implementation

## Required Proof

Hard-start:

```bash
git status --porcelain=v1 --branch
git diff --name-only || true
git ls-files --others --exclude-standard || true
git fetch --all --prune
git rev-parse origin/main
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
gh api /repos/QuantumShieldLabs/qsl-protocol/branches/main/protection/required_status_checks
python3 scripts/ci/public_safety_gate.py selftest-full-suite-cost-control
```

Implementation validation:

```bash
git status --porcelain=v1 --branch
git diff --name-only origin/main...HEAD
git diff --stat origin/main...HEAD
git diff --check
python3 -m py_compile scripts/ci/public_safety_gate.py
python3 -m py_compile scripts/ci/qsl_evidence_helper.py
python3 scripts/ci/public_safety_gate.py selftest-advisories-resilience
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/run_model_checks.py
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
```

## Fixture Expectations

`python3 scripts/ci/public_safety_gate.py selftest-advisories-resilience` must
prove:

- clean success fixture classifies as `clean_success`;
- transient RustSec advisory database IO/fetch failure classifies as
  `transient_fetch`;
- real advisory fixture classifies as `real_finding`;
- warning advisory fixture classifies as `real_finding`;
- unknown lockfile/tool failure fixture classifies as `unknown_failure`;
- mixed transient and real-advisory text classifies as `real_finding`.

## CI Expectations

- Required PR checks pass normally.
- CodeQL neutral is acceptable only under the repository's existing acceptance
  basis.
- `public-safety` remains required and green before merge and after merge.
- Workflow/script changes are workflow-security scope, so full-suite waits/jobs
  are expected.
