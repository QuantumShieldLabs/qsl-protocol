# NA-0625 — ENG-0023 Suite-2 spec-alignment — implementation evidence

Directive: QSL-DIR-2026-07-09-562 (D562). Decision: D-1245. Base: main == `bc512f2e` (clean).
Recorded: 2026-07-09 (UTC). Design-lock: `docs/governance/evidence/NA-0625_design_lock.md`.

## 1. Phase 0 re-verification (live, at Phase-3 resume)

```
git rev-parse HEAD origin/main main   -> bc512f2e… (all three identical)
git status --porcelain                -> clean
df -h /srv/qbuild                     -> 50% used  (< 95% gate)
mountpoint /backup/qsl                -> /backup/qsl is a mountpoint
DECISIONS.md                          -> highest D-1244; D-1245 ABSENT at start
NEXT_ACTIONS.md                       -> exactly one `^Status: READY` (NA-0625)
```
No STOP condition. Phases 0–2 were cleared in the prior chats (see the two appended sections of the
archived directive); the design-lock crux was resolved there and is NOT re-litigated here.

## 2. What changed (mutation-path audit against the STRICT SCOPE)

| Path | Change | In directive's allowed set? |
| --- | --- | --- |
| `tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs` | NHK pair (recv+send); `recv_pq_adv`; ADV body MAC in `send_pq_advertise`; `recv_wire` ADV routing; `header_key` made `pub` for the harness | YES (the two named changes only) |
| `tools/refimpl/quantumshield_refimpl/tests/suite2_scka_sender.rs` | 3 new integration tests | YES |
| `tools/actors/refimpl_actor_rs/src/main.rs` | new `suite2.recv_pq_adv` op; `suite2.boundary.run` gains `hdr_key: nhk\|hk` | YES |
| `inputs/suite2/vectors/qshield_suite2_scka_logic_vectors_v1.json` | 2 replaced, 5 appended | YES (named file) |
| `inputs/suite2/vectors/qshield_suite2_pq_reseed_vectors_v1.json` | 2 appended | YES (named file) |
| `qsl/qsl-client/qsc/src/main.rs` | authenticated ADV intercept; pack-exclusion removal; ENG-0030 caller-side fix | YES |
| `qsl/qsl-client/qsc/tests/handshake_mvp.rs` | 2 new e2e; 2 updated e2e | YES |
| `formal/model_suite2_root_composition_bounded.py` (new), `formal/run_model_checks.py`, `formal/README.md` | Decision-4 slice + wiring | YES (Decision-4 slice only) |
| `docs/design/DOC-G5-008_*`, `docs/design/DOC-G5-004_*` | ENG-0023 note; +32 B ADV observable | YES |
| `docs/ops/IMPROVEMENT_LEDGER.md` | ENG-0023 DONE; ENG-0030 + ENG-0031 filed | YES |
| `DECISIONS.md`, `NEXT_ACTIONS.md`, `TRACEABILITY.md`, `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`, `tests/NA-0625_*`, `docs/governance/evidence/NA-0625_*` | governance | YES |

**NOT touched (verified by `git status --porcelain`):** `tools/refimpl/.../suite2/parse.rs`;
`docs/canonical/**`; `tools/refimpl/.../suite2/state.rs` (no snapshot change);
`tools/refimpl/.../suite2/scka.rs` (`apply_pq_reseed` validation rules); `Cargo.toml`/`Cargo.lock`;
`.github/**`; `.claude/**`; `qsl/qsl-server/**`; `qsl-attachments`.

## 3. The two named changes, precisely

**Gap (1) NHK.** `recv_boundary_in_order` derives
`nhk_r = header_key(kmac, &st.rk, a2b_recv, next=true)` before the candidate loop and opens ONLY
under it (candidates `[nr, nr+1]`, `n == nr` retained). `send_pq_reseed` derives
`nhk_s = header_key(kmac, &rk_old, a2b_send, next=true)` where `rk_old = session_root(&st)` and
seals the boundary header under it. The post-commit `hk_s`/`hk_r` recomputation from `new_rk`
(`next=false`, §8.5.3 step 7) is unchanged. No state-shape change anywhere.

