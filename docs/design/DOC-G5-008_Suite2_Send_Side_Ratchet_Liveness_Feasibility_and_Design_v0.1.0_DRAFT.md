Goals: G1 (primary), G2, supports G3, G4, G5

Status: Design / Feasibility (DRAFT — proposal, not normative; authorizes no implementation)
Owner: QSL Governance
Last-Updated: 2026-07-07

# DOC-G5-008 — Suite-2 Send-Side Ratchet Liveness: Feasibility and Design

Directive: QSL-DIR-2026-07-08-556 (D556). Lane: NA-0619 (docs-only). Ledger: ENG-0012 (P1).

> **What this document is.** A feasibility study and staged design for making the Suite-2
> ratchet actually re-key on the real client send path. It is a PLAN for later implementation
> lanes. It changes no source and makes no normative spec change; where it needs a spec edit it
> RECOMMENDS one for a future authorized lane. No Triple-Ratchet / post-compromise /
> quantum-secure claim is made or implied by this document.

## 1. Problem statement and current-state proof (ENG-0012 / audit C-1+C-2)

Verified against the shipped tree at `8d9b158e`:

- **C-1 — no classical DH ratchet executes in `suite2`.** There is zero X25519 use in the
  `suite2` module; the root key `rk` and header keys `hk_s`/`hk_r` are assigned once in
  `establish.rs::init_from_base_handshake` and never reassigned. The state structs confirm it:
  `Suite2SendState`/`Suite2RecvState` carry a fixed `dh_pub` (used only as a nonce input via
  `nonce_hdr`/`nonce_body`) and have **no X25519 private key, no separate peer-DH field
  (`DHr`), and no live `rk`**. So there is no material with which to run a DH ratchet at all.
- **C-2 — no sender-side boundary / PQ-reseed path.** The only exported send function,
  `send_wire`, rejects any nonzero `flags` (`REJECT_S2_LOCAL_UNSUPPORTED`). There is no
  `send_boundary` / `send_pq_ctxt`. The receive side is complete — `recv_boundary_in_order`
  (`suite2/ratchet.rs`) and `apply_pq_reseed` (`suite2/scka.rs`) — but nothing can originate a
  boundary, so this machinery is unreachable from real traffic.
- **Static-`rk` bootstrap.** The client manufactures the reverse-direction chains from the same
  static root key: `qsp_activate_responder_send_chain_if_needed` /
  `_initiator_recv_chain_if_needed` (`qsc/src/main.rs`) set `ck_ec`/`ck_pq` via
  `kmac_out(&st.recv.rk, "QSP5.0/CK0/B->A", …)`. The entire bidirectional key schedule is thus
  a deterministic function of one `rk` fixed at establishment.

**Delivered security property today:** forward secrecy by chain-key deletion only, for the
whole session lifetime, with **no post-compromise security (PCS / self-healing)**. A one-time
capture of live ratchet state (`rk`, `ck_ec`, `ck_pq`, header keys) compromises all future
messages in that session, because nothing ever injects independent secret material again. This
is weaker than the Signal Double Ratchet the design invokes and contradicts the "Triple
Ratchet" name and DOC-CAN-003. **This is the top-priority correctness gap; it blocks the G1
(always-hybrid per-message keys with a live classical chain + sparse PQ reseed) and G2 (SCKA
liveness) release gates.**

## 2. Feasibility — the machinery already exists

> **Correction (NA-0621 / ENG-0012 Stage 1b-i, 2026-07-08).** The "receiver mirror already
> exists" framing below was imprecise. Design-lock at `1ec1784f` established that
> `recv_boundary_in_order` implements the **PQ-reseed** path (`apply_pq_reseed`, §8.5.3) — NOT
> the classical DH ratchet (0 X25519/`kdf_rk_dh`/keypair uses). The classical DH ratchet was
> therefore absent on **both** the send and receive sides, and Suite-2 had **no `NHK`
> (next-header-key)** machinery, which §8.5.1's boundary-header anti-spoof rule requires. What
> *did* hold: the wire already carries `DH_pub[32]` on every ratchet message (§4.3;
> `parse_ratchet_header` extracts it, `send_wire` writes it), so no wire-format change was needed
> — the DH ratchet is a behavior change, not a format change. NA-0621 (Stage 1b-i) implemented,
> in refimpl: `KDF_RK_DH` (§3.3.2), the `HK/NHK` derivation (§3.4/§8.1), `send_boundary` (DH send,
> §8.5.2), and `recv_dh_boundary` (DH receive + §8.5.1 CURRENT_NHK anti-spoof), proven by a
> two-party round-trip and a PCS-healing test. The qsc trigger + static-`rk` removal are Stage
> 1b-ii; the PQ-reseed sender is Stage 2.

