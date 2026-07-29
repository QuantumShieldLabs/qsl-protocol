# NA-0687 — AS-BUILT EVIDENCE (D621 / D-1326)

**Lane:** NA-0687 · **Directive:** `QSL-DIR-2026-07-29-621` (D621), APPROVED 2026-07-29, all seven
flags ruled at their drafted defaults, three riders folded in; sha256
`fb4a3ac2575ab19b64e948430681078ba5b26df3d48d3f18591e0ace77c73285`, 556 lines.
**Result class:** **`LOG_CAPTURE_SYNC_SWEEP_PASS`** — re-derived 2026-07-30 against §8's original
gate after the lane was extended (§10). ⚠ It first landed in the operator-authorised third class
`…_PASS_WITH_SECOND_MECHANISM_FILED` (§7, left standing with its grounds); **§10 supersedes §7.** **Repos:** `qsl-server` (test-only, repo-local **D-0017**), `qsl-protocol`
(governance, **D-1326**). **Class summaries only; no raw private values.**

Every expectation in this record was written **before** the check it governs. Raw logs are
operator-side under `/srv/qbuild/operator/NA-0687/`; nothing here depends on reading them.

---

## 1. Phase 0 — state verified, 17 of 17 expectations met

`qsl-protocol` `585830e1`, `qsl-server` `8d77cc6c`, both worktrees clean, `READY=NONE`,
`^Status: READY` count **0**, `HIGHEST_NA=0686`, `HIGHEST_D=1325`, `D-1326` absent, qsl-server local
decisions `D-0001…D-0016` each exactly once, ledger 91 `### ENG-` headings topping out at ENG-0091,
`ENG-0092/0093/0094` free tree-wide, root 57%, `/backup/qsl` mounted. Base gates:
`cargo fmt --all -- --check` **exit 0**; infra-literal selftest **13 checks, 0 failed**; tree scan
**clean (81 files, 17 106 lines examined)**.

⚠ **PREMISE CORRECTION AT PHASE 0 — the intent expected the next qsl-server decision to be
`D-0015`; measurement says `D-0017`.** `D-0015` (NA-0677, the infra-literal gate) and `D-0016`
(NA-0678, the invite-slot subsystem) both exist. The Director's own figure was an estimate, and the
"measure, don't inherit" instruction attached to it is what caught it.

⚠ **METHOD NOTE — `origin/main` cannot be resolved in a fresh seat.** `new_checkout.sh` clones from
the local **mirror**; `origin` is configured as the GitHub URL but has no fetched refs, so
`git rev-parse origin/main` fails with *"Needed a single revision"* in a new checkout. The stronger
comparison was used instead: **HEAD == mirror/main == `git ls-remote origin refs/heads/main`** —
verified against the **live** remote without fetching, in both repos.

---

## 2. The census — 12 sites, all FIX

Search keyed on the **capture mechanism**, not on one needle; three independent enumerations
reconciled: capture-writer definitions (**10 files**), subscriber installs (**12**), buffer reads
(hand-filtered — ⚠ `.text()` also matches `reqwest::Response::text()`, so a needle-only search on
that token **over-reports**). 12 installs across 10 files, each in a distinct `#[tokio::test]`.

