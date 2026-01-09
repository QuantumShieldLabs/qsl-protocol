# Formal model checks (CI-gated)

Goals: G4

## 0. Purpose

This directory contains **machine-executable model checks** that serve as an initial, fail-closed verification gate.

The near-term objective is to:
- establish an executable model surface in-repo,
- make the modeled security properties explicit,
- and run these checks in CI so regressions are blocked automatically.

## 1. Tool decision (initial)

We start with a **bounded state-space explorer implemented in Python**.

Rationale:
- zero external dependencies (runs in local dev and GitHub Actions without system packages)
- deterministic execution and stable outputs
- fail-closed by construction (any property violation aborts the job)

This is an intentionally conservative first step that does **not** preclude adopting ProVerif and/or Tamarin later (see FORMAL_VERIFICATION_PLAN.md). It exists to ensure G4 has an executable gate immediately.

## 2. Modeled protocol slice (current)

The initial model checks the **SCKA control-plane logic invariants** only:
- peer ADV monotonicity acceptance/reject rules
- one-time ciphertext targeting + tombstoning
- “reject implies no state change”
- transactional commit semantics (abstracted; no partial commits)

Authoritative sources for meaning:
- DOC-CAN-004 §§3–6 (SCKA normative logic)
- DOC-TST-005 CAT-SCKA-LOGIC-001, CAT-SCKA-KEM-001 (coverage intent)

## 3. Roles and channels (model)

Roles:
- Party A
- Party B

Adversary model (network control):
- can reorder, drop, delay, and replay previously observed messages
- cannot forge an internally “accepted” message without satisfying SCKA’s explicit accept/reject rules

Cryptography model:
- not modeled; ciphertext is treated as an opaque value
- the model is purely about **state-machine invariants and fail-closed logic**

## 4. Properties checked (fail-closed)

The model checks the following properties for all explored executions within the configured bounds:

- **P1 (ADV monotonicity):** a peer ADV is accepted iff its `pq_adv_id` is strictly greater than the local `peer_max_adv_id_seen`.
- **P2 (one-time consumption):** a local advertised key id can be consumed at most once.
- **P3 (tombstones):** a tombstoned target id is always rejected.
- **P4 (no state change on reject):** rejecting a message does not mutate persistent state.
- **P5 (transactional commit):** key consumption and tombstoning occur only on commit (modeled as “accept”).

## 5. Scope limits

- Bounded exploration is not a proof of correctness outside the explored bounds.
- This model does not (yet) cover secrecy, authentication, transcript binding, or key schedule security.
- The model is intentionally narrow to establish the CI lane first.

## 6. Running locally

From the repo root:

- `python3 formal/run_model_checks.py`

The command exits non-zero on any violation.

## 7. CI integration

The model is executed in CI by:
- `.github/workflows/formal.yml`

The job is intended to be a release gate (G4): if the model fails, CI fails.
