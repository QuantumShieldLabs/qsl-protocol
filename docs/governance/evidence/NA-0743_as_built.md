# NA-0743 — AS BUILT (D-1380) — THE ENG-0191 REPAIR: FORMALIZATION AND PROMOTION

**Lane:** NA-0743 · **Decision:** D-1380 · **Ruling:** **R348** · **Base:** main
`ae2047e6b3555cf367bb14f2924f41f17460b989`, verified UNMOVED **bare and unpiped by URL** against the
named GitHub remote **at the moment of assertion**, with the open-PR set **MEASURED EMPTY** and a
positive control returning merged rows.

⚠⚠ **THIS PR IS RECORDS ONLY. ZERO PRODUCT SOURCE BYTES. ENG-0191 IS NOT REPAIRED HERE.** It
promotes the directive a **fresh seat** will execute.

---

## 1. WHAT WAS MEASURED, AND WITH WHAT

Every tree-wide census used **`git grep` over tracked files**, never the shell `grep` wrapper (which
execs `ugrep --ignore-files` and therefore honours `.gitignore`, blinding it to 514 tracked files).
**Where a glob could have been too narrow, no glob was used at all** — a glob cannot be too narrow if
there is none.

### 1.1 The script

`scripts/demo/qsc_remote_handshake_smoke.sh` — sha256
`c885dcf09033cce082290a6856d6835b6dddbebfc90eb4de816bf7fcd9670eef`, **482 lines** by `wc -l`
(**483** by split-on-newline — the two instruments are reconciled here rather than left to
disagree), 18319 bytes.

⛳ **Independently corroborated:** `docs/ops/IMPROVEMENT_LEDGER.md:4600` records NA-0738 re-verifying
the committed script at **`c885dcf0…0eef`, 482 lines** after all four of its runs. Same sha, same
count, two lanes apart ⇒ **the script is byte-unchanged since NA-0738.**

**The eight sites, re-derived BY CONTENT:**

| site | content | role |
|---|---|---|
| `:351` | `assert_marker_present 'event=handshake_status status=established' "$alice_log" …` | assertion (alice) |
| `:352` | `assert_marker_present 'event=handshake_status status=established' "$bob_log" …` | assertion (bob) |
| `:353` | `# Derived lane marker: ACTIVE is asserted from established handshake status above.` | fabrication comment |
| `:354` / `:355` | `echo "QSC_MARK/1 event=qsp_status status=ACTIVE reason=handshake actor=…" >> "$markers"` | **the fabrication** |
| `:448` / `:449` | `echo "handshake_active_{alice,bob}=true"` | literal → `normalized_counts.txt` |
| `:468` | `echo "handshake=ACTIVE(reason=handshake) both_peers"` | literal → `summary.txt` |

**The matching semantics, read from the script's own definitions:** `assert_marker_present`
(`:208-216`) calls `mark_grep` (`:109-115`) = `rg "$@"` or `grep -E "$@"` ⇒ **an unanchored REGEX**
⇒ `status=established` **matches inside** `status=established_recv_only`. That is the 187-day hazard,
in the file, at those two lines.

### 1.2 ⚠ THE BRIEF'S STALENESS PREMISE MEASURED FALSE (SR-16 row 71)

The brief declared all six cited line numbers STALE and ordered re-derivation by content. **Re-derived
by content, all six are CORRECT** — and the reason is structural, not lucky:

    $ git log --oneline 5201c275..HEAD -- scripts/demo/qsc_remote_handshake_smoke.sh
    fbf502c1 NA-0737: ENG-0192 — the fixture addressing repair …
    $ git diff --stat 5201c275 HEAD -- scripts/demo/qsc_remote_handshake_smoke.sh
     1 file changed, 2 insertions(+), 2 deletions(-)
    $ git diff -U0 5201c275 HEAD -- scripts/demo/qsc_remote_handshake_smoke.sh | grep '^@@'
    @@ -375 +375 @@ done
    @@ -388 +388 @@ done

⇒ **an in-place substitution changes bytes without moving anchors.** "The file was touched" never
implies "the anchors moved". The Director re-verified the eight sites directly and accepted the
proof **[E]**.

