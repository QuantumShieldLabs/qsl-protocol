Goals: G4, G5

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-07

# NA-0227 Rolling Journal Entry Test Plan

This file exists to satisfy the directive-scoped rolling-journal companion requirement for `NA-0227`.

## Scope

- Confirm `docs/ops/ROLLING_OPERATIONS_JOURNAL.md` contains one active `DIRECTIVE 267` entry with the required fields from `docs/ops/DOC-OPS-003_Rolling_Operations_Journal_Procedure_v0.1.0_DRAFT.md`.
- Confirm the entry remains secret-safe: no tokens, passphrases, auth headers, secret-bearing URLs, or long hex dumps.
- Confirm the entry is updated with the branch, PR, validation, recovery, and merge state before directive completion.

## Validation notes

- Local goal-lint via a synthesized `GITHUB_EVENT_PATH`.
- Manual markdown link-integrity runbook from `AGENTS.md`.
- Added-line leak-safe scan over changed files.
