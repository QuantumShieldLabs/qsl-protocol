Goals: G1 (primary), G2, supports G3, G4, G5

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-08

# NA-0620 — DH-Ratchet State Plumbing Test Plan (Stage 1a, no behavior change)

## Scope

Validation for NA-0620 (ENG-0012 Stage 1a) under directive QSL-DIR-2026-07-08-557 (D557).
Refimpl suite-2 state/establishment/persistence + the qsc handshake→establishment call site +
tests. No message-path/`send_boundary`/static-`rk`/KDF/AEAD/nonce/wire-format change; no
qsl-attachments/qsl-server change; no dependency/workflow change.

## Required Markers

- NA0620_D1233_CONSUMED_OK
- NA0620_D1234_CONSUMED_OK
- NA0620_FRESH_STARTUP_PROOF_OK
- NA0620_D1235_ABSENT_BEFORE_IMPLEMENTATION_OK
- NA0620_DESIGN_LOCK_STRUCT_PLACEMENT_SNAPSHOT_V2_NO_MESSAGE_PATH_OK
- NA0620_DH_STATE_ADDED_TO_SESSION_STATE_OK
- NA0620_ESTABLISHMENT_POPULATES_DHS_PUB_DHR_RK_OK
- NA0620_DH_SELF_PRIV_THREADED_FROM_QSC_HANDSHAKE_OK
- NA0620_SNAPSHOT_V2_ROUNDTRIP_PRESERVES_DH_OK
- NA0620_RESTORE_REJECTS_NON_V2_VERSION_OK
- NA0620_NO_MESSAGE_PATH_CHANGE_RUNTIME_EQUIVALENCE_OK
- NA0620_STATIC_RK_BOOTSTRAP_UNTOUCHED_OK
- NA0620_FULL_REFIMPL_AND_TOUCHED_QSC_SUITES_GREEN_OK
- NA0620_FMT_CLIPPY_METADATA_LOCKED_GREEN_CARGO_UNCHANGED_OK
- NA0620_ENG0012_STAGE1A_DONE_OK
- NA0620_PRIVATE_MATERIAL_SCAN_OK

## Validation Plan (class-only)

1. Verify fresh operator startup proof (lane NA-0620) and main health; D-1233/D-1234 consumed
   once each and Accepted; D-1235 absent before implementation.
2. Design-lock: DH state placement on `Suite2SessionState`; snapshot v2; `init_from_base_handshake`
   signature unchanged (actor/handshake tests untouched); the crash-restart vector is operational
   (not byte-pinned); no message-path change; static-`rk` bootstrap untouched.
3. Change: `Suite2DhRatchetState` added and populated at establishment; `set_dh_self_priv` threads
   the qsc handshake's X25519 ephemeral private key; snapshot v2 (de)serializes the DH state and
   fails closed on non-v2.
4. Tests: `snapshot_roundtrip_preserves_dh_ratchet_state`, `restore_rejects_non_v2_version` pass;
   `suite2_runtime_equivalence_na0198` passes (persisted state matches the refimpl byte-for-byte
   — no message-path drift); handshake suites pass with the threaded DH private key.
5. Regression: full `quantumshield_refimpl` suite (72 lib + integration) green; the touched qsc
   suites green.
6. Build gates: `cargo fmt --check`, `cargo clippy` (refimpl all-targets + qsc lib) `-D warnings`,
   `cargo build --locked`, `cargo metadata --locked`; Cargo.toml/lock unchanged.
7. Private-material scan on added lines (DH secrets are not printed; class-safe).

## Result

`DH_RATCHET_STATE_PLUMBING_NO_BEHAVIOR_CHANGE`. ENG-0012 Stage 1a done; Stage 1b (DH-ratchet
behavior) = NA-0621. No message-path/wire/crypto/KDF/AEAD change; no dependency/workflow change.
Delivers no post-compromise security on its own.
