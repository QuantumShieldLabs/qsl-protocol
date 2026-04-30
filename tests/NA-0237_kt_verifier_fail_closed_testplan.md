Status: Supporting
Owner: QSL governance
Last-Updated: 2026-04-30

# NA-0237 KT Verifier Fail-Closed Testplan

Goals: G4

## Objective

Prove PR `#708` implements the `DOC-CAN-008` key-transparency verifier on the bounded refimpl/actor path, rejects invalid KT evidence fail-closed, preserves explicit disabled/non-KT behavior, and merges only after restored required checks including `public-safety` pass normally.

## Protected Invariants

- `DOC-CAN-008` and supporting KT canon remain authoritative.
- KT-enabled verification rejects malformed, missing, stale, mismatched, unsigned, wrong-log, and inconsistent evidence.
- STH signatures, inclusion proofs, consistency obligations, bundle signatures, log-id pinning, and responder initiator-KT obligations are enforced.
- Rejected KT evidence does not mutate trusted KT/session state where such state exists.
- Non-KT or explicitly disabled-mode behavior remains bounded to frozen `main` semantics.
- qsl-server remains transport-only; qsl-attachments remains opaque ciphertext-only.
- PR `#708` uses no branch-protection/public-safety exception.

## Scope Guard

Allowed PR `#708` implementation/evidence paths are limited to:

- `tools/refimpl/quantumshield_refimpl/src/kt/**`
- `tools/refimpl/quantumshield_refimpl/src/qsp/handshake.rs`
- `tools/refimpl/quantumshield_refimpl/src/qsp/types.rs`
- `tools/refimpl/quantumshield_refimpl/tests/**`
- `tools/actors/refimpl_actor_rs/src/**`
- `inputs/suite2/vectors/**`
- `DECISIONS.md`, `TRACEABILITY.md`, this testplan, and the rolling journal

Forbidden-path proof must confirm no `.github/**`, `scripts/**`, `NEXT_ACTIONS.md`, Cargo manifests/lockfiles, qsc-desktop, qsl-server, qsl-attachments, website, public-safety/check configuration, branch protection, or unrelated qsc/app paths are changed.

## KT Positive Coverage

- `inputs/suite2/vectors/qshield_suite2_kt_verifier_vectors_v1.json` includes the positive `CAT-QSP-KT-001` case.
- `tools/refimpl/quantumshield_refimpl/tests/kt_verifier_vectors.rs` verifies the positive vector against canonical bundle, STH, inclusion, consistency, and signature material.
- `tools/actors/refimpl_actor_rs/src/main.rs` exercises the actor KT path for the same vector category.

## KT Negative Coverage

Required deterministic reject coverage:

- malformed evidence
- missing evidence
- stale STH timestamp
- mismatched leaf/bundle material
- unsigned or bad bundle signature
- wrong log id
- inconsistent tree transition / invalid consistency proof

The regression harness must assert fail-closed `reason_code` behavior rather than accepting or silently downgrading.

## Responder / Binding Coverage

- `tools/refimpl/quantumshield_refimpl/src/qsp/handshake.rs` must enforce responder-side initiator KT evidence when KT is enabled.
- The direct handshake regression `responder_requires_bundle_equivalent_initiator_evidence` must reject absent/mismatched initiator evidence and accept equivalent verified evidence.
- Transcript binding must continue to reject mismatched handshake material.

## No-Mutation / Disabled-Mode Proof

- Where pinned-log verifier state advances on accepted KT evidence, tests must show rejected evidence leaves the trusted state unchanged.
- If a path has no mutable KT state, code inspection must state that explicitly.
- Disabled/non-KT mode must remain explicit and bounded to harness/non-production configuration; KT-enabled profile must not fall back to disabled acceptance.

## Public-Safety / Protection Proof

- Before PR `#708` mutation, branch protection must show `public-safety` in the required-check set.
- Latest `main` after PR `#721` must have `public-safety` success.
- PR `#722` must remain closed and not merged.
- PR `#708` must pass restored `public-safety` normally; no required-check exception, admin bypass, direct push, or check spoofing is allowed.

## Validation Commands

- `git diff --check`
- `cargo audit --deny warnings`
- `cargo tree -i rustls-webpki --locked`
- `cargo +stable test -p qsc --locked --test send_commit -- --test-threads=1`
- `cargo fmt --check`
- `cargo build --locked`
- `cargo clippy --locked -- -D warnings`
- `cargo test --manifest-path tools/refimpl/quantumshield_refimpl/Cargo.toml --locked kt_verifier_vectors -- --nocapture`
- `cargo test --manifest-path tools/refimpl/quantumshield_refimpl/Cargo.toml --locked responder_requires_bundle_equivalent_initiator_evidence -- --nocapture`
- `cargo build --manifest-path tools/actors/refimpl_actor_rs/Cargo.toml --locked`
- `python3 scripts/ci/run_suite2_establish_vectors.py --actor target/debug/refimpl_actor --file inputs/suite2/vectors/qshield_suite2_establish_vectors_v1.json`
- repo-local goal-lint, queue parser, decision parser, markdown link-integrity, and leak-safe scans as established in `AGENTS.md`

## Required CI Contexts

PR `#708` must satisfy:

- `ci-4a`, `ci-4b`, `ci-4c`, `ci-4d`, `ci-4d-dur`
- `demo-cli-build`, `demo-cli-smoke`
- `formal-scka-model`
- `goal-lint`
- `metadata-conformance-smoke`
- `suite2-vectors`
- `CodeQL`
- `macos-qsc-qshield-build`
- `public-safety`

## Governance References

- `DECISIONS.md` (`D-0440`)
- `TRACEABILITY.md` (`NA-0237 implementation/evidence refresh`)
- `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`
