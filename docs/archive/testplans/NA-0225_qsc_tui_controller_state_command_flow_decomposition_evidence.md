Status: Archive
Owner: QSL governance
Last-Updated: 2026-04-07

# NA-0225 qsc TUI Controller State / Command-Flow Decomposition Evidence

Goals: G4, G5

## Scope

- Repo: `qsl-protocol`
- Lane: `NA-0225`
- Posture: implementation already merged; this closeout is governance-only and introduces no runtime changes
- Implementation PR: #671
- Implementation branch head before merge: `fa7ef6420aff`
- Implementation merge SHA: `1b1dbc22009d`
- Implementation mergedAt: `2026-04-07T03:18:57Z`

## Authority Proof

- refreshed `qsl-protocol` checkout `HEAD`, `main`, `mirror/main`, and `origin/main` all resolved to `1b1dbc22009d`
- refreshed merged main contains `DECISIONS.md` `D-0386`, the `TRACEABILITY.md` `NA-0225 implementation/evidence` entry, the merged `qsl/qsl-client/qsc/src/tui/controller.rs` orchestration shell, the merged child modules under `qsl/qsl-client/qsc/src/tui/controller/**`, and the merged `qsl/qsl-client/qsc/tests/tui_command_catalog_invariants.rs` surface from PR #671
- refreshed live queue still showed `READY_COUNT=1` with `NA-0225` as the sole live `READY` item before this closeout
- refreshed sibling-repo proof stayed read-only and truthful: `qsl-server READY=0`, `qsl-attachments READY=0`
- PR `#671` is already `MERGED`; refreshed merged `main` is authoritative for queue truth, and this closeout lane does not mutate that PR

## Implementation Evidence Source On Main

- `DECISIONS.md` `D-0386`
- `TRACEABILITY.md` `NA-0225 implementation/evidence`
- `qsl/qsl-client/qsc/src/tui/controller.rs`
- `qsl/qsl-client/qsc/src/tui/controller/commands.rs`
- `qsl/qsl-client/qsc/src/tui/controller/render.rs`
- `qsl/qsl-client/qsc/src/tui/controller/state.rs`
- `qsl/qsl-client/qsc/tests/tui_command_catalog_invariants.rs`

## Exact Implementation Summary

- `qsl/qsl-client/qsc/src/tui/controller.rs` concentration was reduced from `9,417 / 9,839` TUI lines (`95.71%`) to `451 / 9,878` TUI lines (`4.57%`)
- controller state, command-flow mediation, and render orchestration now live in smaller TUI-local modules under `qsl/qsl-client/qsc/src/tui/controller/**`
- no runtime surfaces outside the approved TUI seam changed

## Acceptance-Proof Surface

- dominant controller concentration was measurably reduced
- representative TUI suites remained green: `tui_charter.rs`, `tui_product_polish_na0214a.rs`, `tui_fixed_polling.rs`, `tui_relay_drop_reorder.rs`, `tui_contract_na0217j.rs`, `tui_command_catalog_invariants.rs`, and `aws_tui_handshake_na0191.rs`
- desktop/marker contracts remained green where touched: `output_marker_contract_na0217a.rs` and `desktop_gui_contract_na0215b.rs`
- no protocol/service/wire changes beyond the bounded TUI decomposition

## Implementation / CI Nuance Summary

- the implementation landed on PR #671 from refreshed `main`
- protected CI completed green before merge
- this closeout PR is governance-only and introduces no runtime changes

## Closeout Note

- `NA-0225` is now closed truthfully because its merged implementation/evidence state is already durable on refreshed `main`
- refreshed residual TUI-local metrics now justify `NA-0226 — qsc TUI Command Mediation / Help-Catalog Decomposition` as the sole direct successor
