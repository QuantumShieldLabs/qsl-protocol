Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-30

# NA-0239 Public-Safety Red-Main Deadlock Prevention Testplan

Goals: G3, G4

## Objective

Prove `NA-0239` adds an executable, fail-closed public-safety admission path for a bounded non-advisory red-main repair PR, using a synthetic PR `#721` equivalent for the `send_commit` / `vault_mock_provider_retired` deadlock.

## Protected invariant

- `public-safety` remains required and fail-closed by default.
- Ordinary PRs remain blocked when latest `main` public-safety is red.
- A non-advisory red-main repair is admitted only with exact active-NA, PR head, changed-path, marker, advisory-clean, required-check, queue, and PR `#722` closed/unmerged proof.
- Advisory-remediation and workflow self-repair bootstrap behavior remain intact.
- No runtime, protocol, crypto, demo implementation, service, Cargo, qsc-desktop, qsl-server, qsl-attachments, website, PR `#722`, or PR `#708` branch change is made.

## Scope guard

Allowed changed paths for this lane:

- `.github/workflows/public-ci.yml`
- `scripts/ci/public_safety_gate.py`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0239_public_safety_red_main_deadlock_prevention_testplan.md`

Forbidden path proof must confirm no changes under Cargo files, `qsp/**`, `qsc/**`, `qsl/**`, `qsl-client/**`, `apps/**`, `tools/refimpl/**`, `tools/actors/**`, `inputs/**`, `qsc-desktop/**`, `qsl-server/**`, `qsl-attachments/**`, `website/**`, or runtime/protocol/crypto/demo/service surfaces.

## Positive synthetic proof

Run:

```bash
python3 scripts/ci/public_safety_gate.py run-na0239-fixture-proofs
```

The `positive_721_equivalent` fixture must be admitted only when:

- active READY is exactly one item
- the synthetic active READY scope includes the repair path
- the target PR number and head SHA match
- changed paths include `qsl/qsl-client/qsc/tests/send_commit.rs`
- latest main public-safety is failed
- latest main failure log includes `send_commit` and `vault_mock_provider_retired`
- latest main advisories are non-failing
- all required PR checks except public-safety are accepted
- PR `#722` remains closed and unmerged

## Negative proofs

The same fixture command must reject:

- wrong PR number
- wrong head SHA
- unrelated changed path
- missing `vault_mock_provider_retired` marker
- advisory-red main for the non-advisory repair path
- KT/PR `#708` path mismatch
- more than one READY item
- missing required PR check
- unrelated main failure
- ordinary PR while main is red with no bounded admission proof

## No-regression proofs

The same fixture command must prove:

- advisory-remediation fixture still admits dependency-remediation PRs when latest main advisories are the blocker
- advisory-remediation fixture rejects when advisories are not the blocker
- workflow self-repair fixture still admits the sanctioned workflow/helper/testplan shape
- workflow self-repair fixture rejects runtime/Cargo/service-shaped changes

## Public-safety required/green proof

Before and after this PR, branch protection must include `public-safety`, and latest `main` public-safety must be success. This lane must not change branch protection settings or require a settings exception.

## Local validation commands

```bash
python3 -m py_compile scripts/ci/public_safety_gate.py
python3 scripts/ci/public_safety_gate.py run-na0239-fixture-proofs
python3 - <<'PY'
import pathlib, yaml
yaml.safe_load(pathlib.Path('.github/workflows/public-ci.yml').read_text())
print('YAML_OK')
PY
git diff --check
```

Governance validation must also parse `NEXT_ACTIONS.md` for exactly one READY item (`NA-0239`) and parse `DECISIONS.md` for D-0439 through D-0443 exactly once with no duplicate decision IDs.

## CI expectations

Required protected contexts must pass normally:

- `ci-4a`
- `ci-4b`
- `ci-4c`
- `ci-4d`
- `ci-4d-dur`
- `demo-cli-build`
- `demo-cli-smoke`
- `formal-scka-model`
- `goal-lint`
- `metadata-conformance-smoke`
- `suite2-vectors`
- `CodeQL`
- `macos-qsc-qshield-build`
- `public-safety`

`CodeQL` may be neutral only if GitHub accepts it. `public-safety` must be success. No branch-protection exception, admin bypass, direct push, check spoofing, squash merge, or rebase merge is allowed.
