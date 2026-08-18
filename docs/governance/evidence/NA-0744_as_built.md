# NA-0744 / D-1382 — AS BUILT: PULL-PATH INSTRUMENTATION (ENG-0193) + THE REMOTE INVITE ROUND-TRIP GATE (WF-0086)

**PROMOTION PR — RECORDS ONLY. ZERO PRODUCT SOURCE BYTES.** Nothing is instrumented here and no gate
is built here; this PR authorizes a later implementation lane and lands the NA-0743 close set.

**Base** main `d484c065ea0520b696aa9ac5555effae363870ba`, verified UNMOVED **by URL, bare and
unpiped** against the NAMED GitHub remote, with the open-PR set **MEASURED EMPTY** against a positive
control returning rows. **Rulings** R352, R353, R354. **Operator** ASK-2 [O] 2026-08-18.

## 1. THE FINDING

The relay boundary's **receive half is the most instrumented region in `qsc/src/transport/mod.rs`** —
**43** `emit_marker` sites, **39** distinct marker names (`receive_pull_rounds` 28, `receive_execute`
9, `flush_pending_acks` 4, `receive_pull_and_write` 1, `quarantine_then_ack` 1). **What is mute is the
HTTP transport boundary beneath it:** `relay_inbox_pull_mode` (13 exits), `relay_inbox_ack` (11) and
`producer_ack` (2) emit **zero markers across 26 exits**, while their push-side counterpart
`relay_inbox_push_inner` emits a structured `relay_push_diagnostic` on **both** of its exits.
⇒ **When the pull itself fails, or returns nothing, nothing says why** — ENG-0193's recorded
consequence, and why ENG-0192 had to be found by computing sha512 over four candidate strings.

Complementing it: **`invite` occurs 0 times in all of `scripts/demo/`** — no script in the tree,
loopback or remote, exercises the invite first-contact path.

## 2. FIVE BRIEF PREMISES MEASURED FALSE — CORRECTED IN THE OPEN, NOT COPIED

| premise | measured at `d484c065` |
|---|---|
| "the push half explains itself in 38 sites" | no interpretation returns 38; emitter call sites **2**, field keys 14, distinct vocabulary 35, return arms 39, token occurrences **28** — and **28 is the ledger's own figure** in ENG-0193's heading |
| "`relay_pull_diagnostic` 0 tree-wide" | **8 occurrences in 5 RECORDS files**, 0 in source/tests/scripts — every one planted by ENG-0193's own filing |
| "the receive half is mute" | 43 sites / 39 names; the **transport boundary** is the mute part |
| "`mailbox_hash8` is the precedent" | **0 occurrences tree-wide**; the function is `route_token_hash8`, the field is `mailbox_hash` |
| "loopback only for all runs" | **out of bounds by policy**: `HttpRelayTarget` has exactly two variants, and `qsl_server` is a `[dev-dependencies]` pin reachable only from Rust test code |

⛳ **A sixth, inherited:** NA-0743's *"first green in recorded history"* measures **FALSE** over all
215 recorded runs (211 failure / 4 success) — **the true statement is stronger: the first green in
189 DAYS, ending 201 CONSECUTIVE FAILURES.**

⚠ **THE FIELD SPELLINGS ARE LOAD-BEARING.** `should_redact_value` (`qsc/src/output/mod.rs:316-334`)
blanks any key **containing `token`** — the clause is at `:325`. That is why the push side publishes
`route_header_present` and not `route_token_present`, and why `recv_start` publishes `mailbox_hash`.
A successor "improving" either name to say `token` silently deletes the field.

## 3. THE SR-15 COLD READ — FOUR BLOCKERS, EVERY ONE THIS SEAT'S

- **B1** — six of eight emission-site anchors named the **wrong construct**, 3 of 13 exit rows
  correct, and **`A3`/`A4` TRANSPOSED ONTO EACH OTHER**: an implementation seat would have placed a
  network-error diagnostic inside a parse-**SUCCESS** arm. **The counts were all right.**
- **B2** — **`items_count=0` is UNREACHABLE.** `qsl-server` `src/lib.rs:1209-1210` returns **204**
  when `items.is_empty()`, **identical at both revs that matter**, so the OK arm's `items` is
  non-empty **by construction**.
- **B3** — `auth_present` requires vault + file I/O and fits no admissible hunk class.
- **B4** — **neither `.github` artifact existed as bytes** after *"drafted verbatim"* was claimed
  three times: an artifact three chairs confirmed and none held.

