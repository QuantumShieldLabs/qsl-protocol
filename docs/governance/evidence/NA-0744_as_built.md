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

---

# PART 2 — AS BUILT: THE IMPLEMENTATION (D-1383)

**Base:** main `0d75a6a25a227154e61f167a306c50a0cbb89150`, re-derived by URL, bare and unpiped,
against the NAMED GitHub remote (`git ls-remote … refs/heads/main`, rc 0). Open-PR set **MEASURED
EMPTY** with a positive control. Directive v2.1 sha256
`3126fbe80631195e99cacd9fee2cf9ba8a0e46b36541536e551cf6b850510953`, **595 lines / 38054 bytes**,
verified through the pointer chain LATEST.md → STOP 008 (`248c47d76a83b748…`) → v2.1.

⚠ **TWO FIGURES IN THE KICKOFF MEASURED FALSE AND NEITHER IS A DIFFERENT ARTIFACT.** It said the
directive is *"596 lines"* — it is **595**, newline-terminated; STOP 008 §4.1 attests the document by
**sha + 38054 bytes**, both of which match, and the line figure has no backing in the record. It said
main is *"three merges past `d484c065`"* — measured, **two** merges (`241eec97`, `0d75a6a2`) plus
three non-merge commits; the kickoff's own parenthetical (*"#1766 … in two commits"*) is the
corrective. The head sha matched exactly, which is what is dispositive.

## 9. THE EDIT SET AS BUILT

| # | path | change |
|---|---|---|
| 1 | `qsl/qsl-client/qsc/src/transport/mod.rs` | the wrapper block + three named deltas per inner (+460 / −3) |
| 2 | `scripts/demo/qsc_remote_invite_roundtrip_smoke.sh` | NEW, 747 lines |
| 3 | `scripts/demo/qsc_remote_handshake_smoke.sh` | EXACTLY ONE code line: `unset QSC_RELAY_PULL_DIAGNOSTIC`, landed as a quoted pair (+8 / −0, of which 7 are the comment) |
| 4 | `qsl/qsl-client/qsc/tests/relay_pull_diagnostics.rs` | NEW, 9 tests · `…/secret_material_diagnostic_boundary.rs` the E6 sibling (+87 / −0) |
| 4b | `scripts/ci/QSC_SHARD_MANIFEST.txt` | **the row a new test binary REQUIRES** (+1 / −0) — see §14 |
| 5 | records + evidence | this document, `git add -f`, confirmed staged |

⚠ Sizes read with `git diff --numstat`, which reports insertions and deletions **separately**;
`--stat`'s single number is their **SUM**.

## 10. E8 — THE REGION PINS, DISCHARGED BY CLASSIFICATION RATHER THAN BY COUNTING

A count of "three hunks" is satisfied by three WRONG hunks. Every changed line in each inner is
matched to a NAMED delta and any unmatched line is a STOP.

| inner | changed lines | Δ1 signature | Δ2 send-error arm | Δ3 status capture | unclassified |
|---|---|---|---|---|---|
| `relay_inbox_pull_mode_inner` | 12 | 3/3 | 8/8 | 1/1 | **0** |
| `relay_inbox_ack_inner` | 9 | 3/3 | 5/5 | 1/1 | **0** |

**BYTE-UNCHANGED, before and after, by the same instrument:**
`receive_pull_rounds` `eda9d836f59b6895` / 47376 B — **the receive loop, unchanged ENTIRELY** ·
`producer_ack` `40bd87c9a2796765` / 266 B · `relay_inbox_push_inner` `34c4435ac3ae56dc` / 4607 B.

**NEGATIVE CONTROLS, both firing.** (i) One mutated byte inside `relay_inbox_pull_mode` changes
exactly that region's sha and no other. (ii) A planted FOURTH change inside the pull inner is
reported `*** UNCLASSIFIED (STOP)`, rc 1, while the three named deltas still classify.

