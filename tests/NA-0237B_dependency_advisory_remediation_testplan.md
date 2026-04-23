Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-22

# NA-0237B Dependency Advisory Remediation Testplan

Goals: G4

## Docs-only Validation Checkpoints

- `NEXT_ACTIONS.md` marks `NA-0237A` as `BLOCKED` on `RUSTSEC-2026-0104` and promotes `NA-0237B` as the sole `READY` item using the approved successor block.
- `docs/archive/testplans/NA-0237A_blocked_on_rustls_webpki_advisory_evidence.md` records the stopped-lane proof, affected crate/version, patched floor, dependency reachability, and governance-only scope statement.
- `DECISIONS.md` records `D-0426` and states that `NA-0237A` is blocked by dependency health rather than remaining send_commit logic.
- `TRACEABILITY.md` contains both the `NA-0237A blocked-on-advisory` entry and the `NA-0237B READY` entry.
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md` contains the matching Directive 345 entry.
- Local goal-lint passes via the accepted synthetic-event path with governance PR metadata.
- The markdown inventory commands and manual link-integrity runbook from `AGENTS.md` pass.
- The added-line leak-safe scan reports zero secret-like markers.

## References

- `docs/archive/testplans/NA-0237A_blocked_on_rustls_webpki_advisory_evidence.md`
- `DECISIONS.md` (`D-0426`)
- `TRACEABILITY.md`
- `NEXT_ACTIONS.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
