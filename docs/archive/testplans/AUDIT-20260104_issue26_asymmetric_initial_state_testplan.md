# AUDIT-20260104 Issue #26 — Asymmetric initial state in Suite-2 establish

Goals: G2, G3

## Invariant being protected
Asymmetric ZERO32 chainkeys produced by Suite-2 establish are sentinel values only and MUST NOT be consumed for cryptographic use.
Consumption attempts MUST reject deterministically and MUST NOT mutate state on reject.

## What must never happen
- Any chainkey with the all-zero sentinel value is used to derive message keys or perform AEAD operations.
- Reject paths mutate the send/recv state.

## Expected behavior
- Any attempt to consume an unset chainkey returns a deterministic reject that includes a `reason_code=` token.
- State remains unchanged on reject, and the same input yields the same reject string.

## Tests
- `asymmetric_send_unset_chainkey_rejects_deterministically_and_no_mutation` (tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs)
- `asymmetric_recv_unset_chainkey_rejects_deterministically_and_no_mutation` (tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs)
