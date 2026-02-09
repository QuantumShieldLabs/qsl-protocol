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
- Define explicit per-message transition graph.
- Emit deterministic transition markers for each state step.
- Enforce fail-closed behavior for invalid transitions.

## Test vectors
- no false delivered-to-peer states.
- deterministic transition replay.
- reject/no-mutation behavior under tamper/replay.
- no secrets in output.

## Verification checklist
- `cargo fmt -p qsc -- --check`
- `cargo test -p qsc --locked`
- `cargo clippy -p qsc --all-targets -- -D warnings`

## Rollback
- Revert state-model changes if truthful delivery semantics regress.
