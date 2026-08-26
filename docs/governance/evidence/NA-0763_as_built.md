# NA-0763 — AS BUILT: the liveness runway's lane two, the tick (`D-1404`)

**Lane:** NA-0763 · **Decision:** `D-1404` · **Date:** 2026-08-26 · **Bases:** qsl-protocol
`7b51b9de264912e75f8c5f89ba7831854485f5bc` and qsl-desktop
`b25d5fdb79c544cbaba862f2a803a62ae906231f`, both re-derived bare and unpiped at the NAMED
github remotes at STOP 001 **and again at build**, and measured UNMOVED. Desktop qsc pin
`0b9d6967948c2fcf799cb817aeee55d5095835aa`, unchanged — **no pin bump**.
**Ordered by:** `BRIEF_liveness_lane2_tick_20260826.md`, sha256
`deb42b73ad90b4157e72c5e6ff2352e82f74fb74e1d4d8e6233947f142ef85ed`, 177 l / 11067 B, 444.
**Authorized by:** `RULING_NA0763_001_build_20260826.md`, sha256
`e8ff3534ad1cd7f69b8c84bb3b2f87a745d84619169838e8dc2df54c2c47d71a`, 79 l / 5284 B, 444.
**Premises artifact:** `STOP_NA0763_001_20260826T035406Z.md`, sha256
`6112dce60d4504539823f28d7774d0dfb73fe65574e307c61ebddfd71b4e422e`, 582 l / 46864 B, 444.
**Design of record:** `DESIGN_delivery_ladder_metronome_v2_20260825.md`, sha256
`aba8e2a5f8c388d1c7ac850c7b94790365cc9749e92e1a40b63ff22d056b8c59`, sections 1-2 governing.
Every one of the four **sha-VERIFIED against its own bytes BEFORE being read** — all 64 digits
compared mechanically, with a negative control proving the comparator discriminates on each.

⚠ **WHY THIS FILE EXISTS.** The measurements below lived only under `/srv` (D-1 / R331.1). It
carries STOP 001's substance and the build's own measurements into repo truth, citing the stop by
sha rather than restating it as newly derived. It is gitignored (`**/evidence/`) and therefore
added with `git add -f`.

## 1. THE PREMISES, AS MEASURED

**(a) Bases and open-PR sets.** Both mains matched the brief exactly at STOP 001 and again at
build. Open-PR sets **both EMPTY**, with a positive control (`--state all` returning five merged
PRs per repo) proving the query can return rows — so the empty sets are a measurement, not a
broken instrument, and a shared counter derived against main is not blind to a parallel branch
here, because there is none.

**(b) The trigger census, from bytes.** Trigger (a) at `ui/main.js:708` inside `enterMain`;
trigger (b) at `:2439` inside `openRedeemChooser`; both funnel into ONE function. The scan reads
`relay_config_get`, `contact_list`, then per contact `connect_status` (EQUALITY on a closed set of
two) and, for pending contacts only, `invite_finish`. ⚠ **The brief's "bounded 8 pulls × 16
frames" does not match the GUI's own argument, which is `max: 1`** — reported, not reconciled into
qsc. ⚠⚠ **No handshake poll runs from the GUI today and none can:** 44 registered commands, none a
handshake verb; `handshake` occurs in desktop product source exactly **twice, both in comments**.

**(c) The consolidation map and the concurrency guard.** The only relay-facing call in a scan is
`invite_finish`; `relay_config_get`, `contact_list` and `connect_status` are LOCAL. ⇒ **a scan
costs the relay ZERO requests when nothing is pending.** Guard at base: **none at scan level**.
`CoreGateway` is a process-wide single-flight gate over **individual calls** whose callers QUEUE on
a mutex rather than fail, so two overlapping scans would interleave; `core_busy` is the same
granularity and reads true during any unrelated call.

**(d) The knob's storage surface.** `settings_get` returns the whole `AppSettings` struct to JS;
`settings_set` is fixed-arity and "owns ONLY these two fields". `AppSettings` is
`#[serde(deny_unknown_fields)]` and its own in-file `settings_key_allowlist` asserts the default
serializes to exactly `["autolock_minutes"]` — which is why the tempo field must use
`skip_serializing_if`. **Zero JS-side persistence** (`localStorage`/`sessionStorage`/`indexedDB`/
`document.cookie` all absent from `ui/`, measured **unpiped** with a positive control).

**(e) The lease.** `PULL_LEASE_SECS=60`, provenance **[O]**, repo truth inside the `ENG-0142`
entry; source default 60 with a 3600 ceiling, kept as a separate claim; `/v1/server-info` does not
advertise it. ⛳ **The floor is RETIRED:** `403432ce` (NA-0742's producer acks) is an ancestor of
the desktop's pin — `git merge-base --is-ancestor` rc=0 — with the negative control (current
protocol main is **not** an ancestor of the older pin) rc=1.