> **Update (NA-0622 / ENG-0012 Stage 1b-ii, 2026-07-08).** Stage 1b-ii landed: the classical DH
> ratchet now runs on the REAL qsc send path. `qsp_pack` originates a DH boundary via the refimpl
> `send_boundary` on the operator-chosen trigger — RATCHET-ON-REPLY (first send after any receive)
> + a bounded fallback of N=4 messages / T=15 min; `qsp_unpack` routes incoming boundaries to
> `recv_dh_boundary`. The static-`rk` bootstrap (`qsp_activate_*`) is removed — the ratchet now
> creates the responder's send chain and the initiator's recv chain. The reply-driven trigger is
> persisted in a qsc session-blob v2 plaintext (refimpl snapshot frozen). Proven end-to-end over a
> real A/B handshake (round-trip + PCS-healing). §5's open questions are resolved: N_dh=4 / T_dh=15
> min; the DH-boundary observable is accepted + documented in DOC-G5-004, with cover traffic
> deferred to ENG-0022. **Classical** post-compromise security now runs on live traffic; the
> POST-QUANTUM guarantee awaits Stage 2 (PQ-reseed sender). No Triple-Ratchet / post-compromise
> claim beyond the classical, refimpl-and-e2e-proven scope until Stage 2 lands.

> **Update (NA-0623 / ENG-0012 Stage 2a, 2026-07-08).** Stage 2 was sub-staged; Stage 2a landed the
> PQ-reseed SENDER core IN REFIMPL. A Phase-2 re-scope (D560 AMENDMENT) fixed a security gap the
> design-lock surfaced: the existing PQ-reseed RECEIVER (`recv_boundary_in_order` -> `apply_pq_reseed`)
> absorbed the PQ epoch secret into the directional PQ chains but NEVER advanced the root `RK`
> (§8.5.3 steps 5+7 were unimplemented), so the next classical DH ratchet reinitialised `CK_pq` from
> the un-hardened root and WIPED the post-quantum protection. Stage 2a adds `KDF_RK_PQ` (§3.3.3) +
> the `HK` recompute to BOTH the receiver AND the new sender (`send_pq_advertise`, `send_pq_reseed`,
> `track_peer_adv`), so the PQ secret lands in the root and the DH ratchet carries it forward
> permanently. The advertised-key store / ML-KEM KeyGen+Encap are caller-side (the refimpl sender is
> pure functions); no snapshot bump, no non-boundary-path change, no wire-format change; the
> `apply_pq_reseed` CTXT-validation semantics are unchanged. Proven in refimpl by a round-trip
> (advertise -> encapsulate -> `apply_pq_reseed` decrypts) and — the headline — a PQ-PCS-healing
> vector that SURVIVES a subsequent DH ratchet (a pre-reseed snapshot cannot open the post-reseed DH
> boundary), plus monotonicity/one-time/tombstone rejects. Known deviation for Stage 2b/spec
> alignment: the refimpl PQ-CTXT boundary header uses `HK` (the frozen receiver), not the §8.5.1
> `NHK`. Stage 2b (NA-0624) wires the SCKA advertise + reseed cadence into the real qsc send path and
> persists the SCKA state — the stage that delivers post-quantum PCS on live traffic and FULLY closes
> the P1. No POST-QUANTUM / Triple-Ratchet claim on live traffic until Stage 2b lands and the DH+PQ
> composition is independently analyzed.