⚠⚠ **THE INHERITED PINS WERE 1-OF-3 EXACT AND 2 OFF BY ONE IN OPPOSITE DIRECTIONS, ON A FILE THAT
NEVER MOVED.** `transport/mod.rs` was verified byte-identical since `d484c065`, yet
`relay_inbox_ack` measured **3134-3176 / 1626 B** (not 3134-3175 / 1624 B) and `producer_ack`
**3202-3211 / 266 B** (not 3202-3212 / 267 B), while `relay_inbox_pull_mode` reproduced exactly.
⇒ *a pin is instrument-scoped as well as base-scoped.* E8 therefore used **one** instrument on
**both** sides.

## 11. E1 — RED FIRST, SCORED AGAINST A PREDICTION SEALED BEFORE THE RUN

Expectation banked 444 as `E1_SEALED_EXPECTATION.md`
(`d749de81290ee4f335bb0e5fa2ece4d5cb80b749db4d0524e330cf1679b2e36f`) **before** the tests ran.
**PREDICTED 8 FAIL / 2 PASS · MEASURED 8 FAIL / 2 PASS · 10 of 10 correct.**

Antecedent satisfied: **the tree COMPILED** (0 compile errors; the only `error:` line is cargo's
`test failed`). Every failure is **BY NAME**: 6 × `no relay_pull_diagnostic emitted (ENG-0193 is not
repaired)`, 1 × `no op=pull diagnostic emitted (…)`, 1 × `the ON run emitted no pull diagnostic, so
E3 is VACUOUS`.

⚠ **THE TWO PASSES ARE CLASSIFIED, NOT HOLES**, and were classified in the seal beforehand: one
asserts an **ABSENCE** (vacuous while nothing is emitted at all), one scans a **SYNTHETIC** line to
pin the FIELD-NAME CONTRACT, which no emission test can pin.

## 12. THE SEALED SET — VERDICTS WITH MEASURED VALUES

- **E2 — SUITE IDENTICAL.** BEFORE rc **0** / 134 binaries / **634 passed / 0 failed** (3h 05m 22s).
  AFTER rc **0** / 135 binaries / **644 passed /
  0 failed**. Reconciled **per binary**; see §13.
- **E3 — ON/OFF STREAM IDENTITY. PASS.** Committed as
  `gate_on_and_off_streams_differ_only_by_pull_diagnostic_lines`, and **its comparison is itself
  controlled**: two OFF runs must agree before an ON-vs-OFF verdict is admitted. Antecedent (a
  non-zero diagnostic count) is asserted FIRST — that assertion is what made it red before the
  emission existed.
- **E3b — BASE BUILD vs LANE BUILD, THE FLAG ON IN BOTH. PASS.** Binaries `8c7c8f6f…` (base,
  preserved from a clean tree before any edit) vs `3b8e47f9…` (lane). Determinism control base-vs-base
  IDENTICAL; base **0** diagnostic lines, lane **2** (non-vacuous); normalized streams **byte-identical,
  `c0a4f0510ebeff6e…` on both sides**, normalized by the parent's own `:511-517` id-strip list
  (`id`/`sid`/`channel`/`seq`/`idx`/`msg_idx`/`ck_idx`, verified exact) plus the lane's own lines.
- **E4 — DEAD-RELAY DIAGNOSIS. PASS, after a MISS that is recorded in full (§15).**
  `error_class=network_error diagnostic_class=connection_refused status_class=unknown
  status_code=unknown timeout_phase_class=not_timeout qsc_error=relay_inbox_pull_failed`.
  The NAMED FAILURE is avoided: the line does not carry ONLY the bare code.
- **E5 — THE STRONG FORM. PASS.** On `handshake poll`, a NON-RECEIVE caller, against a 204 relay:
  `op=pull ack_mode=lease max=4 status_class=2xx status_code=204 error_class=unknown
  diagnostic_class=empty_mailbox mailbox_hash=4a3ed38b qsc_error=none`. Antecedent asserted first
  (`status_code=204`). `items_count` **ABSENT** — it is `>= 1` by construction on the 200 arm and
  `items_count=0` is UNREACHABLE, so nothing seals on it. `recv_start` count **0**, so the premise
  holds. ⛳ **The pre-lane A/B is the whole case**: the same command on the base build emits only
  `handshake_recv msg=none ok=true` at **rc 0** ⇒ a poll of the WRONG mailbox was indistinguishable
  from a poll of an EMPTY one, at rc 0, under an `ok=true` marker.