### 1.3 The `status=established` consumer census — 7 of 7 classified, zero unclassified

**42** occurrence lines tree-wide (35 `.md`, 4 `.rs`, 2 `.sh`, 1 `.py`); **7 executable**.

| # | site | semantics | class |
|---|---|---|---|
| 1 | `scripts/demo/qsc_remote_handshake_smoke.sh:351` | `grep -E`, unanchored | **REPAIRED by the impl lane** |
| 2 | `scripts/demo/qsc_remote_handshake_smoke.sh:352` | `grep -E`, unanchored | **REPAIRED** — *passes spuriously today* |
| 3 | `qsl/qsl-client/qsc/tests/send_ready_markers_na0168.rs:334` | Rust `.contains` | **REPAIRED** = ENG-0194 |
| 4 | `qsl/qsl-client/qsc/scripts/remote_soak.py:573` | Python `not in` | **UNTOUCHED by ruling (R348 §2)** — named in ENG-0194's amendment |
| 5 | `qsl/qsl-client/qsc/tests/handshake_mvp.rs:1165` | `.contains("…established_recv_only peer=alice")` | **KEEP** |
| 6 | `qsl/qsl-client/qsc/tests/handshake_contract_na0217i.rs:283` | `.contains("…established_recv_only peer=alice")` | **KEEP** |
| 7 | `qsl/qsl-client/qsc/tests/desktop_gui_contract_na0215b.rs:539` | `.contains("status=established_recv_only")` | **KEEP** |

**KEEP reason, stated rather than assumed:** #5–#7 consume a **different literal**. Their needle
carries the discriminating `_recv_only` suffix, and `established_recv_only` is **not a prefix of any
other status** — `hs_status_truth`'s range is exactly {`established_recv_only`,
`awaiting_peer_confirm`, `established`}.

⛳ **THIS REPRODUCES THE TREE'S OWN RATIFIED CENSUS EXACTLY.** ENG-0194's filed table
(`IMPROVEMENT_LEDGER.md:4820-4825`) names **exactly these four** bare-token consumers, and `:4831`
names **exactly these three** as *not part of the defect*. **Two independent instruments, one
answer** (SR-16 row 72).

⚠ **A BLINDNESS CONTROL FIRED.** The same needle through the recursive `.gitignore`-honouring route
returns **41**, blind to `docs/governance/evidence/NA-0737_as_built.md`. The census that would have
been wrong is the one that used the convenient tool.

### 1.4 ⚠⚠ The fabrication is a FORGERY of a real product marker

`status=ACTIVE` occurs **4 times** tree-wide: the two fabrication lines, `qsp_status_truthy.rs:170`
(which asserts the **product** emits `event=qsp_status status=ACTIVE reason=handshake` — it does,
from `main.rs:96-98` via `qsp_status_tuple`), and ENG-0191's own prose at `IMPROVEMENT_LEDGER.md:4587`.

⇒ **the script writes a marker the product really has, by hand, attributed to actors alice and bob,
with no `qsc` process having emitted it** — and hashes it into the published
`normalized_subset_sha256`. Nothing reading the marker set can tell the forged line from a genuine
one. **The sealed E5 therefore classifies forged vs genuinely-emitted rather than counting.**

**Downstream consumers, measured:** `normalized_subset_sha256` (`:478`) and `marker_lines` (`:473`)
each occur at **exactly one site tree-wide — their own producer. Zero consumers.**
**`docs/qsc/DOC-QSC-006_…DRAFT.md:76` is the one real consumer** — it lists the forged marker under
**Required checks** — which is why R348 §1 widened the edit set to repair it rather than leave a
contract naming a retired forgery.

### 1.5 X5, sealed from NA-0738's SEALED evidence

**All five artifact shas verified against NA-0738's own `LATEST.md` BEFORE reading; nothing touched
beyond verification:** STOP 001 `87c35f3c…`/488 · 002 `d2033fa5…`/229 · 003 `d676c6b4…`/122 · BRIEF
`8ab993b9…`/149 · R337 `29ca1c31…`/62 — all five agree with the pointer.

