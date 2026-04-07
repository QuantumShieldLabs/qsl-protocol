Status: Archive
Owner: QSL governance
Last-Updated: 2026-04-07

# NA-0226 qsc TUI Command Mediation / Help-Catalog Decomposition Evidence

Goals: G4, G5

## Scope

- Repo: `qsl-protocol`
- Lane: `NA-0226`
- Posture: implementation already merged; this closeout is governance-only and introduces no runtime changes
- Implementation PR: #673
- Implementation branch head before merge: `534b9fc1125d`
- Implementation merge SHA: `e6c4ca216fd4`
- Implementation mergedAt: `2026-04-07T12:54:00Z`

## Authority Proof

- refreshed `qsl-protocol` checkout `HEAD`, `main`, `mirror/main`, and `origin/main` all resolved to `e6c4ca216fd4`
- refreshed merged main contains `DECISIONS.md` `D-0388`, the `TRACEABILITY.md` `NA-0226 implementation/evidence` entry, the merged `qsl/qsl-client/qsc/src/tui/controller/commands.rs` root, the merged child modules under `qsl/qsl-client/qsc/src/tui/controller/commands/**`, and the merged `qsl/qsl-client/qsc/tests/tui_command_catalog_invariants.rs` surface from PR #673
- refreshed live queue still showed `READY_COUNT=1` with `NA-0226` as the sole live `READY` item before this closeout
- refreshed sibling-repo proof stayed read-only and truthful: `qsl-server READY=0`, `qsl-attachments READY=0`
- PR `#673` is already `MERGED`; refreshed merged `main` is authoritative for queue truth, and this closeout lane does not mutate that PR

## Implementation Evidence Source On Main

- `DECISIONS.md` `D-0388`
- `TRACEABILITY.md` `NA-0226 implementation/evidence`
- `qsl/qsl-client/qsc/src/tui/controller/commands.rs`
- `qsl/qsl-client/qsc/src/tui/controller/commands/catalog.rs`
- `qsl/qsl-client/qsc/src/tui/controller/commands/locked.rs`
- `qsl/qsl-client/qsc/src/tui/controller/commands/relay.rs`
- `qsl/qsl-client/qsc/tests/tui_command_catalog_invariants.rs`

## Exact Implementation Summary

- command/help-catalog concentration was reduced from `4,024 / 9,456` controller-local lines (`42.55%`) to `2,857 / 9,496` (`30.09%`)
- command mediation and help/catalog flow now live in smaller TUI-local modules under `qsl/qsl-client/qsc/src/tui/controller/commands/**`
- no runtime surfaces outside the approved TUI seam changed

## Acceptance-Proof Surface

- dominant residual command-mediation concentration was measurably reduced
- representative TUI suites remained green: `tui_charter.rs`, `tui_product_polish_na0214a.rs`, `tui_fixed_polling.rs`, `tui_relay_drop_reorder.rs`, `tui_contract_na0217j.rs`, `tui_command_catalog_invariants.rs`, and `aws_tui_handshake_na0191.rs`
- desktop/marker contracts remained green where touched: `output_marker_contract_na0217a.rs` and `desktop_gui_contract_na0215b.rs`
- no protocol/service/wire changes beyond the bounded TUI-local decomposition

## Implementation / CI Nuance Summary

- the implementation landed on PR #673 from refreshed `main`
- protected CI completed green before merge
- this closeout PR is governance-only and introduces no runtime changes

## Closeout Note

- `NA-0226` is now closed truthfully because its merged implementation/evidence state is already durable on refreshed `main`
- refreshed residual TUI-local metrics now justify `NA-0227 — qsc TUI State / Poll-Loop Mediation Decomposition` as the sole direct successor because `state.rs` is now the dominant remaining concentration at `3,787 / 9,496` lines (`39.88%`)
