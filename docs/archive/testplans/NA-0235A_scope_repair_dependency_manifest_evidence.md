Goals: G4

Status: Archive
Owner: QSL governance
Last-Updated: 2026-04-16
Replaces: n/a
Superseded-By: n/a

# NA-0235A Scope Repair Dependency Manifest Evidence

## Summary

This governance-only lane repairs `NA-0235A` so the queue scope matches refreshed contradiction proof on current `main`.

No runtime code, manifests, or lockfiles change in this PR. The lane updates governance and evidence only so the next implementation attempt can remediate the live advisory blocker truthfully.

## Refreshed Contradiction Proof On Main

- PR `#695` remains `OPEN` on head `68a3a8081889` and still fails live `advisories` plus `public-safety`.
- `cargo update -p rustls-webpki --precise 0.103.12 --dry-run` succeeds, proving that advisory remains lockfile-fixable inside the existing remediation lane.
- `cargo update -p rand@0.9.2 --precise 0.9.3 --dry-run` succeeds, proving the tooling-only `proptest` path remains fixable inside the existing remediation lane.
- `cargo update -p rand@0.8.5 --precise 0.9.3 --dry-run` fails because the resolved graph still includes a live `^0.8` requirement, so the blocking advisory is not confined to the previously authorized manifests and lockfile.
- `apps/qsl-tui/Cargo.toml` still directly pins `rand = "0.8"` at line 18.
- `apps/qsl-tui/src/**` currently has zero local rand callsites on refreshed `main`, so any compatibility fallout remains bounded and does not justify widening broader runtime surfaces.

## Why The Old Scope Was Insufficient

The previously promoted `NA-0235A` block allowed only:

- `Cargo.lock`
- `Cargo.toml` only if directly touched by the bounded dependency fix
- `qsl/qsl-client/qsc/Cargo.toml` only if directly touched
- `tools/refimpl/quantumshield_refimpl/Cargo.toml` only if directly touched
- `qsl/qsl-client/qsc/src/**` only if directly touched by minimal API-compatibility changes required by the dependency remediation
- `qsl/qsl-client/qsc/tests/**` only if directly touched by minimal API-compatibility changes required by the dependency remediation
- `DECISIONS.md`
- `TRACEABILITY.md`
- docs/governance/evidence only as needed

Refreshed contradiction proof shows the remaining blocking manifest surface also includes:

- `apps/qsl-tui/Cargo.toml`

The next implementation attempt may additionally need:

- `apps/qsl-tui/src/**` only if minimal API-compatibility fallout directly requires it

The old block therefore could not truthfully authorize the actual bounded dependency remediation.

## Exact Repaired Scope Text As Committed

```md
### NA-0235A — Runtime Dependency Advisory Remediation for Public-Safety Unblock
Status: READY
Problem:
- PR `#695` contains the `NA-0235` workflow/governance repair, and the sanctioned `public-safety` bootstrap now attaches truthfully to the PR head. That gate is failing for the correct reason: the current dependency set still trips live RustSec advisories. Refreshed contradiction proof shows the previous `NA-0235A` scope understated the real bounded dependency surface because `apps/qsl-tui/Cargo.toml` still carries a direct blocking `rand = "0.8"` pin that keeps the advisory set red even when the otherwise in-scope lockfile and manifest fixes are available. Until those dependency findings are remediated or truthfully proven non-runtime/tooling-only, `NA-0235` cannot merge.
Scope:
- `Cargo.lock`
- `Cargo.toml` only if directly touched by the bounded dependency fix
- `qsl/qsl-client/qsc/Cargo.toml` only if directly touched
- `tools/refimpl/quantumshield_refimpl/Cargo.toml` only if directly touched
- `apps/qsl-tui/Cargo.toml`
- `apps/qsl-tui/src/**` only if directly touched by minimal API-compatibility changes required by the dependency remediation
- `qsl/qsl-client/qsc/src/**` only if directly touched by minimal API-compatibility changes required by the dependency remediation
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
