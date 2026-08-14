# NA-0724 — SUITE-ECONOMICS TESTPLAN (D-1360)

**Lane:** NA-0724 · **Class:** `SUITE_ECONOMICS_SHARDING_PASS` · **Base:** `f8370bce`
**Discipline:** every expectation is written and banked **444 BEFORE** its instrument runs; every
gating command is run **BARE and UNPIPED** (`cmd > out 2> err; echo $?`), never `cmd | tail`,
because a pipe reports the *pipe's* status and a RED suite would show exit 0. **A silent skip is a
vacuous pass.** No check state is read until its run reports **SETTLED**.
**Sealed before the fact:** `SEAL_PHASE0`, `SEAL_PHASE1`, `SEAL_PHASE2_GATES` and its addendum
`_A1` (the clarified G2b), all frozen 444 in the lane record before their instruments ran.

---

## G1 — THE CONSUMER'S OWN ARITHMETIC

**Why executed rather than PR-validated:** `public-ci.yml` fires on `pull_request_target`, so a PR
runs **main's** copy, and its sizing step is `if: github.event_name == 'push'`. **No PR can ever
execute it** (WF-0073 / 655 DV-1). The instrument is therefore the consumer itself: constants and
`ceiling_of()` lifted **byte-for-byte** from the tree and run bare.

| id | input | expected | rc | result |
|---|---|---|---|---|
| **G1b** | macOS shard ceiling 150 — **RUN BEFORE G1a** | `ERROR: derived watchdog coverage 510m exceeds the safe bound 340m` | 2 | ✅ |
| **G1b′** | macOS ceiling **94** | `COVERAGE 342`, ERROR naming 342 | 2 | ✅ |
| **G1b′** | macOS ceiling **93** | `COVERAGE 339`, **no ERROR line** | 0 | ✅ |
| **G1a** | the real edited tree | `MAX_CEILING 270` · `COVERAGE 330` · `SAFE_BOUND 340` · `--max-iterations 990` | 0 | ✅ |
| **G1c** | either aggregate job | `<EMPTY>` + named error | 2 | ✅ |
| **G1d** | both workflows, job `shard` | shell `awk` == Python parse | 0 | ✅ |

**G1b runs BEFORE G1a: a guard that has not been watched REFUSING is not a guard.**
**SR-19 delta symbol:** `timeout-minutes` under the `shard:` job key of
`.github/workflows/macos-qsc-sharded-suite.yml`. Change that one literal and G1 flips red↔green.

## G2 — MANIFEST COVERAGE AND THE NEW ARGUMENTS

**Green:** Linux manifest (default) rc 0 · `--manifest …_MACOS.txt` rc 0 · both
`--assert-workflow` call sites with their real arguments rc 0.
**Red — twelve arms, each WATCHED, none assumed:**

| id | input | expected |
|---|---|---|
| R1 / R2 | one row deleted from the Linux / macOS manifest | `FAIL: MISSING from manifest (present in tree): …`, rc 1 |
| R3a / R3b | `doc:qsc` given a co-tenant | the named doc-co-tenancy FAIL, rc 1, on **both** the default path and `--emit-args` |
| R4 | omit `--expect-runners` | rc non-zero — **no default, fails closed** |
| R5 | `--expect-runners macos-latest-xlarge` | rc non-zero — outside the hard-coded `STANDARD_RUNNERS` allowlist |
| R6 | omit `--expect-job-runner` | rc non-zero — no default |
| R7 | `--expect-job-runner shard=ubuntu-latest` vs the macOS file | rc non-zero |
| R8 | **the F8 slip:** macOS file with `shard.runs-on: ubuntu-latest` | rc non-zero |
| R9 | **the F13 form:** `shard` using block-sequence `runs-on:` | rc non-zero — *"has no runner this gate can SEE"* |
| R10 | **the F14 binding:** `--ratchet … 75` vs `timeout-minutes: 90` | rc non-zero |
| R11 | `--max-shards 4` against the K=5 macOS file | rc non-zero |

**SR-19 delta symbol:** a manifest row's target name.

## G2b — CALL-SITE INVARIANCE (clarified at R298.1)

The script has **no argparse**: `main()` is a hand-rolled positional dispatcher, and six new flags
force its restructure. Baselines were captured from the **UNEDITED** script before any edit.

- **(a)** `bare`, `--emit-args N`, `--verify-log N LOG` — byte-identical stdout/stderr/rc. ✅
- **(b)** every arity / unknown-mode / bad-argument path — byte-identical **modulo the USAGE
  block**, with the new `USAGE` asserted to name all six new flags (R298.2). ✅
- **(c)** the flagged real call sites — rc 0, base's `scope examined:` lines reproduced
  byte-identically modulo one **accounted** line-number delta (+10, exactly what block (C) inserts
  above the matrix), **plus one added line: the new assertions' own scope, KEPT by R299.1.** ✅
- **(d)** bare `--assert-workflow FILE` — a **named fail-closed refusal**, rc non-zero. ✅

## G3 — THE ARRANGEMENT WATCHED RED ON A GENUINE TEST FAILURE

