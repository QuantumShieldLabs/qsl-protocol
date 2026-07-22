# NA-0665 — as built

**Lane:** NA-0665 — GUI round 4a, pre-main screen chrome
**Directive:** QSL-DIR-2026-07-22-601 (D601), approved+amended 2026-07-22,
sha256 `2d80a72aed608472296e582298c2b2807296a839e5bc1815cfc298fa34a645c5`, 966 lines
**Decisions:** qsl-desktop **D-0006** (implementation) + spine **D-1291** (this closeout)
**Result class:** `GUI_ROUND4A_PRE_MAIN_CHROME_PASS`
**Repo:** qsl-desktop (SATELLITE) — DESIGN/FRONTEND only

> Presentation and window-sizing behavior only. **No security property is improved by this
> lane and none is claimed.** No crypto, no vault internal, no protocol surface, no core call.

---

## 1. What merged

| | |
|---|---|
| qsl-desktop PR | **#6** |
| merge commit | **`02cc9b96`** — a TRUE merge commit, parents `8db2b2a5` (prior main) + `c11fd6a3` (lane) |
| required check | `rust` **PASS**, 4m43s |
| post-merge push run | **SUCCESS** |
| base at drafting and at execution | `8db2b2a5` — verified EXACTLY, so every directive line number held |
| branch protection (before and after) | `['rust']`, strict `true`, enforce_admins `true` |

Change set: **7 files, +498 / −50.** Every file on the directive's MAY-touch list.

---

## 2. The four parts that landed

### (1) The window is the card

All container chrome — background, border, radius, card padding — removed from **all five**
pre-main screens: wizard 1, wizard 2, unlock, erase, wiped. Content sits directly on
`var(--bg)` with a uniform **28px** screen padding (`--sp-x28`, a new SPACING token).

Two construction constraints, both binding, both honoured:

- **The flex column the `.card` provided is RE-HOMED**, not lost. Deleting the layout that
  the chrome was blamed for was the obvious failure mode; the directive named it explicitly.
- **The strip is ID-SCOPED.** The bare `.ceremony-card` rule survives intact, so the
  **Settings** destroy ceremony (`#pane-vault`) keeps its card — it is not a pre-main screen.
  `#scr-erase .card` (0,1,1,0) outranks `.ceremony-card` (0,0,1,0) on specificity, which is
  what makes one class render two ways. A new test pins that the Settings ceremony survives.

### (2) Per-surface window sizing (F1)

`WindowMode` widens from **three variants to six** — one per pre-main surface plus `Full` —
**on the same single shared path.** `apply_window_mode`, the changed-guard, and the NA-0662
deferred-show sequence are **byte-untouched**. A table change, not an architecture change.

**360px is the READING WIDTH**, the operator's chosen measure, found by hand-resizing the
identity window until the copy composed correctly. The round-3 560/460 widths let body text
run too long.

> **WIDTH AND HEIGHT ARE COUPLED.** At 360 the copy wraps into more lines, so every height
> below is measured **at 360** and **is not valid at any other width.** Changing the width
> invalidates all five heights.

| WindowMode | surface | measured content @360 | + 28px×2 padding | **landed** | min |
|---|---|---|---|---|---|
| `WizardVault` | wizard 1 — Create your vault | 527 | 583 | **360 × 585** | 360 × 200 |
| `WizardIdentity` | wizard 2 — Your identity | 564 | 620 | **360 × 625** | 360 × 200 |
| `Unlock` | unlock | 194 | 250 | **360 × 255** | 360 × 200 |
| `Erase` | erase everything | 217 (form) | **273** | **360 × 275** | 360 × 200 |
| *(erase countdown state)* | *same window* | 197 | 253 | *form governs* | — |
| `Wiped` | vault erased | 161 | 217 | **360 × 220** | 360 × 200 |
| `Full` | main + Settings | — | — | 1024 × 700 | 800 × 600 |

Literals round the measured total **up to the next multiple of 5**, so a sub-pixel difference
cannot clip the last element or trip the card's `overflow-y: auto` scrollbar. Erase is sized
to the **taller** of its two states, since one window serves both without a resize.

**The compact minimum became a single floor (360 × 200)** rather than round-3's
"minimum == initial", which is what keeps the pre-main windows resizable at all.

> **A REAL BUG, CAUGHT IN-LANE BY A NEWLY WRITTEN ASSERTION AND NOT BY REVIEW.** The floor was
> first set to 360 × **280**. The wiped window is **210** tall — *below* that floor — so
> `set_min_size(280)` would have silently overridden `set_size(210)` and the wiped screen
> would never have taken its literal. The failure would have been invisible in code review
> and visible only as "the wiped screen is oddly tall". **The floor must sit at or below the
> shortest window**, and an assertion now enforces exactly that.

