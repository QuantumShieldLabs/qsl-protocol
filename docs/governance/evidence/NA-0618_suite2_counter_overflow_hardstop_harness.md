Goals: G1 (primary), supports G2, G3, G4, G5

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-07

# NA-0618 — ENG-0013 Suite-2 Symmetric Counter Overflow Hard-Stop

## Summary

NA-0618 implements ledger ENG-0013 (Comprehensive/Suite-2 review H-1) under directive
QSL-DIR-2026-07-07-555 (D555). The Suite-2 symmetric message counters `ns`/`nr` advanced with
`saturating_add(1)` and no `u32::MAX` hard-stop. Because Suite-2 sessions never re-key (no DH
ratchet / PQ reseed fires — see ENG-0012), a counter that reaches `u32::MAX` would freeze, and
with static header keys the header nonce/ciphertext would repeat — a nonce-reuse-class failure
(bounded behind ~4.29e9 one-way messages). This lane makes it **fail closed**. Refimpl
`suite2` source + tests + one canonical-spec reject-code line only; no key-schedule/KDF/AEAD/
nonce/wire-format/descriptor change, no qsc-client/qsl-attachments/qsl-server change, no
dependency/workflow change.

Result classification: `SUITE2_COUNTER_OVERFLOW_FAIL_CLOSED`.

## Design (design-lock)

- Three counter-advance sites in `tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs`:
  `send_wire` (`ns`), `recv_nonboundary_ooo` (`nr`), `recv_boundary_in_order` (`nr`) — all
  `saturating_add(1)`. The sibling `qsp/ratchet.rs` already guards `ns == u32::MAX`.
- No-mutation-on-reject confirmed: the recv reject paths return `state: st` (unchanged) and
  never build `new_state`; the guard is placed before the state clone.
- Unreachable in normal operation: a compliant `send_wire` refuses to originate a message at
  the saturating counter, so a well-behaved receiver never sees `header_n == u32::MAX`; the
  receive-side guards are defense-in-depth against a malicious/buggy peer.

## Change

- New `checked_counter_inc(counter: u32) -> Result<u32, &'static str>` — a fail-closed `+1`
  returning `REJECT_S2_COUNTER_OVERFLOW` when it would pass `u32::MAX`. Used at all three
  advance sites in place of `saturating_add`, so a saturated counter can never freeze.
- `send_wire` computes `ns_next = checked_counter_inc(st.ns)?` before deriving key material
  (fail closed early); `recv_nonboundary_ooo` and `recv_boundary_in_order` compute
  `nr_next` and, on `Err`, return their outcome with `reason = REJECT_S2_COUNTER_OVERFLOW`
  and unmutated state.
- New `REJECT_S2_COUNTER_OVERFLOW` reject const (local reason code, pattern of
  `REJECT_S2_CHAINKEY_UNSET`); registered in `DOC-CAN-003 §10` for G4 spec/impl parity. Not
  wire-transmitted (consistent with the NA-0612 local-only reject-taxonomy finding).

## Why fail-closed strictness is preserved / strengthened

- Removes a latent nonce-reuse edge: a frozen counter can no longer repeat a header nonce.
- No key-schedule/KDF/AEAD/nonce-derivation/wire-format change; the guard only replaces a
  saturating increment with a checked one and adds a terminal reject.
- No-mutation-on-reject holds at all three sites.

## Tests (deterministic)

`tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs` `#[cfg(test)]`:
1. `checked_counter_inc_boundary_and_normal` — `0 -> Ok(1)`, `u32::MAX-1 -> Ok(u32::MAX)`,
   `u32::MAX -> Err(REJECT_S2_COUNTER_OVERFLOW)`. This is the single point used at all three
   advance sites, so it directly covers the send and both receive guards (the receive-side
   full path at `u32::MAX` is unreachable through the public API because a compliant sender is
   guarded).
2. `send_wire_rejects_counter_overflow_at_ns_max_and_no_mutation` — a valid send state with
   `ns == u32::MAX` rejects with `REJECT_S2_COUNTER_OVERFLOW`, deterministically, with no state
   mutation (and no AEAD use — `PanicAead` is never called).

Regression: full `quantumshield_refimpl` test suite green (70 lib unit tests + all integration
tests); `cargo fmt --check` clean; `cargo clippy -p quantumshield_refimpl --all-targets
-D warnings` clean; `cargo metadata --locked` green; Cargo unchanged. The suite2 conformance
vectors use small counters and are unaffected.

## Claim boundary

Research/demo. No public/production/security-complete/crypto-complete/bug-free/vulnerability-
free claim. This lane closes one bounded nonce-reuse edge; it does NOT implement the DH ratchet
/ PQ reseed (ENG-0012, the P1 post-compromise-security gap) and makes no post-compromise or
Triple-Ratchet claim.
