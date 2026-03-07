# NA-0118 Message State Model Plan

## Scope and assumptions
- Scope limited to `qsl/qsl-client/qsc/**`.
- Message-state semantics are client-local and deterministic.

## Threat model notes
- False delivery claims that overstate message assurances.
- Non-deterministic state transitions under rejects/replays.

## Must-never list
- Must never claim `delivered_to_peer` without `receipt_recv`.
- Must never silently advance state on reject/tamper paths.
- Must never emit non-deterministic transition markers.

## Proposed design
- Canonical states in qsc timeline: `CREATED -> SENT|RECEIVED -> DELIVERED` with `FAILED` terminal.
- Deterministic transition/reject markers:
  - `QSC_MARK/1 event=message_state_transition from=<X> to=<Y> id=<id> ok=true`
  - `QSC_MARK/1 event=message_state_reject code=<reason> reason=<reason> id=<id>`
- Persist state in vault-backed timeline entries (`timeline.json` secret key) and only mutate on allowed transitions.
- Integrate explicit ACK consumption with `SENT -> DELIVERED` transition validation bound to peer timeline + message id.

## Test vectors
- Honest delivery:
  - `honest_delivery_requires_explicit_ack` proves no `DELIVERED` without explicit ACK.
  - `wrong_peer_ack_rejected_no_mutation` proves wrong peer/session ACK is rejected and state remains unchanged.
- Reject/no-mutation:
  - `replay_ack_does_not_advance_state` proves duplicate ACK emits deterministic reject and does not advance state.
- Determinism + secret hygiene:
  - `state_markers_are_deterministic_and_secret_safe` compares normalized transition markers across two runs.
- Transition ordering/terminal behavior:
  - unit tests in `src/main.rs` assert failed-terminal and no-skip constraints.

## Verification checklist
- `cargo fmt -p qsc -- --check`
- `cargo test -p qsc --locked`
- `cargo clippy -p qsc --all-targets -- -D warnings`

Executed on 2026-02-10:
- `cargo fmt -p qsc -- --check` PASS
- `cargo test -p qsc --locked` PASS
- `cargo clippy -p qsc --all-targets -- -D warnings` PASS

Added/updated tests:
- `qsl/qsl-client/qsc/tests/message_state_model.rs`
- `qsl/qsl-client/qsc/tests/receipts_delivered.rs` (vault init in fixtures for stateful receipt path)
- `qsl/qsl-client/qsc/src/main.rs` (`#[cfg(test)]` state transition guard tests)

## Rollback
- Revert state-model changes if truthful delivery semantics regress.
