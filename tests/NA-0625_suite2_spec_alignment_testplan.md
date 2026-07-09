# NA-0625 — ENG-0023: Suite-2 spec-alignment (§8.5.1 NHK boundary header + authenticated ADV receive) — Test Plan

Directive: QSL-DIR-2026-07-09-562 (D562). Decision: D-1245. Closes ENG-0023.

## Objective
Prove the two named header-authentication gaps are closed on the previously frozen Suite-2 receiver
surface, WITHOUT changing any other refimpl semantic, any normative DOC-CAN text, `parse.rs`, the
QS2S snapshot format, any KDF/AEAD/KEM primitive, or the seed-model runtime equivalence. Conformance
vectors change ONLY inside the NAMED, REVIEWED set — two files by directive, plus a third (24 bytes
of one vector) added by Operator Decision 5 when the NHK change was found to invalidate a byte-pinned
HK-sealed boundary frame there.

Gap (1) — the PQ-CTXT boundary header moves from `HK` to the §8.5.1 `NHK` (receiver
`recv_boundary_in_order` + Stage-2a sender mirror `send_pq_reseed`), derived on the fly from the
pre-reseed root. Gap (2) — an authenticated ADV receive path (`recv_pq_adv`) cryptographically binds
a tracked advertisement to the session BEFORE it is persisted, and consumes its chain slot in order.

## Refimpl unit tests (`tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs`)
1. `ctxt_boundary_hk_downgrade_rejects_no_mutation` — a pre-NA-0625-style boundary frame (header
   sealed under `HK_r`) fails generically with `REJECT_S2_HDR_AUTH_FAIL`; the boundary-state
   snapshot is byte-identical before/after. GREEN.
2. `adv_recv_accept_consumes_chain_and_tracks` — an authenticated in-order ADV is accepted; BOTH
   receive chains step exactly once, `nr` advances exactly once, `mkskipped` stays empty, and the
   app payload (MAC stripped) is returned. The frozen CTXT-path `peer_max_adv_id_seen` field is
   NOT touched (the peer-ADV watermark is caller-owned — the two id spaces are distinct). GREEN.
3. `adv_recv_spoofed_header_rejects_no_mutation` — a planted ADV whose header is sealed under a
   foreign key fails FIRST at the header AEAD (`REJECT_S2_HDR_AUTH_FAIL`), no mutation. GREEN.
4. `adv_recv_bad_mac_rejects_no_mutation` — a valid AEAD frame whose ADVAUTH MAC was computed under
   a foreign root fails `REJECT_S2_BODY_AUTH_FAIL` (reused code), no mutation. GREEN.
5. `adv_recv_missing_mac_rejects_no_mutation` — a pre-NA-0625-format ADV body (no leading MAC — the
   ADV downgrade case) fails `REJECT_S2_BODY_AUTH_FAIL`, no mutation. GREEN.
6. `adv_recv_out_of_order_rejects_no_mutation` — an ADV at `n = nr + 1` opens in the candidate
   window but rejects `REJECT_S2_BOUNDARY_NOT_IN_ORDER` (in-order-only control plane), no mutation.
   GREEN.
7. `adv_recv_nonmonotonic_rejects_no_mutation` — an authenticated ADV with `id <= watermark` rejects
   `REJECT_SCKA_ADV_NONMONOTONIC` (existing code), no mutation. GREEN.
8. `adv_recv_combined_flags_rejects` — a combined `ADV|CTXT` frame stays
   `REJECT_S2_LOCAL_UNSUPPORTED` (ENG-0026 territory, untouched). GREEN.

Pre-existing boundary tests (`boundary_success_advances_ck_pq_recv_from_reseed`,
`boundary_reject_is_deterministic_and_no_state_mutation_on_bad_ct_len`,
`issue22_boundary_single_attempt_no_mutation_on_reject`, the `header_pt_invalid_*` pair) still pass:
the two that seal a real frame now seal under NHK; the fake-AEAD ones are key-agnostic. The
single-attempt (issue-22) property is preserved — the receiver still tries exactly the two
candidates `[nr, nr+1]`, now under NHK only.

