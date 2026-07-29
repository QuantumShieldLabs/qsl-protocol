# NA-0687 — LOG-CAPTURE SYNCHRONISATION: THE INSTRUMENT RECORD (testplan)

**Lane:** NA-0687 · **Decision:** D-1326 · **Directive:** `QSL-DIR-2026-07-29-621` (D621) ·
**Repo under test:** `qsl-server` · **Result class:** **`LOG_CAPTURE_SYNC_SWEEP_PASS`**
(re-derived 2026-07-30; §9's addendum supersedes §7's filed-and-unfixed disposition)

This file records **the instrument**, not the narrative: what was searched, what was asserted, what
each control would catch, and what each measurement can and cannot support. The as-built record is
`docs/governance/evidence/NA-0687_as_built.md`.

---

## 1. The property under test

> **A log-capture assertion must be true because of what the relay logged, not because of when the
> scheduler happened to run it.**

Two failure directions, both in scope:

| direction | how it fails | had it ever been seen? |
|---|---|---|
| **positive** — *a line is present* | fails when the emit loses the race | **yes** — twice on the GitHub runner (ENG-0091), and 1-in-5 locally at full parallelism (this lane's M2) |
| **negative** — *a line is absent* | passes **vacuously** when read before the buffer is populated | **no, and it never could** — an assertion that cannot fail produces no evidence of anything |

---

## 2. The census instrument

**Keyed on the capture MECHANISM, never on one needle.** Three independent enumerations, reconciled:

| search | what it keys on | result |
|---|---|---|
| 1 | `struct SharedWriter` definitions | **10 files** |
| 2 | subscriber installs (`= set_default(`, `set_global_default(`) | **12** |
| 3 | buffer reads (`from_utf8_lossy`, `.text()`, `.lock()`), hand-filtered | ⚠ `.text()` also matches `reqwest::Response::text()` — a needle-only search on that token **over-reports** |

**Reconciliation: 12 installs across 10 files** (`src/lib.rs` carries 3; every other file exactly 1),
each inside a distinct `#[tokio::test]` fn. **Population = 12 assertion sites**, all read in full.

**Reported with the size of the input, because a census that does not say what it examined cannot be
audited:** 27 test targets · 115 test attributes · 10 capture-writer files · 12 sites · and the
repo's own gate clean over **81 files / 17 106 lines**. **Empty input is a failure, not a pass.**

⚠ **The search surface must match the claim.** Search 2 was first run over `src/lib.rs` + `tests/*.rs`
— narrower than search 1's `src/` + `tests/`. The M1 baseline exposed it (a +14 test-count miss
traced to `src/main.rs`'s `mod cli_tests`), and the enumeration was re-run over the complete surface:
`main.rs` and `store.rs` carry no capture mechanism, so the population stood. **A filter that
examined less than it claimed is the vacuous-pass failure one level up.**

---

## 3. What each site asserts, and what the remedy may not touch

- **28 positive** and **19 negative** static assertions over captured text (7 negatives are
  `for forbidden in [...]` loops that expand further at runtime).
- **All 28 positive needles are SERVER-emitted** — traced to their emitters, including the four
  `NA####_..._METADATA` sentinels, which are msg-id header values the test sends and the relay logs.
  **No site asserts on a marker it emitted itself**, so no site was exempt from the pattern.
- ⚠ **The remedy changes WHEN the buffer is read and nothing else. Not one needle was weakened,
  dropped, or made conditional.** A guard's assertion is never rewritten down.

---

## 4. The remedy, and the two rules inside it

`tests/common/mod.rs`: `capture()` · `await_log()` · `await_logs()` · `try_await_log()` ·
`LogWaitError::Timeout{needle, waited_ms, bytes, lines}`. **5 s deadline, 50 ms poll — derived from
the tree's existing readiness idiom, not invented** (rider R-b).

1. ⚠ **ORDER: await the sentinel → then `abort()` → then assert.** 10 of 12 sites aborted before
   reading. On a current-thread runtime `abort()` guarantees a not-yet-emitted line is **never**
   emitted, so a wait placed after it can never succeed — it would convert a flake into a
   deterministic timeout.
2. ⚠ **ANCHOR EVERY NEEDLE, not just the first.** `await_logs` awaits each positive needle in order;
   a site asserting on both a push and a pull line has two emits to lose. The buffer only grows, so
   the final snapshot contains every needle awaited before it.

**The error message is part of the instrument.** It names the needle, the wait **and the size of the
buffer examined**, because *nothing emitted at all* and *the wrong thing emitted* are different
defects that must not be reported by the same words. ⚠ **This is the line that revealed the second
mechanism (§6); without it, ENG-0094 would still be misfiled as a flush race.**

---

## 5. Controls — what each one would catch

The gated writer stages bytes and reveals them only on release: the lost race becomes a **state**, so
no control depends on thread count, core count or luck.

| control | shape | expected | permanence | what its absence would let through |
|---|---|---|---|---|
| **A** | unfixed shape, gate withheld | **RED**, exit non-zero | temporary; reverted with the revert **proved** | a fix that was never shown to be a fix |
| **A′** | gate withholds, then reveals | GREEN | permanent | a silently-releasing gate would make control A green and "prove" the defect impossible |
| **B** | fixed shape, released late from another task | GREEN, wait provably ≥150 ms | permanent | a helper that returns early, or reads a buffer that happened to be full |
| **C** | fixed shape, never released | `Err(Timeout{..})`, `bytes==0`, `lines==0`, `LOG_SYNC_TIMEOUT` in the message | permanent | an unbounded wait, or an anonymous failure |
| **C2** | fixed shape, populated buffer lacking the needle | `Err(Timeout{..})`, `bytes>0`, `lines≥1` | permanent | the empty/populated distinction collapsing — the whole basis of §6 |

**Plus one control for the duplicated helper**: `log_sync_timeout_is_named_and_reports_what_it_read`
in `src/lib.rs`'s test module. The F2 fallback left a second copy of the wait helpers there
(§7), and **an unguarded copy could drift into vacuity silently** — this makes its contract
observable rather than trusted.

**Measured:** A **RED** (`assertion failed: text.contains(NEEDLE)`, exit 101; the four permanent
controls still passed in the same run), revert sha256 `2588ffcc…22e9fd3d` → `2588ffcc…22e9fd3d`
**byte-identical**. A′, B, C, C2 all **GREEN** as specified.

---

## 6. Measurements, and what each can support

| | predicted (first) | measured | supports |
|---|---|---|---|
| **M1** `RUST_TEST_THREADS=2` | 27 bin / 115 | **28 / 129 / 0 / 0 / exit 0** | a completed baseline; the miss exposed the census-surface gap |
| **M2** full parallelism ×5, **pre-fix** | ≥1 red matching the signature | **1 of 5 RED**, both ENG-0091 instances, both positive, **0 negative failures** | **the instrument CAN show red** — without this, M6 proves nothing |
| **M5** `RUST_TEST_THREADS=2`, post-fix | 29 / 134 / 0 / 0 / exit 0 | **exact match** | a test-only change moved nothing at the house thread count |
| **M6** full parallelism ×5, post-fix | 5/5 exit 0 | **4 of 5** | the surviving failure, and its diagnosis |

**Pre-declared signature (rider R-a):** *a POSITIVE log-capture assertion failing at a census FIX
site.* M2's red matched it, so it was recorded rather than treated as a new stop. **M6's red did not
get that carve-out** — R-a is M2-only by its own words — and took the full §14.5 treatment.

⚠ **WHAT THESE NUMBERS DO NOT SUPPORT.** **No claim that the full-suite flake rate fell: it was
1-in-5 before and 1-in-5 after.** Measured instead: **failing sites in the red run 2 → 1**, and a
failure that is now diagnosable. The 1-in-20 figure in §6 of the as-built has a **different
denominator** (one 16-test binary, not the 29-binary suite). **Five runs cannot establish a rate in
either direction.** ⚠ **The operator ruled that the 4/5 stands and must not be re-run for a 5/5** —
*a green obtained by repeating until the noise stops is indistinguishable from one obtained by fixing
something.*

---

## 7. The second mechanism, and why it is filed rather than fixed

M6 run 3: `LOG_SYNC_TIMEOUT: needle "channel_id=" not observed within 5027ms (buffer 0 bytes, 0
lines)`. **`0 bytes` after the full deadline falsifies slow-emit** — nothing was ever captured.

**One discriminating experiment, prediction first, both arms confirmed:** whole binary at full
parallelism **1 red in 20**; that test alone (`--exact`) **0 in 20**, with `15 filtered out`
confirming the filter matched exactly one test. **The failure requires sibling tests in the same
process.**

**Hypothesis — INFERENCE, not measurement:** `tracing` caches callsite `Interest` globally per
process while `set_default` is thread-local; 15 of 16 tests drive the same single `info!` callsite
with no subscriber on their threads. **Honest limit: the confirming experiment IS the candidate
fix**, so it was not run. Filed **ENG-0094**; **ENG-0091 stays open on it**; ENG-0065 closes.

---

## 8. Reproduction

```
cd <qsl-server checkout>
cargo test --test na0687_log_sync_controls          # the four controls: 4 passed
RUST_TEST_THREADS=2 cargo test --no-fail-fast       # 29 binaries / 134 passed / exit 0
cargo test --no-fail-fast                           # full parallelism; see §6 for what to expect
cargo test --test na0678_invite_slots                # ENG-0094's site, with its 15 siblings
cargo test --test na0678_invite_slots -- --exact \
  bundle_is_opaque_bytes_in_bytes_out_and_never_logged   # the same site, alone: 0/20 red
```

⚠ Redirect output to a file and take `$?` **separately**. `cmd | tail` reports the **pipe's** exit
status, which is the mistake this project has already paid for once.

---

## 9. ADDENDUM 2026-07-30 — the second mechanism, and the instrument that ruled its fix

§7 above recorded ENG-0094 as filed-and-unfixed. **It was fixed in-lane** after it blocked this lane's
own merge (PR #69's required check went red at census site 5 with a **populated** buffer). §7 is left
byte-identical; this addendum records the instrument that chose the remedy.

**The reproducer:** 15 sibling tests driving the relay's shared `push channel_id=` callsite with no
subscriber on their threads, plus 1 capture test; arm selected by env var so each ran in a **fresh
process**, because the state under test is process state. **Temporary, deleted, revert proved.**

| arm | mechanism | predicted | measured | what its result proves |
|---|---|---|---|---|
| base | `set_default` alone | ≥1 red | **16/20 RED** | the instrument can show red — without this, nothing below discriminates |
| D3 | + `rebuild_interest_cache()` | 0 red under H1 | **19/20 RED** | **not** stale per-callsite interest that a rebuild repairs |
| D2 | `WithSubscriber` on the emitting future | 0 red under H2 | **20/20 RED** | **not** thread-local dispatcher visibility (OBS-10's family) |
| D1 | global default + thread-local routing | 0 red | **0/20** | fixes, but consistent with either hypothesis — the weakest evidence |
| **D4** | permissive global → `io::sink` | 0 red if global FILTER state | **0/20** | **decisive: it discards everything, so it can only have changed process-global filter state** |

⚠ **Both hypotheses written down in advance were falsified.** The prediction table required either D2
red + D3 green, or D2 green + D3 red. Both were red — the "neither account explains the data" case,
pre-declared. The surviving account is labelled **INFERENCE**; the five outcomes are not.

**The control for the new mechanism** — `control_d4_the_permissive_global_is_installed_and_permissive`
— goes RED if no process-global default is set, or if the global max level would drop the relay's INFO
lines. It exists because **capture now depends on that global**, and without the control a regression
there would resurface only as the original flake returning at random, which is the state this lane was
opened to end.

**Confirmatory arms on the real binaries, predictions first, both 0 of 20:**
`hardening_auth_reject_logging` (site 5) and `na0678_invite_slots` (site 10). ⚠ **Load-bearing, not
ceremonial:** the reproducer never produced site 5's populated-buffer presentation, so it modelled the
mechanism but not both of its faces, and only these arms touched the real test mix.

**Reproduction:**

```
cargo test --test na0687_log_sync_controls           # 5 controls incl. control_d4_*
RUST_TEST_THREADS=2 cargo test --no-fail-fast        # 29 binaries / 135 passed / exit 0
cargo test --test hardening_auth_reject_logging      # site 5's binary
cargo test --test na0678_invite_slots                # site 10's binary
```
