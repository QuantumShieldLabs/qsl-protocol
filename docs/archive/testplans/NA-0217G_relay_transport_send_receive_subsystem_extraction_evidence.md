Status: Archive
Owner: QSL governance
Last-Updated: 2026-04-02

# NA-0217G Relay Transport Send/Receive Subsystem Extraction Evidence

Goals: G4, G5

## Scope

- Repo: `qsl-protocol`
- Lane: `NA-0217G`
- Posture: implementation already merged; this closeout is governance-only and introduces no runtime changes
- Implementation PR: #639
- Implementation branch head before merge: `996508b500bd`
- Implementation merge SHA: `429f03c7f5e8`
- Implementation mergedAt: `2026-04-02T03:35:54Z`

## Authority Proof

- refreshed `qsl-protocol` checkout `HEAD`, `mirror/main`, bare mirror `main`, and `origin/main` all resolved to `429f03c7f5e8`
- refreshed merged main contains `DECISIONS.md` `D-0357`, the `TRACEABILITY.md` `NA-0217G implementation/evidence` entry, `qsl/qsl-client/qsc/src/transport/mod.rs`, and `qsl/qsl-client/qsc/tests/transport_contract_na0217g.rs`
- refreshed live queue still showed `READY_COUNT=1` with `NA-0217G` as the sole live `READY` item before this closeout
- refreshed sibling-repo proof stayed read-only and truthful: `qsl-server READY=0`, `qsl-attachments READY=0`

## LOC Proof

- `qsl/qsl-client/qsc/src/main.rs` before merge parent: `17,775` LOC
- `qsl/qsl-client/qsc/src/main.rs` after merged extraction: `16,033` LOC
- `qsl/qsl-client/qsc/src/transport/mod.rs`: `1,741` LOC

## Practical Moved-Helper Inventory

- execution wrappers and relay send/receive entrypoints:
  - `send_execute`
  - `send_abort`
  - `receive_execute`
  - `receive_pull_and_write`
  - `relay_serve`
  - `relay_send`
  - `relay_send_with_payload`
- local relay HTTP parsing / bounded relay I/O helpers:
  - `relay_try_handle_http_inbox`
  - `parse_http_target`
  - `parse_http_route_token`
  - `read_http_request`
  - `find_http_header_end`
  - `write_http_response`
  - `read_frame`
  - `write_frame`
- auth-token, inbox, and transport-fault helpers:
  - `fault_injector_from_env`
  - `fault_injector_from_tui`
  - `relay_auth_token`
  - `relay_auth_token_from_env`
  - `relay_auth_token_from_account_secret`
  - `relay_auth_token_from_token_file`
  - `relay_inbox_push`
  - `relay_inbox_pull`
  - `fault_action_for`
  - `next_fault_index`
- outbox replay / send-state helpers:
  - `outbox_record_load`
  - `outbox_next_state_store`
  - `outbox_next_state_load`
  - `outbox_next_state_clear`
  - `finalize_send_commit`
  - `read_send_state`
- intentionally left for `NA-0217H`:
  - attachment journal / staging ownership
  - file manifest / chunk bookkeeping
  - receipt-linked file-transfer helpers
  - qsl-attachments service interaction
  - attachment confirmation linkage

## No-Drift Proof Surface

### header-carried route-token behavior

- `cargo test --test relay_auth_header`
- `cargo test --test route_header_migration_docs_na0195a`

### bounded receive behavior

- `cargo test --test transport_contract_na0217g`
- the live regression proves FIFO queue order with `max=1` bounded pulls under `qsc relay serve`

### outbox replay semantics

- `cargo test --test ratchet_durability_na0155`

### qsc-desktop-sensitive transport / store proof

- `cargo test --test desktop_gui_contract_na0215b`
- no `qsc-desktop` path changed while relay transport ownership moved from `main.rs` into `transport`

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

### marker / output canary

- `cargo test --test output_marker_contract_na0217a`
- the implementation lane actually ran the marker/output canary because transport helpers still emit user-visible markers and status surfaces after the move

## Implementation / CI Nuance Summary

- the transport seam moved a coherent relay send/receive helper cluster without widening into the attachment pipeline or handshake execution
- the live transport regression proves a FIFO / bounded-pull relay contract under `qsc relay serve`
- the implementation lane needed no CI reruns and all 34 protected checks went green before merge

## Exact Commands / Tests Run For The Merged Implementation Lane

- `cargo fmt --check -- qsl/qsl-client/qsc/src/main.rs qsl/qsl-client/qsc/src/transport/mod.rs qsl/qsl-client/qsc/tests/transport_contract_na0217g.rs`
- `cargo build`
- `cargo clippy -- -D warnings`
- `cargo test --test relay_auth_header`
- `cargo test --test route_header_migration_docs_na0195a`
- `cargo test --test remote_soak_diag_mapping_na0168`
- `cargo test --test ratchet_durability_na0155`
- `cargo test --test desktop_gui_contract_na0215b`
- `cargo test --test timeline_delivery_contract_na0217f`
- `cargo test --test identity_foundation_contract_na0217d`
- `cargo test --test protocol_state_contract_na0217c`
- `cargo test --test fs_store_contract_na0217b`
- `cargo test --test transport_contract_na0217g`
- `cargo test --test output_marker_contract_na0217a`
- `scripts/audit/run_goal_lint_pr.sh 639`

## Why NA-0217G Stayed Narrower Than NA-0217H

- `NA-0217G` moved only the relay transport send/receive seam
- attachment staging/journaling, file manifest/chunk bookkeeping, receipt linkage, confirmation handling, and qsl-attachments service interaction remained in `qsl/qsl-client/qsc/src/main.rs`
- `DOC-QSC-011` orders the attachment / file-transfer pipeline immediately after the transport seam because attachment logic still crosses transport, timeline, and qsl-attachments contract boundaries

## Closeout Note

- This closeout PR is governance-only.
- No runtime, workflow, protocol, relay, attachment-service, or qsc-desktop paths change in this closeout.