**From STOP 001 `:149-150`:** alice **`established_recv_only`** (send_ready no, `chainkey_unset`,
peer_confirmed yes) · bob **`awaiting_peer_confirm`** (send_ready yes, peer_confirmed no).

⛳ **Three runs agree, including one whose ANTECEDENT FAILED** — run X1 (`:182`), where alice's
receive returned rc 1 and X4 missed, still measured the same X5 pair; run 2 (`:226`) *"X5 role-dual
pair reproduces — HIT"*; run 3 (`:229`) *"reproduces run 2 byte-identically, `diff` rc 0 across all
12 observations"*. ⇒ **X5 is invariant across both the delivery outcome and the lease knob**, and
there is a mechanism: `hs2` **REPLACES** the session, so X5 is a function of **hs2's roles alone**.
**E3 is sealed, not downgraded.**

### 1.6 ⚠⚠ NA-0738's (d) BLOCKER IS CLEARED — a premise nobody had re-checked

`IMPROVEMENT_LEDGER.md:4589-4598` records **"A BLOCKER (d) MUST CLEAR"**: a stale handshake frame
redelivered at the head after the 60 s visibility timeout, with `transport/mod.rs:1249
return Err(...)` **aborting the entire receive**; NA-0738 reached X4/X5 only at
`PULL_LEASE_SECS=3600`. Measured from source at this base:

| fact | site |
|---|---|
| `resolve_ack_mode` = `explicit` else `stored_ack_mode().unwrap_or(AckMode::Lease)` ⇒ **default Lease** | `qsl/qsl-client/qsc/src/lib.rs:938-943` |
| the script passes **no** `--ack-mode` (needle → **0 hits**) ⇒ takes the default | `scripts/demo/qsc_remote_handshake_smoke.sh` |
| `if ctx.ack_mode == AckMode::Lease && frame_class.is_known_foreign()` → `recv_frame_skipped … disposition=left_leased` → **`continue`** | `transport/mod.rs:555` … `:575` |
| `is_known_foreign()` = `Handshake \| InviteInit \| InviteResp` | `frameclass.rs:51-57` |