Branch **`na0724-g3-red-control`** (the `na0724-` prefix is what admits it past the macOS PR
guard). One deliberate `assert!(false)` in a qsc test the **macOS partition assigns to a KNOWN
shard**. **All four observations together:** that shard's job **FAILS** · `--verify-log` **still
reconciles by name** · the **aggregate concludes `failure`** · **sibling shards still PASS**
(`--no-fail-fast` + `fail-fast: false`). Then **revert and prove the revert by `cmp` against base.**
⚠ **SR-06 binds:** the EXACT red set is committed in writing before the control runs; **one edit
per control; cmp-identical restore; a one-test deviation is a STOP.**
⚠ **S8: two `na0724-*` PRs must never be open simultaneously** — the red control runs ONCE, so this
PR is opened and closed before the lane PR.
**SR-19 delta symbol:** the `assert!` expression in the seeded test.

## G3b — THE PR GUARD'S **FALSE** ARM

From a branch whose name does **NOT** start with `na0724-`, carrying the new workflow, one trivial
draft PR. **Expected:** `macos-qsc-shard-0..4` and `macos-qsc-sharded-suite` **SKIPPED / never
created**; `classify-macos-qsc-sharded-scope` and `macos-qsc-shard-manifest-gate` still run and
pass; **ZERO macOS jobs consumed. Read the run's JOB LIST, not the PR's green.**
⚠ Without G3b, *"FALSE for ordinary PRs"* is asserted, not proven — and the guards'
GitHub-expression semantics were **explicitly not validated at drafting** (`actionlint` absent;
expressions are evaluated only by GitHub). G3/G3b are where they become measured.

## G4 — THE RATCHET, BOTH WAYS ON BOTH ARMS

`1000 90 push` quiet rc 0 (18.5 %) · `4400 90 push` **WARN** rc 0 (81.5 %) · `4900 90 push`
**WARN naming the suppressed FAIL** rc 0 (90.7 %) · `4900 90 pull_request` **FAIL rc 1** · omit
`--ratchet-arm` rc non-zero. WF-0076's 80/90 thresholds unaltered.
**V.1:** against a deliberately corrupted manifest, bare mode rc 1 while `--ratchet` rc 0 — the
short-circuit proven by decisive combination, not asserted.
**SR-19 delta symbol:** the `SECONDS` argument.

## G5 — HOUSE GATES

`goal-lint` locally with a synthesized `GITHUB_EVENT_PATH` — ⚠ **the `Goals:` line must be in the
PR BODY; nowhere else satisfies it** · `public-safety` on the PR · `infra-literal-scan` · the full
required set **SETTLED** before any claim · `git diff --cached --name-only` **contains the
force-added evidence file** (`docs/governance/evidence/**` is gitignored, so `git add -f`, and
`git status` will never list it) · the SR-18 census needles re-run post-edit against counts stated
in advance.

## G6 — THE MANDATORY PARSE AND SELFTEST GATE

**G6a:** `yaml.safe_load` over all five edited workflows ⇒ each returns a mapping (14/14 in the
tree). `actionlint` additionally **if present** — measured **ABSENT**, so `yaml.safe_load` is the
**PRIMARY** arm. RED arm proven on a corrupted copy ⇒ `yaml.YAMLError`.
⚠⚠ **ASSERT NOTHING ABOUT SPECIFIC KEYS. `yaml.safe_load` parses `on:` as the boolean `True`
(YAML 1.1) — a gate asserting an `"on"` key is a FALSE-RED on every workflow in this repo.**
**G6b:** `python3 scripts/ci/public_safety_gate.py selftest-timeout-resilience` against the EDITED
gate ⇒ rc 0 (baseline at base: rc 0, 82 stdout + 6 stderr = 88 lines).
⚠ **That is the CLI subcommand. `run-timeout-resilience-selftest` is the FUNCTION name and exits 2
with `invalid choice`.**
**Rationale:** a parse break reaching main means `public-ci` never starts for any event ⇒ the
REQUIRED `public-safety` context is never minted ⇒ **every PR waits forever, including the reserved
rollback door**, and docs-door-then-revert fails precisely when the break is in `public-ci.yml`
itself. **G6 is prevention, not a rollback path.**
**SR-19 delta symbol:** one character of `public-ci.yml`'s YAML indentation.

## POST-MERGE OBSERVATIONS (named, and NOT claimed in advance)

On the first non-docs-only main push after merge: (i) the sizing block's printed output shows
`x fan-out waves : 3`, `binding ceiling : 270m`, `coverage : 330m (safe bound 340m)`,
`--max-iterations : 990` · (ii) `public-safety` reaches the poll instead of refusing · (iii) both
aggregate checks conclude · (iv) each macOS shard's `runner host:` line, measured wall-clock and
queue delay are recorded — **the input to the ceiling re-fit this lane does not perform.**
⚠ **Watch the SUITE check-runs directly, never the wrapper** (ENG-0052's false-red precedent).
⚠ **The first macOS run has a COLD cache and may exceed the 60-minute band while HEALTHY** — S5 is
first-run-aware and that is recorded, not a finding.
⚠ **If `wait_for_required_checks` returns rc 2, the three signatures discriminate:**
`transient_poll_errors=0` **and** final iteration `status=in_progress` = genuine wave exhaustion
⇒ R282's redesign becomes NECESSARY; `transient_poll_errors > 0` = API flake; `CHECK …: missing` =
a trigger defect. **Only the first is the escalation trigger.**

**END OF NA-0724 TESTPLAN** — if this line is missing, the copy is truncated.
