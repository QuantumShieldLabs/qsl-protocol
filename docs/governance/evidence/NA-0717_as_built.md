# NA-0717 — AS BUILT — THE MACOS LOCK-REGISTRY ERRNO FIX (D-1353, directive 653 + AMENDMENT A1)

Base spine main **`5b43eefe`** (verified against the **named `github` remote**, bare and
unpiped, at drafting AND re-verified at execution; unmoved between them). Two branches per
R269: **PR-A records** `na0717-macos-lock-registry-records` (this commit's PR) and **PR-B
impl** `na0717-macos-lock-registry-errno` (carries the mod.rs edit; opens after PR-A
merges, taking post-PR-A main by MERGE). Governing texts: 653 (sha `236ed846…d130`, 233 ln,
sealed, never rewritten) + A1 (sha `879f0f54…2a0f8`, 392 ln) — **A1 governs on conflict**;
rulings R264–R274. This file is written at PR-A time: Phase 0 and Phase 1 are complete and
measured; Phases 2B–7 (PR-A merge watch, PR-B, the dispatch pair, the merge watch,
closeout) are PLANNED here and their results land in PR-B's evidence note
(`tests/NA_0717_lock_registry_errno_platform_evidence.md`, R272) and the lane's stop-files.

## 1. THE DEFECT AND THE DIAGNOSIS (C-2, ratified R264)

The four `model::na0696_lock_registry_tests` denial tests assert
`(rc, errno) == (-1, 11)` on raw `flock` probes made on a SECOND open file description.
Literal 11 is EWOULDBLOCK/EAGAIN **on Linux only**; on Darwin EWOULDBLOCK == EAGAIN ==
**35** (11 is EDEADLK). Measured across five failing runs plus a fresh log pull (job
93971504692, image macos-26-arm64): every denial-expecting probe that EXECUTED on macOS
returned `(-1, 35)` — the denial the tests demand, spelled in the platform's own integer.
`upgrade_refused_fail_closed`'s own compound message prints
`wrong_error=None depth_after=1 probe_rc=-1 probe_errno=35`: every conjunct green except
the literal. Exclusion, depth, drop-order and unwind-restore all HELD on macOS. Production
is portable (`LockGuard::acquire` classifies via `ErrorKind::WouldBlock`, mod.rs:161-166)
and untouched by this lane.

Scope honesty (A1 §6.1): each failing test dies at its first failing assert, so the sites
at base :295/:324 have never executed on macOS — Phase 4b is their first execution, caught
by the by-name PASS seal. The failure-log instrument is structurally blind to silent
acquisitions; no denial-expecting probe was OBSERVED acquiring (`left: (0, _)` appears in
no log), and the :375 co-holder probe is DESIGNED to acquire and provably did.

- **C-1 (regression): dead by direct enumeration.** The serial job executed on exactly five
  main pushes since the tests landed — a0b18d66 (2026-08-05, the introduction merge
  itself) · 3fcda47d · 6680a468 · b845e678 · 5b43eefe — ALL failing with the identical
  four-test signature; every other push classified docs-only and SKIPPED the job. Zero
  green ever; there is no last-green sha. The event-unfiltered enumeration
  (MACOS_RUN_ENUM.log, banked at execution) shows ZERO dispatches in-window and five
  pre-introduction Aug-5 push rows PROVEN INERT (zero `na0696_lock_registry_tests`
  occurrences at each sha; all ancestors of a0b18d66) ⇒ "the tests have NEVER passed on
  macOS" stands outright.
- **Both platforms at one sha:** Linux lib target at 5b43eefe = 122 passed / 0 failed with
  the four tests ok by name; macOS at the same sha = 118/4 with the errno signature. Same
  code, opposite outcomes, explained by one integer.

## 2. THE EDIT (653 §4 as amended by R273)

Inside `#[cfg(test)] mod na0696_lock_registry_tests` ONLY — zero production bytes:

    #[cfg(target_os = "linux")]
    const EWOULDBLOCK_RAW: i32 = 11;
    #[cfg(target_os = "macos")]
    const EWOULDBLOCK_RAW: i32 = 35;

with the 653 §4(a) comment block above it, and the six literal sites re-pointed at the
const (base :290/:295/:321/:324/:347/:394). **The assertion is NOT weakened: rc == -1 AND
errno == the platform's would-block value, exact per platform.** An unlisted `target_os`
fails to COMPILE at the six use sites — fail-closed by construction (and NOTE-1: base
already encoded Linux silently for unlisted Unix targets; post-fix that silence becomes a
compile refusal). `ErrorKind::WouldBlock` mapping was considered and set aside as WIDER
than the claim (SR-21).

**Byte form (NEW-2 → R273).** 653 sealed both §4(b)'s exact one-line site forms AND
`cargo fmt --check` clean on the file; measured at execution they are JOINTLY
UNSATISFIABLE — the base file is fmt-clean under the same instrument (rustfmt
1.9.0-stable, no repo rustfmt config, no CI fmt gate), and the file with §4(b)'s exact
bytes fails: rustfmt rewraps 4 of the 6 sites. R273 adopted the CANONICAL form (STOP_003
§4 is the byte authority; the insert and the two short sites keep §4(b)'s exact bytes).
**Mechanism record, as the Director measured and ordered stated (R273): the symbol is 13
chars longer than the literal (len 15 vs 2), not 14 as first reported; after the swap only
the :394 site crosses max_width=100 (75→88, 80→93, 81→94, 93→106 measured at base); the
operative trigger for the other three sites is an INNER rustfmt call-width heuristic whose
EXACT RULE IS UNMEASURED — the governing evidence is the measured rc chain (base 0 ·
sealed-form 1 · canonical 0) plus the four-hunk diff, not any width model.** The delta is
four whitespace-only hunks, ZERO token changes.

## 3. PROOFS AT THE SHIPPED BYTES (Phase 1, re-proven per R273 after the ruling)

| proof | result |
|---|---|
| needle `(-1, 11)` | 5 → 0 |
| needle `probe_errno == 11` | 1 → 0 |
| needle `EWOULDBLOCK_RAW` | 0 → 8 (2 defs + 6 uses) |
| needle `(-1, EWOULDBLOCK_RAW)` | 0 → 5 |
| needle `probe_errno == EWOULDBLOCK_RAW` | 0 → 1 |
| cfg-adjacency (A1 MINOR-4): linux→`= 11;` / macos→`= 35;` | 0 → 1 each (base `target_os` count in mod.rs = 0; a transposed insert now fails the TABLE) |
| `rustfmt --check` on the file | base rc=0 · §4(b)-sealed form rc=1 · shipped canonical form rc=0 |
| path-scoped untouched-proof | `git log 4fb7fe89..HEAD -- qsl/qsl-client/qsc/src/model/mod.rs` EMPTY before this lane's edit (MINOR-1's instrument) |
| Linux by-name run (shipped bytes) | exit 0; `test result: ok. 122 passed; 0 failed`; sorted pass-name set IDENTICAL to BASELINE_LINUX_LIB.log — the fix moves nothing on Linux |
| clippy | not configured in CI (zero matches in `.github/workflows/`); 653's "per validation defaults if configured" resolves to no-op |
| diff scope | one file, mod.rs, +32/−6, all inside the test module |

## 2b. WHAT SHIPS WHERE (R269 + R272)

- **PR-A (this PR, docs-only by construction):** `DECISIONS.md` `## D-1353` ·
  `TRACEABILITY.md` dated G4 row (gains its impl-PR pointer in PR-B) · `NEXT_ACTIONS.md`
  `### NA-0717` block with `Status: MERGING (PR #PRNUM)` placeholder (the NA-0716/#1727
  shape; the flip to the real number is PR-B's authorized second commit) + STATE advance to
  `HIGHEST_NA=0717 | HIGHEST_D=1353` · ledger filings ENG-0184 / WF-0073 / WF-0074 /
  WF-0075 · prediction rows 1–6 · this file (gitignored path, `git add -f`, presence
  proven in the staged name list). Pre-open gate: `classify_ci_scope.sh` run over the
  exact name-only diff, expected and required `docs_only=true`; sealed expectation on the
  PR: the gate-invocation step reads SKIPPED from the step list.
- **PR-B (impl):** the mod.rs commit (already authored) · the MERGING flip naming PR-B's
  number · `tests/NA_0717_lock_registry_errno_platform_evidence.md` (R272: the platform
  evidence summary incl. the 4a/4b dispatch results once measured) · one-line D-1353 and
  TRACEABILITY addenda naming the impl PR number. Growth beyond this is a STOP (SR-02).

## 4. THE WALL, THE DOOR, AND WHY TWO PRS (BLOCKER-1 → R268 RESCIND → R269)

653 §6 claimed "NO PR is admissible by ANY path while main is red." The three-admission-
paths half is TRUE for runtime PRs (all three refuse: bootstrap and advisory-remediation
need main's `advisories` FAILING — D-1350 made it green; red-main-repair's only profile
does not describe the lock-test failure). The leap was FALSE: the gate's first check is
invoked only for runtime_critical/workflow_security PRs (public-ci.yml:476-477), and a
docs-only push's public-safety check-run completes GREEN unconditionally (:505-506,
:592-597), re-greening main HEAD for the first check (public_safety_gate.py:1161-1176).
Measured week: 16 of 21 first-parent pushes docs-only; docs PR #1724 re-greened red main
b845e678 and admitted #1726 — this lane's own base. R269: PR-A records (docs-only by
construction) merges NORMALLY and re-greens the signal; PR-B impl follows against a green
first check; NO bypass, the door used once, on record, FILED (WF-0074). Sequencing hold:
#1723/#1725/#1727 wait for truly-green main after PR-B's merge push (R260 §3.5 order).