> **Update (NA-0624 / ENG-0012 Stage 2b, 2026-07-08).** Stage 2b landed: the Stage-2a SCKA sender is
> wired into the REAL qsc send/receive path, reusing the frozen refimpl semantics exactly (the
> runtime-equivalence test stays byte-for-byte). `qsp_pack` originates SCKA advertisements
> (`send_pq_advertise`) as separate CONTROL envelopes pushed before the main message — on
> establishment, when the local advertised key is consumed, and on rotation — and originates PQ
> reseeds (`send_pq_reseed`) on the operator-approved sparse cadence (Decision 3: first reseed as
> soon as a fresh unconsumed peer advertisement is available, then every N_pq=8 sent DH boundaries
> or T_pq=3600 s, evaluated on non-boundary sends so reseeds co-schedule after DH boundaries). An
> advertisement consumes a send-chain slot the receiver skips as a control message — a NORMAL main
> message heals the receiver's n-gap via the OOO machinery and a DH boundary abandons the old
> epoch, but the frozen reseed receiver is strict-in-order, so an advertisement NEVER shares a
> pack with a reseed (it defers to the next send).
> `qsp_unpack` intercepts `FLAG_PQ_ADV` before `recv_wire` (the frozen receiver has no ADV path) and
> validates via `track_peer_adv`; a `FLAG_PQ_CTXT` boundary decapsulates against the local
> advertised ML-KEM key, INJECTS the canonical session root into the receive state (`recv.rk :=
> dh.rk` when live — a DH boundary advances only `dh.rk` and the frozen reseed SENDER derives from
> `session_root`, so the receiver must mirror it or the parties derive `KDF_RK_PQ` from different
> roots: the NA-0623 dh.rk-sync carry-over, resolved caller-side), drives the frozen
> `apply_pq_reseed` via `recv_wire`, and then ADOPTS the advanced root into the DH-ratchet slot
> (`dh.rk := recv.rk`) — the caller-side composition Stage 2a deferred to qsc, so a later classical
> DH ratchet carries the PQ hardening instead of wiping it.
> The SCKA state (bounded advertised-key store with deterministic eviction, peer advertisement,
> cadence counters) persists inside the AEAD session blob as a length-delimited v3 section (refimpl
> QS2S snapshot frozen; v2/v1 migrate; a non-advertising session has an empty section) with a G2
> monotonic side-record (`peer_max_adv_id_seen` / `local_next_adv_id` / `peer_adv_max_seen` /
> `peer_adv_consumed_max` / tombstones): a rolled-back blob fails closed
> (`session_rollback_detected`), which also prevents re-consuming a one-time peer target across a
> restore. An enabling fix landed with the wiring: the transport deliver path now persists the
> qsp_pack trigger (the NA-0622 cleared ratchet-on-reply flag and N/T fallback counters previously
> never persisted on the main send path, so every post-receive send ratcheted and a non-boundary
> reseed send could never fire; the documented D-1239 cadence is now live). Proven end
> to end over a real A/B handshake: advertise -> reseed mid-conversation -> both decrypt; the
> headline PQ-PCS-healing vector survives a subsequent DH ratchet ON THE REAL CLIENT (the pre-reseed
> snapshot holds every classical secret — the DH private key is in it and the boundary DH public is
> on the wire — and still cannot decrypt); rollback fails closed. Flagged deviations carried to the
> spec-alignment successor lane (with the §8.5.1 NHK item): (1) ADV TRACKING IS UNAUTHENTICATED —
> `qsp_unpack` validates length + monotonicity but cannot authenticate the ADV header (the frozen
> receiver rejects `FLAG_PQ_ADV`), so an attacker able to inject into the relay inbox can plant an
> advertisement (bounding: the planted-key reseed still MIXES into `RK` via `KDF_RK_PQ`, so classical
> security is unaffected and the result is no worse than "no reseed" for the PQ layer, plus a
> tracking-DoS via a max `adv_id`); (2) a lost/dropped ADV or reseed envelope degrades to the
> classical status quo until rotation re-advertises (the outbox replays only the main message).
> The POST-QUANTUM / Triple-Ratchet / post-compromise claim on live traffic remains WITHHELD pending
> independent analysis of the DH+PQ composition (the standing claim boundary).

