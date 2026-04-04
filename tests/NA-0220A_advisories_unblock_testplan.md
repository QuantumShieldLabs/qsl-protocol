Goals: G4, G5

# NA-0220A Advisories Workflow / Toolchain Unblock Test Plan

## Scope

- validate the governance-only `NA-0220A` unblock lane;
- confirm the unblock work stays limited to the required `advisories` context path plus matching governance/evidence updates; and
- confirm the current `NA-0220` audit PR can be resumed or superseded cleanly after the unblock lands.

## Docs-only validation checkpoints

- local goal-lint using a synthesized `GITHUB_EVENT_PATH`
- markdown inventory counts from the `AGENTS.md` manual runbook
- deterministic local-link existence check from `AGENTS.md`
- changed-path scope proof limited to the approved unblock files
- added-line leak-safe scan confirming no secret-like values were introduced

## Acceptance checkpoints

- the required `advisories` context goes green on the unblock PR
- no runtime/code paths under qsc, qsc-desktop, qsl-server, or qsl-attachments change
- no `Cargo.toml` or `Cargo.lock` changes occur
- after the unblock lands, the current `NA-0220` audit PR can be resumed or superseded cleanly without queue confusion
- docs/governance validation passes