## Refimpl integration tests (`tools/refimpl/quantumshield_refimpl/tests/suite2_scka_sender.rs`)
9. `adv_recv_round_trip_consumes_chain_and_next_message_in_order` — A advertises, B's `recv_wire`
   routes to `recv_pq_adv`, the MAC verifies under the shared root, both receive chains consume the
   slot, and A's NEXT normal message decrypts strictly in order with `mkskipped` still empty (the
   Decision-2 retirement of the mkskipped control-slot growth). GREEN.
10. `adv_then_reseed_same_pack_round_trips` — THE DECISION-2 PROOF at refimpl level: A packs
    `[ADV, reseed]`; B accepts the ADV (which passes `nr` through the control slot) and then the
    reseed's strict `n == nr` check holds; both parties converge on the advanced root and no
    `mkskipped` entry is created. GREEN.
11. `reseed_receiver_send_schedule_must_be_refreshed_from_advanced_root` — the ENG-0030 FINDING,
    pinned: after a party RECEIVES a reseed, the receive path alone leaves its `send.hk_s` and
    `send.ck_pq` on the pre-reseed schedule (both asserted stale); after the caller-side composition
    (adopt `dh.rk`, recompute `send.hk_s` from the advanced root, take `send.ck_pq` from
    `recv.ck_pq_send`) both directions are coherent and the receiver's next advertisement
    authenticates at the peer's `recv_pq_adv` — header AND body. GREEN.

Pre-existing NA-0623 proofs unchanged and still green:
`reseed_round_trip_sender_to_apply_pq_reseed_decrypts_and_converges`, `pq_pcs_healing_survives_dh_ratchet`
(the headline), `advertise_frames_parseable_boundary_and_track_enforces_monotonicity`,
`reseed_sender_rejects_are_deterministic_and_no_mutation`, `reseed_replay_is_rejected_one_time`.

## Conformance vectors (the NAMED files; append/replace only — TWO by directive, a THIRD added by Operator Decision 5)
`inputs/suite2/vectors/qshield_suite2_scka_logic_vectors_v1.json` (14 -> 19):
- CHANGED bytes (2, as pinned at design-lock): `S2-SEND-PQADV-ACCEPT-0001` (`wire_hex`: `body_ct`
  +32 for the ADVAUTH MAC) and `S2-SEND-PQRESEED-ACCEPT-0001` (`wire_hex`: `hdr_ct` under NHK).
  Their `new_state` expectations are unchanged in both cases.
- APPENDED (5, op `suite2.recv_pq_adv`): `S2-RECV-PQADV-ACCEPT-0001`,
  `S2-RECV-PQADV-REJECT-SPOOFED-0001`, `S2-RECV-PQADV-REJECT-BADMAC-0001`,
  `S2-RECV-PQADV-REJECT-REPLAY-0001`, `S2-RECV-PQADV-REJECT-NOMAC-0001`.

`inputs/suite2/vectors/qshield_suite2_pq_reseed_vectors_v1.json` (5 -> 7):
- APPENDED (2, op `suite2.boundary.run`): `S2-RECV-PQRESEED-NHK-ACCEPT-0001` (NHK round-trip) and
  `S2-RECV-PQRESEED-REJECT-HK-DOWNGRADE-0001` (the header-downgrade rejection of DoD 4).

BYTE-IDENTITY PROOF (machine-checked during regeneration, recorded in
`docs/governance/evidence/NA-0625_vector_regen_proof.json.txt`): the other **17** pre-existing
vectors across the two files are byte-identical, and the changed set is exactly the two named
`S2-SEND-*-ACCEPT` vectors. Regenerator archived at `docs/governance/evidence/NA-0625_vector_regen.py`.

ALL OTHER frozen vector sets are byte-identical (untouched on disk) and green: boundary (4/4),
parse (6/6), kdf (6/6), transcript (4/4), mk_hybrid (3/3), establish (14/14), ooo_replay (6/6),
crash_restart (3/3), interop (3/3), interop_ximpl (2/2), scka_kem (5/5), downgrade (5/5).
Regenerated runners: scka_logic 19/19, pq_reseed 7/7.