## 5. EXECUTION FINDINGS BEYOND THE BRIEF (each ruled before any push)

1. **NEW-1 (R272):** `tools/goal_lint.py:82-88/:90-93` fails BOTH the ruled PR-B shape and
   653's original single-PR shape — an in-lib `#[cfg(test)]` edit is invisible to the
   tests/-path heuristic, so 653 Phase 3's sealed "goal-lint PASS" was unsatisfiable as
   written and three chairs missed it. Cure: the repo's own tests/-md evidence convention
   + one-line addenda, in PR-B. Blind spot filed as WF-0075.
2. **NEW-2 (R273):** the fmt-seal/byte-form conflict of §2 above — the second
   consumer-never-executed-against-its-sealed-shape defect in one lane (prediction rows 5
   and 6 carry the lesson).
3. **GH007 at the seat:** the clone's commit identity was a personal address rather
   than the repo noreply identity; corrected clone-local to the repo noreply identity
   BEFORE the first commit, per 653 Phase 0's own order.
4. **LINUX_SUITE_HISTORY.log (A1 MAJOR-4):** `qsc-linux-full-suite` is GREEN on all five
   red pushes (~3h40m per run) — main's ONLY red is the macOS serial suite, which
   strengthens (but does not discharge) Phase 6's conditional public-safety seal.

