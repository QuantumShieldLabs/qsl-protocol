# NA-0638 — NA-0609B Coverage Re-Examination — Testplan

Goals: G4, G5

Governance/ledger-only lane (D574, D-1261). No runtime surface: validation is
structural. This lane's own method note applies: no model or test was built or
run to DECIDE any claim — the checks below validate the GOVERNANCE artifacts.

1. **Scope guard.** `git diff --name-only <base>..HEAD` returns exactly:
   `docs/governance/evidence/NA-0638_as_built.md`,
   `docs/ops/IMPROVEMENT_LEDGER.md`,
   `tests/NA-0638_na0609b_coverage_reexamination_testplan.md`,
   `NEXT_ACTIONS.md`, `DECISIONS.md`, `TRACEABILITY.md`,
   `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`. Nothing under source, `formal/`,
   `tests/*.rs`, vectors, canonical, `.github/**`.
2. **Verdict-table completeness.** The evidence doc's table has exactly nine
   rows; each row carries a verdict in {EXERCISED, INSPECTED-ONLY,
   CONTRADICTED}, a named mechanism (or named absence) at the audit-time SHA
   `c0b30265`, and a slice-overlap cell. Claim 5 = CONTRADICTED citing
   ENG-0038 only (no new work). Claim 7 (the 9th) present with the WF-0019
   8-vs-9 discrepancy recorded.
3. **Ledger integrity.** WF-0021 and WF-0022 exist, follow the ledger schema,
   and each names the exercise that settles it; WF-0021 is closed-as-paid
   (NA-0628); WF-0022 is open and pointed at successor 0c; WF-0019's status
   line records closure by this lane (its own anticipated lifecycle — not an
   alteration of NA-0609B's record). No duplicate filings for NA-0636's
   slices 2/3/4 (they remain tracked under ENG-0038).
4. **Queue invariants.** Exactly one `^Status:` line carries a state in
   NEXT_ACTIONS.md section 2 per lane block; after closeout the NA-0638 block
   is `Status: DONE`, STATE line reads
   `READY=NONE | HIGHEST_NA=0638 | HIGHEST_D=1261`, ON-DECK 0a is marked DONE,
   and 0b/0c exist as ON-DECK items (not READY, not promoted). NA-0635
   untouched.
5. **goal-lint.** `scripts/audit/run_goal_lint_pr.sh <PR#>` passes (Goals
   line present; changed-file classes consistent).
6. **Boundary attestation.** The evidence doc §1 records that the D574
   mapping boundary was never crossed (no exercise built or run to decide a
   claim); the two INSPECTED-ONLY verdicts rest on enumerated ABSENCE of
   mechanisms at `c0b30265`, reproducible via the `git grep`/`git ls-tree`
   commands cited there.
