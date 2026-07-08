Goals: G1 (primary), G2, G3, G4, G5

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-08

# NA-0621 Evidence — ENG-0012 Stage 1b-i: Suite-2 DH Ratchet (send+receive) + NHK, refimpl core

Directive QSL-DIR-2026-07-08-558 (D558). Decision D-1237 (implementation); D-1238 (closeout).
All commands run in the lane workspace at the pinned toolchain; class summaries only.

## 1. Design-lock conclusion (STOP condition NOT triggered)

At `1ec1784f` the design-lock established a clean, spec-faithful implementation is achievable
without a wire-format change, a non-boundary-path change, or a PQ-reseed-semantics change:

- **`DH_pub[32]` is already on the wire** for every ratchet message (DOC-CAN-003 §4.3):
  `parse_ratchet_header` extracts `header[0..32]`; `send_wire` writes `st.dh_pub` at the front of
  the header. The current code merely uses the *stored* `dh_pub` rather than ratcheting on it — so
  the DH ratchet is a BEHAVIOR change, not a FORMAT change.
- **The classical DH ratchet was absent on BOTH sides.** `recv_boundary_in_order` performs the
  PQ-reseed (`apply_pq_reseed`, §8.5.3) with 0 X25519/`kdf_rk_dh`/keypair uses; there was no
  DH-ratchet send or receive.
- **No `NHK` existed in Suite-2**, though §8.5.1's boundary-header anti-spoof requires it. `NHK` is
  derived on demand from `RK`, so no stored field is added and the snapshot format is unchanged.
- **A DH-only boundary needs its own path**: `parse_pq_prefix` (the boundary-processing parser)
  requires `FLAG_PQ_CTXT`, so `recv_dh_boundary` handles the DH-only case directly (the PQ path is
  untouched).

## 2. Derivations implemented (spec-cited)

- `KDF_RK_DH` (§3.3.2): `tmp = KMAC256(RK, "QSP5.0/RKDH", dh_out, 64)`; `RK' = tmp[0:32]`,
  `CK_ec0 = tmp[32:64]`.
- Header keys (§3.4/§8.1): `HK/NHK_{A->B,B->A} = KMAC32(RK, "QSP5.0/{HK,NHK}/{A->B,B->A}", [0x01])`,
  mapped to send/recv by role.
- `send_boundary` (§8.5.2): `boundary_hk = NHK_s(pre-boundary RK)`; fresh X25519 keypair;
  `dh_out = X25519(new_priv, DHr)`; `KDF_RK_DH`; reinit `CK_pq_send` from the new `RK`; recompute
  `HK_s`; `PN := Ns`, `Ns := 0`; emit `FLAG_BOUNDARY` with the new `DH_pub`, header under
  `boundary_hk`, body under the new epoch's first message key. The send chain is CREATED by the
  ratchet (so the responder — whose send chain is zero until its first ratchet — can send).
- `recv_dh_boundary` (§8.5.2 + §8.5.1): header MUST decrypt under CURRENT `NHK_r` (anti-spoof),
  else reject; then `dh_out = X25519(DHs_priv, msg.DH_pub)`, `KDF_RK_DH`, reinit `CK_pq_recv`,
  recompute `HK_r`, `DHr := msg.DH_pub`, `Nr := 0`, decrypt the body; state is committed only on
  full success (no mutation on reject).

## 3. Proof (co-located refimpl conformance tests)

`cargo test -p quantumshield_refimpl --lib dh_ratchet` → 7 passed / 0 failed, including:
- `dh_ratchet_two_party_roundtrip_both_directions` — the ratchet fires both directions and
  messages decrypt.
- `dh_ratchet_pcs_healing` — a pre-ratchet epoch-0 snapshot CANNOT decrypt a post-ratchet message
  once both parties have advanced (post-compromise self-healing).
- `dh_ratchet_no_mutation_on_reject` — a tampered ciphertext is rejected with the state unchanged
  (`snapshot_bytes` equal).

Regression / build gates (all green):
- `cargo test -p quantumshield_refimpl` — 75 lib + all integration suites pass (non-boundary and
  PQ-reseed paths unchanged).
- `cargo build --workspace --all-targets` — clean (WF-0013 applied; the actor builds).
- `cargo fmt -p quantumshield_refimpl --check` — clean; `cargo clippy -p quantumshield_refimpl
  --all-targets -- -D warnings` — clean; `cargo metadata --locked` — clean; Cargo unchanged.

## 4. Boundary and claim

Mutations: `tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs` (DH ratchet + NHK + tests)
plus governance/design docs. No qsc client change; no static-`rk` removal; no PQ-reseed change; no
KDF/AEAD primitive change; no normative DOC-CAN change; no Cargo/workflow/`.claude` change; no
wire-format change; no snapshot change. Research/demo only. This lane delivers the DH-ratchet
crypto core in refimpl; it is NOT wired into the client and is NOT on its own a post-compromise,
Triple-Ratchet, security-complete, or production-ready claim (Stage 1b-ii wires it; Stage 2 adds PQ
reseed). No endpoint, token, capability, key, seed, plaintext, ciphertext body, or raw private
material is published (DH secrets are struct fields, never printed).
