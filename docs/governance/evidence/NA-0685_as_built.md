# NA-0685 — AS-BUILT: tailnet-address sanitization (D620)

Goals: G4 (primary), supports G1, G5

**Result class:** `TAILNET_ADDRESS_SANITIZATION_PASS`
**Directive:** `QSL-DIR-2026-07-29-620_tailnet_address_sanitization.md`, sha256
`9323e7e7b1a7554be668e30ff19df666697d45d569dc6f6ff48c6a029389e0f9`, 239 lines.
**Ruling + closeout:** spine **D-1324**.
**Testplan (the instrument):** `tests/NA-0685_tailnet_sanitization_testplan.md`.
**Predecessor:** NA-0684 (D-1322 / D-1323) — this lane is the successor its handoff-out names.

⚠ **This document names no private address.** Classes and counts only — the rule an instrument
enforced on the predecessor lane twice: **redaction records name fields, never values.**

---

## 0. What this lane was

The predecessor lane sanitized two retired **public** hostnames and, while doing so, measured a
second class it did not own: **private tailnet addresses in public reproduction runbooks.** It
disclosed the class, ruled it out of its own scope, and filed it to this lane — **sequenced
before ENG-0089's Tier-1 promotion, because a promoted gate must not go red on published
content.**

**The exposure, stated honestly rather than inflated** (recorded verbatim from the lane intent):

> CGNAT (100.64/10) is not publicly routable. The class reveals tailnet-internal topology, not a
> public route — lower urgency than the DNS names, still operator infrastructure that public
> repos must not teach.

---

## 1. The census, reported before any edit

    TOTAL  raw 39  =  A 0  +  B 16  +  C 23      GATE (A + B) = 16

| repo | examined | raw | A | B | C |
|---|---|---:|---:|---:|---:|
| `qsl-protocol` | 2 297 files / 609 099 lines | 39 | **0** | **16** | 23 |
| `qsl-server` | 80 files / 16 510 lines | 0 | 0 | 0 | 0 |
| `qsl-attachments` | 41 files / 13 792 lines | 0 | 0 | 0 | 0 |
| `qsl-desktop` | 49 files / 17 218 lines | 0 | 0 | 0 | 0 |
| `.github` | 5 files / 217 lines | 0 | 0 | 0 | 0 |

**One repo carried the whole class.** The other four are **zeros over examined trees** — reported
with their file and line counts, because a zero over an unexamined tree is not a measurement.

**Class A — script defaults — is EMPTY, verified rather than assumed.** The lane intent stated
the expectation and required the check. No script defaults a tailnet address anywhere.

---

## 2. What shipped

**16 class-B occurrences → placeholders**, across three cross-host reproduction runbooks (6 / 7 /
3). **One substitution per occurrence; ports, paths, flags, command structure and prose
unchanged.** The diff is 17 lines changed with **no line-count movement in any file**.

**Two placeholders, one per address** — operator-confirmed in advance, because the runbooks use
the two as **a talking pair** (host A and host B of one cross-host reproduction) and a single
token would make the instructions unreproducible.

**Plus one ruled addition:** a placeholder→placeholder edit unifying the site the predecessor
lane had redacted with a generic token. **No literal is involved, so no swept pattern can see
it** — which is exactly why it is carried as a separate named figure (§4).

**23 dated-record occurrences left byte-identical**, in 8 files: the append-only journal (6),
three dated audits (8), two dated testplans (5), the queue archive (2), traceability (2).

---

## 3. ⚠ WHY THE SPLIT IS CLEAN — three pieces of evidence, none of them taste

D-1322's property, cited rather than re-derived: *a line is in scope when, read today, it DIRECTS
TRAFFIC; out of scope when it REPORTS WHAT WAS TRUE.*

**1. The runbook's own author already placeholdered this exact value.** One of the three files
carries the host-A token at three sites, **introduced by the commit that created the document**.
The author treated the relay address as a reader-supplied parameter from day one; the literal
lines in the same file are the same parameter, left literal by inconsistency. *The strongest
classification evidence available is the document arguing with itself — and it argued for B.*

**2. THE RECORD TWINS ALREADY EXIST, IN DIFFERENT FILES, AND THIS LANE LEFT THEM UNTOUCHED.**
The runbook's parameter block and the dated audit's measurement block record the same three
facts. ⚠ **Nothing measured is lost by fixing the runbook, because the measurement lives in the
audit. The record and the runbook are DIFFERENT FILES** — the cleanest expression of the property
this project has produced, and it arrived as a working consequence rather than an argument.

**3. `docs/demo/` was already ruled.** The predecessor classified a sibling in the same directory
— same front matter, same "records the … proof" opening — as class B and edited it.

⚠ **Two C lines LOOK like commands** (a relay-serve invocation in an audit, two ping lines in a
testplan). They are **captures inside dated records**, and the predecessor's F5 ruled exactly this
shape: *commands direct; a frozen capture reports.* **Precedent applied, not re-litigated.**

---

## 4. The one flag, raised before the edit and ruled

