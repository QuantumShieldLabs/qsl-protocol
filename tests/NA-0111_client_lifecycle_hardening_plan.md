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

## Executed evidence (implementation PR)
- Implemented startup panic redaction hook in `qsl/qsl-client/qsc/src/main.rs`:
  - deterministic marker emitted on panic: `QSC_MARK/1 event=panic code=panic_redacted`
  - panic payload/backtrace text is not printed by qsc panic hook path.
- Added explicit panic demo command:
  - `qsc util panic-demo`
  - demo panic includes sentinel `QSC_SECRET_PANIC_SENTINEL=SHOULD_NOT_LEAK` for regression verification.
- Added lifecycle regression tests in `qsl/qsl-client/qsc/tests/lifecycle.rs`:
  - `panic_is_redacted_no_secrets`
  - `no_cwd_artifacts_for_common_commands`
  - `no_secrets_in_outputs_smoke`

- Local gates run (isolated caches):
  - `cargo fmt -p qsc -- --check` PASS
  - `cargo test -p qsc --locked` PASS
  - `cargo clippy -p qsc --all-targets -- -D warnings` PASS

- Deterministic marker expectations validated:
  - panic path output includes `event=panic code=panic_redacted`
  - panic path output does not include sentinel `QSC_SECRET_PANIC_SENTINEL=SHOULD_NOT_LEAK`
