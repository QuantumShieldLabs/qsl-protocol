Status: Archive
Owner: qsl-protocol maintainers
Last-Updated: 2026-04-05

# NA-0221 First-Contact TOFU Policy / Scope Repair v2 Evidence

## Summary

- This governance-only lane repairs `NA-0221` queue truth on refreshed `main`.
- No qsc runtime, qsc-desktop, qsl-server, qsl-attachments, workflow, or cargo-manifest surfaces change in this PR.
- PR #657 remains a stale/open runtime lane and is not mutated here.

## Canonical / Audit Truth

- `DOC-CAN-003` says Suite-2 state must not be committed until authenticated peer identity exists, and establishment must reject when authenticated commitment cannot be provided.
- `DOC-AUD-002` records `NA-0220` `P1`: qsc accept paths can still write durable pending/session state for unknown or unpinned peers and must instead reject before `hs_pending_store(...)` or `qsp_session_store(...)`.

## Conflicting Protected Expectations

### `qsl/qsl-client/qsc/tests/identity_binding.rs`

- `tofu_pins_on_first_handshake()` currently seeds only route tokens for Alice/Bob and then expects full first-contact handshake success across init/poll/poll/poll.
- `handshake_accepts_verification_code_pin_without_peer_mismatch()` still leaves Alice route-only for Bob while Bob pins Alice, and then expects full handshake establishment success without peer mismatch.

### `qsl/qsl-client/qsc/tests/identity_foundation_contract_na0217d.rs`

- `verification_code_pin_preserves_handshake_contract()` currently route-sets Bob at Alice, verifies Alice at Bob, and then expects every handshake step to succeed without `handshake_reject`.

## Decision

- For `NA-0221`, first-contact TOFU establishment in qsc Suite-2 is retired on both initiator and responder paths.
- Fail-closed authenticated establishment takes precedence over preserving legacy TOFU convenience on either side of this Suite-2 handshake lane.
- The next implementation lane must update both protected test surfaces so they match canonical and audit truth.

## Scope Repair Only

- This evidence lane updates queue, decision, and traceability truth only.
- It introduces no runtime changes.
