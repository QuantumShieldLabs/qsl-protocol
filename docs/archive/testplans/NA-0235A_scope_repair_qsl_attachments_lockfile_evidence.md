Goals: G4

Status: Archive
Owner: QSL governance
Last-Updated: 2026-04-16
Replaces: n/a
Superseded-By: n/a

# NA-0235A Scope Repair qsl-attachments Lockfile Evidence

## Summary

This governance-only lane repairs `NA-0235A` again so the queue scope matches refreshed contradiction proof on current `main`.

No runtime code, manifests, or lockfiles change in this PR. The lane updates governance and evidence only so the next implementation attempt can remediate the remaining dependency blockers truthfully.

## Refreshed Contradiction Proof On Main

- PR `#695` remains `OPEN` on head `68a3a8081889` and still fails live `advisories` plus `public-safety`.
- The current `NA-0235A` block still does not authorize `qsl-attachments/Cargo.lock`.
- The cross-repo `qsl-attachments` blocker remains real:
  - `qsl/qsl-client/qsc/Cargo.toml:32` still pulls the `qsl-attachments` git dependency into the active qsc dev/test graph.
  - `qsl/qsl-client/qsc/tests/common/mod.rs:5` still actively imports the `qsl_attachments` harness.
  - `/srv/qbuild/work/NA-0235A/qsl-attachments/Cargo.toml:8` still directly pins `rand = "0.8"`.
  - `/srv/qbuild/work/NA-0235A/qsl-attachments/Cargo.lock` still records that dependency in the root package dependency list.
- The active refimpl runtime blocker also remains real:
  - `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs` still imports `rand::{rngs::OsRng, RngCore}` and uses `OsRng.fill_bytes(...)`.
  - `tools/refimpl/quantumshield_refimpl/src/qsp/mod.rs` still carries the same old rand API pattern in the active source tree.
  - `tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs` still carries the same old rand API pattern in the active source tree.
- A minimal scratch proof shows the missing lockfile write is real: replacing the qsl-attachments manifest/source pair with a `rand_core`-style migration and then running `cargo check --locked` fails because Cargo must update `/tmp/.../Cargo.lock`.

## Exact Remaining Active Blockers

- `qsl/qsl-client/qsc/Cargo.toml:32`
- `qsl/qsl-client/qsc/tests/common/mod.rs:5`
- `/srv/qbuild/work/NA-0235A/qsl-attachments/Cargo.toml:8`
- `/srv/qbuild/work/NA-0235A/qsl-attachments/Cargo.lock`
- `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs`
- `tools/refimpl/quantumshield_refimpl/src/qsp/mod.rs`
- `tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs`

The remaining live blocker therefore includes the cross-repo `qsl-attachments` harness path, the required `qsl-attachments/Cargo.lock` update, and active refimpl runtime `rand 0.8` API usage.

## Proof The TUI Stack Is Not The Active Blocker Now

- `cargo tree --locked -i ratatui-termwiz@0.1.0`
- `cargo tree --locked -i termwiz@0.23.3`
- `cargo tree --locked -i phf_generator@0.11.3`

Each inverse tree still prints nothing, so the earlier `ratatui -> ratatui-termwiz -> termwiz -> terminfo -> phf_generator` theory is not the active blocker on refreshed current `main`.

## Why The Missing Lockfile Authorization Matters

- Phase A is required to run `cargo build --locked`, `cargo clippy --locked`, and `cargo test --locked` in `qsl-attachments`.
- The smallest truthful Phase A change still touches `qsl-attachments/Cargo.toml` plus `src/lib.rs`.
- Because `qsl-attachments/Cargo.lock` still records the old `rand` dependency in the root package dependency list, that checked-in lockfile must change too.
- Therefore the current queue block is still understated by one final concrete path: `qsl-attachments/Cargo.lock`.

## Exact Repaired Scope Text As Committed

```md
### NA-0235A — Runtime Dependency Advisory Remediation for Public-Safety Unblock
Status: READY
Problem:
- PR `#695` contains the `NA-0235` workflow/governance repair, and the sanctioned `public-safety` bootstrap now attaches truthfully to the PR head. That gate is failing for the correct reason: the current dependency set still trips live RustSec advisories. Refreshed contradiction proof shows the current `NA-0235A` scope still understated the real dependency-remediation surface because the remaining active blocker now includes the cross-repo `qsl-attachments` test-harness dependency path, the necessary `qsl-attachments/Cargo.lock` update that follows from that Phase A change, and active refimpl runtime `rand 0.8` API usage that requires minimal source compatibility edits: `qsl/qsl-client/qsc/Cargo.toml` still pulls `qsl-attachments`, `qsl/qsl-client/qsc/tests/common/mod.rs` still actively imports that harness, `qsl-attachments/Cargo.toml` still carries the blocking `rand = "0.8"` pin, `qsl-attachments/Cargo.lock` still records that dependency in the root package dependency list, and `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs` still uses `OsRng.fill_bytes(...)` from the old `rand` API while the active source tree still carries the same import pattern in `tools/refimpl/quantumshield_refimpl/src/qsp/mod.rs` and `tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs`. Until those dependency findings are remediated or truthfully proven non-runtime/tooling-only, `NA-0235` cannot merge.
Scope:
- `Cargo.lock`
- `Cargo.toml` only if directly touched by the bounded dependency fix
- `qsl/qsl-client/qsc/Cargo.toml`
- `tools/refimpl/quantumshield_refimpl/Cargo.toml`
- `tools/refimpl/quantumshield_refimpl/src/crypto/**`
- `tools/refimpl/quantumshield_refimpl/src/qsp/**` only if directly touched by minimal dependency/API-compatibility changes required by the remediation
- `tools/refimpl/quantumshield_refimpl/src/suite2/**` only if directly touched by minimal dependency/API-compatibility changes required by the remediation
- `apps/qsl-tui/Cargo.toml`
- `apps/qsl-tui/src/**` only if directly touched by minimal API-compatibility changes required by the dependency remediation
- `qsl/qsl-client/qsc/src/**` only if directly touched by minimal API-compatibility changes required by the dependency remediation
- `qsl/qsl-client/qsc/tests/common/mod.rs`
- `qsl/qsl-client/qsc/tests/**` only if directly touched by minimal API-compatibility changes required by the dependency remediation
- `qsl-attachments/Cargo.toml`
- `qsl-attachments/Cargo.lock`
- `qsl-attachments/src/**` only if directly touched by minimal dependency/API-compatibility changes required by the remediation
- `DECISIONS.md`
- `TRACEABILITY.md`
- docs/governance/evidence only as needed
- no `.github`, website, `qsc-desktop`, `qsl-server`, or `qsl-attachments` service/runtime semantics changes
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
Execution note:
1) if the cross-repo `qsl-attachments` manifest/source change is required, execute `NA-0235A` as a paired implementation set:
   - `qsl-attachments` dependency-fix PR first, including `qsl-attachments/Cargo.lock`
   - `qsl-protocol` dependency-remediation PR second, including refimpl source/API compatibility as needed
2) do not create any new queue item for that pairing; it remains one NA item
Acceptance:
1) the dependency findings blocking `NA-0235` are resolved or truthfully downgraded on the final head
2) the repaired `public-safety` gate remains fail-closed and can pass on the resumed `NA-0235` lane
3) docs-only PRs remain cheap and the fast smoke PR path remains intact
4) no unrelated runtime or workflow drift is introduced
```

## Governance-Only Note

This PR repairs queue truth only. It introduces no runtime changes, no manifest or lockfile changes, and does not remediate the dependency set itself.
