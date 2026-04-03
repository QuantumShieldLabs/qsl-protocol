Status: Archive
Owner: QSL governance
Last-Updated: 2026-04-03

# NA-0217I Handshake Execution Subsystem Extraction Evidence

Goals: G4, G5

## Scope

- Repo: `qsl-protocol`
- Lane: `NA-0217I`
- Posture: implementation already merged; this closeout is governance-only and introduces no runtime changes
- Implementation PR: #643
- Implementation branch head before merge: `0d00e4a8f759`
- Implementation merge SHA: `044ac1009ea8`
- Implementation mergedAt: `2026-04-03T14:16:22Z`

## Authority Proof

- refreshed `qsl-protocol` checkout `HEAD`, `mirror/main`, bare mirror `main`, and `origin/main` all resolved to `044ac1009ea8`
- refreshed merged main contains `DECISIONS.md` `D-0361`, the `TRACEABILITY.md` `NA-0217I implementation/evidence` entry, `qsl/qsl-client/qsc/src/handshake/mod.rs`, and `qsl/qsl-client/qsc/tests/handshake_contract_na0217i.rs`
- refreshed live queue still showed `READY_COUNT=1` with `NA-0217I` as the sole live `READY` item before this closeout
- refreshed sibling-repo proof stayed read-only and truthful: `qsl-server READY=0`, `qsl-attachments READY=0`

## LOC Proof

- `qsl/qsl-client/qsc/src/main.rs` before merge parent: `13,872` LOC
- `qsl/qsl-client/qsc/src/main.rs` after merged extraction: `12,589` LOC
- `qsl/qsl-client/qsc/src/handshake/mod.rs`: `1,293` LOC

## Practical Moved-Helper Inventory

- handshake wire/message types plus encode/decode helpers:
  - `HsInit`
  - `HsResp`
  - `HsConfirm`
  - `hs_encode_init`
  - `hs_decode_init`
  - `hs_encode_resp`
  - `hs_decode_resp`
  - `hs_encode_confirm`
  - `hs_decode_confirm`
- pending-state ownership helpers:
  - `HandshakePending`
  - `hs_pending_legacy_path`
  - `hs_pending_secret_key`
  - `hs_pending_load`
  - `hs_pending_store`
  - `hs_pending_clear`
- transcript / confirm / session-derivation helpers:
  - `hs_transcript_mac`
  - `hs_transcript_hash`
  - `hs_pq_init_ss`
  - `hs_dh_init_from_shared`
  - `hs_confirm_key`
  - `hs_confirm_mac`
  - `hs_sig_msg_b1`
  - `hs_sig_msg_a2`
  - `hs_build_session`
- handshake execution helpers:
  - `handshake_status`
  - `handshake_init`
  - `handshake_poll`
  - `handshake_init_with_route`
  - `perform_handshake_init_with_route`
- pinned-identity mismatch handling kept with the handshake seam:
  - `emit_peer_mismatch`
- intentionally left for `NA-0217J`:
  - final TUI controller ownership
  - headless scripting flow
  - render / layout helpers
  - poll-loop mediation
  - view-state orchestration

## No-Drift Proof Surface

### transcript binding

- `cargo test --test handshake_security_closure`
- `cargo test --test qsp_protocol_gate`

### pinned-identity mismatch reject behavior

- `cargo test --test handshake_security_closure`
- `cargo test --test desktop_gui_contract_na0215b`

### no session mutation on tamper

- `cargo test --test handshake_security_closure`

### qsc-desktop-sensitive handshake / store proof

- `cargo test --test desktop_gui_contract_na0215b`
- the merged handshake regression pins the already-proven desktop-side status truth after a full exchange, including Bob's `status=established_recv_only` / `send_ready_reason=chainkey_unset` outcome

### attachments canary

- `cargo test --test attachment_streaming_na0197c`
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

### marker / output canary

- `cargo test --test output_marker_contract_na0217a`
- the implementation lane ran the marker/output canary because handshake status and mismatch marker surfaces remain user-visible after the extraction

## Implementation / CI Nuance Summary

- the seam moved a coherent handshake execution cluster without widening into final TUI decomposition
- the handshake regression pins the already-proven desktop-side status truth, including Bob's established receive-only / chainkey-unset state
- the implementation lane completed with all 34 protected checks green before merge

## Exact Commands / Tests Run For The Merged Implementation Lane

- `BASE_SHA=$(git rev-parse --verify origin/main^{commit}); HEAD_SHA=$(git rev-parse --verify HEAD^{commit}); EVENT_FILE=$(mktemp /tmp/goal_lint_na0217i_XXXXXX.json); GITHUB_EVENT_PATH="$EVENT_FILE" python3 tools/goal_lint.py`
- `cargo fmt --check`
- `cargo build`
- `cargo clippy -- -D warnings`
- `cargo test --test handshake_security_closure`
- `cargo test --test qsp_protocol_gate`
- `cargo test --test desktop_gui_contract_na0215b`
- `cargo test --test attachment_streaming_na0197c`
- `cargo test --test attachments_contract_na0217h`
- `cargo test --test transport_contract_na0217g`
- `cargo test --test timeline_delivery_contract_na0217f`
- `cargo test --test relay_auth_header`
- `cargo test --test identity_foundation_contract_na0217d`
- `cargo test --test protocol_state_contract_na0217c`
- `cargo test --test fs_store_contract_na0217b`
- `cargo test --test handshake_contract_na0217i`
- `cargo test --test output_marker_contract_na0217a`

## Why NA-0217I Stayed Narrower Than NA-0217J

- `NA-0217I` moved only the handshake execution subsystem frozen in `DOC-QSC-011`
- the final TUI controller, headless scripting flow, render/layout helpers, poll-loop mediation, and view-state orchestration stayed in `qsl/qsl-client/qsc/src/main.rs`
- `DOC-QSC-011` places the final TUI/controller/headless/render decomposition after the handshake seam once the shared business logic is already module-owned elsewhere

## Closeout Note

- This closeout PR is governance-only.
- No runtime, workflow, protocol, relay, qsc-desktop, or sibling-repo paths change in this closeout.
