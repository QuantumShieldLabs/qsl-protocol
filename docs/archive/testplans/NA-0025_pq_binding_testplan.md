Goals: G3
Status: DRAFT

Scope:
- Suite-2 establishment binding only.
- Protocol-core changes allowed; no new wire formats.

Objective:
- Bind PQ KEM public key identifier and prekey identifier into the authenticated establishment transcript/AD.
- Reject missing or mismatched bindings deterministically (fail-closed).

CI-gated assertions:
- CAT-S2-ESTABLISH-001 vectors include:
  - Positive establish with matching pq_kem_pub_id/pq_prekey_id bound values.
  - Negative establish when binding is missing (reject: REJECT_S2_ESTABLISH_PQ_BIND_MISSING).
  - Negative establish when binding mismatches (reject: REJECT_S2_ESTABLISH_PQ_BIND_MISMATCH).

Evidence:
- suite2-vectors CI logs and artifacts in artifacts/suite2/establish_vector_report.json.
