# NA-0113 Delivered Receipts (Client ACK) Plan

## Scope and assumptions
- Scope is limited to `qsl/qsl-client/qsc/**`.
- No server or workflow changes are in scope.
- Delivered receipts are client-generated ACK messages only.
- Relay acceptance and peer delivery remain distinct states.

## Threat model and metadata notes
- Presence risk: immediate ACK can reveal receiver online state.
- Metadata risk: receipt-specific size/class patterns can become an oracle.
- Integrity risk: false delivery claims if ACK is accepted without receiver decrypt/unpack proof.
- Replay/tamper risk: forged or replayed ACK could mutate sender state incorrectly.

## Message and marker schema
- Sender-side delivery markers:
  - `delivered_to_relay` for relay push acceptance.
  - `receipt_recv kind=delivered msg_id=<redacted>` only when peer ACK received/validated.
- Receiver-side ACK marker:
  - `receipt_send kind=delivered msg_id=<redacted> bucket=...` after unpack success.
- Explicit-off marker:
  - `receipt_disabled` when receipt option is not enabled.
- Reject/fail paths:
  - deterministic `event=error code=<...>` markers; no silent fallback.

## Camouflage and bucket policy
- ACK payload is encrypted via the same qsp/qse path as normal messages.
- ACK size is forced into the same bounded small-message bucket class.
- No separate plaintext transport class or server-side receipt opcode is introduced.
- Bucket bounds must remain explicit and deterministic.

## Test vectors
- `receipts_on_happy_path`:
  - sender requests delivered receipt
  - receiver unpacks and emits ACK
  - sender receives deterministic `receipt_recv`.
- `receipts_off_no_ack`:
  - receiver does not emit ACK; sender never observes delivered-to-peer.
- `ack_tamper_reject_no_mutation`:
  - tampered ACK rejected fail-closed; sender state unchanged.
- `ack_replay_reject_no_mutation`:
  - replayed ACK rejected; no duplicate delivery mutation.
- `delayed_ack_still_valid`:
  - deferred/batched ACK accepted without requiring immediate receiver presence.
- `no_secrets_in_receipt_markers`:
  - outputs exclude token/secret/credential patterns.
- `camouflage_bucket_enforced`:
  - ACK maps to configured small-message bucket class.

## Verification checklist
- `cargo fmt -p qsc -- --check`
- `cargo test -p qsc --locked`
- `cargo clippy -p qsc --all-targets -- -D warnings`
- Receipt tests prove on/off semantics, deterministic markers, camouflage bounds, and reject/no-mutation behavior.

## Executed evidence (2026-02-09)
- Local gates executed and passing:
  - `cargo fmt -p qsc -- --check`
  - `cargo test -p qsc --locked`
  - `cargo clippy -p qsc --all-targets -- -D warnings`
- CLI delivery receipt surface implemented:
  - `qsc send --receipt delivered` (explicit request; off by default)
  - `qsc receive --emit-receipts delivered` (explicit emit; off by default)
- Deterministic markers implemented:
  - `receipt_disabled`
  - `receipt_send kind=delivered bucket=small msg_id=<redacted>`
  - `receipt_recv kind=delivered msg_id=<redacted>`
  - `receipt_send_failed code=<deterministic>`
  - `delivered_to_peer kind=delivered msg_id=<redacted>`
- Tests added:
  - `qsl/qsl-client/qsc/tests/receipts_delivered.rs`
    - `receipts_off_no_ack_sent`
    - `delivered_receipt_roundtrip`
    - `ack_camouflage_small_bucket`
    - `no_secrets_in_receipt_outputs`

## Rollback
- Revert NA-0113 implementation commits if:
  - delivered-to-peer semantics regress,
  - ACK camouflage/bounds regress,
  - reject/no-mutation or no-secret invariants regress.
