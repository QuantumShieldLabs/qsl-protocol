Goals: G3, G4

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-02

# NA-0242 KT Consistency No-Mutation Hardening Test Plan

## Objective

Add executable KT accepted-state no-mutation coverage for rejected consistency advancement and related reject paths while preserving already-canonical fail-closed KT verifier semantics.

## Protected Invariant

- Rejected KT consistency proof must not advance accepted KT state.
- Wrong-log, stale, malformed, missing, unsigned, or responder-binding reject paths must not mutate accepted KT state where state exists.
- Disabled/nonproduction KT mode remains explicit and bounded.
- Responder initiator-KT obligations remain enforced.
- No new KT semantics, wire behavior, qsl-server behavior, qsl-attachments behavior, qsc-desktop behavior, website behavior, public-safety behavior, branch-protection setting, or Cargo metadata change is introduced.

## Scope Guard

Allowed changed paths:

- `tools/refimpl/quantumshield_refimpl/src/kt/**`
- `tools/refimpl/quantumshield_refimpl/src/qsp/**` only if directly required
- `tools/refimpl/quantumshield_refimpl/tests/**`
- `tools/actors/refimpl_actor_rs/src/**` only if directly required
- `inputs/suite2/vectors/**` or `inputs/**` only if adding bounded KT/no-mutation vectors
- `docs/governance/evidence/NA-0242_kt_consistency_no_mutation_audit.md`
- `DECISIONS.md`
- `TRACEABILITY.md`
- `tests/NA-0242_kt_consistency_no_mutation_hardening_testplan.md`
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`

Forbidden paths include `.github/**`, `scripts/**`, Cargo files, qsc/qsl app code, qsc-desktop, qsl-server, qsl-attachments, website, branch-protection settings, public-safety configuration, and unrelated runtime/service code.

## Rejected Consistency Advancement No-Mutation Proof

Run:

```bash
cargo test --manifest-path tools/refimpl/quantumshield_refimpl/Cargo.toml --locked 'kt::canonical::tests::rejected_consistency_advancement_does_not_mutate_accepted_state' -- --nocapture
```

Required result:

- Accepted KT state is primed at tree size `1`.
- A tree-size `2` bundle with invalid consistency proof rejects deterministically.
- The accepted-state snapshot after reject equals the pre-reject snapshot.

## Related Reject No-Mutation Proof

Run:

```bash
cargo test --manifest-path tools/refimpl/quantumshield_refimpl/Cargo.toml --locked 'kt::canonical::tests::wrong_log_reject_does_not_mutate_accepted_state' -- --nocapture
cargo test --manifest-path tools/refimpl/quantumshield_refimpl/Cargo.toml --locked 'kt::canonical::tests::responder_binding_reject_after_valid_advanced_evidence_does_not_mutate_accepted_state' -- --nocapture
```

Required result:

- Wrong-log reject leaves the accepted pinned-log state unchanged.
- Responder initiator-KT binding reject after otherwise valid advanced KT evidence leaves the accepted pinned-log state unchanged.

## Durable / State Snapshot Proof

The canonical verifier exposes a test-only accepted-state snapshot as `AcceptedSth`, which is `Copy`, `Eq`, and includes tree size, root hash, and timestamp. NA-0242 tests compare full pre/post `AcceptedSth` equality rather than only checking tree-size.

## Existing KT Verifier No-Regression Proof

Run:

```bash
cargo test --manifest-path tools/refimpl/quantumshield_refimpl/Cargo.toml --locked kt_verifier_vectors -- --nocapture
cargo test --manifest-path tools/refimpl/quantumshield_refimpl/Cargo.toml --locked responder_requires_bundle_equivalent_initiator_evidence -- --nocapture
```

Required result:

- Existing PR `#708` KT vector and responder-binding tests remain green.

## Disabled / Nonproduction Mode Boundary Proof

Run:

```bash
cargo test --manifest-path tools/refimpl/quantumshield_refimpl/Cargo.toml --locked disabled_shape_requires_explicit_nonproduction_mode -- --nocapture
```

Required result:

- Disabled KT shape is rejected when explicit nonproduction mode is not enabled.
- Disabled KT shape is accepted only through explicit nonproduction configuration.

## Local Validation Commands

```bash
git status --porcelain=v1 --branch
git diff --name-only origin/main...HEAD
git diff --stat origin/main...HEAD
git diff --check
cargo fmt --check
cargo audit --deny warnings
cargo build --locked
cargo clippy --locked -- -D warnings
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
cargo test --manifest-path tools/refimpl/quantumshield_refimpl/Cargo.toml --locked 'kt::canonical::tests::rejected_consistency_advancement_does_not_mutate_accepted_state' -- --nocapture
cargo test --manifest-path tools/refimpl/quantumshield_refimpl/Cargo.toml --locked 'kt::canonical::tests::wrong_log_reject_does_not_mutate_accepted_state' -- --nocapture
cargo test --manifest-path tools/refimpl/quantumshield_refimpl/Cargo.toml --locked 'kt::canonical::tests::responder_binding_reject_after_valid_advanced_evidence_does_not_mutate_accepted_state' -- --nocapture
cargo test --manifest-path tools/refimpl/quantumshield_refimpl/Cargo.toml --locked kt_verifier_vectors -- --nocapture
cargo test --manifest-path tools/refimpl/quantumshield_refimpl/Cargo.toml --locked responder_requires_bundle_equivalent_initiator_evidence -- --nocapture
cargo test --manifest-path tools/refimpl/quantumshield_refimpl/Cargo.toml --locked disabled_shape_requires_explicit_nonproduction_mode -- --nocapture
```

Also run goal-lint, queue parser, decision parser, markdown inventory/link validation, leak-safe scan, and scope guard using the established repository patterns.

## CI Expectations

Required contexts for the PR:

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

All listed contexts must succeed except CodeQL may be accepted as neutral only if GitHub branch protection accepts it. `public-safety` must be success. No branch-protection exception, admin bypass, direct push, check spoofing, squash merge, or rebase merge is allowed.
