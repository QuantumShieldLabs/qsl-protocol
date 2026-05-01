Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-30

# NA-0240 SCKA Persistence and Monotonicity Vectors Testplan

Goals: G2, G3, G4

## Objective

Prove `NA-0240` strengthens executable SCKA persistence, monotonicity, rollback, tombstone/one-time consumption, and no-state-mutation reject coverage without inventing new SCKA semantics or changing service/runtime boundaries.

## Protected Invariant

- SCKA peer epoch observations are monotonic.
- Durable SCKA state persists across restart.
- Rollback to older peer epoch, local target, or tombstone state is rejected.
- Consumed or tombstoned SCKA material is one-time use only.
- Rejected SCKA input does not mutate in-memory or durable state.
- Suite-2 transcript and downgrade fail-closed invariants remain unchanged.
- qsl-server remains transport-only.
- qsl-attachments remains opaque ciphertext-only.
- qsc-desktop remains untouched.

## Scope Guard

Allowed changed paths:

- `tools/refimpl/quantumshield_refimpl/src/**` only for bounded SCKA invariant test coverage or enforcement
- `tools/refimpl/quantumshield_refimpl/tests/**`
- `inputs/suite2/vectors/**`
- `formal/**`
- `docs/governance/evidence/NA-0240_scka_persistence_monotonicity_audit.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `tests/NA-0240_scka_persistence_monotonicity_vectors_testplan.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden path proof must confirm no `.github/**`, `scripts/**`, `Cargo.toml`, `Cargo.lock`, `qsc/**`, `qsl/**`, `qsl-client/**`, `apps/**`, `qsc-desktop/**`, `qsl-server/**`, `qsl-attachments/**`, `website/**`, branch-protection settings, public-safety/check configuration, or unrelated runtime/protocol/crypto/demo/service paths changed.

`NEXT_ACTIONS.md` must remain unchanged in the `NA-0240` implementation PR; `NA-0240` remains READY pending later closeout.

## Persistence/Restart Proof

Required proof:

- `snapshot_restore_persists_scka_state_and_rejects_regressions_without_mutation` persists `peer_max_adv_id_seen`, known targets, consumed targets, and tombstoned targets through snapshot/restore.
- The restored snapshot is byte-identical before and after rejected repeated/lower epoch input.
- The restored snapshot is byte-identical before and after rejected tombstoned consumed material.

## Rollback/Monotonicity Reject Proof

Required proof:

- Existing nonmonotonic replay reject remains deterministic.
- New lower-epoch Suite-2 SCKA logic vector rejects with `REJECT_SCKA_ADV_NONMONOTONIC`.
- Formal durable-record assertions detect rollback from an accepted higher ADV state to an older peer-max state.

## Tombstone/One-Time Consumption Proof

Required proof:

- Tombstoned consumed target reject returns `REJECT_SCKA_TARGET_TOMBSTONED`.
- Input target registries remain unchanged after tombstone/consumed rejection.
- Formal durable-record assertions detect rollback from an accepted CTXT transition to a pre-tombstone state.

## No-State-Mutation-On-Reject Proof

Required proof:

- Refimpl SCKA tests assert pre/post equality for borrowed target registries.
- Refimpl snapshot tests assert pre/post durable snapshot equality after reject.
- SCKA logic vectors use `state_unchanged: true` for new negative cases.
- Actor restore paths build replacement session maps and run rollback checks before replacing durable session state.

## Formal-SCKA-Model Proof

Required proof:

- `python3 formal/run_model_checks.py` passes.
- The model includes restart snapshot/restore equality.
- The model includes rollback detection for accepted ADV and CTXT durable records.

## Suite2/Vector Proof

Required proof:

- `python3 scripts/ci/validate_suite2_vectors.py` passes.
- `python3 scripts/ci/run_suite2_scka_logic_vectors.py --actor /srv/qbuild/cache/targets/qsl-protocol/release/refimpl_actor --file inputs/suite2/vectors/qshield_suite2_scka_logic_vectors_v1.json` covers the new SCKA logic cases.
- `python3 scripts/ci/run_suite2_crash_restart_vectors.py --actor /srv/qbuild/cache/targets/qsl-protocol/release/refimpl_actor --file inputs/suite2/vectors/qshield_suite2_crash_restart_vectors_v1.json` remains green.

## Local Validation Commands

- `git status --porcelain=v1 --branch`
- `git diff --name-only origin/main...HEAD`
- `git diff --stat origin/main...HEAD`
- `git diff --check`
- `cargo fmt --check`
- `cargo audit --deny warnings`
- `cargo build --locked`
- `cargo clippy --locked -- -D warnings`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `cargo test -p quantumshield_refimpl --locked scka_`
- `cargo test -p quantumshield_refimpl --locked snapshot_restore_persists_scka_state`
- `python3 formal/run_model_checks.py`
- `cargo build -p refimpl_actor --release --locked`
- `python3 scripts/ci/validate_suite2_vectors.py`
- exact Suite-2 vector runner commands for SCKA logic, crash/restart, PQ reseed, boundary, SCKA KEM, and downgrade
- deterministic queue parser
- deterministic decision parser
- repo-local goal-lint via synthetic PR event
- markdown inventory commands from `AGENTS.md`
- manual markdown link-integrity runbook from `AGENTS.md`
- leak-safe added-line scan

## Required CI Context Expectations

The implementation PR must satisfy the protected context set:

- `ci-4a`
- `ci-4b`
- `ci-4c`
- `ci-4d`
- `ci-4d-dur`
- `demo-cli-build`
- `demo-cli-smoke`
- `formal-scka-model`
- `goal-lint`
- `metadata-conformance-smoke`
- `suite2-vectors`
- `CodeQL`
- `macos-qsc-qshield-build`
- `public-safety`

`CodeQL` may be accepted as neutral only if GitHub branch protection accepts it. `public-safety` must be success. No branch-protection exception, admin bypass, direct push, check spoofing, squash merge, or rebase merge is allowed.

## References

- `DECISIONS.md` (D-0445)
- `TRACEABILITY.md`
- `docs/governance/evidence/NA-0240_scka_persistence_monotonicity_audit.md`
- `formal/model_scka_bounded.py`
- `inputs/suite2/vectors/qshield_suite2_scka_logic_vectors_v1.json`
- `tools/refimpl/quantumshield_refimpl/src/suite2/scka.rs`
- `tools/refimpl/quantumshield_refimpl/src/suite2/state.rs`