### (3) The verification code can no longer clip silently (F4)

The lane intent framed this as the same root cause as the sizing defect. **The drafting
census refuted that and the operator withdrew the framing** — sizing the window does not fix
code clipping.

What was actually wrong, verified from source at `8db2b2a5`:

- `fitCode` (`main.js:215-222`) shrinks 17px → **11px floor**, then stops.
- `.verify-code` is `white-space: nowrap; overflow: hidden`, so **below the floor the code was
  CLIPPED outright — silently, with no ellipsis and no scroll.**
- `fitCode` ran **only at render**. `grep -n "resize" ui/main.js` → **exit 1, zero matches**;
  zero across the entire `ui/` tree. `resizable` is unset in `tauri.conf.json` and therefore
  defaults **true**, so the user can resize — and **the code never re-fitted. Ever.**

Landed: a **debounced `resize` listener** refits **both** call sites, and at the floor the
code **wraps at a group boundary** (`.verify-code.wrapped`) instead of clipping.
**The flagged horizontal-scroll fallback was NOT needed.**

> **ORDERING IS LOAD-BEARING, AND A TRAP WAS SET AND CAUGHT.** The frozen needle in the
> STOP-class `design_round2.rs` slices from the **FIRST** `.verify-code` in the file to the
> next `}` and requires `white-space: nowrap` inside it. Naming `.verify-code.wrapped` in the
> **comment above** the base rule **relocated that slice** — and it still **PASSED, by luck**,
> because the displaced slice happened to run through to the base rule's closing brace anyway.
> Only a newly written test exposed it.
>
> **Generalized hazard, now recorded in the CSS itself: in `ui/style.css`, a pinned selector
> mentioned even in a COMMENT before its rule can silently relocate a frozen needle's slice.**

### (4) The destroy rider (item D)

The Settings destroy-vault ceremony **replaces** its trigger button rather than sitting below
it. **Cancel restores it, and so does any state transition** via `clearCeremonyState` — the
second path being the one a naive implementation misses. Behavior only: the passphrase,
typed-phrase and tokened-core-call gates are **byte-unchanged** and newly pinned by test
(needles at `main.js:586` and `:591`).

---

## 3. The measurement method — this lane's reusable result

Heights were measured **headlessly in WebKit2 4.1 — the same engine tauri uses on Linux —
against the real `ui/index.html`** loaded read-only over `file://`. Nothing was copied and
nothing in the repo was modified. `fitCode`'s shrink/wrap algorithm is **replicated in the
harness**, so the verification code's rendered size is included in the height.

**INDEPENDENT CORROBORATION, which is what makes the table trustworthy:**

| source | wizard-2 height at 360 wide |
|---|---|
| headless harness | **620** |
| operator's independent hand measurement | **621** |
| operator's after-shot, minus the 66px GNOME titlebar | **625 landed → renders correctly** |

All five literals matched the operator's after-shots **to the pixel** once the **constant 66px**
titlebar/frame offset is removed (captured 651/691/321/341/286 against literals
585/625/255/275/220; 388 wide = 360 + frame). A uniform offset across all five is itself
evidence the sizing path does exactly what the table says.

> **AN EARLIER DERIVED (UNMEASURED) TABLE WAS WRONG, AND OPERATOR EYES CAUGHT IT.**
> 560×500 / 560×520 / 460×250 / 460×280 / 460×210 **undershot wizard-2 by 100px and wizard-1
> by 83px, and CLIPPED both**, because narrowing to the reading width makes copy *taller*.
> That derivation was performed **only because a mid-lane amendment forbade the executor from
> rendering**; the conflict with D601/F1's explicit *"render it, read the content height"*
> instruction was **recorded rather than silently resolved**, and the amendment was followed.
> The correction cost one round trip and no rework beyond a table.

Harness: `/srv/qbuild/tmp/NA0665_gui_round4a_20260722T051031Z/measure.py` — re-runnable at any
width, read-only against the live `ui/` tree, **no launch and no screenshots**.

---

## 4. Two operator rulings that changed mid-lane

Recorded here because a reader of the directive alone would otherwise find the shipped result
contradicting an approved flag.

**(a) F2 WAS OVERRIDDEN AT CENSUS REVIEW.** As ruled at readback, F2 stripped the neutral outer
`.card` only and **KEPT** the E.4 red ceremony chrome on the destructive screens. After seeing
the census the operator **revised** it: strip **ALL** chrome from all five pre-main screens
**including the danger border**, with **red TEXT as the sole danger signal**. The build
implements the revision. **Both the original ruling and the revision are recorded.**