**Gap (2) AUTH-ADV.**
```
adv_mac = KMAC32(RK, "QSP5.0/ADVAUTH", u32be(pq_adv_id) || pq_adv_pub || [0x01])
body_pt = adv_mac(32) || app_payload
```
`recv_pq_adv` (Suite2RecvWireState-level) enforces, in order: flags exactly `BOUNDARY|PQ_ADV`;
`hdr_ct` length; `body_ct` minimum; header open under `hk_r` at `[nr, nr+1]`; `n == nr`; chain keys
set; `derive_mk_step` on `(ck_ec, ck_pq_recv)`; body open; `len(body_pt) >= 32`; constant-time MAC
compare under `st.rk`; `track_peer_adv(watermark, id, pub)`; `checked_counter_inc(n)`. Only then does
it commit `ck_ec'`, `ck_pq_recv'`, `nr+1`. **Every reject returns the input state unmodified.**

**Watermark design note (worth recording).** `recv_pq_adv` takes the peer-ADV watermark as a
CALLER-OWNED parameter rather than reading `st.peer_max_adv_id_seen`. That session field is the
frozen CTXT receiver's watermark for OUR OWN advertised targets (`apply_pq_reseed` compares the
peer's reseed `peer_adv_id` against it). The two id spaces are independent — both parties allocate
from their own counters — so tracking the peer's ADV id 1 in that field would make the peer's next
reseed of our target id 1 reject `REJECT_SCKA_ADV_NONMONOTONIC`. This was caught by the
`adv_then_reseed_same_pack_round_trips` test on its first run and is now pinned by an explicit
assertion that `recv_pq_adv` leaves `peer_max_adv_id_seen` untouched. On the wire path, `recv_wire`
passes the existing `peer_adv_id: Option<u32>` parameter through as the watermark (None ⇒ 0); qsc
supplies the SCKA store's `peer_adv_max_seen`.

## 4. Vector byte-identity proof

Regenerator: `docs/governance/evidence/NA-0625_vector_regen.py` (archived).
Machine-checked output: `docs/governance/evidence/NA-0625_vector_regen_proof.json.txt`.

```
changed (exactly 2):
  scka_logic:S2-SEND-PQADV-ACCEPT-0001        (wire_hex only; body_ct +32)
  scka_logic:S2-SEND-PQRESEED-ACCEPT-0001     (wire_hex only; hdr_ct under NHK)
byte_identical (17): the 12 remaining scka_logic + all 5 pq_reseed pre-existing vectors
appended (7): 5x S2-RECV-PQADV-*, 2x S2-RECV-PQRESEED-*
```
The script asserts all three sets before writing; it fails closed if the changed set is anything
other than the two named vectors, if the byte-identical count is not 17, or if the appended count is
not 7. Both files round-trip byte-identically through `json.dumps(..., indent=2) + "\n"`, so the
diff carries no formatting noise.

Runner results after regeneration: `scka_logic 19/19`, `pq_reseed 7/7`. All other frozen sets are
untouched on disk; all but one are green: boundary 4/4, parse 6/6, kdf 6/6, transcript 4/4,
mk_hybrid 3/3, establish 14/14, ooo_replay 6/6, crash_restart 3/3, interop 3/3, interop_ximpl 2/2,
scka_kem 5/5, downgrade 5/5.

**`e2e_recv` is the THIRD named file — see §8 (STOP raised, resolved by Operator Decision 5). Now 4/4.**

## 5. The ENG-0030 finding (how it surfaced, how it was proved)

The first full `scka_e2e` run after the qsc intercept upgrade failed with
`event=qsp_scka_adv code=qsp_hdr_auth_failed dir=recv ok=false` at the m5 step — Alice's rotated
advertisement, packed as a control pre-envelope alongside her DH boundary, after she had RECEIVED
Bob's reseed.

