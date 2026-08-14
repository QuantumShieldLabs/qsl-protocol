# NA-0724 — AS BUILT — the suite-economics lane (D-1360)

**Lane:** NA-0724 · **Decision:** D-1360 · **Result class:** `SUITE_ECONOMICS_SHARDING_PASS`
**Directive:** `QSL-DIR-2026-08-14-655` (sha256 `1d5e35d3…10da`, 879 l) as amended by
**AMENDMENT A1** (`452b81b5…3f9a`, 1001 l, R295) and **AMENDMENT A2** (`dadd3745…d3ea`, 394 l,
R296). **Precedence: A2 > A1 > base**; earlier files are never rewritten, only byte-frozen.
**Rulings:** R295 `d138ecc4…aaa0` · R296 `612e316e…0edb6` · R297 `2a524df8…eb82` ·
R298 `3237d86d…92de` · R299 `90257388…bf83`. **SR-15 findings:** `826c1a1f…b328`
(3 BLOCKER / 6 MAJOR / 5 MINOR / 6 NOTE, all dispositioned).
**Base:** main `f8370bcee9c4749c1947ebf0ab8b069290a6ab21`, re-verified UNMOVED by bare unpiped
`git ls-remote` against the NAMED `github` remote at Phase 0 and again at the edit. Open PRs
measured **0** at both points. **Branch:** `na0724-suite-economics-sharding`.

**What this lane buys, in one sentence:** ENG-0185 is cured by arithmetic — the push-arm
watchdog's own inequality now holds with `MAX_CEILING` **270 ≤ 280** instead of 330 — and
push-suite feedback collapses from ~3.5 h to under an hour, in one act.

---

## 1. THE CENTRAL CLAIM, PROVEN BY EXECUTING THE CONSUMER

WF-0073 means this cannot be proven by any PR: `public-ci.yml` fires on `pull_request_target`, so
a PR runs **main's** copy, and its sizing step is `if: github.event_name == 'push'` — **no PR ever
executes it** (655 DV-1). It is therefore proven by lifting `public-ci.yml`'s constants and
`ceiling_of()` **byte-for-byte** from the tree, dedenting the 10-space `run: |` indent, and
running the whole sizing block **bare and unpiped** against the edited workflow files.

| gate | tree | derived | rc |
|---|---|---|---|
| **G1b** (RED-FIRST, run BEFORE G1a) | macOS shard ceiling 150 | `ERROR: derived watchdog coverage 510m exceeds the safe bound 340m` | **2** |
| **G1b′** boundary | macOS ceiling **94** | `MAX_CEILING 282` · `COVERAGE 342` · ERROR naming 342 | **2** |
| **G1b′** boundary | macOS ceiling **93** | `MAX_CEILING 279` · `COVERAGE 339` · **no ERROR line** | **0** |
| **G1a** | the real edited tree, 90/90, waves 3 | `binding shard ceiling 90m` · `x fan-out waves 3` · **`binding ceiling 270m`** · `coverage 330m (safe bound 340m)` · `--max-iterations 990` | **0** |
| **G1c** | both aggregates | `<EMPTY>` + the named `no timeout-minutes found` error | **2** |
| **G1d** | both workflows, job `shard` | shell `awk` `'90'` == Python parse `'90'` | 0 |

**At base the identical instrument refuses**: `MAX_CEILING 330` · `COVERAGE 390` · rc **2** with
`ERROR: derived watchdog coverage 390m exceeds the safe bound 340m`. That refusal **is** ENG-0185,
and it was also read from production: `public-safety` at `0ad65a58` (the last runtime-relevant
push) is `completed/failure`, and its **job log** — not its step name — carries exactly that line.
The step is *named* for suite redness; only the log reports the cause.

**⇒ `MAX_CEILING` = 270 ≤ 280 and `COVERAGE` = 330 ≤ 340, by the consumer's own code.**

**THE REFUSAL BOUNDARY IS NOW A MEASURED FACT (R299.3): the guard refuses at 342 (shard ceiling
94) and passes at 339 (ceiling 93). 93 is the largest admissible shard ceiling.** A future lane
that re-fits either shard ceiling to **94 or above turns the sizing step red on the next non-docs
main push and cannot be caught pre-merge** — the sizing step is push-only. Its repair is an edit to
`.github/workflows/**` ⇒ `workflow_security` ⇒ the class the admission freeze refuses, so the
repair route runs through the §T9 two-step door. **S17 makes any such re-fit a STOP before it is
written, and this row is its evidence.**

