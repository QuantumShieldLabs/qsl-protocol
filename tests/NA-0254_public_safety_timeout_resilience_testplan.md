Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-07

# NA-0254 Public-Safety Timeout Resilience Test Plan

## Objective

Validate that public-safety push-suite polling tolerates bounded GitHub API/HTML timeout transients without false-greening real watched-suite failures.

## Protected Invariant

`public-safety` remains fail-closed: `qsc-linux-full-suite`, `macos-qsc-full-serial`, and `qsc-adversarial-smoke` must complete successfully before the wait path can pass.

## Scope Guard

Allowed changed paths:

- `scripts/ci/public_safety_gate.py`
- `docs/governance/evidence/NA-0254_public_safety_timeout_resilience_audit.md`
- `tests/NA-0254_public_safety_timeout_resilience_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden path classes include `.github/**`, Cargo manifests/locks, qsc/qsl/app/runtime/protocol/crypto/demo/service implementation paths, qsl-server, qsl-attachments, qsc-desktop, website, apps, tools, inputs, formal, public-safety configuration outside `scripts/ci/public_safety_gate.py`, and branch-protection settings.

Expected guard command:

```bash
python3 scripts/ci/qsl_evidence_helper.py scope-guard --base origin/main --allowed "scripts/ci/public_safety_gate.py" --allowed "DECISIONS.md" --allowed "TRACEABILITY.md" --allowed "docs/governance/evidence/NA-0254_public_safety_timeout_resilience_audit.md" --allowed "tests/NA-0254_public_safety_timeout_resilience_testplan.md" --allowed "docs/ops/ROLLING_OPERATIONS_JOURNAL.md"
```

## Transient Handling Proof

Run:

```bash
python3 scripts/ci/public_safety_gate.py selftest-timeout-resilience
```

Required passing cases:

- HTML timeout response while watched suites are pending, followed by watched-suite success.
- Non-JSON API response while watched suites are pending, followed by watched-suite success.
- HTTP 502, 503, and 504 responses while watched suites are pending, followed by watched-suite success.
- Clear rate-limit/secondary-limit response is classified as bounded transient in wait polling only.

## Real Failure Fail-Closed Proof

The self-test must include concrete failure fixtures for:

- `qsc-linux-full-suite`
- `macos-qsc-full-serial`
- `qsc-adversarial-smoke`

Each fixture must return failure immediately instead of waiting for a later success.

## Duplicate/Stale Latest-Run Proof

The self-test must prove both sides of duplicate check-run handling:

- stale failure does not override a latest successful duplicate run.
- stale success does not override a latest failed duplicate run.

## Missing/Pending Timeout Proof

The self-test must prove:

- watched suite pending beyond the bounded budget fails closed.
- watched suite missing beyond the bounded budget fails closed.

## Branch-Protection 403 Non-Bypass Proof

Generic HTTP 403 without rate-limit markers must not be classified as transient. Branch-protection required-status-check reads remain on the existing fail-closed API path and are not moved to the wait-only transient fallback.

## Queue Parser Expectation

Before Packet B closeout:

```text
READY_COUNT 1
READY NA-0254 Public-Safety Timeout-Resilient Push-Suite Polling Hardening
```

`NA-0254` remains READY after this implementation PR; `NEXT_ACTIONS.md` is not edited in Packet B.

## Decision Parser Expectation

Before Packet B:

- `D-0475` exists once.
- `D-0476` is absent.
- `D-0477` is absent.

After Packet B:

- `D-0476` exists once.
- `D-0477` is absent.
- no duplicate decision IDs exist.

## CI Expectations

Required local validation:

```bash
git diff --check
python3 -m py_compile scripts/ci/public_safety_gate.py
python3 scripts/ci/public_safety_gate.py selftest-timeout-resilience
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
python3 scripts/ci/qsl_evidence_helper.py queue
python3 scripts/ci/qsl_evidence_helper.py decisions
python3 scripts/ci/qsl_evidence_helper.py link-check
python3 scripts/ci/qsl_evidence_helper.py leak-scan --mode added --base origin/main
goal-lint
```

Required PR/main outcomes:

- protected PR checks pass normally.
- `public-safety` remains required.
- `public-safety` succeeds on the PR head and post-merge main.
- no branch-protection exception, admin bypass, check spoofing, direct push, squash merge, or rebase merge occurs.
