Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-23

# NA-0237C public-safety Main-Red Recursion Repair Testplan

Goals: G4

## Validation Checkpoints

- `.github/workflows/public-ci.yml` still runs the required protected context named `public-safety`, keeps docs-only PRs cheap, and passes PR context into `check-main-public-safety` only for relevant PR lanes.
- `scripts/ci/public_safety_gate.py` keeps bare latest-`main` red handling fail-closed, but now permits exactly one bounded exception when latest `main` is red because `advisories` failed, the PR head's own `advisories` result is green, and the PR changes `Cargo.lock` or a `Cargo.toml` path.
- Local proof on refreshed live data shows:
  - bare `check-main-public-safety` fails on current red `main`
  - PR `#713` head `e4032d3906f5` passes because it changes `Cargo.lock` and clears `advisories`
  - PR `#708` head `7f54ea7ab4ae` still fails because it changes no dependency-remediation path
- `DECISIONS.md` records `D-0429`, including the bounded advisory-remediation exception and rejection of broader bypasses or duplicate protected contexts.
- `TRACEABILITY.md` contains the `NA-0237C implementation/evidence` entry linking the workflow/helper repair to the live positive and negative proofs.
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md` contains the matching Directive 350 entry with refreshed SHAs, READY proof, validations, CI state, canary rerun evidence, and any recoveries.
- Local goal-lint passes via the accepted synthetic-event path with governance PR metadata.
- The markdown inventory commands and manual link-integrity runbook from `AGENTS.md` pass.
- The added-line leak-safe scan reports zero secret-like markers.

## References

- `.github/workflows/public-ci.yml`
- `scripts/ci/public_safety_gate.py`
- `DECISIONS.md` (`D-0429`)
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
