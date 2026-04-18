Goals: G4

Status: Archive
Owner: QSL governance
Last-Updated: 2026-04-17
Replaces: n/a
Superseded-By: n/a

# NA-0235A Runtime Dependency Advisory Remediation Evidence

## Summary

`NA-0235A` resolved the live dependency-advisory blocker as a merged paired implementation set across `qsl-attachments` and `qsl-protocol`.

qsl-attachments Phase A PR: #30, `NA-0235A qsl-attachments rand remediation`
qsl-attachments Phase A merge SHA: `a1a4c1269899`
qsl-attachments hotfix PR: #31, `NA-0235A qsl-attachments macOS width fix`
qsl-attachments hotfix merge SHA: `1e1ae272a4cb`
qsl-protocol implementation PR: #702, `NA-0235A protocol dependency unblock`
qsl-protocol implementation merge SHA: `2113201edff6`

## Merged-State Proof

- qsl-attachments Phase A merged first as PR #30 (`a1a4c1269899`), replacing its single `rand 0.8` use with `rand_core` and refreshing the sibling-repo lockfile.
- qsl-attachments salvage hotfix PR #31 then merged as `1e1ae272a4cb`; refreshed qsl-attachments `main` carries that merge commit durably and therefore carries the Phase A remediation plus the minimal macOS width-compatibility fix in `src/lib.rs`.
- qsl-protocol PR #702 merged as `2113201edff6`; refreshed qsl-protocol `main` carries that merge commit durably.
- Refreshed qsl-protocol `main` now passes `cargo audit --deny warnings`, which is the dependency-audit proof `NA-0235A` needed to unblock the repaired fail-closed `public-safety` gate.
- PR `#695` remains OPEN and unmerged, so the next truthful queue state is to close `NA-0235A` and resume `NA-0235` from refreshed `main`.

## Exact Implementation Outcome On Main

- The cross-repo qsl-attachments harness path no longer carries the live `rand 0.8` blocker; the sibling repo now uses `rand_core`, and refreshed qsl-attachments `main` includes the macOS salvage needed to keep the paired implementation truthful across platforms.
- The active refimpl runtime path no longer carries the old `rand 0.8` API shape; refreshed qsl-protocol `main` migrated the bounded refimpl RNG imports to `rand_core`.
- The stale `ratatui-termwiz -> termwiz -> phf_generator -> rand 0.8.5` lock path is gone from refreshed qsl-protocol `main`; the bounded TUI dependency replacement is merged.
- The runtime `rustls-webpki 0.103.10` blocker is removed on refreshed qsl-protocol `main`; the merged implementation now carries `rustls-webpki 0.103.12`.
- The tooling-only `rand 0.9.x` blocker is also resolved on refreshed qsl-protocol `main`.
- No `.github`, policy-downgrade, advisory-ignore, qsl-server, qsc-desktop, website/public-runtime, or unrelated protocol/service semantic change was required.

## Queue Transition Truth

- `NA-0235A` is now complete from already-merged implementation state.
- The old blocker on `NA-0235` was dependency health under the repaired `public-safety` gate.
- Refreshed `main` now resolves that blocker through the merged `NA-0235A` implementation set.
- PR `#695` predates that merged dependency remediation and remains OPEN, so the next truthful READY lane is restoring `NA-0235` and resuming it from refreshed `main`.

## Closeout Scope

This closeout PR is governance-only. It archives durable merged evidence, marks `NA-0235A` `DONE`, and restores `NA-0235` as the sole READY item without reopening runtime implementation or mutating PR `#695`.
