# NA-0626 Phase-2 DESIGN-LOCK — ENG-0024 + ENG-0026: RK unification + combined DH+PQ boundary (D563)

Goals: G1, G2, G3, G4, G5

Lane: NA-0626 (sole READY at D-1247). Directive: QSL-DIR-2026-07-09-563 (APPROVED, five operator
decisions signed). Base: main == origin/main == `842f6757` (clean). Recorded: 2026-07-09 (UTC).
Status: DESIGN-LOCK COMPLETE — no code written (Phase-2 discipline). Implementation (Phase 3+)
resumes in a fresh chat per the session-handoff convention (ONE handoff, at this boundary).

---

## 0. WF-0014 discharge — Phase 2(a) byte-scan (FIRST obligation; gates everything else)

Scanner: `docs/governance/evidence/NA-0626_wf0014_pinned_frame_scan.py` (faithful ports of
`parse.rs` `decode_suite2_wire` AND `decode_suite2_ratchet_message`, plus a strict QS2S parser).
Output (deterministic, run at `842f6757`):
`docs/governance/evidence/NA-0626_wf0014_pinned_frame_scan_output.txt`.
Scanned: all 17 `inputs/**/vectors/*.json` (directive glob) PLUS all 134 other `inputs/**/*.json`
(fail-closed superset; 6 deliberate malformed-JSON ops fixtures raw-scanned). Every hex string
was decode-attempted at BOTH the envelope level and the headerless ratchet-message level
(the NA-0625 scan covered only envelopes; parse_vectors pin headerless messages — a blind spot
this scan closes), and msg/wire-named hex that decodes as NEITHER is reported rather than skipped.

