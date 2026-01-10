# Test Plan — AUDIT-20260104 Issue #4 (OsRng for crypto RNG)

Goals: G4, G5

## Scope
- Remove crypto-critical `thread_rng` usage in StdCrypto.
- Ensure secret material uses OS-backed RNG (OsRng).

## Validation steps
1. Confirm stdcrypto.rs no longer references `thread_rng`.
2. Run `cargo test -p quantumshield_refimpl` and verify:
   - `x25519_keypair_uses_os_rng`
   - `random_nonce12_not_all_zero`
3. Ensure CI checks pass on the PR.

## Expected results
- StdCrypto uses OsRng for keypair and nonce generation.
- Tests pass without panics or flakiness.

## Evidence
- CI logs for PR.
