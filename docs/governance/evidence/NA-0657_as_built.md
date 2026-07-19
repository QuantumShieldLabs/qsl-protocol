# NA-0657 as-built — qsl-desktop bootstrap (D593, D-1280 spine + D-0001 qsl-desktop)

Result class: DESKTOP_BOOTSTRAP_PASS (stopped at the two open PRs; the
operator merges qsl-desktop PR #1 first, applies the F2 console companion,
then the spine closeout). Directive: QSL-DIR-2026-07-18-593 (D593, APPROVED
2026-07-18, AMENDED in place at approval — header→APPROVED, END line, the F2
flip made non-optional; approved sha256 `2ae6ad4b…`, 722 lines; F1/F2/F3
resolved to the drafted defaults, rulings recorded in the NA-0657 promotion
block). Base: main `131e1cdc` (the #1595 seating merge; qwork-proven).
Proof root: `/srv/qbuild/tmp/NA0657_qsl_desktop_bootstrap_20260719T011958Z/`.

## §1 Phase 0 (all green; neither directive STOP fired)

- qwork proof (startup.qsl-protocol.kv): startup_result=OK; lane=NA-0657;
  head==origin_main==main==`131e1cdc` (re-verified live post-fetch);
  worktree/index/untracked clean; ready_count=1; queue_top_ready=NA-0657;
  requested_lane_status=READY; shared_target_ready=yes.
- Disk 52% (<95%); /backup/qsl mounted.
- D-1280 canonical count 0 pre-lane, D-1279 canonical count 1
  (decision_id_counter.py, both exit 0). Anchored `^Status: READY` ×1 =
  NA-0657; STATE as seated.
- Main health on `131e1cdc`: 7 push workflows success at start (incl.
  public-ci); formal-ci + qsc-adversarial in their normal windows —
  qsc-adversarial concluded SUCCESS during the lane; formal-ci conclusion
  verified before the first push (see §3).
- **STOP check 1 — qsl-desktop EMPTY:** `git ls-remote` returned ZERO refs;
  API size 0; public; pushed_at 2026-07-18T00:00:57Z (creation only). NOT
  fired.
- **STOP check 2 — PVR:** `private-vulnerability-reporting` enabled=true.
  NOT fired.
- As-found settings: default_branch setting `main` (no ref);
  delete_branch_on_merge=false (the F2.2 flip target).
- Mirror re-verified UNCHANGED from drafting: qsl-server ci.yml sha256
  `594dbbce…` identical; branch protection = required `rust`, strict=true,
  enforce_admins=true.
- Byte-copy sources re-verified: spine CODE_OF_CONDUCT.md `2cbf021e…`,
  spine LICENSE `459cd3e0…`.
- Toolchain: rustc 1.95.0, cargo 1.95.0.

## §2 Inheritance consumed (Phase 1 of the directive numbering)

D-1279 (the registration; its four-item owed list is this lane's (a)–(d));
D-1278/NA-0655 (community-health forms; CoC byte-identity; the verbatim
reporting-section rule); D578/D-1265 (the satellite model); NA-0608A/D-1207
(the pointer-CLAUDE.md pattern); D-1266/ENG-0041 + D-1277/ENG-0046 (the
rev-pin bump-lane pattern cited by the pointer for the FUTURE qsc pin);
gate D-A/L9 (v1 Linux-only — stated in README and CLAUDE.md); the GH007
identity ruling (every commit in both repos:
`Tebbens4832 <238594419+Tebbens4832@users.noreply.github.com>` — verified on
both qsl-desktop commits); the NA-0654 resolver-drift disposition (N/A here:
zero dependencies, nothing to update).

## §3 The qsl-desktop delta

- Root anchor (F1): `fc7c00d9e78ed9fd5709c69792a55eeefc5c8dd4` — ONLY
  CLAUDE.md; direct push to main; the repository's ONLY-ever direct push,
  sanction SPENT. Pushed only after all nine base main-push runs concluded
  green.
- PR #1: branch `na0657-desktop-bootstrap`, head
  `31198b27656baed47c37049dc813f63e998b7d3b`, +858 lines, EXACTLY the 12
  files of the D593 allow-list. The `rust` check RAN and PASSED on the PR
  (the workflow rides the head branch).

sha256 table (13 files as landed):

| file | sha256 (first 8) | source |
|---|---|---|
| CLAUDE.md | 1b7cd116 | Appendix E, cmp 0 |
| .github/workflows/ci.yml | 9ed83982 | Appendix A, cmp 0 |
| .gitignore | 44c92e3a | Appendix G3, cmp 0 |
| Cargo.toml | 7e7408b2 | Appendix G1, cmp 0 |
| Cargo.lock | 23fbcff7 | generated; single-package ASSERTED |
| src/main.rs | a535f07c | Appendix G2, cmp 0 |
| README.md | 433fab05 | Appendix B, cmp 0 |
| LICENSE | 459cd3e0 | spine byte-copy, cmp 0 |
| NOTICE | 310398cf | Appendix H, cmp 0 |
| SECURITY.md | aa89be77 | Appendix C, cmp 0; reporting section == spine (awk-extract, cmp 0) |
| CODE_OF_CONDUCT.md | 2cbf021e | spine byte-copy, cmp 0 |
| CONTRIBUTING.md | 4966d151 | Appendix D, cmp 0 |
| DECISIONS.md | 5077eea0 | Appendix F, exact after the single `<DATE>`→2026-07-19 substitution; opens D-0001 |

## §4 Local gate + scans

- `cargo generate-lockfile`: lock contains ONLY `qsl-desktop 0.1.0` (zero
  external dependencies; single-package assert PASS).
- `cargo fmt --all -- --check` CLEAN; `cargo test -q` 1 passed / 0 failed;
  `cargo clippy -q -- -D warnings` CLEAN (exit 0);
  `cargo metadata --locked --format-version=1` OK. Proof-root clone, own
  target (`env -u CARGO_TARGET_DIR`); the spine shared target untouched.
- Binary output (class-recorded): `qsl-desktop 0.1.0 (bootstrap
  placeholder; no application functionality)`.
- Publication scans (added_line_publication_scan.py --new-file, all 13
  files): overclaim_hits [] ×13; secret/private/qscwork/ssh patterns 0
  ×13; 9 files class "pass", 4 files class "review" on `http_url` ONLY —
  README (the spine link), NOTICE (the three canonical repo links, the
  house-pattern form), LICENSE (gnu.org/fsf.org — byte-copy of the
  already-public spine file), CODE_OF_CONDUCT.md (the Contributor Covenant
  link — byte-copy, already public ×4 repos). Disposition: the deliberate
  public canonical links; PASS. Raw scan output proof-root-only.

## §5 The spine closeout delta (this PR; EXACTLY 6 paths)

DECISIONS.md (D-1280 appended; canonical count 1 post-append, tool exit 0);
NEXT_ACTIONS.md (STATE → `READY=NONE | HIGHEST_NA=0657 | HIGHEST_D=1280`;
prior-STATE comments; NA-0657 DONE one-line comment; READY pointer → NONE;
lane block → Status: DONE + OUTCOME; anchored `^Status: READY` ×0);
TRACEABILITY.md (NA-0657 row); docs/ops/ROLLING_OPERATIONS_JOURNAL.md (lane
entry); this file (git add -f; staged-list confirmed); the testplan.

## §6 Boundaries held / NOT claimed

NO Tauri dependency; NO GUI code; NO qsc dependency (the skeleton lane
introduces the pin); NO spine code/test/dependency/lockfile/workflow
change; NO claim-boundary change; NO DOC-PROG-004 edit (statuses fold at
the next revision — the D586/D592 pattern); NO ledger edit (no finding);
NO executor settings mutation (F2 = operator console); NO second direct
push. NOT claimed: any GUI exists; any application functionality; the
branch protection applied (OWED to the operator's companion step at merge
time — the `rust` context cannot bind before the workflow's first main
run); any public/production/security-completion posture. Claim boundary
UNCHANGED.

## §7 Merge choreography (operator; recorded from the directive + the
operator's restatement)

1. Merge qsl-desktop PR #1 (merge commit).
2. Console companion: Appendix I1 branch protection (required `rust`,
   strict, enforce_admins=true, no reviews, no force pushes/deletions;
   merge-commit process per house rule) + Appendix I2
   delete_branch_on_merge=true.
3. Merge the spine closeout PR last. Post-merge verification records
   protection state AS FOUND.
