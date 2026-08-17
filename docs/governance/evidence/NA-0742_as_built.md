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

---
---

# NA-0742 — AS BUILT (PART 2 — THE IMPLEMENTATION)

**Lane:** NA-0742 / **D-1379** — LANE 2 OF THE ENG-0142/ENG-0196 REPAIR PROGRAM.
**This PR IMPLEMENTS the directive PART 1 promoted.** PART 1 above is **not edited**; this part is
appended beside it, and PART 1's input-provenance table remains the only copy of those shas in repo
truth.
**Base:** main `b69600e559271d3c09004d7dcb48ea05dd91f022` — re-derived by this seat, **bare and
unpiped, BY URL** against the NAMED GitHub remote. **Exactly ONE merge past
`215972779514c636cceb4c6752ab7b5401002532`**: parents `21597277` + `a1c6c969`, the #1759 promotion
merge, whose head sha is byte-identical to the one the promotion banked. Open-PR set **MEASURED
EMPTY**, with a positive control returning #1759/#1758/#1757 by `mergedAt`.
⚠ **NO CHECK STATE IS CLAIMED ANYWHERE IN THIS DOCUMENT.**

---

## 7. THE EXECUTABLE DOCUMENT WAS VERIFIED BEFORE IT WAS CONSUMED

`DIRECTIVE_NA0742_FINAL_R345.md` — **sha256 `e947014c4639bb41e0218083b01f310019e0d8771ed1e888d364c20cb539d119`,
535 lines, 34511 bytes** — matched `LATEST.md` exactly before a single instruction was executed, as
did R345 (`9e8283d1…`), R346 (`e0733ce4…`), CARGO v2 (`62ddf832…`) and STOP 004 (`b4acf5e6…`).
**The promotion merge touched records only** — `git diff --stat 21597277..b69600e5` returns six
records files, `+148/−3`, and **zero product source** — so every §3.8 content pin and every line
anchor the directive measured at `21597277` applies unchanged at this base. **They were re-derived
anyway rather than inherited**, and every one reproduced.

## 8. ⚠ THE FIRST ACT WAS RED, AND THE RED FIGURES ARE THE DELIVERABLE

The arms were written and run **on the unrepaired tree, in this session, before any product byte
changed**. The run is preserved and was never re-run to green (R332.1).

| arm | class | measured RED on the unrepaired tree | after the repair |
|---|---|---|---|
| **T1** ENG-0196 spelling 1 | RED-FIRST | `invite finish` rc≠0, `QSC_MARK/1 event=error code=handshake_envelope_malformed` | rc 0, `invite_finish=ok`, `handshake_complete role=initiator` |
| **T2** ENG-0196 spelling 2 (E1b) | RED-FIRST | rc≠0, `code=handshake_envelope_version_newer` — ⛳ **the first time any test in this tree has ever produced that code** | rc 0, and the code is **unreachable on this path** |
| **T3** zero residue | BASELINE CONTROL, BOTH NUMBERS | **3 residue frames, each NAMED rather than counted**: inviter inbox `51 48 53 4d` (the A2), redeemer inbox `01 02 02 00` (the invite reply), invite slot `01 01 01 0c` (the A1 envelope) | **0** |
| **T4** the tax ends | BASELINE CONTROL, BOTH NUMBERS | **skip tax = 8** (4 inviter + 4 redeemer — ⚠ the same frame re-skipped once per receive round, not one per frame) | **0** |

⚠ **`PULL_LEASE_SECS = 1` for T3 and T4, and it is stated beside every figure they produce.** The
reason is mechanical: a leased-but-unacked row is **invisible to a pull**, so a residue probe run
before expiry reports "empty" for a mailbox that is merely reserved. Waiting past expiry is what
lets the probe tell **ACKED (deleted, gone)** from **LEASED (coming back)**. Every other arm runs at
the production parity figure `PULL_LEASE_SECS = 60`.

## 9. THE FULL ARM SET, CLASSIFIED PER ARM AS RULED

