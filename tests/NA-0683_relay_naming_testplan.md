# NA-0683 — TESTPLAN / INSTRUMENT RECORD: the naming sweep (D618)

Goals: G4 (primary), supports G1

**What this document is:** the instrument this lane was measured with, the expectations
written *before* each run, and the controls that prove the instruments could return positive.
`RELAY_NAMING_PASS` rests on these, not on a green check mark.

---

## A. The sweep instrument

`naming_sweep.py` (archived with the lane). It reads **every tracked file** via `git ls-files`
and classifies **every occurrence** — not every line, because a line can carry two
(`<button data-pane="server" …>Server</button>`).

| class | meaning | action |
|---|---|---|
| `USER-FACING` | text a user reads | **the gate: must be 0** |
| `RULED-LEAVE` | an operator-ruled exception, **printed by name every run** | leave |
| `TEST-NEEDLE` | a literal inside `src-tauri/tests/**` | reviewed by a human |
| `DESIGN-SPEC` | `docs/DESIGN_SPEC*.md` — binding authority | F1 decided it |
| `IDENTIFIER` | allowlisted code identifiers, DOM ids, CSS classes, wire names | leave |
| `COMMENT` | source/doc/HTML/CSS comments | leave |
| `LEAVE-FILE` | `LICENSE`, `NOTICE`, `DECISIONS.md` | leave |

**Three properties make it evidence rather than decoration:**

1. **It asserts its classes sum to the raw hit count.** A lost hit crashes; it cannot quietly
   miscount.
2. **It prints what it examined** — files, lines, raw hits — so a clean result cannot be
   confused with a no-op.
3. **`RULED-LEAVE` entries are printed in full.** An exception you cannot see is not an
   exception; it is a hole.

⚠ **COMMENT is tested BEFORE IDENTIFIER on purpose.** Most comments in this tree mention an
identifier, and labelling those `IDENTIFIER` would report code where there is only prose
about code.

### A.1 The instrument was wrong once, and the disagreement was the point

The first run reported **22** user-facing where the hand census said 21. The extra was
`ui/main.js:610`, the pane-key array `["identity", "server", …]` — an identifier whose
identifier-ness comes from **context**, not from a distinctive token. The allowlist now
carries that context **keyed on content, not on a line number**, so it survives edits above
it. The hand census was right; the machine was fixed.

Separately, the hand-written LEAVE list said `LICENSE 490/547`; the real lines are **492/550**
— the wrong pair had been carried across from **qsl-attachments' copy of the AGPL** while
writing the cross-repo report. **Both errors argue the same thing: emit the totals, do not
type them.**

---

## B. Expectations, written before each run

### B.1 Base (PHASE 0)

Derived mechanically before running (`grep -c '^#\[test\]'`): 93 in `tests/` + 5 in the lib.

| check | expected | measured |
|---|---|---|
| `cargo fmt --all -- --check` | EXIT 0, no output | ✅ |
| `cargo test -q` | EXIT 0 · **98 passed / 0 failed / 0 ignored** | **97 / 0 / 1** |
| `cargo clippy --all-targets -q -- -D warnings` | EXIT 0 | ✅ |
| `infra_literal_scan.py --mode tree` | clean, **stating what it examined** | `clean (tree; 48 files, 16974 lines examined)` |

⚠ **The one mismatch was mine and it is recorded rather than smoothed over.** I derived "98
passed" by counting `#[test]` and never checked for `#[ignore]`;
`slice_a_flows.rs:323-324` carries one. **98 test cases was right; the pass/ignore split was
wrong.** The prediction was falsifiable, so the instrument corrected it — which is the entire
point of writing it first.

### B.2 The gate's RED control — run FIRST

```
naming_sweep.py <seat>   ->  EXIT 1
GATE FAIL: 21 user-facing "server" occurrence(s) remain
```

### B.3 Post-fix, restated three times as the tree changed

| # | why the expectation had to be rewritten | total |
|---|---|---|
| 1 | the F4 guard **must spell the strings it forbids**; measured in a **sandbox copy** (lane seat untouched) it adds exactly **24** occurrences — TEST-NEEDLE +14, COMMENT +9, IDENTIFIER +1, USER-FACING +0 | 197 |
| 2 | the guard's own fix (§D) added one more comment occurrence | 222 |
| 3 | the 14th F1 line could not be committed (as-built §3) | **223** |

…plus ***d* = 24**, the occurrences inside the new D-0021 entry, counted from its written text
before the run.

**Final, matched exactly:** total **223** · `USER-FACING` **0** · `RULED-LEAVE` **1** ·
`TEST-NEEDLE` **24** · `DESIGN-SPEC` **6** · `IDENTIFIER` **62** · `COMMENT` **52** ·
`LEAVE-FILE` **78**. Sum check: 0+1+24+6+62+52+78 = **223** ✓

⚠ **The lesson worth keeping: when a gate's subject includes the lane's own paperwork, the
arithmetic must be recomputed at EVERY added artifact, not once at the start.** An expectation
rewritten whenever the tree changes stays an expectation; one written once becomes a story
about the past.

### B.4 Suite, N stated first

`relay_naming.rs` carries **N = 5** `#[test]` functions → **97 + 5 = 102 passed / 0 failed /
1 ignored** over 11 targets. Measured: exactly that, locally **and in CI's own log**.

---

## C. The anchor check

37 `file:line → expected substring` anchors covering every line the lane intended to touch,
verified before any edit: **37 checked, 0 drift.**

⚠ The first run reported one "drift" at `AppendixF:421` — **the manifest was wrong, not the
tree**: I had transcribed the *continuation* half of a wrapped sentence. Fixed in the
manifest; the repo was correct all along.

---

## D. F4's binding red-capability control

```
reintroduce "No server configured." into ui/index.html
  -> cargo test --test relay_naming   EXIT 101
     FAILED: the_pane_is_named_relay_everywhere_it_is_shown   (positive pin)
     FAILED: the_retired_server_wording_stays_gone            (negative pin)
restore -> cmp byte-identical AND git status clean
  -> cargo test --test relay_naming   EXIT 0, 5 passed
```

**Two independent tests caught the reintroduction.**

⚠ **The guard failed on its FIRST run against the CORRECT tree, and it was right to.** Its
needle `!cmds.contains("server connectivity")` also matched `commands.rs:319`'s **section
comment**. The needle tested the **mechanism** (the word appears in this file) instead of the
**property** (the rendered string says it); it is now the literal
`slice: "B (server connectivity:`.

---

## E. What this plan does NOT prove

- **The `Relay version` row was not exercised live.** It renders only on a successful probe
  and the rig is retired. Covered by `relay_naming.rs` and both mockups; **recorded as
  unverifiable rather than counted as a pass.**
- **The gate proves TEXT, not layout.** Nothing here asserts where the renamed strings appear
  on screen; that is what the operator's flight is for.
- **`USER-FACING == 0` is a claim about the classifier's rules**, not a proof that no user
  can ever see the word. The rules are stated above so they can be argued with.
