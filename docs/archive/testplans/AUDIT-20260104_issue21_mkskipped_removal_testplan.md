# AUDIT-20260104 Issue #21 — MKSKIPPED removal without recovery (Suite-2 ratchet)

Goals: G2, G3

## Invariant being protected
When a received message hits an `MKSKIPPED` entry (matching `dh_pub` and `n`) but body authentication/decryption fails, the
implementation MUST reject deterministically and MUST NOT mutate ratchet state on the reject path.

## What must never happen
- `MKSKIPPED` entry is removed/consumed on a decrypt/auth failure.
- Any ratchet state advance occurs on reject (counters/epochs/mkskipped mutated).

## Expected behavior
- Return deterministic reject: `REJECT_S2_BODY_AUTH_FAIL`.
- State remains unchanged (no mutation on reject); the same input repeated produces the same reject.

## Test vectors / cases
- Unit test: `issue21_mkskipped_not_removed_on_auth_fail` (in `suite2/ratchet.rs`)
  - Arrange: recv state contains a single `MKSKIPPED` entry.
  - Act: feed a message that matches the entry but fails body auth/decrypt.
  - Assert:
    - reject code is `REJECT_S2_BODY_AUTH_FAIL`
    - state snapshot equals pre-call snapshot
    - repeat call yields same reject and same state

## Notes
This test is intentionally unit-level to prove the “no mutation on reject” invariant independent of higher-level harness behavior.