## 2. THE DESIGN

- **Linux half — retire the duplicate.** `qsc_linux_full_suite` deleted from `ci.yml`. Nothing
  replaces it: the census is covered by `qsc-sharded-suite` (§3) and the release build by the
  **REQUIRED** `ci-4a`, which already runs `cargo +stable build -p qsc --release --locked` on the
  PR critical path. The watchdog's Linux arm becomes the existing honest name `qsc-sharded-suite`.
- **macOS half — a new sharded workflow.** `.github/workflows/macos-qsc-sharded-suite.yml`,
  K=5 (four working shards + the doc shard, which must be ALONE because cargo refuses `--doc`
  mixed with any other target selector). `macos_qsc_full_serial` deleted from `macos-build.yml`;
  `macos-qsc-qshield-build` — also REQUIRED — already carries both macOS release builds.
- **The cancellation defect, cured in the same commit.** `concurrency.group` is keyed **per-SHA on
  push**, per-ref on `pull_request`. `github.ref` is `refs/heads/main` for every main push, so the
  old group let a later merge cancel an earlier merge's suite — measured on main `b9aebb2a`
  (run 31740533267, `qsc-shard-4` cancelled). Once the check is `--required`, a cancellation is an
  immediate RED for a reason unrelated to test health. The new macOS file is **born** with the
  cure; it never carries the defect.
- **The sizing block reads the `shard` job key, never the aggregate.** The aggregate mints the
  check name but bounds no work — its own timeout is absent, so `ceiling_of` returns `<EMPTY>` and
  exits 2 rather than collapsing `MAX_CEILING` to a false-small number that would PASS the guard
  while UNDER-covering the wait.
- **`FANOUT_WAVES = 3`, and 3 is the ceiling of the arithmetic path.** 2 waves are STRUCTURAL —
  5 macOS shards + the REQUIRED `macos-qsc-qshield-build` = 6 jobs against a 5-slot pool, before
  any contender exists — and the third wave buys exactly ONE concurrent contender against an
  **org-wide** pool. `4 × 90 = 360 > 280`, so a fourth wave is unreachable: if one is ever needed,
  the arithmetic path is spent and R282's `workflow_run` redesign becomes necessary rather than
  deferred. **Headroom is 10 minutes, not 100** — the honest cost of replacing a false margin.

## 3. CENSUS EQUIVALENCE, BOTH DIRECTIONS, AT ONE CENSUS

`cargo metadata --no-deps --offline` over package `qsc`: **1 lib** (`test=true`, `doctest=true`) ·
**1 bin** · **129 test** · **0 examples** · **0 benches** · **0** targets with `required-features` ·
**0** with `test=false`. ⇒ `cargo test -p qsc --locked` selects **132** target sets.

Both manifests prove exact cover **in both directions at the same census**:
`IN CENSUS NOT IN MANIFEST: []` and `IN MANIFEST NOT IN CENSUS: []`, `EXACT SET EQUALITY: True`,
shard ids contiguous, no empty shard, `doc:qsc` alone in each (Linux shard 11, macOS shard 4).
`qsc_shard_check.py` exits **0** on both.

**Corroboration from production:** over the last 30 first-parent main commits both suites ran
comparably on 8 shas and agreed green on 6. The 2 disagreements are classified and **neither is a
test outcome** — one a `cancelled` shard (the defect §2 cures), one an `actions/checkout` TLS
failure before any test ran. **Zero disagreements attributable to coverage or test behaviour.**

## 4. THE macOS PARTITION — MEASURED, NOT PROJECTED

Seed re-extracted at execution from the **newest completed `macos-qsc-full-serial` run**, not the
drafting's: **job 94606278726** (run 31747736537, sha `0ad65a58`), **180.95 min**. Its log carries
**132 `Running`/`Doc-tests` lines and 132 `test result:` lines — an exact cover of today's census**
(the drafting's run had 131 at a 131-census; the suite grew by one target in eight days and this
run covers it). Test phase **175.91 min**; fixed overhead before the first `Running` **5.05 min**.

