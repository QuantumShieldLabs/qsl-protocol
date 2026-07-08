Goals: G1 (primary), G2, G3, G4, G5

Status: Supporting
Owner: QSL Governance
Last-Updated: 2026-07-08

# NA-0622 Evidence — ENG-0012 Stage 1b-ii: qsc DH-ratchet trigger + static-rk removal (D559)

Directive QSL-DIR-2026-07-08-559 (D559). Decision D-1239 (implementation); D-1240 (closeout).
Class summaries only; no raw private material.

## 1. What landed (the classical DH ratchet now runs on the real client)

- `qsp_pack` (the qsc send funnel) originates a classical DH boundary via the refimpl
  `send_boundary` when the trigger fires; otherwise a normal message via `send_wire_canon`.
- `qsp_unpack` routes an incoming DH-only boundary (FLAG_BOUNDARY, no FLAG_PQ_CTXT — detected via
  `decode_suite2_wire_canon`) to the refimpl `recv_dh_boundary`; otherwise the normal recv path.
- The static-`rk` bootstrap (`qsp_activate_responder_send_chain_if_needed`,
  `qsp_activate_initiator_recv_chain_if_needed`) is REMOVED. Ratchet-on-reply now creates the
  responder's send chain (its first reply is a boundary — `should_ratchet` returns true when the
  send chain is unset) and the initiator's recv chain (created when it processes the responder's
  first boundary).
- Trigger policy (operator decision, D559): RATCHET-ON-REPLY (first send after any receive; a
  `pending` bit set on every receive) + a bounded fallback of N=4 messages / T=15 min.
- The refimpl `Suite2SessionState` / QS2S snapshot and the DH-ratchet crypto are FROZEN — qsc calls
  the Stage-1b-i functions unchanged (refimpl `git status` clean).

## 2. Persistence (qsc session-blob v2)

qsc is load-per-message (no in-memory session cache), so the reply-driven trigger is PERSISTED.
The trigger (`pending` + `msgs_since_ratchet` + `last_ratchet_unix_secs`, 13 bytes) is carried
inside the encrypted session-blob plaintext as `b"QTRG"` + trigger + QS2S snapshot (v2). Legacy
raw-snapshot plaintexts migrate transparently (default trigger). `qsp_session_store` (unchanged
signature; 18 call sites untouched) preserves the on-disk trigger; the message path uses
`qsp_session_store_with_trigger`. No refimpl snapshot bump; no blob-envelope-format change.

## 3. Seed-fallback gate (session-based, test-only, honest scoping)

The DH ratchet is gated OFF for a DEGENERATE SELF-DH session — one whose peer DH key equals its
own (`st.dh.dhr == st.dh.dhs_pub`). That is the unsafe seed-fallback TEST model: it derives a
SYMMETRIC (both role-A) session that cannot round-trip the direction-sensitive DH ratchet (a
sender signs a boundary header under NHK_A->B while a role-A receiver would try NHK_B->A), and its
send chain is already seeded, so it retains pre-ratchet behavior. The gate keys off the SESSION
STATE, NOT the `allow_unsafe_seed_fallback_for_tests()` flag — because real-handshake tests also
set that flag (they merely PERMIT seed fallback), and a real session must ratchet even when the
flag is set (its responder send chain is unset once the static-`rk` bootstrap is gone). REAL,
handshake-established sessions have `dhr != dhs` and always evaluate the trigger; the seed model
(`dhr == dhs`) never happens in production. The ratchet is proven on real sessions by the e2e
handshake tests below; this session-based gate is why the pre-existing seed-model regression suite
stays byte-for-byte green while real-handshake suites exercise the live ratchet.

## 4. Proof (end-to-end over a REAL A/B handshake)

`cargo test -p qsc --test handshake_mvp dh_ratchet_e2e` → 2 passed:
- `dh_ratchet_e2e_roundtrip_over_real_handshake` — Alice (initiator/role A) and Bob
  (responder/role B) complete the real handshake; Alice's first send is a NORMAL message; Bob's
  reply is a DH BOUNDARY (`event=qsp_dh_ratchet dir=send`, ratchet-on-reply — creating his send
  chain now that the static-`rk` bootstrap is gone); Alice processes the boundary
  (`event=qsp_dh_ratchet dir=recv`) and decrypts it; the reverse direction ratchets too.
- `dh_ratchet_e2e_pcs_healing_over_real_handshake` — after both parties ratchet forward past a
  captured epoch, a restored pre-ratchet session snapshot CANNOT decrypt a post-ratchet message
  (post-compromise self-healing on the real client). The ratchet crypto's PCS is proven in the
  refimpl (NA-0621); this shows the client wiring preserves it end to end.

Regression / build gates (all green):
- Full `quantumshield_refimpl` and qsc suites pass; the runtime-equivalence test
  (`suite2_runtime_equivalence_na0198`) stays byte-for-byte equivalent to the refimpl (seed path,
  gated), and the attachment/message/transport/relay/handshake suites are unchanged.
- `cargo build --workspace --all-targets` clean (WF-0013); `cargo fmt --check` clean; `cargo
  clippy -p qsc --lib` and the changed test targets `-D warnings` clean; `cargo metadata --locked`
  clean; Cargo unchanged.

## 5. Boundary and claim

Mutations: `qsl/qsl-client/qsc/src/**` (trigger + recv routing + static-`rk` removal + blob-v2
persistence + markers), qsc tests (e2e vectors + a v2-strip helper fix + one pre-existing
`== false` clippy fix), and governance/design docs (DOC-G5-004 observable, DOC-G5-008 update,
ENG-0022, ENG-0012 Stage-1b-ii-done). No refimpl change; no PQ-reseed change; no KDF/AEAD
primitive change; no wire-format change; no normative DOC-CAN change; no Cargo/workflow/`.claude`
change. Research/demo only. The CLASSICAL half of the P1 is closed — classical post-compromise
security now runs on live qsc traffic — but this is NOT a post-quantum, Triple-Ratchet,
security-completion, or production-readiness claim: the POST-QUANTUM guarantee awaits Stage 2 (the
PQ-reseed sender = NA-0623). The DH-boundary metadata observable is documented in DOC-G5-004; cover
traffic is deferred to ENG-0022. No endpoint, token, capability, key, seed, plaintext, ciphertext
body, or raw private material is published.
