# NA-0769 — AS BUILT: the security filing lane

Bases: qsl-protocol main `503dcdadeca2a15aaf3f177913cf26c564461c3e`, qsl-desktop main
`11f695dfcb3e6c1f3b3ff78a14eee71e878b0439`, each re-derived **bare and unpiped** at the NAMED
`github` remote with all 40 digits compared. **Filing only — zero product source bytes in either
repo, zero tests, zero `.github/**`, nothing repaired, nothing merged, no severity assigned.**
Decision `D-1410`. ⚠ Every figure below was measured when this file was written, not carried
from a draft, and not adopted from either cold read.

## 1. INPUT VERIFICATION — BEFORE ANY INPUT WAS READ

| file | sealed sha256 | measured | verdict |
|---|---|---|---|
| `BRIEF_security_filing_20260828.md` | `4669076e…a72b1` | identical | **MATCH**, 64/64 |
| `FINDINGS_SR15_NA0768_002_20260828T000117Z.md` | `aa40aa79…9f19efd` | identical | **MATCH**, 64/64 |
| `FINDINGS_SR15_NA0768_20260827T224959Z.md` | `28f1f12c…2ee82bd` | identical | **MATCH**, 64/64 |

Each quoted digest was first measured for **length** (all three = 64 characters) so a truncated
paste could not pass as a match.

**COMPARATOR NEGATIVE CONTROL, BOTH ARMS.** Positive arm: `sha256sum -c` over the three, run
unpiped — three `OK`, **rc 0**. Negative arm: one hex digit of the brief's digest flipped
(`4669076e` → `4669076f`), same command — the brief's line reads **FAILED**, `rc 1`, **and the
other two still read OK**, so the refusal is localised rather than blanket. ⚠ The gating check was
**not piped**: its exit status was read directly.

## 2. SR-14 BANKING — `/srv/qbuild/operator/NA-0769/`

All three copied and `chmod 444`. **Verified two independent ways**, because a digest of the
destination proves the file exists and never that the intended bytes are in it:
- `cmp` of each banked copy against its source: **rc 0, rc 0, rc 0** (silence = identical).
- `sha256sum -c` of the banked bytes against the Director-sealed digests: **three `OK`, rc 0**.

**IMMUTABILITY CONTROL, BOTH ARMS, AND THE ARMS DIFFER:**

| arm | file | append attempt | digest before → after | result |
|---|---|---|---|---|
| A | banked `BRIEF…md`, mode 444 | `Permission denied`, **rc 1** | `4669076e…a72b1` → `4669076e…a72b1` | **UNCHANGED — refused** |
| B | identical content copied to mode **644** | **rc 0** | `4669076e…a72b1` → `b336aacd…a38e5` | **CHANGED — tamper lands** |

⇒ **ARMS DIFFER**, so arm A's refusal is the mode bit and not a broken command. ⚠ This control is
run precisely because a 444 fixture *silently* refuses a naive tamper, which would otherwise make a
dead instrument look like a passing one. Ran as `victor`, not root.

## 3. IDS — DERIVED ON DECLARING FORMS, BOTH POLARITIES

`DECISIONS.md` carries **four** record forms and all four were searched, because a form-specific
needle would be right only by luck: `^## D-####` (104, max **D-1408**) · `^- **ID:** D-####`
(1300, max D-1312) · `^### D-####` (7, max D-0116) · `^**D-####` (2, max D-1340).
`NEXT_ACTIONS.md`: `^### NA-####` (816, max **NA-0767**). `IMPROVEMENT_LEDGER.md`:
`^### ENG-####` (251, max **ENG-0251**). Secondary forms were classified, not counted:
the 8 `^**NA-####` are status lines maxing at NA-0673 and the 5 `^**ENG-####` are prose mentions
maxing at ENG-0199 — **both below the tail, so neither competes.**

