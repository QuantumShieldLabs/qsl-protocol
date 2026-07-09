# NA-0626 — closeout test plan (D-1248)

Directive: QSL-DIR-2026-07-09-563 (D563). Implementation decision: D-1247. Closes ENG-0024,
ENG-0026, ENG-0030, ENG-0031.

## Closeout markers
- ENG-0024 / ENG-0026 / ENG-0030 / ENG-0031 marked DONE in `docs/ops/IMPROVEMENT_LEDGER.md`,
  with impl PR #1530 / merge `fb2f1c21` filled into all four headers.
- ENG-0025 re-triage note recorded in the ledger (seam contract shrank; remaining scope:
  persistence choreography + `main.rs` extraction; the `recv.ck_pq_send` wire-op transport slot;
  the qsc combined-send CADENCE switch — D561 operator-set).
- `NEXT_ACTIONS.md`: NA-0626 -> `Status: DONE` + OUTCOME; NA-0627 (ENG-0028 ProVerif) appended
  as the sole READY block; LIVE QUEUE header `READY=NA-0627 | HIGHEST_NA=0627 | HIGHEST_D=1248`.
  Exactly one `^Status: READY` (verified with the anchored grep).
- `DECISIONS.md`: D-1247 (implementation) and D-1248 (closeout) each recorded exactly once.
- `TRACEABILITY.md` + `docs/ops/ROLLING_OPERATIONS_JOURNAL.md` map the implementation, the
  closeout, and the promotion.
- Response filed (`NA0626_rk_unification_combined_boundary_<UTC>_D563.md`); directive archived
  (the D563 file carries Chat A's DESIGN-LOCK CONCLUSIONS appendix).

## Post-merge verification (Phase 6)
1. `main` fast-forwarded to merge commit `fb2f1c21` (impl PR #1530); `HEAD == origin/main ==
   main`, tree clean.
2. Main-push workflows on the merge commit all SUCCESS **at job level**, including the two
   event-filtered suites that SKIP on pull_request by design and therefore only exercise on the
   main push: `qsc-linux-full-suite` (qshield-ci) and `macos-qsc-full-serial` (macos-build).
3. Post-merge spot re-verification on `main`:
   - all 15 suite2 vector runners green (incl. `scka_logic 20/20`, `pq_reseed 11/11`,
     `e2e_recv 4/4`, `interop_ximpl 2/2`);
   - `python3 scripts/ci/validate_suite2_vectors.py` OK;
   - `python3 formal/run_model_checks.py` OK (root-composition slice: 15,032 states, 9 shapes);
   - `cargo test -p quantumshield_refimpl` green (all suites, incl.
     `suite2_combined_boundary`).
4. Full local `cargo test -p qsc` against the final tree (recorded pre-merge in
   `tests/NA-0626_rk_unification_testplan.md`): 144 binaries, 587 passed / 0 failed / 3 ignored.

## What this lane proved (carried forward as the regression surface)
- The session root is SINGLE-SLOT BY CONSTRUCTION: no state can represent a diverged
  `recv.rk`/`dh.rk`, and no caller composition (INJECT/ADOPT/send-half refresh) exists to
  forget — the defect class behind the D560 amendment, the NA-0624 dh.rk-sync desync, and
  ENG-0030 is structurally closed.
- A reseed- or combined-RECEIVE returns the receiver's FULL coherent schedule (both header-key
  directions, both PQ chains) — pinned by the INVERTED
  `reseed_receiver_send_schedule_must_be_refreshed_from_advanced_root`, the combined round-trip
  suite, invariant 4 of the replaced model, and the post-reseed-ADV e2e authentication.
- The combined DH+PQ boundary round-trips with the design-locked DH-first-then-PQ composition;
  the PQ-first mis-composition is pinned as a DETECTED counterfactual at model level; hybrid
  healing survives subsequent boundaries (refimpl + model).
- A pre-v3 snapshot/blob is UNRECOVERABLE with a DISTINCT deterministic marker and zero on-disk
  mutation (unit + integration, one test per removed legacy branch).
- The vector freeze is byte-proven: the only changed pre-existing vector member is the removed
  `dh_rk` duplicate (zero wire bytes), machine-asserted with a cross-set sha256 guard.
- The seed-model wires are pinned to absolute golden SHA-256 digests (Operator Decision 3's
  strengthened gate).
