# NA-0100 Identity Binding (TOFU) Plan

## Scope & assumptions

## Identity definition (PQ KEM pubkey fingerprint)

## Storage/pinning rules (TOFU)

## Deterministic markers (identity_pin, identity_mismatch, identity_ok)

## Test vectors (first pin, mismatch reject no mutation, replay/out-of-order interaction)

## Verification checklist

## Rollback

## Executed evidence
- Added tests: qsl/qsl-client/qsc/tests/identity_binding.rs
- Tests:
- `tofu_pins_on_first_handshake`
- `tofu_mismatch_rejected_no_mutation`
- Gates: `cargo fmt -p qsc -- --check`, `cargo test -p qsc --locked`, `cargo clippy -p qsc --all-targets -- -D warnings`
