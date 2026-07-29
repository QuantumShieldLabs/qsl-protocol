# NA-0685 — TESTPLAN / INSTRUMENT RECORD: tailnet-address sanitization (D620)

Goals: G4 (primary), supports G1, G5

**Result class:** `TAILNET_ADDRESS_SANITIZATION_PASS`.
**What this plan records:** the instrument that measured the lane, every expectation written
**before** the run it governed, and the controls that make a green result mean something.

⚠ **The instrument itself is operator-side and is NOT in this tree.** It names the private
addresses, and this document does not. That is the rule the predecessor lane had enforced on it
by an instrument rather than by discipline: **redaction records name fields, never values.**

---

## A. The sweep instrument

Successor to NA-0684's, same method, different needle class. **Tracked files only**, over the
two private addresses, across all four repositories and the org profile repo.

It inherits five properties, each of which exists because of a defect a previous lane paid for:

1. **It counts OCCURRENCES, not lines.**
2. **Every classification is keyed on PATH + CONTENT NEEDLE, never a line number.**
3. **It ASSERTS that the classes sum to the raw count** — a lost hit crashes rather than
   miscounting.
4. **It prints what it EXAMINED** — files and lines per repo — so a clean result cannot be
   confused with a no-op. **A zero over an examined tree is evidence; silence is not.**
5. **It prints every LEAVE.** An exception you cannot see is not an exception.

### A.1 One property is NEW, and it exists because this lane substitutes two things

**Two separate needles, not one structural class pattern, and the per-address split is asserted
to sum to the raw count independently of the A/B/C split.** The lane substitutes a *different*
placeholder per address, so the instrument must be able to tell them apart — and the split is
what would catch a correct total reached with the wrong mix. It did real work: the predicted
total was derived by subtraction from the predecessor's archived census, and **the per-address
split confirmed the derivation rather than merely the sum.**

⚠ **The structural `100.64/10` class is deliberately NOT used here.** It belongs to ENG-0089's
scanner. This lane sweeps the two known addresses so that **every hit is attributable to one of
them**; a structural class would have made the census a different measurement than the fix.

### A.2 A per-file C baseline against an EMPTY delta table

NA-0684 needed a per-file baseline because a ruled class reached into its record class and made
the total reachable by removing the wrong occurrences. **Nothing outranks the property here**, so
this lane's expected-delta table is **empty** — which is a *stronger* check, not a weaker one:
**any** C movement fails the gate, not merely an unnamed one. Baseline: 8 files at base.

---

## B. THE EXPECTATIONS, EACH WRITTEN BEFORE THE RUN IT GOVERNED

Three statements, operator-side, none rewritten — each supersedes the last in place:

| # | when | what it fixed in advance |
|---|---|---|
| 0 | before the instrument existed in runnable form | the raw total, **derived** as *(predecessor's census) − (its one ruled removal)*, with the per-address split, the file count, class A **empty**, and the personal-name census **zero** |
| 1 | after the census, before the RED control | the B/C split as measured, with **the miss reported** rather than absorbed |
| 2 | after the F1 ruling, before the first edit | the ruled addition as a separate named figure |

### B.1 ⚠ The prediction that missed, and why that is the method working

    PREDICTED   raw 39 = A 0 + B 14 + C 25     GATE 14
    MEASURED    raw 39 = A 0 + B 16 + C 23     GATE 16

**+2 on B, −2 on C.** The 14 came from the predecessor directive's own words — "**about 14** of
the 40 are B-shaped" — an estimate made while measuring a different class, naming three files
without counting their occurrences.

⚠ **METHOD NOTE: a figure inherited from a prior lane's parenthetical is an ESTIMATE, and
restating it as a POINT prediction is what made the miss visible.** A range would have hidden it.
**Everything else predicted held exactly**, including the derived total and the per-address split.

---

## C. THE CONTROLS

### C.1 RED at base, twice

    run 1  GATE FAIL: A+B=16 ruled_additions=0 personal_name=0 c_delta_ok=True   exit 1
    run 2  GATE FAIL: A+B=16 ruled_additions=1 personal_name=0 c_delta_ok=True   exit 1

Run 2 is not a repeat: the F1 ruling added a figure to count, so **the control was re-run against
the restated expectation rather than the original one.**

### C.2 GREEN post-fix

    raw 23 = A 0 + B 0 + C 23    GATE PASS, exit 0
    ruled_additions 0 · personal_name 0 · c_delta empty (C byte-stable)

**39 − 23 = 16 removed, all class B. No other number moved.**

### C.3 The positive control on the personal-name census

The personal-name census returns **0** — the predecessor redacted all seven sites and none came
back. **That zero is only evidence because the same instrument returned 39 hits for the address
needles over the same trees.** A negative result counts only if the instrument could have
returned positive.

### C.4 The repo's own gate, all three modes, each over a NON-EMPTY input

    --mode tree     clean (2305 files, 611517 lines examined)
    --mode staged   clean (4 files, 17 lines examined)
    --mode diff     clean (4 files, 17 lines examined)

⚠ **Diff mode REFUSED A VACUOUS PASS** when it was run before the commit existed:
*"NOTHING EXAMINED (diff) — refusing to report a pass over an empty input"*, exit 2. **The
standing rule against silent skips, enforced by an instrument rather than by discipline** — and a
reminder that the examined-count is part of the result, not decoration.

### C.5 The Option-B check

Every line this lane re-adds was checked for a Tier-2b literal **by the staged gate**, not by
inspection. It passed — the one adjacent risk (a captured block whose neighbouring line carries
an account name) was not a re-added line. **Predicted in advance, then measured.**

---

## D. WHAT THIS PLAN DOES NOT PROVE

- **It does not prove the class cannot return.** The durable fix is a **CGNAT structural class in
  the committed scanner** — ENG-0089's work, for which this census is the second allowlist input.
  Until then the tree is clean because a lane measured it, not because a gate defends it.
- **It does not prove the record class is harmless.** 23 occurrences remain in dated records by
  ruling. They are **printed by the gate in every run**, so they are visible rather than
  forgotten; the mitigation for what is already published is the operator's standing
  registration/topology posture, not scrubbing.
- **It exercises no code.** This lane is text; the scope classifier reports `docs_only`.