**(b) THE ACCEPTANCE VEHICLE WAS AMENDED** to operator-flown review with **no executor
rendering or screenshots** — which is why this lane's after-evidence is the operator's own
five shots rather than an automated capture, and why the F1 measurement instruction had to be
reconciled against it (§3).

---

## 5. Phase-1 census — 5-for-5 CONFIRMED, nothing contradicted

Every anchor re-read from source at `8db2b2a5`; every directive line number landed exactly
where drafted, including the ones easiest to drift: the `.card` flex column at
`style.css:128`, `app.set_menu` at `lib.rs:218`, the destroy needles at `main.js:586`/`:591`.

**FINDING C2 — SETTLED EMPIRICALLY.** Main was built **unchanged** and captured under the
NA-0661 rig (Xvfb 2560×1600, `GDK_BACKEND=x11`, webkit software knobs, 20s settle, `xwininfo`
geometry). **There is NO white menu strip on the pre-main screens at `8db2b2a5`** — the top
16 rows of both compact screens measure **exactly RGB(29,29,31) = `--bg` #1D1D1F**, pixel
exact. **The screenshots that motivated item C predate NA-0662 and are SUPERSEDED**, exactly
as the census inferred and as the approval ruled must be tested.

**The measured dead space that justified the work:**

| screen | geometry | trailing void |
|---|---|---|
| wizard-1 (CompactWizard) | 560 × 660 | **153 px — 23.2% of the window** |
| unlock (CompactGate) | 460 × 420 | **164 px — 39.0% of the window** |

**WHY THE VOID SURVIVED ROUND 3 — recorded because it explains a rule that looked satisfied
and was not.** Appendix E §E.1 **already** forbade "no vertical void". Round 3 satisfied its
**letter** by **stretching the card** to fill the window (`align-items: stretch` +
`.card{width:100%}`) — which kept the void and merely **moved it inside the card**.
**A rule can be satisfiable without fixing the thing it was written to prevent.**
Per-surface sizing is what actually closes it, and [E.1] is amended so the formulation cannot
be satisfied that way again.

**Reachability ceiling, recorded honestly:** only **two of five** pre-main screens are
launch-reachable (`route()`, `main.js:159-170`: `s0 → wizard-1`, `s1/s2 → unlock`). The other
three are input-driven and the host has **no input driver** (no `xdotool`, no passwordless
sudo). D601 §1.3 asked for all four; on this host that was not achievable, the gap was
reported rather than papered over, and the operator **accepted the gap** — the manual flight
covers all five.

---

## 6. Dispositions

**ITEM C — DROPPED, drop CONFIRMED (ENG-0060).** The File/Edit/View/Help bar is a **Tao/GTK
NATIVE widget** attached via `app.set_menu` — outside the DOM, unreachable by any rule in
`ui/style.css`, unfixable by **any** frontend change. The measured ~RGB(202,222,233) is the
**ambient GTK light theme**, not an app choice. **Theming or hiding it is platform-specific
work owned by the future Appearance-pane / dark-frame story, NOT a frontend lane, and no
filing may imply otherwise.**

**ENG-0061 — FILED, ACCEPTED AS-IS, DELIBERATELY NOT FIXED.** The wiped / "Vault erased"
screen ships with **no danger colour at all**, because its heading was **never** red — it is a
plain `<h1>` inheriting `--fg`, unlike erase's `.ceremony-head`. **The F2 override's own
stated rationale ("the headings are already red") is FALSE for that one screen**, and
stripping `.danger-card`'s border removed its only signal. **Ruled ACCEPTED AS-IS:** it is a
calm post-hoc notice, the data is already gone, nothing can be mis-triggered, and the pixels
were approved on sight — the error was in the verbal rationale, not the approval.
**Deferred to round 4c with the Settings-pane pass.**

**DISCHARGED IN PASSING — THAT ITEM ONLY.** The still-owed NA-0662 operator flight's
**compact-mode menu-visibility item** is satisfied by this lane's before-shots, **on
operator-accepted mode-sharing INFERENCE and not on five direct observations**: two of four
compact screens were directly observed, they cover **both** compact mode classes, and menu
attachment is decided **per-MODE not per-surface** (`apply_window_mode` keys solely on the
`menu_visible` bool). **The executor declined to claim this discharge on its own authority
and the operator ruled it.** The **rest of the NA-0662 flight stays OWED, untouched, and
unclaimed.**

**Still owed after this lane:** GUI slice B; the WF-0024 DOC-PROG-004 micro-lane; round 4b
(the rail toggle); the remainder of the NA-0662 operator flight.