- **E6 — REDACTION. PASS.** 14 fields on a real emitted line, **0 keys containing `token`**, **0
  values rendered `<redacted>`**. The near miss is by construction: `route_token_hash8` as a KEY
  would be blanked by `should_redact_value` (`output/mod.rs:325`); the field is `mailbox_hash`,
  exactly as `recv_start` spells it. Negative control: a synthetic leak is REJECTED for each of four
  forbidden markers.
- **E7 — NOT RUN BY ANY SEAT.** Operator-dispatched after merge, by directive.
- **E9 — executed at assembly** (yaml parse OK, roster name equality with a firing mutation control).
- **E10 — THE INVITE-CODE EXTRACTION. PASS on all four arms**, executed against the SHIPPED bytes
  (the function was extracted from the script file itself, not retyped): bare capture → the code;
  a prepended warning line → the **same** code; two sole-line codes → fails BY NAME; none → fails BY
  NAME.

## 13. E2 — PER-BINARY RECONCILIATION

| | BEFORE | AFTER |
|---|---|---|
| rc | **0** | **0** |
| binaries | 134 | **135** |
| passed | 634 | **644** |
| failed | **0** | **0** |
| ignored | 2 | 2 |
| wall clock | 07:28:39Z → 10:34:01Z (3h05m22s) | 10:44:27Z → 13:50:27Z (3h06m00s) |

**Reconciled per binary against an expectation sealed 444 BEFORE the run**
(`d225666412100aff16710b4192beb62ce9057781a9d1c692cfc6f86977ccb817`):

- binaries **LOST: none**
- binaries **ADDED: exactly one** — `tests/relay_pull_diagnostics.rs`, 9 passed / 0 failed
- rows with **changed counts: exactly one** — `tests/secret_material_diagnostic_boundary.rs`
  **4 → 5** passed (the E6 sibling; it MUST change or the sibling never ran)
- **133 of 133 untouched binaries byte-identical**

**Two negative controls, both firing:** a perturbed EXPECTATION (sealed 8 where 9 is true) → STOP,
rc 1; a perturbed LOG (one binary removed from AFTER) → `binaries LOST`, rc 1.

⚠ **THE FIRST CUT OF THIS INSTRUMENT WAS WRONG AND ITS DATA WAS RIGHT.** It called ANY changed row a
mismatch — which cannot hold for a lane that ADDS a test to an EXISTING file. The sound contract is
not *"nothing changed"* but *"what changed is exactly what was sealed, and nothing else"*.

## 14. GATES EXECUTED, EACH WITH A DISCRIMINATING CONTROL

**SHARD MANIFEST** (`scripts/ci/qsc_shard_check.py`) — pristine **rc 0** (census 134 / manifest 134);
test present, manifest untouched **rc 1**, `MISSING from manifest (present in tree):
tests/relay_pull_diagnostics.rs` (135 / 134); row added **rc 0** (135 / 135). ⚠ **The edit set does
not name the manifest**; it is a dependency of "New tests", and without it CI goes red on a change
that is otherwise correct.

**CI SCOPE CLASS** (`scripts/ci/classify_ci_scope.sh`) — subject **`runtime_and_workflow`**; control A
(same set minus the `scripts/ci` path) **`runtime_critical`**; control B (records only) **`docs_only`**.
Control A perturbs the property itself and therefore survives any base. ⚠ `workflow_security=true`
is reached SOLELY through the shard-manifest row, and it puts this PR on the HEAVY check set with
`public-safety` step 15 executing.