Root cause, read from the code rather than guessed: `send_pq_reseed` (session-level) commits
`st.send.hk_s = hk_s_new`, `st.send.ck_pq = ck_pq_send_after`, `st.recv.hk_r = hk_r_new`, ... for the
SENDER. The receive path is `recv_wire` → `recv_boundary_in_order`, which operates on
`Suite2RecvWireState` and can only return recv-side state (`rk`, `hk_r`, `ck_pq_send`, `ck_pq_recv`,
`ck_ec`, `nr`). qsc's CTXT arm copied that into `next_state.recv` and never refreshed
`next_state.send.{hk_s, ck_pq}` — so the receiver's send schedule stayed on the pre-reseed root while
the peer's receive schedule had moved. (The receiver's correct post-reseed send PQ chain is
`recv.ck_pq_send`, which `apply_pq_reseed` derived.)

Why it was latent: the reply-driven trigger makes any send after a receive a DH boundary, and
`send_boundary` reinitialises both `hk_s` and `ck_pq` from the new DH root. Only a control
pre-envelope — which is packed BEFORE the boundary check and rides the CURRENT send chain — reads
the stale values, and before NA-0625 nobody opened an ADV header.

Fix (qsc CTXT arm, beside the dh.rk ADOPT):
```rust
next_state.dh.rk = next_state.recv.rk;                                    // NA-0624 ADOPT
next_state.send.hk_s = header_key(&c, &next_state.recv.rk,
                                  next_state.recv.role_is_a, false)?;     // NA-0625
next_state.send.ck_pq = next_state.recv.ck_pq_send;                       // NA-0625
```
Pinned by `reseed_receiver_send_schedule_must_be_refreshed_from_advanced_root` (asserts BOTH the
staleness and the post-composition coherence, then round-trips a real advertisement through
`recv_pq_adv`), by the `scka_e2e_*` proofs, and by invariant 4 of the bounded model.

Filed as ENG-0030 with the structural fix (a session-level reseed-receive entry point returning a
fully updated `Suite2SessionState`) recommended into ENG-0024/ENG-0025.

## 6. Formal model (Operator Decision 4)

`formal/model_suite2_root_composition_bounded.py`, wired into `formal/run_model_checks.py`.

```
Root composition states: 15494
Root composition transitions: 23886
Root composition unique visited: 15494
Root composition regression shapes: 6
```
(~1.5 s.) Regression shapes asserted directly: reseed→DH and DH→reseed both keep the PQ secret in the
root lineage; `[ADV, reseed]` in one pack round-trips; WITHOUT chain-consume the in-pack reseed
fails the in-order check (the counterfactual the retired exclusion rule existed to avoid); the
HK-downgrade boundary frame is rejected with no mutation; and the ENG-0030 stale-send-schedule
advertisement is rejected while the correctly-composed one authenticates.

Two modelling errors were caught by the model's own invariants while writing it and fixed in the
model (not the protocol): `recv_dh_boundary` must leave the receiver's SEND half untouched (a DH
boundary reinitialises only the receiving direction), and the ADV watermark is per-direction.

## 7. Claim boundary

Unchanged. ENG-0023 is closed; the control plane is authenticated; the relay-inbox ADV-injection
vector is eliminated. The project still makes NO post-quantum, Triple-Ratchet, post-compromise, or
self-healing claim. The bounded model abstracts crypto to injective tuple hashes and therefore proves
agreement/coherence, NOT secrecy — the independent DH+PQ composition analysis remains ENG-0028.

## 8. STOP — a frozen vector set OUTSIDE the two named files was invalidated by gap (1)

**Status: RAISED at the merge boundary, RESOLVED by Operator Decision 5 (D562 addendum, 2026-07-09),
which extended the NAMED, REVIEWED vector-file list from two files to three with a bounded,
machine-checked mutation. Executed. All 15 suite2 vector runners are green.** The record below is
kept in full because the root cause (a vector-freeze claim asserted from a prose note rather than
from the bytes) is filed as WF-0014.

### The finding
`inputs/suite2/vectors/qshield_suite2_e2e_recv_vectors_v1.json` -> `S2-E2E-ACCEPT-BOUNDARY-0001`
now fails:

```
Suite-2 e2e recv vectors FAILED: 1 failing of 4
- S2-E2E-ACCEPT-BOUNDARY-0001 suite2.e2e.recv:
    {'code': 'INVALID', 'message': 'invalid request: reject: REJECT_S2_HDR_AUTH_FAIL'}
```

