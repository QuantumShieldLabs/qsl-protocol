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

## Invocation record — 2026-07-14 (PR #1571, operator-authorized)

- Trigger: GitHub never emitted the push-triggered `public-ci` run for main merge
  `9018ae4f` (the NA-0645 promotion, PR #1569) — main HEAD carried NO
  `public-safety` check-run, so the required gate on PR #1570 (NA-0645) failed
  closed. Content was not the cause (goal-lint and all non-skipped checks green).
- Disclosure: an earlier mis-shaped `workflow_dispatch` (run 29375806367,
  pr_number=1570) failed its own validation by design and attached
  `advisories=failure` + `public-safety=failure` check-runs to main@`9018ae4f`;
  those record the dispatch rejection, not any content or advisory failure.
- This invocation: bootstrap PR #1571 modifies exactly the sanctioned scope
  (`.github/workflows/public-ci.yml` comment, `scripts/ci/public_safety_gate.py`
  comment, this stub, the rolling journal). Its merge push restores a
  legitimately green `public-safety` run on the new main HEAD; PR #1570 merges
  only after that run is green AND `formal-proverif-composition` is green.