⇒ **by construction a stale handshake frame at the head is now SKIPPED and the receive proceeds.**
**NA-0741 (#1758) and NA-0742 (#1760) discharged a predecessor's blocker without setting out to, and
nothing said so.**

⚠ **A SOURCE CLAIM, NOT A RUN.** Whether it makes the reordered script green at the **default** knob
is empirical and is sealed as **E2(b)**; a MISS there is a **RESULT** that reopens the blocker and is
a STOP carrying the measurement — **never cured by raising the knob** (R332.1).

### 1.7 The script's consumers, and why `drop-reorder` gates the outcome

`.github/workflows/remote-handshake-tests.yml` is the **only** executable consumer: `:81-84`
(happy-path seed=1), `:92-95` (**drop-reorder seed=7**), artifact upload `:97-102`. It consumes the
**exit code only**; no step parses `summary.txt`, `normalized_counts.txt`, `normalized_subset.txt`,
`normalized_subset_sha256` or `marker_lines`.

⚠⚠ **The sole `if:` in the whole file is `if: always()` at `:99` on the artifact upload, and there is
no `continue-on-error` anywhere** ⇒ **both scenario steps gate the job** ⇒ `remote-handshake-tests`
cannot go green, and **#1745 cannot close**, unless `drop-reorder` also exits 0 under the reordered
script. That is a gate on the **OUTCOME**, not merely a boundary on the claim — which is why R348 §3
authorized a loopback `drop-reorder` arm as **E7**.

---

## 2. THE ID DERIVATION

All three WF-0068 input sets, **DECLARING forms**, declaration-vs-mention classified **before** any
maximum, fence-aware over **2359** tracked files from `git ls-files -z`:

| space | max at main | open PRs | operator dirs | taken |
|---|---|---|---|---|
| NA | 0742 | *(set EMPTY)* | NA-0742 | **NA-0743** |
| D (all four record forms: `## D-`→1379, `### D-`→0116, `**D-`→1340, `- **ID:** D-`→1312) | **1379** | — | — | **D-1380** |
| ENG / WF / SR | 0197 / 0087 / 22 | — | — | *(none minted)* |
| SR-16 rows | max **70**, contiguous | — | — | **71, 72** |

**Controls, both polarities:** NEGATIVE — `NA-0799`/`D-1999`/`ENG-0299`/`WF-0199` each **0**;
POSITIVE — `NA-0742`, `D-1379`, `ENG-0197` each **1 declaring site**. **WF-0087 plant check:**
`NA-0743` and `D-1380` measure **0 mentions AND 0 declarations**.

### 2.1 ⚠⚠ THE R-ID DERIVATION NEEDED FOUR ROUTES AND THE FIRST TWO ARE BLIND

| route | max | what it cannot see |
|---|---|---|
| A — banked-ruling **filenames** | **R337** | the filename form **changed at NA-0738** and stopped carrying the id |
| B — banked-ruling **content** | **R339** | recent rulings **do not name their own id at all** — NA-0742's two banked rulings contain **zero** R-ids, both opening with the same *"R-id derived at the edit"* sentence |
| C — **repo records citing rulings** | **R347** | ⬅ **the only route that governs** |
| D — operator tree at large | R348 | its 2 occurrences are **MENTIONS planted by NA-0742's own sweep log** — one literally reads `R348 … => FREE and UNPLANTED` — with **0 declarations** |

⇒ **R348 is FREE and is taken.** This is **WF-0087 in the R-id space**, and it makes NA-0740's
recorded procedural tension **wider than recorded**: the id is in **neither filename nor content**.

⚠ **The synthetic negative control `R999` is SPENT** — planted in **9** operator files by a
predecessor's own published control (the tree says so: *"R888, R952, R999, R613, R724 and R809 are
all SPENT by this program's cure"*). A **fresh unplanted** control was verified 0 across both routes
**before** use. ⚠ Publishing a control's token spends it; that is the recorded property, and this
paragraph spends one more.

---

## 3. WHAT THIS PR LANDS — the eight cargo rows

| # | row | landing |
|---|---|---|
| 1 | STATE move + the `### NA-0743` block **born READY at promotion** | `NEXT_ACTIONS.md` |
| 2 | NA-0742 → **DONE**, class `INVITE_FLOW_RESIDUE_ZERO_PASS` (R347 §4), `mergedAt` + true-merge parents | `NEXT_ACTIONS.md` |
| 3 | **ENG-0196 CLOSED**, beside its entry, with the **Legacy residual** named as a ruled lab-mode bound | `docs/ops/IMPROVEMENT_LEDGER.md` |
| 4 | lane 1's **E3(b) completed** | `NEXT_ACTIONS.md` |
| 5 | **D-1379's artifact list completed** — SR-16 citation corrected to rows **61–70**, omitted `IMPROVEMENT_LEDGER.md` added | `TRACEABILITY.md` |
| 6 | **ENG-0191 amended** beside its five options; (a)/(b)/(c) refused one line each, (e) **absorbed** | `docs/ops/IMPROVEMENT_LEDGER.md` |
| 7 | **SR-16 rows 71 and 72** | `docs/ops/PREDICTION_LEDGER.md` |
| 8 | **ENG-0194 amended and STAYING OPEN**, re-scoped to its one remaining consumer | `docs/ops/IMPROVEMENT_LEDGER.md` |

### 3.1 ⚠ Row 4's premise was wrong, and only drafting the record found it

The cargo said lane 1's E3(b) note was *"not recorded against lane 1"*. **Measured: a clause already
existed** on NA-0741's `Status:` line — but written **forward-looking** (*"is discharged by NA-0742's
T3 arm"*) **before lane 2 ran**, carrying **no measured figures**. The landing therefore **COMPLETES
that clause beside itself** rather than duplicating it, and says so in the text. *A record you have
only described can hide a dependency.*

### 3.2 ⚠ An observation surfaced, deliberately NOT acted on

**NA-0742's promotion did not record a `<!-- prior: STATE: … -->` line** for the NA-0741 → NA-0742
transition — measured (`NA-0742 PROMOTED` = **0** occurrences) with a **positive control** confirming
the instrument finds such lines (`NA-0741 PROMOTED` = 1). The prior-STATE chain has a gap at that
transition. **Not backfilled: it is another lane's record and outside this cargo.**

---

## 4. THE NO-DRIFT PROOFS

Both the STOP and the FINAL directive were built by an **assembler that substitutes each embedded
document from its OWN BYTES** — never retyped — after which each block was **extracted back out of
the finished file** and diffed against its source, each with a **negative control that mutates the
LAST CHARACTER** (which always differs, the cure for the vacuous-control family this program has hit
three times).

| artifact | embedded | extract == source | negative control |
|---|---|---|---|
| `STOP_NA0743_001_FORMALIZATION.md` | the DRAFT directive (39225 chars) + the brief (10003 chars) | **identical, both** | **differs, both** |
| `DIRECTIVE_NA0743_FINAL_R348.md` | **R348 verbatim** (4714 chars), Appendix A | **identical** | **differs** |

**Plus a COMPLETENESS GATE** over the ask-ids: each of **R1–R5** must appear in the FINAL directive's
FOLD-IN LEDGER with a named landing site — **all five FOLDED** — with a **negative control on the
gate itself** (a non-existent ask-id must read MISSING; it does).

**The assembler refuses to ship an unsubstituted placeholder** (asserted).

---

## 5. INSTRUMENT DEFECTS OF MY OWN — recorded, not tidied

1. **`git ls-files` C-QUOTES non-ASCII paths.** The id sweep died with `FileNotFoundError` on
   `docs/audit/incoming/…/Audit #5 — Nonce Uniqueness Regression Fuzzing.md`. Cured with
   `git ls-files -z` + a `surrogateescape` decode. ⚠ **It failed loudly, which is the only reason it
   is a footnote** — the same defect in a *counting* instrument returns a silently short census.
2. **AN OVER-STRICT ASSERT.** The stop assembler banned the bare marker `@@` and fired on the
   directive's own quoted `git diff` hunk headers (`@@ -375 +375 @@`). **The sound contract is to ban
   the ACTUAL placeholder tokens**, enumerated — not the marker. The assert was right to fire; it was
   stated at the wrong width.
3. **A NEEDLE BUILT FROM MY MODEL, NOT THE BYTES.** Row 4's landing asserted NA-0741's `Status:` line
   ended with *"the seat does not."*; it does not. The assert caught it, and reading the true bytes
   is what surfaced §3.1's finding.
4. **A LOOSE COMPLETENESS GATE.** A `^§[1-7]\.` count over the ruling returned **8** where seven
   sections exist — the 8th is a **continuation line** beginning `§3.2's…`. **Enumerated, not
   refined.**
5. ⚠ **A FIGURE WRITTEN WITHOUT BEING MEASURED.** The lane pointer recorded the DRAFT directive as
   "735 l"; it is **626**. The STOP was correct because the assembler *computed* it. Corrected in the
   pointer (0664); a sealed 444 artifact would not have been editable — which is the argument for
   computing every figure rather than typing it.

---

## 6. THE CLAIM BOUNDARY

**Nothing is repaired by this PR.** No product source, no test, no script, no workflow, no
`.github/**`, no dependency, no lock. No test weakened, skipped or deleted. No ENG, WF or standing
rule minted. No fenced ruling and no sealed artifact edited.

**ENG-0191 stays OPEN** until the repair lands. **ENG-0194 stays OPEN**, re-scoped to one consumer.
**ENG-0142 is not closed.** ENG-0193 not built; WF-0086's and WF-0087's gates not built; ENG-0197
open.

⚠ **#1745 — an ISSUE, not a PR — stays OPEN and is NOT closed by this lane, by the repair's merge, or
at any point before a real remote green run**, which the operator closes it against, citing the run
URL. **The operator merges; the seat does not.**

---

# NA-0743 — AS BUILT (PART 2 — THE IMPLEMENTATION)

**This PR IMPLEMENTS the directive PART 1 promoted.** PART 1 above is **not edited**; this part is
appended beside it, and PART 1's input-provenance table remains the only copy of those shas in repo
truth. **Decision:** D-1381 · **Rulings:** R348, R349, R350, R351 · **Base:** main
`b087f859c115a3124494482e48043948b193d8de`, re-derived **bare and unpiped by URL** — exactly ONE
merge past `ae2047e6`, second parent `9bc785f7` = the recorded #1761 head · **Implementation
commit:** `e5da616dc5786542e283285acf7ed2a18817bcb6`.

**ZERO PRODUCT SOURCE BYTES.** Three files changed.

## P2-1. THE RESULT

**The suite red for 187+ days exits 0 on loopback in BOTH of the workflow's scenarios**, at the
**DEFAULT `PULL_LEASE_SECS=60`**, with both peers' EXTRACTED status `== established` at the
assertion checkpoint, compared by equality and never by substring.

## P2-2. THE SEALED EXPECTATIONS, EVERY ONE WITH ITS MEASURED VALUE

| # | verdict | measured |
|---|---|---|
| **E1** RED CONTROL | **HIT** | committed script, loopback: rc **1**, stderr *"alice handshake status is not established"*. Extracted **by equality**: alice `awaiting_peer_confirm`, bob `established_recv_only`. ⛳ **The 187-day hazard, demonstrated rather than argued:** the committed unanchored needle **MATCHES bob and not alice**, while by equality **neither peer is `established`** — which is why every artifact for 187 days named alice alone |
| **E2(a)** REPAIRED | **HIT** | rc **0** end to end; `handshake_status_alice_at_checkpoint=established`, `handshake_status_bob_at_checkpoint=established`, `handshake_checkpoint=established/established before_rehandshake` |
| **E2(b)** DEFAULT KNOB | **HIT** | relay started with `PULL_LEASE_SECS` **unset** ⇒ server default **60**; run green ⇒ **NA-0738's (d) blocker is CLEARED EMPIRICALLY**, not merely as the source claim §2.8 made. `recv_commit` **1** each direction ⇒ antecedent HELD. **Nothing was knob-cured** |
| **E3** X5 | **HIT** | `handshake_status_alice_after_rehandshake=established_recv_only`, `..._bob_...=awaiting_peer_confirm` — ⛳ **a pair SEALED FROM NA-0738's OWN BYTES three lanes earlier, confirmed by an independent run**. Guard: per-log `handshake_status` markers **1 → 2** in both logs ⇒ the X5 read is NEW, not a stale tail |
| **E4(i)(a)** | **HIT** | probe asserting equality to `established_recv_only` **PASSED** ⇒ that is bob's true value at the site |
| **E4(i)(b)** | **HIT** | committed `contains("status=established")` **PASSED TWICE** — unmodified, and again inside the (a) probe where it was deliberately left in place ⇒ **it passes on a value that is not what it names** |
| **E4(i)(c)** NEG CONTROL | **HIT** | needle swapped to `status=awaiting_peer_confirm` ⇒ **rc 101, FAILED** ⇒ the arm executes and CAN fail ⇒ (a)/(b)'s greens are non-vacuous |
| **E4(ii)** | **HIT** | red: **rc 101**, `left: "established_recv_only"` / `right: "established"`. green: **rc 0**. ⛳ **Same value, same point in the flow, opposite verdicts from the old and new assertion** |
| **E5** FABRICATION | **HIT** | `event=qsp_status status=ACTIVE` **0 occurrences** across `markers`, `normalized_subset.txt`, `summary.txt`, `normalized_counts.txt`, in **both** E2 and E7. **forged 0, genuine 0**, classified by two independent discriminators with a control firing in both polarities |
| **E6** NEG CONTROL | **HIT** | `cmp` rc **1** first (differs; sole delta = the `hs2` block's position), then the control run rc **1**: *"alice handshake status is 'established_recv_only', expected exactly 'established'"*. **Both receives DELIVERED** ⇒ antecedent HELD ⇒ the failure is genuinely the checkpoint ⇒ **the RELOCATION is the cause, not a correlate** |
| **E7** drop-reorder | ⚠ **CHECKPOINT HIT · X5-CLAUSE MISS** | rc **0** ✅, checkpoint `established`/`established` ✅, X5 `established`/`established` ✗. **Ruled MISDRAWN AT THE SEAL (R351 §1)** and filed as **ENG-0198** |

## P2-3. ⛳ THE FORGERY WAS LATENT — measured, and it sharpens why removal belongs in THIS lane

`qsc` emits `qsp_status status=ACTIVE` **nowhere in this flow** (0 occurrences in every peer log),
so the fabricated lines were the **only** possible source of that marker. And in the committed
script they sit **below** the assertion that always failed — E1 died at it — so **no real run ever
wrote them**. The forgery would have begun publishing **the moment the ordering defect was fixed**.
This lane removes it in the same act that would have made it reachable.

## P2-4. THE E7 MISS AND ITS MECHANISM — filed as ENG-0198

`handshake_complete` measures **4 in E2 but only 2 in E7**; E7's `hs2_poll_2` emits
`handshake_pending` ×1 and **`handshake_reject` ×4** and nothing else, **exhausting its `--max 4`
budget entirely on rejects**. Every `hs2_*` step still returns **rc 0**, so `set -e` never fires and
the prior session survives — `handshake status` then truthfully reports it. **The stale-tail guard
cannot catch this and is not supposed to:** it proves the status READING is fresh (markers 1 → 2),
not that the re-handshake happened. ⇒ **a re-handshake can silently no-op, and no gate in the tree
can see it.** ⚠ **EXPOSED, NOT CAUSED — labelled as inference:** the committed script never reaches
`hs2` at all, so the reorder is what makes this observable for the first time; not source-proven,
and deliberately not chased.

## P2-5. THE FULL SUITE, BOTH SIDES

| run | tree | rc | binaries | passed | failed | ignored |
|---|---|---|---|---|---|---|
| **BASELINE** | pristine main `b087f859`, `git status` **verified empty at run start**, run to completion **BEFORE any edit existed** | **0** | 134 | 634 | **0** | 2 |
| **AFTER** | the **exact committed tree** `e5da616d` | **0** | 134 | 634 | **0** | 2 |

**Reconciliation:** **IDENTICAL ON EVERY AXIS** — same binaries, same passed, same failed, same ignored. The one test whose assertion changed (`send_ready_markers_na0168`) is green on its **strengthened** assertion, and nothing else moved. Both `rc` captured **bare** — redirection only, so `$?` is the process's
own and not a pipeline's.

## P2-6. THE RULED TEXT WAS AMENDED TWICE, AND THE SWEEP THAT FOLLOWED

⚠ **The ruled §3.4 comment carried TWO base-scoped locators**, each amended before landing on this
seat's measurement, with OLD/NEW read from **the ruling's own bytes** and never retyped:

| ruling | OLD | NEW | controls |
|---|---|---|---|
| **R349 §2** | `…(see :343-346 and the` | `…(see the NA-0622 bootstrap` + `comment below and the` | **2** negative controls HARD-FAILED |
| **R350 §1** | `…the comment ten` + `lines below says obtains…` | `…the NA-0622` + `bootstrap comment below says obtains…` | **3** HARD-FAILED (drifted payload, own output, wrong ruling sha) |

**R350 §1's sweep, needle and counts.** **N1** = number word or digit + `line(s)` + a direction, run
over a **prose view joining consecutive comment lines** — a locator split across a line break is
exactly how the second one hid. **N2** = a deliberately **broad** `:`+digits, then every hit
**classified**, because a needle narrow enough to avoid false positives is narrow enough to miss the
thing sought. **Repaired test 0/0 · repaired script 0/0 · repaired doc 0/0 · E6 control 0/0**, with a
positive control firing on both needles. ⚠ The sweep found **three more locators in the seat's own
prose** — two counted offsets and a cross-file citation that **measured CORRECT** — and all three
were rewritten to content anchors anyway, because the rule is stated for landed text generally.

## P2-7. ⚖ R349 §1 — THE QUOTED-PAIR DEPARTURE, RATIFIED

For a **REMOVAL** (not a retarget), the plant-hazard discipline **overrides** the in-file quoting
habit: quoting the forged token back into the script would tax every future source census of the
very token E5 exists to zero. The pair is preserved where it already lives — the FINAL directive's
§2.2 table, the stops, git history — and the removal is **DESCRIBED** in the comment. **Ruled, not
improvised.** Measured: repaired script `status=ACTIVE` **0 code / 0 comment**; the old ASSERTIONS
*are* quoted, at **0 code / 2 comment**, because there quoting costs nothing.

## P2-8. IDS, RE-DERIVED AT THE EDIT

`D-1381` and `ENG-0198` each measured **0 declarations and 0 mentions** immediately before use;
SR-16 rows **73–76** against a table measured **72 rows, min 1, max 72, no gaps, no
duplicates**. Open-PR set **MEASURED EMPTY** with a positive control returning merged rows, so no
parallel branch could hold them. ⚠⚠ **The R-id naive maximum is 999 and is WRONG** — predecessors'
**published synthetic controls** (`R613`/`R724`/`R809`/`R888`/`R952`/`R999`) are in the tree and
**`R460` is not a ruling id at all** (*"GlobalSign Root R460"*, a CA name). Classified instead:
**R348 highest genuine**, nothing between it and R460 ⇒ **R349, R350, R351**, each verified 0/0
before use. ⚠ `R777` and `R901` are **also SPENT** (3 operator files each). ⚠⚠ **Route B is blind on
this lane's own rulings: the banked R349 file contains ZERO occurrences of `R349` in its own
content** — only its FILENAME carries it.

## P2-9. THE LOOPBACK DOOR, AND THE CLAIM BOUNDARY

`qsl-server` rev `37ec82072cbbd68e4eaba83e192282fbcb96e5b4` (NA-0738's), built in-lane. Bearer
generated locally from `/dev/urandom`, mode 0600, **never printed**; **zero secrets read**,
`relay.env` never opened. Plain **HTTP** on `127.0.0.1`, no TLS ⇒ **this says nothing about the CI
transport path**. `MAX_BODY_BYTES=65536` (AWS parity per NA-0738); **`PULL_LEASE_SECS` never set** ⇒
server default 60. **Auth gate proven live on every start: 401 unauthenticated → 200 with bearer.**

⚠ **NOT CLAIMED:** anything about the remote run's outcome. **Loopback proves the SCRIPT** — now for
both scenarios. ⚠ **#1745 — an ISSUE, not a PR — stays OPEN and is NOT closed by this lane, by this
PR's merge, or at any point before a real remote green run cited by URL. The operator closes it.**

## P2-10. BOUNDS OBSERVED

No `.github/**`, no workflow, no dependency, no lock, no `qsl-server` change to the repo.
`remote_soak.py` **UNTOUCHED**. **No test weakened, skipped or deleted** — the one changed assertion
is strictly STRONGER, proven at E4. E4's probes were added as **untracked NEW test targets and
deleted afterwards**, with the committed test file's sha256 **verified unchanged before and after
every probe run**, so E4(i) is literally *"on the UNEDITED tree"*. No secrets, no relay ssh, no
sudo, no `qwork`/`qstart`/`qresume`/`qnext`, no re-run-to-green. `PULL_LEASE_SECS` never raised.
**ENG-0198 filed, NOT repaired.** ENG-0191's own closure is **not** taken here — it rides a
successor's records act. **The operator merges; the seat does not.**

⚠ **ONE DEVIATION OF THIS SEAT'S OWN, DECLARED AND ACCEPTED (R351 §4):** the directive's order lists
**E6 before E7**; this seat ran **E7 first**. Both are independent single-run measurements against
fixed inputs, each against a freshly restarted relay with a clean store, so neither consumes the
other's state — but the order differs from the order written, and it is recorded rather than
smoothed.
