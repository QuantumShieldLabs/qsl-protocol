# NA-0713 — AS BUILT — THE RUSTSEC-2026-0253 UNBLOCK (D-1350, directive D649)

Base spine main **`731b02a8`** (verified against the **named `github` remote**, bare and
unpiped; ⚠ the local mirror `origin` was **stale at `32e572c7`** and was trusted for
nothing). Branch `na0713-rustsec-2026-0253-unblock`. ⚠ Main moved to **`a54cb50a`**
mid-lane when PR #1724 merged; **main was merged into this branch before any record edit**,
so the record is written against the current governing text rather than a base that no
longer exists.

## 1. WHAT SHIPPED

**12 code paths, closed at formalization, no growth:**

| path | change |
|---|---|
| `Cargo.toml` | one line — `"apps/qsl-tui",` removed from `[workspace] members` |
| `apps/qsl-tui/**` | **DELETED** — 9 files, 889 lines of Rust plus README and manifest |
| `tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs` | **−6 lines, 292 bytes** — the `AllowedUnguardedDh` entry and `PINNED_DH_SITE_COUNTS` row for `apps/qsl-tui/src/demo.rs` |
| `Cargo.lock` | regenerated — **+1 / −459**, 45 packages removed, **zero version changes** |

**Record:** `DECISIONS.md` (+`## D-1350`) · `NEXT_ACTIONS.md` (`### NA-0713`, `STATE:`) ·
`TRACEABILITY.md` · `docs/ops/IMPROVEMENT_LEDGER.md` · this file · the test plan.

## 2. ⚠⚠ THE LANE'S FIRST FINDING: THE AUTHORIZING BRIEF'S INSTRUMENT MEASURED THE RED TREE AS CLEAN

The directive's constraint 3 named **bare `cargo audit`**, and required the advisory be
shown *"present then absent"* with the before-run serving as the red control.

**Measured at `731b02a8`, on the unmodified red tree:** bare `cargo audit` **exits 0**
(`warning: 1 allowed warning found`) while printing RUSTSEC-2026-0253 in full — because
cargo-audit treats `informational = "unsound"` as an *allowed* class. The gate is
`cargo audit --deny warnings` (`public-ci.yml:222` push lane, `:235` PR lane), which
**exits 1** with `error: 1 denied warning found!`.

⇒ ⚠⚠ **A lane obeying its own authorizing document literally would have measured the red
tree as clean and reported a fix it never made** — in a brief whose same constraint 3 says
*"a gate is not trusted until it is proven able to go RED on the class it guards."*

Both runs ship. The bare one is an **exhibit**, not a footnote: `exit 0` printed beside the
advisory's own text is the finding, and a later reader needs to see it rather than be told
about it. **Standing form adopted at R250 §1.2: the instrument is the one the GATE runs,
not the one the brief names.**

## 3. ⚠⚠ THE SECOND FINDING: THE CRATE WAS NEVER NA-0645's

The brief stated `apps/qsl-tui` *"IS STILL A WORKSPACE MEMBER — and the TUI was RETIRED at
NA-0645."* **Measured false.** NA-0645 retired `qsl/qsl-client/qsc/src/tui/` — 18 files,
~10,007 lines, ~18.9k total — and cleaned **qsc's** manifest. `apps/qsl-tui` is a
**different crate**: 889 lines across 6 `.rs` files, a **refimpl** demo depending on
`quantumshield_refimpl`, not on `qsc`.

⚠ **NA-0645's own testplan says so, and had already filed it:**

> The out-of-scope `apps/qsl-tui` demo client still references the 4 ratatui/crossterm
> deps at the WORKSPACE level (the lockfile keeps the packages for it); qsc's own manifest
> is clean. **Hygiene candidate for a later lane.**

⇒ **The residue was FORESEEN, RECORDED AND DEFERRED — not overlooked — and this lane is
the later lane that testplan named.** NA-0645 is the **witness**, not the cause. ⚠ **That
is a stronger warrant for removal than the brief claimed, not a weaker one:** the honest
headline is not *"a retired surface is holding a required gate red"* but *"a second TUI no
retirement lane ever owned, filed as a hygiene candidate 28 days ago, is holding a required
gate red."*

## 4. THE FOUR OPTIONS, ALL MEASURED IN THROWAWAY COPIES BEFORE ANYTHING WAS MUTATED

| | **A1** unlist only | **A2** unlist + delete *(RULED)* | **B** bump | **C** ignore |
|---|---|---|---|---|
| paths | 2 | **12** | 1 (lock only) | 1 |
| `cargo audit --deny warnings` | exit 0 | **exit 0** | exit 0 | exit 0 *locally only* |
| crates scanned | 392 → **347** | 392 → **347** | 392 → **402** ⚠ +10 | 392 → 392 |
| version bumps | none | **none** | ⚠ 3, incl. `bitflags` into qsc's dev graph | none |
| DH scan | passes | fails → passes after de-pin | passes | passes |
| green on its OWN PR | ✅ | ✅ | ✅ | ⛔ **NO** |
| residue | ⚠⚠ **worse than before** | **none in the build graph** | surface + 45-pkg subtree | everything |

