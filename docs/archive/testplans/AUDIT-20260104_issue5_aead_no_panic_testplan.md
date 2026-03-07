# Test Plan — AUDIT-20260104 Issue #5 (AEAD no panic, fail-closed)

Goals: G4, G5

## Scope
- StdCrypto AEAD seal must not panic and must fail closed on invalid inputs.
- Call sites must reject empty ciphertext deterministically.

## Validation steps
1. Run unit tests in `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs`:
   - `aead_seal_invalid_key_len_is_fail_closed`
   - `aead_seal_invalid_nonce_len_is_fail_closed`
2. Confirm `ratchet_encrypt` and Suite-2 `send_wire` reject empty ciphertexts.
3. Ensure CI lanes exercising refimpl tests pass.

## Expected results
- Invalid key/nonce lengths return `CryptoError::InvalidKey` (no panic).
- AEAD seal failure yields empty ciphertext and is rejected by callers.

## Evidence
- Test output from `cargo test -p quantumshield_refimpl`.
- CI checks (suite2, ci-4a..ci-4d) pass on the PR.