| id | declarations | repo files | operator files | disposition |
|---|---|---|---|---|
| NA-0767 / D-1408 / ENG-0251 | **1 / 1 / 1** | — | — | **positive controls, all hit** |
| `NA-0769` | 0 | 0 | 2 | **TAKEN** — both hits are NA-0768's own negative-control line |
| `D-1409` | 0 | 0 | 4 | ⚠⚠ **REFUSED — CLAIMED.** NA-0768 STOP 001: *"⇒ D-1409 FREE. TAKEN BY THIS LANE."* |
| `D-1410` | 0 | 0 | 2 | **TAKEN** — both hits are NA-0768's negative controls; never claimed |
| `ENG-0252` | 0 | 0 | 2 | **TAKEN** — NA-0768 states it *"NOT TAKEN"* in terms |
| `ENG-0253` | 0 | 0 | 2 | **TAKEN** — negative-control mentions only |
| `ENG-0254` | 0 | **0** | **0** | **TAKEN** — clean in every space |
| `NA-0793` / `D-1493` / `ENG-0293` | 0 | 0 | — | fresh negative sentinels, unspent |

⚠⚠ **THE COLLISION AND WHY A REPO-ONLY INSTRUMENT MISSES IT.** `D-1409` has **zero declarations**
because NA-0768 is paused with nothing landed — its claim exists only in the operator tree. A
counter derived from declarations alone would have issued a taken id. ⇒ **derive against main,
every open PR, AND every paused lane's claim.** Open PRs: **zero** on both repositories, with the
query's ability to return rows proven separately (`--state merged` returns rows on both).

⚠ **`NA-0768` HAS ZERO OCCURRENCES IN THE ENTIRE PROTOCOL TREE** — it is allocated in the operator
tree only. The brief's *"NA max 0768"* is right about **allocation**; the repo's declared max is
**NA-0767**. Both readings give the same next free id.

## 4. THE SEC-2 RE-MEASUREMENT — FOUR DISAGREEMENTS, REPORTED NOT RECONCILED

The substance of every sec-2 premise reproduces. What did not:

| # | the reads / the brief say | measured | class |
|---|---|---|---|
| 1 | `hs_contexts_match`'s `_ => false` arm is at `:1281-1290` | the arm is at **`:1279`**; the function spans `:1272-1281`. `:1281-1290` holds `hs_reject_context_mismatch` and `hs_reject_key_context` — two **different** functions | cite wrong, substance right |
| 2 | the desktop's `max: 1` is at `ui/main.js:2888` | the invoke is at **`:2891`**, `max: 1` at **`:2892`**; `:2888` is `marks.pending += 1` | cite wrong, value right |
| 3 | today's destruction-of-one under Legacy is bounded by *"the head-only `.next()`"* | ⚠⚠ **wrong mechanism.** `.next()` bounds **consumption**; **`max` bounds destruction** — the URL carries `max={max}` under both arms (`transport/mod.rs:3541`) and every returned frame is deleted before `.next()` runs. They coincide only because the desktop passes `max: 1` | **mechanism wrong** |
| 4 | four reasons can clear an addressed record | **six** (reason, gate) classes — see `D-1410` DV-2 | **count wrong** |

**Reproduced exactly:** both clears ungated (`:1778`/`:2066`, the `is_explicit()` test above each
being a separate `if` with its own clear at `:1773`/`:2061`) · the clear precedes all crypto
(`c.decap` at `:1784`, six lines below; `hs_confirm_mac`/`hs_ct_eq_32` at `:2087`/`:2093`, twenty-one
below) · the four-statement distance (`:1762`, `:1768`, `:1777`, `:1778`, counting top-level
statements of the `Ok(..)` arm inclusive of both ends — **the counting method is stated because the
figure is uncheckable without it**) · the correlator first at a fixed offset in all three frame
types (`:541`, `:626`, `:685`) · `decode_envelope_resp` a pure TLV parse with **no** crypto
(`invite/mod.rs:769-803`) · `1.clamp(16,128) = 16` · the 14 clear sites, all inside the poll, and
`hs_pending_clear` occurring **nowhere else in the crate**.

## 5. THE ENUMERATION — WHY NINE REASONS CANNOT REACH A CLEAR

