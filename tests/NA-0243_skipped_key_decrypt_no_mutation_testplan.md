Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-03

# NA-0243 Skipped-Key and Receive-Decryption Reject No-Mutation Test Plan

## Objective

Add executable proof that failed skipped-key decrypt and receive/decrypt reject paths preserve durable Suite-2 session state.

## Scope Guard

Allowed implementation paths:

- `tools/refimpl/quantumshield_refimpl/tests/**`
- `docs/governance/evidence/NA-0243_skipped_key_decrypt_no_mutation_audit.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `tests/NA-0243_skipped_key_decrypt_no_mutation_testplan.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

No `.github`, `scripts`, Cargo files, qsc/qsl app code, qsl-server, qsl-attachments, qsc-desktop, website, public-safety, branch-protection, or unrelated runtime/service path changes are allowed.

## Protected Invariants

- Failed skipped-key decrypt rejects deterministically.
- Failed skipped-key decrypt does not consume skipped-key state.
- Failed receive/decrypt rejects deterministically.
- Failed receive/decrypt does not advance `nr`, ratchet chains, or durable session state.
- Tests do not invent new protocol semantics.

## Executable Tests

Primary tests:

- `skipped_key_body_auth_reject_does_not_consume_skipped_key_or_mutate_snapshot`
- `receive_body_auth_reject_does_not_advance_or_mutate_session_snapshot`

Expected command:

```bash
cargo test -p quantumshield_refimpl --locked --test na_0243_skipped_key_decrypt_no_mutation -- --test-threads=1
```

## Existing No-Regression Coverage

Run existing relevant coverage:

- `cargo test -p quantumshield_refimpl --locked --test suite2_bounded_receive -- --test-threads=1`
- existing Suite-2 vector command if available in the validation bundle
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`

## Decision Parser Expectations

After the patch:

- D-0110 exists once.
- D-0451 exists once.
- D-0452 exists once.
- D-0453 is absent.
- No duplicate decision IDs exist.

## Queue Expectations

Packet A must not edit `NEXT_ACTIONS.md`; the queue parser must continue to report:

```text
READY_COUNT 1
READY NA-0243 Skipped-Key and Receive-Decryption Reject No-Mutation Hardening
```

## CI Expectations

- Required GitHub contexts pass normally.
- `public-safety` remains required and green.
- No qsl-server, qsl-attachments, qsc-desktop, website, Cargo, public-safety, or branch-protection drift.
