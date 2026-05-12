# NA-0270 Closeout and NA-0271 Restoration Test Plan

## Objective

Close NA-0270 after the qsl-server read-only audit and test-harness design PR
merged, and restore NA-0271 as the sole READY successor for qsl-attachments
read-only audit and test-harness design.

## Protected Invariants

- Exactly one READY item exists after closeout.
- NA-0270 is DONE.
- NA-0271 is READY.
- D-0510 remains present once.
- D-0511 is added once.
- public-safety remains required and green.
- No qsl-server implementation changes occur.
- No qsl-attachments implementation changes occur.
- No protocol, wire, crypto, auth, or state-machine changes occur.
- No website, workflow, script, Cargo, dependency, branch-protection, or
  public-safety configuration changes occur.
- No production readiness claim is inferred from read-only audit/design work.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0270_closeout_restore_na0271_testplan.md`

## Forbidden Scope

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
- runtime, protocol, crypto, demo, service, or deployment code
- branch-protection settings
- public-safety/check configuration

## Queue Requirements

- Before closeout PR merge: `READY_COUNT 1`, READY `NA-0270`.
- After closeout patch validation: `READY_COUNT 1`, READY `NA-0271`.
- NA-0270 records PR #793 head and merge evidence.
- NA-0271 scope is read-only audit/design only.

## Decision Requirements

- D-0510 exists once.
- D-0511 exists once.
- No duplicate decision IDs exist.
- D-0511 states that NA-0270 produced qsl-server read-only audit/test-harness
  design, NA-0271 moves to qsl-attachments read-only audit/test-harness design,
  and no production implementation is authorized.

## Validation Commands

```bash
git status --porcelain=v1 --branch
git diff --name-only origin/main...HEAD
git diff --stat origin/main...HEAD
git diff --check
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main \
  --allowed NEXT_ACTIONS.md \
  --allowed DECISIONS.md \
  --allowed TRACEABILITY.md \
  --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md \
  --allowed tests/NA-0270_closeout_restore_na0271_testplan.md \
  --forbidden '.github/**' \
  --forbidden 'scripts/**' \
  --forbidden 'Cargo.toml' \
  --forbidden 'Cargo.lock' \
  --forbidden 'qsp/**' \
  --forbidden 'qsc/**' \
  --forbidden 'qsl/**' \
  --forbidden 'qsl-client/**' \
  --forbidden 'apps/**' \
  --forbidden 'tools/**' \
  --forbidden 'inputs/**' \
  --forbidden 'formal/**' \
  --forbidden 'qsc-desktop/**' \
  --forbidden 'qsl-server/**' \
  --forbidden 'qsl-attachments/**' \
  --forbidden 'website/**'
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 formal/run_model_checks.py
```

## Overclaim Expectations

Added or changed text must not imply qsl-server or qsl-attachments production
maturity. Any mention of production boundaries must be explicit, conservative,
and tied to future gated work.

## CI Expectations

- Required PR checks complete normally.
- CodeQL neutral is acceptable only under the repository's established neutral
  allowance.
- public-safety remains required and succeeds.
- Docs/governance-only changes may skip full suites under NA-0262A cost-control
  behavior.

## Successor Handoff

The restored successor is:

- NA-0271 - qsl-attachments Read-Only Code Audit and Test-Harness Design.

NA-0271 may inventory and design tests for qsl-attachments, but it must not
change qsl-attachments implementation or claim service production maturity.
