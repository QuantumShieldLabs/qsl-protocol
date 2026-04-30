Goals: G1, G2, G3, G4, G5

Status: Authoritative
Owner: QSL governance
Last-Updated: 2026-04-30

# Conformance Vector Prioritization

## Priority classes

Conformance work should prioritize vectors and tests that prove fail-closed behavior, cross-implementation consistency, and durable state invariants.

### KT verifier vectors

Expand `CAT-QSP-KT-001` with malformed, missing, stale, mismatched, unsigned, wrong-log, inclusion-root mismatch, consistency-proof, same-tree mismatch, and responder initiator-evidence cases.

### SCKA monotonicity and persistence vectors

Prioritize peer advertisement monotonicity, one-time ciphertext targeting, tombstone retention, restart persistence, rollback detection, and no durable mutation on rejected SCKA input.

### Downgrade-resistance vectors

Add vectors where both peers support Suite-2 and any weaker negotiation, transcript mismatch, unknown suite, or policy mismatch rejects deterministically.

### No-state-mutation-on-reject vectors

Target stateful paths first: KT accepted-state update, SCKA epoch state, skipped-key use, send/commit failures, receive decrypt failures, attachment confirmation, relay replay, and demo establish cache.

### Metadata conformance vectors

Expand checks for loopback binding, explicit unsafe acknowledgement, authorization-required paths, store permissions, padding bucket behavior, bounded queue sizes, stable output markers, and leak-safe evidence.

### Demo acceptance vectors

Treat demo acceptance vectors as product-facing conformance: valid establish/send/receive, malformed reject, downgrade reject, replay reject, invalid KT reject, and attachment reject where applicable.

### Release-readiness vectors

Before release readiness, every G1-G5 claim should have at least one machine-checkable test/vector path and one CI or repeatable local command that proves it.

## First 10 recommended next vectors/tests

1. KT malformed STH vector with valid bundle signature and deterministic `kt_fail`.
2. KT responder missing initiator-bundle evidence vector.
3. KT responder mismatched initiator bundle vector.
4. KT no-state-mutation vector for rejected consistency-proof advancement.
5. SCKA restart persistence vector for accepted monotonic epoch.
6. SCKA rollback-detection vector for regressed peer advertisement state.
7. Downgrade vector where both peers support Suite-2 but negotiated suite is weaker.
8. No-state-mutation-on-reject vector for failed skipped-key decrypt.
9. Demo invalid establish replay/malformed input acceptance test.
10. Metadata conformance test for leak-safe evidence and loopback-only default relay binding.
