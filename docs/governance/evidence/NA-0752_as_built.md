# NA-0752 — AS BUILT: THE STATUS-FOOTER TRUTH LANE

The first Slice-4 SCREEN act. The existing `#status-line` footer stops knowing one sentence and
starts reporting the desk's typed state. Spine decision **D-1394** (this repo); screen decision
**D-0033** (`qsl-desktop`). Ruled at **`R374`**.

## 1. THE AUTHORITY CHAIN, EACH LINK BANKED BEFORE IT WAS CONSUMED

| link | artifact | sha256 | size |
|---|---|---|---|
| design bank (operator-blessed) | `RBANK_status_footer_truth_design_20260821.md` | `f3ba1222…d4b8a` | 21 l / 1466 B |
| Director's brief (SR-14, FIRST ACT) | `BRIEF_NA0752_STATUS_FOOTER_TRUTH_20260821T171636Z.md` | `bb3fffba…bfc93` | 127 l / 9688 B |
| NA-0751's close-out (the class source) | `CLOSEOUT_NA0751_DIRECTOR_20260821T162300Z.md` | `05412c14…e87da` | 20 l / 1813 B |
| the consolidated ruling artifact | `STOP_NA0752_002_…_CONSOLIDATED_RULING_FILE_SIX_ASKS.md` | `683d9d3b…1edeef` | 639 l |
| the ruling | `RULING_NA0752_R374_…_BUILD_AUTHORIZED_20260821T180930Z.md` | `22fcad25…58bff` | 71 l / 5489 B |

The design bank's sha was VERIFIED before it was read (a mismatch was a stated STOP). The brief was
banked before anything consumed it, with every id sweep run BEFORE the banking per `WF-0087`. The
R-space was re-swept at the ruling's banking, as ordered.

⚠ **THE CONSOLIDATED ARTIFACT IS ITSELF A RECORD OF A FAULT.** STOP 001 plus three supplements had
become a four-file *read-them-in-order* set — the exact fragmentation the stop-file convention
exists to prevent — while STOP 001 §10 still promised *"the Director needs only this file"*. The
remedy was to RE-ASSEMBLE into one file, built by an assembler that substituted each carried
document from its own bytes, extracted them back out of the finished file and diffed (both
IDENTICAL, each with a live negative control), asserted no placeholder survived, and measured
29/29 containment. Accepted as the remedy at `R374`.

## 2. THE EDIT SET, AND NOTHING OUTSIDE IT

**THIS REPO (6 files) — zero product source, zero tests, zero scripts, zero workflows:**

    NEXT_ACTIONS.md                                  DONE flip + block birth + STATE advance
    DECISIONS.md                                     D-1394
    docs/ops/IMPROVEMENT_LEDGER.md                   ENG-0187 instance 2 · WF-0088 · WF-0089
    docs/governance/evidence/NA-0751_as_built.md     APPENDED §9 correction
    docs/ops/PREDICTION_LEDGER.md                    rows 157-169
    TRACEABILITY.md                                  one row
    docs/governance/evidence/NA-0752_as_built.md     this file (gitignored; git add -f)

**`qsl-desktop` (6 files, PR opened separately) — see D-0033:**

    ui/main.js                                              the footer writer
    src-tauri/tests/harness/scenarios/f_h_status_footer_truth.json   ONE scenario
    src-tauri/tests/gui_driver.rs                           the three-line wrapper
    src-tauri/tests/design_polish.rs                        seal F1b
    scripts/ci/EXPECTED_TEST_INVENTORY.txt                  re-pin 118 -> 129
    DECISIONS.md                                            D-0033

⚠⚠ **TWO MEASURED DELTAS FROM THE FORMALIZED DIRECTIVE, BOTH FROM ONE ROOT CAUSE, BOTH REPORTED
RATHER THAN ABSORBED.** §F1 enumerated **five** desktop files and constrained `gui_driver.rs` to
*"the THREE-LINE wrapper only"* — leaving seal **F1b**, which `R374` §4 ruled *as proposed*, with
**no file in the enumeration able to host it.** Resolved in favour of the ruled seal, because
dropping it would silently discard a ruled requirement:
- the desktop PR is **SIX** files (`design_polish.rs` is the sixth — the existing home of the
  `ui_file("main.js")` source-discipline idiom the seal names, whose module doc already requires
  every test to ship a proof it can fail);
- the re-pin is **129**, not 128: 127 at base + the wrapper + F1b's presence test. The directive's
  128 was computed while F1b had no home.

