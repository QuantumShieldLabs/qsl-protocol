# NA-0105 Truthful ACTIVE Session-Only Plan

## Scope
- `qsl/qsl-client/qsc/**` only.
- Remove seed-only ACTIVE status in production status/gate paths.
- Keep seed fallback available only behind explicit test-only override.

## Reasons Enum
- `missing_home`
- `unsafe_parent`
- `channel_invalid`
- `missing_seed`
- `no_session`
- `session_invalid`
- `handshake`

## Session Validation Rules
- ACTIVE requires `qsp_session_load(peer)` to succeed and deserialize.
- Missing session returns INACTIVE (`missing_seed` when no seed env, `no_session` when seed set but no validated session).
- Corrupt/invalid session file returns INACTIVE `session_invalid`.
- Send/receive gates are peer-scoped (`to`/`from`) and only pass when ACTIVE for that peer.
- Seed fallback path is blocked unless `QSC_ALLOW_SEED_FALLBACK=1` is explicitly set.

## Test Vectors
- `status_seed_alone_is_inactive`
- `status_missing_seed_reason_missing_seed`
- `status_invalid_session_reason_session_invalid`
- `status_valid_session_reason_handshake`
- Existing protocol-gate and on-wire suites retained for fail-closed and no-secrets coverage.

## Executed Evidence
- `cargo fmt -p qsc -- --check`
- `cargo test -p qsc --test qsp_status_truthy --locked`
- `cargo test -p qsc --locked`
- `cargo clippy -p qsc --all-targets -- -D warnings`
- Forensics log root:
  - `/home/victor/work/qsl/_forensics/na0105_impl_20260207T225554Z`

## Rollback
- Revert `qsp_status_tuple(peer)` and peer-scoped gate changes in `qsl/qsl-client/qsc/src/main.rs`.
- Revert NA-0105 status tests in `qsl/qsl-client/qsc/tests/qsp_status_truthy.rs`.