**Two independent attributions were computed and cross-checked (R298.3):** cargo's own
`finished in Ns` values, and **deltas between consecutive `Running` timestamps** — the latter
immune to stdout/stderr transposition because it never pairs the two streams. **They agree on all
132 targets; zero disagree by more than 5 s.** §4c's interleaving caveat (11 of 131 targets
misattributed in the earlier capture) therefore **does not bind this capture**, and the partition
stands on the better seed.

Greedy-LPT over the 131 non-doc targets into 4 working shards: loads **43.97 min each, within
0.153 s of one another** — the ideal. Largest single target `tests/handshake_mvp.rs` at
**29.36 min** is the floor no partition can beat. **Projected worst shard ≈ 49.0 min warm = 54.5 %
of the 90-minute ceiling**, comfortably under the ratchet's 80 % warn line.

⚠ **NOT CLAIMED: that the 90-minute macOS shard ceiling is FITTED.** It is an initial hang-bound,
~1.6× the projection, in the shape NA-0698 ruled for the Linux 90 — an overrun is a bounded
failure rather than a six-hour hang. It is re-fitted only from the **first measured sharded macOS
main run** (R289: a projection may not fit a ceiling).
⚠ **The first such run has a COLD cache** — `shared-key: macos-qsc-sharded-suite` has never been
saved and `save-if` fires only on main shard 0 — so a **healthy** worst shard plausibly lands
58–70 min and may trip the 60-minute signal band. **That is anticipated — F11's expectation, S5's
first-run-aware band, restated unchanged at R298.4 — recorded, and it is not a finding.**

## 5. GATES, EACH WATCHED RED BEFORE IT WAS TRUSTED

**G2 — manifest coverage and the new arguments: twelve red arms, every one observed rc 1.**
R1 Linux manifest short a row · R2 macOS manifest short a row · R3a/R3b `doc:qsc` co-tenancy on
**both** the default path and `--emit-args` · R4 omit `--expect-runners` · R5
`--expect-runners macos-latest-xlarge` (outside the hard-coded `STANDARD_RUNNERS` allowlist) ·
R6 omit `--expect-job-runner` · R7 `--expect-job-runner shard=ubuntu-latest` against the macOS
file · **R8 the F8 slip itself** (macOS file with `shard.runs-on: ubuntu-latest`) · **R9 the F13
form** (block-sequence `runs-on:`) · **R10 the F14 binding** (`--ratchet … 75` against
`timeout-minutes: 90`) · R11 `--max-shards 4` against K=5.

**G2b — call-site invariance across the dispatcher restructure**, in the form clarified at R298.1
after this seat found A1 §10's operative sentence contradicting 655 §5.4's fail-closed property:
**(a)** `bare`, `--emit-args N`, `--verify-log N LOG` byte-identical · **(b)** every arity /
unknown-mode / bad-argument path byte-identical **modulo the USAGE block**, with the new `USAGE`
asserted to name all six new flags (R298.2 — no usage-message gap exists, none is filed) ·
**(c)** the flagged real call sites rc 0, reproducing base's `scope examined:` lines
byte-identically **modulo one accounted line-number delta** — the matrix moved `:115` → `:125`,
**exactly the +10 lines block (C) inserts above it** (concurrency 3→9, assert call 1→5) — plus
**one added line**, the new assertions' own scope, **KEPT by R299.1** because suppressing a new
check's own report to win identity would make the new checks unreportable · **(d)** the bare
`--assert-workflow FILE` now a **named fail-closed refusal, rc 1**, listing all three missing
required arguments.

**G4 — the ratchet, both ways on both arms.** `1000 90 push` quiet rc 0 (18.5 %) · `4400 90 push`
**WARN** rc 0 (81.5 %) · `4900 90 push` **WARN naming the suppressed FAIL** rc 0 (90.7 %) ·
`4900 90 pull_request` **FAIL rc 1** · omit `--ratchet-arm` **rc 1**. WF-0076's 80/90 thresholds
are unaltered; only *which arm* may turn them into an exit code changes, because the repair for
suite GROWTH is a `workflow_security` PR — the class the admission freeze refuses. **Alarm and
freeze must not share a trigger.**
**V.1 proven by decisive combination:** against one deliberately corrupted manifest the bare mode
returns **rc 1** while `--ratchet` returns **rc 0** — it genuinely short-circuits ahead of the
census/manifest read, so a runtime alarm is never coupled to manifest health.

