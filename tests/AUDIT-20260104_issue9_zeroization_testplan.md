# Audit Issue #9 — Key Zeroization Test Plan (DRAFT)

Goals: G4, G5

## Invariant
Secret-bearing key material must be zeroized on drop/overwrite to prevent residual secret exposure in memory.

## Scope
- X25519 private key material in the refimpl crypto layer.
- Regression guards that fail if Zeroize/ZeroizeOnDrop are removed.

## Tests
1) Compile-time guard: X25519Priv implements `Zeroize` and `ZeroizeOnDrop`.
2) Runtime guard: calling `.zeroize()` on X25519Priv zeros all bytes.

## Evidence
- Rust unit tests in `tools/refimpl/quantumshield_refimpl/src/crypto/traits.rs`.
- CI: `cargo test -q` via required checks.
