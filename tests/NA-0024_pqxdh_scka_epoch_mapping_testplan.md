Goals: G4, G5

# NA-0024 — PQXDH-style bundle mapping to SCKA initial epoch test plan

Status: DRAFT
Scope: Docs/vectors/CI wiring only. No protocol-core changes.

## Objective
- Define a self-contained mapping from PQXDH-style bundle outputs to SCKA epoch 0 state.
- Lock the mapping with deterministic vectors.

## CI-gated assertions
Enforced by: `suite2-vectors` job (runner `scripts/ci/run_suite2_scka_logic_vectors.py`)

- Mapping inputs (session_id, dh_init, pq_init_ss, role, dh pubs, authenticated=true) produce the expected RK and initial SCKA epoch state.
- Mapping rejects invalid inputs (lengths/role/authentication) per DOC-CAN-004 §3.5.3 (if/when negative vectors are added).

## Evidence
- suite2-vectors CI logs and `artifacts/suite2/scka_logic_vector_report.json`.
