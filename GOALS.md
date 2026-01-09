# QuantumShield Program Goals (Canonical)

This file is the single source of truth for program goals and merge criteria.

## Goal IDs

**G1 — Always-hybrid per-message keys (Suite-2):**  
For every application message, derive the AEAD key as:
- `ec_mk` from the classical Double Ratchet chain (per-direction)
- `pq_mk` from a per-direction PQ chain that advances **every message** and is **reseeded sparsely** when PQ SCKA epochs advance  
Then:
- `mk = KDF_HYBRID(ec_mk, pq_mk)`  
No application message may be encrypted using only the classical chain when Suite-2 is negotiated.

**G2 — Explicit PQ engine (SCKA) with epoch monotonicity + persistence safety:**  
PQ ratcheting MUST be specified as an explicit state machine (SCKA) with:
- monotonic epoch identifiers / ordering rules
- deterministic accept/reject semantics (fail-closed)
- crash-safe state persistence requirements and rollback detection

**G3 — Fail-closed downgrade resistance:**  
If both peers support Suite-2, Suite-2 MUST be used.  
Capability negotiation MUST be transcript-bound (commitment) and MUST NOT silently fall back.

**G4 — Verification as a release gate:**  
Protocol changes MUST be accompanied by:
- conformance vectors/tests for new or modified behavior
- a maintained formal verification plan (and, when models exist, model checks in CI)

**G5 — Metadata minimization lane (QSE/QSP integration):**  
Protocol and envelope work MUST consider linkability and fingerprinting:
- uniform error behavior where applicable
- padding/traffic profiles where applicable
- minimization of stable identifiers and observable state where feasible

## Global non-regression rules (apply to every PR)

1. **Goal impact statement required:** every PR must declare which Goal IDs it advances (G1–G5).
2. **Fail-closed bias:** if correctness or safety is uncertain, reject/abort rather than accept/continue.
3. **No silent downgrades:** any reduction in security must be explicit, negotiated, and documented in DECISIONS.md.
4. **Traceability required:** changes to protocol behavior require updates to TRACEABILITY.md and/or DECISIONS.md (see checklist).
5. **Test/vector discipline:** protocol behavior changes require tests/vectors in the same PR.

## Release readiness (Suite-2)

Suite-2 is considered release-ready only when all are true:
- G1 is implemented and enforced in both spec and reference implementation
- G2 SCKA state machine is fully specified and tested
- G3 negotiation is transcript-bound and fail-closed with negative tests
- G4 includes at least one machine-checkable model with CI execution
- G5 profiles are specified with explicit privacy tradeoffs