- ⚠ **A1 was refused despite being the brief's literal wording and the smaller diff.** It
  reaches the identical audit result, but turns a crate that **compiles and runs 3 passing
  tests** into **889 lines nothing compiles, nothing tests and no gate covers**, while the
  ENG-0034 scan keeps pinning `.dh(` sites inside it. **A1 makes the residue worse than
  the state the brief called the defect.**
- ⚠ **B WORKS, and the lane's written prediction that it would not was WRONG.** Recorded
  rather than quietly dropped: `cargo update -p lru --precise 0.18.2` fails as predicted
  (exit 101 — `ratatui-core 0.1.0` requires `lru ^0.16`), **but `cargo update -p
  ratatui-core` (0.1.0 → 0.1.2) carries `lru` to the patched 0.18.2 with no manifest edit
  at all.** It is refused on **cost, not impossibility**: +10 packages, and
  `bitflags 2.10.0 → 2.13.1` is reached by `proptest` (a qsc dev-dependency) and by
  `rusqlite` via `qsl-server` ⇒ **not confinable to the retired subtree.**
- ⚠⚠ **C is NOT A REMEDY, on mechanism** — see §7 and **WF-0067**.

## 5. ⚠⚠ THE INSTRUMENT THIS LANE'S TRUST RESTS ON IS NOT ITS OWN

ENG-0034's non-contributory-DH anti-regression scan pins per-file `.dh(` site counts, and
its own comment states that **drift in either direction fails**. Measured:

| state | result |
|---|---|
| crate deleted, pin **NOT** retired | ⛔ **exit 101** — `ratchet.rs:3887`, *"the set of `.dh(` call sites changed"* |
| crate deleted, pin retired (−292 bytes) | ✅ `na0628_every_dh_call_site_is_guarded_or_allowlisted` **1 passed**, exit 0 |

⇒ **red capability proven on precisely the change being made, by a gate built earlier for
an unrelated purpose.** That is stronger than any instrument this lane could have written
for itself, and it is the reason the removal is trustworthy rather than merely green.

⚠ **No guarantee was weakened:** every remaining allowlisted site and every remaining count
is byte-untouched. One allowlisted site retired **with the file that held it**, and the
pin's own `reason` string already read *"retirement tracked by ENG-0032"* — **the pin
anticipated this.**

## 6. ⚠ "REGENERATE THE LOCKFILE" IS NOT ONE INSTRUCTION

| command | delta | verdict |
|---|---|---|
| `cargo metadata --format-version 1 --offline` | **+1 / −459**, 45 packages removed, **zero version changes** | ✅ used |
| `cargo generate-lockfile` | **+389 / −1019** — re-resolves the whole graph, silently bumping `reqwest`, `sha3`, `thiserror` | ⛔ **forbidden** |

The single insertion is cargo collapsing `"foldhash 0.1.5"` → `"foldhash"` because the
second `foldhash` (0.2.0) left with the ratatui subtree — **a de-disambiguation, not a
bump.** ⚠ **Obeying "regenerate it" with the obvious command would have violated "no other
dependency bumps" in the same act.** `cargo metadata --locked` returns rc 0 afterwards, so
every `--locked` build in CI still resolves.

## 7. ⚠⚠ WHY OPTION C CANNOT WORK — AND WHY THE SAME MECHANISM MAKES A2 PROVE ITSELF

1. `public-ci.yml` triggers on **`pull_request_target`**, and the `advisories` job's
   checkout — named, in the workflow itself, *"Checkout workflow definition ref"* — passes
   **no `ref:`**, so it takes the **base** branch. It then fetches the PR head's lockfile
   via the API into `/tmp/pr-Cargo.lock` and audits **that** — which is only necessary
   *because* the checkout is not the head.
2. `classify_ci_scope.sh` makes `.cargo/audit.toml` **`runtime_critical`** (`.cargo/*` is
   neither a docs path nor a workflow path), so the sanctioned bootstrap noop is
   **ineligible** and the real audit runs.

⇒ **A branch's own ignore is read from main, not from the branch. It cannot turn its own
PR green.** ⚠ **And the same mechanism is exactly why this lane proves itself:** A2 changes
`Cargo.lock`, **which the PR lane reads from the head.** *The gate can see your
dependencies but not your configuration.* Filed as **WF-0067**.

## 8. GATES

| gate | result |
|---|---|
| `cargo audit --deny warnings` | ⭐ **exit 1 → exit 0**, **392 → 347** crates, advisory absent |
| ENG-0034 DH scan | ✅ 1 passed, exit 0 — red control **exit 101** |
| `cargo check --workspace --all-targets --locked` | ✅ exit 0 |
| `cargo metadata --locked` | ✅ rc 0 |
| `qsc_shard_check.py` | ✅ exit 0 — census 131 / manifest 131 / missing 0 / unknown 0; manifest **byte-untouched** |
| full `cargo test --workspace --locked` | baseline **168 sets / 810 passed / 0 failed / 2 ignored / exit 0**; after reconciled BY NAME |

## 9. ⚠ A DISK FAULT, RECORDED SO IT IS NOT MISREAD LATER