**(f) Harness capability.** The rig fires unlock and reads arbitrary JS by `exec`; `runner.py`
already injects `QSLD_DATA_DIR` and `GDK_BACKEND` per launch, which is the mechanism the tempo seam
rides. ⚠ `exec` is a ONE-SHOT compare and the runner had no bounded predicate poll — hence the one
new `poll_exec` op, authorized at `R8` and inside the brief's edit set.

## 2. WHAT WAS BUILT

`ui/main.js` (the tick, the one handler with `{source, at}`, single-flight + rerun bit, the knob
read, backoff-and-report, the marker separation, two stale-comment corrections), `ui/index.html`
(one `<p class="status-line-quiet">` on `scr-main`; **no CSS minted**), `src-tauri/src/settings.rs`
(the `Tempo` knob with the default omitted; the pure seam parser), `src-tauri/src/commands.rs` (the
seam on `AppInfoDto` — a `Serialize`-only type with no save path). **Four source files against
SR-15's line of five**, no lock or crypto adjacency, nothing retired.

Tests and rig: `gui_driver.rs` (+`run_scenario_with_env`, +the `m` flow), the new bounded
`poll_exec` runner op, `f_m_liveness_tick.json` (51 steps), and the `EXPECTED_TEST_INVENTORY` pin.
Records: desktop `DECISIONS.md` (`D-0040`, and `D-0039` as a deliberate recorded gap).

## 3. INSTRUMENTS AND THEIR RED ARMS, RUN

| arm | green | RED ARM, run |
|---|---|---|
| I1 tick-fires | 4 ticks after 2 polls | interval disabled ⇒ `TIMEOUT after 20 polls, last=0` |
| I2 lock-gates | zero ticks across a bounded locked window; `tickNextDueAt` null after (schedule RESET, not paused) | gate flipped ⇒ post-lock check read `true` |
| I3 single-flight | exactly 1 refusal, exactly 1 rerun, pending slot empty | — (the refusal count IS the measurement) |
| I4 marker separation | shared slot still reads `unlock` after many ticks | separation removed ⇒ slot read `tick` |
| I5 threshold/recovery | status raises at 3 with the exact copy; recovery hides it | — (see the bound below) |
| `R4` seam seal | seam absent from the file under every field combination | `#[serde(skip)]` dropped ⇒ **two** arms red, file read `{"autolock_minutes":60,"tick_override_ms":137}` |

**Suite at the head: 182 passed / 0 failed**, all **13** gui-driver flows green (162.04 s);
`fmt`, `clippy -D warnings`, `test_inventory` all rc=0. The pre-edit baseline at the base was
**166 passed / 0 failed**, run to completion before anything was touched.

⚠ **BOUNDS, STATED NOT GLOSSED.** **I5's relay-fault PATH is FIELD-VERIFIED**, not harness-proven:
a fresh profile has no pending contact, so no relay call is attempted and no failure can accrue
(`ENG-0226`, open). **The jitter DISTRIBUTION is not harness-proven either**: the seam is
deliberately un-jittered so the instrument is deterministic. **Harness green is not a field claim.**

## 4. WHAT THIS LANE GOT WRONG, AND HOW IT WAS CAUGHT

- **The seam's first design could never have worked.** `#[serde(skip)]` on `AppSettings` omits the
  field from serialization, and Tauri's IPC uses the same impl — the lane's own passing test proved
  it could not reach the FILE and thereby that it could not reach the WEBVIEW. Caught by probing
  rather than reasoning; moved to a `Serialize`-only DTO, which is structurally stronger.
- **STOP 001's census was too narrow.** Scoped to tests pinning the scan MARKERS, it missed the two
  tests pinning the trigger CALL SITES and the whole-file `setInterval` count, and the two
  exhaustive `AppSettings` literals. The FULL suite caught all of them at once — subset-green would
  not have.
- **STOP 001's headline prediction measured false.** The two committed `"why":"surface_open"`
  expects do NOT go red without the marker separation, because `R10`'s relay gate stops the tick
  from running in that scenario at all.
- **I3 needed quiescence, not just a closed gate.** It measured 60, then 2, before the arm was made
  deterministic by waiting for `relayScanBusy` to fall. Closing a gate stops new work; it does not
  abort work already in flight.
- **Two seal breaks were the seat's own prose**, not real timers: a doc comment containing the
  literal `setInterval(`, and another naming the primitive inside the invite-module slice where it
  is banned outright. The seals were right; the wording changed.

## 5. BOUNDS

**ZERO qsl-protocol product source bytes.** Zero relay or server changes, zero `.github/**`, no pin
bump, no standing rule minted, **no test weakened, skipped or deleted** — arms were ADDED and one
seal re-aimed with its count still an exact equality. The three riders from NA-0762's close-out all
landed, rider (iii) beside `ENG-0142` with **0 deletions in the diff** and the entry still not
closed. Nothing merged by the seat; the operator merges.