**G6 — the mandatory parse and selftest gate.** `actionlint` is **ABSENT** on this box, so
`yaml.safe_load` is the **PRIMARY** arm, not a fallback. All five edited workflows parse to
mappings; **14/14 workflows in the tree parse** (13 at base + the new file). RED arm proven on a
deliberately corrupted copy ⇒ `yaml.YAMLError`. ⚠ **The gate asserts NOTHING about specific keys:
`yaml.safe_load` parses `on:` as the boolean `True` (YAML 1.1), reproduced on all five — a gate
asserting an `"on"` key would be a FALSE-RED on every one of them.**
`selftest-timeout-resilience` against the edited gate ⇒ **rc 0, 82 stdout + 6 stderr = 88 lines,
exactly the baseline shape**, now exercising the NEW check names 30 times with **zero** old-name
references — the coupling A1 §8.3 identified, proven to have followed the rename.
⚠ **The subcommand is `selftest-timeout-resilience`; `run-timeout-resilience-selftest` is the
FUNCTION name and exits 2 with `invalid choice`.** Both measured.

## 6. SR-18 BLAST RADIUS — reproduced exactly, zero live sites outside the edit set

| needle | repo-wide | live sites |
|---|---|---|
| `qsc_linux_full_suite` (job key) | 11 in 6 files | **2** — `ci.yml:371`, `public-ci.yml:555` |
| `macos_qsc_full_serial` (job key) | **3** in 3 files | **2** — `macos-build.yml:99`, `public-ci.yml:554` |
| `qsc-linux-full-suite` (check name) | 408 in 75 files | **12** in 5 files |
| `macos-qsc-full-serial` (check name) | 382 in 64 files | **14** in 4 files |

All four match the directive's census exactly, including the per-file live distribution. The
remainder is immutable history in the record monoliths; **rewriting it to make a needle green is
the move NA-0691 refused.** `public-ci.yml:524` naming `qsc-linux-full-suite` is the ENG-0052
incident narrative — history, deliberately NOT edited.

## 7. WHAT WENT WRONG, AND WHAT CAUGHT IT

**Three defects in the governing set were found at execution, and every one was caught by an
instrument rather than by reading.**

1. **A contradiction between two RULED gates (STOP 005 → R298).** G2b froze
   `--assert-workflow FILE` at its base **rc 0**; 655 §5.4 required that exact invocation to
   **fail closed**. No implementation satisfies both, and S16 fires either way. It surfaced only
   because the G2b baselines were **actually captured before the edit and compared against the new
   contract**. R298 adopted Reading B, clarified G2b into four clauses over twelve shapes, and
   **recorded the contradiction as the Director's own miss** — R296.3 ruled G2b in by its short
   name without checking that name against A1 §10's operative sentence.
2. **An SR-21 instrument-scope error of my own, twice.** (i) A positive control for the
   directive-number sweep was scoped to the four record files, while the fixture it targets lives
   in `inputs/local_ops/response_history_catalog_fixtures/`; repo-wide it returns rc 0 / 2 hits.
   (ii) **A `--ratchet` needle matched PROSE** — the macOS workflow's own header comment says its
   `timeout-minutes` is *"bound to the `--ratchet` argument below"* — so the gate reported
   "found 2" and **failed a correct file**. Tree right, needle wider than its claim both times.
   The needle was rebuilt from the invocation's own bytes (a non-comment line that actually runs
   this script). **My own two-sided gate caught its author** (R299.4).
3. ⚠ **A NAMED NEAR-MISS (R299.5): the branch name is load-bearing.** The macOS PR guard is
   `startsWith(github.head_ref, 'na0724-')`. The seat clone's checked-out branch was **`mainwork`**
   — not `main`, and not `na0724-*`. A PR opened from it would have evaluated the guard **FALSE**
   and **silently skipped the macOS shard and aggregate jobs**, killing G3's macOS red control —
   the exact capability F3's cure exists to create, lost with no red anywhere. Caught before the
   branch was cut. **The successor that removes the now-dead `if:` clause (A2 §8.4) also owes this
   branch-name coupling.**

