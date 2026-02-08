# NA-0107 — Remote Relay Auth Header Plan

## Scope
- qsl/qsl-client/qsc/** only.
- Add optional bearer token auth support for relay inbox push/pull.

## Assumptions
- Remote relay may require `Authorization: Bearer <token>`.
- Open/local relays may not require auth and must keep existing behavior.

## Auth Rules
- Env precedence: `QSC_RELAY_TOKEN` first, then `RELAY_TOKEN`.
- If token exists and is non-empty, set `Authorization` header on relay inbox push/pull.
- If token missing, do not send auth header.

## Deterministic Errors
- Map relay 401/403 to deterministic client error `relay_unauthorized`.
- Unauthorized path must be no-mutation.

## No-Secrets Rules
- Never print token value in markers/logs/UI/artifacts.
- Security scan checks outputs for token-like content.

## Test Vectors
- Auth-required relay:
  - no token => deterministic unauthorized failure.
  - wrong token => deterministic unauthorized failure.
  - correct token => send/receive success.
- Open relay:
  - no token => unchanged behavior.
- Determinism:
  - same input/token state => same marker subset.

## Verification Checklist
- `cargo fmt -p qsc -- --check`
- `cargo test -p qsc --locked`
- `cargo clippy -p qsc --all-targets -- -D warnings`
- `remote-relay-tests` workflow PASS:
  - `happy-path` with `seed=1`
  - `drop-reorder` with `seed=7`

## Rollback
- Revert header injection changes and deterministic error mapping together.
- Re-run qsc gates and workflow verification.
