Status: Archive
Owner: QSL governance
Last-Updated: 2026-04-08

# NA-0228 qsc TUI Command Residual Shell / Dispatch Decomposition Evidence

Goals: G4, G5

## Scope

- Repo: `qsl-protocol`
- Lane: `NA-0228`
- Posture: implementation already merged; this closeout is governance-only and introduces no runtime changes
- Implementation PR: #677
- Implementation branch head before merge: `67c8ed5a16c6`
- Implementation merge SHA: `dca4cb7e127e`
- Implementation mergedAt: `2026-04-09T00:44:41Z`

## Authority Proof

- before this governance-only closeout branch mutated docs, refreshed `qsl-protocol` `main`, `mirror/main`, and `origin/main` all resolved to `dca4cb7e127e`
- refreshed merged main contains `DECISIONS.md` `D-0392`, the `TRACEABILITY.md` `NA-0228 implementation/evidence` entry, the merged `qsl/qsl-client/qsc/src/tui/controller/commands.rs` root, the merged child modules under `qsl/qsl-client/qsc/src/tui/controller/commands/**`, and the merged `qsl/qsl-client/qsc/tests/tui_command_catalog_invariants.rs` surface from PR #677
- refreshed live queue still showed `READY_COUNT=1` with `NA-0228` as the sole live `READY` item before this closeout
- refreshed sibling-repo proof stayed read-only and truthful: `qsl-server READY=0`, `qsl-attachments READY=0`
- PR `#677` is already `MERGED`; refreshed merged `main` is authoritative for queue truth, and this closeout lane does not mutate that PR

## Implementation Evidence Source On Main

- `DECISIONS.md` `D-0392`
- `TRACEABILITY.md` `NA-0228 implementation/evidence`
- `qsl/qsl-client/qsc/src/tui/controller/commands.rs`
- `qsl/qsl-client/qsc/src/tui/controller/commands/catalog.rs`
- `qsl/qsl-client/qsc/src/tui/controller/commands/contacts.rs`
- `qsl/qsl-client/qsc/src/tui/controller/commands/dispatch.rs`
- `qsl/qsl-client/qsc/src/tui/controller/commands/key.rs`
- `qsl/qsl-client/qsc/src/tui/controller/commands/locked.rs`
- `qsl/qsl-client/qsc/src/tui/controller/commands/messages.rs`
- `qsl/qsl-client/qsc/src/tui/controller/commands/navigation.rs`
- `qsl/qsl-client/qsc/src/tui/controller/commands/relay.rs`
- `qsl/qsl-client/qsc/tests/tui_command_catalog_invariants.rs`

## Exact Implementation Summary

- command-shell / dispatch concentration was reduced from `2,857 / 9,072` controller-local lines (`31.49%`) to `179 / 9,033` (`1.98%`)
- residual command-shell and dispatch flow now live in smaller TUI-local modules under `qsl/qsl-client/qsc/src/tui/controller/commands/**`
- no runtime surfaces outside the approved TUI seam changed

## Acceptance-Proof Surface

- dominant residual command-shell concentration was measurably reduced
- representative TUI suites remained green: `tui_charter.rs`, `tui_product_polish_na0214a.rs`, `tui_fixed_polling.rs`, `tui_relay_drop_reorder.rs`, `tui_contract_na0217j.rs`, `tui_command_catalog_invariants.rs`, and `aws_tui_handshake_na0191.rs`
- desktop/marker contracts remained green where touched: `output_marker_contract_na0217a.rs` and `desktop_gui_contract_na0215b.rs`
- no protocol/service/wire changes beyond the bounded TUI-local decomposition

## Implementation / CI Nuance Summary

- the implementation landed on PR #677 from refreshed `main`
- protected CI completed green before merge
- this closeout PR is governance-only and introduces no runtime changes

## Closeout Note

- `NA-0228` is now closed truthfully because its merged implementation/evidence state is already durable on refreshed `main`
- refreshed residual TUI-local metrics now justify `NA-0229 — qsc TUI State Residual Shell / Ownership Mediation Decomposition` as the sole direct successor because `state.rs` is now the dominant remaining concentration at `2,336 / 9,033` lines (`25.86%`), with ownership/account-state mediation still concentrated across `state.rs` and `state/ownership.rs`
