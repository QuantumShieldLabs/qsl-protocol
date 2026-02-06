# NA-0099 Handshake A2 Confirm Plan

## Scope & assumptions

## Message flow (A1/B1/A2)

## Key derivation: K_confirm domain separation from PQ secret

## State machine: PendingConfirm vs Established

## Deterministic markers

## Test vectors (happy, tamper, replay, out-of-order)

## No-mutation-on-reject checks

## Verification checklist

## Rollback

## Executed evidence
- Tests updated/added in `qsl/qsl-client/qsc/tests/handshake_mvp.rs`:
  - `handshake_a2_required_for_b_active`
  - `handshake_a2_tamper_rejects_no_mutation`
  - `handshake_a2_replay_rejects_no_mutation`
  - `handshake_a2_out_of_order_rejects_no_mutation`
- Local gates (to be run before PR):
  - `cargo fmt -p qsc -- --check`
  - `cargo test -p qsc --locked`
  - `cargo clippy -p qsc --all-targets -- -D warnings`
