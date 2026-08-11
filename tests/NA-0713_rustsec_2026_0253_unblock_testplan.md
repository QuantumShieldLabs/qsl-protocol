# NA-0713 — TEST PLAN — THE RUSTSEC-2026-0253 UNBLOCK (D-1350, D649)

⚠ **This lane adds no test.** It **deletes** a crate and **retires** one pin inside an
existing gate. The plan below is therefore about **instruments and controls**, not about
new assertions — and its central claim is that **every instrument used here was proven
able to go RED on the class it guards before its green was accepted.**

## 1. WHAT IS BEING PROVEN

| # | claim | instrument |
|---|---|---|
| C1 | RUSTSEC-2026-0253 is not reachable from the workspace at this lane's head | `cargo audit --deny warnings` |
| C2 | the ENG-0034 DH guarantee is not weakened by retiring one allowlisted site | `na0628_every_dh_call_site_is_guarded_or_allowlisted` |
| C3 | the workspace still compiles with the crate gone | `cargo check --workspace --all-targets --locked` |
| C4 | nothing but `qsl-tui`'s own targets left the test census | `cargo test --workspace --locked`, reconciled BY NAME |
| C5 | the shard manifest needs no change | `scripts/ci/qsc_shard_check.py` |

## 2. ⚠⚠ THE INSTRUMENT THE AUTHORIZING BRIEF NAMED CANNOT PROVE C1

The directive's constraint 3 named **bare `cargo audit`**, run before and after.
**Measured at `731b02a8` on the unmodified, red tree:**

    $ cargo audit            # bare — the brief's instrument
    Crate: lru  Version: 0.16.3  Warning: unsound  ID: RUSTSEC-2026-0253
    warning: 1 allowed warning found
    EXIT_STATUS=0

⚠ It **prints the advisory in full and exits 0**, because cargo-audit treats
`informational = "unsound"` as an *allowed* warning class unless told otherwise.

The gate is `.github/workflows/public-ci.yml` — line 222 (push lane) and line 235 (PR
lane) — and both run **`cargo audit --deny warnings`**:

    $ cargo audit --deny warnings    # the gate's instrument
    error: 1 denied warning found!
    EXIT_STATUS=1

⇒ ⚠⚠ **A lane that had obeyed the brief literally would have measured the RED tree as
CLEAN and reported a fix it never made.** Both runs are preserved: the `--deny warnings`
run is the **red control**, and the bare run ships as an **exhibit**, because `exit 0`
beside the advisory's own text *is* the finding rather than a footnote about it.

**Standing form adopted at R250 §1.2: the instrument is the one the GATE runs, not the one
the brief names. A brief that names a command has asserted a claim about that command, and
it gets measured like any other.**

## 3. CONTROLS — EVERY GREEN HERE HAS A RED BEHIND IT

| control | how RED was produced | result |
|---|---|---|
| **C1 red** | none needed — the unmodified tree **is** the red state | `--deny warnings` **exit 1**, one denied finding, 392 crates |
| **C1 green** | the committed edit set | **exit 0**, **347** crates, RUSTSEC-2026-0253 absent |
| ⚠⚠ **C2 red** | **delete the crate WITHOUT retiring the pin** | **exit 101** — `ratchet.rs:3887`, *"the set of `.dh(` call sites changed. Re-derive the inventory…"* |
| **C2 green** | retire the allowlist entry + the count pin (−6 lines, 292 bytes) | `na0628_…` **1 passed**, exit 0 |
| **C4 reference** | the baseline suite on the unmodified tree | **exit 0 · 168 sets · 810 passed · 0 failed · 2 ignored** |

⚠ **C2's red control is the strongest evidence this lane has, and it is not this lane's
work.** The scan was written for NA-0628/ENG-0034, for a different purpose, years of lanes
ago; it fails on precisely the deletion proposed here, and passes only once the pin is
retired deliberately. **A guard that can fail on your change, written by someone with no
stake in your change, beats any instrument a lane authors for itself.**

⚠ Its own source comment claims *"no CI job runs `cargo test -p quantumshield_refimpl`, so
this scan guards the lane gate and local runs, NOT pull requests."* **That comment is
stale** — `.github/workflows/ci.yml:369`, job `ci-4a`, runs exactly that, added by NA-0630
for exactly this purpose. **Filed as WF-0065; not fixed here, because the edit set is
closed and growth is a STOP.**

## 4. C4 — THE BY-NAME RECONCILIATION, AND A FALSIFIED PREDICTION

