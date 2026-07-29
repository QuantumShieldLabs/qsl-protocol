# NA-0684 — AS-BUILT: infra-hostname sanitization (D619)

Goals: G4 (primary), supports G1, G5

**Result class:** `INFRA_HOSTNAME_SANITIZATION_PASS`
**Directive:** `QSL-DIR-2026-07-29-619_infra_hostname_sanitization.md`, sha256
`de0a8472445b0c86479f4f9180423161f1e1b26fe4bb75cf47499d99955b5603`, 583 lines — **amended
three times in place** (D610 precedent; the drafted `63e5a36c…`/386, the seven-flag
`b73e4045…`/488 and the two-findings `a8dab7f1…`/539 are superseded and marked, never
rewritten).
**Merges:** `qsl-protocol` PR **#1677** → `0509aae36177bce8a35299f3d3cdad9e76c01e63` (lane
commits `05abbecb` + `ed6c07e5`, spine **D-1322**) · `qsl-attachments` PR **#42** →
`a71d348ae4613dd97215a75f6c45d9fbf4a9bc9a` (**D-0013**) · `qsl-desktop` PR **#21** →
`f91fc75f142fcf1b70c04a9809e1b9292975eb0d` (**D-0022**).
**Testplan (the instrument):** `tests/NA-0684_hostname_sanitization_testplan.md`.

⚠ **This document names no retired literal, no operator account and no private address.** That
is not decorum: the retired-rig and account classes fire on **added lines**, so an evidence doc
that spelled them **could not be committed**. Classes and counts only.

---

## 0. What this lane was

Two operator-owned public hostnames were **retired from service with their registrations held
indefinitely**. Live instructions and script defaults in the published trees had to stop
pointing at them. **No history is rewritten** — the mitigation for what git already carries is
the registration hold, a standing operator action, recorded as a standing rule in D-1322 so a
future reader finding those names in `DECISIONS.md` sees the ruling working rather than a leak
that was missed.

**The sharp part was three lines of shell.** Three demo scripts defaulted their base URL to the
operator's endpoint, so **a stranger who ran the demo sent their traffic there having chosen
nothing.**

## 1. The census, and the four premises it corrected

Measured read-only over tracked files of four repositories and the org profile repo,
**occurrence-level**, before any edit:

| repo | examined | raw | A script default | B live instruction | C history-leave |
|---|---|---:|---:|---:|---:|
| `qsl-protocol` | 2 295 files / 608 467 lines | 811 | **3** | **23** | 785 |
| `qsl-server` | 80 files / 16 510 lines | 8 | 0 | **0** | 8 |
| `qsl-attachments` | 41 files / 13 764 lines | 13 | 0 | **5** | 8 |
| `qsl-desktop` | 49 files / 17 186 lines | 1 | 0 | **1** | 0 |
| org `.github` | 5 files / 217 lines | 0 | 0 | 0 | 0 |
| **total** | | **833** | **3** | **29** | **801** |

*(F1's later ruling moved one occurrence B → C: 28 / 802.)*

**The premises that did not survive measurement:**

1. **"Three repos" survived only as a RESULT.** A fourth repo carries 8 occurrences — every one
   dated record, so it got **no PR**. Measured clean of live instructions, not assumed clean.
2. **⚠ "Two of the repos have no infra-literal scan" was WRONG, and the correction cut both
   ways.** All four carry `scripts/ci/infra_literal_scan.py`, wire it into a workflow and ship a
   `pre-commit` hook. **But that gate is blind to this lane's subject:** the retired public
   names are in **no digest list** and match **no structural class**, and the retired-rig class
   is **Tier 2b — added lines only**. The method's "gate RED at base" was therefore impossible
   with the repo's own gate, and the lane brought its own instrument.
3. **The inherited spec line had MOVED**, `:239 → :241`, pushed down by the very lane that
   deferred it. Every anchor was keyed on **content**; the move cost nothing.
4. **The class-B work set was 29 occurrences, not the four files the intent named** — 18 kin,
   11 of them in a single test plan.

**A positive control proved the sweep could have found a fourth script:** every `http(s)://`
literal in every tracked shell script was enumerated; the only non-loopback endpoint anywhere
was the operator host in exactly those three files, and the control returned the three known
lines. Every other script is loopback; the two remote smoke scripts take the endpoint as
**required input with no default** — the shape this lane adopted was already the house standard
three files away.

## 2. What shipped

**Class A (3).** Each demo script now requires `QSL_RELAY_BASE_URL` and **fails fast naming
it**, in the idiom two sibling scripts already used — adopted for the property that makes it
right: **the message names the variable and the failure happens before any work**. ⚠ The guard
sits **above every side effect** because one script dies at a hard-coded path; a guard below
that point **could never be reached** by the unset-variable test.

**Class B (28).** Angle-bracket placeholders, command structure unchanged, one substitution per
line — a README, a demo doc, 11 of a test plan's 13 (its frozen golden-run block **reports** and
was left), a live operational authority doc, and a reviewer runbook's captured block. **One
ruled exception to one-substitution-per-line:** `qsl-desktop`'s Appendix F line took **both**
edits.

