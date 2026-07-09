# NA-0626 — ENG-0024 + ENG-0026 (+ ENG-0030 structural, ENG-0031 rider): RK unification + combined DH+PQ boundary — Test Plan

Directive: QSL-DIR-2026-07-09-563 (D563). Decision: D-1247. Closes ENG-0024, ENG-0026, ENG-0030,
ENG-0031.

## Objective

Prove the Suite-2 root-key coherence is STRUCTURAL (one `rk`; the wire-level ops root-explicit;
the qsc INJECT/ADOPT and send-half-refresh dances unrepresentable), the session-level SCKA
receive entry points return a FULL coherent schedule, the combined DH+PQ boundary round-trips
with the design-locked DH-first-then-PQ composition, the QS2S v2→v3 migration fails closed with
a distinct marker (no migration path), and the conformance-vector artifact matches the
Operator-Decision-5 byte-scanned split EXACTLY — with no wire FORMAT change, no parse.rs hook,
no KDF/AEAD/KEM primitive change, and no normative DOC-CAN change beyond the single ENG-0031
sentence.

## Refimpl unit tests (`src/suite2/state.rs`, `src/suite2/ratchet.rs`)

1. `restore_rejects_non_v3_version` (REWRITTEN from `restore_rejects_non_v2_version`) — a valid
   magic with version 1/2/4 fails closed with the DISTINCT static
   `unsupported suite2 snapshot version`, deterministically, input bytes never mutated; a
   synthesized v2 header in particular gets the distinct (not generic) marker; a bad magic keeps
   the generic `bad suite2 snapshot`. GREEN.
2. `snapshot_roundtrip_preserves_dh_ratchet_state` — the v3 layout round-trips the single
   session root and the DH key material (`dh.rk` no longer exists). GREEN.
3. Pre-existing restore cap/truncation/SCKA-persistence tests — unchanged semantics on v3 bytes.
   GREEN.
4. The 8 NA-0625 ADV-receive unit tests — re-threaded to the root-explicit `recv_pq_adv`
   (`adv_recv_state` returns `(state, rk)`); same assertions. GREEN.
5. Pre-existing boundary/nonboundary/send_wire tests — unchanged (the boundary bundle keeps its
   transient `rk` slot). GREEN.

## Refimpl integration tests

`tests/suite2_combined_boundary.rs` (NEW, ENG-0026):
6. `combined_boundary_round_trip_converges_on_dh_then_pq_composition` — one 0x0006 frame with a
   FRESH caller-supplied `DH_pub`: plaintext round-trips; both roots converge on the
   INDEPENDENTLY recomputed `RK_final = KDF_RK_PQ(KDF_RK_DH(R0, dh_out), ss)`; full directional
   schedule coherence (both header-key directions, both PQ chains); receiver lands at `nr == 1`
   of the new epoch with `mkskipped` empty and the target consumed one-time; A's next normal
   message (n=1) arrives strictly in order. GREEN.
7. `combined_boundary_healing_survives_subsequent_dh_ratchet` — a pre-combined snapshot (root
   R0) CANNOT open a post-combined DH boundary; the live receiver can (hybrid healing carried in
   the root lineage). GREEN.
8. `combined_boundary_rejects_out_of_order_n_without_mutation` — a combined header claiming
   n=1 (sealed at the n=0 nonce so it opens) rejects `REJECT_S2_BOUNDARY_NOT_IN_ORDER`;
   snapshot-byte-identical state. GREEN.
9. `combined_boundary_anti_spoof_rejects_without_mutation` — zero `DH_pub` ⇒
   `REJECT_S2_HDR_AUTH_FAIL`; fresh `DH_pub` with `dhs_priv` zero ⇒
   `REJECT_S2_LOCAL_UNSUPPORTED`; 0x0007 (ADV|CTXT|BOUNDARY) ⇒ `REJECT_S2_LOCAL_UNSUPPORTED`;
   each with no mutation. GREEN.
10. `combined_boundary_replay_rejects_without_mutation` — the accepted combined wire
    re-presented carries a now-CURRENT `DH_pub` (PQ-only discrimination) under the OLD NHK at a
    stale counter: rejected, no mutation. GREEN.
11. `combined_sender_rejects_are_deterministic_and_no_mutation` — bad ct/ss lengths, zero
    caller keypair, missing peer DH key: deterministic fail-closed rejects; retained state
    byte-identical. GREEN.

`tests/suite2_scka_sender.rs`:
12. `reseed_receiver_send_schedule_must_be_refreshed_from_advanced_root` — **INVERTED** (the
    directive-mandated ENG-0030 flip; name kept): B receives a reseed through the SESSION-LEVEL
    `recv_pq_reseed` — no INJECT, no ADOPT, no caller send-half refresh — and the returned state
    is coherent on the advanced root in BOTH directions (`assert_eq!` where NA-0625 asserted
    `assert_ne!`); B's next advertisement authenticates at A. GREEN.
13. `recv_pq_adv_session_round_trip_and_reject_no_mutation` (NEW) — the session-level ADV
    companion: authenticated round trip, chain slot consumed, ROOT UNTOUCHED, next message in
    order; a replayed ADV rejects with the full session state byte-unchanged. GREEN.
14. Pre-existing SCKA sender/receiver integration tests — re-threaded to the root-explicit
    `recv_wire` (callers adopt `outcome.rk`); same assertions incl. the [ADV, reseed] one-pack
    and PQ-PCS healing. GREEN.

