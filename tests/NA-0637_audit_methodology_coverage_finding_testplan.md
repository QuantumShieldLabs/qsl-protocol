# NA-0637 test plan — audit-methodology coverage finding (governance/ledger only)

Goals: G4, G5

Directive: QSL-DIR-2026-07-12-573 (D573). Decision: D-1260. Lane class: governance/ledger ONLY —
**no protocol/source/wire/crypto/state-machine change**, therefore **no Rust test, no vector, no
model change**. The deliverables are two ledger filings, one ON-DECK scoping entry, and the
governance spine updates; the "tests" are the lane's fail-closed validation gates below.

## Validation gates (run at PR time, results recorded)

| Gate | Command / mechanism | Expected | Result |
|---|---|---|---|
| Phase 0 CONFIRM-LIVE | qwork proof + directive/D-number/WF-number greps | D573 highest directive; D-1260 absent pre-lane; WF-0018 highest (WF-0019/0020 free); sole READY = NA-0637 | PASS (recorded in `NA-0637_as_built.md` §0) |
| The drop, verified | `grep -ci 'audit-methodolog\|coverage finding\|0609' docs/governance/evidence/NA-0634_as_built.md` | 0 (NA-0634 never filed it) | PASS — 0 |
| Scope guard | `git diff --name-only main...HEAD` ⊆ the D573 allowed-path list | 7 files, all allowed; no source/`formal/`/vector/canonical/`.github` path | PASS — exactly the 7 allowed files |
| goal-lint (local) | `GITHUB_EVENT_PATH=<synthesized> python3 tools/goal_lint.py` (body carries `Goals: G4, G5`; base=main `5d31f108`, head=the final lane commit) | exit 0 | PASS — exit 0, "OK: goal compliance checks passed." |
| Private-material scan | grep the full diff for secret/token/key-material patterns; class-only content check | no private value; prose + public identifiers only | PASS — 0 pattern hits over the full staged diff |
| Ledger integrity | `grep -c '^### WF-0019\|^### WF-0020' docs/ops/IMPROVEMENT_LEDGER.md` | exactly 1 each, appended at tail | PASS — 1 + 1; ledger diff 19 additions, 0 deletions |
| Queue invariants at closeout | `grep -c '^Status: READY$' NEXT_ACTIONS.md` = 0; STATE line = `READY=NONE \| HIGHEST_NA=0637 \| HIGHEST_D=1260`; ON DECK 0a present, NOT promoted | all hold | PASS — 0 READY lines; STATE `READY=NONE \| HIGHEST_NA=0637 \| HIGHEST_D=1260`; 0a present once |
| Decision-present exactly once | `grep -c 'ID:\*\* D-1260' DECISIONS.md` | 1 | PASS — 1 |

## What this lane must NOT do (STOP conditions exercised as review checks)

- No re-audit: the diff contains NO new claim about any NA-0609B conclusion beyond the already-
  recorded ENG-0038 contradiction; the re-examination exists only as scope text (ON DECK 0a).
- No reopening: ENG-0003/ENG-0004 ledger entries byte-untouched.
- No claim movement: no `docs/canonical/**`, no `docs/public/**`, no claim-carrying doc touched.

## CI

No new check is added and none is modified. The PR runs the standard required checks; the diff is
docs/governance-only, so source-scoped suites skip by path filter as designed.