This is not a from-scratch protocol design; the receive side and a reference send side already
exist and constrain the answer:

- **Spec is complete.** DOC-CAN-003 §8.5.2 fully specifies the sender's DH-boundary construction
  and the receiver's processing; §8.5.3 specifies the PQ-reseed (boundary-with-PQ) path. The
  gap is implementation and trigger policy, not specification.
- **Receiver mirror target.** `recv_boundary_in_order` and `apply_pq_reseed` already perform
  exactly the inverse of what a sender must do; the sender is their structural mirror.
- **Working DH-ratchet reference in the same crate.** `qsp/ratchet.rs::dh_ratchet_send`
  implements the §8.5.2 send steps correctly (check `ns == u32::MAX`; `pn := ns`; `ns := 0`;
  new X25519 keypair; `dh_out = X25519(new_priv, peer_pub)`; `KDF_RK_DH`; recompute header
  keys). A suite-2 `send_boundary` adapts this into suite-2's state/label namespace.
- **Parse already permits DH-only boundaries (audit correction).** `suite2/parse.rs` requires
  `FLAG_BOUNDARY` only when a PQ flag (`FLAG_PQ_ADV`/`FLAG_PQ_CTXT`) is present; a bare
  `FLAG_BOUNDARY` (DH-only boundary) is valid. The audit note that "parse requires FLAG_PQ_CTXT
  whenever FLAG_BOUNDARY is set" is imprecise — DH-only and DH+PQ boundaries are both
  parse-legal, so the design need not force co-scheduling.

Conclusion: **feasible with bounded, well-scaffolded work.** The risk is in getting the trigger
policy, the state additions, and the conformance coverage right — not in inventing crypto.

## 3. Trigger policy (the core open design decision)

The spec defines HOW a boundary is built but not WHEN to originate one. Proposed policy:

- **DH ratchet cadence:** originate a DH boundary on the first send after receiving a message
  in a new receive epoch (Signal-style "ratchet on reply"), AND as a bounded fallback every
  `N_dh` messages or `T_dh` seconds since the last local boundary, whichever comes first. The
  "ratchet on reply" rule is what gives PCS its healing latency (one round-trip).
- **PQ reseed cadence:** originate a PQ reseed (co-scheduled on a DH boundary) every `N_pq`
  boundaries or `T_pq` seconds, and whenever a new SCKA epoch is available. PQ reseed is more
  expensive (KEM ct on the wire) so it is sparser than the DH ratchet — this matches G1's
  "advances every message [symmetric], reseeded sparsely [PQ]".
- **First-boundary bootstrap:** replace the static-`rk` activation. The initiator's first send
  and the responder's first reply each perform their initial DH boundary so both directions are
  seeded from independent DH output rather than the shared static `rk`.
- Parameters (`N_dh`, `T_dh`, `N_pq`, `T_pq`) are policy, not wire format; propose defaults with
  a rationale and make them configurable. Open question for the operator: exact defaults and
  whether cadence is fixed or adaptive (see §10).

## 4. DH-only vs co-scheduled DH+PQ boundaries

Given parse permits both:
- **DH-only boundary** (bare `FLAG_BOUNDARY`): cheap, frequent; provides classical PCS. Follows
  §8.5.2.
- **DH+PQ boundary** (`FLAG_BOUNDARY | FLAG_PQ_CTXT`, carrying `pq_target_id` + `pq_ct`):
  provides PQ reseed; follows §8.5.3. Sparser.
Proposed composition: every PQ reseed rides on a DH boundary (never PQ without DH), so the root
always advances with fresh DH output whenever it advances with PQ material. This keeps the
hybrid ordering (classical then PQ) that establishment already uses. No spec change required for
DH-only; RECOMMEND a clarifying note to DOC-CAN-003 §8.5 that DH-only boundaries are a
first-class sender behavior (a future spec lane — not this lane).

