Goals: G1 (primary), G2, supports G3, G4, G5

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-08

# NA-0623 — ENG-0012 Stage 2a: Suite-2 SCKA sender core (advertisement + PQ reseed) + both-sides RK advance

## Scope

Implements the Suite-2 SCKA send side IN REFIMPL ONLY (directive QSL-DIR-2026-07-08-560 / D560),
under DOC-CAN-004 §3.1–§3.4 and DOC-CAN-003 §8.5.3/§8.5.4/§3.3.3/§3.3.6. Adds `KDF_RK_PQ`, the
advertisement sender (`send_pq_advertise`), peer-ADV tracking (`track_peer_adv`), and the PQ-reseed
sender (`send_pq_reseed`); and — per the D560 AMENDMENT — the both-sides ROOT ADVANCE (the receiver
`recv_boundary_in_order` now advances `RK := KDF_RK_PQ` + recomputes `HK_r` after `apply_pq_reseed`,
and the sender mirrors it) so the PQ epoch secret lands in the root and the classical DH ratchet
carries it forward permanently. Does NOT wire the qsc client (Stage 2b = NA-0624), does NOT change
`apply_pq_reseed`'s CTXT-validation semantics, and does NOT touch the KEM/AEAD/KDF primitives, the
snapshot, or the wire format.

## Required Markers

- NA0623_SCKA_NO_SNAPSHOT_CHANGE_OK             (target sets already persist v2; caller-side key store)
- NA0623_SCKA_NO_WIRE_FORMAT_CHANGE_OK          (parse.rs already parses FLAG_PQ_ADV/FLAG_PQ_CTXT)
- NA0623_SCKA_NO_NONBOUNDARY_PATH_CHANGE_OK     (send_wire/recv_nonboundary unchanged)
- NA0623_SCKA_APPLY_PQ_RESEED_SEMANTICS_FROZEN_OK  (validation unchanged; vectors byte-identical)
- NA0623_SCKA_KDF_RK_PQ_QSP50_RKPQ_OK           (§3.3.3 KMAC32(RK,"QSP5.0/RKPQ",pq_ss||0x01))
- NA0623_SCKA_KDF_PQ_RESEED_REUSED_OK           (§3.3.6 seeds shared; §3.3.6 ordering CAT-S2-KDF-001)
- NA0623_SCKA_SEND_PQ_ADVERTISE_OK              (§8.5.4 / DOC-CAN-004 §3.1)
- NA0623_SCKA_SEND_PQ_RESEED_OK                 (§8.5.3 sender side / DOC-CAN-004 §3.3)
- NA0623_SCKA_BOTH_SIDES_RK_ADVANCE_OK          (receiver + sender advance the root; D560 AMENDMENT)
- NA0623_SCKA_ROUND_TRIP_APPLY_PQ_RESEED_DECRYPTS_OK
- NA0623_SCKA_PQ_PCS_HEALING_SURVIVES_DH_RATCHET_OK  (the headline)
- NA0623_SCKA_MONOTONICITY_ONE_TIME_TOMBSTONE_NO_MUTATION_OK
- NA0623_SCKA_HARNESS_OPS_AND_VECTORS_OK        (send ops + 6 CAT-SCKA-LOGIC-001 vectors, append-only)
- NA0623_SCKA_FULL_REFIMPL_REGRESSION_GREEN_OK
- NA0623_SCKA_WORKSPACE_ALL_TARGETS_BUILD_OK    (WF-0013)
- NA0623_SCKA_NO_QSC_CHANGE_OK
- NA0623_SCKA_HK_NOT_NHK_DEVIATION_FLAGGED_OK   (frozen receiver uses HK; §8.5.1 NHK deferred to 2b)
- NA0623_SCKA_CLAIM_BOUNDARY_OK

## Test Inventory

`tools/refimpl/quantumshield_refimpl/tests/suite2_scka_sender.rs`:
- `reseed_round_trip_sender_to_apply_pq_reseed_decrypts_and_converges` — a receiver advertises a
  receive key; the sender encapsulates to it and calls `send_pq_reseed`; the frozen `apply_pq_reseed`
  path (via `recv_wire`) decrypts the body; the directional PQ seed and the advanced root converge on
  both parties.
- `pq_pcs_healing_survives_dh_ratchet` — the headline: after a PQ reseed (root R0->R1) then a
  classical DH ratchet (R1->R2), a pre-reseed snapshot (root R0) CANNOT open the post-reseed DH
  boundary while the live receiver can. Proves the reseed advanced the root the DH ratchet reads —
  the PQ hardening is carried forward permanently (the D560 re-scope's whole point).
- `reseed_sender_rejects_are_deterministic_and_no_mutation` — bad ciphertext length, bad
  shared-secret length, unset send chain: deterministic reject, retained state unchanged.
- `reseed_replay_is_rejected_one_time` — a replay to an already-consumed/tombstoned target rejects
  with no state mutation.
- `advertise_frames_parseable_boundary_and_track_enforces_monotonicity` — parseable `FLAG_PQ_ADV`
  boundary + `known_targets` recorded; peer-ADV monotonicity + bad-pub-len reject.

Conformance vectors — `inputs/suite2/vectors/qshield_suite2_scka_logic_vectors_v1.json` (+6,
CAT-SCKA-LOGIC-001, append-only) via `scripts/ci/run_suite2_scka_logic_vectors.py` (14/14): harness
ops `suite2.send_pq_advertise` / `suite2.send_pq_reseed` — advertise-accept + reseed-accept
(byte-pinned wire + advanced-root/reseeded-chain state) + 4 rejects (bad-pub-len, non-monotonic
advertise, bad-ct-len, bad-ss-len). Frozen sets stay green: pqreseed 5/5 (`apply_pq_reseed`
byte-identical), boundary 4/4, scka_kem 5/5, kdf 6/6; schema validation OK.

Regression: the full `quantumshield_refimpl` suite (75 lib + integration) stays green — the
non-boundary message path and `apply_pq_reseed`'s validation semantics are unchanged.

## Result

`NA0623_SCKA_OK`. The SCKA sender core (advertisement + PQ reseed) and the both-sides RK advance are
implemented and proven in refimpl, including PQ-PCS healing that survives a classical DH ratchet.
Not wired into qsc; no post-quantum / Triple-Ratchet claim on live traffic (Stage 2b = NA-0624 wires
the advertise + reseed cadence into the real send path and persists the SCKA state, closing the P1).
