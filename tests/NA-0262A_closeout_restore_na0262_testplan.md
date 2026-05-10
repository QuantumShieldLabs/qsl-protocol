# NA-0262A Closeout and NA-0262 Restoration Test Plan

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-10

## Objective

Close NA-0262A after the public-safety full-suite cost-control implementation merged and post-merge public-safety completed green, then restore NA-0262 as the sole READY successor for demo adversarial stress, chaos, and abuse testing.

## Protected Invariants

- Exactly one READY item exists after closeout.
- NA-0262A is DONE.
- NA-0262 is READY.
- D-0492 remains present exactly once.
- D-0493 is added exactly once.
- Public-safety remains required and green.
- Branch protection is unchanged.
- Docs/governance-only acceleration remains path-classified and fail-closed.
- Runtime, security, Cargo, workflow, scripts/ci, app, qsl-server, qsl-attachments, qsc-desktop, mixed, unknown, and ambiguous changes still require Linux and macOS full suites.
- No protocol, runtime, crypto, demo implementation, qsl-server, qsl-attachments, qsc-desktop, website, workflow, Cargo, branch-protection, or public-safety configuration change is made in this closeout.

## Allowed Scope

- `NEXT_ACTIONS.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
- `tests/NA-0262A_closeout_restore_na0262_testplan.md`

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
- `READY NA-0262 Demo Adversarial Stress, Chaos, and Abuse Testing Harness`
- `NA-0262A DONE Docs/Governance-Only public-safety Full-Suite Cost Control`

## Decision Proof

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py decisions
```

Expected:

- D-0492 exists once.
- D-0493 exists once.
- No duplicate decision IDs.

## Scope Proof

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main \
  --allowed NEXT_ACTIONS.md \
  --allowed DECISIONS.md \
  --allowed TRACEABILITY.md \
  --allowed docs/ops/ROLLING_OPERATIONS_JOURNAL.md \
  --allowed tests/NA-0262A_closeout_restore_na0262_testplan.md
```

Expected:

- Changed paths are limited to the allowed closeout files.
- No forbidden implementation, workflow, script, Cargo, branch-protection, public-safety, qsl-server, qsl-attachments, qsc-desktop, website, or external website path appears.

## Cost-Control Smoke Proof

Run:

```bash
python3 scripts/ci/public_safety_gate.py selftest-full-suite-cost-control
bash scripts/ci/classify_ci_scope.sh NEXT_ACTIONS.md DECISIONS.md TRACEABILITY.md docs/ops/ROLLING_OPERATIONS_JOURNAL.md tests/NA-0262A_closeout_restore_na0262_testplan.md
bash scripts/ci/classify_ci_scope.sh qsl/qsl-client/qsc/src/main.rs
bash scripts/ci/classify_ci_scope.sh NEXT_ACTIONS.md qsl/qsl-client/qsc/src/main.rs
bash scripts/ci/classify_ci_scope.sh
```

Expected:

- Selftest fixtures pass.
- Docs/governance-only closeout paths produce `docs_only=true`.
- Runtime path produces `runtime_critical=true`.
- Mixed docs plus runtime paths produce `runtime_critical=true`.
- Empty/ambiguous path set produces `runtime_critical=true`.

## Link / Leak / Goal-Lint Expectations

Run:

```bash
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
goal-lint
```

Expected:

- Link check passes.
- Added-line leak scan reports zero secret findings.
- Goal-lint passes with the PR body Goals line.

## Dependency / Main Health Expectations

Run:

```bash
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
```

Expected:

- Cargo audit passes.
- `rustls-webpki` remains at the patched locked version.
- `send_commit` tests pass.

## CI Expectations

- Required protected checks pass normally.
- CodeQL neutral/skipped is acceptable only under the repository's existing acceptance basis.
- Public-safety remains required on `main`.
- This docs/governance-only closeout main push should skip `qsc-linux-full-suite`, skip `macos-qsc-full-serial`, and skip only public-safety's wait for those two full-suite contexts.
- Post-merge main public-safety completes green before NA-0262 work starts.