## 15. ⚠⚠ E4's MISS — THE LANE'S MOST USEFUL RESULT, RECORDED AND NOT TUNED

Built exactly as §3.1(d) instructs — *"REUSE only the five `reqwest::Error`-based classifiers"* — the
dead-endpoint arm published **`diagnostic_class=not_timeout` for a REFUSED CONNECTION**.

**Cause, measured.** `*_for_send_error` hands the substring classifiers `err.to_string()`. A
`reqwest` connect failure keeps the operating system's reason in its **`source()` chain**, so the
text those classifiers test for is never present in what they are given. The structured predicate in
the same family (`err.is_connect()`, feeding `error_class`) works, and did — that contrast is the
finding.

**⛳ IT IS INHERITED, NOT INTRODUCED, AND THAT WAS MEASURED RATHER THAN ASSUMED.** The PRE-LANE build,
push half, dead endpoint, gate on, emits the identical wrong value:
`event=relay_push_diagnostic … error_class=network_error diagnostic_class=not_timeout …`.
Nothing caught it because `relay_push_diagnostics.rs` only ever drives a LIVE fixture returning
status codes — it never points the client at a dead port.

**The cure, and its boundary.** The classifier is REUSED as ruled; its **INPUT** is corrected —
`reqwest_error_chain_text` walks `Error::source()` and hands the SAME pure `_from_parts` function the
full chain. **The push half is deliberately NOT repaired** (a behaviour change on a path outside this
lane's bounds) and is filed as **ENG-0199**. **The consequence is recorded rather than hidden: for one
and the same failure the two halves now disagree** — pull `connection_refused`, push `not_timeout`.

⚠ This was not "tuning": no assertion was moved to match an output. The assertion is the directive's
own §3.4 contract, and the implementation was corrected until it met it.

## 16. TWO "REQUIRED GATES" ARE RED AT BASE AND NOTHING RUNS THEM

`scripts/ci/preflight_qsc_impl.sh` advertises three `required_gates`. Measured at `0d75a6a2`:
`cargo fmt -p qsc -- --check` **rc 1, 246 hunks**; `cargo clippy -p qsc --all-targets -- -D warnings`
**rc 101, 29 errors**; `git grep` finds both **only inside that script**, never in `.github/`.
A seat adopting them reports failure on a correct tree. The base-valid instrument is a **DELTA**:
`transport/mod.rs` **13 → 12** fmt hunks (**0 introduced, 1 removed** — the pristine tree already
violated fmt on the very line Δ2 rewrites, so landing Δ2 canonically removed it), and **0 of 29**
clippy errors inside the lane's changed line ranges (earliest changed line **3083**; all errors at
**93–261**).

## 17. AN ENUMERATED COVERAGE GAP

**No in-tree Rust test exercises the ACK wrapper's HTTP 200 / 404 arms.** Reaching them needs a frame
that DURABLY COMMITS; measured against a 200-with-items fixture the receive loop emits
`recv_frame_skipped class=invite_resp … disposition=left_leased` and never acks. Driving it would
require the real in-process `qsl_server` (a `[dev-dependencies]` pin) and a full lease round trip —
NA-0644's 838-line harness — which is outside this lane's edit set. Covered instead by: the shared
pre-flight/send-error code path proven on the pull side, E8's byte pins on the ack's own three
deltas, the E6 sibling's `safe_ack` field contract (including `error_class` ABSENT on the 404
success arm), and **E7**, whose published counts include the `op=ack` lines.

## 18. CLAIM BOUNDARY

**No `.github/**` file written by any seat.** No dependency, no lockfile, no server or wire change.
No test weakened, skipped or deleted. No standing rule minted. No fenced ruling edited. No sealed
artifact edited. No remote run by any seat — **E7 is the operator's**. No re-run-to-green: E1's red
is preserved. ENG-0142's remainder, ENG-0194, ENG-0196's disposition, ENG-0197 and ENG-0198 all stay
**OPEN**. **ENG-0199 is FILED, not fixed.** The operator merges; the seat does not.
