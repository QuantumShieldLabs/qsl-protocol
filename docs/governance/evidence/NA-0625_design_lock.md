# NA-0625 Phase-2 DESIGN-LOCK — ENG-0023 Suite-2 spec-alignment (D562)

Lane: NA-0625 (sole READY at D-1245). Directive: QSL-DIR-2026-07-09-562 (APPROVED).
Base: main == origin/main == `bc512f2e` (clean). Recorded: 2026-07-08 (UTC).
Status: DESIGN-LOCK COMPLETE — no code written (Phase-2 discipline). Implementation (Phase 3+)
resumes in a fresh chat per the session-handoff convention.

---

## 1. CRUX RESOLUTION — §8.5.1 NHK DOES apply to the PQ-CTXT boundary. PROCEED AS DIRECTED.

Settled from DOC-CAN-003 exact text (no assumption):

- §8.5 (line 511): "A 'boundary' message is any message with `FLAG_BOUNDARY = 1`. Boundaries are
  the only points where Suite-2 permits: DH ratchet advancement, **and/or application of SCKA
  reseed events** to PQ chain keys." — the PQ reseed is a first-class boundary in the spec's own
  taxonomy, not an ordinary in-order message that happens to carry a prefix.
- §8.5.1 sender rule (line 517): "A boundary message header MUST be encrypted under the sender's
  `NHK_s` derived from the **pre-boundary** `RK`." — unconditional over FLAG_BOUNDARY=1; no
  epoch-creating qualifier.
