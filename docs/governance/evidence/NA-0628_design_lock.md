# NA-0628 — DESIGN-LOCK (ENG-0034: reject non-contributory / low-order X25519 on every LIVE DH path)

Goals: G1, G2, G4

Lane: NA-0628. Directive: QSL-DIR-2026-07-10-565 (D565) **as amended by AMENDMENT 1 (D565-A1,
approved 2026-07-10)**, which struck DoD 4 + Operator Decision 1 (the ENG-0019 arm) after Phase 0
falsified D565's CORRECTED FACT 2. Base: `main == 1fdd5b9b`. Recorded 2026-07-10 (UTC).
**No product code was written before this design-lock. Every line number below was RE-DERIVED live;
D565's line numbers are evidence, not scripture.**

---

## 0. Phase-0 state (verified live, not copied)

`HEAD == main == origin/main == 1fdd5b9b347edc694e6ac29a8433f1c5acc286dd` (`ls-remote` confirmed);
`git status --porcelain` empty; on-disk kv proof matches the D565 splice on every REQUIRED field;
exactly ONE anchored `^Status: READY` and it is NA-0628; highest canonical decision `D-1250` with
**D-1251 ABSENT**; highest directive 565; disk 34%; `/backup/qsl` mounted.

**Surface re-verification (D565's claims, re-derived):**

| Claim | Verdict |
|---|---|
| 4 `dh.dh(` sites in `suite2/ratchet.rs` at `:1306/:1475/:1885/:2390` | **CONFIRMED**, exact |
| `hs_dh_shared` defined once, `qsc/src/handshake/mod.rs:801`, already returns `Result` | **CONFIRMED** |
| its two call sites `:1449` (initiator) / `:1877` (responder) | **CONFIRMED** |
| `qsc` never uses `refimpl::qsp` | **CONFIRMED** (the `qsp_protocol_gate` CI test is a name collision: it asserts qsc's own `qsp_pack`/`qsp_unpack` log events and imports no refimpl) |
| `qsp/**` has zero callers outside the module | **FALSE — see D565-A1.1.** ENG-0019 left this lane. |

---

## 1. FIX SHAPE (Operator Decision 2) — (b) contained guard + (c) mandatory anti-regression scan

**Chosen: (b) + (c), as approved.** The trait `X25519Dh::dh` keeps its `[u8;32]` return.

### 1.1 WF-0015 caller surface, RE-DERIVED LIVE (not copied from D565)

The rule earns its keep by being computed, so here is the computation. Live inventory of **every**
`.dh(` call site in the repo's Rust sources — **21 sites across 8 files**:

| File | Sites | Disposition under shape (b) |
|---|---|---|
| `tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs` | 6 | **GUARD the 4 production sites** `:1306`, `:1475`, `:1885`, `:2390`; 2 are in-file `#[cfg(test)]` scaffolding |
| `tools/refimpl/quantumshield_refimpl/src/qsp/ratchet.rs` | 4 (`:107`, `:158`, `:265`, `:462`) | allowlisted (legacy Suite-1/1B conformance; ENG-0019) |
| `tools/refimpl/quantumshield_refimpl/src/qsp/handshake.rs` | 4 (`:134`, `:145`, `:285`, `:297`) | allowlisted (same) |
| `qsl/qsl-client/qsc/src/handshake/mod.rs` | 1 (inside `hs_dh_shared`) | **GUARD** — covers both call sites at once |
| `tools/refimpl/quantumshield_refimpl/tests/suite2_combined_boundary.rs` | 2 | allowlisted (test scaffolding) |
| `tools/refimpl/quantumshield_refimpl/tests/suite2_scka_sender.rs` | 1 | allowlisted (test scaffolding) |
| `tools/actors/refimpl_actor_rs/src/main.rs` | 1 | allowlisted (actor-local `DhDet`) |
| `apps/qsl-tui/src/demo.rs` | 2 | allowlisted (demo; **boundary-FORBIDDEN path**) |

**Trait implementations (7):** `StdCrypto` (`crypto/stdcrypto.rs:167`); `DummyDh` ×4 (`qsp/mod.rs`,
`qsp/ratchet.rs`, `qsp/handshake.rs`, `tests/na_0071_header_key_derivation.rs`); `DhDet` ×2 (the
orphaned repo-root `main.rs:171`, `tools/actors/refimpl_actor_rs/src/main.rs:949`).

**Caller surface of THIS change = ZERO.** Shape (b) changes no signature:
- the trait `X25519Dh::dh` is untouched → all 7 impls and all 21 call sites compile unchanged;
- `hs_dh_shared` **already** returns `Result<[u8;32], &'static str>` → its signature is unchanged, so
  its two call sites `:1449`/`:1877` are untouched;
- the four refimpl guards return through each function's **existing** error channel (`Err(&'static str)`
  for the two senders; the local `reject!` macro for the two receivers) → no signature change, no
  caller change.

Therefore **`apps/**`, `tools/actors/**`, `.github/**`, and Cargo/lockfile are not touched.** This is
exactly the blast radius Decision 2 chose (b) to avoid: shape (a) would have forced edits to
`apps/qsl-tui/src/demo.rs:378-379`, a boundary-FORBIDDEN path (the NA-0626 lesson, ENG-0032).

### 1.2 NEW FACT (not in D565, recorded here): these are SHIPPED-CLIENT paths

D565 describes the four refimpl sites as "refimpl Suite-2 ratchet". They are more than that. The
shipped `qsc` client calls the refimpl ratchet **directly**:

```
qsl/qsl-client/qsc/src/main.rs:23    use quantumshield_refimpl::suite2::ratchet::{
                                       recv_dh_boundary, recv_pq_adv_session, recv_pq_reseed,
                                       send_boundary, send_pq_advertise, send_pq_reseed};
qsl/qsl-client/qsc/src/main.rs:2320  send_boundary(...)       -> site :1306
qsl/qsl-client/qsc/src/main.rs:2657  recv_pq_reseed(...)      -> site :2390 (via recv_combined_boundary)
qsl/qsl-client/qsc/src/main.rs:2683  recv_dh_boundary(...)    -> site :1475
```

So sites `:1306`, `:1475`, `:2390` are on the shipped client's live ratchet path, and `:1885`
(`send_combined_boundary`) is exercised by the conformance actor. Together with `hs_dh_shared` (the
shipped establishment DH), **all six guarded sites are live code, and three of them are in the
client's own send/receive path.** This strengthens, and does not alter, the lane's rationale.

### 1.3 The guard predicate, and why the OUTPUT check is necessary and sufficient

`is_zero32(&dh_out)` — reusing the existing private helper `suite2/ratchet.rs:52`.

RFC 7748 §6.1 requires protocols needing contributory behaviour to check the all-zero DH **output**.
X25519 clamps the scalar to a multiple of 8, so every small-order peer point maps to the identity and
yields the all-zero output, *whatever the scalar*. **Proved, not assumed** — a dependency-free
Montgomery-ladder implementation of RFC 7748 (self-checked against the §5.2 test vector) was run over
all eight classical low-order encodings against three structurally different clamped scalars:

| encoding | rejected by the EXISTING `is_zero32(dh_pub)` ingress check? | `dh_out == 0` for every scalar? |
|---|---|---|
| `00…00` (u=0) | **yes** | yes |
| `01 00…00` (u=1) | no | **yes** |
| `e0eb7a…b800` (order 8) | no | **yes** |
| `5f9c95…1157` (order 8) | no | **yes** |
| `ecff…ff7f` (u = p−1) | no | **yes** |
| `edff…ff7f` (u = p, non-canonical) | no | **yes** |
| `eeff…ff7f` (u = p+1, non-canonical) | no | **yes** |
| `ecff…ffff` (u = p−1, high bit set) | no | **yes** |

Negative control: an honest peer public key (`X25519(0x77…, basepoint)`) yields a non-zero output for
every scalar.

Two consequences, both load-bearing:

1. **The output check catches all eight encodings**, including the two non-canonical ones and the
   high-bit variant. An ingress small-order screen is therefore *not required* (D565 CORRECTED FACT 3
   confirmed), and would additionally have to handle non-canonical encodings to be complete.
2. **The new guard is not shadowed by the existing ingress check.** `is_zero32(&parsed.dh_pub)`
   (`ratchet.rs:1420`, `:2317`) rejects exactly ONE of the eight. The other **seven** reach `dh.dh(`
   today and produce an all-zero shared secret with no signal. This is precisely why the negative
   vectors below use `u=1` rather than the all-zero encoding: an all-zero `dh_pub` would be caught by
   the pre-existing check and would test the wrong guard.

---

## 2. GUARD PLACEMENT AND THE NO-MUTATION PROOF, PER LIVE SITE

The guard goes **immediately after** each `dh.dh(` call and **before** `dh_out` is consumed by
`kdf_rk_dh`, so no derived material is ever computed from a degenerate secret.

| # | Site | Function | Error channel | Proof of NO STATE MUTATION on reject |
|---|---|---|---|---|
| 1 | `ratchet.rs:1306` | `send_boundary` | `return Err("REJECT_S2_DH_NONCONTRIBUTORY")` | `st` is taken **by value** (`mut st`); the first write to it is the commit block at `:1366-1374`. The guard precedes it. The caller's state is a separate binding; on `Err` the moved copy is dropped. |
| 2 | `ratchet.rs:1475` | `recv_dh_boundary` | `reject!(st, "REJECT_S2_DH_NONCONTRIBUTORY")` | The `reject!` macro (`:1391`) returns `RecvDhBoundaryOutcome{ state: st, ok:false, … }` with `st` **unmodified**; the commit is `let mut new = st;` at `:1503`, strictly after. |
| 3 | `ratchet.rs:1885` | `send_combined_boundary` | `return Err("REJECT_S2_DH_NONCONTRIBUTORY")` | `mut st` by value; the commit block begins at `:1970` (`st.rk = rk_final;`). The guard precedes it. |
| 4 | `ratchet.rs:2390` | `recv_combined_boundary` | `reject!(st, "REJECT_S2_DH_NONCONTRIBUTORY", Some(header_pn), Some(n_val))` | The `reject!` macro (`:2303`) returns `RecvSessionOutcome{ state: st, ok:false, … }` unmodified; the commit is strictly after `apply_pq_reseed` / `kdf_rk_pq`. |
| 5 | `qsc/handshake/mod.rs:801` | `hs_dh_shared` (covers `:1449` + `:1877`) | `return Err(HS_DH_NONCONTRIBUTORY)` | The function is **pure**: it owns only local `sk`/`pk` buffers and touches no session state. Both callers already fail closed on `Err` — `:1449` emits `handshake_reject` and `return Ok(())`; `:1877` emits and `continue`s — **before** `hs_build_session` is ever reached. |

**Independent machine confirmation for site 4:** the conformance actor already asserts this invariant
for every reject shape of the combined-boundary receive — `tools/actors/refimpl_actor_rs/src/main.rs`
snapshots `st.snapshot_bytes()` before the call and raises `ActorError::Internal("combined boundary
reject mutated state")` if the post-reject state differs. Our new negative vector therefore proves
"rejects **and** does not mutate" mechanically, without trusting the reason string alone.

Each of sites 1–4 additionally gets a co-located `#[cfg(test)]` test asserting reject + byte-identical
state (`snapshot_bytes()` compare), and site 5 gets one in the existing qsc test module
(`qsc/src/handshake/mod.rs:2065`).

---

## 3. REASON CODES (Operator Decision 3)

**Canonical (Suite-2 ratchet), the single authorized addition:** `REJECT_S2_DH_NONCONTRIBUTORY`,
returned at sites 1–4. A distinct code (not a reuse of `REJECT_S2_HDR_AUTH_FAIL`) because a
non-contributory peer key is a different failure from a header-auth failure, it is the signal an
operator most wants in logs, and conflating them would make the negative vectors untestable.

**qsc establishment marker — NOT canonical, stated explicitly per Decision 3.** `hs_dh_shared` returns
`&'static str` markers consumed only by qsc's local `emit_marker("handshake_reject", …, reason=…)` log
convention (existing siblings: `dh_pub_invalid`, `dh_failed`, `dh_missing`, `pq_encap_failed`). The new
marker is **`dh_noncontributory`**, and the existing `handshake_dh_len` → `dh_failed` mapping is
preserved so no existing log assertion changes. Verified: no qsc test asserts on `dh_failed` or
`dh_pub_invalid` today, so adding a distinct marker breaks nothing. Introducing a *second* canonical
`REJECT_S2_ESTABLISH_*` code would exceed Decision 3's bounded unfreeze and is therefore NOT done.

---

## 4. THE BOUNDED DOC-CAN-003 EDIT (Operator Decision 3), QUOTED VERBATIM

Exactly two additions. **Any further canonical edit is a STOP.**

**(i) Reason-code registry (§8.6 list, appended after `REJECT_S2_COUNTER_OVERFLOW`):**

```
- `REJECT_S2_DH_NONCONTRIBUTORY` — the X25519 DH output is the all-zero value, i.e. the peer's
  `DH_pub` lies in the small subgroup and contributes no entropy to the root (RFC 7748 §6.1). The
  receiver MUST reject and MUST NOT commit any state.
```

**(ii) §8.5.2, one normative sentence appended to the DH-ratchet step list:**

```
After computing `dh_out = X25519(DHs_priv, msg.DH_pub)`, the implementation MUST reject the message
with `REJECT_S2_DH_NONCONTRIBUTORY` and MUST NOT commit any state if `dh_out` is the all-zero value
(RFC 7748 §6.1 contributory-behaviour check); the same check MUST be applied to the DH output of the
combined DH+PQ boundary (§8.5.3) and to the sending side before the root advances.
```

`DOC-CAN-004` is not touched. No KDF/AEAD/KEM primitive changes.

---

## 5. VECTOR PLAN (Operator Decision 4) — and a reachability finding D565 did not anticipate

### 5.1 Reachability matrix (derived live from the actor's op dispatch)

The conformance actor calls exactly these ratchet entry points: `recv_boundary_in_order`, `recv_wire`,
`recv_pq_reseed`, `send_combined_boundary`, `send_pq_advertise`, `send_pq_reseed`, plus the pure KDF
helpers. Therefore:

| Site | Actor-reachable? | How | Vector? |
|---|---|---|---|
| `:2390` `recv_combined_boundary` | **yes** | op `suite2.combined_boundary.run` → `recv_pq_reseed` → dispatches to `recv_combined_boundary` when `parsed.dh_pub != st.dh.dhr` (`ratchet.rs:2245`) | **YES** |
| `:1885` `send_combined_boundary` | **yes** | op `suite2.send_combined_boundary` | **YES** |
| `:1306` `send_boundary` | **no** | no actor op calls it | co-located Rust test |
| `:1475` `recv_dh_boundary` | **no** | no actor op calls it (`recv_wire`/`recv_boundary_in_order` perform no DH — the four `dh.dh(` sites are the complete list) | co-located Rust test |
| `hs_dh_shared` | n/a (qsc, not the actor) | — | co-located qsc test |

**RECORDED DEVIATION (not silently adapted).** D565 Operator Decision 4(i) says the lane must "add
negative vectors that drive a low-order `DH_pub` through `recv_dh_boundary` **and**
`recv_combined_boundary`". `recv_combined_boundary` is vector-reachable and gets one.
**`recv_dh_boundary` is not reachable from any actor op**, and making it so would require a new op in
`tools/actors/refimpl_actor_rs/**` — a **boundary-FORBIDDEN path**. Per D565's own rule ("do NOT 'just
fix' the caller"), the actor is left untouched; `:1475` is covered by a co-located Rust test that
asserts both the reject and byte-identical state. The operator decides at closeout whether a follow-up
lane adds the actor op. **No check is weakened: the guard at `:1475` is identical in form and proof to
the one at `:2390`, which IS vector-proved end-to-end.**

### 5.2 The vectors to add (ADDITIVE — new cases, no existing case altered)

1. **`S2-RECV-COMBINED-REJECT-DH-NONCONTRIBUTORY-0001`** — file
   `inputs/suite2/vectors/qshield_suite2_pq_reseed_vectors_v1.json`, op `suite2.combined_boundary.run`,
   tags `["CAT-S2-PQRESEED-001","CAT-S2-COMBINED-001","G2","G4"]`, `kind: negative`. Cloned from
   `S2-RECV-COMBINED-ACCEPT-0001` with **one field changed**: `message.new_dh_pub` →
   `0100000000000000000000000000000000000000000000000000000000000000` (u=1: low order, non-zero, so
   it survives the pre-existing `is_zero32(dh_pub)` ingress check). The actor seals the header itself,
   so the frame is honestly authenticated and the receive reaches `:2390`.
   `expect: {ok:false, reason_code:"REJECT_S2_DH_NONCONTRIBUTORY"}`.
2. **`S2-SEND-COMBINED-REJECT-DH-NONCONTRIBUTORY-0001`** — file
   `inputs/suite2/vectors/qshield_suite2_scka_logic_vectors_v1.json`, op `suite2.send_combined_boundary`,
   tags `["CAT-SCKA-LOGIC-001","CAT-S2-COMBINED-001","G2","G4"]`, `kind: negative`. Cloned from
   `S2-SEND-COMBINED-ACCEPT-0001` with **one field changed**: `dh_state.dhr` → the same `u=1` encoding,
   so the sender's `dh(new_dh_priv, dhr)` is all-zero and `:1885` rejects before the root advances.
   `expect: {ok:false, reason_code:"REJECT_S2_DH_NONCONTRIBUTORY"}`.
3. **qsc establishment negative case** (Decision 4(ii)): a co-located `#[cfg(test)]` test in
   `qsc/src/handshake/mod.rs` asserting `hs_dh_shared(sk, u1_low_order) == Err("dh_noncontributory")`
   for both an initiator-shaped and responder-shaped scalar, and `Ok(_)` for an honest peer key.

**Why these two files, and not a new one.** Both existing runners hardcode
`--file inputs/suite2/vectors/<name>.json` and select by tag (`run_suite2_pqreseed_vectors.py:46,51`;
`run_suite2_scka_logic_vectors.py`). A *new* vector file would require a new step in
`.github/workflows/suite2.yml` — a **FORBIDDEN** path — and the vectors would therefore never run in
CI. Appending tagged cases to the existing files means the **required** `suite2-vectors` check
executes them on every PR with zero workflow change.

### 5.3 The WF-0014 byte claim, and the reading of Decision 4 it rests on

Decision 4's STOP is: *"any EXISTING `inputs/suite2/**` vector byte changes — it means the guard
rejects an honest transcript."* The claim is therefore about **each existing vector's bytes**, not
about file-level byte equality (a purely additive append necessarily changes a file's bytes while
changing no existing vector, and "sha256 cross-set guards" only makes sense per-vector). The
byte-scan asserts, mechanically, against `git show HEAD:<file>`:

1. every pre-existing vector `id` still present (**none removed**);
2. for every pre-existing `id`, the canonical serialization (`json.dumps(..., sort_keys=True,
   separators=(',',':'))`) is **byte-identical**, compared by sha256;
3. every `inputs/suite2/**` file other than the two appended files is **byte-identical at file level**
   (sha256);
4. the set of new ids equals exactly the two ids above (an explicit allowlist — nothing else may
   appear);
5. a cross-set guard: `sha256(concat(sorted(per-vector hashes)))` over the pre-existing set matches
   the baseline computed from `HEAD`.

Any failure is a STOP: it would mean the guard rejects an honest transcript. The scan is a script, not
prose, and its output is archived to the proof root.

---

## 6. ANTI-REGRESSION SCAN (Operator Decision 2(c), as amended by D565-A1.5)

**What it does.** Walks every `*.rs` file in the repo (excluding `target/`), finds every `.dh(` call
site, and requires each to be either:

- **guarded** — an `is_zero32(&dh_out)`-shaped fail-closed check within the following few lines; or
- **allowlisted** — present in `ALLOWED_UNGUARDED_DH`, a **per-site** table keyed by
  `(file, enclosing function)` and carrying a **written reason**, with a **pinned per-file count**.

**Three mutations it must fail on** (a scan that cannot fail is not a scan — the NA-0627 gate-mutation
discipline applied to a lint). The design-lock will not be considered discharged until all three are
demonstrated:

1. a synthetic unguarded `.dh(` site added to unlisted code → **FAIL**;
2. an allowlist entry whose reason string is empty/missing → **FAIL**;
3. an extra `.dh(` site added to an **already-allowlisted** file (count drift) → **FAIL**.

Mutation 3 is the one that makes the allowlist safe: without the pinned count, the next unguarded call
site would hide inside `qsp/ratchet.rs` or, worse, inside `suite2/ratchet.rs` itself (which contains
both guarded production sites and allowlisted test sites).

`qsp`'s 8 sites are allowlisted with the reason: *"legacy Suite-1/1B conformance surface (exercised by
required checks ci-4b / ci-4d-dur); not shipped-client code; auth-safety tracked by ENG-0019 (P2)."*

**Placement, and an honest limitation.** The scan is a `#[cfg(test)]` test in the refimpl crate
(in-boundary). **It will not run in CI.** Verified live: no workflow runs `cargo test -p
quantumshield_refimpl` — the only `cargo test` invocations in `.github/workflows/**` are qsc's (`ci.yml:364-366,380`;
`macos-build.yml:88-90,123`) plus the miri/adversarial jobs. The refimpl crate's unit tests have **no
CI enforcement at all** today. Consequences, stated rather than papered over:

- the scan and the co-located no-mutation tests run in this lane's Phase-5 gate and in any local
  `cargo test -p quantumshield_refimpl`, but a future PR that adds an unguarded `.dh(` site **would not
  be stopped by CI**;
- the **CI-durable** proof of the guard is the pair of negative conformance vectors (§5.2), which the
  **required** `suite2-vectors` check runs on every PR;
- wiring `cargo test -p quantumshield_refimpl --locked` into an existing job is a **one-line
  `.github/**` change** — a FORBIDDEN path here. It is filed as a follow-up (§8) rather than smuggled
  in, and it is the cheapest durability win available to the successor lane.

Decision 2(c)'s stated purpose ("what makes (b) safe") is therefore **partially** met in-lane: the scan
exists, is mutation-proved, and runs in the lane gate; its CI enforcement is blocked by the Result
boundary and is filed. **No claim is made that a future call site "cannot" forget the guard.**

---

## 7. DoD 5 — can Q7 be honestly strengthened? **NO. Nothing changes in `formal/proverif/**`.**

Q7 is `not event(BoundaryAccepted(zeroG))`, proved `is true.` on both root-advancing receive arms.
`zeroG` is an **algebra-free constant** carrying no equation — deliberately, because extending
ProVerif's DH theory with a degenerate element makes the tool diverge (D-1249; probe
`dh_theory_finding.txt`), and because standard symbolic DH **cannot express** small-subgroup behaviour
at all (abstraction **A4**, `DOC-G4-002` §2).

With the output guard shipped, one is tempted to say Q7 now "models the shipped check". It does not.
Q7 mirrors the shipped `is_zero32(DH_pub)` **ingress encoding** check; the new guard is on the DH
**output**, whose degeneracy is exactly the algebraic fact A4 masks. A model that cannot represent a
low-order point cannot represent the condition the new guard tests. Strengthening the query text would
therefore assert something the model does not decide — the vacuous green the Decision-5 discharge
exists to prevent.

**Conclusion, per DoD 5's own instruction ("If nothing honest can be strengthened, say so and change
nothing"): no model, query, header, or gate expectation is modified.** The NA-0627 ProVerif gate must
stay green **unchanged**; if it goes red, that is a STOP. Q7 remains, in every artifact, explicitly
**not an attack-existence proof**.

---

## 8. Findings this design-lock records (filed, not fixed in-lane)

1. **CI does not run the refimpl crate's tests** (§6). The guards' unit tests and the anti-regression
   scan are lane-gate-only. Remedy is one line in an existing workflow job; `.github/**` is forbidden
   here. → filed as a follow-up item for the successor.
2. **`recv_dh_boundary` (`:1475`) and `send_boundary` (`:1306`) are not reachable from the conformance
   actor** (§5.1), so two of the four refimpl guards cannot be proved by conformance vector without a
   new actor op (`tools/actors/**`, forbidden). → recorded; operator decides.
3. **The four refimpl DH sites are shipped-`qsc` code paths** (§1.2), not merely reference-implementation
   paths. This corrects a framing in D565 and in the ENG-0034 filing. It raises, not lowers, the value
   of the lane.

## 9. Verification trail

```
$ grep -rn '\.dh(' --include='*.rs' . | grep -v /target/ | wc -l
21
$ grep -n 'dh\.dh(' tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs
1306 (send_boundary)  1475 (recv_dh_boundary)  1885 (send_combined_boundary)  2390 (recv_combined_boundary)
$ grep -n 'hs_dh_shared' qsl/qsl-client/qsc/src/handshake/mod.rs
801 (def, returns Result)  1449 (initiator)  1877 (responder)
$ grep -rn 'send_boundary(\|recv_dh_boundary(\|recv_pq_reseed(' qsl/qsl-client/qsc/src/main.rs
2320 send_boundary   2657 recv_pq_reseed   2683 recv_dh_boundary
$ grep -rn 'cargo test' .github/workflows/ | grep -c 'quantumshield_refimpl'
0
$ python3 <x25519 RFC 7748 probe, self-checked against §5.2 vector 1>
  all 8 low-order encodings -> dh_out == 0 for every clamped scalar
  7 of 8 survive the existing is_zero32(dh_pub) ingress check
  honest peer key -> non-zero output (negative control)
```
