# KICKOFF TEMPLATE — the operator's paste that opens a lane

**REPO HOME.** This is the canonical, in-repo copy. It is landed here by `NA-0771` (`D-1412`)
as the disposition of `NA-0770` `STOP 010` finding **F-A**. Its source is
`/srv/qbuild/operator/method/KICKOFF_TEMPLATE_20260828.md`, sha256
`ae08050c2adb3267b6b603037b7a7f161a27f1a9a6925c853f52e8b789c0cd0b`, banked 444 and
sha-VERIFIED against its own bytes before being read — all 64 digits compared mechanically
with a negative control proving the comparator discriminates.

**PROVENANCE, CARRIED RATHER THAN TIDIED (F-A).** `PROPOSAL_SR26_20260827.md` sec 4 orders
*"the kickoff template updated in the same lane"*. `NA-0770` searched at that edit and found
**no kickoff-template artifact anywhere** in the repo or in `/srv/qbuild/operator` — the only
matches for *"kickoff"* were three documents MENTIONING one. **The premise "there is a
template to update" measured FALSE**, so the consumer was CREATED rather than amended, and
that is recorded here rather than smoothed over. ⚠ The Director should rule whether this is
the artifact intended; nothing depends on it until he does.

**WHY THIS FILE EXISTS.** `D-2`/`R305`: a rule without an executable consumer decays. `SR-26`
(stop-file self-containment, audited as an instrument) lives in `docs/ops/STANDING_RULES.md`.
Its instrument is the audit itself, run by the seat over the finished stop's bytes. **This
template is where the seat is TOLD to run it**, at the one moment the seat is guaranteed to
read: the kickoff.

**ASCII ARMOR.** The two blocks below are carried **byte-verbatim from the source file**,
including its ASCII armor, in which `" -- "` denotes an em dash. **No transform was applied**,
so no transform has to be proven invertible.

---

## 1. THE SR-26 BLOCK TO PASTE INTO EVERY KICKOFF

Copy verbatim; fill the bracketed slots.

```

  SR-26 BINDS THIS LANE'S STOP FILES FROM THE FIRST ONE. Before banking ANY
  stop, run the self-containment audit MECHANICALLY over that stop's finished
  bytes and PRINT THE RESULT INSIDE THE SAME FILE:
    (a) absolute path, filename and sha256 of the stop, at its head AND its end;
    (b) every question the governing brief/ruling asks and every record it
        orders, enumerated, EACH with the quoted line in the stop that answers
        it -- an item with no quote is a GAP: cure it, or state it unanswered
        and why;
    (c) confirmed as measured properties, not assertions: every document the
        PROSE cites is CARRIED IN FULL (strip embeds before counting, so a
        filename inside a carried document is not miscounted) - every record
        PROPOSED is drafted as TEXT, not named as a destination - every control
        prints BOTH ARMS and their values - every measured output quoted
        VERBATIM - what was NOT measured stated plainly with its reason, n=,
        and the claim boundary;
    (d) the verdict line, exactly:
        "SELF-AUDIT COMPLETE -- N items checked, M gaps found and cured, K
         stated unanswered."  (a clean run says zero)
  The audit is a RUN, not a recollection or a checklist read. A stop without its
  printed result is incomplete on its face. The cure for a gap is always to
  CARRY MORE, never to claim less. A supplement chain is not a stop file: if a
  banked stop is found defective, RE-ASSEMBLE it as a new whole numbered stop.
```

---

## 2. THE REST OF THE KICKOFF, IN ORDER

```
== THE REST OF THE KICKOFF, IN ORDER ==
1. LANE ID and one-sentence subject. [NA-XXXX / subject]
2. GOVERNING DOCUMENTS, each with its full sha256 and its banked path, to be
   verified digit-by-digit by the seat with a negative control on the
   comparator. [paths + shas]
3. BASES: each repo's main, to be RE-DERIVED by the seat, bare and unpiped, at
   the NAMED github remote -- never at `origin`, which is the local mirror.
4. STANDING CONSTRAINTS, restated every lane because they are what a tired seat
   drops first: no .github/** - no secrets - no sudo - no qwork/qstart/qresume/
   qnext - named github remotes for anything load-bearing - noreply identity per
   repo - no Co-Authored-By trailer - THE OPERATOR MERGES EVERYTHING.
5. THE STOP CONDITION, stated as a trigger that stays ARMED through the build,
   and named concretely enough that the seat can recognise it under momentum.
   [what specifically stops the lane]
6. THE FIRST STOP'S CONTENT: what must be measured and reported before anything
   is edited or deleted.
7. WHAT THE SEAT MAY NOT DO. [explicit prohibitions]
```

---

## 3. THE PROPERTY CHECKLIST — ADDED BY `D-1413` (P-FACADE) AND `D-1414` (L1–L5)

⚠ These lines are **not** in the source template: they are the consumers those two orders
name, landed here in the same records act that files the orders themselves. **Each line
carries the order and lever it comes from, so a later reader can check it against its source
rather than against this file.**

Paste as item **8** of section 2 above:

