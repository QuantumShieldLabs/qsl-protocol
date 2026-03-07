# NA-0092 — QSP/QSE on-wire enforcement plan

## Scope & assumptions
- qsc send/receive must pack/unpack QSP/QSE envelopes on-wire.
- Server remains ciphertext-only; no protocol-core changes.

## Envelope format proof strategy
- Identify envelope header/magic/version from refimpl.
- Assert on-wire bytes contain header and not raw payload (no plaintext match).

## Pack/unpack call-site proof
- Send path: pack/encrypt before relay/inbox push body creation.
- Receive path: verify/decrypt/unpack after pull, before write.

## ACTIVE/INACTIVE truth table
- ACTIVE: pack/unpack executed successfully for current operation.
- INACTIVE: pack/unpack unavailable/failed; must include reason.

## Test vectors
- Send path on-wire bytes include envelope header; raw payload not present.
- Receive path rejects invalid/oversize/malformed envelopes deterministically.
- No-mutation on reject; no secrets in markers/output.

## Verification checklist
- cargo test -p qsc --locked
- cargo clippy -p qsc --all-targets -- -D warnings
- Traceability updated with implementation PR link

## Executed / evidence
- Tests added/verified:
  - qsl/qsl-client/qsc/tests/qsp_qse_onwire.rs:
    - on_wire_is_envelope_not_raw
    - tamper_rejects_no_write
    - status_truthy_active_inactive
- Local gates (isolated caches under _forensics):
  - cargo fmt -p qsc -- --check
  - cargo test -p qsc --locked
  - cargo clippy -p qsc --all-targets -- -D warnings

## Rollback
- Revert qsc pack/unpack wiring and tests; restore prior send/receive behavior.
