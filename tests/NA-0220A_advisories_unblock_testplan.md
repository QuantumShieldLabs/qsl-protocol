Goals: G4, G5

# NA-0220A Advisories Workflow / Toolchain Unblock Test Plan

## Scope

- validate the governance-only `NA-0220A` unblock lane;
- confirm the unblock work stays limited to the required `advisories` context path plus matching governance/evidence updates; and
- confirm the current `NA-0220` audit PR can be resumed or superseded cleanly after the unblock lands.

## Implementation proof targets

- prove the failing path from PR `#652` shows the prior `advisories` job timing out through `cargo-binstall` and then failing source install because `cargo-audit 0.22.0` requires Rust `1.85` or newer while the job pinned `1.84.0`;
- confirm the repaired `advisories` job pins Rust `1.85.1`;
- confirm the repaired `advisories` job installs `cargo-audit 0.22.0` with `cargo install --locked`; and
- confirm the fail-closed `cargo audit --deny warnings` gate and advisories artifact upload remain unchanged.

## Docs-only validation checkpoints

- local goal-lint using a synthesized `GITHUB_EVENT_PATH`
- markdown inventory counts from the `AGENTS.md` manual runbook
- deterministic local-link existence check from `AGENTS.md`
- changed-path scope proof limited to the approved unblock files
- added-line leak-safe scan confirming no secret-like values were introduced
- workflow-surface proof that the `advisories` job uses Rust `1.85.1` plus direct locked `cargo-audit 0.22.0` installation

## Acceptance checkpoints

- the unblock proof captures the previous `1.84.0` plus `cargo-binstall` failure path truthfully
- the required `advisories` context goes green on the unblock PR
- the unblock PR keeps fail-closed `cargo audit --deny warnings` behavior and advisories artifact upload unchanged
- no runtime/code paths under qsc, qsc-desktop, qsl-server, or qsl-attachments change
- no `Cargo.toml` or `Cargo.lock` changes occur
- after the unblock lands, the current `NA-0220` audit PR can be resumed or superseded cleanly without queue confusion
- docs/governance validation passes