```
8. THE PROPERTY CHECKLIST -- answered in the kickoff, not discovered at the PR.
   (a) Does this lane add or change a CLI subcommand, flag or config key? If yes:
       name the facade call it wraps and the GUI affordance that uses it, OR
       declare it lab-only with the reason -- and name its facade test.
   (b) Does this lane add a Tauri command? Name the facade call it wraps.
   (c) Does this lane add a facade function the GUI will call? Then name the
       desktop PR.
   (d) Which driver test does this lane add?
```

| line | source order | sha256 | lever |
|---|---|---|---|
| **(a)** first clause | `ORDER_one_facade_two_frontends_20260828.md` | `b812117e844862f26aeebd99f5df66d0e37533364297d96925be7c56d322b594` | sec 3(a) — **P-FACADE** |
| **(a)** *"and name its facade test"* | `ORDER_gui_build_levers_20260828.md` | `e42d0e69ac5c7443b6edf6a091b3b5bc85ca5d04bb48362cb37a0513841d2944` | **L1**, which *extends* the line above rather than adding one |
| **(b)** | `ORDER_gui_build_levers_20260828.md` | `e42d0e69…41d2944` | **L2** — *"applies to desktop lanes verbatim"* |
| **(c)** | `ORDER_gui_build_levers_20260828.md` | `e42d0e69…41d2944` | **L3** — the pin moves with the facade |
| **(d)** | `ORDER_gui_build_levers_20260828.md` | `e42d0e69…41d2944` | **L5** — one driver test per GUI lane |

⚠⚠ **THE COUNT IS RECONCILED HERE RATHER THAN LEFT TO READ DOUBLE.** The `ENG-0252` kickoff
sec 5(3) enumerates **five** items for this checklist; `CLOSEOUT_NA0770_laneA_20260829.md`
sec 8 calls them *"the three kickoff-checklist lines"*. Measured against the orders' own
words, both are right about different things and **the checklist is FOUR lines**: the
one-facade order contributes line (a); `L1` **extends** (a) rather than adding a line (*"the
kickoff-template line from the one-facade order extends to '…and name its facade test'"*);
and `L2`/`L3`/`L5` each contribute one — which is the close-out's three. Five *items*, three
*new* lines, four *lines total*.

---

## 3b. THE QUEUE BLOCK'S BIRTH, AND ITS STATUS LINE — ADDED BY `WF-0090`

⚠⚠ **THE ORDERING RULING SAID "UNDER THE BLOCK-BIRTH STEP", AND THERE WAS NO SUCH STEP.**
Measured at this edit: section 2's seven items name the lane id, the governing documents,
the bases, the standing constraints, the stop condition, the first stop's content and the
prohibitions — **and never mention the queue block at all** (`block-birth` / `queue block`
occurrences in this file before this edit: **0**). The premise *"there is a block-birth
step to add a line under"* therefore **measures FALSE**, so the step is **CREATED here
rather than amended**, and that is recorded rather than smoothed over. ⚠ This is the same
disposition, on the same file, that `NA-0770` took when `PROPOSAL_SR26` sec 4 ordered *"the
kickoff template updated"* and no template existed — see the PROVENANCE note at the head.
⚠ It is also why the line is NOT inserted into section 1 or 2: those two blocks are carried
**byte-verbatim** from the 444 source and are proven so by a round-trip diff; editing inside
them would destroy the proof that they are verbatim.

**8. BIRTH THE LANE'S `### NA-####` QUEUE BLOCK.** The block is created at **PROMOTION**,
not at enqueue — it is what promotion AUTHORIZES, so it exists at the moment of
authorization (Director ruling 2026-07-27, OBS-DY). Its status line is:

```
   Status: READY (D-####)          at promotion
   Status: MERGING (PR #N)         at the PR
   Status: DONE (<result class>)   at close
```

⚠⚠ **BARE AND UNBOLDED, EXACTLY AS WRITTEN.** `scripts/ci/qsl_evidence_helper.py queue`
matches `^\s*-?\s*Status:\s*([A-Z_]+)\b` and requires **EXACTLY ONE** lane whose token is
`READY`. `[A-Z_]+` cannot pass a `*`, so **`Status: **DONE**` is invisible to it**, and
`Status: PROMOTED for **D-####**` yields `PROMOTED` — which is what made `READY_COUNT`
measure **0** on merged main and `qwork` fail closed at `queue-helper-failed`, live, on
2026-08-29. ⚠ Note the asymmetry the seat must hold in mind: `scripts/ci/preflight_governance.sh`
fails only when that count is **> 1**, so **zero is valid to the preflight and fatal to
`qwork`** — the two gates disagree about zero, and that disagreement is FILED as `WF-0090`
and NOT settled by this template.

---

## 4. NOTES FOR THE OPERATOR

- The `SR-26` block in section 1 is the only part that is not lane-specific. Everything else
  is filled per lane.
- If a lane is a records-only or filing-only act, **`SR-26` still binds**: those lanes produce
  stop files too, and their stops are the ones most often thinned.
- The property checklist in section 3 is answered **in the kickoff**. A lane that answers
  *"none of these apply"* says so explicitly rather than omitting the section.
