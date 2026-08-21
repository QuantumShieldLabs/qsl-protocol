# NA-0753 — AS BUILT

**Lane:** NA-0753 — THE FLIGHT-FIXES LANE. **Spine:** `D-1395`. **Desktop:** `D-0034`.
**Rulings:** `R375`, `R376`, `R377` (each banked 444 under SR-14, R-space swept before each banking).
**Bases:** qsl-protocol `fef80bc0db9004e20e53c70ded2fd09b80dad9b4` · qsl-desktop `f6ad42a00ccc2da010bb13843a78210dc5a9b85f` — both re-derived bare and unpiped against the NAMED `github` remotes, measured UNMOVED, and both **are NA-0752's merge commits**. Open-PR set EMPTY in both, proven non-vacuous by a positive control.

## What this buys, in one sentence

A wipe the operator performed in his first real flight left the previous profile's relay address and display name behind; it now does not, and three tripwires stand where the other flight defects were.

## The finding

Three vault-destroying paths, not two:

| # | site | path:line | removes |
|---|---|---|---|
| 1 | `destroy_vault_impl` (tokened destroy) | `commands.rs:350` | `settings.json` + `.tmp` |
| 2 | `erase_all_impl` (manual "Erase everything") | `commands.rs:396` | qsc dir + `settings.json` (not `.tmp` → `ENG-0119`) |
| 3 | **armed erase-after-N** (`O::Wiped`) | `commands.rs` (this lane) | `vault.qsv` (qsc) **+ `settings.json` + `.tmp`** |

Reproduced before any fix: post-wipe `data_dir` listing `["qsc", "settings.json"]`, `relay_url` and `self_alias` intact, `vault.qsv` gone.

## Seals

| seal | result | evidence |
|---|---|---|
| X1 the wipe is a wipe | **HIT, both arms** | base RED `["qsc", "settings.json"]` (log preserved 444) · fixed GREEN `["qsc"]` |
| X2 grouped code | **HIT** | both surfaces, group shape + payload shape driven in `f_i`; structural pin in `design_polish.rs` |
| X3 the gate | **HIT** | `https://1234` refused with the field byte-identical and no test fired; missing port named; scheme prepend visible |
| X4 the port hint | **HIT (presence)** | presence + attachment to the `unreachable` branch; behaviour undrivable without a network dependency — stated in the seal's own doc |
| X5 baseline | **HIT / one MISS** | 8 priors = **297** predicted and confirmed exactly; `f_i` **58 vs 54 predicted — MISS**, model corrected; inventory 129 → 134, control fatal-ward |
| X6 pre-push | **HIT** | fmt 0 · test 0 (124/0/10) · inventory 0 · clippy 0 · literal selftest 0 · literal tree 0 · audit 0 · gui_driver 9/9 |
| X7 records | **HIT** | ids once each with negative controls; class extracted-then-diffed-back with a tamper control; `ENG-0048` appended, never rewritten |
| X8 R282 | at each open | — |

## Instrument notes, carried

- ⚠ `read_text` polls but is **visibility-coupled**; `read_tc` reads a property but does **not** poll. **`prop_eq` is the only op with both** — the right instrument for text that may be off-screen. (NA-0752 moved `read_tc` → `read_text`; this is the same trade from the other side.)
- ⚠ Opening the Server pane calls `refreshServerPane()`, which is **async** and assigns the field AFTER an await; the pane `className` flips synchronously, so waiting on it alone races. Settle on `#relay-token-help`.
- ⚠ Arming erase-after-N is gated on `#wipe-ack`; without it the handler returns early and the arm **silently does not happen** while the click still reports `rc=0`.
- ⚠ `grep` on this box is `ugrep`: complex regexes fail loudly with *"exceeds complexity limits"* rather than matching nothing.
- ⚠ The runner already avoids the `pgrep -f` self-match trap (`runner.py:257` uses `pgrep -x`). Recorded so a future harness edit cannot regress it silently.
- ⚠ `cargo audit` must run from the **repo root**; the workspace `Cargo.lock` is there, not under `src-tauri/`.

## Corrected harness emission model

launch #1 = **4** (`launchN_ready` + `launchN_session` + the liveness pair) · each later launch = **2** · `isolation_bracket` = **1 per scenario** · `teardown` and `note` = **0** · every other op = **1**.

## Owed, recorded rather than fixed out of scope

- A **mockup refresh**: 07/07b's code card moves to the space-grouped single-node form at the next mockup-maintenance touch.
- `style.css`'s below-floor comment still cites the retired `QSCF-XXXX-` format as its wrap example. The BEHAVIOUR is now true (spaces give the wrap a real group boundary); the comment's example stays stale, and its file is outside this lane's enumeration.
