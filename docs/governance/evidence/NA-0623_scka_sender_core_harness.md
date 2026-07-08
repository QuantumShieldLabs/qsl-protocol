Goals: G1 (primary), G2, G3, G4, G5

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-08

# NA-0623 Evidence — ENG-0012 Stage 2a: Suite-2 SCKA sender core (advertisement + PQ reseed) + both-sides RK advance

Directive QSL-DIR-2026-07-08-560 (D560). Decision D-1241 (implementation); D-1242 (closeout).
All commands run in the lane workspace at the pinned toolchain; class summaries only.

## 1. Design-lock conclusion (STOP condition NOT triggered)

At `1251eb2f` the Phase-2 design-lock established that a clean, spec-faithful SCKA sender is
achievable with NO snapshot bump, NO non-boundary-path change, and NO wire-format change:

- **The SCKA target sets already persist.** `recv.{known,consumed,tombstoned}_targets` and
  `peer_max_adv_id_seen` exist in `Suite2RecvWireState` and round-trip through snapshot v2 — no
  bump. The KEM key material is **caller-side**: `apply_pq_reseed` takes `pq_epoch_ss` as an input
  (the caller decapsulates), so the sender's advertised-key store (`advkeys`), `local_next_adv_id`,
  and the peer's advertised public key live in the caller (the interop actor / a test for Stage 2a;
  qsc for Stage 2b). The refimpl SCKA sender is therefore **pure functions**.
- **No wire-format change.** `parse.rs` already parses `FLAG_PQ_ADV` (`pq_adv_id` + `pq_adv_pub`,
  1184 B) and `FLAG_PQ_CTXT` (`pq_target_id` + `pq_ct`, 1088 B); both require `FLAG_BOUNDARY`. The
  sender constructs the existing format.
- **The security re-scope (D560 AMENDMENT).** The existing PQ-reseed receiver
  (`recv_boundary_in_order` -> `apply_pq_reseed`) absorbed `pq_epoch_ss` into the directional PQ
  chains but NEVER advanced the root `RK` (DOC-CAN-003 §8.5.3 steps 5+7 unimplemented). Because the
  classical DH ratchet (`send_boundary`/`recv_dh_boundary`, NA-0621) reinitialises `CK_pq` from `RK`
  (§8.5.2 step 6), the next DH reply would WIPE the PQ hardening. Fix BOTH sides: advance
  `RK := KDF_RK_PQ` + recompute `HK` on the receiver AND the new sender. DOC-CAN-003 §8.1 defines
  ONE `RK` per session; the refimpl stores it in `recv.rk` (PQ path) and `dh.rk` (DH ratchet), and
  the reseed writes the advanced root to BOTH slots so the DH ratchet carries the PQ hardening.
  `apply_pq_reseed`'s CTXT-validation semantics (monotonicity/one-time/tombstone/target/ct-len) are
  UNCHANGED.

## 2. Derivations implemented (spec-cited)

- `KDF_RK_PQ` (§3.3.3): `RK' = KMAC32(RK, "QSP5.0/RKPQ", pq_ss || [0x01])` — the same derivation the
  base handshake uses for the initial PQ mix-in (`suite2::establish`).
- `KDF_PQ_RESEED` (§3.3.6): `ct_hash = H(pq_ct)[0:32]`;
  `ctx = "QSP5.0/SCKA/CTXT" || u32be(pq_target_id) || ct_hash || pq_epoch_ss`;
  `CK_pq_seed_{A2B,B2A} = KMAC32(RK_old, "QSP5.0/PQSEED/{A->B,B->A}", ctx)`. Factored into a shared
  `kdf_pq_reseed_seeds` helper reused by `apply_pq_reseed` (byte-identical output) and the sender, so
  both parties converge. §3.3.6 normative ordering (CAT-S2-KDF-001): seeds from `RK_old` FIRST, then
  `RK := KDF_RK_PQ`, then apply the directional `CK_pq`.
- `send_pq_advertise` (§8.5.4 / DOC-CAN-004 §3.1): records `pq_adv_id` in `known_targets`; frames a
  `FLAG_PQ_ADV | FLAG_BOUNDARY` message carrying `pq_adv_id || pq_adv_pub`.
- `send_pq_reseed` (§8.5.3 sender side / DOC-CAN-004 §3.3): the exact structural mirror of
  `recv_boundary_in_order`'s PQ path — seeds from `RK_old`, advance `RK`, recompute `HK_s`/`HK_r`,
  replace the directional `CK_pq_send`/`CK_pq_recv`, frame `FLAG_PQ_CTXT | FLAG_BOUNDARY`
  (`pq_prefix = pq_target_id || pq_ct`) under the PRE-reseed key schedule, and write the new root to
  both `recv.rk` and `dh.rk`.
- `track_peer_adv` (DOC-CAN-004 §3.2): peer-advertisement monotonicity + public-key-length checks.

**Refimpl-vs-spec note (flagged, not fixed):** the frozen CTXT receiver `recv_boundary_in_order`
opens the PQ-CTXT boundary header under the ordinary `HK_r`, not the §8.5.1 `NHK`; the sender
mirrors `HK_s` so the round-trip holds. Reconciling the PQ-CTXT boundary header to the §8.5.1 `NHK`
rule would change the frozen receiver's semantics (out of scope here) — recorded for Stage 2b / a
spec-alignment lane.

