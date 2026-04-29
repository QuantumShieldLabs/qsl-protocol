Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-28

# NA-0237A Public-Safety Red-Main Admission Testplan

## Objective

Add a fail-closed public-safety admission for the active `NA-0237A` red-main repair without merging PR `#721`, bypassing branch protection, or weakening public-safety for unrelated PRs.

## Protected Invariant

- Public-safety remains required and fail-closed.
- Ordinary PRs stay blocked when latest `main` is red.
- Advisory remediation and workflow-only self-repair bootstrap behavior remain bounded.
- PR `#708` and KT paths remain untouched until `NA-0237A` closes.

## Positive Proof

- `check-main-public-safety` with `--allow-main-public-safety-remediation-pr 721`, the exact PR `#721` head SHA, and both expected main-failure markers admits PR `#721` only when:
  - latest `main` public-safety is completed/failure
  - latest `main` advisories are completed/success
  - latest `main` macOS qsc full serial logs contain `send_commit` and `vault_mock_provider_retired`
  - origin/main queue has `NA-0237A` as sole READY
  - PR `#721` changes only the bounded NA-0237A repair/evidence paths
  - all required PR `#721` checks except public-safety are accepted
  - PR `#708` remains open at the expected head

## Negative Proofs

- Wrong PR number fails.
- Wrong head SHA fails.
- Missing expected failure marker fails.
- Unrelated changed path fails through the PR changed-path allowlist.
- Main advisories red fails the NA-0237A remediation admission.
- PR `#708` moved or KT path changes fail closed.
- More than one READY item or a READY item other than `NA-0237A` fails closed.

## Local Validation Commands

```bash
python3 -m py_compile scripts/ci/public_safety_gate.py
python3 - <<'PY'
import yaml
from pathlib import Path
yaml.safe_load(Path('.github/workflows/public-ci.yml').read_text())
PY
GH_TOKEN="$(gh auth token)" python3 scripts/ci/public_safety_gate.py check-main-public-safety --repo QuantumShieldLabs/qsl-protocol --allow-main-public-safety-remediation-pr 721 --expected-remediation-pr-sha 711d78a2c949e155bdedd6ef543edc4706029aa3 --expected-main-failure-marker send_commit --expected-main-failure-marker vault_mock_provider_retired
```

## Required CI Expectations

The helper PR must pass the normal required contexts, including `public-safety`, without admin bypass. PR `#721` must still pass `public-safety` on its own refreshed head before merge.

## Non-Bypass Statement

This helper does not merge PR `#721`, does not mark PR `#721` safe by assertion, and does not bypass public-safety. It only lets public-safety evaluate the active NA-0237A repair when the exact evidence chain is present.