Every correction was **re-derived from bytes by the seat before folding** (R337, verify don't adopt)
**and independently by the Director — zero discrepancies from either pass.**

⇒ **THE SHAPE CHANGED, AND ONE CHANGE DISSOLVED FOUR FINDINGS.** In-place emission sites are replaced
by **a WRAPPER** over renamed `*_inner` functions, emitting **exactly one line per call, every
outcome, pre-flight included**. That dissolves the arity defect (M3), the entire anchor layer (B1),
`auth_present` (B3) and the scrutinee question (m1) at once. **R354 §1** rules the one carve-out an
**OUT-PARAMETER** with **three named deltas per inner** — the signature line, one write before the
*unchanged* return, and one inserted line (`resp.status()` is `&self` + `Copy`) — each landing as a
**quoted pair**, and **corrects R353's *"threaded in the inner RETURN"*** because the literal reading
would force ~18 `Err`-site edits and **delete the byte-unchanged evidence E8 exists to give**.

## 4. THE SEALED SET

E1 red-first · E2 suite-identical · **E3 ON/OFF stream identity (REQUIRED)** · **E3b base-build vs
lane-build-with-the-flag-ON — the only arm that closes the loop** · E4 dead-relay diagnosis ·
**E5 re-sealed** on `mailbox_hash` + `status_code=204` over the four non-receive pull callers, the
impossible `items_count=0` clause **STRUCK** · E6 redaction · E7 the remote full flow, with
`relay_pull_diagnostic` count ≥ 1 · E8 region pins · **E9 the `.github` drafts** · **E10 the
invite-code extraction**. Each carries an antecedent branch: *if the act the checkpoint follows did
not happen, the checkpoint is a NON-RESULT and nothing is concluded.*

## 5. THE `.github` ARTIFACTS — DRAFTED AS BYTES, LANDED BY THE OPERATOR

```
  .github/workflows/remote-invite-roundtrip-tests.yml   sha256 fc96b54197b4b9d0…  121 lines
  the main-red-sentinel.yml roster addition (one line)  sha256 0d6bc7f63db42b49…    7 lines
```

⚠ **THEY LAND TOGETHER.** Sentinel coverage comes from `main-red-sentinel.yml`'s own hardcoded
12-name `on.workflow_run.workflows:` roster — **not** from the watched workflow — so without the
roster line the new workflow runs red and announces itself to nobody, which is WF-0074's exact
condition. **E9 executed:** the yaml parses, and the roster line's added name is asserted **equal** to
the workflow's `name:` field against a mutated-name negative control. **No seat writes `.github/**`.**

⚠ `workflow_dispatch` **only** in v1: the cron is added by the operator afterwards from a **measured**
runtime of *this* job. The withdrawn 03:45 guess was n=1 off a `workflow_dispatch` run of a different
suite whose composition had just changed. `concurrency.group: relay-remote` is a shared literal ready
to be extended to the two existing relay workflows — **noted, NOT DONE**.

## 6. THE R-SPACE — `WF-0088` DERIVED FREE, THEN WITHDRAWN

By this lane's own duplication test the content **amends WF-0087**; **no new countable id is minted**
(R353 §9). The finding: **the R-id space has no declaring form at all.** Route A (banked ruling
FILENAME) is blind on **10 of 15** rulings in `R337..R351`; route B (content) is blind on a lane's
own — and **R353 and R354 each name their own id ZERO times in their own content**. ⇒ **the union of
both routes is the instrument**, path-class split across **three** classes, the third being the
**shared mutable pointer that carries no lane name**. Recorded with the verdict-inversion instance,
where a sealed instrument printed an affirmative `TAKEN` about a free id, minutes after its own
banking.

## 7. THE FIRST LIVE MAIN-RED EVENT SINCE THE SENTINEL LANDED

Recorded in D-1382. Every layer fired correctly on first contact: T5p's antecedent guard named its own
broken precondition rather than reporting a false defect; the classification was made **before** any
re-run; the sentinel opened two issues and **closed neither** (R318.1); the operator closed both with
the classification cited. ⚠ The episode was described as the sentinel reacting *"in 23 minutes"*;
**measured, the two issues are 24 SECONDS apart and the whole red episode is 38 SECONDS.** Corrected
in the open — a claim in the program's favour deserves the same measurement as one against it.

## 8. CLAIM BOUNDARY

**No product source, no test, no script, no `.github/**` file written by any seat, no dependency, no
lock.** No test weakened, skipped or deleted. **No standing rule minted.** No fenced ruling edited.
**No sealed artifact edited.** **ENG-0193 is NOT repaired. WF-0086's gate is NOT built.** ENG-0142's
remainder, ENG-0194, ENG-0196's disposition, ENG-0197 and ENG-0198 all stay **OPEN**.
The operator merges; the seat does not.
