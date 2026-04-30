Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-28

# NA-0237C public-safety Main-Red Recursion Repair Testplan

Goals: G4

## Validation Checkpoints

- `.github/workflows/public-ci.yml` still runs the required protected context named `public-safety`, keeps docs-only PRs cheap, and preserves the merged self-repair bootstrap from `NA-0237D`.
- The advisories-side self-repair detector may tolerate a temporarily missing latest-main `public-safety` check only for exact sanctioned self-repair PR scope while latest-main `advisories` is completed/failure; the final `public-safety` job remains strict.
- `scripts/ci/public_safety_gate.py` keeps bare latest-`main` red handling fail-closed, but permits a bounded advisory-remediation exception when latest `main` is red because `advisories` failed, the PR head's own `advisories` result is green, and the PR changes `Cargo.lock` or a `Cargo.toml` path.
- Local proof on refreshed live data shows:
  - bare `check-main-public-safety` fails on current red `main`
  - PR `#715` passes the self-repair classifier when latest-main `public-safety` is temporarily missing but latest-main `advisories` is failing and the PR path set is exact
  - PR `#713` head `e4032d3906f5` passes because it changes `Cargo.lock` and clears `advisories`
  - PR `#708` head `7f54ea7ab4ae` still fails because it changes no dependency-remediation path
- `DECISIONS.md` records `D-0432`, including the bounded missing-check tolerance, advisory-remediation exception, and rejection of broader bypasses or duplicate protected contexts.
- `TRACEABILITY.md` contains the `NA-0237C implementation/evidence` entry linking the workflow/helper repair to the live positive and negative proofs.
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md` contains the matching Directive 358 entry with refreshed SHAs, READY proof, validations, CI state, canary rerun evidence, and any recoveries.
- Local goal-lint passes via the accepted synthetic-event path with governance PR metadata.
- The markdown inventory commands and manual link-integrity runbook from `AGENTS.md` pass.
- The added-line leak-safe scan reports zero secret-like markers.

## References

- `.github/workflows/public-ci.yml`
- `scripts/ci/public_safety_gate.py`
- `DECISIONS.md` (`D-0432`)
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