## 5. Sender construction plan (implementation lane, not this lane)

- **`send_boundary`** — mirror `recv_boundary_in_order` + `qsp::dh_ratchet_send`: save
  `boundary_hk = NHK_s`; generate a new X25519 keypair; `PNs := Ns`; `Ns := 0`;
  `dh_out = X25519(DHs_priv_new, DHr)`; `(RK, CK_ec_send) = KDF_RK_DH(RK, dh_out)`; reinit the
  PQ send chain (`CK_pq_send := KMAC32(RK, "QSP5.0/PQ0/<dir>", [0x01])`); recompute `HK/NHK`;
  encrypt the header under `boundary_hk`; set `FLAG_BOUNDARY`.
- **`send_pq_ctxt`** — mirror `apply_pq_reseed`: run SCKA encapsulation (DOC-CAN-004) to produce
  `pq_target_id` + `pq_ct` and `pq_epoch_ss`; `KDF_PQ_RESEED`; `RK := KDF_RK_PQ(RK, pq_epoch_ss)`;
  set `FLAG_PQ_CTXT` and attach the PQ prefix; co-schedule on a DH boundary.
- **Required state-struct additions** (the load-bearing change): add to the live send/recv state
  a full X25519 keypair (`DHs_priv`, `DHs_pub`), the peer DH pub (`DHr`), and the live root key
  `RK` (today the send/recv structs carry neither `DHr` nor `RK`). These are the fields a
  ratchet needs and the current structs lack. Persisted state (`state.rs` snapshot) and its
  versioning must extend to cover them (G2).

## 6. qsc client wiring (implementation lane)

- Remove `qsp_activate_*_chain_if_needed`; the initial chains come from the first real DH
  boundary instead of the static `rk`.
- Drive the §3 trigger policy from the real send path (`send_wire_canon` call site,
  `qsc/src/main.rs`), choosing `send_wire` vs `send_boundary`/`send_pq_ctxt` per policy.
- Persistence/rollback: the new DH keypair + `RK` must be crash-safe and rollback-detected
  (extend the existing `state.rs` snapshot + SCKA monotonicity guards; G2). Losing a boundary
  keypair across a crash must fail closed, not silently reuse an old chain.

## 7. Conformance-vector requirements (the proof the current tests lack)

The current suite2 tests exercise the receive side in isolation. The implementation lane must
add vectors that exercise the SEND path end to end:
- A full two-party session where the DH ratchet fires mid-conversation (both directions) and
  every message still decrypts after the boundary.
- A session where a PQ reseed fires mid-conversation and subsequent messages decrypt.
- Interleaved DH + PQ boundaries with out-of-order and skipped messages across a boundary.
- No-state-mutation-on-reject for the new send paths; SCKA epoch monotonicity across sender
  reseeds; rollback rejection after a simulated crash mid-boundary.
- A "PCS healing" vector: given a snapshot captured before a boundary, messages sent after the
  boundary must NOT be decryptable with the pre-boundary state (demonstrating self-healing).

## 8. Interaction with the NA-0618 counter hard-stop (ENG-0013)

A live DH ratchet resets `Ns := 0` (and `Nr := 0` on the receiver) at every boundary. With any
reasonable `N_dh`/`T_dh`, the per-chain counter never approaches `u32::MAX`, so the NA-0618
`REJECT_S2_COUNTER_OVERFLOW` hard-stop becomes an unreachable backstop rather than a practical
session terminator. The two fixes reinforce each other; the hard-stop remains as
defense-in-depth for a degenerate no-boundary session.

## 9. Staged implementation plan and claim boundary

Proposed follow-on lanes (each its own directive, gated on operator acceptance of THIS design):
1. **Stage 1 — DH ratchet, send + recv, on the real client path.** Add the DH state, implement
   `send_boundary`, remove the static-`rk` bootstrap, wire the DH trigger, add the two-party DH
   conformance vectors. (Classical PCS lands here.)
2. **Stage 2 — PQ reseed sender.** Implement `send_pq_ctxt` + SCKA sender encapsulation, the PQ
   trigger, and the PQ + interleaved vectors. (Hybrid PQ self-healing lands here.)
