Goals: G4

Status: Archive
Owner: QSL governance
Last-Updated: 2026-04-16
Replaces: n/a
Superseded-By: n/a

# NA-0235A Scope Repair TUI Dependency Stack Evidence

## Summary

This governance-only lane repairs `NA-0235A` again so the queue scope matches refreshed contradiction proof on current `main`.

No runtime code, manifests, or lockfiles change in this PR. The lane updates governance and evidence only so the next implementation attempt can remediate the remaining dependency blocker truthfully.

## Refreshed Contradiction Proof On Main

- PR `#695` remains `OPEN` on head `68a3a8081889` and still fails live `advisories` plus `public-safety`.
- The current `NA-0235A` block already authorizes `apps/qsl-tui/Cargo.toml` and bounded `apps/qsl-tui/src/**` fallout, but it still does not explicitly authorize the broader bounded TUI dependency-stack replacement now implied by refreshed contradiction proof.
- `cargo metadata --locked --format-version 1` still resolves `ratatui 0.30.0` with both `ratatui-crossterm 0.1.0` and `ratatui-termwiz 0.1.0`, and `Cargo.lock` still carries the remaining stale `rand 0.8.5` path through:
  - `ratatui 0.30.0`
  - `ratatui-termwiz 0.1.0`
  - `termwiz 0.23.3`
  - `terminfo 0.9.0`
  - `phf_generator 0.11.3`
  - `rand 0.8.5`
- Refreshed source proof still shows zero local rand callsites under `apps/qsl-tui/src/**`.
- Refreshed source proof shows the bounded additional replacement seam is:
  - `apps/qsl-tui/src/main.rs`
  - `qsl/qsl-client/qsc/src/main.rs`
  - `qsl/qsl-client/qsc/src/tui/**`

## Why The Current Scope Was Still Insufficient

The previously repaired `NA-0235A` block already added:

- `apps/qsl-tui/Cargo.toml`
- `apps/qsl-tui/src/**` only if directly touched by minimal API-compatibility changes required by the dependency remediation
- `qsl/qsl-client/qsc/src/**` only if directly touched by minimal API-compatibility changes required by the dependency remediation

Refreshed contradiction proof shows the remaining blocker is no longer just the direct qsl-tui manifest pin. The next truthful implementation attempt may need bounded TUI dependency-stack replacement in the active TUI entry surfaces themselves, namely:

- `apps/qsl-tui/src/main.rs`
- `qsl/qsl-client/qsc/src/main.rs`
- `qsl/qsl-client/qsc/src/tui/**`

The current block therefore still understates the real bounded remediation surface.

## Exact Repaired Scope Text As Committed

```md
### NA-0235A — Runtime Dependency Advisory Remediation for Public-Safety Unblock
Status: READY
Problem:
- PR `#695` contains the `NA-0235` workflow/governance repair, and the sanctioned `public-safety` bootstrap now attaches truthfully to the PR head. That gate is failing for the correct reason: the current dependency set still trips live RustSec advisories. Refreshed contradiction proof shows the current `NA-0235A` scope still understated the real dependency-remediation surface because the remaining stale `rand 0.8.5` path is carried by the latest `ratatui -> ratatui-termwiz -> termwiz -> terminfo -> phf_generator` chain and may require bounded TUI dependency-stack replacement across the active TUI entry surfaces. Until those dependency findings are remediated or truthfully proven non-runtime/tooling-only, `NA-0235` cannot merge.
Scope:
- `Cargo.lock`
- `Cargo.toml` only if directly touched by the bounded dependency fix
- `qsl/qsl-client/qsc/Cargo.toml` only if directly touched
- `tools/refimpl/quantumshield_refimpl/Cargo.toml` only if directly touched
- `apps/qsl-tui/Cargo.toml`
- `apps/qsl-tui/src/**` only if directly touched by minimal API-compatibility changes or bounded TUI dependency-stack replacement required by the remediation
- `qsl/qsl-client/qsc/src/main.rs` only if directly touched by bounded TUI dependency-stack replacement required by the remediation
- `qsl/qsl-client/qsc/src/tui/**` only if directly touched by bounded TUI dependency-stack replacement required by the remediation
- `qsl/qsl-client/qsc/src/**` only if directly touched by minimal non-TUI API-compatibility changes required by the dependency remediation
- `qsl/qsl-client/qsc/tests/**` only if directly touched by minimal API-compatibility changes required by the dependency remediation
- `DECISIONS.md`
- `TRACEABILITY.md`
- docs/governance/evidence only as needed
- no `.github`, website, `qsc-desktop`, `qsl-server`, or `qsl-attachments` changes
- no weakening of the repaired `public-safety` gate
Must protect:
- the faster PR smoke path from `NA-0233A`
- fail-closed dependency-audit behavior for runtime/security PRs
- transcript binding
- pinned mismatch reject behavior
- NA-0221 fail-closed no-mutation behavior
- NA-0222 honest operator-visible status/marker truth
- qsl-server remains transport-only
- qsl-attachments remains opaque ciphertext-only
Deliverables:
1) prove the exact current RustSec findings and runtime/tooling reachability truth on refreshed main and the `NA-0235` resume target
2) update or replace vulnerable dependencies, or truthfully prove specific findings non-runtime/tooling-only where applicable
3) add direct verification evidence that the repaired `NA-0235 public-safety` gate can pass without weakening its semantics
4) update governance/evidence truthfully
Acceptance:
1) the dependency findings blocking `NA-0235` are resolved or truthfully downgraded on the final head
2) the repaired `public-safety` gate remains fail-closed and can pass on the resumed `NA-0235` lane
3) docs-only PRs remain cheap and the fast smoke PR path remains intact
4) no unrelated runtime or workflow drift is introduced
```

## Governance-Only Note

This PR repairs queue truth only. It introduces no runtime changes, no manifest or lockfile changes, and does not remediate the dependency set itself.