4. ⚠⚠ **A macOS FLAKE SURFACED ON THE LANE'S OWN PR, AND IT WAS NOT RE-RUN (ENG-0187, R300).**
   `macos-qsc-shard-2` went red on `relay_auth_uses_account_token_file_when_env_missing` — a test
   this lane never touched — with `code=relay_inbox_bad_request`. **The failed job was preserved,
   not replayed** (ENG-0091's precedent: *the lane STOPPED on the red rather than rerunning it*),
   and both the red and the green logs are banked 444 in the lane record.
   **One failure in three macOS sharded runs; every other context measured is green** — the same
   shard on the G3 run, the same commit on Linux, and four banked macOS *serial* runs that reached
   the binary (4/4 tests ok each). The seven red-era macOS logs are **silent, not exonerating**:
   they died in the lib unit tests before any integration target ran.
   **R300.2 ordered ONE predicted experiment**, not a rate hunt: a byte-identical tree on a new
   head, the prediction sealed before the branch existed, with the *failing* arm as the informative
   one. The test passed. ⚠ **That FAILED TO REFUTE non-determinism; it did not prove independence**,
   and three runs is not a rate. **Do NOT attribute it to §4b row 9** — co-residency was identical
   between the passing and failing runs.
   ⚠⚠ **The consequence is accepted knowingly:** after this lane, `macos-qsc-sharded-suite` is in
   the watchdog's `--required` list, so **a macOS flake that was previously invisible can red
   MAIN** on a non-docs push, with repair running through §T9's two-step door.

## 8. CLAIM BOUNDARY

**CLAIMED, and only these:** `MAX_CEILING` **270** and `COVERAGE` **330 ≤ 340**, under 280, by the
consumer's own execution, with the refusal boundary itself watched at **93 green / 94 red** ·
census coverage exact, both directions, both partitions, one census · every edited workflow parses
and the edited gate script's own selftest passes · push-suite feedback measured **35.2 m** (Linux,
already) against the serial's **224 m** · the `qsc-sharded-suite` census equals
`cargo test -p qsc --locked`'s target set exactly, from `cargo metadata`, both directions.

**NOT CLAIMED:**
- That ENG-0185's **ARCHITECTURE** is fixed. **R282's `workflow_run`-completion redesign stands,
  retained and deferred.** This lane buys arithmetic headroom, not a new architecture.
- That the macOS shard ceiling of 90 is *fitted*, or that queueing was *measured*. The wave model
  is a **ceiling-basis worst-case bound**, not a queueing measurement, and headroom is **10
  minutes, not 100**.
- That suites stop growing. The census grew 131 → 132 in eight days, measured.
- That cross-target co-residency (655 §4b row 9) is covered. **An accepted, named loss:** a test
  that passes only because another target ran first in the same workspace is observable in the
  serial arrangement and in no other. Six agreeing green pairs is corroboration, not proof.
- ⚠ **That the red-main-repair MARKER admission path works after this lane. IT DOES NOT.** The
  profile's `failure_check` now resolves, but its marker condition can never be satisfied against
  an **aggregate's** log, which structurally cannot carry a test name. **That arm is INERT**, is
  filed as **ENG-0186**, and its repair is a gate-logic change this lane's non-goals forbid.
  Repointing the name buys referential integrity and nothing else — a dangling name would be worse.
- ⚠ **That macOS suite growth is ENFORCED.** Post-merge the macOS ratchet can only **WARN** (the
  guard keeps macOS shards off ordinary PRs and the FAIL arm is limited to non-`push` arms), so
  its FAIL arm is exercised exactly once, on this lane's own PR. **The Linux FAIL arm stays live.**
- ⚠ **That the rollback door is available unconditionally.** It is inert in exactly one case — a
  `public-ci.yml` parse break — whose only exit is an operator branch-protection act. **G6 exists
  to make that case unreachable: it is prevention, not a rollback path.**
- ⚠ **That the reserved rollback supply is protected by anything but the record.** Nothing
  mechanical prevents another lane spending PROC-1 or the class-drift filing as its door; S15 is a
  re-measurement, not a lock.
- ⚠ **That the two `if:` guards' GitHub-expression semantics were validated at drafting.** They
  were not — `actionlint` is absent and expressions are evaluated only by GitHub. **G3 and G3b are
  where they become measured.**
- Anything about WF-0073, WF-0074, WF-0075, ENG-0120, or SR-24.

**END OF NA-0724 AS BUILT** — if this line is missing, the copy is truncated.
