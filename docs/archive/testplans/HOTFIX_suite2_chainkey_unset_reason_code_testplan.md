# HOTFIX — Suite-2 CHAINKEY_UNSET includes reason_code token

Date: 2026-01-17  
Goals: G2, G3

## Invariant

When Suite-2 rejects due to an unset chain key, the emitted reject string MUST include:

- `reason_code=REJECT_S2_CHAINKEY_UNSET`

This preserves deterministic, machine-checkable reject metadata for interop and regression testing.

## Test

Run:

- `cargo test -p quantumshield_refimpl suite2::ratchet::tests::asymmetric_recv_unset_chainkey_rejects_deterministically_and_no_mutation`

Expected:

- Test passes.
- The reject string observed by the test contains the `reason_code=REJECT_S2_CHAINKEY_UNSET` token.
