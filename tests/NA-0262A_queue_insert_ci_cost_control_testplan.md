# NA-0262A Queue Insert CI Cost-Control Test Plan

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-10

## Objective

Insert `NA-0262A — Docs/Governance-Only public-safety Full-Suite Cost Control` before `NA-0262`, without starting the demo adversarial stress implementation lane.

## Protected Invariants

- Exactly one READY item exists after the queue insertion.
- `NA-0262A` is READY.
- `NA-0262` is WAITING or otherwise explicitly deferred.
- `D-0490` remains present exactly once.
- `D-0491` is added exactly once.
- No duplicate decision IDs exist.
- `public-safety` remains required and green.
- Branch protection is unchanged.
- No public-safety, workflow, runtime, Cargo, protocol, crypto, demo, service, qsl-server, qsl-attachments, qsc-desktop, website, or external website implementation path is changed in this governance insertion.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0262A_queue_insert_ci_cost_control_testplan.md`

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
- external website source
- runtime/protocol/crypto/demo/service code
- branch-protection settings
- public-safety/check configuration

## Queue Proof

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py queue
```

Expected:

- `READY_COUNT 1`
- `READY NA-0262A Docs/Governance-Only public-safety Full-Suite Cost Control`
- `NA-0262` is not READY.

## Decision Proof

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py decisions
```

Expected:

- `D-0490` exists once.
- `D-0491` exists once.
- `D-0492` is absent.
- No duplicate decision IDs.

## Scope Proof

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main \
  --allowed NEXT_ACTIONS.md \
  --allowed DECISIONS.md \
  --allowed TRACEABILITY.md \
  --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md \
  --allowed tests/NA-0262A_queue_insert_ci_cost_control_testplan.md
```

Expected:

- Changed paths are limited to the allowed governance insertion files.
- No `.github`, scripts, Cargo, runtime, protocol, crypto, demo, service, qsl-server, qsl-attachments, qsc-desktop, website, external website, branch-protection, or public-safety configuration path appears.

## Preservation Proof

Expected final queue text:

- `NA-0262A` defines fail-closed public-safety/full-suite cost-control scope and acceptance.
- `NA-0262` remains preserved for later demo adversarial stress work.
- `NA-0262` includes the deferral note: "Deferred until NA-0262A public-safety full-suite cost-control completes."

## Local Validation Expectations

Run:

```bash
git diff --check
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --allowed NEXT_ACTIONS.md --allowed DECISIONS.md --allowed TRACEABILITY.md --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md --allowed tests/NA-0262A_queue_insert_ci_cost_control_testplan.md
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
goal-lint
```

Required PR/main outcomes:

- Required checks attach and pass normally.
- `public-safety` remains required.
- Post-merge queue proof shows exactly one READY item: `NA-0262A`.
