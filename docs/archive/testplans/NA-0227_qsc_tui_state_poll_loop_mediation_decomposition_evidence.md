Status: Archive
Owner: QSL governance
Last-Updated: 2026-04-07

# NA-0227 qsc TUI State / Poll-Loop Mediation Decomposition Evidence

Goals: G4, G5

## Scope

- Repo: `qsl-protocol`
- Lane: `NA-0227`
- Posture: implementation already merged; this closeout is governance-only and introduces no runtime changes
- Implementation PR: #675
- Implementation branch head before merge: `14573681433f`
- Implementation merge SHA: `6aa48816879e`
- Implementation mergedAt: `2026-04-08T02:19:18Z`

## Authority Proof

- refreshed `qsl-protocol` checkout `HEAD`, `main`, `mirror/main`, and `origin/main` all resolved to `6aa48816879e`
- refreshed merged main contains `DECISIONS.md` `D-0390`, the corrected `TRACEABILITY.md` `NA-0227 implementation/evidence` entry, the merged `qsl/qsl-client/qsc/src/tui/controller/state.rs` root, the merged child modules under `qsl/qsl-client/qsc/src/tui/controller/state/**`, and the merged `qsl/qsl-client/qsc/tests/tui_command_catalog_invariants.rs` surface from PR #675
- refreshed live queue still showed `READY_COUNT=1` with `NA-0227` as the sole live `READY` item before this closeout
- refreshed sibling-repo proof stayed read-only and truthful: `qsl-server READY=0`, `qsl-attachments READY=0`
- PR `#675` is already `MERGED`; refreshed merged `main` is authoritative for queue truth, and this closeout lane does not mutate that PR

## Implementation Evidence Source On Main

- `DECISIONS.md` `D-0390`
- `TRACEABILITY.md` `NA-0227 implementation/evidence`
- `qsl/qsl-client/qsc/src/tui/controller/state.rs`
- `qsl/qsl-client/qsc/src/tui/controller/state/ownership.rs`
- `qsl/qsl-client/qsc/src/tui/controller/state/poll.rs`
- `qsl/qsl-client/qsc/tests/tui_command_catalog_invariants.rs`

## Exact Implementation Summary

- state/poll-loop concentration was reduced from `3,787 / 9,045` controller-local lines (`41.87%`) to `2,336 / 9,072` (`25.75%`)
- state ownership and poll-loop mediation now live in smaller TUI-local modules under `qsl/qsl-client/qsc/src/tui/controller/state/**`
- no runtime surfaces outside the approved TUI seam changed

## Acceptance-Proof Surface

- dominant residual state/poll-loop concentration was measurably reduced
- representative TUI suites remained green: `tui_charter.rs`, `tui_product_polish_na0214a.rs`, `tui_fixed_polling.rs`, `tui_relay_drop_reorder.rs`, `tui_contract_na0217j.rs`, `tui_command_catalog_invariants.rs`, and `aws_tui_handshake_na0191.rs`
- desktop/marker contracts remained green where touched: `output_marker_contract_na0217a.rs` and `desktop_gui_contract_na0215b.rs`
- no protocol/service/wire changes beyond the bounded TUI-local decomposition

## Implementation / CI Nuance Summary

- the implementation landed on PR #675 from refreshed `main`
- protected CI completed green before merge
- this closeout corrects stale metric references from the implementation/evidence entry because refreshed merged counts differ from the pre-final-fix draft values
- this closeout PR is governance-only and introduces no runtime changes

## Closeout Note

- `NA-0227` is now closed truthfully because its merged implementation/evidence state is already durable on refreshed `main`
- refreshed residual TUI-local metrics now justify `NA-0228 — qsc TUI Command Residual Shell / Dispatch Decomposition` as the sole direct successor because `commands.rs` is now the dominant remaining concentration at `2,857 / 9,072` lines (`31.49%`)
