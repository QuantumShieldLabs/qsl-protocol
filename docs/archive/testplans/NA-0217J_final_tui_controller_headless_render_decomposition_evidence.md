Status: Archive
Owner: QSL governance
Last-Updated: 2026-04-03

# NA-0217J Final TUI Controller / Headless / Render Decomposition Evidence

Goals: G4, G5

## Scope

- Repo: `qsl-protocol`
- Lane: `NA-0217J`
- Posture: implementation already merged; this closeout is governance-only and introduces no runtime changes
- Implementation PR: #645
- Implementation branch head before merge: `a6242c4ad45a`
- Implementation merge SHA: `2a0379a97ce5`
- Implementation mergedAt: `2026-04-03T16:50:16Z`

## Authority Proof

- refreshed `qsl-protocol` checkout `HEAD`, `mirror/main`, bare mirror `main`, and `origin/main` all resolved to `2a0379a97ce5`
- refreshed merged main contains `DECISIONS.md` `D-0363`, the `TRACEABILITY.md` `NA-0217J implementation/evidence` entry, `qsl/qsl-client/qsc/src/tui/mod.rs`, and `qsl/qsl-client/qsc/tests/tui_contract_na0217j.rs`
- refreshed live queue still showed `READY_COUNT=1` with `NA-0217J` as the sole live `READY` item before this closeout
- refreshed sibling-repo proof stayed read-only and truthful: `qsl-server READY=0`, `qsl-attachments READY=0`

## LOC Proof

- `qsl/qsl-client/qsc/src/main.rs` before merge parent: `12,589` LOC
- `qsl/qsl-client/qsc/src/main.rs` after merged extraction: `2,933` LOC
- `qsl/qsl-client/qsc/src/tui/**`: `9,839` LOC

## Practical Moved-Helper Inventory

- shared TUI boundary and type ownership:
  - `tui_entry`
  - `TuiState`
  - shared `TuiMode` / inspector / poll / config / status / session / file-item structures in `tui/mod.rs`
- controller / headless / poll-loop ownership:
  - headless script execution flow
  - command dispatch
  - key handling
  - view-state orchestration
  - help/catalog rendering
  - inspector/home/status orchestration
  - draw glue and poll-loop mediation in `tui/controller.rs`
- script ownership:
  - `load_tui_script`
  - `parse_tui_command`
  - `parse_tui_command_tokens`
- render/layout ownership:
  - `internal_divider_style`
  - divider helpers
  - padding/truncation helpers
  - contacts-row formatting helpers in `tui/render.rs`
- intentionally left outside the seam:
  - `output/**` ownership
  - `fs_store/**` ownership
  - `protocol_state/**` ownership
  - `identity/**` ownership
  - `contacts/**` ownership
  - `timeline/**` ownership
  - `transport/**` ownership
  - `attachments/**` ownership
  - `handshake/**` ownership
  - broader non-TUI CLI helpers in `main.rs`

## No-Drift Proof Surface

### deterministic marker / output truth

- `cargo test --test output_marker_contract_na0217a`
- `cargo test --test tui_contract_na0217j`

### fixed polling behavior

- `cargo test --test tui_fixed_polling`

### headless scripting behavior

- `cargo test --test tui_contract_na0217j`

### relay drop / reorder presentation semantics

- `cargo test --test tui_relay_drop_reorder`

### qsc-desktop-sensitive TUI / store proof

- `cargo test --test desktop_gui_contract_na0215b`
- `cargo test --test fs_store_contract_na0217b`

### handshake canary

- `cargo test --test handshake_contract_na0217i`

### attachments canary

- `cargo test --test attachments_contract_na0217h`

### transport canary

- `cargo test --test transport_contract_na0217g`

### timeline canary

- `cargo test --test timeline_delivery_contract_na0217f`

### contacts canary

- `cargo test --test relay_auth_header`

### identity canary

- `cargo test --test identity_foundation_contract_na0217d`

### protocol_state canary

- `cargo test --test protocol_state_contract_na0217c`

### fs_store canary

- `cargo test --test fs_store_contract_na0217b`

## Implementation / CI Nuance Summary

- the seam moved the final TUI/controller/headless/render shell without re-coupling protocol/service logic
- the new deterministic TUI/headless regression proves equivalent runs emit the same stable headless/controller markers
- the only CI follow-up was a command-catalog invariant ownership update to scan the live `src/tui/**` command-source set rather than the old monolithic `main.rs`
- the implementation lane completed with all 34 protected checks green before merge
- this closeout PR is governance-only and introduces no runtime changes

## Exact Commands / Tests Run For The Merged Implementation Lane

- `GITHUB_EVENT_NAME=pull_request GITHUB_EVENT_PATH="$EVENT_FILE" python3 tools/goal_lint.py`
- `cargo fmt --check`
- `cargo build`
- `cargo clippy -- -D warnings`
- `cargo test --test tui_charter`
- `cargo test --test tui_product_polish_na0214a`
- `cargo test --test tui_fixed_polling`
- `cargo test --test tui_relay_drop_reorder`
- `cargo test --test desktop_gui_contract_na0215b`
- `cargo test --test output_marker_contract_na0217a`
- `cargo test --test handshake_contract_na0217i`
- `cargo test --test attachments_contract_na0217h`
- `cargo test --test transport_contract_na0217g`
- `cargo test --test timeline_delivery_contract_na0217f`
- `cargo test --test relay_auth_header`
- `cargo test --test identity_foundation_contract_na0217d`
- `cargo test --test protocol_state_contract_na0217c`
- `cargo test --test fs_store_contract_na0217b`
- `cargo test --test tui_contract_na0217j`
- post-CI-fix reruns:
  - `cargo test --test tui_command_catalog_invariants`
  - `cargo fmt --check`
  - `cargo build`
  - `cargo clippy -- -D warnings`
  - `cargo test --test tui_contract_na0217j`

## Why NA-0217J Completes The Modularization Wave

- `DOC-QSC-011` places TUI decomposition as the late consumer wave after the shared logic is already isolated out of `main.rs`
- refreshed merged main now carries dedicated module ownership for output, fs-store, protocol-state, identity, contacts, timeline, transport, attachments, handshake, and the final TUI/controller/headless/render seam
- the next truthful blocker is continuity/runbook/goal-roadmap canon rather than another runtime extraction

## Closeout Note

- This closeout PR is governance-only.
- No runtime, workflow, protocol, relay, qsc-desktop, qsl-server, or qsl-attachments paths change in this closeout.
