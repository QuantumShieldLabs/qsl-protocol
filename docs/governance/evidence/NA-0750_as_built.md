# NA-0750 — AS BUILT: PIN-BUMP-2, THE DESKTOP MEETS THE `qsl-fp-v1` FINGERPRINT

**Lane:** NA-0750 · **Spine decision:** `D-1392` · **Desktop decision:** `D-0031` (qsl-desktop PR #31)
**Bases:** qsl-protocol `7180fb88` · qsl-desktop `d7e718ac` — both re-derived **bare, unpiped**, at the
**named** `github` remotes and measured **UNMOVED** from the brief's drafting values. Open-PR set
**EMPTY in both repos**, with the instrument proven able to return rows (`--state merged --limit 3`
returned three per repo). ⚠ Both local mirrors measured **STALE** (protocol `241eec97`, desktop
`11e8e17c`) and were **never used as a source of truth**.

## ⚠⚠ PACKET ERRATUM FOR THE SUCCESSOR (ordered at `R365` §2)

**The 2026-08-20 succession packet's §7 lease-unknown flag was STALE AT WRITING.** It carried the
production relay's `PULL_LEASE_SECS` as a `[P] UNKNOWN`. **The fact landed in repo truth on
2026-08-16** — `docs/ops/IMPROVEMENT_LEDGER.md:3776`, NA-0740 / `D-1375` — with `[O]` provenance, the
`qsl-server/src/store.rs:7` corroboration, and an explicit separation of the **default** from the
**running** value. A directive built on that flag ordered a duplicate into repo truth; `R365` §2
struck the act and recorded the order as the Director's own error. **A packet flag is a premise, not
a finding: re-measure it before ordering anything on it.**
⛳ **What remains genuinely open is the OTHER half of NA-0739's flag** — the **AWS relay's rev** is
still unmeasured in repo truth. The only rev inside `ENG-0142`'s bounds, `qsl-server` `37ec8207`, is
the **loopback test relay's**. It needs an operator measurement; this lane did not invent one.

## THE IDS, RE-DERIVED AT THE EDIT PER WF-0068 — AND SWEPT BEFORE THE BRIEF WAS BANKED

| space | declaring form (from the file's own bytes) | decls | max | taken | positive control | negative control | verdict |
|---|---|---|---|---|---|---|---|
| lane | `^### NA-####` in `NEXT_ACTIONS.md` | 798 | `NA-0749` | **`NA-0750`** | `NA-0748`=1, `NA-0749`=1 | `NA-0750` **0 decl / 2 mentions** (classified); `NA-0751` 0/0 | FREE |
| protocol D | **union of four forms** (A 1300 / B 87 / C 7 / D 2) | 1396 | `D-1391` | **`D-1392`** | `D-1390`=1, `D-1391`=1 | `D-1392` **0 decl / 0 mentions**; `D-1393` 0/0 | FREE |
| desktop D | `^- \*\*ID:\*\* D-00NN` | 29 | `D-0030` | **`D-0031`** | `D-0029`=1, `D-0030`=1 | `D-0031` **0 decl / 1 mention** (classified); `D-0032` 0/0 | FREE |
| SR | `^- \*\*SR-##` in `docs/ops/STANDING_RULES.md`, a form appearing in **no other file** | 22 | `SR-22` | **`SR-25`** | `SR-21`=1, `SR-22`=1 | `SR-26`, `SR-27` | see below |
| SR-16 rows | `^\| N \|` in `docs/ops/PREDICTION_LEDGER.md` — 121 rows, 1..121, **no gaps, no duplicates** | 121 | 121 | **122–129** | row 121 = 1 | rows 122/123 = 0 before the edit | FREE |
| ENG / WF / R | — | — | — | **none minted** | — | — | — |

⚠⚠ **`WF-0087` FIRED ON THREE OF THE FOUR CANDIDATES AND WAS CLASSIFIED, NOT ADOPTED.** `NA-0750`'s
two tree mentions are `DECISIONS.md:43630` and `docs/governance/evidence/NA-0749_as_built.md:18`;
desktop `D-0031`'s single mention is that repo's `DECISIONS.md:1818`. **Every one is a predecessor
recording the id as its own negative control**, plus four operator-tree files, all NA-0749's stops
and blocks. **A mention-counting sweep would have refused three of four with a plausible reason
attached.** Only `D-1392` was clean on every route.
⛳ **NA-0748's own file-choice slip was carried as a control:** the SR-16 row needle aimed at
`IMPROVEMENT_LEDGER.md` returns **16 hits, max 5** — visibly the wrong file, which is what makes the
`PREDICTION_LEDGER.md` figure evidence rather than assertion.

### ⚠⚠ THE SR SPACE IS NOT A MAX, AND THE NAIVE ANSWER IS WRONG

`SR-23` is **permanently reserved-and-unminted** (`R310`, in `D-1363`) and `SR-24` is **REFUSED**
(`R305` / `WF-0078`). Both numbers are **disposed**, not free. `SR-25` is the first number carrying no
disposition; `SR-27` is clean on every route in both spaces.
Landing `SR-25` makes `R310`'s sentence *"the constitution ends at SR-22 and that is now a decision,
not a gap"* read false to a later reader, so the landing carries a **correcting append** that
supersedes it **in the open** — `STANDING_RULES.md` §D's own conflict rule — on the authority of the
**OPERATOR'S ORDER of 2026-08-19**, which outranks a Director ruling. `R310`'s fenced text is not
edited. And the operator-area `PROPOSAL_CHAIR_BOUNDARIES_20260813.md`, which declares `SR-25` and
`SR-26` **in this file's exact declaring form** while its own `SR-24` names a *different* rule from
directive 654's, is **not a numbering authority**: its number claims are **retired as numbers**.

## THE R-ID SWEEP — THREE ROUTES, AND TWO OF THEM LIED FIRST

`R365` was derived by the **union of routes**, after both naive instruments returned a wrong answer:
- **Route A (banked ruling FILENAMES)** first returned **R0**, because the needle `\bR(\d{3})\b`
  **cannot match after an underscore** — `_R364_` has no word boundary before `R`. Repaired to
  `(?<![A-Za-z0-9])R(\d{3})(?![0-9])` it returns **max R364**, matching the Director's own note.
- **Route B (`DECISIONS.md` declaring forms)** returns **max R335** across four forms — lower, because
  not every ruling receives a heading in that file. A form-specific needle is narrower than the space.
- **Route C (repo content)** returns **999** naively. Enumerated, the six high values
  (`R613`/`R724`/`R777`/`R809`/`R888`/`R901`/`R952`/`R999`) are **published synthetic negative
  controls planted by predecessors** — already classified in repo truth. **The naive max is wrong.**
⇒ union max **R364**; **`R365`** measured **0 filenames / 0 declaring / 0 repo files**, with `R363`
returning 1 filename and 4 repo files as the positive control, and `R366` clean as the outer control.

## THE SEALS — every arm, and every accepting arm's control

Full detail and verbatim logs: `/srv/qbuild/operator/NA-0750/build_evidence/` (mode 444).

| seal | verdict | measured |
|---|---|---|
| **W0** compile red | **HIT, in two stages** | stage 1 `cargo check --all-targets` rc 101 naming `commands.rs:145`; ⚠ a **NON-RESULT** for site 2 — cargo aborted at the lib error and never compiled the test target (`slice_a_flows` = **0** occurrences in the whole log). Stage 2, after repairing site 1 only, rc 101 naming `tests/slice_a_flows.rs:57` and nothing else |
| **W1** the values | **HIT** | `4cb507ef…5b62ad98` (64 lowercase hex) · `752204175629941029783252236085` (30 ASCII digits) on a **deterministic** fixture — no vault, no keygen, no I/O. Equality on extracted values, never `contains`. **3 source mutations each turned the suite RED**, every restore `cmp`-identical. Harness: 5 `exec` arms all `{"value":true}` |
| **W2** the inversion | **HIT** | moved off `QSCFP-4527910e41bb92b4478d95ad8b42eee0` / `4527-910E-41BB-92B4-V`; **both old values pinned inside the test** so green-in-the-old-form is impossible. `QSCFP` = **0** in `src-tauri/**` and `ui/**`; three residuals named and out of bounds |
| **W3** vault continuity | **HIT 5/5** | old-pin vault unlocks at the new pin (`vault_unlock ok=true state=unlocked`); `vault.qsv` **and** the identity record **byte-identical by sha256 either side**; magic `QSCV02`; `kem_pk` (1184 B) / `sig_pk` (1952 B) byte-identical; fingerprint **MOVES** `QSCFP-04c31dfe…` → `6072e978…a7a2` |
| **W4** suite + harness | **HIT** | `cargo test` rc 0 — 15 targets, **114 passed / 0 failed / 8 ignored**; inventory **118 → 122, ZERO removed**; live gate rc 0; harness **7/7**, the six pre-existing scenarios reproducing NA-0748's baseline **exactly** — 96/20/28/25/52/21 = **242, delta +0** |
| **V1** the lock | **HIT** | +2/−2, only the two `source =` rev lines; **518 → 518** entries, **461 → 461** names, **0 version-changed** on the `(name, version)` **multiset** (a name-keyed dict keeps only the last version and could not have seen a change inside a duplicated name) |

⛳ **The harness's own arms were proven RED-CAPABLE**, not merely accepting: a mutant scenario
demanding `^[0-9]{31}$` ran **rc 1** with exactly **one** FAIL row (`{"value":false}`) and terminal
`result=FAIL`. **An accepting-only control proves nothing.**
⛳ **And the runner carries its own liveness controls**, which fired in every run: `liveness_absent_selector`
(rc 2, *no such element*) and `liveness_wrong_text`.

## ⛳ A STRUCTURAL FACT STRONGER THAN THE CONSTRAINT IT SATISFIES

`R365` §3.2's routing constraint — the voice tier reaches the **combined** identity fingerprint and
nothing else — is **enforced by construction, not by discipline**: `identity_fingerprint_from_pk` and
`hs_sig_fingerprint` measure **ZERO files** in `qsc/src` at `7180fb88`, NA-0749's redesign having
removed them outright. The only public fingerprint-producing functions are
`identity_fingerprint_from_identity` and `identity_voice_form`. **"Unreachable by construction" is a
different and stronger statement than "not done", and the two are not collapsed.**

## ⚠ THIS LANE'S OWN INSTRUMENT DEFECTS, RECORDED RATHER THAN SMOOTHED AWAY

1. **The brief's `"16-char form"` descriptor measures FALSE as a rendered length.** The retired code
   renders as **21 characters** (16 Crockford payload + 4 hyphens + a check character). A needle
   asserting 16 would have returned a false result in either direction; the landed on-screen arm
   asserts a **shape**, `^[0-9A-Z]{4}(-[0-9A-Z]{4}){3}-[0-9A-Z]$`, absent from `document.body.innerText`.
2. **`W3`'s first negative control could not discriminate.** Invoked with `--unlock-passphrase-file`
   (the vault-open source) where `--passphrase-file` (the credential under validation) was required,
   **both the correct and the wrong passphrase returned rc 1** and the control "passed" while proving
   nothing. Repaired and re-run the arms differ. *A control that cannot discriminate is the finding.*
3. **The R-id filename needle was word-boundary-blind** (see above) — the NA-0748 F-2 shape, again.

## THE HYPOTHESISED BLOCKER THAT MEASURED FALSE

Adding the ordered test looked certain to red the live `scripts/ci/test_inventory.sh` gate (118
pinned names, run at `ci.yml:32`) — an edit **outside** the brief's §1 enumeration, the same shape as
NA-0749's shard-manifest contingency. **Read from the gate's own bytes it is ASYMMETRIC**: `ADDED`
prints *"NEW tests (allowed…)"* and only `MISSING` exits 1. On the committed tree it ran **rc 0**,
printing the four new tests as allowed, then `PASS`. **No `scripts/ci` byte was requested or
touched**, and the standing note that a predecessor *"had to update it"* describes a **convenience
re-pin, not a gate requirement**. ⇒ *a premise re-measured from the instrument's own bytes is what
kept the enumeration intact.*

## ⚠⚠ A SEVENTH DEFECT, AND IT IS THIS SEAT'S: "THE FULL SUITE" IS NOT "WHAT THE JOB RUNS"

**The first push to qsl-desktop #31 went RED on `cargo fmt --all -- --check`.** CI's `rust` job runs
**FOUR** gating steps — `fmt`, `test`, the test-inventory gate, `clippy --all-targets -- -D warnings`
— enumerated from `.github/workflows/ci.yml`'s own bytes **after** the failure. **Two of the four had
never been run locally.** SR-05's *"one full suite on the exact committed tree"* was honoured for
step 2 and the gate was run for step 3; steps 1 and 4 were simply not considered.

The defect was real and entirely cosmetic — **two line-length wraps rustfmt wanted in this lane's own
new `#[cfg(test)] mod tests`**. It was diagnosed by reading the **job log's step banner**, not the
check name: a `rust` FAILURE reads like *"the suite failed"*, and the suite was green. Fixed with
`cargo fmt --all`, **confined to the file rustfmt named** (+7/−2 in `src-tauri/src/commands.rs`, no
other file moved), after which all four steps ran green in CI's own order and the suite was
**unchanged at 114 passed / 0 failed / 8 ignored**.

⇒ **Enumerate a job's steps from the workflow's own bytes and run all of them.** The check that would
have caught this costs one second and was skipped because *"the suite is green"* felt like
completion. Recorded as SR-16 row **130**. Same family as *a step's NAME is not its failure reason*.

## CLAIM BOUNDARY

One machine, the **build box**. The GUI harness proves the on-screen value's **shape**, never its
**legibility** — `textContent` reads identically whether the element is clipped or not, and the
rendered voice form is nearly twice the retired code's length. `ui/main.js:300 fitCode()` shrinks
17px→11px and then wraps, and the two tests covering it assert **the mechanism's presence, not that
any given value fits**. No CSS or HTML byte is changed. `ENG-0209` stays **OPEN** and the key-length
class is **NOT** closed. `ENG-0142` is **untouched** by this lane. Nothing is merged by the seat.
