# NA-0774 — AS BUILT

Lane `NA-0774` · `D-1417` · desktop `D-0045` · 2026-08-31 · seat: ui-fixes (Claude Code)

Bases, re-derived **bare and unpiped at the NAMED github remote** (never `origin`, the local mirror):
qsl-protocol main `2a4b159d64d0eae3377c0d03ea8f3660f2212a68` · qsl-desktop main `0a908099e0eee094aa3cde053cbae4da4b80ec82`

⚠ The kickoff's `A1` says *"protocol past `2a4b159d`"*. **Measured: protocol main's tip IS `2a4b159d`** — at it, not past it. Nothing has landed in protocol since NA-0773. The promotion PR bases on that commit exactly.

## 1. HARNESS ATTESTATION (brief sec 2(b), as amended by kickoff `A1`)

The brief says "eight declared at this base"; `A1` supersedes with **fourteen**, and fourteen is what exists. Run on this box at `0a908099` **before** anything was added, exactly CI's command (`ci.yml:190`):

```
cargo test --test gui_driver -- --ignored --test-threads=1
test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 189.43s
```

Per flow, read from each run's own `verdict.jsonl` keyed by **unique path** (never by name — the NA-0771 trap where two targets sharing a name silently overwrote one another): `a` PASS · `b` PASS · `c` PASS · `d` PASS · `e` PASS · `f` PASS · `g` PASS · `h` PASS · `i` PASS · `j` PASS · `k` PASS · `l` PASS · `m` PASS · `n` PASS. **14 verdict files found, 14 reported passed — the counts reconcile. 0 FAIL rows.**

⚠ **WHAT THIS RUN DOES NOT SHOW.** Flow `i`'s and flow `g`'s `#settings-code` preconditions both satisfied on their **first** poll, so this run **did not race**. It is a clean attestation of the base and it is **not** evidence that the NA-0768 precondition cure works; that evidence is the injected A/B and the one unforced `'true after 2 poll(s)'` row on desktop `#49`'s own CI run. It is also a **local** run and does not count toward the ten runs on `main` the forward predictions are scored over.

## 2. THE 2(d) ENUMERATION — every gateway call reachable from `relayScan` when `ev.source === "tick"`

Traced from bytes, link by link: `setInterval` (`ui/main.js:1995`) → `relayScan` → `relayScanOnce` (`:3406`) → `SCAN_CLASSES` (`:3385`) `= [finishScanClass, autoConnectClass]`.
Gate (`:1947`): the tick beats only on `scr-main`/`scr-settings` **and** with a relay configured.

| command | site | frequency |
|---|---|---|
| `relay_config_get` | `finishScanClass` `:2881`, `autoConnectClass` `:3344` | **twice per beat**, unguarded |
| `contact_list` | `finishScanClass` `:2887` | once per beat, unguarded |
| `invite_list` | `autoConnectClass` `:3350` | once per beat, unguarded |
| `connect_status` | `finishScanClass` | per contact row |
| `invite_finish` | `finishScanClass` | per eligible row |
| `invite_accept` | `autoConnectClass` | per due invite |

⚠⚠ **THE PER-BEAT FLOOR IS FOUR CALLS AND IT IS UNCONDITIONAL** — with zero contacts, zero invites and nothing whatever to do. `invoke()` (`:12-20`) showed `#busy-indicator` on every one of them and hid it only when `pendingCalls` returned to zero; the calls are sequential awaits, so the counter rose and fell repeatedly. That is the flash, enumerated rather than described.

## 3. THE COMMITTED BYTES

**`ui/style.css` — fix (a), two rules:**
```css
.list-pane { border-right: 1px solid var(--border); display: flex; flex-direction: column; min-width: 0; min-height: 0; }
#contacts-rows { flex: 1 1 auto; min-height: 0; overflow-y: auto; }
```

**`ui/main.js` — fix (b), the guard and its suppression point:**
```js
let tickQuietDepth = 0;
function invoke(cmd, args) {
  if (tickQuietDepth > 0) return tauriInvoke(cmd, args);
  pendingCalls += 1;
  byId("busy-indicator").classList.remove("hidden");
  ...
```
```js
  const quiet = ev && ev.source === "tick";
  if (quiet) tickQuietDepth += 1;
  try {
    for (const cls of SCAN_CLASSES) marks = await cls(marks);
  } finally {
    if (quiet) tickQuietDepth -= 1;
  }
```

**`ui/main.js` — fix (c), the identity-pane branch:**
```js
  let readFailed = false;
  try {
    rec = await invoke("identity_show");
  } catch (_) { readFailed = true; }
  ...
  if (!rec) {
    empty.classList.toggle("hidden", readFailed);
    readError.classList.toggle("hidden", !readFailed);
    body.classList.add("hidden");
    return;
  }
```

**`ui/index.html` — fix (c), the new element:**
```html
<div id="identity-read-error" class="hint hidden">Couldn't read your identity. Try again.</div>
```

## 4. THE SEALED EXPECTATIONS (brief sec 6, plus kickoff `E6`)

