Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-23

Goals: G4

# NA-0237D public-safety Self-Repair Bootstrap Testplan

## Validation checkpoints

- `.github/workflows/public-ci.yml` auto-detects sanctioned workflow-only public-safety self-repair PRs on both `pull_request_target` and `workflow_dispatch`, noops `advisories` only for that bounded case, and preserves the existing docs-only and runtime/dependency paths.
- `scripts/ci/public_safety_gate.py` keeps `check-main-public-safety` fail-closed by default, but now allows a PR only when latest `main` is red via `advisories` and the PR changed-path set stays inside the sanctioned self-repair scope.
- Local live-data proof shows:
  - `check-main-public-safety` without a bootstrap PR argument fails on current red `main`
  - `validate-self-repair-bootstrap-pr` passes for PR `#715` head `019e0385a5a9`
  - `check-main-public-safety --allow-self-repair-bootstrap-pr 715 --expected-pr-sha 019e0385a5a9...` passes on the same current live data
  - `validate-self-repair-bootstrap-pr` fails for KT/runtime PR `#708`
  - `validate-self-repair-bootstrap-pr` fails for dependency-remediation PR `#713`
- The sanctioned bootstrap path for the self-repair PR uses the real `public-ci` workflow on the PR head via `workflow_dispatch`; the post-merge canary uses a rerun/re-request of PR `#715` so merged-main logic re-evaluates `public-safety` on the unchanged PR head.
- Local goal-lint passes via the accepted synthetic-event path with governance PR metadata.
- The markdown inventory commands and manual link-integrity runbook from `AGENTS.md` pass.
- The added-line leak-safe scan reports zero secret-like markers.

## References

- Decision entries: `DECISIONS.md` D-0429 and D-0430
- Traceability entries: `TRACEABILITY.md` entries for `NA-0237C blocked-on-bootstrap`, `NA-0237D READY`, and `NA-0237D implementation/evidence`
- Workflow/script seam: `.github/workflows/public-ci.yml`; `scripts/ci/public_safety_gate.py`
- Rolling journal entry: `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