| # | file | test | abort before read | pos | neg | sync | class |
|---|---|---|---|---|---|---|---|
| 1 | `src/lib.rs` | `payload_not_logged` | YES | **0** | 1 | nudge only | FIX |
| 2 | `src/lib.rs` | `logs_do_not_contain_raw_channel` | YES | 1 | 1 | none | FIX |
| 3 | `src/lib.rs` | `overload_logs_are_safe_and_structured` | YES | 3 | 2 | none | FIX |
| 4 | `tests/abuse_rate_queue_logging.rs` | `pressure_logs_redact_route_auth_payload_and_keep_msg_id_boundary` | YES | 4 | 1+loop | nudge only | FIX |
| 5 | `tests/hardening_auth_reject_logging.rs` | `logs_do_not_leak_route_auth_or_payload_on_success_or_rejects` | YES | 2 | 1+loop | nudge only | FIX |
| 6 | `tests/idempotency_logging.rs` | `x_msg_id_log_boundary_is_metadata_only` | YES | 2 | 1+loop | nudge only | FIX |
| 7 | `tests/na0349_end_to_end_integration_contract.rs` | `na0349_qsl_server_qsl_attachments_contract_model_is_end_to_end_bounded` | **NO** | 2 | 4+loop | none | FIX |
| 8 | `tests/na0598_exact_4mib_relay_logging.rs` | `exact_4mib_relay_logs_remain_metadata_only` | YES | 3 | 1+loop | nudge only | FIX |
| 9 | `tests/na0642_retention_logging.rs` | `retention_cleanup_logs_redact_route_auth_payload` | YES | 4 | 1+loop | nudge only | FIX |
| 10 | `tests/na0678_invite_slots.rs` | `bundle_is_opaque_bytes_in_bytes_out_and_never_logged` | YES | 1 | 2 | none | FIX |
| 11 | `tests/qsl_attachments_integration_contract.rs` | `na0347_secret_env_public_ingress_and_log_redaction_boundaries_hold` | **NO** | 2 | 3 | none | FIX |
| 12 | `tests/rate_global_cap_logging.rs` | `rate_and_route_cap_logs_redact_route_auth_payload` | YES | 4 | 1+loop | nudge only | FIX |

**28 positive + 19 negative** static assertions over captured text · **0 sites carried a sufficient
synchronisation** · 10 of 12 aborted before reading · **0 multi-thread sites** (so stop condition 8
did not fire at the census). **All 28 positive needles were traced to their emitters and every one
is SERVER-emitted** — including the four `NA####_..._METADATA` sentinels, which are msg-id header
values the test sends and the relay logs. **Not one site asserts on a marker it emitted itself.**

**Ten point predictions written before the enumeration: four held exactly, six missed.** The shape
was consistent — **the population was over-predicted (18 → 12) and its uniformity under-predicted**.
⚠ **The instructive miss: NOT-THIS-PATTERN predicted 3, measured 0.** The prediction reasoned about
how such a test *would plausibly* be written; the measurement traced all 28 needles to their
emitters. *A sweep measures what tests ASSERT, not what the code plausibly does.*

⚠ **A CENSUS-SURFACE GAP, CAUGHT BY THE BASELINE RATHER THAN BY REASONING.** M1 measured **129
passed** against a predicted **115**; the +14 is `src/main.rs`'s `mod cli_tests`, which the static
attribute count had never included — **so the census's second search had examined a narrower surface
than its first.** The enumeration was re-run over **all** of `src/` and `tests/`: `main.rs` and
`store.rs` carry no capture mechanism. **The population of 12 stood, now proven over the complete
surface rather than assumed over a partial one.** The count was wrong; the census was not — and that
could not have been known without re-checking.

⚠ **CENSUS CORRECTION MADE DURING PHASE 5:** the table above says *"nudge only"* where the original
census said *"none"*. **Six of the twelve sites carried a single `tokio::task::yield_now().await`
before the abort/read** — a best-effort nudge granting the server task exactly one scheduling
opportunity, which neither waits for nor detects the emit. The precise statement is **0 of 12
carried a sufficient synchronisation, 6 of 12 carried a nudge.** ⚠ **Every observed failure — this
lane's M2 and M6 reds and both of ENG-0091's runner instances — landed in the six UN-nudged sites.**
*The nudge was never a synchronisation, but it was the difference between a defect that fires and
one that had not yet been seen to.*

---

## 3. The remedy as built

