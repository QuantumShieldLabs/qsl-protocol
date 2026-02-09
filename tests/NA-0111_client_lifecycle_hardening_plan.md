# NA-0111 Client Lifecycle Hardening Plan

## Scope & assumptions
- Scope is limited to `qsl/qsl-client/qsc/**` for implementation.
- No server/refimpl/workflow changes are required for NA-0111 implementation.
- Existing vault/session encryption posture remains baseline; this NA hardens lifecycle handling and leak resistance.

## Threat model
- Local client attacker with access to process output, logs, and working directory artifacts.
- Crash-surface exposure through panic text, backtraces, or crash dump handling.
- Leakage risk via env/token handling, marker formatting, temporary files, and lifecycle reject paths.

## Must never
- Secrets or secret substrings in stdout/stderr/markers/logs.
- Any writes of state/session/temp artifacts into CWD or repo root.
- Silent retries or non-deterministic reject behavior on lifecycle/security failures.

## Proposed hardening steps
- Add a centralized redaction guard for all marker and output formatting paths.
- Enforce explicit token env handling and avoid token echo in diagnostics.
- Install panic hook that emits deterministic redacted marker only.
- Enforce deterministic debug/backtrace gating with release-safe defaults.
- Tighten temporary file lifecycle discipline and shutdown cleanup.

## Test vectors
- Simulate panic path and capture output to confirm no secret leakage.
- Simulate token-bearing environment and verify token value is never printed.
- Add CWD write probes for vault/session/temp and assert no repo-root artifacts created.
- Add documentation note and checks for core dump/backtrace operational guidance in release mode.

## Verification checklist
- `cargo fmt -p qsc -- --check`
- `cargo test -p qsc --locked`
- `cargo clippy -p qsc --all-targets -- -D warnings`
- Regression tests cover:
  - no-secret output scanning
  - panic leakage prevention
  - CWD write prevention
  - deterministic fail-closed lifecycle rejects with no mutation

## Rollback
- Revert NA-0111 implementation commit(s) if leakage/no-mutation invariants regress.
- Keep deterministic reject behavior as the default fallback while narrowing blast radius.
