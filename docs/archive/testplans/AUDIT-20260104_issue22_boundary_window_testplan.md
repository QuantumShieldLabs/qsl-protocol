# AUDIT-20260104 Issue #22 — Boundary message window not enforced (Suite-2 ratchet)

Goals: G2, G3

## Invariant being protected
Boundary receive MUST attempt header authentication only for `cand = st.nr` (no boundary window scan). Reject paths MUST be deterministic
and MUST NOT mutate state.

## What must never happen
- Boundary receive attempts `MAX_HEADER_ATTEMPTS` candidates when semantics require `n == st.nr`.
- State mutation on any reject path (boundary counters/targets/ck_* changes).

## Expected behavior
- Exactly one boundary header auth attempt per message (cand = st.nr).
- Rejects remain deterministic:
  - On header auth failure: `REJECT_S2_HDR_AUTH_FAIL`.
  - If header decrypt yields `n != st.nr`: `REJECT_S2_BOUNDARY_NOT_IN_ORDER` (defensive).
- State remains unchanged on reject.

## Tests
- `issue22_boundary_single_attempt_no_mutation_on_reject` (tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs)
  - Arrange: boundary state via existing helpers.
  - Act: send boundary message that fails header auth twice.
  - Assert: deterministic reject + no mutation; header try count increments by 1 per call.
