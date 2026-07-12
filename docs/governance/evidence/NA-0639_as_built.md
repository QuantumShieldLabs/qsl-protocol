# NA-0639 As-Built — WF-0022 Atomic-Write Crash-Window Harness

Goals: G4, G5
Lane: NA-0639 (directive QSL-DIR-2026-07-12-575, D575, APPROVED; decision D-1262)
Date: 2026-07-12
Base main: `818618ce`
Result class: **WF0022_ATOMIC_WRITE_CRASH_HARNESS_PASS**

## 1. What this lane did

Built and ran the settling exercise WF-0022 named: a test harness that
EXERCISES the crash-window atomicity of the real `write_atomic`
(`qsl/qsl-client/qsc/src/fs_store/mod.rs:94-125`) and demonstrates its own
non-vacuity (WF-0017). Closed WF-0022 on the ledger. Zero production-source
change — `write_atomic` was read, driven, and interrupted; never edited.

Harness file (the only non-governance artifact):
`qsl/qsl-client/qsc/tests/NA_0639_wf0022_atomic_write_crash_window.rs` — four
tests, all green locally; the two negative controls trip as required.

## 2. Harness form — chosen at Phase 1, with rationale (D575 requirement)

**Chosen: the PREFERRED form — an in-suite Rust integration test**
(`qsc/tests/*.rs`, matching the NA_0633/NA_0634 precedent; runs under the
same `cargo test -p qsc` gate; no new tooling, no new dependencies).

Phase-1 findings that shaped it:

- `write_atomic` is `pub(crate)`. The qsc lib target exports only
  `adversarial` (binding-fuzz surface) and `envelope`; there is NO in-process
  call path from an integration test, and creating one (a re-export, a
  `#[cfg(test)]` module in `fs_store/mod.rs`, a Cargo.toml dev-dependency)
  would edit production source or manifests — Tier-5 forbidden.
- The suite's established driver for fs_store behavior is the REAL compiled
  `qsc` binary via `assert_cmd` (`tests/fs_store_contract_na0217b.rs`).
  `qsc config set policy-profile baseline|strict` reaches the real
  `write_atomic` on `<QSC_CONFIG_DIR>/config.txt` via `write_config_atomic`
  (`main.rs` `config_set` → `fs_store/mod.rs:75-82`), with byte-exact
  old/new contents (`policy_profile=baseline\n` ↔ `policy_profile=strict\n`),
  and — once the store layout exists — performs EXACTLY ONE `write_atomic`
  (one rename) per invocation. The store-meta write is seeded first so the
  measured run has a single window.
- The FALLBACK (standalone fault-injection harness / syscall interposer) was
  NOT needed: the crash window is reachable in-suite because `write_atomic`'s
  temp name is deterministic (`<file>.tmp.<pid>`, `fs_store/mod.rs:105-110`)
  and its rename — unlike its in-flight temp-write — requires directory write
  permission. No strace/seccomp/LD_PRELOAD tooling was introduced.

## 3. The invariant and the interruption model (D575 requirement)

**Invariant (OLD-XOR-NEW):** a reader of the target path always observes the
complete old content or the complete new content — never a torn/partial
write; the temp file is never observed as the live target.

**Named interruption point:** between `write_atomic`'s completed temp-write
(`write_all` + `sync_all`, `fs_store/mod.rs:120-121`) and its
`fs::rename(&tmp_path, path)` (`fs_store/mod.rs:122`).

**Mechanism (simulated crash):** the parent test spawns the real binary
writing NEW, polls for the deterministic temp file
(`config.txt.tmp.<child-pid>`), and on its appearance removes WRITE
permission from the store directory (0o700 → 0o500). The child already holds
an open fd, so its `write_all` + `sync_all` complete without needing
directory write; its `rename` is then DENIED. The child aborts
(`IoWriteFailed`, nonzero exit), leaving on disk exactly the state a crash
immediately before the rename would leave: target = OLD (intact),
temp = NEW (complete), rename never applied. Determinism: `sync_all` is a
real fsync barrier (milliseconds) while the parent's detect→chmod is
microseconds; a bounded retry loop (25 trials) covers the residual race, and
the test FAILS if no trial demonstrably lands in-window — the harness cannot
pass without the exercise actually firing.

**Observations asserted (positive tests):**
1. `wf0022_crash_window_subsequent_reader_sees_old_xor_new` — after the
   in-window interruption: a subsequent reader sees the target byte-identical
   to OLD; the temp residue exists BESIDE the target holding complete NEW
   (never the live target); after cleanup, a re-run lands byte-identical NEW
   (the NEW arm, post-rename).