## qsc tests

15. `suite2_runtime_equivalence_na0198` — RESTATED per Operator Decision 3: the four
    state checkpoints compare QS2S **v3** bytes (identical field inventory; the ONLY dropped
    field is the second 32-byte root copy, unrepresentable by construction); the wire half keeps
    both refimpl equalities AND pins each seed-model wire to a FIXED golden SHA-256 constant
    (`f14eabfa…530a0ef8`, `9b20496c…2d6100d1`). GREEN.
16. `protocol_state/mod.rs` unit tests — `v2_plaintext_layout_is_unrecoverable` and
    `v1_raw_snapshot_plaintext_is_unrecoverable` (one per removed legacy split branch;
    deterministic, input never mutated); `v3_plaintext_join_split_roundtrips`; the SCKA
    section codec + rollback-guard tests unchanged. GREEN.
17. `session_state_at_rest.rs` — `legacy_plaintext_session_is_unrecoverable_no_mutation` (the
    removed migrate-function branch): distinct `session_unsupported_version` marker, INACTIVE
    status, NO blob created, legacy file byte-unchanged, deterministic on repeat;
    `tamper_session_blob_rejects_no_mutation` re-based on a send-created v3 blob;
    `session_not_plaintext_on_disk` / `no_secrets_in_output` unchanged. GREEN.
18. `protocol_state_contract_na0217c.rs` —
    `status_reports_legacy_session_unrecoverable_no_mutation` (replaces the migration-to-ACTIVE
    contract). GREEN.
19. `handshake_mvp.rs` `scka_e2e_*` — the five end-to-end proofs over the REAL client, now
    driving the session-level entry points through the rewritten arms (ADV →
    `recv_pq_adv_session`; CTXT → `recv_pq_reseed`); marker/persist orderings unchanged
    (ADV session-first; CTXT erase-consumed-key-before-plaintext). GREEN (full `cargo test -p
    qsc`, ~80 min — result recorded below).
20. Full `cargo test -p qsc`: run started at R2 completion; final result recorded in this file
    before the impl PR merge.

## Conformance vectors (Operator Decision 5 — the byte-scanned split, machine-asserted)

21. Regenerator `docs/governance/evidence/NA-0626_vector_regen.py` (committed evidence, after
    the scan evidence per the binding R3 ordering) asserts FAIL-CLOSED: **CHANGED** = exactly
    `scka_logic:S2-SEND-PQRESEED-ACCEPT-0001`, `wire_hex` byte-identical (ZERO wire bytes), the
    ONLY delta the removed `dh_rk` member whose pinned value equalled `rk`; **BYTE-IDENTICAL** =
    the other 25 pre-existing vectors + every other vector file (sha256 cross-set guard);
    **APPENDED** = the 5 named combined-boundary ids. Proof:
    `docs/governance/evidence/NA-0626_vector_regen_proof.json.txt`. PASS.
22. `scripts/ci/validate_suite2_vectors.py` — schema OK.
23. All 15 `scripts/ci/run_suite2_*_vectors.py` runners GREEN (scka_logic 20/20, pq_reseed
    11/11, e2e_recv 4/4, interop 3/3, interop_ximpl 2/2 — the Python actor unchanged — and the
    other ten sets at full count).

## Formal model (`formal/model_suite2_root_composition_bounded.py`, REPLACED)

24. Single-root parties; alphabet {DH boundary, PQ reseed, ADV, COMBINED} × {send, recv};
    depth-5 exploration: 15,032 states / 21,512 transitions, ~1.6 s, zero violations.
25. Regression shapes 6 → 9: the six NA-0625 shapes kept (ENG-0030 shape INVERTED — structural
    coherence of the entry point, plus detection of a hand-staled schedule), + combined
    round-trip incl. the [ADV, combined] pack, + the PQ-FIRST MIS-COMPOSITION COUNTERFACTUAL
    (the model detects the §8.5.2-step-6 clobbering of the ct-bound seeds under the flipped
    order — pins the design-locked ordering at model level), + combined-then-DH healing
    persistence (a pre-combined snapshot cannot open the post-combined DH boundary). PASS via
    `formal/run_model_checks.py`.

## Gates (Phase 5; the list derived MECHANICALLY from the touched workflows' scripts/ci/*.py)

- `cargo fmt --all -- --check` — GREEN.
- `cargo build --workspace --all-targets` (WF-0013) — GREEN.
- `cargo clippy --all-targets -- -D warnings` on the touched crates (`quantumshield_refimpl`,
  `refimpl_actor`, `qsc`, `qsl-tui`) — GREEN. (The pre-existing `apps/qshield-cli`
  `needless_borrow` lint reported at D-1245 is untouched and out of scope.)
- `validate_suite2_vectors.py`, all 15 vector runners, `formal/run_model_checks.py` — GREEN.
- Remaining workflow-derived gates + the impl PR checks are recorded at the merge boundary.

## Boundary notes

- REPORTED Result-boundary deviation: `apps/qsl-tui/src/demo.rs`, three mechanical
  `recv_wire` call-site updates forced by the root-explicit signature + WF-0013 (see D-1247).
- `parse.rs` untouched; `apply_pq_reseed` untouched; no new reason code; no new primitive;
  `docs/canonical/**` changed by exactly the one ENG-0031 sentence.