The rejection is **correct** — it is the §8.5.1 rule doing exactly what this lane implemented. That
vector's `input.steps[0].wire_hex` is a byte-pinned 1180-byte Suite-2 frame with
`flags = 0x0006 (FLAG_PQ_CTXT | FLAG_BOUNDARY)` whose 24-byte header ciphertext was sealed under the
ordinary `HK_r` by the pre-NA-0625 sender. Under the NHK-only boundary receiver no conformant
implementation can accept it. This is the same frame class as the new, intentional
`S2-RECV-PQRESEED-REJECT-HK-DOWNGRADE-0001` vector.

### Exhaustive impact analysis (this is the whole blast radius)
A scan of every `inputs/suite2/vectors/*.json` for pinned byte strings that decode as a Suite-2 wire
envelope with `flags != 0` finds exactly THREE, and only one lives outside the two named files:

| File | Vector | Field | flags |
| --- | --- | --- | --- |
| `qshield_suite2_scka_logic_vectors_v1.json` (NAMED) | `S2-SEND-PQADV-ACCEPT-0001` | `expect.output.wire_hex` | `0x0005` ADV\|BOUNDARY |
| `qshield_suite2_scka_logic_vectors_v1.json` (NAMED) | `S2-SEND-PQRESEED-ACCEPT-0001` | `expect.output.wire_hex` | `0x0006` CTXT\|BOUNDARY |
| **`qshield_suite2_e2e_recv_vectors_v1.json` (NOT NAMED)** | **`S2-E2E-ACCEPT-BOUNDARY-0001`** | **`input.steps[0].wire_hex`** | **`0x0006` CTXT\|BOUNDARY** |

The other 3 vectors in `e2e_recv` carry `flags = 0` frames and pass. `qshield_suite2_boundary_vectors_v1.json`
contains PQ-CTXT boundary vectors but pins NO wire bytes — the actor CONSTRUCTS the frame at run
time (`suite2.boundary.run`, which this lane taught to seal under NHK by default), which is why it
is legitimately 4/4 and byte-identical. Every other frozen set is unaffected.

