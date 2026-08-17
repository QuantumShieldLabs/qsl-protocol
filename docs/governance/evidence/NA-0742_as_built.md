# NA-0742 — AS BUILT (PART 1 — THE PROMOTION)

**Lane:** NA-0742 / **D-1378** — LANE 2 OF THE ENG-0142/ENG-0196 REPAIR PROGRAM:
**INVITE-FINISH SCAN + PRODUCER ACKS.**
**This PR PROMOTES a directive. It implements nothing: records only, ZERO product source bytes.**
**Base:** main `215972779514c636cceb4c6752ab7b5401002532`, verified UNMOVED bare and unpiped against
the NAMED GitHub remote at the moment of assertion. Open-PR set **MEASURED EMPTY** with a positive
control returning the most recent merged PR.

⚠ **PART 2 IS RESERVED FOR THE IMPLEMENTATION. DO NOT OVERWRITE THIS PART — APPEND.** This part
carries the input provenance table below, which is the only copy of those shas in repo truth.

---

## 1. INPUT PROVENANCE — every document this lane consumed, by sha256

| artifact | sha256 | lines | how it was proven |
|---|---|---|---|
| Director's formalization brief (banked verbatim, SR-14) | `9d49ee326826fbc06674698ce9d719ca31cb93045248a10de3d2475a06bb4da2` | 137 | ⚠ arrived as chat text — **no byte source exists to diff a banking against**; only the Director can confirm byte-identity |
| Operator-blessed DESIGN BLOCK, PARTS 3 and 4 | `316b09acedee3221a7c429898ef84d4833b3ad7edaf40bd06898d7ec5b306e6b` | 56 | ⛳ **exactly the sha the brief cites**; `cmp` against NA-0741's sealed source rc 0, with a last-character-mutation negative control returning rc 1 and `assert altered != original` proving the control non-vacuous |
| STOP 001 — formalization (carries the draft directive + cargo v1 inline) | `b8744a12d692d4ac6638f640b8e794d21798f239e3663203e70b7cc28f2a2988` | 1003 | assembled by substitution from source bytes, each block extracted back out and diffed, per-block negative control |
| STOP 002 — the protected-region anchor correction | `7441a6c1d54bc2b37da748a5c46f444556f621ab471187a42d0747088ecbb27e` | 132 | — |
| **R345** — final ruling, SR-15 read absorbed (banked verbatim, SR-14) | `9e8283d1b45cc2ae51d3cb8d8bd73c5af7f5697500c3026db3aa0d294d46f9b5` | 131 | ⚠ chat text, same limitation; the Director independently confirmed the embedded copy byte-faithful |
| **FINAL DIRECTIVE** — the executable document | `e947014c4639bb41e0218083b01f310019e0d8771ed1e888d364c20cb539d119` | 535 | carries R345 verbatim in its Appendix A, extract-and-diff proven; **a completeness gate over all 28 finding ids the ruling names refused to write the file until every one appeared in the fold-in ledger** |
| **CARGO v2** — the landable text this PR executes | `62ddf8325aadc4646e54a1782b507b81d5bd1a0106d7f5741f0620bca9753058` | 173 | — |
| STOP 003 — the final assembly | `bb589631c54a493cbeac5a29551f3c49923e8e7a67ca5e29750096f618086e09` | 895 | carries the directive and cargo v2 inline, both extract-and-diff proven |
| **R346** — Director verification, proceed to promotion (banked verbatim, SR-14) | `e0733ce4f0301aace6d07da9346c943bdfffc61fe096bb8b99ab495edad7291c` | 30 | ⚠ chat text, same limitation |

**Superseded and deliberately UNEDITED:** the draft directive `19af38e1…` (479 l) and CARGO v1
`6b3be8fd…` (184 l). Where they differ from the final pair, the final pair governs, and every
difference is enumerated in the directive's §0 fold-in ledger.

---

## 2. ⚠ THE FIVE-vs-TWELVE RECONCILIATION — ORDERED RECORDED AT R346 §1

R345 §3(b) ordered *"the five non-consuming pending-mutation sites"* named as non-acking. **The
formalization seat could not reproduce "five"** and derived the set itself rather than inherit a
figure it could not check, reporting the difference instead of composing a correction.

| instrument | scope | result |
|---|---|---|
| the SR-15 read's 3B | **branch-traced, ILLUSTRATIVE subset** | `:1963` `:2000` `:2030` `:2143` `:2152` — **five** |
| the seat's sweep | **exhaustive file sweep** of `hs_pending_clear` (14 sites) minus the 2 that follow a `qsp_session_store` (`:1888`, `:2103`) | **twelve** |