**ZERO BYTES:** `ui/index.html` · any mockup · `tests/harness/runner.py` and `wd_client.py` ·
`.github/**` · any `qsc`/protocol source · `capabilities/**` · `Cargo.toml` · `Cargo.lock` (no pin
moves — the desktop already pinned `9dcded4d`, verified). No new dependency, no test weakened,
skipped or deleted, no standing rule minted, no `ENG` and no `SR` minted.

## 3. THE THREE MEASUREMENTS THE DESIGN RESTS ON — none designed, all read out of the tree

1. **TWO SOURCES ARE STRUCTURAL.** `qsp_status_tuple` never reads relay config: `config_dir()`
   reads only env (`fs_store:10-30`), `check_parent_safe` only filesystem permissions (`:279+`),
   `qsp_session_load` only the session blob (`protocol_state:946-965`). ⇒ the desk **cannot** say
   *no relay configured*; `relay_config_get` cannot say the store is unwell. Either alone ships a
   false line, which is why the pre-existing writer survives BESIDE the desk.
2. **EXACTLY TWO OF NINE REASONS ARE APP-LEVEL.** Read off the tuple's own precedence ladder
   (`protocol_state:79-102`), the peer argument is first consulted at `:87`: `missing_home` (`:82`)
   and `unsafe_parent` (`:85`) are decided without it. The other five describe ONE PEER and fall
   through by design — a healthy fresh profile answers `missing_seed`, so signalling it would call
   every new install broken.
3. **THE TREE ALREADY ANSWERED THE PEER-LABEL QUESTION.** `main.rs:95` hard-codes
   `let status_peer = "peer-0";` inside qsc's own `status` verb — production code doing exactly
   this job. Its collision hazard is recorded in D-0033 with a FORWARD TRIGGER rather than absorbed.

⚠ **TWO BLESSED LINES ARE RESIDUAL, NOT ROUTINE — measured, and kept anyway.** `#status-line` sits
at `index.html:156` INSIDE `<section id="scr-main">`; `show()` hides every other screen; and both
lock paths (`:1412`, `:1431`) navigate to `scr-unlock` ⇒ **a user cannot see this footer while the
vault is locked by any in-app path.** And `missing_home` is unreachable while `bootstrap()`
(`lib.rs:302-311`) sets `QSC_CONFIG_DIR` before the runtime exists. Kept because a footer that
cannot say *storage is wrong* when the desk says so is the dishonesty this lane exists to remove.

## 4. THE HARNESS BASELINE — DERIVABLE FROM BYTES, NOT MERELY OBSERVED

Verdict rows are **not** declared JSON steps (a..f declare 233, emit 242). From the runner's own
emission rules — `note`/`teardown` 0 · `countdown_commit` 2 (`:438`+`:440`) · `launch` 2
(`:229`,`:236`) **plus `liveness_pair()`'s 2 more when `n == 1`** (`:238`) · `finish()` +1
(`:455`) — the model reproduces every published figure exactly:

    f_a 96 · f_b 20 · f_c 28 · f_d 25 · f_e 52 · f_f 21  = 242   the SIX-scenario figure
    f_g 26                                                ⇒ TRUE SEVEN-SCENARIO BASELINE 268
    f_h 28  (this lane)                                   ⇒ NEW TOTAL 296

⛳ **268 had never been recorded anywhere**; `g`'s own count appeared in no record, so comparing a
seven-scenario run against 242 manufactured a false **+26**. `f_h`'s **28 was predicted from the
model BEFORE the run and confirmed BY the run**, with all seven priors reproducing EXACTLY.

## 5. WHAT THE SEALS MEASURED (figures inserted after measurement, never in the pass that produced them)

- **F1a HIT.** `f_h` PASS, 28/28 rows, both drivable states by equality on extracted text.
  **Mutation control:** rows 4 and 5 swapped in a copy of the mapping ⇒ **RED**, rc 101,
  `read_text #status-line` rows not PASS. Restored byte-identical, re-verified green. Arms differ.
- **F1b HIT.** Presence test green. **Two controls, both fired:** rewording a ruled sentence
  (em-dash → hyphen) ⇒ RED naming the declaration; deleting the `vault_locked` arm ⇒ RED naming the
  dead copy. Restored byte-identical.
- **F2 HIT.** All seven prior scenarios reproduced EXACTLY; total 296 = 268 + 28.
- **F3 HIT — and its first control was WRONG.** Pin re-pinned at **129**, gate green with zero
  ADDED. The seal as drafted said *"remove one name from a COPY ⇒ RED"*; run, that exercises the
  **ADDED** direction, which the gate treats as informational **by design** — an asymmetry this
  lane had itself measured and then aimed a control against the wrong arm of. Re-run in the fatal
  **DISAPPEARANCE** direction: **rc 1, "TESTS DISAPPEARED"**, naming the sentinel.