One shared helper, `qsl-server` `tests/common/mod.rs` (name and `#![allow(dead_code)]` header
derived from the spine's existing `qsl/qsl-client/qsc/tests/common/mod.rs`): `capture()`,
`CaptureWriter`, `log_text()`, `try_await_log()`, `await_log()`, `await_logs()`,
`LogWaitError::Timeout{needle, waited_ms, bytes, lines}`, plus the test-side gated writer the
controls need. **5 s deadline, 50 ms poll** — derived from the tree's existing readiness idiom
(rider R-b), not invented.

Per site: **await the site's own positive sentinel → then `abort()` (if the site aborts) → then
assert, unchanged.** ⚠ **`await_logs` awaits EVERY needle a site asserts positively, not just the
first**: a site asserting on both a push line and a pull line has two emits to lose, and because the
buffer only grows, the final snapshot contains every needle awaited before it.

⚠ **`payload_not_logged` acquired its first failure mode by operator ruling (F6+R4).** Its only
assertion was negative and read after `abort()`, so an empty buffer satisfied it: it could not fail.
The anchor is a **synchronisation precondition, not a new content claim** — 7 of the 12 sites already
assert `channel_id=` positively — but the consequence is intended and is recorded in D-0017.

**Side-effect inventory, written BEFORE the ten hand-rolled writers were replaced** (nine items:
buffer type, `write`, `flush`, clone semantics, read path, return type, lock-poisoning handling,
the writer/reader variable split, and the deliberate non-folding of subscriber construction).
**One behavioural difference in nine:** two sites read strictly (`String::from_utf8`, panicking on
invalid UTF-8) and now read lossily, matching the ten-site majority. Nothing asserts UTF-8 validity
of the log, every needle is ASCII, and lossy is the more permissive direction — it cannot make a
`contains` assertion pass that would otherwise fail.

---

## 4. Controls — red-capable, deterministic

The gated writer stages bytes and reveals them only on release, so the lost race is a **state**, not
a scheduler accident.

| control | expected (written first) | measured |
|---|---|---|
| **A** (temporary) unfixed shape, gate withheld | RED, positive assertion fails, non-zero exit | **RED**: `assertion failed: text.contains(NEEDLE)`, exit **101**; the four permanent controls still passed in the same run |
| **A′** the gate withholds, then reveals | GREEN | GREEN |
| **B** fixed shape, released late from another task | GREEN, and the wait provably waited | GREEN, waited **≥150 ms** |
| **C** fixed shape, never released | named timeout over an **empty** buffer, bounded | GREEN as an assertion on `Err(Timeout{..})`; `bytes == 0`, `lines == 0`, `waited_ms ≥ 5000`, message contains `LOG_SYNC_TIMEOUT` |
| **C2** fixed shape, populated buffer lacking the needle | timeout reporting a **populated** buffer | GREEN, `bytes > 0`, `lines ≥ 1` |

⚠ **Control A left no residue, and the revert was PROVED, not assumed:** sha256 of the control file
before the temporary edit `2588ffcc…22e9fd3d`, after the revert **`2588ffcc…22e9fd3d`** —
byte-identical. (The file was untracked at that point, so `git checkout --` could not have restored
it; a hash comparison against a pre-edit copy is the stronger proof.)

⚠ **A′ exists because the control instrument needs a control.** If the gate silently released,
control A would go green and "prove" the defect cannot happen — ENG-0089's lesson one surface over.

---

## 5. Measurements

| | predicted (written first) | measured |
|---|---|---|
| **M1** baseline `RUST_TEST_THREADS=2` | 27 binaries / 115 passed / 0 / 0 / exit 0 | **28 / 129 / 0 / 0 / exit 0** |
| **M2** pre-fix, full parallelism ×5 | ≥1 RED matching the pre-declared signature | **1 of 5 RED** (127/2) |
| **M5** post-fix `RUST_TEST_THREADS=2` | 29 / 134 / 0 / 0 / exit 0 | **29 / 134 / 0 / 0 / exit 0 — exact match** |
| **M6** post-fix, full parallelism ×5 | 5/5 exit 0 | **4 of 5** (one red, 133/1) |

M5's addition is exactly the named controls: +4 in the new `na0687_log_sync_controls` binary and +1
in the lib (the F2-fallback copy's own control). **Nothing else moved**, as §9 required.

**M2's two failures (pre-fix), verbatim:** `na0678_invite_slots.rs:562` *"redacted id must be
logged"* and `qsl_attachments_integration_contract.rs:362` *"assertion failed:
text.contains("channel_id=")"* — **both positive, both at census FIX sites, matching the
pre-declared signature**, so rider R-a applied and no separate stop cycle was opened. **Zero
negative assertions failed in any of the five runs**, independently confirming ENG-0091's *"the
assertion is the positive one, every time"*. The second message is **byte-identical** to the one
ENG-0091 recorded from GitHub run `30483439679`.

⚠ **M2 WAS AN ADDITION TO THE APPROVED INTENT (flag F3), AND IT IS WHAT MAKES M6 READABLE.** *A
negative result is evidence only if the instrument could have returned positive.* It also produced
the **first local reproduction of this flake in the project's history**, landing on the runner's two
instances rather than ENG-0065's.

⚠ **NO CLAIM IS MADE THAT THE FULL-SUITE FLAKE RATE FELL: 1-in-5 before, 1-in-5 after.** Measured:
**failing sites in the red run 2 → 1**, and a failure that is now diagnosable. The 1-in-20 figure in
§6 has a **different denominator** (one 16-test binary, not the 29-binary suite) and cannot be
compared with the 1-in-5 figures. ⚠ **This is the annotation-corrected arithmetic.**
`STOP_NA0687_002` originally said the rate "fell"; the executor caught it while checking its own
summary against the run logs, and the stop-file carries an **append-only** correction with the
original sentence left byte-identical. *The parenthetical-estimate lesson, recurring inside the
lane's own record.*

**Gates after the change:** `cargo fmt --all -- --check` exit 0 (formatted with `rustfmt` on this
lane's own files, never `cargo fmt --all`); `cargo clippy --all-targets -q -- -D warnings` exit 0.

---

## 6. ⚠ THE SECOND MECHANISM — the fix revealed that the diagnosis was incomplete

M6 run 3 failed **legibly**:

```
LOG_SYNC_TIMEOUT: needle "channel_id=" not observed within 5027ms (buffer 0 bytes, 0 lines)
```

**`0 bytes` after the full deadline falsifies slow-emit outright.** A lost race yields the needle
inside the deadline or a **populated** buffer missing it; it cannot yield a buffer still empty after
100 reads at 50 ms. **Nothing was ever captured.**

**One discriminating experiment, prediction written first, both arms confirmed:**

| arm | scope | runs | predicted | measured |
|---|---|---|---|---|
| A | whole binary, full parallelism | 20 | ≥1 red with `0 bytes` | **1 red**, same signature |
| B | that test ALONE (`--exact`) | 20 | 0 red | **0 red** (`15 filtered out` confirms the filter matched exactly 1 test — not a silent skip) |

**The failure requires sibling tests in the same process** — inconsistent with a per-emit race.
**Hypothesis, labelled as inference:** `tracing` caches callsite `Interest` globally per process
while `set_default` is thread-local; 15 of the binary's 16 tests drive the same single
`info!("push channel_id={} id={} bytes={}", …)` callsite with no subscriber on their threads.
**Honest limit: the experiment that would confirm the mechanism — installing a process-global
subscriber — IS the candidate fix**, so it was deliberately not run.

**Filed as ENG-0094**, unfixed, ruled to its own lane on **D-1319's grounds** (an unreviewed design
proposed late in a long lane). ⚠ **The pre-fix instrument printed the same text for both
mechanisms**, so **ENG-0091's own recorded data points may include this one and there is no way to
tell retrospectively** — which is why **ENG-0091 stays OPEN** while **ENG-0065 closes**.

---

## 7. Result classification, and why a third class was authorised

D621 §8 declared `..._PASS` / `..._STOP`. **Neither is honest.** `_STOP` misstates a sweep that
completed with every in-scope expectation met (12/12 sites, M5 an exact match, all controls as
predicted, the revert proved). Unqualified `_PASS` is **barred by §8's own gate**, which required M6
5/5 and measured **4/5** — *a lane does not get to soften the gate it wrote.* The operator
authorised **`LOG_CAPTURE_SYNC_SWEEP_PASS_WITH_SECOND_MECHANISM_FILED`** and ruled that **the 4/5
stands and must not be re-run for a 5/5**.

---

## 8. Governance shipped

**`qsl-server`:** repo-local **D-0017**; one `TRACEABILITY.md` row (this lane's own only — ENG-0066's
older gap is still owed by directive); the test changes.
**`qsl-server` PR #69** — merged **2026-07-30T00:45:15Z** as **`37ec8207`**, two commits
(`51bb2a3` the sweep, `1b6df98` the ruled extension); all four checks green on the extension commit
(`rust` 1m46s, `advisories` 1m59s, `infra-literal-scan` 7s, `public-safety` 5s), and **the earlier red
run on `51bb2a3` was never re-run** — it stands in the record as the measurement that forced the
extension.

**`qsl-protocol`:** **D-1326**; `NEXT_ACTIONS.md` STATE bump `0686/1325 → 0687/1326`, the `prior:`
record, and the `### NA-0687` block **born DONE** (one-act enqueue/promotion/execution on the
NA-0678 / NA-0685 precedent); `TRACEABILITY.md` Changelog bullet; this evidence file; the testplan.

**Ledger:** the **`Resolution:` convention** and its **partial-closure rule** adopted in the header;
seven retro `Resolution:` lines (ENG-0075, 0082, 0084, 0085, 0088, 0089, 0090), each verified against
its own closure annotation first; **ENG-0065 CLOSED**; **ENG-0091 annotated, left OPEN, no
`Resolution:`**; **ENG-0092** (qsl-server CI's `cargo test -q`), **ENG-0093** (the scanner's
untracked `__pycache__`) and **ENG-0094** (the second mechanism) filed. Ledger `### ENG-` headings
**91 → 94**; `Resolution:` lines **0 → 8**.

⚠ **The convention's first two uses both needed the partial-closure rule** (ENG-0087's annex,
ENG-0091's second mechanism). *Had it gone the other way, they would both have reported open work as
closed — the convention failing at exactly the job it was adopted for.*

---

## 9. Findings recorded so no successor pays for them twice

1. ⚠ **A `#[path]` module declared inside an INLINE module resolves relative to `<dir of this
   file>/<inline module name>/`** — for `mod tests` in `src/lib.rs`, the **phantom** directory
   `src/tests/`. It does not exist, so the kernel cannot resolve `..` through it and **no** relative
   path escapes: `couldn't read src/tests/../tests/common/mod.rs` (and `../../` fails identically).
2. ⚠ **`include!` cannot take a module file's inner attributes**: `an inner attribute is not
   permitted in this context` plus `E0753: expected outer doc comment` ×6, because
   `#![allow(dead_code)]` and `//!` docs are exactly what make it a proper module file.
   **F2's ruled fallback was taken** — one shared definition for the integration tests, a second copy
   in the lib's test module naming the source of truth, **with its own control** so it cannot drift
   into vacuity. ⚠ **A working third option (a top-level `#[cfg(test)] #[path]` module) was
   DELIBERATELY NOT TAKEN** because it sits outside §6's permission; named as a non-choice a later
   lane may revisit.
3. ⚠ **`tokio`'s `time` feature is not declared by `qsl-server`** — it arrives transitively via
   `axum 0.7.9` and `reqwest 0.12.28`, which is what keeps the remedy inside a no-`Cargo.toml`-change
   scope. Recorded with its no-feature fallback (`Instant` + `yield_now()`, at the cost of spinning a
   core).
4. ⚠ **ENG-0093's subject, found by this lane's own gate run:** the scanner leaves an untracked,
   **not**-gitignored `scripts/ci/__pycache__/`; a lane staging with `git add -A` after verifying its
   own gate would commit bytecode. Tree-wide, since the scanner is byte-identical in four repos.

---

# 10. THE EXTENSION — ENG-0094 FIXED IN-LANE (added 2026-07-30)

⚠ **§7's classification above is SUPERSEDED and is left byte-identical.** It was correct when
written. What changed: **the deferred second mechanism blocked this lane's own merge.**

## 10.1 What forced it

PR #69's required `rust` check went **RED** on the 2-vCPU runner:

```
logs_do_not_leak_route_auth_or_payload_on_success_or_rejects   (census site 5)
LOG_SYNC_TIMEOUT: needle "push channel_id=" not observed within 5018ms (buffer 83 bytes, 1 lines)
```

A **POPULATED** buffer — where site 10's had been `0 bytes`. **The sweep did not cause it**: the
pre-sweep form asserted the same needle on an *immediate* read, so a line still absent after 5 018 ms
of polling was certainly absent at t≈0; waiting cannot lose a line that immediate reading would have
caught. ⚠ **The relay's emit is at `src/lib.rs:1161`, BEFORE the 200 at `:1168`** — so a client that
observed 200 proves the emit had already run, which removes any "the response beat the log" account
and leaves only "the event was discarded before reaching the subscriber".

## 10.2 The design was RULED FROM MEASUREMENT, not chosen

Scratch reproducer of the exposure pattern (15 sibling tests driving the shared callsite with no
subscriber + 1 capture test), **20 runs per arm, predictions written first**:

| arm | mechanism | predicted | **measured** |
|---|---|---|---|
| base | `set_default` alone | ≥1 red | **16 / 20 RED** |
| D3 | + `rebuild_interest_cache()` | 0 red under H1 | **19 / 20 RED** |
| D2 | `WithSubscriber` on the emitting future | 0 red under H2 | **20 / 20 RED** |
| D1 | global default carrying data + thread-local routing | 0 red | **0 / 20** |
| **D4 — RULED** | permissive global → `io::sink`, capture untouched | 0 red if global FILTER state | **0 / 20** |

⚠ **BOTH PRE-WRITTEN HYPOTHESES FALSIFIED.** H1 (stale per-callsite `Interest`) required D3 green — it
was red. H2 (thread-local dispatcher visibility, **OBS-10's family**, the standing suspicion since the
census) required D2 green — it was red. **D4 is decisive because its subscriber discards everything**:
it cannot capture, so the only thing it can have changed is **process-global filter state**. Internals
remain **INFERENCE**; the five outcomes are the claims. **D1 measured identically, rejected on blast
radius** (it would route every event in the binary through one writer and rely on per-thread
bookkeeping to keep tests apart; D4 cannot capture, leak or misroute, and fails **loudly**).

⚠ **A FIDELITY GAP, and a secondary prediction that missed:** all 16 base reds reported `0 bytes` —
the reproducer never produced site 5's populated presentation, because its capture test exercises only
ONE callsite. It modelled the mechanism, not both faces of it, which is exactly why the confirmatory
arms below were load-bearing.

## 10.3 What shipped, and its own control

`install_permissive_global_once()` in both helper copies, one call at each of the twelve sites, **not
one assertion changed**. The operator-authorised **bounded excerpt** (240 bytes, newlines flattened,
**test-data surface only**) now rides in the timeout message — size separated *empty* from
*populated*, but only content names **which** line arrived, and its absence is why this mechanism cost
a five-arm experiment instead of one CI log. Plus
**`control_d4_the_permissive_global_is_installed_and_permissive`**: RED if no global default is set, or
if the global max level would drop the relay's INFO lines. *A fix needs a control for its own failure
mode, not only for the one it replaced.*

## 10.4 Measurements after the extension, each predicted first

| | predicted | measured |
|---|---|---|
| M5 `RUST_TEST_THREADS=2` | 29 binaries / 135 / 0 / 0 / exit 0 | **exact match** |
| M6 ×5 full parallelism | 5/5 exit 0, 135 each | **5/5 exit 0, 135 each** |
| arm S5 `hardening_auth_reject_logging` ×20 | 0/20 red | **0 / 20** |
| arm S10 `na0678_invite_slots` ×20 | 0/20 red | **0 / 20** |

Gates: `fmt --check` 0 · `clippy --all-targets -D warnings` 0 · infra-literal selftest 13/13, tree
clean over **83 files / 17 764 lines**, staged clean over **12 files / 212 lines**.

⚠ **TWO PROCESS SLIPS OF THE SAME KIND, and the second names the pattern.**

**(a) One implementation slip, recorded because the catch was luck.** A scripted edit adding an
assertion on the new `excerpt` field **silently did not apply** — `rustfmt` had rewrapped the anchor
text, and the script used a replace without asserting the pattern matched. **`clippy -D warnings`
caught it only because the now-unused binding was visible.** Had control C2 destructured with `..`
instead of naming the field, the assertion would simply never have existed and nothing would have
said so. *The same defect class this lane exists to close, inside the lane's own edit process.*

**(b) Four values in this lane were ASSERTED where an instrument was available.** `D-0015` (taken from
the lane intent — measured `D-0017`); the required-check set (taken from the handoff packet — measured
three contexts, not four, and `public-safety` no longer among them); the "flake rate fell" arithmetic
(taken from my own summary — measured 1-in-5 either side); and the **stop-file timestamps** (taken from
expectation — `STOP_NA0687_005` and `_006` carry stamps ~36 minutes in the future of their measured
mtimes, and 006's claimed time was later than the moment the correction was written). Each was
corrected in place, mark-don't-rewrite, and the fourth was caught only because polling CI printed the
real clock next to the claim.

⚠ **THE GENERALISATION, which is worth more than any of the four corrections: the defect this lane
fixed in test code is the same defect I kept committing in records.** A log-capture assertion that
reads a buffer without synchronising on the writer is a claim taken from expectation rather than from
the instrument — and so is a decision id copied from an intent, a required-check list copied from a
packet, an arithmetic summary trusted over its own logs, and a timestamp typed instead of read.
**Forward rules adopted: every scripted edit asserts that its pattern matched before writing, and every
timestamp in a record comes from `date -u` at the moment of writing.** A stop-file whose stamp is in the
future cannot be ordered against the events it describes.

## 10.5 Result class re-derived against §8's ORIGINAL gate

Every criterion now passes, **M6 5/5 included**, so the class is plain
**`LOG_CAPTURE_SYNC_SWEEP_PASS`**. The interim third class and the reasoning that produced it stay in
D-1326: *a lane does not get to soften the gate it wrote, and the record of why it could not is worth
more than the label.*

## 10.6 Refusals, recorded with the options they answer

**Weakening the two exposed waits** (a needle chosen for being un-poisoned, or a non-fatal wait):
**REFUSED** — choosing needles to pass reintroduces the vacuity this lane removes. **Splitting the PR**
to land only "unexposed" sites: **unsound** — exposure is a property of the callsite and the binary's
test mix, not of a site's own code, so "unexposed" could only ever mean "has not failed yet".

## 10.7 Standing rule ratified here

> **Scratch-space investigation is pre-authorised: an executor may build and run throwaway experiment
> code OUTSIDE tracked repo paths to diagnose a mechanism, provided nothing is committed, tracked
> trees stay clean, and the revert is proved. Repo edits — including test code — still require a
> ruling.**

Exercised first in this lane; the reproducer was deleted with its revert **proved** (`git status
--porcelain` empty, `git diff HEAD` empty). To be folded into the house-defaults document when
**ENG-0081**'s micro-lane runs — **no `CLAUDE.md` edit here**, since that file's edit fires both full
suites and is out of scope.
