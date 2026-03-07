# AUDIT-20260104 Issue #23 — ss3 entropy discarded in handshake

Goals: G1, G2, G3

## Invariant being protected
If `ss3` is produced by PQ KEM (encap/decap) during the handshake, it MUST be mixed into the handshake transcript/key schedule so it
contributes entropy and binding.

## What must never happen
- `ss3` is computed/decapsulated and then discarded without contributing to derived keys.
- Handshake proceeds successfully while ignoring `ss3` (silent entropy loss).

## Expected behavior
- Both sides (initiator and responder) incorporate `ss3` into the same derivation step(s) (same ordering/labels).
- No wire-format changes.
- Deterministic behavior.
- On failure (e.g., decap fails), reject deterministically with no session state mutation or partial construction.

## Tests
- `ss3_mix_changes_rk0_deterministically` (tools/refimpl/quantumshield_refimpl/src/qsp/handshake.rs)
  - Changing `ss3` changes derived `rk0` deterministically.
- `ss3_decap_failure_rejects_deterministically_and_no_mutation` (tools/refimpl/quantumshield_refimpl/src/qsp/handshake.rs)
  - Forced decap failure returns deterministic error; input HS2 remains unchanged.
