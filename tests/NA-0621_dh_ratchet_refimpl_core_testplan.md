Goals: G1 (primary), G2, supports G3, G4, G5

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-08

# NA-0621 — ENG-0012 Stage 1b-i: Suite-2 DH Ratchet (send+receive) + NHK, refimpl core

## Scope

Implements the classical X25519 DH ratchet for Suite-2 IN REFIMPL ONLY (directive
QSL-DIR-2026-07-08-558 / D557→D558), under DOC-CAN-003 §3.3.2/§3.4/§8.1/§8.5.1/§8.5.2. Adds the
missing `NHK` header-key machinery, `send_boundary` (DH send) and `recv_dh_boundary` (DH receive
+ anti-spoof), and proves post-compromise self-healing. Does NOT wire the qsc client, does NOT
remove the static-`rk` bootstrap (Stage 1b-ii), and does NOT touch the PQ-reseed path (Stage 2).

## Required Markers

- NA0621_DHR_NO_WIRE_FORMAT_CHANGE_OK          (DH_pub[32] already on wire per §4.3)
- NA0621_DHR_NO_NONBOUNDARY_PATH_CHANGE_OK     (send_wire/recv_nonboundary unchanged)
- NA0621_DHR_NO_PQ_RESEED_CHANGE_OK            (apply_pq_reseed semantics untouched)
- NA0621_DHR_NO_SNAPSHOT_CHANGE_OK             (NHK derived on demand; no state growth)
- NA0621_DHR_KDF_RK_DH_QSP50_RKDH_OK           (§3.3.2 KMAC256 64-byte split)
- NA0621_DHR_HK_NHK_DERIVATION_OK              (§3.4/§8.1 directional HK/NHK)
- NA0621_DHR_SEND_BOUNDARY_OK                  (§8.5.2 send)
- NA0621_DHR_RECV_DH_BOUNDARY_OK               (§8.5.2 receive)
- NA0621_DHR_CURRENT_NHK_ANTISPOOF_OK          (§8.5.1 header under CURRENT_NHK only)
- NA0621_DHR_TWO_PARTY_ROUNDTRIP_BOTH_DIRECTIONS_OK
- NA0621_DHR_PCS_HEALING_OK                    (pre-ratchet snapshot cannot decrypt post-ratchet)
- NA0621_DHR_NO_MUTATION_ON_REJECT_OK
- NA0621_DHR_FULL_REFIMPL_REGRESSION_GREEN_OK
- NA0621_DHR_WORKSPACE_ALL_TARGETS_BUILD_OK    (WF-0013)
- NA0621_DHR_NO_QSC_CHANGE_OK
- NA0621_DHR_CLAIM_BOUNDARY_OK

## Test Inventory (co-located refimpl conformance tests)

`tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs::tests`:
- `dh_ratchet_two_party_roundtrip_both_directions` — establishes a matched A/B pair, A performs a
  DH boundary and B decrypts it, then B performs a DH boundary and A decrypts it. Proves the
  ratchet fires and round-trips in both directions (incl. the responder's send chain, which the
  boundary CREATES).
- `dh_ratchet_pcs_healing` — the headline: an adversary snapshots B at epoch 0; A ratchets, B
  ratchets with a fresh keypair, A ratchets again and sends a secret; the real B decrypts it, but
  the epoch-0 snapshot CANNOT (its stale `NHK` fails to authenticate the boundary header). This is
  the post-compromise self-healing property, absent before this lane.
- `dh_ratchet_no_mutation_on_reject` — a tampered body ciphertext is rejected and the session
  state is returned byte-for-byte unchanged (`snapshot_bytes` equal).

Regression: the full `quantumshield_refimpl` suite (lib + integration) stays green — the
non-boundary message path and the PQ-reseed path are unchanged.

## Result

`NA0621_DHR_OK`. The classical DH ratchet (send + receive + NHK) is implemented and proven in
refimpl, with PCS self-healing demonstrated. Not wired into qsc; no post-compromise/Triple-Ratchet
claim (Stage 1b-ii wires the trigger + removes the static-`rk` bootstrap; Stage 2 adds PQ reseed).
