# Audit Issue #6 — ck_pq_recv Boundary Handling Spec Review (DRAFT)

Goals: G4, G5
Date: 2026-01-10
Status: DRAFT

## Finding summary (Issue #6)
Audit Issue #6 flags that `ck_pq_recv` may not be updated correctly at a boundary in the current Suite-2 receive path. This risks
inconsistent PQ chain advancement across parties, breaking the intended Suite-2 invariant that boundary processing applies the PQ reseed
and sets `CK_pq_recv` directionally according to the new epoch.

## Current implementation evidence
- File: `tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs`
- Anchor: `recv_boundary_in_order` uses `derive_mk_step` and discards `_ck_pq_p` (line ~365) before SCKA apply.
- Anchor: `new_state.ck_pq_recv = apply.ck_pq_recv_after;` (line ~406) applies SCKA reseed result.

Snippet anchors (current):
- `suite2/ratchet.rs:365` — `let (ck_ec_p, _ck_pq_p, mk) = ...`
- `suite2/ratchet.rs:406` — `new_state.ck_pq_recv = apply.ck_pq_recv_after;`

## Canonical spec evidence (in-repo)
Suite-2 boundary + PQ reseed rules are specified in DOC-CAN-003:
- `docs/canonical/DOC-CAN-003_QSP_Suite-2_True_Triple_Ratchet_v5.0.0_DRAFT.md`
  - §3.3.6 PQ reseed KDF and application (lines ~190–209, 553–561)
  - §8.5.3 Boundary with PQ CTXT (lines ~551–561) — requires directional PQ chain update

SCKA transactional rules (commit only on success) are specified in DOC-CAN-004:
- `docs/canonical/DOC-CAN-004_QSP_SCKA_Sparse_Continuous_Key_Agreement_v1.0.0_DRAFT.md`
  - §3.2.5/§3.2.6 (lines ~156–169) — decap + tombstone updates are staged and commit only on Suite-2 commit
  - §3.2.6 transactional rule (line ~169) — no SCKA state commit on reject

## Interpretation
The spec requires boundary handling to apply directional PQ reseed output (`CK_pq_send` / `CK_pq_recv`) on successful boundary
processing. The receive path should not silently discard PQ chain advancement if it is required to derive the boundary message key or
apply a reseed. If the current implementation drops `_ck_pq_p` without a defined rationale, the spec-consistent behavior is ambiguous
and must be resolved.

## Proposed invariant (must never happen)
- On a boundary message with PQ reseed (`FLAG_PQ_CTXT`), a successful message decrypt MUST commit the correct directional
  `CK_pq_recv` update for the new epoch; it must not discard an intermediate PQ chain step that is required for correctness.
- On any reject path (header/body/SCKA failure), `ck_pq_recv` MUST remain unchanged.

## Proposed mitigation (high-level, fail-closed)
- Clarify whether `derive_mk_step` should advance `CK_pq_recv` for the boundary message key or whether the SCKA apply step fully
  defines `CK_pq_recv` post-boundary. If both are needed, sequence explicitly and commit only after successful body + SCKA checks.
- If `_ck_pq_p` is required by the spec for boundary mk derivation, apply it deterministically and include it in the commit state.
- If `_ck_pq_p` must be ignored, document the rationale and add an explicit check to ensure the boundary mk derivation remains correct.

## Test plan for the implementation PR
Deterministic reject:
- Same invalid boundary input twice from identical starting state returns the same reject code.

No mutation on reject:
- Snapshot `Suite2RecvState` before boundary processing; after a reject, confirm `ck_pq_recv` and related state are unchanged.

Vector additions:
- Add a boundary vector where the PQ reseed is required and verify `ck_pq_recv` changes only on success.

## Go/No-Go criteria for implementation PR
- Spec interpretation resolved and cited with exact anchors.
- Tests prove deterministic reject + no mutation on reject.
- CI green with new vector/test coverage.
