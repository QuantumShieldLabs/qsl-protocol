# NA-0624 — ENG-0012 Stage 2b: qsc SCKA wiring (advertise + reseed cadence + persistence) — Test Plan

Directive: QSL-DIR-2026-07-08-561 (D561). Decision: D-1243. Closes ENG-0012 (the P1).

## Objective
Prove the Stage-2a SCKA sender is wired into the real qsc send/receive path end to end, reusing the
frozen refimpl semantics exactly, with crash-safe + rollback-detected SCKA persistence, WITHOUT
changing the non-PQ message path, any refimpl semantics, or the seed-model runtime equivalence.

## New end-to-end vectors (qsc `tests/handshake_mvp.rs`, over a REAL A/B handshake)
1. `scka_e2e_advertise_reseed_roundtrip_over_real_handshake` — advertise (control envelope) ->
   track -> PQ reseed mid-conversation in BOTH directions -> both decrypt; a DH boundary rides the
   PQ-advanced root between the reseeds (proves the dh.rk adoption live on both sides). GREEN.
2. `scka_e2e_pq_pcs_healing_survives_dh_ratchet_over_real_handshake` — THE HEADLINE: a session
   snapshot captured before Alice's reseed (containing the root, chains, her DH PRIVATE key, her
   advertised ML-KEM secrets, and Bob's advertised PUBLIC key) cannot decrypt Bob's
   post-reseed-post-DH-boundary message: the only secret outside the snapshot is the ML-KEM shared
   secret encapsulated to BOB's receive key. No classical ratchet occurs between the snapshot and
   the target message that the snapshot could not classically follow. GREEN.
3. `scka_e2e_rolled_back_session_blob_fails_closed` — G2: rolling Bob's session blob back to the
   pre-reseed snapshot fails closed on load (`session_rollback_detected`; the monotonic side-record
   has advanced), preventing re-consumption of a one-time target across a restore. GREEN.

## New co-located unit tests (`qsc/src/protocol_state/mod.rs::scka_tests`)
- `scka_section_roundtrips`; `scka_default_is_empty_section` (a non-advertising session persists
  `scka_len == 0`); `scka_decode_fails_closed` (truncation at EVERY prefix length, trailing bytes,
  oversize advkey count, non-boolean flag — deterministic rejects);
- `v3_plaintext_join_split_roundtrips_and_reads_legacy` (v3 round trip; v3-empty offset = 17+4;
  legacy v2 and v1 split with defaults; malformed v3 fails closed);
- `scka_rollback_guard_rejects_regressions_and_accepts_progress` (peer_max / local_next /
  peer_adv_max / tombstone-superset matrix);
- `scka_advkey_cap_evicts_deterministically_and_tombstones` (CAP=4; consumed-first then lowest-id;
  eviction tombstones; consumption erases the secret). GREEN.

## Regression gates (must be UNCHANGED green)
- `suite2_runtime_equivalence_na0198` — byte-for-byte (seed-model session: SCKA gated off,
  `scka_len == 0`, identical QS2S snapshot + identical wire). GREEN.
- `na_0302` / `na_0304` / `na_0313` handshake tests (v3 snapshot-offset helper updated: skip
  `scka_len + scka`). GREEN.
- `handshake_fs_identity_compromise_cannot_decrypt_recorded_message` — updated for the
  advertisement control envelope (the recorded message is the LAST of [adv, message]). GREEN.
- `dh_ratchet_e2e_*` (NA-0622) — unchanged assertions, now exercising ADV control envelopes, the
  bounded control re-pull in `receive`, AND the trigger-persistence enabling fix (the transport
  deliver path now persists the cleared ratchet-on-reply flag + fallback counters, making the
  documented D-1239 cadence live; previously every post-receive send ratcheted). GREEN.
- `dh_ratchet_e2e_bounded_fallback_fires_over_real_handshake` (NEW — pins the enabling fix
  directly: four consecutive sends with no reply, the fourth is FORCED to a DH boundary with
  `reason=fallback`, and the peer decrypts the whole run including across the boundary). GREEN.
- Full qsc suite: 146 test targets, 584 tests passed, 0 failed (final full run; the two test binaries touched after that run — the lint-fix and new-fallback-vector files — re-proven green on the final tree).
- Full refimpl suite (`quantumshield_refimpl` — UNTOUCHED sources): 101 tests passed, 0 failed (sources untouched).

## Validation gates (WF-0013)
- `cargo fmt --check`: clean
- `cargo build --workspace --all-targets`: clean
- `cargo clippy -p qsc --all-targets -- -D warnings`: clean (two pre-existing main lints fixed in-scope)
- `cargo metadata --locked`: clean
- `cargo audit`: clean (1159 advisories, 375 dependencies, no findings)
- Scope guard (diff touches only D561-allowed paths): clean (all changed paths within the allowlist)

## Explicitly out of scope / flagged
- ADV-header authentication (the frozen receiver has no ADV path) — flagged deviation, deferred to
  the spec-alignment successor candidate with the §8.5.1 NHK item (D-1243).
- Combined DH+PQ boundary (Operator Decision 1 — future refimpl lane).
- Cover traffic / size+cadence obfuscation (ENG-0022; DOC-G5-004 §3.1 row accepted+documented).
