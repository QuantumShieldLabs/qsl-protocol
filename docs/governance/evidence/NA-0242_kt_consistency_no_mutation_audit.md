Goals: G3, G4

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-05-02

# NA-0242 KT Consistency No-Mutation Audit

## Objective

Strengthen executable proof that rejected KT consistency advancement and related KT reject paths do not mutate the accepted KT state tracked by `CanonicalKtVerifier`.

## Findings

- `CanonicalKtVerifier` evaluates a bundle into a candidate `state_update`, then commits only after all KT checks and responder-binding checks succeed.
- The accepted KT state is an in-memory map keyed by pinned log id. The existing test-only `accepted_state` accessor returns a durable snapshot value for equality checks.
- NA-0242 did not need a runtime implementation fix. The existing commit boundary already preserved fail-closed/no-mutation behavior.
- The actor harness still uses explicit disabled/nonproduction KT mode for non-KT vector execution; NA-0242 did not widen that boundary.

## Executable Coverage Added

- `rejected_consistency_advancement_does_not_mutate_accepted_state`
  - primes an accepted STH at tree size `1`;
  - presents a tree-size `2` bundle with an invalid consistency proof;
  - asserts deterministic `consistency_proof_invalid` reject; and
  - asserts pre/post accepted-state equality.
- `wrong_log_reject_does_not_mutate_accepted_state`
  - primes accepted state for the pinned log;
  - presents a signed bundle with an unpinned/wrong log id; and
  - asserts deterministic `unpinned_log_id` reject plus pre/post accepted-state equality.
- `responder_binding_reject_after_valid_advanced_evidence_does_not_mutate_accepted_state`
  - primes accepted state at tree size `1`;
  - presents otherwise valid tree-size `2` KT evidence through responder binding;
  - makes the responder HS1 initiator-KT binding fail; and
  - asserts deterministic `hs1_pq_rcv_id_mismatch` reject plus pre/post accepted-state equality.

## Existing Invariants Preserved

- Bundle signature verification still runs before KT state evaluation.
- STH signature, inclusion proof, consistency proof, and pinned log-id checks remain fail-closed.
- Responder initiator-KT obligations remain enforced.
- Disabled/nonproduction KT mode remains explicit and bounded.
- qsl-server remains transport-only.
- qsl-attachments remains opaque ciphertext-only.

## Commands

Baseline:

```bash
cargo audit --deny warnings
cargo tree -i rustls-webpki --locked
cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1
cargo fmt --check
cargo build --locked
cargo clippy --locked -- -D warnings
cargo test --manifest-path tools/refimpl/quantumshield_refimpl/Cargo.toml --locked kt_verifier_vectors -- --nocapture
cargo test --manifest-path tools/refimpl/quantumshield_refimpl/Cargo.toml --locked responder_requires_bundle_equivalent_initiator_evidence -- --nocapture
cargo build --manifest-path tools/actors/refimpl_actor_rs/Cargo.toml --locked
python3 scripts/ci/run_suite2_establish_vectors.py --actor "${CARGO_TARGET_DIR:-target}/debug/refimpl_actor" --file inputs/suite2/vectors/qshield_suite2_establish_vectors_v1.json
```

Targeted NA-0242 tests:

```bash
cargo test --manifest-path tools/refimpl/quantumshield_refimpl/Cargo.toml --locked 'kt::canonical::tests::rejected_consistency_advancement_does_not_mutate_accepted_state' -- --nocapture
cargo test --manifest-path tools/refimpl/quantumshield_refimpl/Cargo.toml --locked 'kt::canonical::tests::wrong_log_reject_does_not_mutate_accepted_state' -- --nocapture
cargo test --manifest-path tools/refimpl/quantumshield_refimpl/Cargo.toml --locked 'kt::canonical::tests::responder_binding_reject_after_valid_advanced_evidence_does_not_mutate_accepted_state' -- --nocapture
```

## Remaining Gaps

- The KT vector JSON category remains verifier-behavior oriented and does not yet encode accepted-state snapshot assertions directly.
- Demo KT-negative acceptance remains a separate demo-surface question; this lane stays on the canonical refimpl KT verifier and responder-binding boundary.
- Skipped-key and receive/decrypt reject no-mutation coverage is intentionally deferred to a later successor lane.