## 6. CLAIM BOUNDARY

Verbatim per A1 §14: the four failures are diagnosed to C-2 with source and five-run log
evidence · the remedy is one test-module edit with exact-per-platform assertion strength
preserved · Linux invariance is proven by-name at base and re-proven at the shipped bytes
(and re-proven again after PR-B's take-main merge) · the docs door's mechanism is
source-verified three times independently at 5b43eefe · PR-A is docs-only by construction,
proven pre-open by the consumer classifier itself. NOT CLAIMED: that the REST of the macOS
serial suite is green — it has not completed since 2026-07-21 and runs for the first time
behind this fix (any new failure is a fresh finding, not this lane's scope) · that any PR
merges — every merge is the operator's act · that main's public-safety greens on a
schedule (it needs BOTH push-only suites green post-merge; qsc-linux-full-suite's color is
not this lane's claim) · that the gate's other defects (the 403/F-2, A′/#1727, WF-0073,
ENG-0182's marker substring) are altered · anything about #1723/#1725/#1727's contents ·
that a dispatch result equals a push result (mitigated, not erased, by 4a; toolchain lines
recorded from 4a/4b/Phase-6, any mismatch a NOTED CONFOUND) · that notification defaults
are measured · that the evidence chain is attestable beyond same-box content hashes ·
that goal-lint's design is right or wrong.
