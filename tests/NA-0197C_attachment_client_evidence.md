# NA-0197C Attachment Client Evidence

## Scope

- qsc-only runtime integration against the live `qsl-attachments` service contract/runtime.
- No qsl-server changes.
- No qsl-attachments runtime changes in this item.

## Reused Baseline

- Legacy `file_chunk` / `file_manifest` path remains the `<= 4 MiB` path.
- Existing relay routing and route-token header migration remain authoritative.
- Existing message-plane delivery semantics remain authoritative:
  - attachment upload commit != `accepted_by_relay` != `peer_confirmed`

## Implemented qsc Surfaces

- CLI integration:
  - `file send --attachment-service ...`
  - `receive --attachment-service ...`
- Sender attachment path:
  - generate `enc_ctx_*` per `DOC-CAN-007`
  - stage ciphertext locally under qsc attachment staging
  - create qsl-attachments session
  - upload missing parts
  - commit object
  - emit authenticated `attachment_descriptor`
- Receiver attachment path:
  - parse and validate `attachment_descriptor`
  - fetch ciphertext from qsl-attachments
  - verify ciphertext shape / Merkle root
  - decode `enc_ctx_*`
  - decrypt to local file
  - emit attachment completion confirmation only after verified local persistence
- Resume/persistence:
  - separate attachment journal in `attachments.json`
  - restart-safe outbound/inbound attachment records
  - upload resume via session status + missing part ranges
  - download resume via persisted ciphertext staging + bounded HTTP `Range`

## Deterministic Coverage Added

- `qsl/qsl-client/qsc/tests/attachment_streaming_na0197c.rs`
  - upload resume + invalid resume token fail-closed
  - end-to-end fetch/verify/decrypt/store + peer confirmation after persistence
  - invalid fetch capability + malformed `enc_ctx` fail-closed
  - coexistence rule: `--attachment-service` does not displace legacy send below `<= 4 MiB`
- `qsl/qsl-client/qsc/tests/common/mod.rs`
  - real qsl-attachments runtime harness for integration tests

## Required Regression Proof

- `cargo test -p qsc --locked --test relay_auth_header -- --nocapture`
- `cargo test -p qsc --locked --test tui_relay_config -- --nocapture`
- `cargo test -p qsc --locked --test route_header_migration_docs_na0195a -- --nocapture`
- `cargo test -p qsc --locked --test qsp_qse_onwire -- --nocapture`
- `cargo test -p qsc --locked --test handshake_mvp -- --nocapture`
- `cargo test -p qsc --locked --test identity_secret_at_rest -- --nocapture`

These remained green after the attachment integration changes.

## Coexistence Rule Implemented

- Default client rule:
  - keep the legacy in-message path for `<= 4 MiB`
  - use the attachment service path only for files above `<= 4 MiB` and only when `--attachment-service` is supplied
- This preserves existing small-file behavior while enabling the new attachment plane for larger files.

## Large-File Local Proof

- Required local proof:
  - `QSC_ATTACHMENT_LARGE_BYTES=67108864 cargo test -p qsc --locked --test attachment_streaming_na0197c attachment_large_local_roundtrip_proof -- --ignored --nocapture`
  - Result: PASS
  - Observed duration: `77.30s`
- Stretch attempt:
  - `QSC_ATTACHMENT_LARGE_BYTES=104857600 cargo test -p qsc --locked --test attachment_streaming_na0197c attachment_large_local_roundtrip_proof -- --ignored --nocapture`
  - Result: PASS
  - Observed duration: `117.02s`
- Full-suite attempt:
  - `timeout 300s cargo test -p qsc --locked`
  - Result: NOT COUNTED AS PASS
  - Blocker: the run reached `tests/aws_file_medium_boundary_na0192a.rs`, then the three medium-boundary cases were still running after `60s` each and the overall attempt hit the `300s` timeout cap.

## qsl-attachments Correction Decision

- No qsl-attachments runtime corrections were needed for this item.
- The service runtime satisfied the canonical contract for the implemented qsc path.

## Source of Truth

- `docs/canonical/DOC-CAN-005_QSP_Attachment_Descriptor_and_Control_Plane_v0.1.0_DRAFT.md`
- `docs/canonical/DOC-CAN-006_QATT_Attachment_Service_Contract_v0.1.0_DRAFT.md`
- `docs/canonical/DOC-CAN-007_QATT_Attachment_Encryption_Context_and_Part_Cipher_v0.1.0_DRAFT.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