**Class C (802 → 796).** Edited only where the personal-identity tier reached; **printed by the
gate in every run** so a leave cannot hide inside a green result.

### 2.1 What deliberately did not change — and must not later

Identifiers, ssh aliases, crate/repo names, routes and wire fields. ⚠ **The ssh aliases stay on
a measured finding, not a preference:** one of them names the canonical attachment-service
contract document and appears in that crate's source; the other is the project. **Only the DNS
names were operator infrastructure.** Build-root and home-path literals, and the two remote
account names, remain under D612 Tier 2 — **except on lines this lane re-added** (§4).

## 3. ⚠ The deferred spec line landed, and the two edits were MUTUALLY ENABLING

`qsl-desktop`'s Appendix F line was NA-0683's **fourteenth F1 line**, which that lane could not
commit: it carried an operator-infrastructure literal already on `main`, the class fires on
**added lines only**, and the one-word edit re-added it. NA-0683 **reverted rather than working
around it**, because redacting a hostname inside a naming PR would have pre-empted this lane.

**Here the same line took both edits and the scan read clean.** Removing the literal is what
makes the commit possible — the two edits are not merely adjacent, they are **mutually
enabling**. That is the measured answer to a question NA-0683 could only pose.

## 4. ⚠ Two classes outranked the lane's own property, and one of them overrode a ruling

**The property** (D-1322, verbatim): *a line is in scope when, read today, it **directs
traffic**; out of scope when it **reports what was true**.* **F1 settled that when the
operator's enumeration and the property disagree, the property governs.**

**(a) PERSONAL IDENTITY outranks both mark-don't-rewrite and the property.** A census for the
personal-name token returned **7 sites, not the zero the ruling expected** — one already in
scope and **six inside class C**. They were **not** occurrences the sweep had missed: all seven
carried the rig token and were already counted. What the check found is that **six C-class
occurrences carry a person's name**, a sensitivity the rig-token classification could not see.
All seven were redacted. ⚠ **This is why the lane edited the append-only journal** — two lines,
one substitution each.

**(b) TIER-2b ON ADDED LINES overrode a ruling of this same lane.** Five edits were **refused by
the repository's own gate**: those lines also carry a remote account name, a class that fires
on added lines only. They had read clean on `main` for months and became *added* lines the
moment the lane touched them — while an earlier flag had ruled that account name **stays**.
**Ruled Option B:** placeholder it on **exactly the lines the lane re-adds, nowhere else** —
**not a reversal of D612 Tier 2 but Tier-2b's designed migration semantics**, the tier firing
on added lines *so legacy content grandfathers while every edit ships clean*.

⚠ **The visible artifact was predicted in advance, not discovered:** the authority doc carries
that token on five lines and the lane re-added four, so **the fifth stays, directly beneath a
placeholdered line** — adjacent lines, one redacted and one not.

## 5. The gate, and the arithmetic that had to be restated four times

    base       raw 833 = A 3 + B 28 + C 802
               GATE FAIL: A+B=31 ruled_additions=1 personal_name=7, exit 1
    post-fix   raw 796 = A 0 + B 0 + C 796
               GATE PASS: A+B=0 ruled_additions=0 personal_name=0 c_delta=as ruled
    post-merge identical, re-measured against all three merged mains

**Restated four times — before the census, after the seven flags, after the personal-name and
tailnet rulings, and after Option B — each time BEFORE the edit it governed.** An expectation
rewritten when scope changes stays an expectation; one adjusted afterwards is a result wearing
an expectation's clothes.

⚠ **C MOVED this lane, so the total stopped being evidence on its own.** `796` is reachable by
removing the wrong six. The instrument records a **per-file C baseline (91 files)** and compares
the change set against a **named table**; **an unnamed mover fails the gate even when the total
is right.** It matched exactly: five files, six occurrences.