## 3. Proof (co-located refimpl integration tests + conformance vectors)

`cargo test -p quantumshield_refimpl --test suite2_scka_sender` → 5 passed / 0 failed:
- `reseed_round_trip_sender_to_apply_pq_reseed_decrypts_and_converges` — the round-trip: a receiver
  advertises a receive key, the sender encapsulates to it and calls `send_pq_reseed`, and the frozen
  `apply_pq_reseed` path (via `recv_wire`) decrypts the body; the directional PQ seed and the
  advanced root converge on both parties (`send.ck_pq == recv.ck_pq_recv`,
  `recv.rk == KDF_RK_PQ(RK_old, ss)` on both, `send.hk_s == recv.hk_r`).
- `pq_pcs_healing_survives_dh_ratchet` — **THE HEADLINE**: after a PQ reseed (root R0 -> R1), a
  subsequent classical DH ratchet (root R1 -> R2) is sealed under `NHK` derived from R1; a state
  snapshot captured BEFORE the reseed (root R0) CANNOT open that DH boundary (`recv_dh_boundary` ok
  == false), while the live receiver can (ok == true, body decrypts). The PQ epoch secret advanced
  the root the DH ratchet reads, so the post-quantum hardening is carried forward permanently — this
  is the whole point of the D560 re-scope (without the root advance the pre-reseed snapshot would
  succeed).
- `reseed_sender_rejects_are_deterministic_and_no_mutation` — bad ciphertext length, bad
  shared-secret length, and an unset send chain reject deterministically; the retained state is
  unchanged.
- `reseed_replay_is_rejected_one_time` — a replayed reseed targeting an already-consumed/tombstoned
  advertised key is rejected by the frozen `apply_pq_reseed` path with no state mutation.
- `advertise_frames_parseable_boundary_and_track_enforces_monotonicity` — the advertisement frames a
  parseable `FLAG_PQ_ADV` boundary and records the id; peer-ADV monotonicity + bad-pub-len reject.

Conformance vectors (harness ops `suite2.send_pq_advertise` / `suite2.send_pq_reseed`):
- `scripts/ci/run_suite2_scka_logic_vectors.py --actor target/debug/refimpl_actor` → **14 / 14**
  (the 8 pre-existing CAT-SCKA-LOGIC-001 vectors + 6 new byte-pinned sender vectors:
  advertise-accept, reseed-accept, and 4 rejects — bad-pub-len, non-monotonic advertise, bad-ct-len,
  bad-ss-len). Appended only; the existing 8 vectors are byte-identical.
- `scripts/ci/validate_suite2_vectors.py` → schema OK.

Regression / build gates (all green — the STOP condition on non-PQ-path / `apply_pq_reseed`
regression did NOT trigger):
- Frozen vector sets UNCHANGED: `run_suite2_pqreseed_vectors.py` 5/5 (`apply_pq_reseed`),
  `run_suite2_boundary_vectors.py` 4/4, `run_suite2_scka_kem_vectors.py` 5/5,
  `run_suite2_kdf_vectors.py` 6/6.
- `cargo test -p quantumshield_refimpl` — 75 lib + all integration suites pass (non-boundary and
  `apply_pq_reseed` paths unchanged; the receiver RK-advance is invisible to the existing
  boundary/reseed conformance ops, which do not emit `rk`).
- `cargo build --workspace --all-targets` — clean (WF-0013; the actor builds).
- `cargo fmt --check` on the changed crates — clean; `cargo clippy --all-targets -- -D warnings` on
  the changed crates — clean; `cargo metadata --locked` — clean; Cargo unchanged.

## 4. Boundary and claim

Mutations: `tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs` (KDF_RK_PQ + receiver
RK-advance + `recv_wire` propagation + the SCKA sender), `.../suite2/scka.rs` (extracted shared seed
helper; `apply_pq_reseed` byte-identical), `.../tests/suite2_scka_sender.rs` (new),
`tools/actors/refimpl_actor_rs/src/main.rs` (two SCKA sender ops),
`inputs/suite2/vectors/qshield_suite2_scka_logic_vectors_v1.json` (+6 vectors, append-only), plus
governance/design docs. No qsc client change; no `apply_pq_reseed` CTXT-validation-semantic change;
no KDF/AEAD/KEM primitive change; no normative DOC-CAN change; no snapshot or wire-format change; no
Cargo/`.github`/`.claude`/hook change; no operator-startup-command execution; no runtime/LAN action.
Research/demo only. This lane delivers the SCKA sender core + both-sides root advance in refimpl
(incl. PQ-PCS surviving a DH ratchet); it is NOT wired into the client and is NOT on its own a
post-quantum, Triple-Ratchet, security-complete, crypto-complete, post-compromise, or
production-ready claim (Stage 2b wires it into the real send path). No endpoint, token, capability,
key, seed, plaintext, ciphertext body, or raw private material is published (ML-KEM secret keys and
shared secrets are struct fields / caller-side, never printed).
