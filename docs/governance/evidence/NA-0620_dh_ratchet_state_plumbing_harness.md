Goals: G1 (primary), G2, supports G3, G4, G5

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-08

# NA-0620 — ENG-0012 Stage 1a: Suite-2 DH-Ratchet State Plumbing

## Summary

NA-0620 implements Stage 1a of the accepted design DOC-G5-008 (ledger ENG-0012, the P1 blocking
the G1/G2 release gates) under directive QSL-DIR-2026-07-08-557 (D557). It makes the Suite-2
session state able to CARRY and PERSIST the DH-ratchet material the Stage-1b send-side ratchet
needs, with **no message-path behavior change**. Refimpl suite-2 state/establishment/persistence
+ the qsc handshake→establishment call site + tests. No `send_wire`/`recv_wire`/message-path
change; no `send_boundary`; the static-`rk` bootstrap is untouched (Stage 1b); no
KDF/AEAD/nonce/wire-format change; no dependency/workflow change.

Result classification: `DH_RATCHET_STATE_PLUMBING_NO_BEHAVIOR_CHANGE`.

## Design-lock (confirmed at `98c21666`)

- The DH keypair material already existed and partly flowed: the qsc handshake retains `dh_sk`
  (local X25519 priv) and `dh_pub`; `init_from_base_handshake` already accepted
  `dh_self_pub`/`dh_peer_pub` and computed `rk`. What the ratchet state LACKED: the local DH
  private key, `DHr`, and a session-level live `RK` (the send/recv structs carry only a fixed
  `dh_pub` nonce input; the recv wire state already carried `rk`).
- The crash-restart conformance vector is operational (the runner captures snapshot bytes at
  runtime and restores them), NOT byte-pinned — so a snapshot version bump is transparent to it.
  This confirmed a clean no-behavior-change plumbing is achievable (the directive STOP condition
  did not trigger).

## Change (plumbing only)

- New session-level `Suite2DhRatchetState { dhs_priv, dhs_pub, dhr, rk }` added as a `dh` field
  on `Suite2SessionState` (`suite2/ratchet.rs` for the struct; `suite2/state.rs` for the field).
- `establish.rs` populates `dh` (`dhs_pub` = local pub, `dhr` = peer pub, `rk` = the computed
  root; `dhs_priv` left zero for callers that do not ratchet). `init_from_base_handshake`'s
  signature is UNCHANGED, so the interop actor and the handshake tests are untouched.
- `Suite2SessionState::set_dh_self_priv` — the qsc handshake supplies its retained X25519
  ephemeral private key post-establishment (`hs_build_session` gains a `dh_self_priv` param;
  both call sites thread it — the initiator converts `pending.dh_sk` fail-closed on length).
- Snapshot format bumped to **v2**: the DH state is serialized after `mkskipped`; restore
  accepts v2 only and fails closed on v1/any other version (pre-release — eliminate legacy per
  the PROJECT_CHARTER design tenet). The seed-fallback session (`protocol_state/mod.rs`) derives
  its DH material from the existing seed labels (plus `QSC.QSP.DH.PRIV`).

## Why there is no behavior change

- No message-path code reads the `dh` state in Stage 1a; `send_wire`/`recv_wire`, all nonces,
  headers, and wire bytes are byte-for-byte unchanged (proven by the runtime-equivalence test).
- The static-`rk` bootstrap (`qsp_activate_*`) is untouched — it is removed in Stage 1b.
- `dhs_priv`/`rk` are secrets persisted only through the vault-encrypted snapshot, consistent
  with the existing `ck_ec`/`ck_pq` posture.

## Tests

New (`suite2/state.rs`): `snapshot_roundtrip_preserves_dh_ratchet_state` (the DH state
round-trips; snapshot bytes are a fixed point) and `restore_rejects_non_v2_version` (v1 and v3
fail closed). Regression: full `quantumshield_refimpl` suite green (72 lib + all integration);
the touched qsc suites pass — `session_state_at_rest`, `protocol_state_contract_na0217c`,
`qsp_qse_onwire`, `suite2_runtime_equivalence_na0198` (the runtime-equivalence check confirms the
persisted state matches the refimpl byte-for-byte, i.e. no message-path drift), and the handshake
suites (`handshake_mvp`, `handshake_security_closure`, `na_0304`, `na_0313`) exercising the
threaded DH private key. `cargo fmt --check` clean; `cargo clippy` (refimpl all-targets + qsc lib)
`-D warnings` clean; `cargo metadata --locked` green; Cargo unchanged.

## Claim boundary

Research/demo. No public/production/security-complete/crypto-complete/post-compromise/
Triple-Ratchet claim. This lane is plumbing only; it does NOT implement the DH ratchet behavior
(Stage 1b) and delivers no post-compromise security on its own.