- §8.5.3 step 1 (line 556), the FLAG_PQ_CTXT receiver, verbatim: "Require `hdr_source ==
  CURRENT_NHK` (see §8.5.1)." — zero ambiguity for the CTXT case specifically.

The NA-0623 counter-argument ("a PQ reseed is in-order, not a fresh DH epoch, so NHK's anti-spoof
purpose may not map") fails against the text AND against the purpose: a PQ reseed IS a
key-schedule transition — §8.5.3 steps 5–7 advance `RK` (KDF_RK_PQ) and recompute `HK`/`NHK` —
even though `Nr` continues. §8.5.1's anti-spoof rationale (bind the transition header to the
pre-transition root) maps exactly; message-counter continuity is orthogonal to header-key choice.
Mechanically there is no obstacle: the receiver's current `NHK_r` derives from the same
pre-reseed `RK` the sender's `NHK_s` derives from, at any `n == nr`.

**Verdict: the NA-0623 "HK-not-NHK deviation" was a REAL normative deviation, not imprecise
labeling. Gap (1) is a receiver/sender semantic change as directed. NOT a DOC clarification; NO
STOP; NO operator re-present required on the crux.**

Residual textual tension recorded (NOT blocking, NOT this lane's semantics): §8.5.1's sender
sentence literally also covers FLAG_PQ_ADV boundaries, but §8.5.4 (ADV) conspicuously omits the
"Require hdr_source == CURRENT_NHK" step that §8.5.2 and §8.5.3 both state, and §8.5.1's receiver
sentence scopes itself to "a boundary **epoch transition**" — which an ADV is not (RK unchanged).
Both readings are defensible for ADV; see §3 below for the pinned choice and the follow-up note.

## 2. Gap (1) pin — NHK flag-conditional key choice (receiver + sender mirror)

- **Receiver** `recv_boundary_in_order` (ratchet.rs:688-707): replace `st.hk_r` with
  `nhk_r = header_key(kmac, &st.rk, a2b_recv, next=true)` derived on the fly (the
  `recv_dh_boundary` reference pattern, ratchet.rs:1334). Candidate structure UNCHANGED:
  try [nr, nr+1], enforce `n == nr` (preserves the NOT_IN_ORDER vs HDR_AUTH_FAIL distinction).
  NHK-ONLY open for FLAG_PQ_CTXT frames — no HK trial, no new reason code: a pre-NHK
  (HK-sealed) frame fails generically with `REJECT_S2_HDR_AUTH_FAIL`. §8.5.1's "decrypts under
  any other candidate key MUST reject" is satisfied by never trying other keys.
  `st.rk` is the canonical pre-reseed root at this point: in qsc the NA-0624 root INJECTION
  (`recv.rk := dh.rk` pre-recv_wire, main.rs:2624-2629) already guarantees it — the injection
  stays load-bearing under NHK (forward-study caution §5 checked: NHK derives from the same
  injected root; no change to the inject/adopt dance).
- **Sender mirror** `send_pq_reseed` (ratchet.rs:1675-1680): seal under
  `nhk_s = header_key(kmac, &rk_old, a2b_send, next=true)` where `rk_old = session_root(&st)`
  (== the root that produced the stored `st.send.hk_s`, so the mirror is exact). Post-commit
  recompute of `hk_s`/`hk_r` from `new_rk` UNCHANGED (next=false).
- **No stored NHK field anywhere** (derive-on-demand, the in-repo DH-boundary precedent) ⇒
  **no Suite2*State field change ⇒ no QS2S snapshot bump ⇒ Decision 3 stays NO — the ENG-0024
  re-present clause is NOT triggered.** Verified against `Suite2BoundaryState` /
  `Suite2RecvWireState` / `Suite2SessionState`: all already carry the needed `rk`.
- FORBIDDEN surfaces untouched: `apply_pq_reseed` CTXT-validation rules, non-boundary
  send_wire/recv_wire paths, DH ratchet, KDF/AEAD/KEM primitives.

## 3. Gap (2) pin — authenticated ADV receive (Decisions 1+2)

**Frame layout (the binding scope constraint):** DOC-CAN-004 §1.1/§1.3 normatively fixes the ADV
prefix as `pq_adv_id(4) || pq_adv_pub(1184)` with "cannot parse fixed-size fields MUST reject",
and `docs/canonical/` is NOT an allowed mutation path this lane. Therefore the MAC CANNOT ride in
the pq_prefix (that would be a normative wire change + a parse.rs hook). **PINNED: the MAC rides
as the first 32 bytes of the sealed ADV body plaintext:**

```
adv_mac  = KMAC32(RK, "QSP5.0/ADVAUTH", u32be(pq_adv_id) || pq_adv_pub || [0x01])
body_pt  = adv_mac(32) || app_payload   (app_payload may be empty)
```

- `RK` = canonical session root at send time (`session_root(&st)`); receiver verifies under its
  canonical root (`st.rk`, injection-coherent). The MAC inherits EXACTLY the header-key
  synchronization envelope — any crossing scenario that would desync ADVAUTH already desyncs
  `hk_s/hk_r`; no new failure mode. KMAC only; no new primitive; label follows house style
  (trailing `[0x01]`, same shape as the directive's approved example).
- `pq_prefix`, `pq_bind`, `ad_hdr`, `ad_body` layouts: BYTE-UNCHANGED (DOC-CAN-004 §1.4 binding
  preserved verbatim). parse.rs: NO hook (the directive's stated preference holds).
- Wire delta: ADV `body_ct` grows exactly +32 → **DOC-G5-004 observable note required** (padding
  buckets may mask it on the wire; note it regardless).
- Reason codes: NO new normative code (DOC-CAN-004 §3.6 is frozen to us). MAC mismatch after a
  successful body open ⇒ reuse `REJECT_S2_BODY_AUTH_FAIL`; body shorter than 32 ⇒ same. A planted
  ADV from a relay injector fails FIRST at the header AEAD (`REJECT_S2_HDR_AUTH_FAIL`) — the AEAD
  under session keys with `pq_bind` in AD is the primary authentication; the ADVAUTH MAC is the
  approved SPQR-style root-keyed authenticator on top (defense-in-depth + explicit
  payload-under-RK binding).

**ADV header key: PINNED = HK (unchanged), NOT NHK.** Rationale: the directive's DoD names the
NHK move for PQ-CTXT only ("flag-conditional key choice"); §8.5.4 omits the CURRENT_NHK step and
§8.5.1's receiver rule scopes to epoch transitions (an ADV advances no root, so HK-vs-NHK confers
zero attacker advantage — both prove possession of the same-RK-derived key; keeping HK is not a
weakening); and flipping `send_pq_advertise`'s header key is outside the two named gaps (STOP
condition: do not widen). The §8.5.1-sender-sentence/§8.5.4 textual tension is recorded in the
DOC-G5-008 ENG-0023 note for the operator at D-1245 (a future one-line DOC-CAN clarification or a
bounded NHK flip riding ENG-0026 — either way not this lane).

**`recv_pq_adv` semantics (Decision 2: chain-consume = YES), pinned flow — fail-closed, no state
mutation on any reject:**

1. Flags must be exactly `FLAG_BOUNDARY|FLAG_PQ_ADV` (a combined ADV+CTXT frame stays
   `REJECT_S2_LOCAL_UNSUPPORTED` — ENG-0026 territory, untouched).
2. Open header under `hk_r`, candidates [nr, nr+1], mirror the CTXT receiver's structure;
   `n != nr` ⇒ `REJECT_S2_BOUNDARY_NOT_IN_ORDER` (in-order-only control plane; qsc delivers
   control pre-envelopes in pack order, so this matches live traffic; a lost predecessor degrades
   bounded by outbox replay / T_pq re-ADV — the existing CTXT posture, no new class).
3. Chain-consume IN-ORDER: `derive_mk_step(ck_ec, ck_pq_recv)` → open body under `mk`
   (replay of an old authentic ADV: fails header open at step 2 — nonce/counter mismatch — AND
   would fail `track_peer_adv` monotonicity; doubly rejected, no mutation).
4. Parse `body_pt`: `len >= 32` else `REJECT_S2_BODY_AUTH_FAIL`; split `adv_mac || app_payload`.
5. Verify `adv_mac` (constant-time compare) under `st.rk` ⇒ mismatch `REJECT_S2_BODY_AUTH_FAIL`.
6. `track_peer_adv` length+monotonicity checks (existing codes) — an UNAUTHENTICATED or failed
   ADV is REJECTED, never tracked.
7. Commit atomically: `ck_ec'` AND `ck_pq'` (mirror the sender: `send_pq_advertise` advances
   BOTH chains, ratchet.rs:1578-1579), `nr+1` (checked_counter_inc), `peer_max_adv_id_seen`,
   staged peer_adv. **Retires both NA-0624 workarounds**: no receive-chain gap ⇒ no mkskipped
   control-slot growth; [ADV, reseed] may share a pack (exclusion rule removed) because nr passes
   through the ADV slot before the reseed's `n == nr` check.

**Entry point:** a `Suite2RecvWireState`-level `recv_pq_adv` in ratchet.rs, routed from
`recv_wire`'s boundary dispatch (replacing the `FLAG_PQ_ADV ⇒ REJECT_S2_LOCAL_UNSUPPORTED` arm at
ratchet.rs:1071-1073 — this IS named gap (2), not a widening; the flags==0 non-boundary path is
byte-untouched). `Suite2RecvWireState` already carries everything needed (rk, ck_ec, ck_pq_recv,
nr, peer_max_adv_id_seen); no state-shape change here either.

**qsc intercept upgrade (main.rs:2559-2597):** the FLAG_PQ_ADV arm stops short-circuiting to
parse+track; it mirrors the CTXT arm: gate `qsp_scka_enabled` (seed sessions never reach it),
INJECT `recv.rk := dh.rk`, drive `recv_wire` (→ recv_pq_adv), ADOPT returned chain/root state,
then persist **session state FIRST, scka store SECOND** (G2 pin: a crash between them loses only
an untracked peer_adv — bounded by the peer's T_pq re-advertise — and can never break the chain,
accept a replay, or roll back consumed-monotonicity; contrast the CTXT path which keeps its
erase-consumed-key-BEFORE-plaintext order for the one-time-key hazard, unchanged).
`qsp_scka_adv` marker gains `auth=ok`; every reject path emits the existing reject marker and
persists NOTHING. `peer_adv_consumed_max` in the G2 mono-record: unchanged semantics.

## 4. Runtime-equivalence + frozen-surface invariants (STOP-condition check: CLEAR)

- Seed-model sessions: gated out of both SCKA arms (`qsp_scka_enabled`); `recv_wire` flags==0 and
  `send_wire` byte-untouched ⇒ `suite2_runtime_equivalence_na0198` must pass byte-for-byte — no
  mechanism in this design touches its path.
- Neither gap requires breaking any frozen non-boundary path, the equivalence test, or any vector
  set outside the two named files. **No Phase-2 STOP condition fires.**

## 5. Exact vector-regeneration list (two named files ONLY; append/replace only)

`inputs/suite2/vectors/qshield_suite2_scka_logic_vectors_v1.json` (14 now):
- CHANGED bytes: `S2-SEND-PQRESEED-ACCEPT-0001` (wire_hex: hdr_ct under NHK),
  `S2-SEND-PQADV-ACCEPT-0001` (wire_hex: body_ct +32 MAC).
- BYTE-IDENTICAL (assert in review): the 3 `scka.peer_adv.process`, 4 `scka.decap.check`,
  1 `scka.initial_epoch.map`, and all 4 send-REJECT vectors (input-level rejects, no wire).
- APPEND (ADV receive path, actor tamper plumbing `none|body|header` reused):
  `S2-RECV-PQADV-ACCEPT-0001` (authenticated, chain-consumed, new_state pinned),
  `S2-RECV-PQADV-REJECT-SPOOFED-0001` (planted/foreign-key ADV ⇒ HDR_AUTH_FAIL, no mutation),
  `S2-RECV-PQADV-REJECT-BADMAC-0001` (valid AEAD, corrupted MAC ⇒ BODY_AUTH_FAIL, no mutation),
  `S2-RECV-PQADV-REJECT-REPLAY-0001`, `S2-RECV-PQADV-REJECT-NOMAC-0001` (pre-NA-0625-format
  body ⇒ BODY_AUTH_FAIL — the ADV downgrade case).

`inputs/suite2/vectors/qshield_suite2_pq_reseed_vectors_v1.json` (5 now):
- BYTE-IDENTICAL: all 5 `suite2.pqreseed.apply` (pure logic: rk/chains/targets; no frame bytes —
  verified against vector input/expect shapes).
- APPEND (CTXT boundary receive under NHK):
  `S2-RECV-PQRESEED-NHK-ACCEPT-0001` (NHK round-trip),
  `S2-RECV-PQRESEED-REJECT-HK-DOWNGRADE-0001` (HK-sealed boundary frame ⇒ HDR_AUTH_FAIL — the
  header-downgrade rejection vector of DoD (4)).

ALL OTHER frozen vector sets byte-identical (parse_vectors are format-level, kdf_vectors are
derivation-level, e2e_recv/interop/crash_restart embed no reseed/ADV frames — forward-study
sizing confirmed against live files).

qsc e2e (tests, not vector files): NA-0624 `scka_e2e_*` green under NHK; NEW: spoofed/planted-ADV
rejection e2e; header-downgrade rejection e2e; **[ADV, reseed] one-pack round-trip** (Decision-2
proof); mkskipped stays empty across an ADV under in-order delivery (growth retirement proof).

## 6. Decision-4 slice pin — `formal/model_suite2_root_composition_bounded.py`

Two-party bounded explorer (zero-dep Python, run_model_checks.py + CI wiring like the existing
models) over event alphabet {A/B × (DH-boundary send/recv, PQ-reseed send/recv, ADV send/recv)}
with abstract tuple-hash KDFs, all interleavings to a fixed bound (target: full in-order
interleavings ≤ 6 events + the NA-0624 regression shapes: reseed-then-DH, DH-then-reseed,
ADV+reseed same pack). Asserted invariants, guarding exactly this lane's surface:
1. Root convergence: after any delivered-in-order schedule, `A.rk == B.rk` and (per party)
   `recv.rk == dh.rk` (would have caught the NA-0624 dh.rk-sync bug pre-implementation).
2. PQ healing composes: a reseed's secret survives a subsequent DH boundary in the root lineage
   (the D560-amendment property, epoch-granular PCS shape — claim boundary language untouched).
3. Chain continuity under chain-consume: nr advances exactly once per delivered frame incl. ADV;
   no receive-chain gap; mkskipped empty in-order (the Decision-2 retirement, model-level).
4. Reject ⇒ no state mutation (extended to the ADV receive events).
The ProVerif composition model remains ENG-0028's own lane.

## 7. Result-boundary conformance of this design (mutation-path audit)

Touches ONLY: ratchet.rs (recv_boundary_in_order NHK; send_pq_reseed NHK; new recv_pq_adv +
send_pq_advertise body-MAC + recv_wire ADV-boundary routing — all inside the two named gaps),
refimpl tests, actor ADV-recv op, the two vector files, qsc intercept + pack-exclusion removal +
tests, DOC-G5-008 (ENG-0023 note incl. the §8.5.1/§8.5.4 ADV tension), DOC-G5-004 (+32B ADV
observable), ledger, formal/ (Decision-4 slice), governance set. parse.rs: NOT touched.
DOC-CAN-*: NOT touched. Primitives: NOT touched. Snapshot format: NOT touched.

## 8. For the incoming (Phase-3) chat

Implement in this order: (1) refimpl NHK pair + co-located tests; (2) recv_pq_adv + send-side MAC
+ recv_wire routing + tests; (3) actor op for ADV recv (reuse tamper plumbing); (4) regenerate +
append vectors per §5, assert the BYTE-IDENTICAL list; (5) qsc intercept + pack-exclusion
retirement + e2e per §5; (6) formal slice per §6; then gates (fmt / WF-0013 workspace build /
clippy -D warnings / metadata --locked / audit), D-1245, scope guard, impl PR, bounded REST
polling. Evidence files need `git add -f`. grep -c exits 1 on zero matches (no && chains).
Never run operator startup commands.
