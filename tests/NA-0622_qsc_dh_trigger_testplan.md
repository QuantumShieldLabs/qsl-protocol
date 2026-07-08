Goals: G1 (primary), G2, supports G3, G4, G5

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-08

# NA-0622 — ENG-0012 Stage 1b-ii: qsc DH-ratchet trigger + static-rk removal

## Scope

Wires the classical DH ratchet into the REAL qsc send/receive path (directive D559): ratchet-on-
reply + N=4/T=15min fallback via the refimpl `send_boundary`/`recv_dh_boundary`, static-`rk`
bootstrap removed, trigger persisted in a qsc session-blob v2. Closes the CLASSICAL half of the P1
(ENG-0012). PQ reseed is Stage 2 (NA-0623). Refimpl frozen.

## Required Markers

- NA0622_TRIGGER_RATCHET_ON_REPLY_OK
- NA0622_TRIGGER_FALLBACK_N4_T15MIN_OK
- NA0622_RECV_BOUNDARY_ROUTING_OK
- NA0622_STATIC_RK_BOOTSTRAP_REMOVED_OK
- NA0622_RESPONDER_FIRST_SEND_CREATES_CHAIN_OK
- NA0622_BLOB_V2_TRIGGER_PERSISTED_OK
- NA0622_BLOB_V1_MIGRATION_OK
- NA0622_REFIMPL_FROZEN_OK
- NA0622_NO_WIRE_FORMAT_CHANGE_OK
- NA0622_SEED_FALLBACK_GATE_TEST_ONLY_OK
- NA0622_E2E_ROUNDTRIP_REAL_HANDSHAKE_OK
- NA0622_E2E_PCS_HEALING_REAL_HANDSHAKE_OK
- NA0622_RUNTIME_EQUIVALENCE_BYTE_FOR_BYTE_OK
- NA0622_FULL_QSC_REGRESSION_GREEN_OK
- NA0622_WORKSPACE_ALL_TARGETS_BUILD_OK
- NA0622_CLAIM_BOUNDARY_CLASSICAL_ONLY_OK

## Test Inventory

`qsl/qsl-client/qsc/tests/handshake_mvp.rs` (real A/B handshake):
- `dh_ratchet_e2e_roundtrip_over_real_handshake` — Alice's first send is normal; Bob's reply is a
  DH boundary (ratchet-on-reply creates his send chain); Alice decrypts the boundary; the reverse
  direction ratchets and decrypts too.
- `dh_ratchet_e2e_pcs_healing_over_real_handshake` — a restored pre-ratchet session snapshot
  cannot decrypt a post-ratchet message (end-to-end post-compromise self-healing).

Regression: `suite2_runtime_equivalence_na0198` (seed path, gated — byte-for-byte equivalent to
the refimpl), the migration/session-at-rest tests (blob v2 + v1 migration), and the full qsc
suite (attachment/message/transport/relay/handshake) all green. The refimpl suite is unchanged
(frozen).

## Result

`NA0622_OK`. The classical DH ratchet runs on the real qsc send path (ratchet-on-reply + N=4/T=15
min), the static-`rk` bootstrap is removed, and post-compromise security is proven end-to-end over
a real handshake. Classical half of the P1 closed; POST-QUANTUM guarantee awaits Stage 2. No
Triple-Ratchet / post-compromise / production claim beyond the classical, proven scope.