3. **Stage 3 — spec + claim reconciliation.** RECOMMENDED spec edits (DH-only boundary note;
   any trigger-policy normative text) and the honest claim update.

**Claim boundary (binding until Stages 1–2 land and vectors pass):** the project MUST NOT claim
"Triple Ratchet", "post-compromise security", "self-healing", or "quantum-secure by design"
for the shipped protocol. A separate low-cost docs lane should add this caveat wherever such
language currently appears (candidate ledger item).

## 10. Open questions for operator decision

- Trigger defaults: values for `N_dh`/`T_dh`/`N_pq`/`T_pq`; fixed vs adaptive cadence.
- Whether "ratchet on reply" (Signal-style) is acceptable given its metadata signature (a
  boundary message is observable), vs a purely time/count-based cadence (see §7 G5 note below).
- Whether to co-schedule PQ strictly on DH boundaries (proposed) or allow independent PQ epochs.
- Scope of the state-snapshot version bump and back-compat stance (pre-release: eliminate, do
  not carry legacy — per the PROJECT_CHARTER design tenet).

## ENG-0023 implementation note (NA-0625, D-1245) — §8.5.1 NHK + authenticated ADV receive

Recorded here because this document is the design home for the Suite-2 ratchet lanes. NA-0625
closed the two header-authentication gaps carried out of NA-0623/NA-0624. It changed no normative
DOC-CAN text; `docs/canonical/` and `parse.rs` were untouched.

**Gap (1) — the §8.5.1 NHK boundary header (a real deviation, now fixed).** The design-lock's crux
was whether §8.5.1's `NHK` rule reaches the PQ-CTXT boundary at all, or whether NA-0623's
"HK-not-NHK deviation" was imprecise labelling. It was settled from DOC-CAN-003's exact text, not
from intuition: §8.5 defines a boundary as **any** message with `FLAG_BOUNDARY = 1` and names
"application of SCKA reseed events" as a boundary purpose; §8.5.1's sender rule is unconditional
over such messages; and §8.5.3 step 1 states verbatim, for the `FLAG_PQ_CTXT` receiver, "Require
`hdr_source == CURRENT_NHK` (see §8.5.1)". The counter-argument (a reseed is an in-order message on
the pre-reseed schedule, not a fresh DH epoch, so NHK's anti-spoof purpose may not map) fails
against both the text and the purpose: a reseed **is** a key-schedule transition — §8.5.3 steps 5–7
advance `RK` and recompute `HK`/`NHK` — and §8.5.1's rationale is to bind the transition header to
the pre-transition root. Message-counter continuity is orthogonal to header-key choice.

Both sides now derive the NHK on the fly from the canonical pre-reseed root (the in-repo
`recv_dh_boundary` pattern): no stored NHK field, therefore no `Suite2*State` change, therefore
**no QS2S snapshot bump** — which is why Operator Decision 3 (ENG-0024 co-scope) stayed NO and its
re-present clause never triggered. The receiver opens NHK-only, keeping candidates `[nr, nr+1]` and
the `n == nr` check, so the `NOT_IN_ORDER` vs `HDR_AUTH_FAIL` distinction is preserved and a
pre-NHK (HK-sealed) frame dies generically as `REJECT_S2_HDR_AUTH_FAIL`. §8.5.1's "decrypts under
any other candidate key MUST reject" is satisfied by never trying another key.

**Gap (2) — authenticated ADV receive (Operator Decisions 1 + 2).** DOC-CAN-004 §1.1/§1.3 fixes the
ADV prefix normatively as `pq_adv_id(4) || pq_adv_pub(1184)`, and `docs/canonical/` was not a
mutable path this lane, so the SPQR-style control-plane MAC could not ride in the pq_prefix. It
rides instead as the first 32 bytes of the **sealed body plaintext**:

```
adv_mac = KMAC32(RK, "QSP5.0/ADVAUTH", u32be(pq_adv_id) || pq_adv_pub || [0x01])
body_pt = adv_mac(32) || app_payload
```

