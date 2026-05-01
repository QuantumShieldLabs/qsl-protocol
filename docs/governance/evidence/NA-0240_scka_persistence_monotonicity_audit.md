Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-30

# NA-0240 SCKA Persistence and Monotonicity Audit

Goals: G2, G3, G4

## Objective

Record the executable SCKA persistence, monotonicity, rollback, tombstone, one-time consumption, and no-state-mutation coverage added for `NA-0240`.

## Findings

- Canonical SCKA requirements are in `docs/canonical/DOC-CAN-004_QSP_SCKA_Sparse_Continuous_Key_Agreement_v1.0.0_DRAFT.md`.
- Existing executable SCKA surfaces include `formal/model_scka_bounded.py`, Suite-2 SCKA vectors, crash/restart vectors, and refimpl `suite2::scka` / `suite2::state` tests.
- The refimpl already rejected nonmonotonic peer epochs before mutation, checked tombstones before consumed targets, and kept staged reseed output non-committing.
- No implementation semantic fix was required. The change strengthens proof coverage around already-canonical behavior.

## Executable Coverage Added

- `tools/refimpl/quantumshield_refimpl/src/suite2/state.rs`
  - Snapshot/restore now proves persisted SCKA `peer_max_adv_id_seen`, known targets, consumed targets, and tombstones survive restart.
  - After restore, repeated/lower epoch input rejects with `REJECT_SCKA_ADV_NONMONOTONIC` and the durable snapshot remains byte-identical.
  - After restore, tombstoned consumed material rejects with `REJECT_SCKA_TARGET_TOMBSTONED` and the durable snapshot remains byte-identical.
- `tools/refimpl/quantumshield_refimpl/src/suite2/scka.rs`
  - Tombstone-over-consumed reject precedence is deterministic and leaves input target registries unchanged.
  - Non-commit staging derives seeds without advancing peer epoch, chain keys, consumed targets, or tombstones.
- `inputs/suite2/vectors/qshield_suite2_scka_logic_vectors_v1.json`
  - Adds a lower-epoch monotonicity reject vector.
  - Adds a tombstone-plus-consumed reject vector proving tombstone precedence and no-state-mutation expectations.
- `formal/model_scka_bounded.py`
  - Adds restart snapshot/restore equality as a party invariant.
  - Adds durable-record rollback detection assertions for accepted ADV and CTXT transitions.

## No-Mutation Proof

Reject paths covered by `NA-0240` compare input registries or durable snapshots before and after rejection. The tested paths either borrow immutable state until acceptance (`apply_pq_reseed`) or return errors before replacing restored actor/session state. The strengthened tests assert equality after reject rather than relying on documentation only.

## Commands

Baseline commands executed before edits:

- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `cargo fmt --check`
- `cargo build --locked`
- `cargo clippy --locked -- -D warnings`
- `python3 formal/run_model_checks.py`
- `python3 scripts/ci/validate_suite2_vectors.py`
- Suite-2 vector runner scripts for SCKA logic, crash/restart, PQ reseed, boundary, SCKA KEM, downgrade, and the full workflow-equivalent vector set.

Expected final validation:

- `cargo fmt --check`
- `cargo audit --deny warnings`
- `cargo build --locked`
- `cargo clippy --locked -- -D warnings`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `cargo test -p quantumshield_refimpl --locked scka_`
- `cargo test -p quantumshield_refimpl --locked snapshot_restore_persists_scka_state`
- `python3 formal/run_model_checks.py`
- `python3 scripts/ci/validate_suite2_vectors.py`
- `python3 scripts/ci/run_suite2_scka_logic_vectors.py --actor /srv/qbuild/cache/targets/qsl-protocol/release/refimpl_actor --file inputs/suite2/vectors/qshield_suite2_scka_logic_vectors_v1.json`
- `python3 scripts/ci/run_suite2_crash_restart_vectors.py --actor /srv/qbuild/cache/targets/qsl-protocol/release/refimpl_actor --file inputs/suite2/vectors/qshield_suite2_crash_restart_vectors_v1.json`

## Gaps And Recommendations

- The qsl-server transport-only and qsl-attachments opaque ciphertext-only boundaries remain protected by scope guard rather than new service tests in this lane.
- A later successor should add downgrade/transcript negative vectors that combine capability commitment failures with no-state-mutation assertions.
- A later successor should audit user-facing demo/helper unwraps and convert any non-test panic path into explicit fail-closed errors.