- **F4 HIT.** Required set **re-measured from branch protection at build time**, never inherited:
  `["rust","advisories","infra-literal-scan"]` — confirming `ENG-0208` (ci.yml's own comment still
  names a different set). `rust`'s four gating steps in CI's order: `cargo fmt --all -- --check`
  rc 0 · `cargo test` **120 passed / 0 failed / 9 ignored** · `test_inventory.sh` PASS · `cargo
  clippy --all-targets -q -- -D warnings` rc 0. `infra-literal-scan`: selftest **13 checks, 0
  failed** · Tier 1 clean (78 files / 24276 lines) · Tier 2b clean over **353 added lines**
  (non-vacuous — it examined the diff rather than refusing an empty one). `advisories`:
  `cargo audit --deny warnings` rc 0 over 518 crate dependencies / 1225 advisories. Every exit
  status read UNPIPED.
- **F5 HIT.** Every derived id declared exactly once. The class and all three retrospective axes
  transcribed by **extract → resolve → diff-back**, each diff EMPTY, each with a tamper control
  proven to differ FIRST. The `NA-0751_as_built` correction APPENDED with the originals proven
  unmodified by a **prefix test** (the pre-edit file is an exact prefix of the post-edit file, its
  sha matching) plus a negative control showing that test can fail. `ENG-0187`'s insertion is a
  **pure addition, 53 added / 0 deleted**, anchored bracket-first because `- Severity:` occurs
  **192 times** in that file.
- **F6** — recorded at the stop, verified immediately before each PR opens.

## 6. THE MISSES, RECORDED BEFORE THEY WERE TOUCHED

A miss is a result. All are rowed in `PREDICTION_LEDGER.md` **157-169**.

1. A standing *"protocol main is RED"* condition written from an INHERITED record. Re-measured at
   the same sha: **69/69 green**. ⇒ re-deriving a base sha and re-measuring its CHECK STATE are two
   different acts, and only the first was performed.
2. `unsafe_parent` predicted from the reason's NAME instead of `check_parent_safe`'s bytes.
3. A DefaultHome control that was VACUOUS — caught by its own control returning the same value on
   an all-0700 tree, because that branch walks every ancestor and `/tmp` is 1777.
4. The `ENG-0187` substance carried as a DESTINATION STRING, not as text (6 of 8 containment probes
   NO) — inside the very section titled *"BUILD-PHASE DEPENDENCY CLOSURE"*. ⇒ **compute the closure
   against the ORDERING document, not against your own draft of it.**
5. A dependency closure asserted over `STOP 013` having read only §7, never §5b.
6. A needle WEAKENED between two audit runs (`'SEND sub-process'` → `'send'`), which would have
   manufactured a pass. Caught; the original eight re-run verbatim, 0 missing.
7. A confirmation line that LAUNDERED a write failure — `wc -l` read the OLD file after
   `Permission denied`. Cured by staging + `cmp` with a tampered copy proving `cmp` discriminates.
8. Four files where the convention demands one (see §1).
9. Seal F3's control aimed at the gate's informational arm (see §5).
10. `read_tc` would have shipped a RACE; substituted `read_text`, which polls and is
    visibility-coupled. ⛳ The substitution also buys a stronger assertion: `index.html:156`'s
    static default is a DIFFERENT, shorter string, so a writer that never ran fails the check.

## 7. THE ONE FIGURE THE SEAT REFUSED TO LAND, AND WHY IT LANDED ANYWAY

The brief ordered *"the #1767 fast-fail context, 1m10s, too fast to reach the test"* into the
`ENG-0187` entry. Measured: #1767's failing CHECK-RUNS are 4 s / 24 s / 4 s / 7 s, none near 1m10s,
none a macOS shard job; `#1767` appears nowhere in `STOP_NA0751_013`. **The seat refused to land an
unreconciled figure and raised it as a ruling ask.** `R374` §5 supplied the missing instrument: it
is a **workflow-RUN** duration from `gh run list`, a different object class from a PR check-run —
so the sweep was CORRECT and could not have found it. The clause now lands as sourced CONTEXT with
its provenance stated beside it, and the Director rowed the ordering of a figure without its
instrument against his own chair. ⇒ **state the instrument beside the figure, always.**
