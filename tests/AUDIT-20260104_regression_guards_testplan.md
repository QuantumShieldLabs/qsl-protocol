Goals: G2, G4
Status: DRAFT

Scope:
- CI regression guards for CRITICAL issues #1–#3 from docs/audit/CODE_ANALYSIS_REPORT_20260104.md.
- Refimpl tests only; no wire semantics changes.

Assertions (CI-gated):
- Issue #1 (verify fallback): ed25519 verify returns false on invalid pubkey length and does not panic.
- Issue #3 (sign panic): ed25519 sign returns fail-closed output on invalid privkey length and does not panic.
- Issue #2 (skip-loop overflow): ratchet skip-loop increment rejects on u32 overflow deterministically.

Evidence:
- Unit tests:
  - tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs
    - ed25519_verify_rejects_invalid_pubk_len
    - ed25519_sign_invalid_priv_len_is_fail_closed
  - tools/refimpl/quantumshield_refimpl/src/qsp/ratchet.rs
    - checked_inc_nr_overflow_rejects
- CI: suite2-vectors / ci-4x lanes (cargo test for refimpl crate)
