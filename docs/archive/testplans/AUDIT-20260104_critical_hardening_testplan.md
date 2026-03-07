Goals: G2, G4
Status: DRAFT

Authoritative audit source:
- docs/audit/CODE_ANALYSIS_REPORT_20260104.md

Scope:
- Refimpl hardening only; no wire semantics changes.

Objective:
- Ensure ed25519 sign/verify fail-closed on invalid key lengths.
- Ensure skipped-key derivation rejects on u32 overflow deterministically.
  - Issue mapping:
    - Ed25519 fail-closed => Issue #1 (verify fallback), Issue #3 (sign panic)
    - Ratchet overflow reject => Issue #2 (skip-loop overflow)

CI-gated assertions:
- Unit tests in `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs` verify:
  - invalid-length pubkey returns false
  - invalid-length privkey returns empty signature
- Unit test in `tools/refimpl/quantumshield_refimpl/src/qsp/ratchet.rs` verifies:
  - u32 overflow in skip loop is rejected

Evidence:
- `cargo test -p quantumshield_refimpl`
- suite2-ci on PR.
