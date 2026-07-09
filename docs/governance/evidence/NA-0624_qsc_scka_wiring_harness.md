# NA-0624 — qsc SCKA wiring: implementation evidence

Directive: QSL-DIR-2026-07-08-561 (D561). Decision: D-1243. Base: main `0a21f0ea`.

## What landed (surfaces)
- `qsl/qsl-client/qsc/src/protocol_state/mod.rs` — `SckaLocalState` (bounded advertised-key store
  CAP=4 + deterministic consumed-first/lowest-id eviction; peer advertisement; `local_next_adv_id`;
  `peer_adv_max_seen`; cadence counters); fail-closed section codec; v3 plaintext
  `QTRG(4)+trigger(13)+scka_len(u32 LE)+scka+QS2S` (QS2S FROZEN; v2/v1 migrate);
  `qsp_scka_load`/`qsp_scka_store` (read-modify-write against the stored blob);
  `qsp_session_store_with_trigger[_scka]` (SCKA-preserving); G2 monotonic side-record
  (`<peer>.scka.json`, ids only — the qsc mirror of the actor `dur_scka`) merge-updated on store,
  checked on load (`session_rollback_detected`, fail-closed); constants `QSP_PQ_RESEED_N=8`,
  `QSP_PQ_RESEED_T_SECS=3600`, `QSP_SCKA_ADVKEY_CAP=4`; co-located `scka_tests`.
- `qsl/qsl-client/qsc/src/main.rs` — `qsp_pack`: SCKA policy layer (advertise-due: no live key /
  consumed / rotation; reseed-due: fresh unconsumed peer_adv AND first-immediately-then-N_pq/T_pq;
  gated OFF for the self-DH seed model like the DH ratchet); ADV as a CONTROL pre-envelope on the
  current chain; reseed via frozen `send_pq_reseed` carrying the user payload; single fail-closed
  SCKA store at the pack success boundary. `qsp_unpack`: `FLAG_PQ_ADV` intercepted BEFORE
  `recv_wire` (frozen `track_peer_adv` validation; control outcome, no app payload);
  `FLAG_PQ_CTXT` -> local advkey lookup -> `PqKem768::decap` -> `recv_wire(.., Some(ss),
  Some(target_id))` (frozen `apply_pq_reseed` enforces monotonic/one-time/tombstone) -> ADOPT
  `dh.rk := recv.rk` -> consume+erase the local key before releasing plaintext.
- `qsl/qsl-client/qsc/src/transport/mod.rs` — control pre-envelope push before the main message;
  bounded control re-pull in `receive` (RECV_CONTROL_ROUNDS_MAX=4) so `--max N` yields up to N app
  messages; ENABLING FIX: `finalize_send_commit` now persists the qsp_pack trigger on the deliver
  path (the NA-0622 cleared ratchet-on-reply flag + N/T fallback counters previously never landed
  on the main transport path — every post-receive send ratcheted and the co-scheduled reseed could
  never fire; outbox replay still preserves).
- `qsl/qsl-client/qsc/src/tui/controller/commands/relay.rs` — control-message skip (state
  committed; no file write / message line).
- Tests: `tests/handshake_mvp.rs` `scka_e2e_*` (3 new; see testplan) + FS-test envelope-count
  update; v3 snapshot-offset helper updates in `suite2_runtime_equivalence_na0198.rs`,
  `na_0302_*.rs`, `na_0304_*.rs`, `na_0313_*.rs`.
- Docs: DOC-G5-008 Stage-2b update note; DOC-G5-004 §3.1 PQ-ADV/PQ-CTXT observable row.

## Frozen-surface conformance
- `tools/refimpl/**` UNTOUCHED (git diff shows no refimpl path). `send_pq_advertise`/
  `send_pq_reseed`/`track_peer_adv`/`apply_pq_reseed`/ratchet semantics reused exactly.
- No KDF/AEAD/KEM primitive change; no normative DOC-CAN change; no Cargo/.github/.claude change.
- Runtime equivalence: seed-model sessions are SCKA-gated OFF (`dhr == dhs`) -> `scka_len == 0`
  -> identical QS2S snapshot bytes + identical wire. Confirmed green (suite2_runtime_equivalence_na0198 in the final full run).

## Key design points (why this is safe)
1. ADV is a CONTROL message: the frozen receiver has no ADV body decrypt path (recv_wire rejects
   FLAG_PQ_ADV), so advertisements never carry user payload; the peer's OOO machinery skips the
   consumed chain step. Fail-closed ordering: the advertised ML-KEM SECRET is durable inside the
   AEAD blob before the advertisement envelope can leave `qsp_pack`.
2. Reseed consumes the peer advertisement durably BEFORE the reseed wire exists — a crash cannot
   re-target a consumed (one-time) advertisement, which would desynchronise the root.
3. The root composition on CTXT receive has TWO halves, both caller-side (the NA-0623
   carry-overs): (a) INJECT `recv.rk := dh.rk` before the frozen receiver runs — a DH boundary
   advances only `dh.rk` and the frozen reseed SENDER derives from `session_root` (dh.rk when
   live), so without the injection the two parties derive `KDF_RK_PQ` from different roots and
   desynchronise (header-auth failure on the first post-reseed message); (b) ADOPT
   `dh.rk := recv.rk` afterwards so a later DH boundary reads the PQ-hardened root instead of
   wiping it. Proven live by a DH boundary riding the PQ-advanced root in the round-trip vector.
3b. An advertisement never shares a pack with a reseed: the ADV control envelope consumes a
   send-chain slot the receiver skips without consuming; a NORMAL main message heals the n-gap
   via the OOO machinery and a DH boundary abandons the epoch, but the frozen reseed receiver is
   strict-in-order (n == nr), so a due advertisement defers to the next send.
4. G2 rollback: the side-record holds ids only (no key material); regression of
   `peer_max_adv_id_seen` / `local_next_adv_id` / `peer_adv_max_seen` / tombstone-superset fails
   the session load closed.

## Flagged deviations (deferred, with rationale — carried to the spec-alignment candidate)
1. UNAUTHENTICATED ADV TRACKING (length + monotonicity only): authenticating the ADV header needs
   receiver-side trial-open machinery (same work area as the deferred §8.5.1 NHK reconciliation).
   Bounding: a planted-key reseed still mixes into RK via KDF_RK_PQ (classical security
   unaffected; PQ layer at worst = "no reseed"); a max-adv_id injection is a tracking DoS.
2. Outbox replay carries only the main message; a dropped ADV degrades to the classical status quo
   until rotation (T_pq) re-advertises.
3. Message-path blob decrypt passes increased (trigger + SCKA loads; NA-0622 precedent) — a
   consolidation candidate, not a correctness issue.

## Validation
- Full qsc suite: 146 test targets, 584 tests passed, 0 failed (final full run; the two test binaries touched after that run — the lint-fix and new-fallback-vector files — re-proven green on the final tree). refimpl suite: 101 tests passed, 0 failed (sources untouched). Validation gates: all clean — fmt --check; cargo build --workspace --all-targets (WF-0013); clippy -p qsc --all-targets -D warnings (after fixing two PRE-EXISTING main lints surfaced by the newer stable toolchain, both in D561-scoped qsc files); cargo metadata --locked; cargo audit (1159 advisories, 375 dependencies, no findings). Scope guard: all changed paths within the D561 allowlist. PR + merge SHA recorded at the D-1244 closeout.
