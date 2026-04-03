Status: Archive
Owner: QSL governance
Last-Updated: 2026-04-03

# NA-0217H Attachment / File-Transfer Pipeline Extraction Evidence

Goals: G4, G5

## Scope

- Repo: `qsl-protocol`
- Lane: `NA-0217H`
- Posture: implementation already merged; this closeout is governance-only and introduces no runtime changes
- Implementation PR: #641
- Implementation branch head before merge: `701ff8cab916`
- Implementation merge SHA: `27403b581e48`
- Implementation mergedAt: `2026-04-03T03:26:37Z`

## Authority Proof

- refreshed `qsl-protocol` checkout `HEAD`, `mirror/main`, bare mirror `main`, and `origin/main` all resolved to `27403b581e48`
- refreshed merged main contains `DECISIONS.md` `D-0359`, the `TRACEABILITY.md` `NA-0217H implementation/evidence` entry, `qsl/qsl-client/qsc/src/attachments/mod.rs`, and `qsl/qsl-client/qsc/tests/attachments_contract_na0217h.rs`
- refreshed live queue still showed `READY_COUNT=1` with `NA-0217H` as the sole live `READY` item before this closeout
- refreshed sibling-repo proof stayed read-only and truthful: `qsl-server READY=0`, `qsl-attachments READY=0`

## LOC Proof

- `qsl/qsl-client/qsc/src/main.rs` before merge parent: `16,033` LOC
- `qsl/qsl-client/qsc/src/main.rs` after merged extraction: `13,872` LOC
- `qsl/qsl-client/qsc/src/attachments/mod.rs`: `2,176` LOC

## Practical Moved-Helper Inventory

- attachment staging / journaling / path-policy helpers:
  - `attachment_stage_root`
  - `attachment_staging_dir`
  - `attachment_outbound_rel`
  - `attachment_inbound_rel`
  - `attachment_path_from_rel`
  - `validated_legacy_in_message_stage_from_env`
- manifest / chunk / ciphertext bookkeeping helpers:
  - `attachment_part_size_bytes`
  - `choose_attachment_part_size_class`
  - `attachment_plaintext_capacity`
  - `attachment_part_count_for_plaintext`
  - `attachment_ciphertext_len_for_plaintext`
  - `attachment_ciphertext_part_len`
  - `attachment_nonce`
  - `attachment_part_aad`
  - `attachment_merkle_leaf`
  - `attachment_merkle_root`
  - `file_xfer_chunk_hash`
  - `file_xfer_id`
  - `file_xfer_manifest_hash`
  - `file_xfer_confirm_id`
- qsl-attachments service/session helpers:
  - `attachment_service_create_session`
  - `attachment_service_status`
  - `attachment_service_upload_part`
  - `attachment_service_commit`
  - `attachment_service_reason`
  - `attachment_upload_missing_parts`
- outbound / inbound descriptor and transfer helpers:
  - `attachment_build_outbound_record`
  - `attachment_build_descriptor`
  - `attachment_send_execute`
  - `attachment_record_matches_descriptor`
  - `attachment_validate_descriptor`
  - `attachment_inbound_record_from_descriptor`
  - `attachment_fetch_ciphertext`
  - `attachment_verify_ciphertext_root`
  - `attachment_decrypt_to_output`
  - `attachment_process_inbound_record`
  - `relay_send_file_payload_with_retry`
- confirmation / marker-adjacent helpers tightly coupled to the seam:
  - `attachment_confirm_handle`
  - `attachment_generate_id`
  - `attachment_build_enc_ctx`
  - `attachment_decode_enc_ctx`
  - `attachment_validate_filename_hint`
  - `attachment_output_name`
  - `attachment_find_outbound_by_source`
  - `file_push_retryable`
  - `emit_file_push_retry`
- intentionally left for `NA-0217I`:
  - handshake encode/decode
  - pending-state transitions
  - init/poll execution
  - transcript checks
  - identity-bound mismatch handling
  - shell/UI orchestration

## No-Drift Proof Surface

### attachment send / receive posture

- `cargo test --test attachment_streaming_na0197c`
- `cargo test --test attachments_contract_na0217h`

### file / attachment confirmation linkage

- `cargo test --test attachments_contract_na0217h`
- the regression proves service-backed attachment delivery remains `accepted_by_relay` until the receiver emits the attachment confirmation and only then advances to `peer_confirmed`

### qsl-attachments interaction boundaries

- `cargo test --test attachment_streaming_na0197c`
- `cargo test --test attachments_contract_na0217h`
- no `qsl-attachments/**` runtime path changed in the implementation or closeout lanes

### honest file / attachment confirmation state

- `cargo test --test message_state_model`
- `cargo test --test attachments_contract_na0217h`

### qsc-desktop-sensitive attachment / store proof

- `cargo test --test desktop_gui_contract_na0215b`
- no `qsl/qsl-client/qsc-desktop/**` path changed while attachment/store-adjacent helpers moved from `main.rs` into `attachments`

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
- the implementation lane ran the marker/output canary because attachment confirmation and status-adjacent marker surfaces remain user-visible after the attachment extraction

## Implementation / CI Nuance Summary

- the seam moved a coherent attachment / file-transfer cluster without widening into handshake execution or TUI code
- the live attachments regression proves service-backed attachment delivery remains `accepted_by_relay` until the receiver emits the attachment confirmation and only then advances to `peer_confirmed`
- the implementation lane completed with all 34 protected checks green before merge

## Exact Commands / Tests Run For The Merged Implementation Lane

- `GITHUB_EVENT_PATH=/tmp/goal_lint_na0217h_nda_1i06.json python3 tools/goal_lint.py`
- `cargo fmt --check`
- `cargo build`
- `cargo clippy -- -D warnings`
- `cargo test --test attachment_streaming_na0197c`
- `cargo test --test message_state_model`
- `cargo test --test adversarial_properties`
- `cargo test --test desktop_gui_contract_na0215b`
- `cargo test --test transport_contract_na0217g`
- `cargo test --test timeline_delivery_contract_na0217f`
- `cargo test --test relay_auth_header`
- `cargo test --test identity_foundation_contract_na0217d`
- `cargo test --test protocol_state_contract_na0217c`
- `cargo test --test fs_store_contract_na0217b`
- `cargo test --test attachments_contract_na0217h`
- `cargo test --test output_marker_contract_na0217a`

## Why NA-0217H Stayed Narrower Than NA-0217I

- `NA-0217H` moved only the attachment / file-transfer pipeline frozen in `DOC-QSC-011`
- handshake encode/decode, pending-state transitions, init/poll execution, transcript checks, and identity-bound mismatch handling stayed in `qsl/qsl-client/qsc/src/main.rs`
- `DOC-QSC-011` orders handshake execution immediately after the attachment seam once `qsl/qsl-client/qsc/src/attachments/mod.rs` owns attachment staging/journaling, manifest/chunk bookkeeping, confirmation linkage, and qsl-attachments service helpers

## Closeout Note

- This closeout PR is governance-only.
- No runtime, workflow, protocol, relay, attachment-service, qsc-desktop, or sibling-repo paths change in this closeout.