⚠ **The prediction written BEFORE the baseline was WRONG, and it is kept rather than
quietly corrected.** It said the census would contain *"exactly 5 `qsl-tui` entries"*
carrying *"4 tests"*. **Measured: 6 result sets carrying 5 tests.**

| census row (key: hash-stripped binary `::` target label) | passed |
|---|---|
| `qsl_tui::unittests src/lib.rs` | 2 |
| `qsl_tui::unittests src/main.rs` | 0 |
| **`doc::qsl_tui`** ← the row the prediction had no cell for | 0 |
| `local_e2e::tests/local_e2e.rs` | 1 |
| `meta_line::tests/meta_line.rs` | 1 |
| `metadata_visibility::tests/metadata_visibility.rs` | 1 |
| **6 sets** | **5** |

Two model errors, both instructive: **a `lib` target yields TWO result sets** — a unittest
binary *and* a doc-test set — and the lib unittest set holds **2** tests, not 1.
⚠ **Had the after-run been reconciled against the prediction, a CORRECT 6-set delta would
have read as an anomaly.** That is precisely why expectations are written first and kept
when falsified.

⚠ **The census itself was also rebuilt rather than patched.** Keyed on the target label,
7 of 168 rows came back `<UNATTRIBUTED>`, because `unittests src/lib.rs` occurs once per
package and the label is two tokens. Re-keyed on the hash-stripped binary basename:
**168 rows, 0 unattributed, passed column sums to 810** and reconciles against the log's
own totals.

**ACCEPTANCE for C4:** `BASELINE ∖ AFTER` is **exactly** the six rows above and
`AFTER ∖ BASELINE` is **empty**; totals **162 sets · 805 passed · 0 failed · 2 ignored ·
exit 0**. ⚠ **Any seventh removal, any addition, any changed pass-count on a surviving
row, or any failure is a STOP** — reported, not fixed.

⚠ All three integration-test names are **unique in the tree** (`find` returns exactly one
path each, all under `apps/qsl-tui/tests/`), so no other package's row can be mistaken for
one of these.

## 5. C5 — DISCHARGED BY MEASUREMENT, NOT BY AN EDIT

`qsc_shard_check.py`'s census truth is `qsl/qsl-client/qsc/tests/*.rs` (depth 1) plus
`lib`, `bin:qsc`, `doc:qsc`. It contains **zero** `qsl-tui` targets, and **no manifest
target lives under `apps/`**. Re-measured on the committed tree:

    manifest diff lines: 0 (byte-untouched)
    qsc_shard_check.py → EXIT 0
    "census 131 targets / manifest 131 rows / 12 shards / missing 0 / unknown 0"

⇒ **the census does not change, so the manifest does not change.** Reconciled by name in
both directions. ⚠ A re-measure finding otherwise at execution would have been a STOP.

## 6. COVERAGE SURRENDERED — STATED, NOT MINIMISED

The three deleted integration tests assert only `qsl_tui::demo`'s own API — `run_demo`,
`format_meta_line`, `DemoResult`. `choose_bucket` is the crate's **own private function**,
and its ladder `{256,512,1024,2048,4096,8192}` differs from `qshield-cli`'s 12-step ladder
`{256,512,768,…,8192}`, which carries its own tests (NA-0337, NA-0339, NA-0319, NA-0322,
NA-0324). ⇒ **no product or protocol property loses its only assertion.**

## 7. ⚠ WHAT THE SUITE'S GREEN DOES *NOT* PROVE

A green after-run shows the workspace still passes with the crate gone. **It is not
evidence that the advisory cleared** — that is C1, against a preserved exit-1 control —
**and it is not evidence the DH pin was correctly retired** — that is C2, against a
banked exit-101 control. **Three separate instruments, three separate claims, and none of
them substitutes for another.**

⚠ **And one run proved nothing at all:** the first after-suite attempt exited **101 with
ZERO test result sets** because `rust-lld` died on **signal 7 (Bus error)** at the link
step with the filesystem at **100% (9.0M free)**. **No test ran, so nothing about the
change was measured in either direction.** The build directory was **deleted rather than
reused** — artifacts linked while the disk was exhausted cannot be trusted, and a green
built on them would not be a green — and `git fsck` (exit 0, zero output) confirmed the
repository itself survived, because a full disk is exactly how a repository corrupts.
⚠ The harness reported *"exit code 0"* for that run: **the shell wrapper's status, not the
gate's 101**, recovered from the deliberately captured exit.
