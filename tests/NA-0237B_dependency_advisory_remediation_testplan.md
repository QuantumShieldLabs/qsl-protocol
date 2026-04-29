Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-28

# NA-0237B Dependency Advisory Remediation Testplan

Goals: G4

## Implementation Validation Checkpoints

- Reproduce the refreshed-main advisory exactly: `cargo audit --deny warnings` reports `RUSTSEC-2026-0104` for `rustls-webpki 0.103.12` with patched floor `>= 0.103.13`.
- Prove runtime/merge-critical reachability with `cargo tree -i rustls-webpki --locked`, including the `qsc`, `qsl-tui`, and `qshield-cli` paths.
- Prove lockfile-only feasibility before mutation with `cargo update -p rustls-webpki --precise 0.103.13 --dry-run`.
- Reproduce the pre-edit required clippy stop exactly in `tools/refimpl/quantumshield_refimpl/src/qsp/state.rs` and keep the resumed code touch limited to that authorized file.
- Apply the minimal remediation in `Cargo.lock` plus the bounded clippy-only deterministic-sort cleanup in `tools/refimpl/quantumshield_refimpl/src/qsp/state.rs`, and leave Cargo manifests plus source/API surfaces otherwise unchanged.
- Verify the repaired branch with `cargo audit --deny warnings`, `cargo build --locked`, `cargo clippy --locked -- -D warnings`, and the representative package/test subset listed in the directive.
- Verify `qsl-tui` if it remains a buildable workspace package on refreshed main.
- Confirm `DECISIONS.md` records `D-0434` and `TRACEABILITY.md` records the `NA-0237B implementation/evidence` entry while preserving `origin/main` decisions `D-0428` through `D-0433` exactly.
- Confirm `docs/ops/ROLLING_OPERATIONS_JOURNAL.md` contains the matching `QSL-DIR-2026-04-28-002` entry and records recovered conflict/CLI evidence.
- Local goal-lint passes via the accepted synthetic-event path with implementation PR metadata.
- The markdown inventory commands and manual link-integrity runbook from `AGENTS.md` pass.
- The added-line leak-safe scan reports zero secret-like markers.
- Changed-path proof remains limited to `Cargo.lock`, `tools/refimpl/quantumshield_refimpl/src/qsp/state.rs`, `DECISIONS.md`, `TRACEABILITY.md`, `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`, and this testplan stub.

## References

- `Cargo.lock`
- `tools/refimpl/quantumshield_refimpl/src/qsp/state.rs`
- `DECISIONS.md` (`D-0434`)
- `TRACEABILITY.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