**FIVE ⊂ TWELVE**; the extras are `:1712` `:1731` `:1736` `:1809` `:1976` `:1995` `:2130`.
**THE SUPERSET RESOLUTION IS CONFIRMED AND GOVERNS** (R346 §1): *every* non-consuming
pending-mutation site is a non-acking site.

⇒ **THE TRANSFERABLE POINT, and the Director recorded it against their own wording: "the five"
quoted a figure without its instrument's scope** — WF-0087's own diction, in the chair that wrote
the rule. **No SR-16 row is manufactured for it**; this clause is the record.

---

## 3. WHAT THE SR-15 CEREMONY BOUGHT, IN ONE PLACE

1. ⚠⚠ **The drafted scan was MODE-BLIND**, and under `AckMode::Legacy` — where the relay deletes
   what it returns — it would have amplified a one-frame loss **16×–128×**. Invisible to the chair
   that wrote both the design block and the brief. ⇒ **ruled LEASE-ONLY END TO END.**
2. ⚠⚠ **The ack placement was wrong in two of the poll's three branches.** *"After the durable
   commit"* is not *"after the frame's last effect"*: an outbound **push** follows the commit on the
   initiator and no-pending paths, and `handshake/mod.rs:1926` is a `return Ok(())` **between** them.
3. ⚠⚠ **ENG-0196 has a second false-diagnosis spelling** (`handshake_envelope_version_newer`),
   reachable without an adversary, with **no test and no consumer** — now expectation **E1b**.
4. **The crash window was settled by the source** (`handshake/mod.rs:207-213`), not by argument.
5. **The census of record moved from 52 to 32 files / 118 `#[test]`** — the earlier instrument was
   blind to 17 files.
6. **Protected regions are pinned by CONTENT**, after lane 1's line-number pin was measured to name
   its region on neither tree while its sha reproduced exactly.

---

## 4. INSTRUMENTS AND EVIDENCE — sealed 444 at `/srv/qbuild/operator/NA-0742/`

`instruments_NA0742/` — `id_sweep.py` · `consumer_census.py` · `census_na0740_extracted.py`
(77 l, sha256 `8d77a2be…`, **recovered from NA-0740 STOP 002's fenced block and hash-verified against
the sha that stop itself recorded** — the embed-and-verify cure paying in the hands of the successor
it was written for) · `assemble_stop.py` · `zz_lane2_ack_semantics_measurement.rs`
⚠ **with their PAYLOADS sealed beside them** — `PAYLOAD_tests_common_mod.rs` and
`PAYLOAD_stop_shell.md`. *An instrument's payloads seal with it: the test is not "is the script
there", it is "does it RUN there."* All three python instruments were **executed from their sealed
locations** to prove exactly that.

`evidence_NA0742/` — `ack_wire_semantics_MEASURED.log` and ⚠ `ack_FIRST_PASS_vacuous_instrument.log`.
**The vacuous run is sealed too**: it modelled the message id as a JSON body field where the server
reads the `X-Msg-Id` header, so six arms acked ids that had never existed and returned `acked=0` in a
log that reads clean. **Only a sealed expectation caught it, and the control being vacuous is the
finding — not the result it printed.**

---

## 5. CLAIM BOUNDARY

Loopback plain HTTP against an in-process `qsl-server` at the pinned rev
`131d63f4865544addd2784c305970b21ddbeb69c`. `MAX_BODY_BYTES=65536`, queue depth 64, default lease.
**n = 1 per arm.** **NOT a CI claim.** Not through TLS. Ack semantics are **executed**; the
crash-window client half is **read from source** and is what T5 will settle at implementation.
Server parity with the production relay is **inherited from the SR-15 read**, not re-derived here.
**Every line anchor was read at `21597277`; anchors move, and the impl seat re-derives.**

---

## 6. WHAT THIS PR DOES NOT DO

ENG-0196 is **not repaired**. ENG-0142 does **not close** — its remainder is the original adversarial
clause, capability-gated and bounded, and the operator's `P2` re-grade lands **beside** the `P1`
bullet without rewriting it or the heading. ENG-0191's (a)–(e) stay the operator's; ENG-0194 is not
repaired; ENG-0193 and WF-0086's and WF-0087's gates are not built; **#1745 — an ISSUE, not a PR —
stays OPEN**. No test was weakened, skipped or deleted. No fenced ruling and no sealed artifact was
edited. **Nothing is merged: the operator merges, the seat does not.**
