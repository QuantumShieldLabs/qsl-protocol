Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-03

# NA-0243 Skipped-Key and Receive-Decryption Reject No-Mutation Audit

## Objective

Record executable evidence that failed Suite-2 skipped-key decrypt and receive/decrypt reject paths do not mutate durable session state.

## Findings

- Skipped-key body-auth failure is covered by `skipped_key_body_auth_reject_does_not_consume_skipped_key_or_mutate_snapshot`.
- Stateful receive body-auth failure is covered by `receive_body_auth_reject_does_not_advance_or_mutate_session_snapshot`.
- Both tests use `Suite2SessionState::snapshot_bytes` before and after the reject attempt.
- Both tests repeat the same invalid input and require deterministic `REJECT_S2_BODY_AUTH_FAIL`.
- The skipped-key test verifies the skipped key remains present and unconsumed after reject.
- The receive/decrypt test verifies `nr` does not advance and no skipped-message state is added after reject.

## State Snapshot Proof

The tests use a mutable `Suite2SessionState` harness that commits `recv_wire_canon` output only after success. On reject, the durable session snapshot must remain byte-equal to the pre-attempt snapshot. This directly covers the current refimpl session durability surface without changing protocol or runtime semantics.

## Invariants Covered

- Failed skipped-key decrypt does not consume skipped keys.
- Failed receive/decrypt does not advance ratchet/session state.
- Reject behavior is deterministic for repeated invalid input.
- Durable state snapshot integrity is preserved.
- qsl-server and qsl-attachments boundaries are untouched.

## Commands

Primary local command passed:

```bash
cargo test -p quantumshield_refimpl --locked --test na_0243_skipped_key_decrypt_no_mutation -- --test-threads=1
```

Additional local validation passed:

```bash
cargo fmt --check
cargo build --locked
cargo clippy --locked -- -D warnings
cargo audit --deny warnings
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
cargo test -p quantumshield_refimpl --locked --test suite2_bounded_receive -- --test-threads=1
cargo test -p quantumshield_refimpl --locked mkskipped -- --test-threads=1
python3 scripts/ci/validate_suite2_vectors.py
python3 scripts/ci/run_suite2_ooo_replay_vectors.py --actor "${CARGO_TARGET_DIR:-target}/debug/refimpl_actor" --actor-name suite2-ooo-na0243
python3 scripts/ci/run_suite2_e2e_recv_vectors.py --actor "${CARGO_TARGET_DIR:-target}/debug/refimpl_actor" --actor-name suite2-e2e-na0243
```

## Remaining Gaps

- This lane does not broaden KT or SCKA coverage beyond the NA-0240 through NA-0242 evidence.
- This lane does not change qsl-server, qsl-attachments, qsc-desktop, website, public-safety, branch protection, Cargo metadata, or protocol semantics.

## Recommendations

- Keep future receive-path hardening tied to durable snapshot equality before and after reject.
- Treat any future reject path that returns mutated state on failure as a release-blocking regression.