### Why the design-lock missed it
The NA-0625 forward study asserted "e2e_recv/interop/crash_restart embed NO reseed frames", and the
design-lock §5 repeated that as "verified against live files". It was not actually verified against
the bytes: `S2-E2E-ACCEPT-BOUNDARY-0001` embeds one. The rest of the design-lock's vector sizing is
correct (the two named files' 2-changed / 17-byte-identical / 7-appended split held exactly).

### Why this is a STOP and not an executor decision
The directive's STOP CONDITIONS name it verbatim: *"design-lock finds either gap cannot close
without breaking ... a vector set outside the two named files"*, and Phase 4 adds *"STOP on any
regression outside the two named surfaces."* The standing phrasing also binds: *"Regenerated
conformance vectors are a NAMED, REVIEWED artifact (append/replace only within the two named vector
files)."* The operator sized this lane's risk profile as "one unfreeze, ~19 vectors in 2 files".
Silently regenerating a third frozen conformance file would break that contract, so it was not done.

Gap (1) **cannot** close while this vector stands: an HK-sealed boundary header and an NHK-only
boundary receiver are mutually exclusive by construction.

### Recommended resolution (minimal, one field)
Re-seal that vector's header ciphertext under the NHK derived from its own `recv_state.rk`
(`rk = a5a5…a5`, `role = A`, so the label is `QSP5.0/NHK/B->A`), i.e. regenerate
`input.steps[0].wire_hex` bytes `[1136, 1160)` only:

```
offset 0      envelope (10)
offset 10     dh_pub  (32)
offset 42     flags   (2)   = 0x0006
offset 44     pq_target_id (4) || pq_ct (1088)
offset 1136   hdr_ct  (24)  <-- ONLY THIS CHANGES (re-sealed under NHK, not HK)
offset 1160   body_ct (20)
```

`body_ct` is keyed by the message key from the (unchanged) `ck_ec`/`ck_pq_recv`, and `dh_pub`,
`flags`, `pq_prefix`, `pq_bind` and the AD layouts are byte-unchanged — so the vector's
`expect_nr`, `expect_ok`, and `expect_plaintext_hex` all stay the same, and the other three vectors
in the file stay byte-identical. That is a one-vector, one-field change in a third file.

**Alternative** (if the operator prefers to keep `e2e_recv` byte-frozen): re-shape that vector to
`kind: negative` with `reason_code: REJECT_S2_HDR_AUTH_FAIL` — it then becomes a second
HK-downgrade rejection vector. This is a larger semantic edit to the file and duplicates
`S2-RECV-PQRESEED-REJECT-HK-DOWNGRADE-0001`; NOT recommended.

Either way the operator must extend the named vector-file list from two to three (or explicitly
re-scope), because the frame is byte-pinned in a file this directive froze.

### Resolution as executed (Operator Decision 5)
Bounds approved and enforced by `docs/governance/evidence/NA-0625_e2e_recv_vector_regen.py`, which
fails closed on any violation:
- only `S2-E2E-ACCEPT-BOUNDARY-0001` may change; only `input.steps[0].wire_hex`; only bytes
  `[1136, 1160)`;
- the replacement ciphertext is produced BY THE REFERENCE IMPLEMENTATION, not re-derived by the
  tooling: the script drives `suite2.send_pq_reseed` through the refimpl actor configured as the
  ORIGINATING PEER (role B, whose send chains are the vector receiver's receive chains and whose
  session root is the same pre-reseed `RK`), so the sender seals under
  `NHK_s = header_key(rk_old, "B->A", next=true)` — bit-for-bit the key the vector's receiver derives
  as `nhk_r`. The script then asserts the produced wire differs from the pinned wire in EXACTLY the
  `[1136, 1160)` window before splicing;
- that vector's `recv_state`, `expect`, and every non-`wire_hex` step field are asserted identical;
- the three sibling vectors are asserted byte-identical.

Machine-checked result (`..._e2e_recv_vector_regen_proof.json.txt`):

```
changed: ["S2-E2E-ACCEPT-BOUNDARY-0001"]   field: input.steps[0].wire_hex
changed_byte_range: [1136, 1160]           changed_bytes: 24
old_hdr_ct: 93f8f43e743afdb72aee54056892d5aa12fb93c4af27f683
new_hdr_ct: be79e96495bb6257902c8ba4475690fd6341ac113148db0c
byte_identical_siblings: [S2-E2E-ACCEPT-NONBOUNDARY-0001, S2-E2E-ACCEPT-OOO-0001, S2-E2E-REJECT-PARSE-0001]
expect_fields_unchanged: true
```
`git diff --numstat` on the file: `1 1` (one line). Runner: `e2e_recv 4/4`.

Rejected alternatives, for the record: re-shaping the vector to `kind: negative` (a larger semantic
edit that duplicates the new `S2-RECV-PQRESEED-REJECT-HK-DOWNGRADE-0001` and destroys the only
multi-step accepting-boundary e2e proof); and splitting gap (1) into its own lane (re-runs the whole
gate stack and breaks the deliberate "one unfreeze, one vector regeneration" bundling).

### State at the STOP (before the resolution)
Working tree: implementation complete, all governance written, all other gates green (fmt; workspace
build WF-0013; clippy clean on the three lane crates; `cargo metadata --locked`; `cargo audit`;
`formal/run_model_checks.py`; refimpl 112/112; full `cargo test -p qsc` exit 0; the seed-model
runtime-equivalence test byte-for-byte). NOT done, pending this decision: the impl PR, the merge, the
post-merge verification, the Phase-7 successor triage, and the D-1246 closeout.

(All of that work then proceeded to PR/merge once the operator resolved the STOP.)

One pre-existing, out-of-scope lint also surfaced and was deliberately NOT fixed:
`cargo clippy --workspace --all-targets -- -D warnings` fails on
`apps/qshield-cli/tests/na_0318_qshield_ack_commit.rs:150` (`needless_borrow`, clippy 1.95.0). That
file is untouched by this lane (last modified by NA-0318, commit `4ba069c6`), `apps/**` is not in
this directive's allowed mutation paths, and no CI workflow runs clippy. Reported, not fixed.