| arm | class | result |
|---|---|---|
| T1, T2 | red-first | ⛳ RED then GREEN |
| T3, T4 | baseline control, both numbers | ⛳ both reported |
| T5 (status pin) | post-repair | ⛳ asserts the injected fault is **500 and never 404** |
| T5f finish | post-repair, antecedent-controlled | ⛳ commit lands · ack 500'd · **finish STILL rc 0** · RESP redelivers after the lease · **the retry's ack lands `acked=1`** |
| T5a accept | post-repair, antecedent-controlled | ⛳ accept rc 0 · slot reads Redeemed · the A1 redelivers on the **slot** mailbox |
| T5p poll | post-repair, antecedent-controlled | ⛳ poll rc 0 · session intact · ⚠ **and a measured MISS — see §12** |
| T6 the message survives | post-repair, antecedent-controlled | ⛳ the scanned-past message is delivered **byte-intact** |
| T7 mode discrimination | post-repair, antecedent-controlled | ⛳ **Legacy destroys exactly 1 of 4 and emits no scan marker at all; Lease leaves all 3 collateral and consumes only the reply** |
| T7b markers as rendered | post-repair | ⛳ no `<redacted>`, all five fields present, `classes=` digit-free and drawn only from the fixed vocabulary |
| T8 the a2_sig exit does not ack | post-repair | ⚠ **written as ruled; its driving arm COULD NOT BE EXECUTED — see §12** |

**Every T5/T6/T7 arm asserts its ANTECEDENT with a named failure before it scores anything** — that
the fault actually fired, that the frame was actually consumed, that the scan actually ran. An arm
whose injected fault never fires is measuring the happy path under a fault's name.

## 10. THE PROTECTED RECEIVE-PATH REGIONS — BYTE-UNCHANGED, PINNED BY CONTENT

**Slicing convention, stated because a byte count is an artefact of it: lines joined by `\n` WITH a
trailing `\n`.**

