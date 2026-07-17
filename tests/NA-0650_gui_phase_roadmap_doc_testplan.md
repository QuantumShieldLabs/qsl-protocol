# NA-0650 testplan — DOC-PROG-004 GUI Phase Roadmap landing (D586, D-1273)

Docs-only LITE lane: no code, no test-code, no dep change — the "tests" are the
document-fidelity and governance checks below. All run at base `6ae8e8ff` on branch
`na0650-gui-phase-roadmap-doc`; results recorded in
`docs/governance/evidence/NA-0650_as_built.md` and the lane response.

| # | Check | Method | Expected | Result |
|---|-------|--------|----------|--------|
| 1 | qwork proof invariants | read .qwork kv/json | startup_result=OK; lane=NA-0650; head==origin/main==main; clean; ready_count=1; queue_top=NA-0650; shared_target_ready=yes | PASS |
| 2 | CONFIRM-LIVE decision counter | grep canonical IDs in DECISIONS.md | D-1272 ×1; D-1273 ×0 (next-and-absent) | PASS |
| 3 | Sole-READY | anchored `^Status: READY` count + block identity | exactly 1, the NA-0650 block | PASS |
| 4 | Main-health on `6ae8e8ff` | gh run list on the merge SHA | all 9 push runs completed/success (2 monitored to completion during the lane; re-verified before PR) | see as-built §1/§5 |
| 5 | DOC-PROG-004 unclaimed | repo-wide grep pre-landing | zero refs outside queue/promotion text | PASS |
| 6 | Citation liveness | DOC-QSC-008/009/010 exist; NA-0649 DONE at D-1272; ENG-0044 owed; §4.5/§6 anchors | all live, no contradiction | PASS |
| 7 | FIDELITY DIFF-PROOF | `diff -u` appendix (extracted, 126 lines) vs landed file | every hunk maps to class A/B/C; zero content deltas outside | PASS — exactly 2 hunks (as-built §3) |
| 8 | Manifest — locked decisions | count `| L# |` rows | 8 | PASS |
| 9 | Manifest — standing constraints | count section bullets | 4 (incl. the ENG-0044 rationale) | PASS |
| 10 | Manifest — steps + gates | count `### Step`; gate strings | 9 steps; D-A ×1 (before step 4); D-B ×1 (before step 6) | PASS |
| 11 | Manifest — tracks + corrections | count table data rows; numbered items | 6 rows with triggers; 7 corrections | PASS |
| 12 | Manifest — no future NA numbers | `grep -oE 'NA-[0-9]{4}'` | none > 0650; NA-0650 only in the class-A wrapper | PASS |
| 13 | Wiring minimality | git diff on DOC-CTRL-001 + DOC-PROG-003 | one §4.5 row + Last Updated; one §6 bullet + Last-Updated; nothing else | PASS (diff inspected) |
| 14 | Scope guard | changed-path list vs the D586 allowed list | exactly: the new doc, DOC-CTRL-001, DOC-PROG-003, NEXT_ACTIONS, DECISIONS, TRACEABILITY, journal, evidence, testplan; NO src/tests-code/Cargo/.github/formal/vectors/docs-public/ledger | recorded at closeout |
| 15 | git diff --check | whitespace/conflict-marker check | clean | recorded at closeout |
| 16 | goal-lint | scripts/audit/run_goal_lint_pr.sh <PR#> (fresh event payload; body carries the literal `Goals:` line) | OK | recorded at closeout |
| 17 | Overclaim/private-material spot-scan | grep added lines for prohibited claim wording + paths/secrets | zero hits ("vulnerability-free/audited/formally verified" absent; no private values) | recorded at closeout |
| 18 | Public-safety + advisories | CI at the PR (no gate amendment) | green | recorded at the PR |

Claim boundary: UNCHANGED. A PASS asserts the plan is repo truth with fidelity proven —
NOT that any GUI exists, NOT that any roadmap step is started or authorized, NOT that
the external-review gate moved.