The lane intent proposed two new placeholder tokens. **The census found the tree already carried
a token for this class**, minted by the runbook's own author, plus a second generic one left by
the predecessor. Minting the proposed pair would have left one file carrying **two different
tokens for one value** — reintroducing, via the fix, the very hazard the two-placeholder scheme
exists to prevent.

**RULED: adopt the existing pair; unify the predecessor's site. Two conditions.**

**(a) The A/B letter mapping is DERIVED from the runbook's own usage, not inherited.** Three
independent in-tree statements agree. ⚠ **The derivation agreed with the intent's assignment —
reported as a RESULT, not assumed as a premise.** It is the one place the two could have silently
diverged, and only a measurement could tell.

**(b) The unification is carried as a NAMED RULED ADDITION** with the arithmetic restated, per the
predecessor's method: **an edit no number covers is an edit a green gate would hide.** Its target
letter is likewise derived — from the record twin that still carries the value. ⚠ **The C class
is what made the redacted line's meaning recoverable**: mark-don't-rewrite demonstrated as a
working consequence.

> ⚠ **STANDING RULE THIS EARNS: a lane adopts the vocabulary the tree already uses, and derives
> its mapping from that usage.** The predecessor's F2 gave the positive form — *a placeholder is
> added because the ROLE differs, not because the vocabulary is short.* **This is its converse,
> and it is the census that turns "which token?" from taste into a measurement.**

---

## 5. The measurements

    base       raw 39 = A 0 + B 16 + C 23   GATE FAIL: A+B=16 ruled_additions=1 personal_name=0
    post-fix   raw 23 = A 0 + B  0 + C 23   GATE PASS  (0 / 0 / 0, c_delta EMPTY)

**39 − 23 = 16 removed, all class B. No other number moved.**

**RED at base was run twice** — before and after the ruling changed what was counted — so the
control answered the question that was actually asked each time.

⚠ **C was expected to be BYTE-STABLE, and was.** The predecessor needed a per-file baseline
because a ruled class reached into its record class; nothing outranks the property here, so this
lane's expected-delta table is **empty** — a *stronger* check: **any** C movement fails, not
merely an unnamed one.

**The personal-name census returned 0**, with a positive control: the same instrument returned 39
hits for the address needles over the same trees. **A negative result counts only if the
instrument could have returned positive.**

---

## 6. The prediction that missed, reported rather than absorbed

    PREDICTED   raw 39 = A 0 + B 14 + C 25     GATE 14
    MEASURED    raw 39 = A 0 + B 16 + C 23     GATE 16

The 14 came from the predecessor directive's own words — "**about 14** … are B-shaped" — an
estimate made while measuring a different class, naming three files without counting their
occurrences.

⚠ **METHOD NOTE: a figure inherited from a prior lane's parenthetical is an ESTIMATE, and
restating it as a POINT prediction is what made the miss visible.** A range would have hidden it.
Everything else held exactly, including the total derived by subtraction and the per-address split
— the check that would have caught a right total reached with a wrong mix.

---

## 7. The predecessor's prediction, confirmed

The journal carries 6 of the 23 record occurrences, **including the line the predecessor re-added
and disclosed**. It classifies **C and was left**, exactly as D-1323 said it would: *a journal
capture may legitimately classify C, and the durable fix is the scanner class, not the record
edit.*

⚠ **A prediction written by the predecessor and confirmed by the successor is the strongest form
a handoff can take** — the disclosure was a ruling in advance, not an omission discovered later.

---

## 8. Gates

    infra-literal-scan  --mode tree     clean (2305 files, 611517 lines examined)
                        --mode staged   clean (4 files, 17 lines examined)
                        --mode diff     clean (4 files, 17 lines examined)
    preflight_governance  OK, clean tree     classify_ci_scope  docs_only
    git diff --check      clean             trailers            empty

⚠ **Diff mode REFUSED A VACUOUS PASS** when run before the commit existed — *"NOTHING EXAMINED
(diff) — refusing to report a pass over an empty input"*, exit 2. **The standing rule against
silent skips, enforced by an instrument.** Every green above is stated with the size of the input
that produced it.

**The Option-B check passed:** every line this lane re-adds was checked for a Tier-2b literal **by
the staged gate**, not by inspection. The one adjacent risk was predicted in advance and was not
a re-added line.

---

## 9. Limits, stated plainly

- **The tree is clean because a lane measured it, not because a gate defends it.** No committed
  scanner class matches this address family; **ENG-0089 owns the durable fix.**
- **23 record-class occurrences remain by ruling**, printed by the gate in every run.
- **No code was exercised.** `docs_only`.

---

## 10. HANDOFF OUT — the second allowlist input for ENG-0089

This census is the **second allowlist input**, alongside the predecessor's: **23 record-class
occurrences in 8 named files**, to be met as **known exceptions rather than discoveries** when the
CI/tooling lane adds a **CGNAT structural class** to the scanner needles beside the
`host_retired_rig` Tier-1 promotion.

⚠ **The sequencing precondition that finding records is now satisfied: the known instances are
zeroed.** ⚠ **And a vocabulary note for that lane:** the class now speaks with **one pair of
tokens** across the tree, so a lane that must re-add such a line takes the placeholder as part of
the edit, per Option B.
