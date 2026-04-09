Goals: G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-09

# NA-0230 Security Audit Intake and Remediation Plan Testplan

## Scope

- Docs/governance-only implementation for `NA-0230`.
- Canonical output: `docs/audit/DOC-AUD-003_Security_Audit_Packet_Intake_and_Remediation_Plan_v0.1.0_DRAFT.md`.
- Required companions: `DECISIONS.md`, `TRACEABILITY.md`, and one rolling-journal entry in `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.

## Validation bundle

- Local `goal-lint` via a synthesized `GITHUB_EVENT_PATH`.
- Markdown inventory commands from `AGENTS.md`:
  - `git ls-files 'tests/*.md' | wc -l`
  - `git ls-files 'tests/**/*.md' | wc -l`
  - `git ls-files 'docs/*.md' | wc -l`
  - `git ls-files 'docs/**/*.md' | wc -l`
- Manual markdown link-integrity runbook from `AGENTS.md`.
- Added-line leak-safe scan with summary counts only.

## Non-goals

- No runtime battery rerun.
- No `NEXT_ACTIONS.md` closeout or queue promotion.
- No mutation of the 8 staged incoming audit files.