**`e2e_recv` — the THIRD named vector file (Operator Decision 5, D562 addendum).** The NHK change
invalidated one byte-pinned frame here: `S2-E2E-ACCEPT-BOUNDARY-0001` -> `input.steps[0].wire_hex`
pinned a `flags = 0x0006 (PQ_CTXT|BOUNDARY)` frame whose header was sealed under `HK` by the
pre-NA-0625 sender, which the NHK-only receiver correctly rejects. Raised as a directive STOP at the
merge boundary; the operator extended the named vector-file list from two files to three with a
bounded mutation, executed and machine-checked: exactly the 24 header-ciphertext bytes `[1136, 1160)`
re-sealed under the NHK from that vector's own `recv_state.rk`, one line of the file, one field.
That vector's `recv_state`, `expect`, and non-wire step fields are byte-identical, and its three
siblings (`S2-E2E-ACCEPT-NONBOUNDARY-0001`, `S2-E2E-ACCEPT-OOO-0001`, `S2-E2E-REJECT-PARSE-0001`) are
byte-identical. The replacement ciphertext was produced BY THE REFERENCE IMPLEMENTATION (driving
`suite2.send_pq_reseed` through the actor as the originating peer, role B), not re-derived by the
tooling. Runner: `e2e_recv 4/4`. Regenerator + proof:
`docs/governance/evidence/NA-0625_e2e_recv_vector_regen.py` / `..._proof.json.txt`.
Root cause of the miss filed as WF-0014. It was the ONLY pinned `flags != 0` frame outside the two
originally named files (exhaustively scanned); `boundary_vectors` is 4/4 because it pins no wire
bytes — its actor constructs the frame at run time.

**All 15 suite2 vector runners are green.**

## Harness (`tools/actors/refimpl_actor_rs`)
- New op `suite2.recv_pq_adv` drives the authenticated ADV receiver, reusing the existing tamper
  plumbing (`tamper: none|body|header`) and adding `mac: ok|missing|corrupt` plus an optional
  foreign `hdr_key_hex` (the spoofed/planted-ADV case).
- `suite2.boundary.run` gains `message.hdr_key: nhk|hk` (default `nhk`): it now seals the PQ-CTXT
  boundary header under the NHK derived from the state root, exactly as the sender does; `hk`
  constructs the pre-NA-0625 downgrade frame. `header_key` is exported from the refimpl for this.

## qsc end-to-end (`qsl/qsl-client/qsc/tests/handshake_mvp.rs`, over a REAL A/B handshake)
12. `scka_e2e_advertise_reseed_roundtrip_over_real_handshake` (UPDATED) — now asserts the
    `[ADV, reseed]` ONE-PACK round trip: Bob's advertisement shares the pack with his reseed
    (`qsp_scka_adv dir=send` and `qsp_pq_reseed dir=send` in the same send), Alice authenticates the
    in-pack advertisement (`qsp_scka_adv dir=recv ... auth=ok`) and then applies the reseed, and NO
    `ratchet_skip_store` marker fires (the chain-consumed ADV leaves no receive-chain gap —
    the mkskipped-empty proof on the real client). GREEN.
13. `scka_e2e_spoofed_adv_injection_rejected_never_tracked` (NEW) — a relay-inbox injector plants a
    syntactically valid advertisement built under FOREIGN session keys directly into Bob's inbox.
    The receive FAILS CLOSED (`qsp_scka_adv ... ok=false`, never `auth=ok`); nothing is tracked; and
    the real conversation continues — Bob's subsequent reseed still targets Alice's REAL
    advertisement (`target_id=1`), which a tracked planted id would have broken (both by
    encapsulating to the attacker key and by blocking Alice's later advertisements on
    monotonicity). GREEN.
14. `scka_e2e_unauthentic_reseed_header_rejected_no_mutation_then_recovers` (NEW) — a reseed
    boundary whose header does not authenticate under the receiver's CURRENT NHK is rejected
    fail-closed; the in-pack ADV that precedes it still authenticates; NO state mutation occurs on
    the reject (re-delivering the INTACT frame is then accepted and the post-reseed schedule works
    in both directions). The true HK-downgrade frame — constructed with the session's real header
    keys, which a client-level test cannot reach by design — is byte-pinned at the conformance level
    in `S2-RECV-PQRESEED-REJECT-HK-DOWNGRADE-0001`. GREEN.