Fourteen `REJECT_QSC_HS_*` literals exist in `handshake/mod.rs`. Nine originate **only** in
`hs_parse_parameter_block` (`:285-368`), which is called **only** at `:490` under
`if admit_context`. Both poll decoders pass `admit_context = **false**` — `hs_decode_resp_pending`
`:650`, `hs_decode_confirm_pending` `:705` — so the block parser is never reached on the poll path
and instead the decoder synthesises `HsSuiteContext::ExplicitV2 { block, protocol_version: 0,
suite_id: 0 }` (`:492-496`) for **any** block of length `<= 64`. The one caller that passes `true`
(`hs_decode_confirm`, `:698`) is reached only at `:2268`, in the fall-through arm — where there is
**no pending record to clear** (verified: no `hs_pending_clear` above line 2242). ⇒ the remaining
**six** classes are the whole set; the table with each gate is `D-1410` DV-2.

## 6. THE THIRD DEFECT — HOW THE BRIEF'S OWN QUESTION FOUND IT

Sec 2(c) ordered a measurement of whether any OTHER shipped path scans under Legacy. Every caller
of `relay_inbox_pull` was enumerated and classified: `invite/mod.rs:1226` (inside the Lease-only
scan) · `:1328` (`invite_accept` — head-only `.next()`, and its own comment says *"ACCEPT GETS AN
ACK, NOT A SCAN"*) · `:1426` (the gated Legacy arm) · `transport/mod.rs:516` (the receive loop,
`ENG-0142`'s site) · **`handshake/mod.rs:1722`** — which pulls `max` and iterates it at `:1759`,
`:2047`, `:2267`. That last one is a scan, its `--max` defaults to **4** unclamped
(`cmd/mod.rs:387`), and it branches **only its ack** on ack mode (`:1718-1719`) while its own
comment states the delete-on-pull fact. Filed **`ENG-0254`**.

## 7. STATE SWEEP — THE DUPLICATE CHECK, RUN BEFORE FILING (`WF-0029`)

| needle | DECISIONS.md | NEXT_ACTIONS.md | IMPROVEMENT_LEDGER.md |
|---|---|---|---|
| `hs_contexts_match` | **0** | **0** | **0** |
| `CONTEXT_MISMATCH` | **0** | **0** | **0** |
| `ungated` | **0** | **0** | **0** |
| `ack=lease` | 5 | 8 | 5 |

The ledger's only `hs_pending_clear` mention is inside **`ENG-0176`**, on the unrelated *"writes
`""` rather than deleting"* property. ⇒ **neither hazard is described anywhere in repo truth.**
`ENG-0198`'s two appended causes are *rejection* and *absence*; `ENG-0142`'s remainder is the
message/unknown-class wedge in `receive`. **Neither describes either hazard**, so all three
entries are NEW — with amendments landed **beside** the four entries they touch, none closed and
none edited above. The `ack=lease` row is non-zero and is why `ENG-0142`'s wording was checked at
all, which is how its *"explicit-flag lab mode"* characterisation was measured incomplete.

## 8. WHAT THIS LANE DID NOT MEASURE, STATED PLAINLY

- **No test was run and no build performed.** No `cargo` was invoked.
- **No relay was contacted; n=0 network calls for traffic.** *Delete-on-pull* is read from the
  client's own URL construction (`transport/mod.rs:3540-3543`) and the tree's own comments — it was
  **never observed**. A relay that does not delete on `?max=N` without `ack=lease` would falsify the
  consequence of `ENG-0253`/`ENG-0254` while leaving the branch asymmetry intact.
- **The deployed `ack_mode` and the relay's `retention_ttl_secs` were NOT measured.** Both are
  load-bearing for severity and both are recorded as owed. This is why no severity is assigned.
- **No frame was constructed and none was sent.** `ENG-0252`'s reachability is a statement about
  what the shipped decoders **permit**, derived from reading them.
- **The GUI was not driven.** Desktop claims are source claims.
- **`HandshakePending` zeroize-on-drop was not checked**, and neither was the stale-mirror question;
  both are carried from the reads as open, not re-derived here.
- **Both repositories were READ and neither was edited.** No product source byte, in either repo.
