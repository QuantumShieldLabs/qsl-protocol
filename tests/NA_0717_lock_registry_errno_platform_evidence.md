# NA-0717 — platform evidence: the lock-registry errno fix (D-1353)

Lane evidence note in the repo's tests/-markdown convention (R272; PR-B file 8). The fix:
a per-platform `EWOULDBLOCK_RAW` const inside `#[cfg(test)] mod na0696_lock_registry_tests`,
six literal sites re-pointed; assertion strength preserved exactly (rc == -1 AND errno ==
the platform's would-block value); zero production bytes.

## The diagnosis (C-2, ruled R264)

Literal 11 is EWOULDBLOCK/EAGAIN on Linux only; Darwin returns 35 (11 is EDEADLK). Every
denial-expecting probe that EXECUTED on macOS returned (-1, 35) — the denial the tests
demand, in the platform's own integer. Exclusion, depth, drop-order, unwind-restore all
HELD. Production portable (ErrorKind::WouldBlock, mod.rs:161-166), untouched.

## The five-run history

macos-qsc-full-serial executed on exactly five main pushes since the tests landed —
a0b18d66 (2026-08-05, the introduction merge itself) · 3fcda47d (08-07) · 6680a468
(08-08) · b845e678 (08-10) · 5b43eefe (08-12) — all failing with the identical four-test
signature; every other push classified docs-only and skipped the job. The event-unfiltered
run enumeration (banked in the NA-0717 lane record as MACOS_RUN_ENUM.log) shows zero
workflow dispatches in-window and five pre-introduction Aug-5 push rows proven inert
(zero test-module occurrences at each sha; all ancestors of a0b18d66). The tests never
passed on macOS; C-1 (regression) has an empty suspect set by direct enumeration.

## The one-sha two-platform pair

At 5b43eefe: Linux lib target 122 passed / 0 failed with the four tests ok by name; macOS
118/4 with the errno signature. Same code, opposite outcomes, one integer.

## The needle table (measured base → after, at the shipped R273-canonical bytes)

    (-1, 11)                          5 → 0
    probe_errno == 11                 1 → 0
    EWOULDBLOCK_RAW                   0 → 8   (2 defs + 6 uses)
    (-1, EWOULDBLOCK_RAW)             0 → 5
    probe_errno == EWOULDBLOCK_RAW    0 → 1
    cfg-adjacency linux -> `= 11;`    0 → 1   (base target_os count in mod.rs = 0)
    cfg-adjacency macos -> `= 35;`    0 → 1

rustfmt --check on the file: base rc=0 · 653 §4(b)'s sealed one-line form rc=1 · shipped
canonical form rc=0 (R273: the symbol is 13 chars longer than the literal; only one
rewrapped site crosses max_width=100 — the operative trigger for the other three is an
inner rustfmt call-width heuristic, exact rule unmeasured; the governing evidence is the
measured rc chain plus the four-hunk whitespace-only diff, not any width model). Linux
by-name proof at three points — base 122/0 · the edited bytes 122/0 · the post-take-main
merged head 122/0 — pass-name sets identical at all three; the fix moves nothing on Linux.

## The dispatch pair (Phase 4; R266 ruled 4a IN)

**4a — RED control on post-PR-A main `807f8f7d` (zero code bytes changed since the five
reds): MEASURED.** workflow_dispatch run 31639859157 (2026-08-12, dispatched 20:53Z,
serial job failed 21:00Z — died at the lib target): `test result: FAILED. 118 passed;
4 failed`, exit 101; the SAME four failed names as every banked red
(drop_order_commutative · nested_grants_and_depth · panic_unwind_restores_depth ·
upgrade_refused_fail_closed); `left: (-1, 35)` ×3 plus the compound
`probe_rc=-1 probe_errno=35` line; zero `left: (0, _)` and zero `probe_rc=0` printings
(the C-3 observable absent). Resolved toolchain: rustc 1.97.1 (8bab26f4f 2026-07-14).
Red and green now share the dispatch event context, eliminating the dispatch-vs-push
confound.

**4b — GREEN dispatch on this PR's final head: result recorded in the lane's stop-file of
record and re-proven by the merge push's own serial run (Phase 6).** By construction the
4b result cannot be written INTO the head 4b validates — the dispatch must run on the
exact head asked to merge, so its numbers land in the merge-ask stop-file (with the
resolved toolchain line compared against 4a's; any mismatch a noted confound), and the
permanent in-tree green is Phase 6's: the merge push runs macos-qsc-full-serial on main
itself with the four tests sealed green by name.

## Boundary

Not claimed: that the rest of the macOS serial suite is green (its first full run since
2026-07-21 happens behind this fix; any new failure is a fresh finding, not this lane's
scope) · that any PR merges (the operator's act) · that main's public-safety greens on a
schedule (both push-only suites must green post-merge) · that a dispatch result equals a
push result (mitigated, not erased, by the 4a/4b same-context pair).
