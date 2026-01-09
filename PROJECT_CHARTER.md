# QuantumShield Project Charter (Canonical)

## Purpose
QuantumShield aims to deliver a communication protocol that achieves:
- **Strong post-quantum and classical confidentiality for message contents**, including post-compromise recovery.
- **Operationally robust security**, including downgrade/rollback resistance and crash-safe state management.
- **Meaningful metadata minimization**, acknowledging that cryptography alone does not eliminate network metadata.

## Non-negotiable program goals
This project is governed by GOALS.md (G1–G5). All work must advance at least one goal without regressing the others.

## Design philosophy
- **Hybrid-by-default:** combine classical and PQ assumptions rather than choosing one.
- **Fail-closed semantics:** ambiguous inputs and unexpected states must result in safe abort, not permissive behavior.
- **Specification primacy:** normative requirements live in the canonical specs; implementation guidance is subordinate.
- **Traceability:** every security-critical behavior must map to a spec section and to tests/vectors.

## Threat model (high-level)
- Active network attacker (replay, reorder, drop, inject, tamper).
- Endpoint compromise at arbitrary times; adversary may obtain device state.
- Long-term quantum-capable attacker (store-now-decrypt-later, plus PQ cryptanalysis).
- Operational failures (crashes, partial writes, restored backups / rollbacks).

## What “better than SPQR” means here
We will compete on explicit properties:
- **Per-message hybridization** without per-message PQ bandwidth overhead (G1).
- **Deterministic SCKA state machine** with stronger operational invariants (G2).
- **Hard downgrade resistance** with transcript-bound capability commitment (G3).
- **Verification and conformance gates** that prevent drift (G4).
- **Metadata minimization profiles** integrated with envelope semantics (G5).

## Success criteria
A change is “successful” only if:
- it is linked to Goal IDs (G1–G5),
- it is reflected in traceability,
- and it is backed by tests/vectors (and model updates where relevant).

## Operating rules
- Any change that affects protocol state machines or key schedules requires a DECISIONS.md entry.
- No feature ships without negative tests for downgrade and rollback.
- Documentation updates must precede or accompany code changes; never “later.”
