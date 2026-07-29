# NA-0684 — TESTPLAN / INSTRUMENT RECORD: infra-hostname sanitization (D619)

Goals: G4 (primary), supports G1, G5

**Result class:** `INFRA_HOSTNAME_SANITIZATION_PASS`.
**What this plan records:** the instrument that measured the lane, every expectation written
**before** the run it governed, and the controls that make a green result mean something.

⚠ **The instrument itself is operator-side and is NOT in this tree.** It names the retired
literals, and those classes fire on **added lines** — a committed copy could not be committed.
This plan describes it in classes and counts, which is also the rule the lane adopted:
**redaction records name fields, never values.**

---

## A. The sweep instrument

Case-insensitive, **tracked files only**, over the two retired public names (which share a
domain, so one token cannot miss either) and the retired-rig token in every observed form.

Five properties, each of which exists because of a defect a previous lane paid for:

1. **It counts OCCURRENCES, not lines.** A line-level hand count under-reported the record
   class by 124 (predicted ≥ 677, measured 801) because history lines carry several
   occurrences each.
2. **Every classification is keyed on PATH + CONTENT NEEDLE, never a line number.** NA-0683 was
   bitten twice by line-number keys; this lane's inherited spec line then **moved** between the
   ruling and the execution, and the content key absorbed it at no cost.
3. **It ASSERTS that the classes sum to the raw count.** A lost hit **crashes** rather than
   miscounting.
4. **It prints what it EXAMINED** — files and lines per repo — so a clean result cannot be
   confused with a no-op. A zero over an examined tree is evidence; silence is not.
5. **It prints every LEAVE.** An exception you cannot see is not an exception.

### A.1 Two figures live OUTSIDE the A/B/C sum, by design

Two ruled edits touch literals **no swept pattern matches**: a tailnet address in the ruled
block, and the remote account name on the lines the lane re-adds. Folding them into the gate
number would have made the arithmetic incomparable with the base; leaving them out entirely
would have meant **an edit no number covers — which a green gate would hide**. So each is a
**separate named figure**, printed every run and required to be zero.

### A.2 A per-file baseline, because C had to move

The personal-identity ruling removes six occurrences **from the record class**. `C = 796` is
therefore expected — **and it is reachable by removing the wrong six.** The instrument records
a **per-file C baseline (91 files)** at base and compares the change set against a **named
table**. **An unnamed mover fails the gate even when the total is right.**

## B. Expectations, written before each run

### B.1 Base (PHASE 0), reproduced in the lane seats at the promoted head

    raw 833 = A 3 + B 28 + C 802
    GATE FAIL: A+B=31 ruled_additions=1 personal_name=7      exit 1

### B.2 The RED control — run FIRST, unpiped, before any edit

Ran red at the pre-counted figure. **The gate that can only ever go green is not a gate**; this
one was shown red at a number written down beforehand.

### B.3 Restated FOUR times as the rulings moved scope

| restatement | what moved it |
|---|---|
| 1 — before the census | the hand count, later corrected by the instrument |
| 2 — after seven flags | one occurrence reclassified; one ruled edit added outside the patterns |
| 3 — after the personal-name and tailnet rulings | six record-class occurrences entered scope; C stopped being a constant |
| 4 — before the commit | five account-name edits added after the gate refused the first attempt |

**Each was written before the edit it governed.** An expectation adjusted afterwards is a
result wearing an expectation's clothes.

### B.4 Post-fix, and again post-merge

    raw 796 = A 0 + B 0 + C 796
    GATE PASS: A+B=0 ruled_additions=0 personal_name=0 c_delta=as ruled
    C delta: exactly 5 named files / 6 occurrences; every other C file 0

Re-measured against all three **merged mains** and identical — including that the lane's own
decision text added **zero** occurrences, which is the §3e artifact rule holding under
measurement rather than by intention.

## C. The fail-fast controls — behaviour, verified by running it

For each of the three scripts, **both** runs, expectations written first:

| run | expected | observed |
|---|---|---|
| variable **unset** | one line naming the variable, on **stderr**; **exit 2**; **no output directory created** | as expected, all three |
| variable set to a **dummy** | crosses the guard and **fails for a different reason** | as expected, all three |

⚠ **The second run is the one that carries the proof.** A guard that is merely *absent* also
lets the dummy run proceed; what distinguishes "crossed" from "not there" is that the failure
**changes character** — the two-party scripts print their parameters and reach the build step,
and the third dies at a pre-existing hard-coded path.

⚠ **That third script is also why the guard's PLACEMENT is a requirement rather than a taste:**
it dies at that path, so a guard below it **could never be reached** by the unset-variable
test. The guard sits above every side effect.

## D. The closing measurement is the gate that refused the lane

The repository's own `infra_literal_scan.py` **refused the first commit attempt** — five lines
the lane re-added carry a class that fires on added lines. After the ruled fix, the **same
instrument in the same mode** reads clean, exit 0. **A gate reversing its own verdict is worth
more than a gate that was never asked.**

## E. What this plan does NOT prove

- **It does not prove the names are gone from the world.** They remain in git history by
  ruling; the mitigation is the operator's **registration hold**, not scrubbing.
- **It does not prove the committed scanner would catch a regression.** It would not: the
  retired public names are in **no digest list**, and the rig class fires **only on added
  lines**. The lane's own instrument is the only thing that measures this class today — which
  is precisely the gap ENG-0089 carries.
- **It does not prove the record class is free of sensitive content.** It proves the *swept*
  classes are. The personal-name check found six sites inside the record class that the
  rig-token classification could not see; **the tailnet class is 40 occurrences and is
  deferred**, disclosed in D-1323 with its exposure stated as topology rather than a public
  route.
- **It does not exercise the demo scripts end-to-end.** The dummy run proves the guard is
  crossed, not that the demo completes against a real relay.