**Two figures live outside the A/B/C sum, each named and each required to be zero** — the
tailnet literal in the ruled block, and the account name on added lines. **An edit no number
covers is an edit a green gate would hide.**

## 6. Checks and controls

| | |
|---|---|
| RED control | run **first**, at base, unpiped: `GATE FAIL: A+B=31 ruled_additions=1 personal_name=7`, exit 1 |
| closing measurement | **the same gate that refused the first commit attempt** → `--mode staged` **clean**, exit 0 |
| tree scans | `qsl-protocol` clean 2 303 files / 610 936 lines · `qsl-attachments` clean 41 / 13 792 · `qsl-desktop` clean 49 / 17 218 |
| fail-fast | **run, both ways, all three scripts** — unset → named message on **stderr**, **exit 2**, **no output directory created**; dummy → **crosses the guard and fails for a different reason**, which is what proves it was crossed rather than absent |
| `bash -n` | clean on all three scripts · `git diff --check` clean |
| desktop suite | **102 passed / 0 failed / 1 ignored** (12 result lines) — unchanged by the docs edit |
| CI at merge | spine **35 pass / 2 skipping / 0 fail**; satellites **3/3** each |
| identity | three merge commits, **`trailers=[]`** on every one |

## 7. ⚠ The instrument enforced the lane's own rules on the lane's own paperwork — twice

**Both times the gate was right and the paperwork was wrong.** A queue block and, later, a
scope line quoted the **two remote account names** *while listing what must not be touched*,
and the staged scan **refused the commit**, printing its own standing advice: *if the hit is in
a record about a redaction, name the FIELD rather than the value — a redaction record written
naively re-leaks what it redacts.*

**Recorded as a rule: redaction records name FIELDS, never VALUES** — enforced by an instrument
rather than by discipline. It is also why this document reads in classes and counts.

## 8. ⚠ A method correction: a pipeline reported success for a truncated log

A first reading of the desktop suite came from `cargo test --offline | tail -40`. The pipeline
**truncated the log to its last five result lines** (a false "25 passed") **and reported
`tail`'s exit status rather than cargo's**. The real figure — **102 passed / 0 failed / 1
ignored over 12 result lines** — came from an unpiped re-run.

**Rule: suite numbers are read from unpiped runs or from the log file, never through a pipeline
that can truncate output or mask an exit status.** Same family as the standing rule about never
piping the check that gates you.

## 9. Disclosed and deferred: the tailnet class

Found while establishing the ruled block's base count: **40 occurrences / 12 files / two
distinct addresses**, ~14 of them **B-shaped by this lane's own property** (live reproduction
commands in cross-host runbooks). **The whole class is invisible to the committed scanner** —
CGNAT matches no structural pattern.

**Ruled: not this lane.** A **successor micro-lane**, operator-approved, sequenced immediately
after this one and **before ENG-0089's Tier-1 promotion**, because *a promoted gate must not go
red on published content*. ⚠ **The exposure, stated honestly rather than inflated: CGNAT is not
publicly routable — the class reveals tailnet TOPOLOGY, not a public route.** That is why it
can wait one lane where the DNS names could not.

⚠ **One line this lane re-added still carries such an address**, in the journal capture it
edited for the personal-name tier. **Disclosed and deferred:** it trips no gate, it is inside
the successor's census, and **the successor rules its class** — with the stated expectation
that a journal capture may legitimately classify **C (leave)**, the durable fix being a **CGNAT
Tier-2b scanner class** so that future edits to such lines ship clean.

## 10. Observations carried out of the lane

**ENG-0089 gains a sharper question than the one it was filed with** — not *"promote Tier-1 or
not"* but **"what does a lane do when it must re-add a line it is not allowed to change?"** —
and **Option B is the recorded answer**. This census (the historical proof labels, the 10
tracked paths whose names carry the rig token, and the C class) is the **allowlist input** the
CI/tooling lane must meet as **known exceptions rather than discoveries**.

**A pointer is not an instruction.** Five occurrences name a past proof whose artifacts are
tracked files carrying the same token; renaming the label would break the pointer and renaming
the files is history churn. **Left, and printed in every gate run** — an exception you cannot
see is not an exception.

**Every defect this lane caught was in the apparatus or the paperwork, never in the product's
behaviour:** a truncating pipeline, two redaction records that quoted what they redacted, an
expectation that had to be restated four times, and a gate that refused the lane twice and was
right both times.