| region | unique first line | lines | bytes | sha256 | before | after |
|---|---|---|---|---|---|---|
| the `Err(code)` arm (lane 1's) | `let from_alias = peer_alias_from_channel(ctx.from);` | 64 | 4285 | `6be034fe77f03006…` | ✓ | ✓ |
| the per-item loop | `for item in items {` | 771 | 45238 | `3b333e341d01dabc…` | ✓ | ✓ |
| the pull loop | `'pull: loop {` | 798 | 46520 | `04dce067e26e64a2…` | ✓ | ✓ |

Each anchor was asserted **unique in the file (1 occurrence)** before being used, and a
last-character-mutation negative control fails **all three**, so the check discriminates. ⚠ The
unusable anchors were **measured, not inherited**: bare `}` occurs **421** times and `break 'pull;`
**twice** — neither may anchor anything. Line numbers are reported for convenience and were never
used as the locator.

## 11. THE CENSUS OF RECORD, RE-DERIVED WITH BOTH INSTRUMENTS

| instrument | files | `#[test]` |
|---|---|---|
| A — library symbols (the sealed `consumer_census.py`, executed from its sealed location) | 15 | 52 |
| B — CLI argv pairs | 32 | 118 |
| **UNION — the census of record** | **32** | **118** |

`A − B = 0`; `B − A = 17`; **unexplained = 0 by construction of the union**. Reproduces the ruled
figures exactly. ⚠ **The blindness control is the sharper result:** the SAME argv-pair needle applied
**line by line** sees only **8 of the 32 files — 24 invisible, 75%** — because the prevailing form
wraps each token onto its own line. This program had recorded that shape at 67% on `emit_marker`;
here it measures 75% on a different construct.

## 12. ⚠⚠ EVERY MISS, WITH ITS MEASURED VALUE BESIDE IT — RECORDED, NONE TUNED

**(A) THE RETARGET THAT §6's TARGET-ZERO DID NOT PREDICT — REPORTED, NOT APPLIED.**
`na0741_frame_class_dispatch.rs:889` fails on the repaired tree. Old text, verbatim:

```
    assert!(
        has_marker_line(&r_text, "recv_frame_skipped", &["class=invite_resp"]),
        "the invite reply must be SKIPPED by class, not decoded:\n{r_text}"
    );
```

It asserts that a completed invite leaves an invite reply wedged in the redeemer's ordinary inbox.
**This lane removes exactly that residue at its source**, so the redeemer's `receive` now reports
`recv_none`. A proposed replacement — **NOT APPLIED, the Director's to rule**:

```
    assert!(
        r_text.contains("event=recv_none"),
        "NA-0742: `invite finish` now ACKS the reply it consumes, so the redeemer's inbox is \
         clean and there is nothing left to skip:\n{r_text}"
    );
```

⚠ **The sibling assertion at `:866` still passes** — the inviter's `class=handshake` A2 survives
only because that arm never runs `handshake poll`, which is the caller that would retire it.
⚠ **Why the census predicted zero:** all five idioms enumerated consumers of the COMMAND's outputs
and **none asked which tests assert that the RESIDUE EXISTS**. **The directive's retarget target is
ZERO and a retarget is a STOP, so nothing was retargeted, weakened, skipped or deleted.**

⚠⚠ **SUPERSEDED IN ITS DISPOSITION BY §18, AND MARKED RATHER THAN REWRITTEN.** The paragraph above
records what was true at STOP 006: the retarget was reported and **not** applied. **R347 §1 then
AUTHORIZED it**, and §18 below carries the applied pair, the §1(b) witness measurement, and the branch
taken. The text above is left as issued.

**(B) THE F-C SWEEP — PREDICTED RECORDS-CLASS HITS, MEASURED ZERO.** §4 stated both marker names are
planted by the directive and the impl seat should expect records-class hits. **Measured: `0` files
tree-wide for `invite_scan_summary` and `0` for `producer_ack`**, split by path class. The banking
lives in the **operator tree**; the sweep's scope is the **repo**, and the promotion's landed records
never name either marker. ⇒ **the plant hazard is scoped to the tree the sweep covers.** Controls:
positive `recv_frame_skipped` = **8 files** in exactly the recorded 6-records / 1-source / 1-test
split; negative sentinel = 0.

**(C) T5p — THE RETRY'S ACK DOES NOT LAND, AND THE CRASH COST IS NOT UNIFORM.** §5 predicted *"the
retry's consume+ack lands `acked=1`"*. Measured on the retry over an already-processed frame:
**0 producer-ack markers, 1 frame still resident.** An already-processed A2 no longer decodes into a
consuming branch and reaches `hs_emit_decode_reject; continue`, which by design never acks. ⇒
**`invite finish`'s retry re-consumes and its ack lands (`acked=1`, measured in T5f), so its crash
cost is exactly one lease period with nothing left behind; the poll's retry cannot re-consume, so
its frame is retired by no retry at all and ages out only on the relay's retention TTL.** Bounded
and harmless — lane 1 skips it by class on every receive — but a **permanent** orphan rather than a
transient one. Both numbers are now pinned by the arm.

**(D) T8's RULED MECHANISM IS UNBUILDABLE AT MAIN.** R345 §3(a) ordered the a2_sig exit driven
through `hs_rng_failure_forced("QSC.SIG.A2")`, which compiles only under `--cfg
qsc_rng_failure_test_seam` (D-0883: *"normal builds must not read the seam selector"*).
**Measured: `RUSTFLAGS='--cfg qsc_rng_failure_test_seam' cargo check -p qsc --lib` fails with 4
compile errors in `qsl/qsl-client/qsc/src/vault/mod.rs` (`:570`, `:578`, `:728`, `:733`), reproduced
on a worktree of pristine main with ZERO modified files.** It is invisible because **no workflow
builds that cfg**, so every `#[cfg(qsc_rng_failure_test_seam)]` arm across six existing test files is
equally unexecutable today. The arm is written as ruled and committed, so it becomes executable the
moment the seam build is repaired; its `#[cfg(not(...))]` companion **did run and pass** in the
default suite, proving the seam is genuinely absent from a default build rather than merely unused.
⚠ **A ledger filing is PROPOSED and NOT MINTED** — the next free ENG id measured `ENG-0197`, and
minting it is the operator's act, not this seat's.

## 13. E3 — DISCHARGED, IN THE INSTRUMENT'S OWN TERMS

The store has **no `acked` column**: an ack DELETES the row, so *"zero unacked rows"* is not a column
to read. ⚠ The committed harness starts the relay with `StoreConfig::path = ":memory:"` — **there is
no SQLite file for NA-0740's sealed `census.py` to open**, found by reading `StoreConfig::default()`
rather than by a failed run. A **clone-only** harness therefore stood the same real relay up
**file-backed**, through `qsl_server`'s public API only, changing **no product byte and no committed
harness**; it is not part of the deliverable and was never placed in the seat.

| census | routes | rows | unexplained routes / rows |
|---|---|---|---|
| **A — after `invite redeem` (POSITIVE CONTROL)** | 1 | **1** — the A1 resident in the invite slot, `head8=01 01 01 0C`, `len=7463` | 0 / 0 |
| **B — the full flow with every ack landed** | 0 | **0** | **0 / 0** |

⛳ **This discharges lane 1's owed E3(b)**, which died a MISS ON PREMISE for want of exactly this
mapping. The committed T3 arm asserts the same proposition through the instrument a test has — a
raw lease pull per touched mailbox, measured past expiry — and states plainly that it **cannot**
measure `unexplained`, which is why the census above exists.

## 14. THE MARKERS, AS RENDERED

Captured from a real run, not from a source census — a claim about rendered output cannot be checked
by reading source:

```
QSC_MARK/1 event=invite_scan_summary scanned=1 pulls=1 truncated=false selected=invite_resp classes=invite_resp
QSC_MARK/1 event=producer_ack caller=finish sent=1 acked=1
QSC_MARK/1 event=producer_ack caller=poll sent=1 acked=1
```

⚠ `classes=` carries **bare class names only, deduped, from the classifier's fixed five-token
vocabulary, with no digits** — redaction-safe **at any length by construction** rather than by
staying under 24 characters. The redactor fires on `len() >= 24` **plus a digit**, which is what cost
lane 1's F-B its entire marker-tax diagnostic. Counts ride as their own short fields. T7b asserts the
**rendered** line contains no `<redacted>`, that all five fields are present, and that `classes=`
contains no digit and no token outside the vocabulary.

## 15. THE EDIT SET, THE GATES, AND THE IDS

**Eight authorized rows and nothing else** — `invite/mod.rs` (+210/−4), `handshake/mod.rs` (+78),
`transport/mod.rs` (+42/−1), `cmd/mod.rs` (**+1, a doc comment**), the new test file, one line in
each shard manifest, and records. **`frameclass.rs` is byte-unchanged: consumption only.**
⚠ **One visibility token was widened and it is stated rather than slipped in:** `AckFlushOutcome`
became `pub(crate)`, because §3.5's RULED signature returns it and a caller that cannot name the type
cannot tell `LegacyComplete` from a zero. No variant, no behaviour and no receive-path byte changed.

- **The shard-manifest gate, BOTH POLARITIES:** rc **1** with the test file present and unregistered,
  naming the exact missing target on **both** manifests; rc **0** after one line each; census **134
  targets / 134 manifest rows**, doc shard with 0 co-tenants, on Linux and macOS alike.
- **rustfmt:** the four touched source files carry **18 pre-existing hunks before the edit and 18
  after** — **zero new debt**; the one hunk this seat introduced was found by that comparison and
  fixed. The new test file is rustfmt-clean. ⚠ `cargo fmt --check` is red across ~60 files at main
  and **is not a CI gate** (no workflow runs fmt or clippy), so nothing else was reformatted.
- **IDS RE-DERIVED AT THE EDIT**, declaring forms, both controls per space, classified before any
  maximum: NA max **742** (this lane's own block, landed by the promotion — no new NA taken) · D max
  **1378** ⇒ **D-1379** · SR-16 numbered table **n=60, min 1, max 60, no gaps, no duplicates** ⇒ rows
  from **61** · ENG max 196 · WF max 87 · SR max 22. `D-1379` measured **ZERO occurrences tree-wide**
  immediately before the edit; every positive control SEEN, every negative control absent.

## 15b. THE FULL SUITE, BOTH SIDES

| run | tree | rc | binaries | passed | failed | ignored |
|---|---|---|---|---|---|---|
| **BASELINE** | pristine main `b69600e5`, **run to completion BEFORE any edit existed** | **0** | 133 | 622 | **0** | 2 |
| **AFTER** | the committed implementation tree | **101** | 134 | 633 | **1** | 2 |

**Reconciliation: +1 test binary (133 → 134), 0 removed, 0 assertions retargeted.** Both `rc`
were captured **bare and unpiped** — redirection only, so `$?` is cargo's own and not a pipeline's.

⚠⚠ **THE AFTER-RUN's ONE FAILURE IS THE RETARGET OF §12(A) AND NOTHING ELSE**, and the baseline is
its control: `both_mailboxes_of_a_completed_invite_receive_cleanly` **PASSES on the pristine
baseline tree and FAILS here**. That A/B is the proof of cause — the failure is this lane's change
removing the residue that assertion requires, not a pre-existing red and not a flake. **It was not
retargeted.**

## 15c. ⚠ THE F-C SWEEP, RE-RUN AT THE COMMIT — AND A NEEDLE THAT OVER-COUNTS ITSELF

At the base both names measured **0 files**. At this commit, split by path class:

| needle | files | by class |
|---|---|---|
| `invite_scan_summary` | 5 | spine 1 · evidence 1 · docs 1 · product-source 1 · tests 1 |
| `producer_ack` | 10 | spine 2 · evidence 1 · docs 1 · product-source 3 · tests 1 · **scripts 2** |
| `recv_frame_skipped` (positive control) | 10 | spine 3 · evidence 2 · docs 2 · product-source 1 · tests 2 |
| fresh negative sentinel | 0 | — |

⚠ **THE TWO `scripts/` HITS CARRY NO MARKER AT ALL.** They are the two shard-manifest lines naming
the new test file, `tests/na0742_invite_finish_scan_producer_acks.rs` — and the marker name
`producer_ack` is a **substring of that filename**. ⇒ **a marker-name sweep collides with any FILE
NAMED AFTER THE MARKER**, so the honest figure is **8 marker-bearing files plus 2 filename-only
collisions**, recoverable only by looking at the matched line rather than counting matched files.
Same family as "count declarations, never mentions", one level down: **count OCCURRENCES OF THE
CONSTRUCT, not files containing the string.**

## 16. THE CLAIM BOUNDARY

Loopback plain HTTP against an in-process `qsl-server` at the pinned rev
`131d63f4865544addd2784c305970b21ddbeb69c`. `rustc 1.95.0`. Parity knobs are the default
(`PULL_LEASE_SECS=60`, `MAX_BODY_BYTES=65536`); the arms that need a lease to EXPIRE set
`PULL_LEASE_SECS=1` **and state it beside every figure**. **n = 1 per arm.** Zero secrets read; no
relay contacted; no sudo; `qwork`/`qstart`/`qresume`/`qnext` **not run by this seat**; not through
TLS; **NOT a CI claim**. Server parity with the production relay is inherited from the SR-15 read,
not re-derived here.

## 17. WHAT THIS PR DOES NOT DO

**ENG-0196's ledger entry is NOT closed.** The repair is implemented and proved; the entry's
disposition is a records act this PR's mandate did not enumerate, and NA-0741 set the precedent by
repairing ENG-0142's non-adversarial trigger without closing it. ENG-0142 does not close.
**`invite accept` gets an ack, not a scan**, so a foreign frame at the head of the invite SLOT still
blocks it — capability-gated, inside ENG-0142's remaining adversarial clause, **named and not
fixed**. **Guard 2** — a type-state `Committed` witness that would make the commit-then-ack ordering
unrepresentable-if-wrong at compile time — is **recorded as the right successor shape and not
built**. ENG-0191's (a)–(e) stay the operator's; ENG-0194 is not repaired; ENG-0193 and WF-0086's and
WF-0087's gates are not built; **#1745 — an ISSUE, not a PR — stays OPEN**. No test was weakened,
skipped or deleted, and **no assertion was retargeted**. No fenced ruling and no sealed artifact was
edited. No dependency, no lockfile, no `.github/**`, no `qsl-server` change.
**Nothing is merged: the operator merges, the seat does not.**

## 18. THE RETARGET, APPLIED — AUTHORIZED AT R347 §1

**R347** (banked verbatim 444, `d0211bc55b885daa…`, 63 lines, 4762 bytes; ⚠ it arrived as chat text,
so no byte source exists to diff the banking against — only the Director can confirm byte-identity)
**AUTHORIZED the retarget under the quoted-pair discipline**, on the ground that the expectation at
`na0741_frame_class_dispatch.rs:889` **encodes the residue this lane exists to remove**. §12(A) above
is left as issued and superseded only in its disposition.

### 18.1 THE PAIR

**OLD** — the expectation as it stood, verbatim:

```rust
    assert!(
        has_marker_line(&r_text, "recv_frame_skipped", &["class=invite_resp"]),
        "the invite reply must be SKIPPED by class, not decoded:\n{r_text}"
    );
```

**NEW** — the post-repair truth, which is also lane 2's E4 property observed from lane 1's own
arrangement:

```rust
    assert!(
        r_text.contains("event=recv_none"),
        "NA-0742: `invite finish` now ACKS the reply it consumed, so the redeemer's inbox must be \
         EMPTY by the time this receive runs:\n{r_text}"
    );
    assert!(
        !r_text.contains("recv_frame_skipped"),
        "NA-0742: there is nothing left to skip — the residue is removed at its source, not \
         stepped over once per lease period:\n{r_text}"
    );
```

Both are quoted **side by side in the test's own comment** as well, citing **D-1379** and **R347**,
so the delta is readable at the assertion without a diff. **Nothing else in that file changed:** the
diff is **one hunk, +34 / −2**, and the two deleted lines are exactly the old assertion's body.

### 18.2 ⚠ §1(b) — THE LANE-1 PROPERTY KEEPS ITS WITNESS. **BRANCH TAKEN: NO COMPANION ADDED.**

The ruling required this to be **measured**, not assumed: does any OTHER committed arm assert a
`class=invite_resp` skip **on a pulled frame**? Measured across every tracked Rust test file with a
multi-line-tolerant needle (the single-line form is blind to the wrapped assertion shape this corpus
prefers — the 67%/75% blindness this lane has now measured twice):

| witness | arm | frame planted by | pulled by |
|---|---|---|---|
| `na0741…:666` | `lease_skips_where_legacy_still_aborts` | `push_raw` | `receive` |
| `na0741…:753` | `the_skip_marker_leaks_nothing` | `push_raw` | `receive` |
| `na0741…:944` | `foreign_litter_at_the_head_still_delivers_up_to_max` | `push_raw` | `receive` |
| `na0741…:373` | `foreign_frame_arm` — ⚠ **parameterized**, asserts `class={expect_class}` and is called with `"invite_resp"` once (by T1) | `push_raw` | `receive` |
| ~~`na0741…:889`~~ | the arm retargeted here | — | — |

⇒ **FOUR other witnesses survive** (three literal, one parameterized whose literal never appears at
its own assertion site and which a naive census would have missed). **"Receive skips invite replies
rather than decoding them" does not lose its only witness, so the synthetic-plant companion was NOT
added.** Controls on the instrument: negative `class=zzz_absent` → **0**; positive `class=handshake`
→ **1**, i.e. the same shape finds the sibling class.

⚠ **AN INSTRUMENT CORRECTION MADE ON MYSELF, IN THE OPEN.** The first run of that census reported
**8** assertions because two regexes matched the *same* site at different offsets and the dedup was
keyed on the start line. Re-keyed on **merged spans**, the truth is **4**. A census whose dedup key
is not the thing being counted inflates by exactly the number of ways it can look at one object.

### 18.3 AFTER THE RETARGET

`cargo test -p qsc --test na0741_frame_class_dispatch` → **rc 0, 7 passed, 0 failed**: the retargeted
arm is green and all three surviving literal witnesses still pass.

### 18.4 ⚠ A FIGURE OF MINE, CORRECTED — AND THE ORDERING RULING INHERITED IT

STOP 006 §3(D) said the seam break leaves **"six test files"** unexecutable, and R347 §2(a) repeated
that number. **Measured at this base: NINE test files carry a `#[cfg(qsc_rng_failure_test_seam)]`
arm — EIGHT pre-existing plus this lane's own** — together with **5 product-source files** gated by
the same cfg, and **0 workflows** that build it. The "six" came from a **truncated grep listing read
as if it were a census** — a figure quoted without re-deriving it, which is the shape SR-21 exists
for. **ENG-0197 is filed with the measured number; the ruling stands as issued and this correction
sits beside it.**

### 18.5 THE RECORDS R347 §2 ORDERED

- **(a) ENG-0197 FILED**, born countable, bracketed between `### ENG-0196` and `### WF-0087` with
  both neighbours measured untouched: Severity **P3**, Status **open** (declared at filing by R347
  §2(a) / D-1379), the four `vault/mod.rs` error sites, the pristine-worktree control, the measured
  blast radius, the fact that **no workflow builds the cfg**, and the successor with a **cfg-building
  CI job** as the named candidate gate.
- **(b) F-C gains its SIXTH instance** beside WF-0087, bracketed to that block: **a marker name as a
  SUBSTRING of a test filename inflates a file census** — 2 of `producer_ack`'s 10 files are the
  shard-manifest lines naming `..._producer_acks.rs` and carry no marker at all. ⇒ **count
  occurrences of the CONSTRUCT, not files containing the string.**
- **(c) SR-16 rows 68–70**, numbers re-derived at the edit (the table measured contiguous to 67, no
  gaps, no duplicates): the placeholder fill that stamped another lane's row; the remove-a-state
  enumeration property; and the **HIT** — the baseline-as-control A/B.
- **(d) ALREADY CARRIED, so nothing was duplicated.** §12(C) above and D-1379 both already state
  T5p's non-uniform crash cost — the poll's orphan **permanent until retention, harmless, skipped by
  class on every receive**, finish's **transient**. Measured rather than re-landed.

⚠ **R347 §4's result class `INVITE_FLOW_RESIDUE_ZERO_PASS`, the DONE flip, ENG-0196's
repaired-amendment and its disposition question, and lane 1's E3(b)-discharged note ALL ride the
NEXT records act, not this PR** — enumerated in the ruling so none can be lost, and repeated here so
this document cannot be read as having landed them.