`RK` is the canonical session root on both sides, so the MAC inherits exactly the header-key
synchronisation envelope — no new failure mode, no new primitive (KMAC only), no new reason code
(a MAC mismatch or a short body reuses `REJECT_S2_BODY_AUTH_FAIL`). `pq_bind` and the AD layouts
are byte-unchanged; parse.rs took no hook. The wire delta is exactly +32 B on the ADV `body_ct`
(recorded in DOC-G5-004 §3.1). A planted advertisement fails first at the header AEAD under session
keys; the ADVAUTH MAC is the root-keyed authenticator on top (defence in depth, and an explicit
binding of the advertised key material to the session root).

The ADV receive **consumes its chain slot** in order (Decision 2): both receive chains step and
`nr` advances, mirroring the sender. That retired both NA-0624 workarounds — the ADV/reseed
pack-exclusion rule and the mkskipped control-slot growth — and `[ADV, reseed]` now round-trips in
one pack (proved e2e on the real client). The control plane is in-order-only, matching the CTXT
receiver's posture; a lost predecessor degrades bounded, via outbox replay or `T_pq` re-advertise.

**ADV header key: HK, not NHK — and the spec tension that follows.** §8.5.1's *sender* sentence is
unconditional over `FLAG_BOUNDARY = 1` and so literally covers `FLAG_PQ_ADV` boundaries too. But
§8.5.4 conspicuously omits the "Require `hdr_source == CURRENT_NHK`" step that §8.5.2 and §8.5.3
both state, and §8.5.1's *receiver* sentence scopes itself to "a boundary **epoch transition**" —
which an advertisement is not, since it advances no root. Both readings are defensible. This lane
pinned the ADV header to `HK` (unchanged): HK-vs-NHK confers zero attacker advantage when no root
transition occurs (both prove possession of a key derived from the same `RK`), so this is not a
weakening, and flipping `send_pq_advertise`'s header key was outside the two named gaps.
**Operator: the tension is real and belongs in the spec.** Filed as **ENG-0031** — either a
one-line DOC-CAN-003 clarification (scope §8.5.1's sender sentence to epoch-creating boundaries,
matching §8.5.4's silence and §8.5.1's own receiver sentence) or a bounded NHK flip for the ADV
header riding ENG-0026. Not this lane either way.

**Implementation finding (ENG-0030).** `send_pq_reseed` refreshes both directional header keys and
the send PQ chain for the SENDER, but the receive path can only return recv-side state — so after a
party RECEIVES a reseed its `send.hk_s` / `send.ck_pq` remain on the pre-reseed schedule. Latent
before this lane (any send after a receive is a DH boundary, which reinitialises both); exposed by
the authenticated ADV, because an advertisement rides the CURRENT send chain as a control
pre-envelope and the peer now opens that header. qsc's CTXT intercept arm now mirrors the send half
beside the dh.rk ADOPT. Same caller-owned-coherence class as ENG-0024; making it structural is
recommended there.

**Claim boundary unchanged.** Nothing in NA-0625 introduces a post-quantum, Triple-Ratchet, or
post-compromise claim: the DH+PQ composition still awaits independent analysis (ENG-0028). The
Decision-4 bounded root-composition model (`formal/model_suite2_root_composition_bounded.py`)
guards this lane's surface — root convergence, healing across a DH boundary, chain continuity under
chain-consume, and reject⇒no-mutation — but it is an agreement/coherence model over abstracted
KDFs, not a secrecy proof.

## G5 metadata / traffic-shape note (§7 cross-reference)

Boundary messages are distinguishable (larger — carrying `DH_pub` and, for PQ, `pq_ct`) and, if
cadence is reply-driven, correlate with conversation turns. The trigger policy therefore has a
metadata signature that interacts with the existing message-plane padding/bucketing (DOC-G5-004)
and attachment padding (DOC-G5-007). The implementation lanes must size boundary messages into
the existing buckets where feasible and document the residual (a periodic/time-based cadence
leaks less turn-timing than a strict reply-driven one — a privacy/PCS-latency tradeoff for the
operator). No metadata-free claim.