2. `wf0022_concurrent_reader_sees_old_xor_new_across_real_writes` — a reader
   thread samples the target continuously across 24 REAL `write_atomic`
   cycles (baseline↔strict through the binary); every sample classifies as
   exactly-OLD or exactly-NEW, never Torn (rename-visibility atomicity under
   a concurrent reader).

Both go through ONE shared classifier (`classify: content → Old|New|Torn`);
the negative controls use the same classifier, so a defect in it would
surface on the negative side.

## 4. Non-vacuity — WF-0017 (D575 requirement)

The deliberately non-atomic path is a TEST-LOCAL helper
(`torn_write_inplace`: open target with truncate, sync the truncated state,
pause, write) — NOT a change to production `write_atomic`.

- `wf0022_negative_control_inplace_crash_is_detected_as_torn` — a simulated
  crash mid in-place write (half of NEW written over the target): the SAME
  classifier MUST return Torn. **Trips as required (green = detection
  demonstrated).**
- `wf0022_negative_control_concurrent_reader_catches_inplace_writer` — the
  SAME reader loop against the in-place writer (torn window held open 5 ms)
  MUST observe ≥1 torn sample. **Trips as required.**
- **Red-run demonstration (temporary edit, reverted before commit):** the
  POSITIVE concurrent test was pointed at the non-atomic writer; it FAILED
  with `concurrent reader observed torn content across real write_atomic
  cycles (36961 of 45033 samples): ["", "", ...]` — the positive assertions
  demonstrably go red when atomicity is violated. The committed file contains
  the real-binary version; the demo output is recorded here and in the
  testplan.

## 5. Results (local, Linux, 2026-07-12)

```
running 4 tests
test wf0022_negative_control_inplace_crash_is_detected_as_torn ... ok
test wf0022_crash_window_subsequent_reader_sees_old_xor_new ... ok
test wf0022_negative_control_concurrent_reader_catches_inplace_writer ... ok
test wf0022_concurrent_reader_sees_old_xor_new_across_real_writes ... ok
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

Stability: 10 consecutive full-harness runs, 40/40 green, ~0.2 s each (the
interruption landed on the first trial every run). Full local
`cargo test -p qsc` suite run recorded in the testplan (the real merge gate —
`qsc-linux-full-suite` SKIPS on PRs).

**No real OLD-XOR-NEW violation was found**: `write_atomic` held the
invariant in every exercised trial. No ENG filing was warranted.

## 6. Simulation limits (do not overclaim — D575 requirement)

A PASS asserts the OLD-XOR-NEW property under THIS simulated interruption
model only:

- The "crash" is a DENIED RENAME at the directory-permission seam (plus child
  abort), not a kernel panic or power loss. The on-disk window state is
  equivalent (target old, temp complete, rename absent), but nothing here
  exercises page-cache loss, reordered/lying fsync, partial sector writes, or
  filesystem-specific rename durability (the rename-before-dir-fsync window).
- The interruption lands AFTER the temp-write completes (the fd needs no
  directory permission), i.e., at the sync→rename boundary. Kills DURING
  `write_all`/`sync_all` (partial temp) and per-seam fault matrices are NOT
  exercised — by construction the temp is complete when the rename is denied.
- Unix-only (`#![cfg(unix)]`), matching the suite's existing platform gates.
- The claim-7 residue (store-stage fault ⇒ no `handshake_complete` marker
  without a committed session) is NOT in this lane (D575 scope note); it
  remains ON-DECK 0c residue with the per-seam fault matrix.

Claim boundary UNCHANGED beyond substantiating this property: no new security
claim, no disclaimer weakened. ENG-0003/0004 stay closed.

## 7. Scope proof

Changed paths (single PR): the harness file (new, test-only);
`docs/ops/IMPROVEMENT_LEDGER.md` (WF-0022 → done); this evidence doc;
`tests/NA-0639_wf0022_atomic_write_crash_harness_testplan.md`;
`NEXT_ACTIONS.md`; `DECISIONS.md` (D-1262); `TRACEABILITY.md`;
`docs/ops/ROLLING_OPERATIONS_JOURNAL.md`. NO production source (including
`fs_store/mod.rs`), no `formal/`, no vectors, no canonical, no `.github/`,
no server, no attachments, no Cargo.toml/lock.