| | expectation | result | measured value |
|---|---|---|---|
| E1 | existing driver tests: same pass/fail set before and after | **HIT** | 14/14 before, the same 14/14 after (inside 16/16) |
| E2 | scroll test green on the fix; RED on the reverted rule | **HIT** | green rc=0; red rc=101, `expected "scrolls" / measured "NO-SCROLL scroll=1410 client=1410"` |
| E3 | tick test zero un-hides; RED on the reverted mechanism; control ≥ 1 in BOTH arms | **HIT** | green: 0 un-hides; red rc=101 `"UNHID 2 time(s) under a tick"`; **control PASS in both arms** |
| E4 | `git diff --name-only`: exactly the UI files + tests; `src-tauri/src/**` ABSENT | **HIT, with one widening** | see below |
| E5 | window height with 30 contacts == with 3 | **HIT, and it does not discriminate** | 673 == 673 — it held in the RED state too |
| E6 | identity pane renders the error state on a throw and the absent state on a null, pinned red-then-green | **HIT at the unit level only** | 3 seals green; red rc=101, all three fail. **The error path is not driven end-to-end.** |

⚠ **E4's widening, named rather than glossed:** the brief's list is `ui/style.css`, `ui/main.js`, `src-tauri/tests/gui_driver.rs`. The kickoff's `A2` **adds `ui/index.html`** for the new element and its sec 3 widens tests to `src-tauri/tests/**`. The kickoff wins on conflict. Measured set: `ui/style.css`, `ui/main.js`, `ui/index.html`, `src-tauri/tests/gui_driver.rs`, two scenario JSONs, `src-tauri/tests/na0774_identity_error_state.rs`, `scripts/ci/EXPECTED_TEST_INVENTORY.txt`. **`src-tauri/src/**`: ABSENT.**

⚠ **E5 and the scroll test's assertion (1) are HITs that prove nothing on their own** — both held in the red state. Recorded as HITs because they are true and must not regress, and flagged because a HIT that cannot fail is not evidence.

## 5. BOTH RED ARMS, PRINTED

```
RED ARM (a) -- the two CSS rules REVERTED                       rc=101
  step     : exec var h=document.getElementById('contacts-
  expected : scrolls
  measured : {"value":"NO-SCROLL scroll=1410 client=1410"}

RED ARM (b) -- the tickQuietDepth guard REVERTED                rc=101
  step     : exec return window.__na0774.unhides === 0 ? '
  expected : quiet
  measured : {"value":"UNHID 2 time(s) under a tick"}
  POSITIVE CONTROL row: PASS  measured={"value":"user-call-shows-indicator"}

RED ARM (c) -- the identity error branch + element REVERTED     rc=101
  3 failed: na0774_identity_error_element_exists_with_retry_copy,
            na0774_absent_and_error_are_distinct_elements,
            na0774_refresh_identity_pane_separates_throw_from_absent
```
Each arm was reverted **after the work was committed**, and the tree was proven identical to `HEAD` after every restore (`git status --porcelain` empty).

## 6. LOCAL VERIFICATION ON THE EXACT COMMITTED TREE

All nine steps taken from `.github/workflows/ci.yml` itself, unpiped, real exit codes:

```
1 cargo fmt --all -- --check                                 rc=0
2 cargo test                                                 rc=0
3 bash scripts/ci/test_inventory.sh                          rc=0
4 cargo clippy --all-targets -q -- -D warnings               rc=0
5 cargo test --test gui_driver -- --ignored --test-threads=1 rc=0
6 cargo audit --deny warnings                                rc=0
7 infra_literal_scan_selftest.py                             rc=0
8 infra_literal_scan.py --mode tree                          rc=0
9 infra_literal_scan.py --mode diff --base origin/main       rc=0
SUM=0
```

**PR-7 census, two instruments agreeing:** 22 targets by `Running`/`Doc-tests` lines and 22 by `test result:` lines — so no target silently overwrote another. **195 passed + 17 ignored = 212 = the inventory pin, exactly.** gui-driver 16/16. The `+1` target against NA-0768's 21 has a named cause: the new file `src-tauri/tests/na0774_identity_error_state.rs`.

## 7. CLAIM BOUNDARIES

- **The identity error path is not driven end-to-end anywhere.** `window.__TAURI__.core` is FROZEN (`invoke` non-writable, non-configurable), so a scenario cannot make `identity_show` throw — driven at 1500 ms and at 10 s, both left the flow green. The three seals prove both branches exist and are distinct in the shipped bytes; the rendered result of a real rejection is unproven.
- **The tick suppression has a stated window.** `await` yields, so a user-sourced call made **during** one of a tick pass's awaits is also suppressed. Closing it needs per-call context the platform does not give without threading the source through both scan-class signatures and all six call sites, where missing one is silent.
- **Rows are injected, not seeded through the app**, in the scroll flow: no relay exists in this harness and no UI path mints 30 contacts. The rows carry the same class and shape `renderContactsList()` produces; the CSS under test is layout, and layout does not care how a row arrived.
- **`relayScan` is driven directly with `source: "tick"`** rather than by waiting for a beat: the gate needs a configured relay this harness has none of, and waiting on wall-clock would be a sleep. What the mechanism keys on is the source.

## 8. SR-15

**NOT TRIGGERED, stated so it can be challenged:** no crypto, no lock region, no safety mechanism retired; three UI files, two scenarios, one test file and an inventory pin. Zero `src-tauri/src/**` bytes and zero Tauri commands.
