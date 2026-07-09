# NA-0624 closeout — verification markers

Directive QSL-DIR-2026-07-08-561 (D561); decision D-1244.

- PR #1526 merged via merge commit `9dbb1ed1d820a3746732e2f89fccaf51f25f18d3`; local main
  fast-forwarded and equal to origin/main. VERIFIED.
- PR-path checks: 38 total, 36 success, 2 designed-SKIPPED (event-filtered qsc full suites);
  0 failed, 0 pending at merge; mergeStateStatus CLEAN. VERIFIED.
- Post-merge main-push runs at `9dbb1ed1`: all 9 main-push workflows SUCCESS, including the two event-filtered full suites at job level -- qsc-linux-full-suite SUCCESS (qshield-ci) and macos-qsc-full-serial SUCCESS (macos-build) -- plus public-ci, qsc-adversarial, suite2-ci, formal-ci, demo-packaging, Code Quality, Push on main. VERIFIED.
- On main: SCKA wiring present (`qsp_scka_store` in qsc/src/main.rs); D-1243 present exactly once;
  ENG-0012 Status CLOSED in the ledger; NA-0624 evidence + testplan files present. VERIFIED.
- Queue invariants after the closeout patch: exactly one `^Status: READY` (NA-0625);
  NA-0624 `Status: DONE` with OUTCOME; LIVE QUEUE STATE = READY=NA-0625 | HIGHEST_NA=0625 |
  HIGHEST_D=1244; D-1244 present exactly once; D-1245 absent. VERIFIED.
- Directive archive present: /srv/qbuild/operator/directives/QSL-DIR-2026-07-08-561_qsc_scka_wiring.md. VERIFIED.
- Response file written: /srv/qbuild/operator/responses/NA0624_qsc_scka_wiring_<UTC>_D561.md. VERIFIED.
