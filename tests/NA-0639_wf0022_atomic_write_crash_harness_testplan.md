# NA-0639 — WF-0022 Atomic-Write Crash-Window Harness — Testplan

Goals: G4, G5

Test-only lane (D575, D-1262). The runtime surface IS the deliverable:
`qsl/qsl-client/qsc/tests/NA_0639_wf0022_atomic_write_crash_window.rs`
exercises the real `write_atomic` (`qsl/qsl-client/qsc/src/fs_store/mod.rs`)
through the compiled qsc binary. Production code is untouched.

1. **Scope guard.** `git diff --name-only <base>..HEAD` returns exactly:
   `qsl/qsl-client/qsc/tests/NA_0639_wf0022_atomic_write_crash_window.rs`,
   `docs/governance/evidence/NA-0639_as_built.md`,
   `docs/ops/IMPROVEMENT_LEDGER.md`,
   `tests/NA-0639_wf0022_atomic_write_crash_harness_testplan.md`,
   `NEXT_ACTIONS.md`, `DECISIONS.md`, `TRACEABILITY.md`,
   `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`. Nothing else — in particular no
   `src/**` (including `fs_store/mod.rs`), no `formal/`, no vectors, no
   canonical, no `.github/**`, no Cargo manifest/lock.

2. **Positive, crash window** —
   `wf0022_crash_window_subsequent_reader_sees_old_xor_new`: interrupts the
   real `write_atomic` at the named point (after temp-write + `sync_all`, at
   `fs::rename`, `fs_store/mod.rs:120-122`) by denying directory write
   permission once the deterministic temp (`config.txt.tmp.<pid>`) appears.
   Asserts: subsequent reader sees target byte-identical OLD; temp residue
   holds complete NEW and never became the live target; a recovery run lands
   byte-identical NEW. The test FAILS if no trial (of 25) demonstrably lands
   in-window — the exercise cannot silently not-fire. **Local result: ok.**

3. **Positive, concurrent reader** —
   `wf0022_concurrent_reader_sees_old_xor_new_across_real_writes`: a sampling
   reader thread across 24 real `write_atomic` cycles; every sample must
   classify exactly-OLD or exactly-NEW via the shared classifier.
   **Local result: ok** (zero torn samples).

4. **Non-vacuity negative controls (WF-0017)** — both use a TEST-LOCAL
   non-atomic writer (`torn_write_inplace`: truncate in place, sync, pause,
   write), NOT a production change:
   - `wf0022_negative_control_inplace_crash_is_detected_as_torn`: half-written
     target MUST classify Torn. **Local result: ok (trips).**
   - `wf0022_negative_control_concurrent_reader_catches_inplace_writer`: the
     same reader loop MUST catch ≥1 torn sample against the held-open torn
     window. **Local result: ok (trips).**

5. **Red-run demonstration (recorded, not committed).** The positive
   concurrent test, temporarily pointed at the non-atomic writer, FAILED:
   `concurrent reader observed torn content across real write_atomic cycles
   (36961 of 45033 samples): ["", "", ...]` — the harness demonstrably goes
   red when atomicity is violated. The edit was reverted; the committed file
   drives the real binary.

6. **Stability.** 10 consecutive local runs of the 4-test harness: 40/40
   green, ~0.2 s per run; the interruption landed on the first trial in every
   run.

7. **Full local suite (the real merge gate — `qsc-linux-full-suite` SKIPS on
   PRs).** `cargo test -p qsc` run locally on the lane branch: result
   recorded in the lane's final response and PR body; merge only on green.

8. **Ledger integrity.** WF-0022's status line records closure by NA-0639
   (D-1262, D575), cites the harness file, states the simulation limits, and
   carries the residue note (claim-7 marker case + per-seam fault matrix stay
   ON-DECK 0c residue, per the D575 scope note in NEXT_ACTIONS).

9. **Queue integrity.** After closeout: `STATE: READY=NONE | HIGHEST_NA=0639
   | HIGHEST_D=1262`; zero anchored `^Status: READY` lines; the NA-0639 block
   is `Status: DONE` with the OUTCOME; NA-0635 stays RESERVED.
