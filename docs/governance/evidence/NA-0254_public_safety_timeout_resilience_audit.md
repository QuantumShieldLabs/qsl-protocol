Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-07

# NA-0254 Public-Safety Timeout Resilience Audit

## Objective

Harden the public-safety push-suite polling path against bounded GitHub API and HTML timeout transients while preserving fail-closed behavior for real watched-suite failures.

## Incident Pattern

PR #754 / #755 post-merge evidence showed a public-safety wrapper failure caused by a GitHub timeout HTML response while watched push-only suites were still pending. Later evidence on the same merge commit showed `qsc-linux-full-suite`, `macos-qsc-full-serial`, and `qsc-adversarial-smoke` completed successfully. The failure mode was a transient polling transport/API response, not a concrete watched-suite failure.

## Scope

Changed path:

- `scripts/ci/public_safety_gate.py`

Evidence paths:

- `docs/governance/evidence/NA-0254_public_safety_timeout_resilience_audit.md`
- `tests/NA-0254_public_safety_timeout_resilience_testplan.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

No `.github`, branch-protection, Cargo, runtime/protocol/crypto/demo/service, qsl-server, qsl-attachments, qsc-desktop, website, apps, tools, inputs, or formal paths are changed.

## Implementation Summary

The `wait-commit-checks` subcommand now uses a wait-only GitHub check-run fetch path. That path classifies the following as bounded polling transients:

- HTML or otherwise non-JSON GitHub API responses.
- HTTP 502, 503, and 504 responses.
- urllib timeout, connection reset, remote-disconnect, and temporary network errors.
- HTTP 403 or 429 only when rate-limit or secondary-limit markers make bounded retry safe.

Generic HTTP 403 remains non-transient. Branch-protection required-status-check reads continue to use the existing fail-closed `github_get` path and are not covered by the wait-only transient fallback.

## Fail-Closed Guarantees

- A concrete `qsc-linux-full-suite` failure returns failure immediately.
- A concrete `macos-qsc-full-serial` failure returns failure immediately.
- A concrete `qsc-adversarial-smoke` failure returns failure immediately.
- Missing watched suites after the polling budget return timeout/failure.
- Pending watched suites after the polling budget return timeout/failure.
- Repeated transients consume the existing bounded polling attempts and eventually fail closed.
- Stale duplicate check runs do not decide the result; the highest-id latest run for each watched name is used.

## Self-Test Cases

Command:

```bash
python3 scripts/ci/public_safety_gate.py selftest-timeout-resilience
```

Covered fixtures:

- HTML timeout response while suites are pending, then all watched suites succeed.
- Non-JSON response while suites are pending, then all watched suites succeed.
- HTTP 502, 503, and 504 responses while suites are pending, then all watched suites succeed.
- `qsc-linux-full-suite` concrete failure fails closed.
- `macos-qsc-full-serial` concrete failure fails closed.
- `qsc-adversarial-smoke` concrete failure fails closed.
- Pending watched suites beyond budget fail closed.
- Missing watched suite beyond budget fails closed.
- Stale failure ignored when the latest duplicate run succeeds.
- Stale success ignored when the latest duplicate run fails.
- Generic branch-protection-style 403 is not classified as a transient bypass.
- 429 rate-limit response remains bounded-transient for wait polling.

## Commands Run

```bash
python3 -m py_compile scripts/ci/public_safety_gate.py
python3 scripts/ci/public_safety_gate.py selftest-timeout-resilience
```

Full validation also requires the directive validation bundle, including diff scope guard, queue/decision parsers, link check, leak scan, goal-lint, cargo audit, rustls-webpki reverse tree proof, and direct `send_commit` coverage.

## Limitations

- The new transient handling is intentionally limited to `wait-commit-checks` check-run polling.
- It does not change red-main admission policy.
- It does not change branch protection or required status check configuration.
- It does not extend the polling budget or create an unbounded retry loop.
- It does not attempt to prove GitHub availability; it only handles bounded transient responses safely.

## No-Weakening Statement

Public-safety still requires watched suites to complete successfully. The patch only prevents known transient GitHub polling responses from being treated as final suite failure while the suites are still pending or missing within the bounded wait. Real red check conclusions remain red.

## Future Improvements

- Add a structured summary line for transient counts to the public-safety job summary if future workflow scope authorizes it.
- Consider sharing this wait-only transient classifier with read-only evidence helpers if a later NA authorizes helper consolidation.
- Keep branch-protection and red-main admission reads fail-closed unless a future directive explicitly scopes a separate, audited improvement.
