Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-29

# NA-0237A Public-Safety Red-Main Admission Testplan

## Objective

Add a fail-closed public-safety admission for the active `NA-0237A` red-main repair without merging PR `#721`, weakening public-safety for unrelated PRs, or permitting any PR `#721` bypass. PR `#722` itself may use the one-time operator-approved protected-merge exception only after the branch-dispatched `public-ci` / `public-safety` proof passes on the exact final helper head and the PR-event red state is proven to be the base/default-branch `pull_request_target` bootstrap deadlock.

## Protected Invariant

- Public-safety remains required and fail-closed.
- Ordinary PRs stay blocked when latest `main` is red.
- Advisory remediation and workflow-only self-repair bootstrap behavior remain bounded.
- PR `#708` and KT paths remain untouched until `NA-0237A` closes.
- The PR `#722` protected-merge exception is one-time and helper-only; it does not authorize any blanket bypass or PR `#721` admin merge.
- PR `#721` must later pass required `public-safety` normally before it can merge.

## Positive Proof

- `check-main-public-safety` with `--allow-main-public-safety-remediation-pr 721`, the exact PR `#721` head SHA, and both expected main-failure markers admits PR `#721` only when:
  - latest `main` public-safety is completed/failure
  - latest `main` advisories are completed/success
  - latest `main` macOS qsc full serial logs contain `send_commit` and `vault_mock_provider_retired`
  - origin/main queue has `NA-0237A` as sole READY
  - PR `#721` changes only the bounded NA-0237A repair/evidence paths
  - all required PR `#721` checks except public-safety are accepted
  - PR `#708` remains open at the expected head
- Branch-dispatched `public-ci` for PR `#722` on exact head `cb9cfdba3359` passed `advisories` and `public-safety`, proving the helper behavior on the branch that contains the new logic.
- PR-event `public-safety` for PR `#722` failed because `pull_request_target` used the base/default-branch helper logic that cannot evaluate the new helper admission until PR `#722` lands.

## Negative Proofs

- Wrong PR number fails.
- Wrong head SHA fails.
- Missing expected failure marker fails.
- Unrelated changed path fails through the PR changed-path allowlist.
- Main advisories red fails the NA-0237A remediation admission.
- PR `#708` moved or KT path changes fail closed.
- More than one READY item or a READY item other than `NA-0237A` fails closed.
- The one-time PR `#722` exception fails closed if the exact helper head, branch-dispatch success, PR-event/base-workflow deadlock proof, local positive/negative helper proofs, allowed changed paths, or PR `#708` preservation proof is missing.

## Local Validation Commands

```bash
python3 -m py_compile scripts/ci/public_safety_gate.py
python3 - <<'PY'
import yaml
from pathlib import Path
yaml.safe_load(Path('.github/workflows/public-ci.yml').read_text())
PY
GH_TOKEN="$(gh auth token)" python3 scripts/ci/public_safety_gate.py check-main-public-safety --repo QuantumShieldLabs/qsl-protocol --allow-main-public-safety-remediation-pr 721 --expected-remediation-pr-sha 711d78a2c949e155bdedd6ef543edc4706029aa3 --expected-main-failure-marker send_commit --expected-main-failure-marker vault_mock_provider_retired
gh workflow run public-ci.yml --ref na-0237a-public-safety-red-main-admission -f pr_number=722
```

## Required CI Expectations

The helper PR must pass branch-dispatched `public-ci` / `public-safety` on its exact final head before any protected-merge exception. If GitHub later accepts the PR-event required contexts normally, merge PR `#722` normally. If the PR-event remains red only because of the base/default-branch `pull_request_target` bootstrap deadlock, PR `#722` may be merged only through the explicitly approved one-time helper exception. PR `#721` must still pass `public-safety` on its own refreshed head before merge.

## Non-Bypass Statement

This helper does not merge PR `#721`, does not mark PR `#721` safe by assertion, and does not bypass public-safety for PR `#721`. It only lets public-safety evaluate the active NA-0237A repair when the exact evidence chain is present. The one-time operator-approved PR `#722` protected-merge path exists only to land the helper logic that the PR-triggered base workflow cannot evaluate before it reaches `main`.