**Findings (exhaustive):**
- **Pinned wire ENVELOPES: 7** — e2e_recv: 4x `flags=0` (88 B) + 1x `flags=0x0006`
  (`S2-E2E-ACCEPT-BOUNDARY-0001`, 1180 B, hdr_ct `[1136,1160)`); scka_logic:
  `S2-SEND-PQADV-ACCEPT-0001` (`0x0005`, 1306 B), `S2-SEND-PQRESEED-ACCEPT-0001` (`0x0006`, 1178 B).
  All three flagged frames' `dh_pub` equals the session's current `dh_pub` (e2e_recv frame's
  dh_pub_sha12 matches its file's flags=0 frames) — none is a fresh-DH frame.
- **Pinned headerless MESSAGES: 1** — `S2-PARSE-ACCEPT-NONBOUNDARY-0001` (74 B, flags=0).
- **Deliberate malformed pinned frames: 6** — e2e_recv `REJECT-PARSE` (junk at flags offset) +
  5 parse_vectors rejects (incl. `flags=0x0002` CTXT-without-BOUNDARY and `0x0006` truncated
  prefix). Dispositioned below.
- **Pinned QS2S snapshots: 0.** No vector file pins snapshot bytes anywhere (crash_restart drives
  snapshot/restore at RUN TIME through the actor). **The QS2S v2→v3 bump therefore invalidates
  ZERO pinned vector bytes.**
- **rk-bearing state JSON nodes: 20** across crash_restart(4), e2e_recv(4), interop(3),
  interop_ximpl(2), scka_logic(7) — the actor `recv_state`/`new_state` schema surface.
  Exactly ONE is an EXPECTATION carrying BOTH roots: `S2-SEND-PQRESEED-ACCEPT-0001`
  `expect.output.new_state.data` has `rk` AND `dh_rk`, **and their pinned values are EQUAL**
  (verified from the live JSON).
- **Constructed-frame knobs: 32** (frames built at run time; byte-freeze-independent).

**Decision-5 artifact scope (what the regenerator must assert, fail-closed):**
- **CHANGED: exactly ONE existing vector, zero wire bytes.**
  `qshield_suite2_scka_logic_vectors_v1.json` → `S2-SEND-PQRESEED-ACCEPT-0001` →
  `expect.output.new_state.data`: the `dh_rk` member is REMOVED (§3 below removes the actor
  op's `dh_rk` output; its value is a byte-equal duplicate of `rk` today). `wire_hex` and every
  other member BYTE-IDENTICAL.
- **BYTE-IDENTICAL: everything else.** All 7 pinned envelopes' wire bytes (incl. all three
  flagged frames), the pinned parse message, all 6 malformed pins, and all other files
  byte-for-byte. The single-root refactor changes NO derivation for any existing path and the
  actor keeps its vector-facing JSON schema (§3), so no other vector content moves.
- **APPENDED (new ids only):** combined-boundary vectors per §5 — scka_logic:
  `S2-SEND-COMBINED-ACCEPT-0001` (pinned wire from the new PURE sender); pq_reseed: constructed
  receiver vectors (accept + NOT_IN_ORDER + no-DH-capability reject + reject⇒no-mutation), style
  matching the existing NHK vectors.
- Constructed-knob dispositions (semantics under the new receiver, no bytes involved):
  `S2-INTEROP-REJECT-FLAGS-0001` (flags=1 override) rejects in `send_wire` (`flags != 0` ⇒
  `REJECT_S2_LOCAL_UNSUPPORTED`) — untouched path, disposition unchanged. boundary_vectors +
  pq_reseed NHK vectors (flags=6, message dh_pub == state dh_pub) exercise the wire-level
  PQ-only path — disposition unchanged. negotiation `0x8000` (unknown flag ⇒ parse reject) and
  transcript flags overrides — untouched layers. parse_vectors semantics unchanged (parse.rs
  untouched, §5).

Any vector set changing outside this scope is a STOP (Operator Decision 5).

## 1. Phase 0/1 verification snapshot

`HEAD == origin/main == main == 842f6757`, worktree/index/untracked clean at scan time; disk 34%
(<95%); `/backup/qsl` mounted; NEXT_ACTIONS has exactly one anchored `^Status: READY` = NA-0626
(LIVE QUEUE `READY=NA-0626 | HIGHEST_NA=0626 | HIGHEST_D=1246`); DECISIONS.md highest recorded
`**ID:**` is D-1246 (D-1245 + D-1246 both present once, `Status: Accepted`, consumed); the single
`D-1247` string is D-1246's forward reference ("begins at D-1247"), no D-1247 record; highest
directive on disk = 563 (this one). On-disk qwork kv proof matches the directive splice
(`proof_written_at_utc=2026-07-09T14:57:44Z`). NA-0625 evidence read in full (design-lock,
harness incl. §8 STOP analysis), ledger entries ENG-0024/0025/0026/0030/0031 + WF-0014 read.

## 2. Pin (b) — single-`rk` state shape + QS2S v2→v3 (Operator Decision 1)

**Struct shape (refimpl `state.rs` / `ratchet.rs`):**
```rust
pub struct Suite2SessionState {
    pub rk: [u8; 32],              // THE session root (DOC-CAN-003 §8.1) — the only copy
    pub send: Suite2SendState,     // fields unchanged
    pub recv: Suite2RecvWireState, // `rk` field REMOVED; all other fields unchanged
    pub dh: Suite2DhRatchetState,  // `rk` field REMOVED; keeps dhs_priv/dhs_pub/dhr
}
```
`session_root()` and its dh-preferred fallback are DELETED (line ratchet.rs:1571 today) — with one
slot there is nothing to prefer; the interop-actor "dh left zero" case reads the same single root.
The qsc INJECT (`recv_in.rk = st.dh.rk`, main.rs:2575-2580 and 2668-2673) and ADOPT
(`next_state.dh.rk = next_state.recv.rk`, main.rs:2607 and 2693) become UNREPRESENTABLE — the
fields do not exist; the compiler enforces their removal in the unification commit (§7).

**Wire-level ops become root-explicit (pure in, pure out):** `recv_wire` gains an
`rk: &[u8; 32]` parameter and `RecvWireOutcome` gains `pub rk: [u8; 32]` (the possibly-advanced
root); `Suite2BoundaryState.rk` stays as the transient input-bundle slot (built from the
parameter; never persisted — a parameter bundle, not a second store); `recv_pq_adv` takes the
root parameter for the ADVAUTH verify (advances nothing, returns it unchanged);
`recv_boundary_in_order` unchanged in shape (already reads `st.rk` from the bundle).
`send_wire` is untouched (flags=0 never reads the root). `recv_wire_canon` mirrors the new
signature. Session-level fns (`send_boundary`, `recv_dh_boundary`, `send_pq_advertise`,
`send_pq_reseed`, the new entry points of §4/§5) read/write `st.rk` directly.

**QS2S v3 layout (snapshot_bytes/restore_bytes):**
```
"QS2S" || u8 version=3
|| rk(32)                                              // session root, serialized first
|| send:  session_id(16) pv(2) suite(2) dh_pub(32) hk_s(32) ck_ec(32) ck_pq(32) ns(4) pn(4)
|| recv:  session_id(16) pv(2) suite(2) dh_pub(32) hk_r(32) ck_ec(32) ck_pq_send(32)
          ck_pq_recv(32) nr(4) role(1) peer_max_adv_id_seen(4)
          known(4+4n) consumed(4+4n) tombstoned(4+4n) mkskipped(4+68n)
|| dh:    dhs_priv(32) dhs_pub(32) dhr(32)
```
(v2 minus both embedded `rk` copies, plus the leading session `rk`; net −32 bytes; existing
restore caps and exact-length rule unchanged.)

**`restore_bytes` v2 disposition (Decision 1, fail-closed):** magic mismatch keeps
`Invalid("bad suite2 snapshot")`; a valid magic with `version != 3` (v1, v2, v4, …) returns the
DISTINCT static error `Invalid("unsupported suite2 snapshot version")`. Deterministic (pure
function of the input bytes), never mutates the input, no migration path (charter tenet:
eliminate, do not carry; a v2 snapshot with diverged roots is not soundly migratable). The
NA-0620 `restore_rejects_non_v2_version` test flips to v3 and additionally asserts the distinct
message for a synthesized v2 header.

**qsc session blob (Decision 1):** outer blob layout v3 stays byte-compatible
(`QTRG`+trigger+scka_len+scka+QS2S); only the embedded QS2S section becomes v3-only via
`restore_bytes`. On a failed session load whose plaintext QS2S section has valid magic but
version != 3, qsc emits a DISTINCT deterministic marker (event/code named by Chat B per the
existing `emit_marker` house style, e.g. a `qsp_session_*` event distinct from generic parse
failure), mutates NOTHING on disk (test asserts the stored blob byte-unchanged after the failed
load), and the session must be re-established. The legacy plaintext-migration branches in
`protocol_state/mod.rs` (raw-QS2S and pre-SCKA layouts, lines ~647/883/915) necessarily carry a
v2 QS2S section and are therefore provably dead post-bump: REMOVE those branches in the same qsc
commit, each replaced by the distinct-unrecoverable path, with a test per removed branch.

**`recv.ck_pq_send` stays.** It is the wire-level ops' transport slot for the send-direction
reseed seed (apply_pq_reseed passes it through; vectors pin it as input AND expectation). It is
the same caller-owned-coherence CLASS as ENG-0024/0030 but NOT the named scope; the §4 entry
point makes the qsc-seam hazard moot (the send seed lands directly in `send.ck_pq`). Filed as a
ledger note for ENG-0025 re-triage at closeout — deliberately not widened into this lane.

## 3. Actor / vector-schema mapping (why the vector JSON stays byte-stable)

The actor's VECTOR-FACING JSON schemas are kept UNCHANGED for inputs: `recv_state.rk` keeps its
place in every op's params and maps to the explicit root parameter / `Suite2SessionState.rk`
internally (`parse_suite2_recv_state` returns `(Suite2RecvWireState, rk)`); ops that construct
sessions place it in the single slot. Output schemas keep every key whose value still exists.
EXACTLY ONE output key dies: `suite2.send_pq_reseed`'s `new_state.dh_rk`
(refimpl_actor_rs/src/main.rs:3258, `out.state.dh.rk` — no longer exists; its pinned value is
byte-equal to `rk` today, so removal is pure de-duplication). This is what makes the Decision-5
split provable: one changed vector (one removed JSON member), everything else byte-identical.
`tools/actors/interop_actor_py` keeps its schema identically; it changes only if its wire-level
op plumbing needs the explicit root parameter pass-through (cross-impl op SEMANTICS unchanged).

## 4. Pin (c) — session-level reseed-RECEIVE entry point (ENG-0030 structural)

```rust
pub struct RecvSessionOutcome {
    pub state: crate::suite2::state::Suite2SessionState, // FULLY updated: rk + send + recv + dh
    pub plaintext: Vec<u8>,
    pub ok: bool,
    pub reason: Option<&'static str>,
    pub pn: Option<u32>,
    pub n: Option<u32>,
}

/// Session-level SCKA reseed RECEIVE (mirrors `send_pq_reseed`), incl. the combined DH+PQ
/// boundary (§5). Caller owns the advkey store + decapsulation (DOC-CAN-004), as today.
pub fn recv_pq_reseed(
    hash: &dyn Hash, kmac: &dyn Kmac, aead: &dyn Aead, dh: &dyn X25519Dh,
    st: crate::suite2::state::Suite2SessionState,
    wire: &[u8],
    pq_epoch_ss: &[u8],
    peer_adv_id: u32,
) -> RecvSessionOutcome
```
**Commit semantics (mirror `send_pq_reseed` ratchet.rs:1810-1821 field-for-field):** on success —
`st.rk := new_rk`; `recv.{hk_r := HK(new_rk, recv_dir), ck_ec := ck_ec', ck_pq_send := seed_send,
ck_pq_recv := seed_recv, nr := nr+1, consumed/tombstoned/peer_max from apply_pq_reseed}`; AND the
send half that no caller may ever hold stale again: `send.hk_s := HK(new_rk, send_dir)`,
`send.ck_pq := seed_send`. (`send.ck_ec/ns/pn` untouched — a reseed consumes no send slot.)
**Reject ⇒ the input state is returned unmodified** (every reason code as today; no new codes).
Internal factoring (whether it wraps the wire-level open or implements directly) is Chat B's
choice; the CONTRACT is this outcome shape, this commit set, and reject-no-mutation.

**Companion for uniform arms:** `recv_pq_adv_session(...)` wrapping `recv_pq_adv` at session
level (an ADV advances no root; returns the full state so the qsc arm has no sub-state plumbing).

**qsc arm rewrite (both intercept arms):** ADV arm → `recv_pq_adv_session`; CTXT arm →
`recv_pq_reseed` (which also accepts combined frames, §5); DH-only arm keeps `recv_dh_boundary`.
DELETED in the same commit, enforced by compilation (the fields are gone): both INJECTs, both
ADOPTs, and the ENG-0030 send-half refresh (main.rs:2704-2710). Marker/persist orderings
unchanged (ADV: session-first-then-SCKA-store; CTXT: erase-consumed-key-before-plaintext).

**Test inversion (directive-mandated):**
`reseed_receiver_send_schedule_must_be_refreshed_from_advanced_root` keeps its name (which reads
correctly for the new assertion) and INVERTS from `assert_ne!` (documenting the staleness) to
`assert_eq!`: the session-level receive returns `send.hk_s`/`send.ck_pq` equal to the schedule
the peer derives, then round-trips a real advertisement built on the post-reseed schedule. It is
NOT deleted.

## 5. Pin (d) — combined DH+PQ boundary (ENG-0026; Operator Decision 2 re-present clause: DOES NOT FIRE)

**Flag shape:** `FLAG_BOUNDARY | FLAG_PQ_CTXT` = `0x0006` — the EXISTING reseed shape.
Discrimination is receiver-semantic: `parsed.dh_pub != st.dh.dhr` (fresh key ⇒ combined;
equal ⇒ PQ-only reseed; the AD binds `dh_pub`, so a tampered field fails header AEAD either
way). `0x0007` (ADV|CTXT|BOUNDARY) stays `REJECT_S2_LOCAL_UNSUPPORTED` everywhere (recv_pq_adv's
exact-flags check unchanged).

**Frame layout / NO wire FORMAT change (the Decision-2 PROOF):** the combined frame is
byte-layout-identical to today's `0x0006` reseed frame —
`dh_pub(32) || flags(2) || pq_target_id(4) || pq_ct(1088) || hdr_ct(24) || body_ct` inside the
10-byte envelope. DOC-CAN-003 §4.3 carries `DH_pub[32]` on EVERY ratchet message, so a fresh key
rides the existing field; zero new fields, zero moved bytes. **parse.rs needs NO hook** (the
directive said "expect one, justify it" — justified the other way: `parse_ratchet_header`
already decodes `0x0006` and even the both-prefix combination in canonical order; the
fresh-vs-current `dh_pub` discrimination is receiver semantics that the parse layer cannot and
must not do). Therefore: no normative DOC-CAN change beyond ENG-0031, no wire FORMAT change —
the re-present clause does not fire.

**KDF ordering (DoD 3) — PINNED: DH first, then PQ.**
```
dh_out  = X25519(receiver: dhs_priv, msg.dh_pub | sender: new_priv, dhr)
(RK_dh, CK_ec0) = KDF_RK_DH(RK_pre, dh_out)                  (§3.3.2, §8.5.2 step 5)
CK_pq0_dir      = KMAC32(RK_dh, "QSP5.0/PQ0/<dir>", [0x01])  (§8.5.2 step 6; transient, see below)
(seed_A2B, seed_B2A) = KDF_PQ_RESEED(RK_dh, target, ct, ss)  (§3.3.6 with RK_old := RK_dh)
RK_final        = KDF_RK_PQ(RK_dh, ss)                        (§3.3.3, §8.5.3 step 5)
```
Anchors, in order of force: (1) DOC-CAN-003 §8.2 establishment is normatively classical-then-PQ
(`RK0` from `dh_init`, then `KDF_RK_PQ`); (2) DOC-G5-008 §4 states the combined composition
verbatim — "This keeps the hybrid ordering (classical then PQ) that establishment already uses";
(3) §3.3.6 defines its input as "the RK value before applying `KDF_RK_PQ` for this event" — with
DH applied first that value is `RK_dh`; (4) uniqueness: the PQ-first composition would have
§8.5.2 step 6 (PQ0 chain reinit from the post-DH root) CLOBBER §8.5.3 step 6's ct-bound seeds —
DH-first is the only order in which both step lists hold verbatim with no assignment overwritten.
**Ambiguity report (NOT silent, per the DoD):** DOC-CAN-003 contains no explicit combined-receiver
step list; the ordering above is the unique reading consistent with §8.2/§3.3.6/§8.5.2/§8.5.3 and
is additionally documented in DOC-G5-008 §4. No DOC-CAN edit is NEEDED to implement it; a future
editorial note (alongside DOC-G5-008 §4's own recommended §8.5 note) is RECOMMENDED to the
operator at closeout — deliberately not taken now (the canonical unfreeze is granted for exactly
the ENG-0031 sentence and nothing else).

**Message keying + counters:** the combined frame IS message `n=0` of the new DH epoch
(mirroring `send_boundary`/`recv_dh_boundary`): header under `NHK(RK_pre)` (§8.5.1 —
pre-everything root), sealed/opened at `n == 0` ONLY (`n != 0` ⇒
`REJECT_S2_BOUNDARY_NOT_IN_ORDER`), nonce/AD on the NEW `dh_pub`; body mk =
`derive_mk_step(CK_ec0, CK_pq0_dir)` — the fresh DH-epoch chains, PRE-seed (the reseed applies to
FUTURE messages, mirroring §8.5.3's posture; `CK_pq0_dir` is transient within the event).
Receiver commits: `rk := RK_final; dh.dhr := msg.dh_pub; recv.dh_pub := msg.dh_pub;
recv.hk_r := HK(RK_final, recv_dir); recv.ck_ec := CK_ec0'; recv.ck_pq_recv := seed_recv;
recv.ck_pq_send := seed_send; recv.nr := 1; scka sets from apply_pq_reseed;
send.hk_s := HK(RK_final, send_dir); send.ck_pq := seed_send` (the §4 full-schedule guarantee).
Sender (mirror): `rk := RK_final; dh.{dhs_priv,dhs_pub} := new; send.dh_pub := new_pub;
send.hk_s := HK(RK_final, send_dir); send.ck_ec := CK_ec0'; send.ck_pq := seed_send;
send.pn := old ns; send.ns := 1; recv.ck_pq_recv := seed_recv; recv.ck_pq_send := seed_send;
recv.hk_r := HK(RK_final, recv_dir)`.

**Anti-spoof / candidate / fail-closed rules:** NHK-ONLY open (no HK trial — §8.5.1 receiver rule,
NA-0625 precedent); `dh_pub` zero ⇒ `REJECT_S2_HDR_AUTH_FAIL`; fresh `dh_pub` with no local DH
capability (`dhs_priv` zero — actor plumbing sessions) ⇒ `REJECT_S2_LOCAL_UNSUPPORTED`, no
mutation; SCKA validation = `apply_pq_reseed` UNCHANGED (frozen DOC-CAN-004 §3.4 rules, fed
`rk = RK_dh`); every reject returns the input state unmodified; NO new reason code, NO new
primitive (KMAC/X25519/ML-KEM usage identical to the existing pair of paths).

**Sender fn:** NEW pure `send_combined_boundary(hash, kmac, aead, dh, st, new_dh_priv,
new_dh_pub, pq_target_id, pq_ct, pq_epoch_ss, plaintext)` — the fresh keypair is CALLER-supplied
(the SCKA pure-function precedent: no key generation inside refimpl fns), which is what makes the
sender vector-pinnable deterministically. `send_boundary` (DH-only, self-generating) unchanged.

**qsc cadence: OUT OF SCOPE (explicit).** qsc keeps the D561 operator-decided cadence (PQ-only
reseeds co-scheduled after DH boundaries). This lane delivers the refimpl sender + receiver +
vectors and qsc's ability to RECEIVE combined frames (free via the §4 entry point). Switching
qsc's send cadence to combined boundaries is a live-behavior policy change not named by the DoD —
re-triage with ENG-0025 at closeout.

## 6. Pin (e) — runtime-equivalence restatement (Operator Decision 3, exact before/after)

`qsl/qsl-client/qsc/tests/suite2_runtime_equivalence_na0198.rs` compares TODAY:
- **Wire half:** alice's and bob's emitted wires each `assert_eq!` against the refimpl-computed
  wire (full bytes, both directions), plus plaintext round-trips.
- **State half:** FOUR checkpoints (alice-after-send, bob-after-recv, bob-after-send,
  alice-after-recv) comparing `snapshot_bytes()` equality of the qsc-persisted state against the
  refimpl-expected state — i.e. the QS2S v2 field set: send{session_id, pv, suite, dh_pub, hk_s,
  ck_ec, ck_pq, ns, pn}, recv{session_id, pv, suite, dh_pub, hk_r, **rk**, ck_ec, ck_pq_send,
  ck_pq_recv, nr, role, peer_max_adv_id_seen, known/consumed/tombstoned, mkskipped},
  dh{dhs_priv, dhs_pub, dhr, **rk**}.

AFTER (the restated gate):
- **Wire half — STRENGTHENED (condition b):** both `assert_eq!(wire, refimpl_wire)` checks stay,
  AND each seed-model wire is additionally pinned against a FIXED golden SHA-256 constant in the
  test. Today the refimpl is the only oracle — a coordinated refimpl+qsc drift passes; the golden
  digests make the seed-model wire bytes absolute. (The seeded state is deterministic, so the
  digests are stable.)
- **State half:** the four checkpoints compare QS2S **v3** `snapshot_bytes()` — the identical
  field inventory except the session root appears ONCE (`rk`) instead of twice.
- **Dropped from the comparison (condition c): exactly one 32-byte field** — the SECOND copy of
  the root (v2's `dh.rk` slot beside `recv.rk`). Reason: the duality is no longer representable
  by construction; the root's information content remains fully compared via the single `rk`.
  NOTHING else leaves the comparison.
- Seed-model wire bytes are provably unaffected by the refactor: the flags=0 send/recv paths
  read `hk_*`/`ck_*` chains and never the root; the seeded fixture built `recv.rk == dh.rk` from
  one value, which now populates the single slot.

## 7. No-window choreography + commit plan (Phases 3–4 preview for Chat B)

1. **Commit R1 (refimpl, additive):** session-level `recv_pq_reseed` + `recv_pq_adv_session` +
   `send_combined_boundary` + combined receive path + co-located tests (incl. the INVERTED
   ENG-0030 test driving the new entry point). Everything compiles; the qsc mitigations remain
   present and correct; no window opens.
2. **Commit R2 (workspace-atomic unification):** remove `recv.rk`/`dh.rk`, add session `rk`,
   delete `session_root`, re-thread the wire-op signatures, QS2S v3 encode/restore (+ distinct
   version error), actor mapping (§3, incl. dropping the `dh_rk` output), qsc arm rewrite to the
   session entry points with INJECT/ADOPT/send-half-refresh deletion, qsc blob v2-section
   unrecoverable marker + dead-branch removal, equivalence-test restatement (§6). One commit
   BECAUSE the field removal makes every stale dance and every old signature a compile error —
   the "removed only in the commit that lands its structural replacement" obligation is
   discharged by the type system, not by discipline. (`cargo build --workspace --all-targets`
   before push — WF-0013.)
3. **Commit R3 (vectors):** commit the Phase-2(a) scan evidence FIRST (it is already written to
   `docs/governance/evidence/`, untracked — `git add -f`), then the regenerator + proof asserting
   the §0 changed/byte-identical/appended split fail-closed (NA-0625 regenerator pattern), then
   the regenerated + appended vector files. A set changing that §0 did not predict is a STOP.
4. **Commit R4:** formal model replacement (§8), DOC-CAN-003 ENG-0031 one-sentence edit (§9),
   DOC-G5-008 ENG-0024/0026 note, DOC-G5-004 observable note ONLY if any emitted size changed
   (none expected: combined frame size == existing reseed frame size, 1178 B for an 18-B body ct),
   ledger updates, governance (D-1247), testplan.
   (Ordering within the branch may merge R3/R4; the scan-evidence-before-regeneration ordering is
   binding.)

Phase-5 gate list: derive MECHANICALLY from every `scripts/ci/*.py` invoked by the
`.github/workflows/*.yml` this change touches (incl. `validate_suite2_vectors.py` and
`formal/run_model_checks.py`) — not from memory (WF-0014). `goal-lint` needs the `Goals:` line in
the PR body. Evidence files need `git add -f`. `grep -c` exits 1 on zero matches. Full
`cargo test -p qsc` ≈ 80 min — start early, wait-work per the packet. Merge commits only; bounded
REST polling; never `--watch`.

## 8. Formal model replacement (carried NA-0625 Decision-4 slice; vacuity forbidden)

`formal/model_suite2_root_composition_bounded.py`: the per-party `recv.rk == dh.rk` invariant is
TRIVIALLY true after ENG-0024 — REPLACE, not delete:
- Model the SINGLE root per party; keep two-party root convergence (`A.rk == B.rk` after any
  in-order delivered schedule).
- Event alphabet += combined DH+PQ boundary (send/recv), alongside DH boundary, PQ reseed, ADV.
- Invariants: (1) root convergence incl. combined events; (2) PQ healing composes — a combined
  boundary's PQ secret survives subsequent DH boundaries in the root lineage AND a combined
  boundary heals like a reseed; (3) chain continuity — combined ⇒ receiver `nr := 1`, no
  mkskipped growth in-order, `[ADV, combined]` pack behavior consistent with chain-consume;
  (4) send/recv schedule coherence is STRUCTURAL: after any reseed-receive or combined-receive
  the receiver's send schedule equals what the peer will open with (the ENG-0030 invariant,
  now asserted OF the entry point rather than of the qsc mitigation); (5) reject ⇒ no mutation
  extended to combined tamper shapes (wrong-NHK header, `n != 0`, stale `dh_pub` replay).
- Regression shapes: keep the six NA-0625 shapes (the ENG-0030 shape INVERTED to match §4) and
  add: combined-boundary round trip; the PQ-first mis-composition COUNTERFACTUAL (assert the
  model detects seed-clobbering if the order were flipped — pins §5's ordering at model level);
  combined-then-DH healing persistence.
- Wired into `formal/run_model_checks.py` as today (runtime target of the same order, ~seconds).

## 9. Pin (f) — ENG-0031 sentence, verbatim (Operator Decision 4; bounded canonical unfreeze)

`docs/canonical/DOC-CAN-003_QSP_Suite-2_True_Triple_Ratchet_v5.0.0_DRAFT.md`, §8.5.1, sender
bullet — the EXACT one-sentence replacement (side by side):

- **OLD (current file text, first bullet of §8.5.1):**
  > A boundary message header MUST be encrypted under the sender’s `NHK_s` derived from the
  > **pre-boundary** `RK`.
- **NEW (one sentence):**
  > A boundary message header that applies an epoch transition (DH ratchet advancement and/or an
  > SCKA reseed event) MUST be encrypted under the sender’s `NHK_s` derived from the
  > **pre-boundary** `RK`; an advertisement-only boundary (`FLAG_PQ_ADV` without an epoch
  > transition) advances no root and its header remains under the sender’s current `HK_s`.

One-sentence diff; NOTHING else in `docs/canonical/**` changes (the unfreeze is granted for
exactly this). It scopes the sender rule to epoch-creating boundaries — matching §8.5.4's
silence, §8.5.1's own receiver sentence ("a boundary epoch transition"), and the shipped
implementation (ADV header under `HK_s`, authenticated by the ADVAUTH root MAC). The rejected
alternative (NHK flip for the ADV header) stays rejected per Decision 4: an ADV advances no
root, so NHK confers zero attacker advantage, and the flip would churn the ADV vectors for
nothing. The combined boundary of §5 is an epoch transition and is unambiguously NHK under both
the old and new sentence.

## 10. STOP-condition sweep (all clear) + result-boundary audit

- No normative DOC-CAN change beyond the single ENG-0031 sentence (§5 proves the combined
  boundary implements existing normative text; §9 is the granted sentence). ✓
- No wire FORMAT change (§5 proof — §4.3 layout admits the combined frame byte-identically). ✓
- No KDF/AEAD/KEM primitive change (every derivation above is an existing labeled KDF). ✓
- The migration, the reseed-receive entry point, and the combined boundary are all pinned. ✓
- The qsc ENG-0030 / dh.rk-sync mitigations are removed only in the compiler-enforced
  unification commit that replaces them (§7). ✓
- Vector artifact scope is byte-proven (§0); the regenerator will fail closed on any deviation. ✓
- Planned mutation paths ⊆ the directive's Result boundary (refimpl suite2 state/ratchet/scka
  callers, refimpl tests, both actors, `inputs/suite2/vectors/**`, qsc src+tests, DOC-CAN-003
  (ENG-0031 only), DOC-G5-008, DOC-G5-004 (only if an observable changes — none expected),
  `formal/**`, ledger, governance). parse.rs NOT touched (justified §5). `apps/**`, Cargo,
  `.github`, `.claude`: untouched. ✓

## 11. Workspace state at Phase-3 resume (Chat B's re-verify)

`git status --porcelain` reads CLEAN — the three evidence files created by this chat live under
`docs/governance/evidence/`, which `.gitignore:65` (`**/evidence/`) ignores (this is exactly why
the lane convention requires `git add -f` for evidence). Verify their presence with `ls` or
`git status --ignored=matching -- docs/governance/evidence/`; commit them FIRST in R3 order (§7):
```
docs/governance/evidence/NA-0626_wf0014_pinned_frame_scan.py
docs/governance/evidence/NA-0626_wf0014_pinned_frame_scan_output.txt
docs/governance/evidence/NA-0626_design_lock.md
```
Any TRACKED-file dirt or NON-ignored untracked file at resume is a STOP-grade inconsistency.