The first after-suite run exited **101 with ZERO test result sets**: `rust-lld` died on
**signal 7 (Bus error), core dumped** at the **link** step, with the filesystem at
**100% — 468G total, 444G used, 9.0M available.** Memory was fine (28Gi available).
⇒ **No test ran, so nothing about the change was measured in either direction.**

- **The build directory was DELETED rather than reused.** *Artifacts linked while the disk
  was exhausted cannot be trusted, and a green built on them would not be a green.*
- **`git fsck --no-progress --no-dangling` → exit 0, zero output.** A full disk is exactly
  how a repository corrupts, so the seat's integrity was **measured, not assumed**.
- **Only this lane's own disposables were reclaimed** (18G: four throwaway option-measurement
  copies whose results were already banked, a temp bare clone, and the build dir).
  ⚠ **No other lane's artifacts were touched, and that stays correct even though the box
  remained tight afterwards** — reclaiming another lane's data is an operator act.
- ⚠ **Rig-level finding, reported not acted on:** `/srv/qbuild/work` **190G**,
  `/srv/qbuild/cache` **170G**, `/srv/qbuild/tmp` **29G**, dominated by build directories
  of **closed, merged** lanes. **This blocks every lane's full suite, not just this one.**
- ⚠ **The harness's completion signal reported *"exit code 0"* for that run** — the shell
  wrapper's status, **not the gate's 101**, which was recovered from the deliberately
  captured exit. **WF-0062's fifth instance; not ours.** *The instrument that settles a run
  is the run, never the light.*

## 10. ⚠ THE COUNTER WAS RULED TWICE, AND THE LAG IS DELIBERATE

R250's FLAG-2 chose `D-1350` with `HIGHEST_D` left at **1347**, so as not to touch another
lane's branch. R252 §3.1 confirmed it. ⚠ **Then PR #1724 merged (`a54cb50a`), taking main
to `HIGHEST_D=1349` — which made 1347 a TWO-STEP REGRESSION, the exact failure that ruling
existed to prevent.** R254 §1 ruled that **the ruling's INTENT governs over its VALUE**:

    STATE: READY=NA-0713 | HIGHEST_NA=0713 | HIGHEST_D=1350

⚠ **`D-1350` is gap-free**: #1724 landed in **1349**, exactly the slot R252 §3 reserved for
whichever PR had to renumber. ⚠ **Stopping to ask rather than substituting a number was
itself the correct act** — R252 had named 1347 explicitly, and a premise measuring false is
a stop, not a licence.

**All four ids were re-derived at the edit against main AND every open PR**, per R252 §4.2:
`D-1350` · `ENG-0179` · `WF-0065` · `NA-0713`. ⚠ The discipline fired twice — once when
#1724 took the Director-confirmed `WF-0064` **within the hour**, and once when main moved
under the `HIGHEST_D` value. **A reservation is a measurement with a timestamp, never a
claim.**

## 11. ⚠ AND ONE THING THIS LANE DID NOT DO, WHICH THE OPERATOR MUST SEE

**PR #1723 would currently regress the record.** main is
`READY=NA-0712 | HIGHEST_NA=0712 | HIGHEST_D=1349`; #1723 sets
`NA-0711 | 0711 | 1348`. It has not re-derived, because it has been **`BLOCKED` behind the
very `advisories` gate this lane exists to clear**, so #1724 overtook it. ⚠ **Not this
lane's to fix — its branch was never touched.** It needs the same re-derivation #1724
performed. **This is WF-0068's fourth instance, and the sharpest: the axis is not
concurrency but TIME IN THE QUEUE — the longer a blocked PR waits, the further its counters
fall behind, and nothing tells it.**

## 12. FILED, NOT FIXED

**ENG-0179** — the six-artifact `apps/qsl-tui` demo surface (4 demo scripts, the runbook,
the metadata demo doc), left whole because *deleting the scripts while orphaning their
runbook would be a half-retirement — the shape that produced this lane in the first place*.
**WF-0065** — a gate's KNOWN LIMITATION comment outliving the limitation (and thereby
*understating* its own coverage). **WF-0066** — DOC-OPS-006 §2's directive-counter source
ten behind, the third time that document has been wrong about its own counters.
**WF-0067** — a branch's `audit.toml` is invisible to its own gate. **WF-0068** — the id
derivation cannot see an unmerged claim.
**ENG-0032 RESTATED, NOT CLOSED** — arm (a) closes by construction; **arm (b) measured
still open** (`cargo clippy -p qshield-cli --all-targets --locked -- -D warnings` → exit
101). ⚠ And ENG-0032's own recommended shape said *"no refimpl/qsc change"*, which this
lane necessarily contradicted — **two ledger entries disagreed about who pays for the
removal.** **ENG-0034** and **ENG-0090** annotated; both stay closed.

## 13. NOT CLAIMED

That the tree is free of other advisories · that the advisory database will not name
another crate tomorrow · that any lane other than #1723 is unblocked by this gate · that
the demo-surface residue is removed (**ENG-0179**) · that ENG-0032 is closed · that
PR #1723 will merge — **that is the operator's act, and it needs its own counter
re-derivation first.**
