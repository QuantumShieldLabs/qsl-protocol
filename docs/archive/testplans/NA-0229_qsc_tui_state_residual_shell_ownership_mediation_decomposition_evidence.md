Status: Archive
Owner: QSL governance
Last-Updated: 2026-04-09

# NA-0229 qsc TUI State Residual Shell / Ownership Mediation Decomposition Evidence

Goals: G4, G5

## Scope

- Repo: `qsl-protocol`
- Lane: `NA-0229`
- Posture: implementation already merged; this closeout is governance-only and introduces no runtime changes
- Implementation PR: #679
- Implementation branch head before merge: `45b7fb4c5379`
- Implementation merge SHA: `c7e224a0f413`
- Implementation mergedAt: `2026-04-09T03:23:22Z`

## Authority Proof

- before this governance-only closeout branch mutated docs, refreshed `qsl-protocol` `main`, `mirror/main`, and `origin/main` all resolved to `c7e224a0f413`
- refreshed merged main contains `DECISIONS.md` `D-0394`, the `TRACEABILITY.md` `NA-0229 implementation/evidence` entry, the merged `qsl/qsl-client/qsc/src/tui/controller/state.rs` root, the child modules under `qsl/qsl-client/qsc/src/tui/controller/state/**`, and the merged `qsl/qsl-client/qsc/tests/tui_command_catalog_invariants.rs` surface from PR #679
- refreshed live queue still showed `READY_COUNT=1` with `NA-0229` as the sole live `READY` item before this closeout
- refreshed sibling-repo proof stayed read-only and truthful: `qsl-server READY=0`, `qsl-attachments READY=0`
- PR `#679` is already `MERGED`; refreshed merged `main` is authoritative for queue truth, and this closeout lane does not mutate that PR

## Implementation Evidence Source On Main

- `DECISIONS.md` `D-0394`
- `TRACEABILITY.md` `NA-0229 implementation/evidence`
- `qsl/qsl-client/qsc/src/tui/controller/state.rs`
- `qsl/qsl-client/qsc/src/tui/controller/state/account.rs`
- `qsl/qsl-client/qsc/src/tui/controller/state/ownership.rs`
- `qsl/qsl-client/qsc/src/tui/controller/state/poll.rs`
- `qsl/qsl-client/qsc/tests/tui_command_catalog_invariants.rs`

## Exact Implementation Summary

- state / ownership concentration was reduced from `2,336 / 9,033` controller-local lines (`25.86%`) to `1,756 / 9,046` (`19.41%`)
- residual account/contact/timeline/file-state flow now lives in smaller TUI-local modules under `qsl/qsl-client/qsc/src/tui/controller/state/**`
- no runtime surfaces outside the approved TUI seam changed

## Acceptance-Proof Surface

- dominant residual state / ownership concentration was measurably reduced
- representative TUI suites remained green: `tui_charter.rs`, `tui_product_polish_na0214a.rs`, `tui_fixed_polling.rs`, `tui_relay_drop_reorder.rs`, `tui_contract_na0217j.rs`, `tui_command_catalog_invariants.rs`, and `aws_tui_handshake_na0191.rs`
- desktop/marker contracts remained green where touched: `output_marker_contract_na0217a.rs` and `desktop_gui_contract_na0215b.rs`
- no protocol/service/wire changes beyond the bounded TUI-local decomposition

## Implementation / CI Nuance Summary

- the implementation landed on PR #679 from refreshed `main`
- protected CI completed green before merge
- this closeout PR is governance-only and introduces no runtime changes

## Closeout Note

- `NA-0229` is now closed truthfully because its merged implementation/evidence state is already durable on refreshed `main`
- the staged 8-file security audit packet is now in repo truth under `docs/audit/incoming/2026-04-09_security_batch/`
- refreshed repo truth now justifies `NA-0230 — Security Audit Packet Intake / Verification / Remediation Plan Canon` as the sole direct successor because the packet has been preserved in-repo but has not yet been canonically ingested, de-duplicated, verified against current `main`, or turned into a bounded remediation plan