---

## 7. Bounds held

```
BYTE-IDENTICAL  src-tauri/src/commands.rs        <- BYTE-ABSENT from the diff
BYTE-IDENTICAL  src-tauri/src/gateway.rs         <- BYTE-ABSENT from the diff
BYTE-IDENTICAL  src-tauri/src/settings.rs
BYTE-IDENTICAL  src-tauri/tests/design_round2.rs     <- STOP-class (F3)
BYTE-IDENTICAL  src-tauri/tests/design_system.rs     <- STOP-class
BYTE-IDENTICAL  Cargo.toml / Cargo.lock / src-tauri/Cargo.toml   <- ZERO dependency motion
BYTE-IDENTICAL  src-tauri/src/{state,paths,markers,main}.rs
BYTE-IDENTICAL  src-tauri/tests/{slice_a_flows,slice_a_rules}.rs
BYTE-IDENTICAL  ui/index.html
```

**`ui/index.html` is byte-identical to base** — the entire change landed in CSS, JS and the
Rust size table, so every frozen markup needle in the round-2 and round-3 suites is trivially
intact and the change is demonstrably presentational.

**NO token VALUE changed.** `--bg: #1D1D1F` and `--bg-raised: #252528` stand. The only token
added is `--sp-x28: 28px`, a **spacing** token. Per **F3**, the reference markup governed
**LAYOUT AND STRUCTURE ONLY, NEVER COLOR**: `qsl-tokens.css` is on a **different palette** than
the build ships and was **context only**, the `#22262c` / `#16181c` values in the lane intent
having been handed to the drafter **in error**. Mockups 02-05 (slice B / lane 2) were
**never opened**.

`tauri.conf.json`: `windows[0]` initial size only — `width: 560→360`, `height: 660→585`.
No other key.

---

## 8. Gates

| gate | command | exit |
|---|---|---|
| build | `cargo build` | **0** |
| fmt | `cargo fmt --all -- --check` | **0** |
| test | `cargo test -q` | **0** — 60 passed, 0 failed, 1 ignored |
| clippy | `cargo clippy -q -- -D warnings` | **0** |
| CI | required check `rust` on PR #6 | **PASS**, 4m43s |

Four **new** round-4a tests: `pre_main_screens_have_no_card_chrome`,
`settings_ceremony_card_survives_the_strip`, `verify_code_never_clips_silently`,
`destroy_ceremony_replaces_its_trigger`.

**Appendix E amended minimally, each edit citing its section:** **[E.1]** (the size table, the
wiped notice named for the first time, the reading width and the coupling, the compact floor,
and the WINDOW-IS-THE-CARD rule replacing the round-3 formulation) and **[E.4]** (the ceremony
card treatment now applies to the **Settings** surface only; red text carries danger on the
pre-main screens).

**A pre-existing clippy finding was reported and NOT fixed:** `cargo clippy --all-targets`
fails on `design_round3.rs:100` (`field_reassign_with_default`, round-3 code this lane did not
touch). **It gates nothing** — CI runs clippy **without** `--all-targets`. Recorded as a
possible future micro-lane rather than silently "fixed" outside scope.

---

## 9. Queue

```
STATE: READY=NONE | HIGHEST_NA=0665 | HIGHEST_D=1291 | BACKLOG_SOURCE=docs/ops/IMPROVEMENT_LEDGER.md
### NA-0665 ... Status: DONE
```

**Both layers agree** — the STATE line reads `READY=NONE` and the `### NA-0665` section reads
`Status: DONE`, so the STATE-vs-section mismatch class is not reintroduced. Helper readback:

```
$ python3 scripts/ci/qsl_evidence_helper.py queue --file NEXT_ACTIONS.md
READY_COUNT 0
$ echo $?
2
```

> **⚠ THE EXIT CODE IS 2, NOT 0, AND THAT IS CORRECT — RECORDED SO NO SUCCESSOR READS IT AS A
> FAILURE.** `queue_command` returns 2 whenever `READY_COUNT != 1`
> (`qsl_evidence_helper.py:249-251`), because its contract is *"exactly one lane is ready to
> start"* — the **qwork precondition**. At a closeout, `READY_COUNT 0` is the intended end
> state, so **exit 2 is the helper correctly reporting "no lane is ready to start."**
> `READY_COUNT 0` **with** exit 0 is only obtainable by passing `--allow-nonready-count`
> (verified: exit 0 with the flag). **The substance that matters — both layers agreeing at
> zero — is proven above.**

The executor cannot self-promote. **The operator promotes the successor.**
