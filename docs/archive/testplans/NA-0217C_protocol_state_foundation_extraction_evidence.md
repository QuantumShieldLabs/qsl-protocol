Status: Archive
Owner: QSL governance
Last-Updated: 2026-03-31

# NA-0217C Protocol State / Session-at-Rest Foundation Extraction Evidence

Goals: G4, G5

## Scope

- Repo: `qsl-protocol`
- Lane: `NA-0217C`
- Posture: implementation already merged; this closeout is governance-only and introduces no runtime changes
- Implementation PR: #631
- Implementation branch head before merge: `f89578a64ef9`
- Implementation merge SHA: `6f3db8fa089d`

## Authority Proof

- refreshed `qsl-protocol` checkout `HEAD`, `mirror/main`, bare mirror `main`, and `origin/main` all resolved to `6f3db8fa089d`
- refreshed merged main contains `DECISIONS.md` `D-0349`, the `TRACEABILITY.md` `NA-0217C implementation/evidence` entry, `qsl/qsl-client/qsc/src/protocol_state/mod.rs`, and `qsl/qsl-client/qsc/tests/protocol_state_contract_na0217c.rs`
- refreshed live queue still showed `READY_COUNT=1` with `NA-0217C` as the sole live `READY` item before this closeout
- refreshed sibling-repo proof stayed read-only and truthful: `qsl-server READY=0`, `qsl-attachments READY=0`

## LOC Proof

- `qsl/qsl-client/qsc/src/main.rs` before merge parent: `20,992` LOC
- `qsl/qsl-client/qsc/src/main.rs` after merged extraction: `20,546` LOC
- `qsl/qsl-client/qsc/src/protocol_state/mod.rs`: `483` LOC

## Practical Moved-Helper Inventory

- status-truth helpers:
  - `qsp_status_parts`
  - `qsp_status_user_note`
  - `record_qsp_status`
  - `qsp_status_tuple`
  - `qsp_status_string`
  - `qsp_send_ready_tuple`
- fail-closed protocol gating helpers:
  - `protocol_active_or_reason_for_peer`
  - `emit_protocol_inactive`
  - `protocol_inactive_exit`
- encrypted session-at-rest helpers:
  - `qsp_session_load`
  - `qsp_session_store`
  - per-peer session-path/blob helpers
  - encrypted blob encode/decode and legacy migrate helpers
  - session-store key load/create helpers
- deterministic bootstrap helpers kept inside the same foundation seam:
  - `allow_seed_fallback_for_tests`
  - `qsp_seed_from_env`
  - `kmac_out`
  - `qsp_session_for_channel`

## No-Drift Proof Surface

### `ACTIVE` / `INACTIVE` / `protocol_inactive`

- `cargo test --test qsp_protocol_gate`
- `cargo test --test protocol_state_contract_na0217c`

### qsp status tuple semantics

- `cargo test --test qsp_protocol_gate`
- `cargo test --test protocol_state_contract_na0217c`

### session-at-rest ownership semantics

- `cargo test --test session_state_at_rest`
- `cargo test --test handshake_security_closure`

### qsc-desktop-sensitive store / protocol-state proof

- `cargo test --test desktop_gui_contract_na0215b`
- no `qsc-desktop` path changed while protocol-state ownership moved from `main.rs` into `protocol_state`

### fs-store canary

- `cargo test --test fs_store_contract_na0217b`

### marker / output canary

- not claimed as part of the merged `NA-0217C` implementation evidence
- the implementation lane did not run a separate marker/output canary beyond the inactive-gate and desktop/store-sensitive surfaces above

## Exact Commands / Tests Run For The Merged Implementation Lane

- `cargo fmt --check`
- `cargo build`
- `cargo clippy -- -D warnings`
- `cargo test --test qsp_protocol_gate`
- `cargo test --test session_state_at_rest`
- `cargo test --test handshake_security_closure`
- `cargo test --test desktop_gui_contract_na0215b`
- `cargo test --test fs_store_contract_na0217b`
- `cargo test --test protocol_state_contract_na0217c`
- local `tools/goal_lint.py` revalidation via a synthesized event payload using the actual local base/head SHAs

## Why NA-0217C Stayed Narrower Than NA-0217D

- `NA-0217C` moved only protocol activation/status truth plus encrypted session-at-rest ownership
- identity public/secret record helpers, fingerprint helpers, pin reads/writes, and legacy identity-migration helpers stayed in `qsl/qsl-client/qsc/src/main.rs`
- `DOC-QSC-011` orders that identity-record / pin foundation as the direct successor once the protocol-state seam is merged

## Closeout Note

- This closeout PR is governance-only.
- No runtime, workflow, protocol, relay, attachment, or qsc-desktop paths change in this closeout.