15. `scka_e2e_pq_pcs_healing_survives_dh_ratchet_over_real_handshake` (UPDATED warm-up only, the
    headline assertion unchanged) — still green under NHK.
16. `scka_e2e_rolled_back_session_blob_fails_closed` (unchanged) — G2 rollback still fails closed.
17. `suite2_runtime_equivalence_na0198` (unchanged) — the seed-model runtime equivalence still
    passes BYTE-FOR-BYTE: seed sessions are gated out of both SCKA arms, `send_wire` is untouched,
    and `recv_wire`'s `flags == 0` path is untouched.
18. `dh_ratchet_e2e_*` (unchanged) — the DH-boundary and bounded-fallback proofs still green.

## qsc intercept (the authenticated upgrade)
`qsp_unpack`'s `FLAG_PQ_ADV` arm now mirrors the CTXT arm: gate on `qsp_scka_enabled` (seed sessions
never reach it), INJECT the canonical root (`recv.rk := dh.rk`), drive `recv_wire` (→ `recv_pq_adv`,
passing the SCKA store's `peer_adv_max_seen` as the caller-owned watermark), ADOPT the returned
state, then persist the **session state FIRST and the SCKA store SECOND**. G2 pin: a crash between
the two loses only an UNTRACKED peer advertisement (bounded by the peer's `T_pq` re-advertise) and
can never break the chain, accept a replay, or roll back consumed-monotonicity. Every reject path
emits the reject marker and persists NOTHING. The `qsp_scka_adv` recv marker gains `auth=ok`.
The CTXT arm additionally mirrors the send half of the reseed onto the receiver (ENG-0030).
The `qsp_pack` ADV/reseed pack-exclusion rule is REMOVED.

## Formal (Operator Decision 4)
`formal/model_suite2_root_composition_bounded.py` — a two-party bounded explorer over
{DH boundary, PQ reseed, ADV} x {send, recv}, all in-order interleavings to depth 6 (15,494 states,
23,886 transitions) plus 6 directly-asserted regression shapes. Invariants: root convergence
(`A.rk == B.rk`, and per party `recv.rk == dh.rk` — would have caught the NA-0624 dh.rk-sync bug
pre-implementation); PQ healing survives a subsequent DH boundary; chain continuity under
chain-consume (`ns == peer.nr` per direction; `mkskipped` empty in order); send/receive schedule
coherence in both directions after a reseed receive (the ENG-0030 finding, invariant 4); and
reject ⇒ no mutation, exercised against a spoofed header, a tampered body, an out-of-order frame,
the HK-downgrade boundary frame, an ADVAUTH MAC under a foreign root, and a non-monotonic ADV.
Wired into `formal/run_model_checks.py` (CI `formal.yml`); runs in ~1.5 s. Crypto is abstracted to
injective tuple hashes: this is an agreement/coherence model, NOT a secrecy proof — the ProVerif
composition model remains ENG-0028's own lane.

## Gates
`cargo fmt --all -- --check` CLEAN; `cargo build --workspace --all-targets` (WF-0013) CLEAN;
`cargo clippy --all-targets -- -D warnings` CLEAN on the three crates this lane touches
(`quantumshield_refimpl`, `refimpl_actor`, `qsc`) — the workspace-wide invocation fails on a
PRE-EXISTING, out-of-scope lint in `apps/qshield-cli/tests/na_0318_qshield_ack_commit.rs:150`
(`needless_borrow`, clippy 1.95.0; file untouched by this lane, `apps/**` not in the allowed
mutation paths, and no CI workflow runs clippy) — reported, not fixed; `cargo metadata --locked` OK;
`cargo audit` no findings (1159 advisories, 375 dependencies); `python3 formal/run_model_checks.py`
OK. Refimpl suite 112/112. Full `cargo test -p qsc` exit 0.

## Merge status
All gates green; the STOP was resolved by Operator Decision 5. Impl PR, merge, post-merge
verification, Phase-7 successor triage, and the D-1246 closeout proceed.
