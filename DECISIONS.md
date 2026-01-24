# QuantumShield Decisions Log (ADR-lite)

This file records security- and protocol-relevant design decisions and invariants.

## How to add an entry
Append a new section using the template below.

### Template
- **ID:** D-XXXX
- **Date:** YYYY-MM-DD
- **Status:** Proposed | Accepted | Deprecated
- **Goal IDs:** G1/G2/G3/G4/G5
- **Decision:** (one paragraph)
- **Rationale:** (why this choice)
- **Security invariants introduced/changed:** (bullet list)
- **Alternatives considered:** (bullet list)
- **Implications for spec/impl/tests:** (bullet list)

---

## Entries

- **ID:** D-0001
- **Date:** 2025-12-27
- **Status:** Accepted
- **Goal IDs:** G1, G2, G3, G4, G5
- **Decision:** Establish GOALS.md and associated governance gates as canonical project controls.
- **Rationale:** Prevent drift, enforce fail-closed and verification discipline, and ensure all work advances Suite-2 objectives.
- **Security invariants introduced/changed:**
  - Every PR must declare Goal IDs.
  - Protocol-behavior changes require traceability and tests/vectors in the same PR.
  - Downgrades must be fail-closed and transcript-bound when Suite-2 is supported.
- **Alternatives considered:**
  - Informal guidance only (rejected: insufficient enforcement).
- **Implications for spec/impl/tests:**
  - Add PR templates and CI checks.
  - Maintain TRACEABILITY.md mapping from goals to artifacts.

- **ID:** D-0002
- **Date:** 2025-12-27
- **Status:** Accepted
- **Goal IDs:** G2, G3, G4
- **Decision:** Execute Suite-2 conformance vectors for SCKA logic (CAT-SCKA-LOGIC-001) and downgrade resistance (CAT-S2-DOWNGRADE-001) in CI, fail-closed.
- **Rationale:** Suite-2 requires deterministic, testable accept/reject rules for SCKA state logic and for negotiation downgrade resistance. CI execution prevents drift and blocks silent regressions.
- **Security invariants introduced/changed:**
  - SCKA peer ADV monotonicity is enforced (non-monotonic ADV rejected).
  - SCKA CTXT targeting rejects unknown/consumed/tombstoned targets.
  - Suite-2 selection is fail-closed when Suite-2 is required; no silent fallback to Suite-1.
  - Negotiated protocol_version/suite_id must match AD; mismatches reject.
- **Alternatives considered:**
  - Spec-only guidance without executable vectors (rejected: insufficient enforcement).
- **Implications for spec/impl/tests:**
  - Add vector runners and actor ops for CAT-SCKA-LOGIC-001 and CAT-S2-DOWNGRADE-001.
  - Extend suite2-ci workflow to execute these categories and fail closed.

- **ID:** D-0003
- **Date:** 2025-12-27
- **Status:** Accepted
- **Goal IDs:** G1, G4
- **Decision:** Implement and execute CAT-SCKA-KEM-001 conformance vectors in suite2-ci to validate ML-KEM-768 correctness for SCKA epoch secrets.
- **Rationale:** SCKA safety depends on the cryptographic correctness of the KEM boundary. Deterministic KEM fixtures allow CI to detect regressions in encapsulation/decapsulation behavior and input validation without relying on probabilistic tests.
- **Security invariants introduced/changed:**
  - Deterministic ML-KEM-768 keygen + encapsulation + decapsulation roundtrips yield identical epoch secrets for identical inputs.
  - Wrong decapsulation keys and ciphertext tampering yield different epoch secrets (implicit rejection behavior).
  - Invalid input sizes for deterministic KEM fixtures reject with explicit reason codes.
- **Alternatives considered:**
  - Logic-only SCKA vectors without cryptographic checks (rejected: does not validate KEM boundary).
  - Randomized KEM tests (rejected: non-deterministic and brittle in CI).
- **Implications for spec/impl/tests:**
  - Add CAT-SCKA-KEM-001 vectors and a runner to scripts/ci.
  - Extend the refimpl actor with a dedicated `scka.kem.check` op for conformance execution.
  - Wire CAT-SCKA-KEM-001 into `.github/workflows/suite2.yml` so PRs fail closed on regressions.

- **ID:** D-0004
- **Date:** 2025-12-28
- **Status:** Accepted
- **Goal IDs:** G4
- **Decision:** Record and treat as exceptional any direct-to-main governance-only changes (docs + manifest) that bypass the PR flow.
- **Rationale:** The project governance model is PR-driven for auditability. When operator error results in a direct push, the exception must be recorded explicitly so the repository remains externally reviewable.
- **Security invariants introduced/changed:**
  - Direct pushes to `main` are discouraged; if they occur, they must be recorded as an exception in DECISIONS.md.
- **Alternatives considered:**
  - Rewrite history to “manufacture” a PR trail (rejected: reduces transparency).
- **Implications for spec/impl/tests:**
  - Governance-only direct pushes require an immediate follow-up PR that records the exception.

- **ID:** D-0005
- **Date:** 2025-12-28
- **Status:** Accepted
- **Goal IDs:** G1, G2, G3, G4
- **Decision:** Make DOC-CAN-003 (Suite-2 / QSP v5.0) self-contained and implementable by defining Suite-2 namespaces, downgrade fail-closed rules, transcript/AD binding, the full Suite-2 key schedule (including per-message KDF_HYBRID(ec_mk, pq_mk)), and explicit ratchet boundary semantics aligned to CI-gated Suite-2 categories.
- **Rationale:** NA-0003 requires eliminating security-sensitive ambiguity and removing reliance on prior phase documents for required meaning. A complete normative spec is necessary for interoperable implementations and for conformance vectors to be meaningful.
- **Security invariants introduced/changed:**
  - Suite-2 selection is fail-closed when required by policy and peer capability.
  - `protocol_version` and `suite_id` are bound into AEAD associated data; mismatches reject.
  - Per-message body keys are always hybrid: `mk = KDF_HYBRID(ec_mk, pq_mk)`.
  - SCKA reseed ordering is explicit and commit is fail-closed (no partial commits).
- **Alternatives considered:**
  - Keep DOC-CAN-003 as scaffolding and rely on tests/implementation behavior (rejected: not interoperable and not auditable).
- **Implications for spec/impl/tests:**
  - Update TRACEABILITY.md to reference explicit DOC-CAN-003 sections.
  - Ensure Suite-2 KDF vector ops and CAT-S2-DOWNGRADE-001 remain aligned to the doc’s KDF labels and ordering.
- **ID:** D-0006
- **Date:** 2025-12-28
- **Status:** Accepted
- **Goal IDs:** G2, G4 (supports G1)
- **Decision:** Specify SCKA (DOC-CAN-004) as an implementable, fail-closed PQ control-plane for Suite-2, including strict peer ADV monotonicity, one-time ciphertext targeting with tombstoning, and transactional commit semantics tied to Suite-2 body AEAD success.
- **Rationale:** SCKA fields are public but transcript-bound, and ML-KEM decapsulation does not reliably signal invalid ciphertexts. Strict, explicit logic and commit gating are required to prevent downgrade/DoS vectors (key consumption or epoch advancement) and to preserve crash/rollback safety.
- **Security invariants introduced/changed:**
  - Peer ADV is accepted iff `pq_adv_id` strictly increases; non-monotonic ADV-bearing messages are rejected.
  - Each locally advertised PQ receive key may be targeted at most once; consumed targets are tombstoned and permanently rejected.
  - SCKA state updates are staged and committed atomically only if Suite-2 message processing commits (no partial commits on decrypt failure).
  - Fixed-size parsing for ML-KEM-768 fields (pk=1184, ct=1088, ss=32) is enforced fail-closed.
- **Alternatives considered:**
  - Allow repeated ADV IDs when the public key matches (rejected: complicates deterministic accept/reject and weakens monotonicity signal).
  - Mark targets consumed prior to body decrypt success (rejected: enables unauthenticated DoS by consuming receive keys with invalid ciphertexts).
- **Implications for spec/impl/tests:**
  - DOC-CAN-004 defines the normative logic and reject codes referenced by CAT-SCKA-LOGIC-001 and CAT-SCKA-KEM-001.
  - TRACEABILITY.md must map G2 to the new DOC-CAN-004 sections and SCKA vector fixtures.

- **ID:** D-0007
- **Date:** 2025-12-28
- **Status:** Accepted
- **Goal IDs:** G4 (supports G2, G1)
- **Decision:** Add a CI-gated, bounded executable model lane (`formal/`) and run it fail-closed to provide an immediate machine-checkable verification gate while broader cryptographic models (e.g., ProVerif/Tamarin) are selected and introduced.
- **Rationale:** G4 requires verification as a release gate. A bounded, dependency-free model check can run deterministically on every PR and block regressions in the most security-sensitive SCKA logic invariants, without waiting for heavyweight proof tooling integration.
- **Security invariants introduced/changed:**
  - Formal model checks must run in CI and fail closed if any modeled invariant is violated.
  - The initial modeled invariants include: peer ADV strict monotonicity, one-time CTXT targeting, tombstoning, and “reject implies no persistent state change.”
  - Changes to exploration bounds (depth, replay limits) are governance-relevant and should be recorded (to avoid weakening the gate silently).
- **Alternatives considered:**
  - Wait to select ProVerif/Tamarin before adding any executable model (rejected: delays G4 gating).
  - Treat conformance vectors as “formal” substitutes (rejected: tests are necessary but not a model-check gate).
- **Implications for spec/impl/tests:**
  - Add `.github/workflows/formal.yml` and the `formal/` directory with a bounded SCKA logic model runner.
  - Update TRACEABILITY.md (G4) to reference the formal CI lane and the model entry point.
  - Keep the formal model scope and limitations explicit in FORMAL_VERIFICATION_PLAN.md.

- **ID:** D-0008
- **Date:** 2025-12-28
- **Status:** Accepted
- **Goal IDs:** G4 (supports G1–G3)
- **Decision:** Consolidate all workflow and “what to do next” guidance into START_HERE.md and NEXT_ACTIONS.md, and require deprecated documents to be explicitly non-operative with redirect notes.
- **Rationale:** Multiple competing starters/queues create drift risk and weaken fail-closed governance. A single authoritative queue and constitution reduce operational ambiguity and keep goal-lint enforcement meaningful.
- **Security invariants introduced/changed:**
  - NEXT_ACTIONS.md is the only ordered execution queue; supporting plans may not define competing queues.
  - Deprecated documents must begin with a deprecation header and must not contain operative workflow instructions.
- **Alternatives considered:**
  - Keep legacy starters/queues intact “for convenience” (rejected: increases drift and contradictory instructions).
  - Move all legacy content to an external archive immediately (rejected: unnecessary churn; incremental deprecation is sufficient).
- **Implications for spec/impl/tests:**
  - Update DOC-CTRL-001 with an explicit deprecation/consolidation policy.
  - Update legacy repo-root documents (e.g., CHAT_STARTER.md, START_HERE_2.md, ALL_CHATS.md) to redirect to START_HERE.md and NEXT_ACTIONS.md.

- **ID:** D-0009
- **Date:** 2025-12-28
- **Status:** Accepted
- **Goal IDs:** G4 (supports G1–G3)
- **Decision:** Expand Suite-2 conformance categories to explicitly cover protocol-level composition (transcript/AD binding, per-message hybrid mk correctness, SCKA reseed integration, out-of-order/replay behavior, and crash/rollback persistence invariants).
- **Rationale:** Component-level correctness (KDFs, SCKA logic/KEM) is necessary but insufficient. Protocol-level composition is where downgrade, replay, and state-consumption failures emerge; the conformance plan must explicitly enumerate these categories to keep future implementation work scoped and fail-closed.
- **Security invariants introduced/changed:**
  - Protocol-level vectors must be specified to test reject-on-tamper semantics and “reject implies no persistent state change.”
  - Out-of-order acceptance and replay rejection must be deterministic and bounded.
  - Crash/rollback persistence invariants must be testable and aligned with durability gates.
- **Alternatives considered:**
  - Defer protocol-level categories until after implementation (rejected: encourages ad-hoc testing and makes regressions harder to detect).
- **Implications for spec/impl/tests:**
  - DOC-TST-005 enumerates the new planned categories and maps them to canonical spec sections.
  - NA-0006 is unblocked to implement the protocol-level runner and vectors for these categories.

- **ID:** D-0010
- **Date:** 2025-12-31
- **Status:** Accepted
- **Goal IDs:** G4 (supports G1, G2, G3)
- **Decision:** For NA-0006 bring-up, permit baseline unblocking and Suite-2 lane scaffolding changes that do not alter Suite-1/Suite-1B wire semantics; Suite-2 remains explicitly fail-closed (reject) until end-to-end implemented and CI-gated.
- **Rationale:** CI goal-lint requires governance updates when core protocol paths change. This PR unblocks the refimpl baseline (x25519-dalek StaticSecret feature gate) and aligns parse-only fixtures to current in-repo parser semantics where no in-repo canonical QSP 4.x requirement mandates stricter fixed-length enforcement. Suite-2 is advertised for capability discovery but must reject actual Suite-2 operations until implementation lands.
- **Security invariants introduced/changed:**
  - Suite-1/Suite-1B wire behavior unchanged by NA-0006 preparation work.
  - Suite-2 use is fail-closed via explicit rejection (REJECT_S2_NOT_IMPLEMENTED) until implemented with vectors.
  - No new Suite-1/Suite-1B strictness is introduced absent an explicit in-repo canonical requirement.
- **Alternatives considered:**
  - Introduce new Suite-1/Suite-1B parser enforcement (rejected: behavior change without in-repo canonical requirement).
  - Bypass tests/CI gates (rejected: governance violation).
- **Implications for spec/impl/tests:**
  - Cargo: enable x25519-dalek static_secrets for StaticSecret availability.
  - Tests: parse_only.json expectations aligned to current parser semantics (wire bytes unchanged).
  - Impl: Suite-2 module skeleton added; actor advertises Suite-2 but rejects its use until implemented.

- **ID:** D-0011
- **Date:** 2025-12-31
- **Status:** Accepted
- **Goal IDs:** G3, G4
- **Decision:** Implement Suite-2 §5.1 AD binding helpers in refimpl, expose a dedicated actor op (`suite2.transcript.check`) that rejects any AD mismatch, and add CI-gated CAT-S2-TRANSCRIPT-001 vectors + runner to suite2-ci.
- **Rationale:** Protocol-level Suite-2 transcript/AD binding is a security-critical invariant that must be enforced and tested end-to-end; adding a minimal, fail-closed vector category provides immediate coverage without altering Suite-1/Suite-1B behavior.
- **Security invariants introduced/changed:**
  - AD binding uses SHA-512 and the `QSP5.0/PQ-BIND` label per DOC-CAN-003 §5.1; mismatches are rejected with `REJECT_S2_AD_MISMATCH`.
  - Suite-2 transcript binding is now CI-gated via CAT-S2-TRANSCRIPT-001 vectors.
  - Suite-1/Suite-1B behavior remains unchanged.
- **Alternatives considered:**
  - Defer transcript vectors until full Suite-2 ratchet implementation (rejected: leaves binding invariant untested).
- **Implications for spec/impl/tests:**
  - Impl: `tools/refimpl/quantumshield_refimpl/src/suite2/binding.rs` + actor op in `tools/actors/refimpl_actor_rs/src/main.rs`.
  - Tests: `inputs/suite2/vectors/qshield_suite2_transcript_vectors_v1.json` + `scripts/ci/run_suite2_transcript_vectors.py` + suite2-ci wiring.

- **ID:** D-0012
- **Date:** 2025-12-31
- **Status:** Accepted
- **Goal IDs:** G1, G4
- **Decision:** Add a minimal Suite-2 per-message mk derivation helper and CI-gated CAT-S2-MK-001 vectors via a dedicated actor op (`suite2.mk_hybrid.check`) to verify KDF_EC_CK, KDF_PQ_CK, and KDF_HYBRID composition.
- **Rationale:** Per-message mk derivation is a core Suite-2 invariant (G1) and must be tested at protocol level; a small, fail-closed conformance op provides coverage without touching Suite-1/1B behavior.
- **Security invariants introduced/changed:**
  - Each mk is derived from both EC and PQ chain keys using the canonical KMAC labels and suffix bytes (DOC-CAN-003 §§3.3.1, 3.3.4, 3.3.5).
  - CAT-S2-MK-001 is CI-gated; mismatches reject with `REJECT_S2_MK_MISMATCH`.
  - Suite-1/Suite-1B behavior remains unchanged.
- **Alternatives considered:**
  - Defer mk vectors until full Suite-2 ratchet implementation (rejected: leaves G1 invariant untested).
- **Implications for spec/impl/tests:**
  - Impl: `tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs` + actor op in `tools/actors/refimpl_actor_rs/src/main.rs`.
  - Tests: `inputs/suite2/vectors/qshield_suite2_mk_hybrid_vectors_v1.json` + `scripts/ci/run_suite2_mk_hybrid_vectors.py` + suite2-ci wiring.

- **ID:** D-0013
- **Date:** 2025-12-31
- **Status:** Accepted
- **Goal IDs:** G2, G4
- **Decision:** Add a minimal Suite-2 PQ reseed integration helper and CI-gated CAT-S2-PQRESEED-001 vectors via a dedicated actor op (`suite2.pqreseed.apply`) with transactional commit semantics.
- **Rationale:** PQ reseed integration couples SCKA rules with Suite-2 PQ chain seeds; this invariant must be exercised and gated without changing Suite-1/1B behavior.
- **Security invariants introduced/changed:**
  - PQ reseed derives directional seeds using `QSP5.0/PQSEED/*` and CTXT context per DOC-CAN-003 §3.3.6.
  - Monotonic ADV and one-time target consumption checks reject on violation; commit=false does not mutate state.
  - Suite-1/Suite-1B behavior remains unchanged.
- **Alternatives considered:**
  - Defer reseed vectors until full Suite-2 ratchet implementation (rejected: leaves G2 invariant untested).
- **Implications for spec/impl/tests:**
  - Impl: `tools/refimpl/quantumshield_refimpl/src/suite2/scka.rs` + actor op in `tools/actors/refimpl_actor_rs/src/main.rs`.
  - Tests: `inputs/suite2/vectors/qshield_suite2_pq_reseed_vectors_v1.json` + `scripts/ci/run_suite2_pqreseed_vectors.py` + suite2-ci wiring.

- **ID:** D-0014
- **Date:** 2025-12-31
- **Status:** Accepted
- **Goal IDs:** G2, G4
- **Decision:** Enforce fixed-size ML-KEM-768 ciphertext length (1088 bytes) in Suite-2 PQ reseed integration; reject invalid length with `REJECT_SCKA_CTXT_BAD_CT_LEN`.
- **Rationale:** DOC-CAN-004 §3.4 requires fail-closed fixed-size CTXT parsing; enforcing this in the Suite-2 PQ reseed path prevents malformed ciphertexts from being hashed or committed.
- **Security invariants introduced/changed:**
  - `pq_ct` length must be exactly 1088 bytes for PQ reseed; otherwise reject before any derivation or state updates.
  - Reject reason is explicit and consistent with SCKA taxonomy.
- **Alternatives considered:**
  - Defer length enforcement to later ratchet integration (rejected: violates fail-closed parsing requirements).
- **Implications for spec/impl/tests:**
  - Impl: `tools/refimpl/quantumshield_refimpl/src/suite2/scka.rs` length check in `apply_pq_reseed`.
  - Tests: update PQ reseed vectors to use 1088-byte ciphertexts and add explicit reject case for bad length.

- **ID:** D-0015
- **Date:** 2025-12-31
- **Status:** Accepted
- **Goal IDs:** G4
- **Decision:** Enforce fail-closed QSP parsing: `hdr_ct_len` must be 24 bytes and `body_ct_len` must be at least 16 bytes; reject malformed frames in `ProtocolMessage::decode`.
- **Rationale:** The harness parser enforces fixed-size header ciphertext and a minimum body length; refimpl must match to avoid accepting malformed untrusted inputs.
- **Security invariants introduced/changed:**
  - Reject `hdr_ct_len != 24` with `hdr_ct_len` parse error.
  - Reject `body_ct_len < 16` with `body_ct_len` parse error.
- **Alternatives considered:**
  - Keep permissive parsing and rely on later checks (rejected: violates fail-closed parsing of untrusted inputs).
- **Implications for spec/impl/tests:**
  - Impl: `tools/refimpl/quantumshield_refimpl/src/qsp/types.rs` length checks in `ProtocolMessage::decode`.
  - Tests: `tools/refimpl/quantumshield_refimpl/vectors/parse_only.json` updated to expect rejects for P-QSP-0006/0007.

- **ID:** D-0016
- **Date:** 2025-12-31
- **Status:** Accepted
- **Goal IDs:** G2, G4
- **Decision:** Implement a minimal Suite-2 out-of-order/replay receive path for non-boundary messages only, with bounded MKSKIPPED handling and explicit reject codes.
- **Rationale:** CAT-S2-OOO-001 requires deterministic replay/OOO behavior; limiting scope to non-boundary messages provides fail-closed coverage without touching Suite-1/1B paths.
- **Security invariants introduced/changed:**
  - Non-boundary OOO handling enforces `MAX_SKIP` and `MAX_MKSKIPPED` bounds.
  - Replay detection and auth failures return explicit codes (`REJECT_S2_REPLAY`, `REJECT_S2_OOO_BOUNDS`, `REJECT_S2_HDR_AUTH_FAIL`, `REJECT_S2_BODY_AUTH_FAIL`).
  - MKSKIPPED entry matching `(DH_pub, N)` is deleted on use; if body decrypt fails, deletion is still committed (fail-closed).
- **Alternatives considered:**
  - Full boundary-aware OOO handling in this step (rejected: scope too broad).
- **Implications for spec/impl/tests:**
  - Impl: `tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs` OOO receive helper; actor op `suite2.ooo_replay.run`.
  - Tests: `inputs/suite2/vectors/qshield_suite2_ooo_replay_vectors_v1.json` + `scripts/ci/run_suite2_ooo_replay_vectors.py` + suite2-ci wiring.

- **ID:** D-0017
- **Date:** 2025-12-31
- **Status:** Accepted
- **Goal IDs:** G2, G4
- **Decision:** Add a minimal Suite-2 boundary receive path (CAT-S2-BOUNDARY-001) that applies PQ reseed integration only for in-order boundary messages, with strict PQ_PREFIX parsing and fail-closed reject codes.
- **Rationale:** NA-0006 requires SCKA epoch integration in the Suite-2 receive path. A limited, in-order-only boundary handler provides transactional reseed coverage without expanding to full DH/OOO boundary logic.
- **Security invariants introduced/changed:**
  - Boundary messages are accepted only when `N == Nr`; otherwise reject with `REJECT_S2_BOUNDARY_NOT_IN_ORDER`.
  - PQ_PREFIX is parsed strictly per flag invariants and fixed sizes; malformed prefixes reject with `REJECT_S2_PQPREFIX_PARSE`.
  - PQ reseed is applied transactionally after header+body decrypt; SCKA reject codes (e.g., `REJECT_SCKA_CTXT_BAD_CT_LEN`) are bubbled.
  - Boundary support is limited to `FLAG_BOUNDARY|FLAG_PQ_CTXT`; `FLAG_PQ_ADV` is rejected as unsupported in this micro-step.
  - Boundary header auth uses the current header key (`hk_r`) as a proxy for NHK, and `pq_epoch_ss` is supplied by the conformance op (decapsulation not implemented yet).
- **Alternatives considered:**
  - Full DH boundary + PQ ADV support in the same step (rejected: scope too broad for NA-0006 micro-step sequencing).
- **Implications for spec/impl/tests:**
  - Impl: `tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs` boundary receive helper; actor op `suite2.boundary.run`.
  - Tests: `inputs/suite2/vectors/qshield_suite2_boundary_vectors_v1.json` + `scripts/ci/run_suite2_boundary_vectors.py` + suite2-ci wiring.

- **ID:** D-0018
- **Date:** 2025-12-31
- **Status:** Accepted
- **Goal IDs:** G4
- **Decision:** Add CAT-S2-PARSE-001 strict parse vectors and a Suite-2 ratchet message decoder for fail-closed parsing of prefix/flags and ciphertext lengths.
- **Rationale:** NA-0006 requires protocol-level, fail-closed parsing of Suite-2 ratchet message bytes before crypto/state; this provides CI-gated coverage without touching Suite-1/1B paths.
- **Security invariants introduced/changed:**
  - Reject malformed ratchet prefixes or truncated buffers (`REJECT_S2_PARSE_PREFIX`).
  - Enforce `hdr_ct` length == 24 and `body_ct` length >= 16 (`REJECT_S2_PARSE_HDR_LEN`, `REJECT_S2_PARSE_BODY_LEN`).
  - Enforce flag invariants and PQ_PREFIX sizing (`REJECT_S2_PARSE_FLAGS`, `REJECT_S2_PQPREFIX_PARSE`).
- **Alternatives considered:**
  - Reuse Suite-1 parser for Suite-2 ratchet bytes (rejected: Suite-2 has distinct prefix + PQ_PREFIX invariants).
- **Implications for spec/impl/tests:**
  - Impl: `tools/refimpl/quantumshield_refimpl/src/suite2/parse.rs` + actor op `suite2.parse.check`.
  - Tests: `inputs/suite2/vectors/qshield_suite2_parse_vectors_v1.json` + `scripts/ci/run_suite2_parse_vectors.py` + suite2-ci wiring.

- **ID:** D-0019
- **Date:** 2025-12-31
- **Status:** Accepted
- **Goal IDs:** G2, G4
- **Decision:** Add CAT-S2-E2E-RECV-001 end-to-end receive vectors using raw wire bytes, exercising parse → header decrypt → body decrypt → transactional state commit in a single path.
- **Rationale:** NA-0006 requires strict parsing and state transitions to be tested together on raw wire bytes; this ties parse + OOO + boundary + reseed logic into one CI-gated receive path without touching Suite-1/1B.
- **Security invariants introduced/changed:**
  - Raw wire bytes are parsed via Suite-2 decoder before any crypto/state changes.
  - Rejects on parse/flag invariant failures, header/body auth failures, bounds violations, and SCKA errors with stable reason codes.
  - State is only mutated after full success per commit rule; failure leaves state unchanged.
- **Alternatives considered:**
  - Separate tests for parse and receive only (rejected: does not validate end-to-end transactional behavior on raw bytes).
- **Implications for spec/impl/tests:**
  - Impl: `tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs` recv_wire glue + actor op `suite2.e2e.recv`.
  - Tests: `inputs/suite2/vectors/qshield_suite2_e2e_recv_vectors_v1.json` + `scripts/ci/run_suite2_e2e_recv_vectors.py` + suite2-ci wiring.

- **ID:** D-0020
- **Date:** 2025-12-31
- **Status:** Accepted
- **Goal IDs:** G4
- **Decision:** Add CAT-S2-INTEROP-001 vectors and a Suite-2 send op that exercises a two-actor send→wire→recv exchange for flags==0 only.
- **Rationale:** NA-0006 requires interop coverage; a minimal, fail-closed interop lane validates send/recv symmetry without touching Suite-1/1B or boundary flows.
- **Security invariants introduced/changed:**
  - Suite-2 send is restricted to flags==0 and rejects unsupported flags with `REJECT_S2_LOCAL_UNSUPPORTED`.
  - Interop vectors validate round-trip plaintext and symmetric state advancement (Ns/Nr) across two actors.
- **Alternatives considered:**
  - Full boundary interop or 4B harness expansion (rejected: scope for later micro-step).
- **Implications for spec/impl/tests:**
  - Impl: `tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs` send_wire + actor op `suite2.e2e.send`.
  - Tests: `inputs/suite2/vectors/qshield_suite2_interop_vectors_v1.json` + `scripts/ci/run_suite2_interop_vectors.py` + suite2-ci wiring.

- **ID:** D-0021
- **Date:** 2025-12-31
- **Status:** Accepted
- **Goal IDs:** G2, G4
- **Decision:** Add Suite-2 durability snapshot/restore with strict versioning and bind rollback detection to SCKA monotonic invariants; gate CAT-S2-CRASH-001 in suite2-ci.
- **Rationale:** NA-0007 requires crash/restart resilience and rollback detection for Suite-2; binding to SCKA monotonic state enforces fail-closed durability without changing Suite-1/1B behavior.
- **Security invariants introduced/changed:**
  - Suite-2 snapshots are versioned and fail-closed on parse errors or unknown versions.
  - Rollback detection rejects restore if `peer_max_adv_id_seen`, `local_next_adv_id`, or `tombstones` would regress (REJECT_SCKA_ROLLBACK_DETECTED).
  - Durable replay detection applies to Suite-2 raw-wire receive under test hooks (QSL_DUR_STORE_DIR).
- **Alternatives considered:**
  - Implement Suite-2 handshake to reuse encrypt/decrypt durability path (rejected: no in-repo canonical handshake semantics).
- **Implications for spec/impl/tests:**
  - Impl: `tools/refimpl/quantumshield_refimpl/src/suite2/state.rs` + actor durability helpers + suite2 session snapshot/restore.
  - Tests: `inputs/suite2/vectors/qshield_suite2_crash_restart_vectors_v1.json` + `scripts/ci/run_suite2_crash_restart_vectors.py` + suite2-ci wiring.

- **ID:** D-0022
- **Date:** 2025-12-31
- **Status:** Accepted
- **Goal IDs:** G4
- **Decision:** Add an independent Python actor for Suite-2 KDF/transcript/mk conformance ops and gate it in suite2-ci alongside refimpl_actor.
- **Rationale:** NA-0009 requires interop evidence that key-schedule/KDF semantics are not single-implementation artifacts; a stdlib-only Python actor provides an independent surface without inventing Suite-2 handshake semantics.
- **Security invariants introduced/changed:**
  - Fail-closed typed_data parsing for Suite-2 KDF/transcript/mk ops in an independent implementation.
  - CI gating requires the independent actor to match existing vectors for CAT-S2-KDF-001, CAT-S2-TRANSCRIPT-001, and CAT-S2-MK-001.
- **Alternatives considered:**
  - Reuse refimpl crate in a wrapper actor (rejected: not an independent implementation).
  - Implement Suite-2 handshake to reuse encrypt/decrypt paths (rejected: no canonical handshake spec in-repo).
- **Implications for spec/impl/tests:**
  - Impl: `tools/actors/interop_actor_py/interop_actor.py` (stdlib-only).
  - CI: suite2-ci runs CAT-S2-KDF-001 / CAT-S2-TRANSCRIPT-001 / CAT-S2-MK-001 against the python actor.

- **ID:** D-0023
- **Date:** 2025-12-31
- **Status:** Accepted
- **Goal IDs:** G4
- **Decision:** Add CAT-S2-INTEROP-XIMPL-001 to gate cross-implementation Suite-2 wire interop using refimpl_actor and the independent python actor for flags==0 only.
- **Rationale:** NA-0009 requires evidence that two independent implementations can exchange real Suite-2 wire bytes; restricting to non-boundary messages avoids inventing handshake semantics.
- **Security invariants introduced/changed:**
  - Cross-implementation send→wire→recv must preserve plaintext and state advancement symmetry.
  - Any unsupported flags are rejected with `REJECT_S2_LOCAL_UNSUPPORTED` (fail-closed).
- **Alternatives considered:**
  - Full boundary interop in this step (rejected: scope and missing handshake semantics).
- **Implications for spec/impl/tests:**
  - Impl: `tools/actors/interop_actor_py/interop_actor.py` adds `suite2.e2e.send` / `suite2.e2e.recv` (flags==0 only).
  - Tests: `inputs/suite2/vectors/qshield_suite2_interop_ximpl_vectors_v1.json` + `scripts/ci/run_suite2_interop_ximpl_vectors.py` + suite2-ci wiring.

- **ID:** D-0024
- **Date:** 2025-12-31
- **Status:** Accepted
- **Goal IDs:** G3, G4
- **Decision:** Define Suite-2 session establishment as a base-handshake contract and explicit QSE mapping in DOC-CAN-003 §6, without introducing new wire formats.
- **Rationale:** NA-0011 requires a self-contained establishment story to prevent ad-hoc implementation; the spec already defines negotiation, session_id, and initialization inputs but lacked a single normative mapping section.
- **Security invariants introduced/changed:**
  - Suite-2 establishment is gated by §2 negotiation checks and must bind negotiated `(protocol_version, suite_id)` into AD.
  - Base handshake outputs (`session_id`, `dh_init`, `pq_init_ss`) are length-validated and fail-closed.
  - Unknown `msg_type` for `(0x0500, 0x0002)` is rejected; unauthenticated commitment to Suite-2 negotiation is rejected.
- **Alternatives considered:**
  - Define a new Suite-2-specific handshake wire format (rejected: no canonical basis; would invent semantics).
- **Implications for spec/impl/tests:**
  - Spec: `docs/canonical/DOC-CAN-003_QSP_Suite-2_True_Triple_Ratchet_v5.0.0_DRAFT.md` §6 adds establishment mapping and explicit reject rules.
  - Registry: `docs/spec-closure/DOC-SCL-002_Shared_Schemas_Error_Reason_Code_Registry_v1.0_DRAFT.md` adds Suite-2 establishment reject identifiers.

- **ID:** D-0025
- **Date:** 2025-12-31
- **Status:** Accepted
- **Goal IDs:** G3, G4
- **Decision:** Anchor CAT-S2-ESTABLISH-001 in DOC-TST-005 and add a non-executable test-plan stub to couple NA-0012 establishment work to explicit vectors/runner/ops.
- **Rationale:** Goal-lint requires tests/harness coupling for core protocol path changes; this documents the establishment category and defers executable coverage to NA-0012 implementation without inventing new wire behavior.
- **Security invariants introduced/changed:**
  - Establishment mapping cases are explicitly enumerated (accept + fail-closed rejects) and tied to DOC-CAN-003 §6.
  - No Suite-1/1B behavior is changed; this is documentation-only anchoring.
- **Alternatives considered:**
  - Add executable vectors in the same step (rejected: NA-0012 Step 2 is anchors-only).
- **Implications for spec/impl/tests:**
  - Tests plan: `docs/test/DOC-TST-005_Suite-2_Conformance_Vector_Categories_v1.0.0_DRAFT.md` adds CAT-S2-ESTABLISH-001.
  - Testplan stub: `tests/NA-0012_suite2_establish_vectors_testplan.md`.

- **ID:** D-0026
- **Date:** 2025-12-31
- **Status:** Accepted
- **Goal IDs:** G3, G4
- **Decision:** Clarify Suite-2 base handshake contract to include authenticated DH public keys (`dh_self_pub`, `dh_peer_pub`) needed to initialize ratchet DH state.
- **Rationale:** NA-0012 requires a canonical, non-invented initialization for `DHs_pub`/`DHr_pub`; DOC-CAN-003 §6/§8.2 now explicitly binds these to the base handshake transcript.
- **Security invariants introduced/changed:**
  - `dh_self_pub` and `dh_peer_pub` are transcript-bound and strict-length validated; mismatches fail closed.
  - Establishment rejects remain limited to registered Suite-2 establishment codes (no new codes introduced).
- **Alternatives considered:**
  - Infer initial DH public keys from `dh_init` or session_id (rejected: not canonical).
- **Implications for spec/impl/tests:**
  - Spec: `docs/canonical/DOC-CAN-003_QSP_Suite-2_True_Triple_Ratchet_v5.0.0_DRAFT.md` §6.3/§6.5/§8.2 updated.
  - Testplan stub: `tests/NA-0012_suite2_establish_vectors_testplan.md` updated to include DH public inputs.

- **ID:** D-0027
- **Date:** 2025-12-31
- **Status:** Accepted
- **Goal IDs:** G3, G4
- **Decision:** Implement Suite-2 establishment as a dedicated actor op (`suite2.establish.run`) backed by a pure refimpl initializer, with strict length/auth checks and negotiated Suite-2 gating; store sessionful Suite-2 state for subsequent `suite2.e2e.send/recv` usage.
- **Rationale:** NA-0012 requires executable establishment without inventing new handshake wire formats; a strict base-handshake contract and explicit op enable conformance vectors while keeping Suite-1/Suite-1B unchanged.
- **Security invariants introduced/changed:**
  - Establishment rejects on bad msg_type, unauthenticated prerequisites, and length mismatches using registered Suite-2 reject codes.
  - Negotiated `(protocol_version, suite_id)` is strictly enforced; AD mismatch rejects are fail-closed.
  - Unset initial chain keys are represented as all-zero 32-byte arrays (Suite-2 only).
- **Alternatives considered:**
  - Implement Suite-2 via handshake_* ops (rejected: would entangle Suite-1 semantics).
  - Define a new on-wire handshake message (rejected: not canonical in DOC-CAN-003).
- **Implications for spec/impl/tests:**
  - Impl: `tools/refimpl/quantumshield_refimpl/src/suite2/establish.rs`; `tools/actors/refimpl_actor_rs/src/main.rs` (suite2.establish.run).
  - Tests: `inputs/suite2/vectors/qshield_suite2_establish_vectors_v1.json` + `scripts/ci/run_suite2_establish_vectors.py` + suite2-ci wiring.

- **ID:** D-0028
- **Date:** 2025-12-31
- **Status:** Accepted
- **Goal IDs:** G4
- **Decision:** Prefer sessionful Suite-2 harness flows when establish stanzas are present: runners invoke `suite2.establish.run` and then use session_id lookups for `suite2.e2e.send/recv`; legacy send_state/recv_state inputs remain supported for back-compat.
- **Rationale:** NA-0013 requires sessionful establishment→send/recv adoption without changing Suite-1/1B behavior or wire semantics; runner preference exercises the canonical establishment contract while keeping fail-closed validation for missing inputs.
- **Security invariants introduced/changed:**
  - Sessionful harness paths are fail-closed if establishment inputs are missing or malformed.
  - Suite-2 sessionful state is used only when explicitly established; otherwise legacy state injection remains required.
  - No Suite-1/1B behavior changes; no qsp/* changes.
- **Alternatives considered:**
  - Force all vectors to remove send_state/recv_state immediately (rejected: would break legacy vectors and reduce incremental safety).
- **Implications for spec/impl/tests:**
  - Tests: `inputs/suite2/vectors/qshield_suite2_interop_vectors_v1.json`, `inputs/suite2/vectors/qshield_suite2_interop_ximpl_vectors_v1.json`, `inputs/suite2/vectors/qshield_suite2_crash_restart_vectors_v1.json` now include establishment stanzas for sessionful coverage.
  - Harness: `scripts/ci/run_suite2_interop_vectors.py`, `scripts/ci/run_suite2_interop_ximpl_vectors.py`, `scripts/ci/run_suite2_crash_restart_vectors.py` prefer sessionful flows when establishment stanzas exist.
  - Actor: `tools/actors/interop_actor_py/interop_actor.py` supports sessionful `suite2.establish.run` and session_id-based `suite2.e2e.send/recv` for cross-impl interop.

- **ID:** D-0029
- **Date:** 2026-01-01
- **Status:** Accepted
- **Goal IDs:** G5 (supports G1–G4)
- **Decision:** Introduce a non-production demo CLI scaffold (`qshield`) with a local relay stub, explicitly separated from protocol-core behavior and Suite-1/Suite-1B semantics.
- **Rationale:** NA-0015 requires a polished, repeatable demo surface without implying production readiness or altering protocol behavior. A scoped CLI scaffold provides UX structure while keeping all protocol-core logic unchanged.
- **Security invariants introduced/changed:**
  - Demo-only tooling is explicitly non-production and fail-closed on missing config/store inputs.
  - No Suite-1/Suite-1B behavior or wire semantics are modified.
- **Alternatives considered:**
  - Defer any demo tooling until full relay features are implemented (rejected: NA-0015 requires early UX scaffolding).
- **Implications for spec/impl/tests:**
  - Demo-only app: `apps/qshield-cli/` (binary `qshield`) with init/relay/status scaffolding and clear non-production disclaimer.
  - No protocol-core code paths or vectors are altered.

- **ID:** D-0030
- **Date:** 2026-01-01
- **Status:** Accepted
- **Goal IDs:** G5 (supports G1–G4)
- **Decision:** Implement the NA-0015 PR2 demo vertical slice: local relay endpoints plus sessionful Suite-2 establish/send/recv via the existing actor interface, with deterministic demo-only establishment inputs.
- **Rationale:** NA-0015 requires a working end-to-end demo without inventing new protocol behavior. Using the actor interface keeps the demo wire-neutral while enabling a repeatable UX flow.
- **Security invariants introduced/changed:**
  - Demo-only establishment derives deterministic inputs from peer ids and public key placeholders; no production security claim.
  - Local relay is localhost-only by default; missing inputs/config fail closed.
  - Suite-1/Suite-1B behavior and protocol wire semantics remain unchanged.
- **Alternatives considered:**
  - Implement a production-grade relay and handshake (rejected: out of scope for NA-0015).
- **Implications for spec/impl/tests:**
  - Demo CLI: `apps/qshield-cli/` now supports register/establish/send/recv and a local relay queue.
  - Test plan: `tests/NA-0015_demo_cli_scaffold_testplan.md` updated to reflect executable demo coverage.

- **ID:** D-0031
- **Date:** 2026-01-01
- **Status:** Accepted
- **Goal IDs:** G5 (supports G1–G4)
- **Decision:** Make the NA-0015 demo a cryptographic end-to-end flow by using the actor interface for Suite-2 establish/send/recv and a local relay queue, and gate it with a deterministic CI smoke test.
- **Rationale:** PR3 must demonstrate a real encrypted round-trip without changing protocol wire semantics or Suite-1/1B behavior. The actor interface provides canonical Suite-2 operations for the demo while remaining non-production.
- **Security invariants introduced/changed:**
  - Demo establishment inputs are deterministic and persisted; missing inputs fail closed.
  - Local relay remains localhost-only by default; smoke uses a bounded, deterministic path.
  - No Suite-1/1B behavior changes; no qsp/* changes.
- **Alternatives considered:**
  - Keep the relay-only smoke and defer crypto (rejected: NA-0015 PR3 requires cryptographic round-trip).
- **Implications for spec/impl/tests:**
  - Demo CLI uses `suite2.establish.run`, `suite2.e2e.send`, and `suite2.e2e.recv` via the actor JSONL contract.
  - CI: `scripts/ci/demo_cli_smoke.sh` now performs a two-party encrypted round-trip.

- **ID:** D-0032
- **Date:** 2026-01-01
- **Status:** Accepted
- **Goal IDs:** G4, G5 (supports G1–G3)
- **Decision:** The NA-0015 demo CLI is explicitly non-production and local-relay-only. The installed binary is `qshield` (apps/qshield-cli/), relay is a subcommand (`qshield relay serve`), and the demo makes no metadata/anonymity claims; G5 work remains scoped to NA-0016.
- **Rationale:** Clarify the demo’s scope and naming to prevent drift and avoid overclaiming privacy properties not implemented.
- **Security invariants introduced/changed:**
  - Demo is local-only; no remote relay deployment in baseline.
  - No metadata minimization claims beyond implemented behavior.
- **Alternatives considered:**
  - Treat demo as production-ready client (rejected: scope mismatch).
  - Separate relay binary (rejected: naming/UX decision already fixed).
- **Implications for spec/impl/tests:**
  - Governance-only: naming/UX conventions and demo-only scope are recorded; no protocol semantics are changed.

- **ID:** D-0033
- **Date:** 2026-01-02
- **Status:** Accepted
- **Goal IDs:** G4 (supports G1–G3)
- **Decision:** Codify an exact goal-lint PR body format (Goals line) in AGENTS.md to reduce governance PR churn and prevent CI failures on metadata-only changes.
- **Rationale:** Goal-lint failures were repeatedly caused by missing or non-compliant Goals lines. A single explicit, stable format reduces ambiguity and enforces fail-closed governance.
- **Security invariants introduced/changed:**
  - PR bodies must include a standalone `Goals: G1, G2, ...` line with ASCII commas.
- **Alternatives considered:**
  - Allow flexible formatting (rejected: increases CI failures and ambiguity).
- **Implications for spec/impl/tests:**
  - Update AGENTS.md with the exact Goals line requirement; no protocol changes.

- **ID:** D-0034
- **Date:** 2026-01-02
- **Status:** Accepted
- **Goal IDs:** G5
- **Decision:** Harden the demo relay/CLI to be safe-by-default: token-authenticated relay endpoints, loopback-only binding unless explicitly and loudly overridden, bounded request sizes/queues, and explicit unauthenticated-demo overrides for session establishment.
- **Rationale:** NA-0016 requires a non-anonymity baseline that still avoids accidental unsafe exposure. The demo relay/CLI is a common entry point and must not default to insecure operation.
- **Security invariants introduced/changed:**
  - Relay endpoints (/register, /send, /poll, /bundle) require a bearer token by default.
  - Non-loopback binding is blocked unless explicitly acknowledged as unsafe.
  - Request bodies and per-recipient/global queues are bounded; overflow rejects without mutation.
  - Demo establishment requires explicit unauthenticated override; defaults fail-closed.
- **Alternatives considered:**
  - Keep unauthenticated endpoints for convenience (rejected: unsafe defaults).
  - Allow public binding with a single flag (rejected: too easy to misconfigure).
- **Implications for spec/impl/tests:**
  - Demo relay/CLI defaults updated; CI adds a metadata conformance smoke to enforce safe defaults.

- **ID:** D-0035
- **Date:** 2026-01-03
- **Status:** Accepted
- **Goal IDs:** G5
- **Decision:** Add the NA-0016 documentation backbone (threat model, leakage inventory, and transport profile v0.1) and explicitly tie CI metadata conformance checks to documented invariants.
- **Rationale:** G5 requires explicit, testable metadata claims. The docs define scope and residual leakage while ensuring CI enforcement aligns with stated invariants.
- **Security invariants introduced/changed:**
  - Metadata posture is explicitly non-anonymity baseline with honest residual leakage statements.
  - CI-gated metadata conformance checks map to named transport invariants.
- **Alternatives considered:**
  - Keep metadata posture implicit in code and README (rejected: unverifiable and non-auditable).
- **Implications for spec/impl/tests:**
  - Add docs/privacy/DOC-G5-001/002/003 and update NA-0016 testplan + TRACEABILITY.md.

- **ID:** D-0036
- **Date:** 2026-01-03
- **Status:** Accepted
- **Goal IDs:** G5
- **Decision:** Enforce safe local storage permissions for the demo CLI (0700 store dir, 0600 config/state files) and expand metadata conformance checks to cover token gating, queue caps, and unauthenticated-establish overrides.
- **Rationale:** The demo store contains identifiers and session material. Default filesystem permissions must prevent casual local leakage, and CI must verify the safe-by-default posture remains intact as the demo evolves.
- **Security invariants introduced/changed:**
  - Store directory defaults to 0700; config/state files default to 0600 on Unix.
  - Unauthenticated register/send/poll/bundle requests reject.
  - Queue caps must be enforced and observable (429) rather than silently accepting overload.
  - Demo unauthenticated override is explicit and off by default.
- **Alternatives considered:**
  - Rely on user umask (rejected: inconsistent and non-auditable).
  - Keep smoke checks limited to relay auth only (rejected: misses key invariants).
- **Implications for spec/impl/tests:**
  - Update metadata_conformance_smoke to validate permissions and safe defaults.
  - Update DOC-G5-002/DOC-G5-003 and NA-0016 testplan mapping accordingly.

- **ID:** D-0037
- **Date:** 2026-01-03
- **Status:** Accepted
- **Goal IDs:** G5
- **Decision:** Add an optional size-bucket padding envelope for the demo relay/CLI transport and CI-gate its correctness when enabled.
- **Rationale:** NA-0016 requires a measurable, honest size-leakage mitigation that is optional and testable without changing protocol core behavior.
- **Security invariants introduced/changed:**
  - When padding is enabled, the queued message length must match a configured bucket and pad_len must be consistent.
  - Padding is OFF by default; enabling requires explicit configuration.
- **Alternatives considered:**
  - Always-on padding (rejected: adds overhead and may surprise demo users).
  - No padding capability (rejected: leaves size leakage unaddressed).
- **Implications for spec/impl/tests:**
  - Update DOC-G5-002/DOC-G5-003 and metadata_conformance_smoke to assert bucket behavior.

- **ID:** D-0038
- **Date:** 2026-01-03
- **Status:** Accepted
- **Goal IDs:** G5
- **Decision:** Mark NA-0016 complete with evidence from PRs #61–#64 and CI-gated metadata conformance checks.
- **Rationale:** G5 requires explicit, testable metadata minimization claims; NA-0016 deliverables are complete and CI-gated.
- **Security invariants introduced/changed:**
  - Metadata conformance checks are enforced in CI for demo relay/CLI defaults and optional padding behavior.
- **Alternatives considered:**
  - Delay closeout until additional anonymity features exist (rejected: out of scope for NA-0016).
- **Implications for spec/impl/tests:**
  - NEXT_ACTIONS.md records evidence; TRACEABILITY.md captures PR linkage and CI gating.

- **ID:** D-0039
- **Date:** 2026-01-03
- **Status:** Accepted
- **Goal IDs:** G4, G5
- **Decision:** Add a clean-room comparative review lane (NA-0017) to extract hardening patterns from Signal without code reuse and map them into QSL Goals and future queue items.
- **Rationale:** The project benefits from a bounded, auditable gap analysis, but any direct code reuse would violate clean-room requirements and risk license/semantics drift.
- **Security invariants introduced/changed:**
  - Comparative review is read-only and paraphrased; no code copying.
- **Alternatives considered:**
  - Skip comparative review entirely (rejected: loses external benchmarking).
  - Import Signal code or test logic (rejected: violates clean-room constraints).
- **Implications for spec/impl/tests:**
  - DOC-REV-001 will be added as a review artifact with citations; follow-on NAs must be explicit and goal-mapped.

- **ID:** D-0040
- **Date:** 2026-01-03
- **Status:** Accepted
- **Goal IDs:** G4, G5
- **Decision:** Add the DOC-REV-001 scaffold for the NA-0017 clean-room comparative review, defining the bounded delta matrix structure and hygiene rules.
- **Rationale:** NA-0017 requires a decision-grade template before any comparative review work begins to prevent scope drift and licensing risks.
- **Security invariants introduced/changed:**
  - Clean-room review remains paraphrase-only with explicit citations.
- **Alternatives considered:**
  - Ad-hoc review notes without a structured template (rejected: hard to audit and map to goals).
- **Implications for spec/impl/tests:**
  - New docs/review/DOC-REV-001_Signal_Comparative_Review_v1.0.0_DRAFT.md scaffold.

- **ID:** D-0041
- **Date:** 2026-01-04
- **Status:** Accepted
- **Goal IDs:** G4, G5
- **Decision:** Populate DOC-REV-001 with the initial decision-grade delta matrix and Top-5 upgrade list based on clean-room, spec-cited Signal sources.
- **Rationale:** NA-0017 requires actionable, bounded comparative findings to seed future queue items without code reuse.
- **Security invariants introduced/changed:**
  - Clean-room paraphrase-only rule preserved; citations required for all comparative claims.
- **Alternatives considered:**
  - Leave DOC-REV-001 as a template until repo access is available (rejected: spec sources are sufficient for an initial decision-grade pass).
- **Implications for spec/impl/tests:**
  - DOC-REV-001 filled with an initial matrix, ranked upgrades, and non-goals; TRACEABILITY updated.

- **ID:** D-0042
- **Date:** 2026-01-04
- **Status:** Accepted
- **Goal IDs:** G4, G5
- **Decision:** Mark NA-0017 complete with the clean-room comparative review artifact and recorded evidence.
- **Rationale:** NA-0017’s deliverable is the decision-grade DOC-REV-001 plus traceable evidence; no code changes are required.
- **Security invariants introduced/changed:**
  - Clean-room posture remains enforced; no code copying from Signal.
- **Alternatives considered:**
  - Defer closeout until repo-based citations are added (rejected: spec-cited review is sufficient for initial closure).
- **Implications for spec/impl/tests:**
  - NEXT_ACTIONS.md records evidence; TRACEABILITY.md links PRs and DOC-REV-001.

- **ID:** D-0043
- **Date:** 2026-01-04
- **Status:** Accepted
- **Goal IDs:** G3, G4, G5
- **Decision:** Add NA-0018..NA-0028 follow-ons from NA-0017 comparative review; promote NA-0018 to READY to resume execution.
- **Rationale:** The comparative review yielded concrete, CI-enforceable gaps; the queue must continue with a single READY item.
- **Security invariants introduced/changed:**
  - None; governance-only queue extension.
- **Alternatives considered:**
  - Defer queue updates until implementation begins (rejected: stalls execution).
- **Implications for spec/impl/tests:**
  - Governance-only; no protocol semantics changed.

- **ID:** D-0054
- **Date:** 2026-01-04
- **Status:** Accepted
- **Goal IDs:** G2, G4
- **Decision:** Enforce bounded MKSKIPPED handling in Suite-2 with deterministic eviction (lowest N, then DH_pub), delete-on-use, and CI-gated negative vectors.
- **Rationale:** Prevent unbounded skipped-key growth and replay acceptance while keeping out-of-order handling deterministic and testable.
- **Security invariants introduced/changed:**
  - MKSKIPPED is capped at `MAX_MKSKIPPED = 1000` with deterministic eviction on insert.
  - Skipped keys are deleted on successful use; evicted or reused keys are rejected deterministically.
- **Alternatives considered:**
  - Reject any message that would exceed MKSKIPPED bounds (rejected: blocks progress despite deterministic eviction).
  - Random eviction (rejected: non-deterministic behavior and CI flakiness).
- **Implications for spec/impl/tests:**
  - DOC-CAN-003 §9.1.1/§9.3 updated; Suite-2 ratchet enforces eviction and delete-on-use; new OOO vectors gate reuse/eviction.

- **ID:** D-0055
- **Date:** 2026-01-04
- **Status:** Accepted
- **Goal IDs:** G4, G5
- **Decision:** Promote NA-0024 to READY to resume execution after NA-0023 completion.
- **Rationale:** Queue requires a single READY item; NA-0024 is the next priority doc-only mapping item from DOC-REV-001 follow-ons.
- **Security invariants introduced/changed:**
  - None; governance-only queue state update.
- **Alternatives considered:**
  - Leave NA-0024 in BACKLOG (rejected: stalls execution).
- **Implications for spec/impl/tests:**
  - Governance-only; no protocol semantics changed.

- **ID:** D-0056
- **Date:** 2026-01-04
- **Status:** Accepted
- **Goal IDs:** G4, G5
- **Decision:** Define a self-contained mapping from PQXDH-style bundle outputs to SCKA initial epoch state and lock it with CI-gated vectors.
- **Rationale:** Make initialization deterministic and testable without external references; prevent ambiguity in PQXDH-to-SCKA mapping.
- **Security invariants introduced/changed:**
  - SCKA epoch 0 state is derived from authenticated base handshake outputs with strict length checks.
  - Initial SCKA state has empty target sets and monotonic counters at zero.
- **Alternatives considered:**
  - Leave mapping implicit in implementation (rejected: undermines conformance and CI).
- **Implications for spec/impl/tests:**
  - DOC-CAN-004 §3.5 added; SCKA logic vectors include mapping case.

- **ID:** D-0057
- **Date:** 2026-01-04
- **Status:** Accepted
- **Goal IDs:** G3
- **Decision:** Promote NA-0025 to READY to resume execution after NA-0024 completion.
- **Rationale:** Queue requires a single READY item; NA-0025 is the next priority Suite-2 establish binding item from DOC-REV-001 follow-ons.
- **Security invariants introduced/changed:**
  - None; governance-only queue state update.
- **Implications for spec/impl/tests:**
  - Governance-only; no protocol semantics changed.

- **ID:** D-0058
- **Date:** 2026-01-04
- **Status:** Accepted
- **Goal IDs:** G3
- **Decision:** Bind PQ KEM public key identifier and prekey identifier into the Suite-2 establishment transcript/AD, CI-gated.
- **Rationale:** Prevent PQ key substitution or prekey misbinding during establishment; enforce deterministic fail-closed checks.
- **Security invariants introduced/changed:**
  - Establishment rejects missing or mismatched PQ KEM public key / prekey bindings.
- **Alternatives considered:**
  - Rely on pq_init_ss alone (rejected: does not bind the public key or prekey identifier).
- **Implications for spec/impl/tests:**
  - DOC-CAN-003 §6.3/§6.6 updated; Suite-2 establish vectors add PQ binding negatives; actor enforces binding checks.

- **ID:** D-0059
- **Date:** 2026-01-04
- **Status:** Accepted
- **Goal IDs:** G2, G5
- **Decision:** Promote NA-0026 to READY to resume execution after NA-0025 completion.
- **Rationale:** Queue requires a single READY item; NA-0026 is the next priority demo storage lifecycle hardening item from DOC-REV-001 follow-ons.
- **Security invariants introduced/changed:**
  - None; governance-only queue state update.
- **Implications for spec/impl/tests:**
  - Governance-only; no protocol semantics changed.

- **ID:** D-0060
- **Date:** 2026-01-04
- **Status:** Accepted
- **Goal IDs:** G2, G5
- **Decision:** Enforce demo store rotation with best-effort secure deletion and CI-gated permission checks.
- **Rationale:** Reduce exposure of local demo artifacts and make lifecycle behavior explicit and testable.
- **Security invariants introduced/changed:**
  - Demo store files are removed on rotation; permissions enforced at 0700/0600.
- **Alternatives considered:**
  - Leave deletion undefined (rejected: unclear lifecycle, not CI-testable).
- **Implications for spec/impl/tests:**
  - Metadata conformance smoke gates rotation + permissions; docs updated with lifecycle policy.

- **ID:** D-0061
- **Date:** 2026-01-04
- **Status:** Accepted
- **Goal IDs:** G5
- **Decision:** Promote NA-0027 to READY to resume execution after NA-0026 completion.
- **Rationale:** Queue requires a single READY item; NA-0027 is the next priority demo UX identity-verification hardening item from DOC-REV-001 follow-ons.
- **Security invariants introduced/changed:**
  - None; governance-only queue state update.
- **Implications for spec/impl/tests:**
  - Governance-only; no protocol semantics changed.

- **ID:** D-0062
- **Date:** 2026-01-04
- **Status:** Accepted
- **Goal IDs:** G5
- **Decision:** Emit a first-establish identity verification warning in the demo CLI, suppressible only via an explicit flag.
- **Rationale:** Prompt users to verify peer identity on first establish; make the behavior CI-testable and fail-closed by default.
- **Security invariants introduced/changed:**
  - First establish prints an identity verification warning unless `--demo-identity-verified` is provided.
- **Alternatives considered:**
  - No warning (rejected: silent risk).
  - Always warn even with explicit override (rejected: blocks deterministic suppression in CI).
- **Implications for spec/impl/tests:**
  - Demo CLI UX only; metadata-conformance-smoke gates the warning behavior.

- **ID:** D-0063
- **Date:** 2026-01-04
- **Status:** Accepted
- **Goal IDs:** G5
- **Decision:** Promote NA-0028 to READY to resume execution after NA-0027 completion.
- **Rationale:** Queue requires a single READY item; NA-0028 is the next priority demo relay quota hardening item from DOC-REV-001 follow-ons.
- **Security invariants introduced/changed:**
  - None; governance-only queue state update.
- **Implications for spec/impl/tests:**
  - Governance-only; no protocol semantics changed.

- **ID:** D-0064
- **Date:** 2026-01-04
- **Status:** Accepted
- **Goal IDs:** G5
- **Decision:** Enforce per-token send quotas in the demo relay with deterministic 429 rejection and CI gating.
- **Rationale:** Reduce probing/abuse beyond queue caps while keeping behavior deterministic and testable.
- **Security invariants introduced/changed:**
  - Over-quota /send requests reject with 429 and a stable error string.
- **Alternatives considered:**
  - Rely on global queue caps only (rejected: insufficient per-token abuse control).
- **Implications for spec/impl/tests:**
  - Demo-only relay behavior; metadata-conformance-smoke enforces per-token quota rejection.

- **ID:** D-0065
- **Date:** 2026-01-05
- **Status:** Accepted
- **Goal IDs:** G5
- **Decision:** Freeze NA promotion after NA-0028; public-release prep mode active; no new READY promotions until public repo cut.
- **Rationale:** Preserve a clean baseline for public-release preparation and avoid scope creep while scrubbing for release.
- **Security invariants introduced/changed:**
  - None; governance-only queue freeze.
- **Alternatives considered:**
  - Continue NA promotions without a freeze (rejected: risks uncontrolled scope during release prep).
- **Implications for spec/impl/tests:**
  - Governance-only; no protocol semantics changed.

- **ID:** D-0066
- **Date:** 2026-01-05
- **Status:** Accepted
- **Goal IDs:** G2, G4
- **Decision:** Apply audit-critical hardening in refimpl stdcrypto Ed25519 and ratchet skipped-key handling (fail-closed, no panics, overflow guard).
- **Rationale:** The 2026-01-04 audit flagged critical fail-open/panic paths and overflow risks; hardening is required before public release.
- **Security invariants introduced/changed:**
  - Ed25519 verify rejects invalid pubkey lengths; sign returns empty output on invalid privkey length.
  - Skipped-key derivation loop rejects on u32 overflow (deterministic error).
- **Alternatives considered:**
  - Leave behavior as-is (rejected: fail-open/panic risk).
- **Implications for spec/impl/tests:**
  - Refimpl-only hardening with new unit tests; no wire semantics changed.

- **ID:** D-0067
- **Date:** 2026-01-07
- **Status:** Accepted
- **Goal IDs:** G5
- **Decision:** Establish a canonical public-release runbook with a single active directive and ordered phases.
- **Rationale:** Prevent scope drift and ensure scrub/cutover work is sequenced and auditable.
- **Security invariants introduced/changed:**
  - None; governance-only ordering and workflow discipline.
- **Alternatives considered:**
  - Ad-hoc sequencing (rejected: ambiguous ordering and overlapping directives).
- **Implications for spec/impl/tests:**
  - Governance-only; references runbook, plan, and test plan.

- **ID:** D-0068
- **Date:** 2026-01-07
- **Status:** Accepted
- **Goal IDs:** G5
- **Decision:** Establish allowlist/denylist inventory as authoritative for the public scrub export set.
- **Rationale:** Prevent accidental publication by enforcing allowlist-based scrub discipline.
- **Security invariants introduced/changed:**
  - None; governance-only inventory control.
- **Alternatives considered:**
  - Ad-hoc export decisions (rejected: inconsistent and unauditable).
- **Implications for spec/impl/tests:**
  - Governance-only; references inventory and test plan.

- **ID:** D-0069
- **Date:** 2026-01-07
- **Status:** Accepted
- **Goal IDs:** G5
- **Decision:** Establish an authoritative public export manifest derived from the allowlist.
- **Rationale:** Provide a deterministic, auditable export set for the public repo cutover.
- **Security invariants introduced/changed:**
  - None; governance-only manifest.
- **Alternatives considered:**
  - Recompute export ad hoc during cutover (rejected: non-deterministic and error-prone).
- **Implications for spec/impl/tests:**
  - Governance-only; references export manifest and test plan.

- **ID:** D-0070
- **Date:** 2026-01-07
- **Status:** Accepted
- **Goal IDs:** G5
- **Decision:** Establish a public repo baseline (README, LICENSE, SECURITY, CONTRIBUTING, notices) and include these files in the public export set.
- **Rationale:** Provide a professional, auditable baseline for public release without leaking private details.
- **Security invariants introduced/changed:**
  - None; governance-only baseline artifacts and allowlist expansion.
- **Alternatives considered:**
  - Defer baseline files until after cutover (rejected: incomplete public posture).
- **Implications for spec/impl/tests:**
  - Governance-only; references baseline docs, allowlist inventory, export manifest, and test plan.

- **ID:** D-0071
- **Date:** 2026-01-07
- **Status:** Accepted
- **Goal IDs:** G5
- **Decision:** Polish public-facing README/SECURITY/CONTRIBUTING guidance for professional release posture.
- **Rationale:** Improve clarity and disclosure expectations while keeping public-release scope constrained.
- **Security invariants introduced/changed:**
  - None; documentation-only updates.
- **Alternatives considered:**
  - Defer polish until after public cutover (rejected: baseline should be present before export).
- **Implications for spec/impl/tests:**
  - Governance-only; references repo-root docs and test plan.

- **ID:** D-0072
- **Date:** 2026-01-07
- **Status:** Accepted
- **Goal IDs:** G5
- **Decision:** Include community health files and templates in the public export allowlist to keep public cuts deterministic.
- **Rationale:** Ensure public repo surface area is reproducible and aligned with org-level community standards.
- **Security invariants introduced/changed:**
  - None; documentation-only export coverage.
- **Alternatives considered:**
  - Add templates manually post-cut (rejected: breaks determinism).
- **Implications for spec/impl/tests:**
  - Governance-only; references allowlist inventory, export manifest, and runbook.

- **ID:** D-0052
- **Date:** 2026-01-04
- **Status:** Accepted
- **Goal IDs:** G5
- **Decision:** Implement deterministic demo relay identifier collision handling and format validation for /register, CI-gated.
- **Rationale:** Prevent ambiguous relay identity reuse and fail-closed on malformed identifiers; keep abuse resistance explicit and testable.
- **Security invariants introduced/changed:**
  - Invalid relay identifiers reject with 400.
  - Duplicate /register attempts reject with 409 without overwriting existing bundles.
- **Alternatives considered:**
  - Allow duplicate overwrites (rejected: enables relay misbinding).
  - Client-only validation (rejected: bypassable).
- **Implications for spec/impl/tests:**
  - Demo-only relay behavior; metadata-conformance-smoke enforces format + collision rejects.

- **ID:** D-0053
- **Date:** 2026-01-04
- **Status:** Accepted
- **Goal IDs:** G2, G4
- **Decision:** Promote NA-0023 to READY to resume execution after NA-0022 completion.
- **Rationale:** Queue requires a single READY item; NA-0023 is the next priority skipped-key eviction/deletion hardening item.
- **Security invariants introduced/changed:**
  - None; governance-only queue state update.
- **Alternatives considered:**
  - Leave NA-0023 in BACKLOG (rejected: stalls execution).
- **Implications for spec/impl/tests:**
  - Governance-only; no protocol semantics changed.
- **ID:** D-0044
- **Date:** 2026-01-04
- **Status:** Accepted
- **Goal IDs:** G3, G5
- **Decision:** Demo relay uses an explicit post-establish bundle consumption endpoint and CI gates at-most-once consumption.
- **Rationale:** Prevent bundle reuse while avoiding consumption on failed establish attempts; enforce the invariant via CI.
- **Security invariants introduced/changed:**
  - Demo bundles are consumed only after successful establish.
  - Reuse attempts reject deterministically without state mutation.
- **Alternatives considered:**
  - Consume bundle on initial fetch (rejected: would burn bundles on failed establish).
- **Implications for spec/impl/tests:**
  - Demo-only relay/CLI changes; metadata-conformance-smoke enforces one-time consumption.

- **ID:** D-0045
- **Date:** 2026-01-04
- **Status:** Accepted
- **Goal IDs:** G3, G5
- **Decision:** Promote NA-0019 to READY to resume execution after NA-0018 completion.
- **Rationale:** Queue requires a single READY item; NA-0019 is the next highest-priority demo establish hardening item from DOC-REV-001 follow-ons.
- **Security invariants introduced/changed:**
  - None; governance-only queue state update.
- **Implications for spec/impl/tests:**
  - Governance-only; no protocol semantics changed.

- **ID:** D-0046
- **Date:** 2026-01-04
- **Status:** Accepted
- **Goal IDs:** G3, G5
- **Decision:** Enforce explicit identity binding for demo establish (bundle.id == peer_id), CI-gated.
- **Rationale:** Prevent ambiguous pairing or relay misbinding at the demo layer; fail-closed on missing/mismatched binding.
- **Security invariants introduced/changed:**
  - Establish rejects missing/mismatched bundle.id binding deterministically.
  - Demo unauthenticated override does not bypass identity binding.
- **Alternatives considered:**
  - Accept relay-keyed binding only (rejected: relay can misbind).
- **Implications for spec/impl/tests:**
  - Demo-only CLI/relay behavior; metadata-conformance-smoke enforces identity binding.

- **ID:** D-0047
- **Date:** 2026-01-04
- **Status:** Accepted
- **Goal IDs:** G3, G4
- **Decision:** Promote NA-0020 to READY to resume execution after NA-0019 completion.
- **Rationale:** Queue requires a single READY item; NA-0020 is next priority demo establish hardening item from DOC-REV-001 follow-ons.
- **Security invariants introduced/changed:**
  - None; governance-only queue state update.
- **Implications for spec/impl/tests:**
  - Governance-only; no protocol semantics changed.

- **ID:** D-0048
- **Date:** 2026-01-04
- **Status:** Accepted
- **Goal IDs:** G3, G4
- **Decision:** Demo relay enforces establish replay rejection using fingerprinted establish_record; CI-gated.
- **Rationale:** Prevent replayed establish from being accepted; enforce deterministic fail-closed behavior.
- **Security invariants introduced/changed:**
  - Establish replays reject without state mutation.
  - Ordering: record -> consume -> persist.
- **Alternatives considered:**
  - Rely on client-only replay detection (rejected: bypassable).
- **Implications for spec/impl/tests:**
  - Demo-only relay/CLI behavior; metadata-conformance-smoke enforces replay rejection.

- **ID:** D-0049
- **Date:** 2026-01-04
- **Status:** Accepted
- **Goal IDs:** G5
- **Decision:** Promote NA-0021 to READY to resume execution after NA-0020 completion.
- **Rationale:** Queue requires a single READY item; NA-0021 is next priority demo relay abuse-resistance item.
- **Security invariants introduced/changed:**
  - None; governance-only queue state update.
- **Implications for spec/impl/tests:**
  - Governance-only; no protocol semantics changed.

- **ID:** D-0050
- **Date:** 2026-01-04
- **Status:** Accepted
- **Goal IDs:** G5
- **Decision:** Implement deterministic demo relay rate limiting/backoff for register/poll with CI gate.
- **Rationale:** Reduce abuse/probing; enforce fail-closed behavior above thresholds without time-based flakiness.
- **Security invariants introduced/changed:**
  - Authenticated register/poll requests beyond threshold return 429 deterministically.
- **Alternatives considered:**
  - Time-window rate limiting (rejected: flaky in CI).
  - Client-only throttling (rejected: bypassable).
- **Implications for spec/impl/tests:**
  - Demo-only relay behavior; metadata-conformance-smoke enforces 429 thresholds.

- **ID:** D-0051
- **Date:** 2026-01-04
- **Status:** Accepted
- **Goal IDs:** G5
- **Decision:** Promote NA-0022 to READY to resume execution after NA-0021 completion.
- **Rationale:** Queue requires a single READY item; NA-0022 is next priority demo relay hardening item from DOC-REV-001 follow-ons.
- **Security invariants introduced/changed:**
  - None; governance-only queue state update.
- **Implications for spec/impl/tests:**
  - Governance-only; no protocol semantics changed.

- **ID:** D-0073
- **Date:** 2026-01-08
- **Status:** Accepted
- **Goal IDs:** G4, G5
- **Decision:** Public export includes docs index and minimal public CI workflow; allowlist remains explicit with a narrow scan exclusion for the workflow file.
- **Rationale:** Preserve deterministic public cuts while enabling auditability and navigation; avoid broad workflow inclusion.
- **Security invariants introduced/changed:**
  - Allowlist remains explicit; high-confidence scans exclude only .github/workflows/public-ci.yml due to intentional regex patterns.
- **Alternatives considered:**
  - Wildcard include of .github/workflows (rejected: too broad).
- **Implications for spec/impl/tests:**
  - Public export allowlist updated; docs and test plan updated with explicit exclusion note.

- **Decision (2026-01-08):** Public export allowlist explicitly includes `.github/CODEOWNERS` to preserve determinism with the public repo. Scope is governance-only; no protocol semantics changed.

- **Decision (2026-01-08):** Public export includes contributor checklists (docs/CHECKLIST_DOCS_PR.md, docs/CHECKLIST_RELEASE.md) and the updated PR template links to preserve public-facing hygiene.

- **Decision (2026-01-09):** Update public README and docs index to reflect the v0.2.0 public development cutover (documentation accuracy; no protocol semantics changed).

- **Decision (2026-01-09):** Update public README and docs index to reflect the v0.2.0 public development cutover (documentation accuracy; no protocol semantics changed).

- **ID:** D-0074
- **Date:** 2026-01-09
- **Status:** Accepted
- **Goal IDs:** G1, G2, G3, G4, G5
- **Decision:** Make `qsl-protocol` the primary development repo by importing the full tracked tree from the private source-of-truth snapshot (276c4dd).
- **Rationale:** Public development improves transparency and aligns CI with the definitive protocol artifacts.
- **Security invariants introduced/changed:**
  - No secrets included (denylist filename scan + high-confidence credential scan pass).
- **Implications for spec/impl/tests:**
  - Full protocol implementation, harness, and CI workflows are now public and run in this repo.

- **ID:** D-0075
- **Date:** 2026-01-09
- **Status:** Accepted
- **Goal IDs:** G4, G5
- **Decision:** Import the 2026-01-04 code analysis report and track CRITICAL #1–#3 with CI regression guards.
- **Rationale:** Make audit findings traceable and prevent regression of critical fail-closed invariants.
- **Security invariants introduced/changed:**
  - Ed25519 sign/verify fail-closed on invalid key lengths.
  - Ratchet skip-loop overflow rejects deterministically.
- **Implications for spec/impl/tests:**
  - Audit report and status table added under docs/audit; regression guards test plan added under tests/.

- **ID:** D-0076
- **Date:** 2026-01-09
- **Status:** Accepted
- **Goal IDs:** G4, G5
- **Decision:** Audit CRITICAL #1–#3 verification/closure tracked via status table update and regression guards; no protocol semantics change.
- **Rationale:** Make audit traceability explicit and prevent regression of critical fail-closed invariants.
- **Security invariants introduced/changed:**
  - Ed25519 sign/verify fail-closed on invalid key lengths.
  - Ratchet skip-loop overflow rejects deterministically.
- **Implications for spec/impl/tests:**
  - docs/audit/CODE_ANALYSIS_REPORT_20260104.md
  - docs/audit/AUDIT_CODE_ANALYSIS_STATUS_20260104.md
  - tests/AUDIT-20260104_regression_guards_testplan.md

- **ID:** D-0077
- **Date:** 2026-01-09
- **Status:** Accepted
- **Goal IDs:** G4, G5
- **Decision:** Fix Issue #8 by making HandshakeInit encode fail-closed when OPK fields are missing; add regression guard test.
- **Rationale:** Prevent panic on malformed internal state and keep audit hardening deterministic.
- **Security invariants introduced/changed:**
  - HandshakeInit encode returns fail-closed output when opk_used is true but OPK fields are missing.
- **Implications for spec/impl/tests:**
  - tools/refimpl/quantumshield_refimpl/src/qsp/types.rs
  - tests/AUDIT-20260104_issue8_opk_invariant_testplan.md

- **ID:** D-0078
- **Date:** 2026-01-09
- **Status:** Accepted
- **Goal IDs:** G4, G5
- **Decision:** Remove AEAD expect/unwrap panics in StdCrypto seal; propagate deterministic failure and add regression guards (Issue #5).
- **Rationale:** Ensure AEAD failures are fail-closed and cannot panic on malformed inputs.
- **Security invariants introduced/changed:**
  - AEAD seal returns fail-closed output on error; callers reject empty ciphertext.
- **Implications for spec/impl/tests:**
  - tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs
  - tools/refimpl/quantumshield_refimpl/src/qsp/ratchet.rs
  - tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs
  - tests/AUDIT-20260104_issue5_aead_no_panic_testplan.md

- **ID:** D-0079
- **Date:** 2026-01-09
- **Status:** Accepted
- **Goal IDs:** G4, G5
- **Decision:** Replace crypto-critical `thread_rng` usage with OS RNG (OsRng) in StdCrypto and add regression guards (Issue #4).
- **Rationale:** Ensure secret material is sourced from OS-backed entropy and avoid weaker RNG initialization paths.
- **Security invariants introduced/changed:**
  - StdCrypto keypair and nonce generation use OsRng.
- **Implications for spec/impl/tests:**
  - tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs
  - tests/AUDIT-20260104_issue4_rng_osrng_testplan.md

- **ID:** D-0080
- **Date:** 2026-01-10
- **Status:** Accepted
- **Goal IDs:** G4, G5
- **Decision:** Make `ratchet_encrypt` fail-closed by committing state only after successful encryption; add a regression guard for deterministic reject + no mutation (Audit Issue #7).
- **PR:** PR #23
- **Rationale:** Prevent state mutation on failed sends and make rejection deterministic for invalid encryption output.
- **Security invariants introduced/changed:**
  - Failed sends do not mutate ratchet state.
  - Rejects are deterministic for invalid AEAD output.
- **Implications for spec/impl/tests:**
  - tools/refimpl/quantumshield_refimpl/src/qsp/ratchet.rs
  - tests/AUDIT-20260104_issue7_send_state_no_mutation_testplan.md

- **ID:** D-0081
- **Date:** 2026-01-10
- **Status:** Accepted
- **Goal IDs:** G4, G5
- **Decision:** Zeroize secret-bearing key material on drop/overwrite and add regression guards (Audit Issue #9).
- **PR:** PR #25
- **Rationale:** Prevent residual secret key material from persisting in memory and make zeroization requirements regression-proof.
- **Security invariants introduced/changed:**
  - X25519 private key material zeroizes on drop and via explicit zeroize calls.
- **Implications for spec/impl/tests:**
  - tools/refimpl/quantumshield_refimpl/src/crypto/traits.rs
  - tests/AUDIT-20260104_issue9_zeroization_testplan.md

- **ID:** D-0082
- **Date:** 2026-01-10
- **Status:** Accepted
- **Goal IDs:** G4, G5
- **Decision:** Update Suite-2 boundary receive to advance `ck_pq_recv` from the post-reseed chain step and add regression guards (Audit Issue #6).
- **PR:** PR #28
- **Rationale:** Prevent PQ chain divergence at boundary and ensure fail-closed behavior is regression-tested.
- **Security invariants introduced/changed:**
  - Boundary processing advances `ck_pq_recv` deterministically on success and does not mutate state on reject.
- **Implications for spec/impl/tests:**
  - tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs
  - tests/AUDIT-20260104_issue6_ck_pq_recv_boundary_testplan.md

- **ID:** D-0083
- **Date:** 2026-01-10
- **Status:** Accepted
- **Goal IDs:** G4, G5
- **Decision:** Remove early-return timing leak in header decryption candidate trials and add regression guards (Audit Issue #10).
- **PR:** PR #30
- **Rationale:** Header-decrypt candidate loops must not short-circuit based on success position; deterministic reject behavior with no state mutation is required for audit closure.
- **Security invariants introduced/changed:**
  - Candidate-key trials are attempted in bounded order without early return on success.
  - Reject paths are deterministic and do not mutate state.
- **Implications for spec/impl/tests:**
  - tools/refimpl/quantumshield_refimpl/src/qsp/ratchet.rs
  - tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs
  - tests/AUDIT-20260104_issue10_header_timing_sidechannel_testplan.md

- **ID:** D-0084
- **Date:** 2026-01-11
- **Status:** Accepted
- **Goal IDs:** G4, G5
- **Decision:** Keep mk_skipped and mk_order consistent on take_mk_skipped; add regression guards (Audit Issue #12).
- **PR:** PR #32
- **Rationale:** Prevent stale mk_order entries that reference missing skipped keys and make the invariant regression-proof.
- **Security invariants introduced/changed:**
  - mk_order never retains an entry for a key absent from mk_skipped after take_mk_skipped.
- **Implications for spec/impl/tests:**
  - tools/refimpl/quantumshield_refimpl/src/qsp/state.rs
  - tests/AUDIT-20260104_issue12_mk_order_stale_testplan.md

- **ID:** D-0085
- **Date:** 2026-01-11
- **Status:** Accepted
- **Goal IDs:** G4, G5
- **Decision:** Enforce SCKA monotonicity fail-closed and add regression guards (Audit Issue #13).
- **PR:** PR #34
- **Rationale:** Prevent acceptance of stale or non-monotonic peer advance IDs and make the invariant regression-proof.
- **Security invariants introduced/changed:**
  - Non-monotonic peer advance IDs reject deterministically without mutating state.
- **Implications for spec/impl/tests:**
  - tools/refimpl/quantumshield_refimpl/src/suite2/scka.rs
  - tests/AUDIT-20260104_issue13_scka_monotonicity_testplan.md

- **ID:** D-0086
- **Date:** 2026-01-11
- **Status:** Accepted
- **Goal IDs:** G4, G5
- **Decision:** Make store_mk_skipped fail-closed and add regression guards (Audit Issue #14).
- **PR:** PR #36
- **Rationale:** Eliminate silent failure paths by requiring deterministic rejection with no partial state mutation when skipped-key storage cannot be guaranteed.
- **Security invariants introduced/changed:**
  - store_mk_skipped rejects deterministically on failure and leaves mk_skipped + mk_order unchanged.
- **Implications for spec/impl/tests:**
  - tools/refimpl/quantumshield_refimpl/src/qsp/state.rs
  - tests/AUDIT-20260104_issue14_store_mk_skipped_silent_failure_testplan.md

- **ID:** D-0087
- **Date:** 2026-01-11
- **Status:** Accepted
- **Goal IDs:** G4, G5
- **Decision:** Make DH ratchet fail-closed on ns overflow and add regression guards (Audit Issue #15).
- **PR:** PR #38
- **Rationale:** Prevent pn corruption and peer desynchronization if the send counter reaches its maximum before a boundary.
- **Security invariants introduced/changed:**
  - DH ratchet rejects deterministically when ns overflows and does not mutate state.
  - pn derives from ns only after overflow checks succeed.
- **Implications for spec/impl/tests:**
  - tools/refimpl/quantumshield_refimpl/src/qsp/ratchet.rs
  - tests/AUDIT-20260104_issue15_pn_ns_overflow_testplan.md

- **ID:** D-0088
- **Date:** 2026-01-11
- **Status:** Accepted
- **Goal IDs:** G4, G5
- **Decision:** Bound Suite-2 restore_bytes collection lengths and add regression guards (Audit Issue #16).
- **PR:** PR #40
- **Rationale:** Prevent DoS via oversized snapshot collection lengths by enforcing deterministic, fail-closed bounds checks.
- **Security invariants introduced/changed:**
  - restore_bytes rejects oversized or truncated collections deterministically and does not mutate pre-existing state.
- **Implications for spec/impl/tests:**
  - tools/refimpl/quantumshield_refimpl/src/suite2/state.rs
  - tests/AUDIT-20260104_issue16_deser_dos_bounds_testplan.md

- **ID:** D-0089
- **Date:** 2026-01-11
- **Status:** Accepted
- **Goal IDs:** G4, G5
- **Decision:** Remove header_pt unwrap panic paths and add regression guards (Audit Issue #17).
- **PR:** PR #42
- **Rationale:** Ensure malformed header_pt inputs reject deterministically without panics or state mutation.
- **Security invariants introduced/changed:**
  - header_pt handling rejects deterministically and preserves session state on reject.
- **Implications for spec/impl/tests:**
  - tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs
  - tests/AUDIT-20260104_issue17_header_pt_unwraps_testplan.md

- **ID:** D-0090
- **Date:** 2026-01-11
- **Status:** Accepted
- **Goal IDs:** G4, G5
- **Decision:** Remove OPK unwrap panic paths in initiator_start; enforce deterministic reject + regression guards (Audit Issue #18).
- **PR:** PR #44
- **Rationale:** Ensure missing OPK fields reject deterministically without panics or input mutation.
- **Security invariants introduced/changed:**
  - OPK handling rejects deterministically and does not mutate inputs on reject.
- **Implications for spec/impl/tests:**
  - tools/refimpl/quantumshield_refimpl/src/qsp/handshake.rs
  - tests/AUDIT-20260104_issue18_opk_unwraps_testplan.md

- **ID:** D-0091
- **Date:** 2026-01-11
- **Status:** Accepted
- **Goal IDs:** G4, G5
- **Decision:** Eliminate SessionState cloning in ratchet paths to reduce secret duplication; preserve fail-closed + no-mutation-on-reject; add guards (Audit Issue #19).
- **PR:** PR #46
- **Rationale:** Prevent avoidable duplication of key material while retaining deterministic reject behavior and state immutability on failure.
- **Security invariants introduced/changed:**
  - Ratchet paths do not clone SessionState and do not mutate state on reject.
- **Implications for spec/impl/tests:**
  - tools/refimpl/quantumshield_refimpl/src/qsp/ratchet.rs
  - tests/AUDIT-20260104_issue19_state_clone_key_material_testplan.md

- **ID:** D-0092
- **Date:** 2026-01-11
- **Status:** Accepted
- **Goal IDs:** G4, G5
- **Decision:** Remove mutex lock unwrap panic paths in CLI relay; deterministic reject + regression guards (Audit Issue #20).
- **PR:** PR #69
- **Rationale:** Ensure poisoned relay state locks fail closed without panics or state mutation.
- **Security invariants introduced/changed:**
  - Relay state lock poisoning rejects deterministically and preserves relay state.
- **Implications for spec/impl/tests:**
  - apps/qshield-cli/src/commands/relay.rs
  - tests/AUDIT-20260104_issue20_cli_mutex_poison_testplan.md

- **ID:** D-0093
- **Date:** 2026-01-11
- **Status:** Accepted
- **Goal IDs:** G2, G3
- **Decision:** Suite-2 MKSKIPPED entries are consumed only after successful body decrypt; reject must not mutate state (Audit Issue #21).
- **PR:** PR #50
- **Rationale:** Preserve fail-closed behavior and state integrity on decrypt/auth failure; avoid state loss from transient failures.
- **Security invariants introduced/changed:**
  - MKSKIPPED entries are not deleted on decrypt/auth failure; reject is deterministic and state is unchanged.
- **Alternatives considered:**
  - Keep delete-on-fail behavior from D-0016 (rejected: violates no-mutation-on-reject invariant for Issue #21).
- **Implications for spec/impl/tests:**
  - Spec: docs/canonical/DOC-CAN-003_QSP_Suite-2_True_Triple_Ratchet_v5.0.0_DRAFT.md
  - Impl: tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs
  - Tests: tests/AUDIT-20260104_issue21_mkskipped_removal_testplan.md

- **ID:** D-0094
- **Date:** 2026-01-12
- **Status:** Accepted
- **Goal IDs:** G2, G3
- **Decision:** Boundary receive attempts header auth only for cand=st.nr to avoid boundary window waste; deterministic rejects preserved; no mutation on reject (Audit Issue #22).
- **PR:** PR #52
- **Rationale:** Prevent resource exhaustion from unnecessary header attempts while preserving existing boundary semantics and fail-closed behavior.
- **Security invariants introduced/changed:**
  - Boundary receive uses a single header candidate (st.nr) and rejects deterministically without mutating state on failure.
- **Implications for spec/impl/tests:**
  - tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs
  - tests/AUDIT-20260104_issue22_boundary_window_testplan.md

- **ID:** D-0095
- **Date:** 2026-01-12
- **Status:** Accepted
- **Goal IDs:** G1, G2, G3
- **Decision:** Mix ss3 into handshake key schedule on both sides to avoid entropy discard and enforce binding (Audit Issue #23).
- **PR:** PR #54
- **Rationale:** Preserve intended PQ entropy contribution; ensure handshake outputs are bound to ss3 while remaining deterministic.
- **Security invariants introduced/changed:**
  - ss3 is mixed into rk0 derivation; decap failures reject without mutating session state.
- **Implications for spec/impl/tests:**
  - tools/refimpl/quantumshield_refimpl/src/qsp/handshake.rs
  - tests/AUDIT-20260104_issue23_ss3_entropy_testplan.md

- **ID:** D-0096
- **Date:** 2026-01-12
- **Status:** Accepted
- **Goal IDs:** G2, G3
- **Decision:** Treat ZERO32 as a sentinel only; reject before consuming unset chain keys in Suite-2 (Audit Issue #24).
- **PR:** PR #57
- **Rationale:** Prevent cryptographic use of placeholder chain keys; preserve deterministic reject and no-mutation-on-reject invariants.
- **Security invariants introduced/changed:**
  - Unset (all-zero) chain keys are rejected before any crypto use; reject is deterministic and state is unchanged.
- **Implications for spec/impl/tests:**
  - tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs
  - tests/AUDIT-20260104_issue24_zero32_testplan.md


- **ID:** D-0097
- **Date:** 2026-01-14
- **Status:** Accepted
- **Goal IDs:** G2, G3
- **Decision:** Canonicalize refimpl boundary errors via RefimplError so Suite-2 string rejects and QSP typed errors are composable and deterministic (Audit Issue #25).
- **PR:** PR #69
- **Rationale:** Ensure all user-visible rejects include a stable reason_code token without changing wire semantics.
- **Security invariants introduced/changed:**
  - Boundary errors always include `reason_code=<CODE>`.
  - Reject paths remain deterministic and do not mutate state.
- **Implications for spec/impl/tests:**
  - tools/refimpl/quantumshield_refimpl/src/refimpl_error.rs
  - tools/refimpl/quantumshield_refimpl/src/suite2/mod.rs
  - tools/refimpl/quantumshield_refimpl/src/qsp/mod.rs
  - tests/AUDIT-20260104_issue25_error_types_testplan.md


- **ID:** D-0098
- **Date:** 2026-01-14
- **Status:** Accepted
- **Goal IDs:** G2, G3
- **Decision:** Treat asymmetric ZERO32 chainkeys from Suite-2 establish as sentinel values; consumption must reject deterministically and preserve state (Audit Issue #26).
- **PR:** PR #69
- **Rationale:** Establishment can legitimately leave one direction unset; safety is enforced at the first consumer with fail-closed checks.
- **Security invariants introduced/changed:**
  - Unset chainkeys are never consumed for crypto; reject includes reason_code and does not mutate state.
- **Implications for spec/impl/tests:**
  - tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs
  - tests/AUDIT-20260104_issue26_asymmetric_initial_state_testplan.md


- **ID:** D-0099
- **Date:** 2026-01-17
- **Status:** Accepted
- **Goal IDs:** G2, G3
- **Decision:** Preserve deterministic reject metadata for Suite-2 CHAINKEY_UNSET by including a reason_code token.
- **PR:** PR #64
- **Rationale:** Ensure reject strings remain machine-parseable and stable for tests/interop without changing wire semantics.
- **Security invariants introduced/changed:**
  - CHAINKEY_UNSET rejects include `reason_code=REJECT_S2_CHAINKEY_UNSET`.
  - No state transition changes; metadata-only output.
- **Implications for spec/impl/tests:**
  - tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs

- **ID:** D-0100
- **Date:** 2026-01-17
- **Status:** Accepted
- **Goal IDs:** G2, G3
- **Decision:** Add fail-fast structural guards in QSP handshake before signature verification (Audit Issue #27).
- **PR:** PR #69
- **Rationale:** Reject malformed inputs before expensive signature verification; preserve deterministic fail-closed behavior.
- **Security invariants introduced/changed:**
  - HS1/HS2 protocol_version/suite_id/signature lengths are validated before verify().
  - Malformed inputs reject deterministically without invoking signature verification.
- **Implications for spec/impl/tests:**
  - tools/refimpl/quantumshield_refimpl/src/qsp/handshake.rs
  - tests/AUDIT-20260104_issue27_sig_verify_order_testplan.md

- **ID:** D-0101
- **Date:** 2026-01-17
- **Status:** Accepted
- **Goal IDs:** G2, G3
- **Decision:** Remove attacker-triggerable expect()/unwrap() panics in QSP ProtocolMessage encode by failing closed with deterministic empty encodes for missing PQ fields (Audit Issue #28).
- **PR:** PR #67
- **Rationale:** Attacker-controlled decode/encode surfaces must not panic; deterministic failures preserve testability and fail-closed semantics without wire changes.
- **Security invariants introduced/changed:**
  - Missing PQ optional fields in ProtocolMessage encode do not panic and return deterministic empty output.
- **Implications for spec/impl/tests:**
  - tools/refimpl/quantumshield_refimpl/src/qsp/types.rs
  - tests/AUDIT-20260104_issue28_safe_unwraps_testplan.md

- **ID:** D-0102
- **Date:** 2026-01-18
- **Status:** Accepted
- **Goal IDs:** G2, G3
- **Decision:** CodeQL hard-coded crypto value cleanup (Rust).
- **PR:** PR #69
- **Rationale:** Treat `rust/hard-coded-cryptographic-value` as a regression gate by removing hard-coded key-like values from Rust helpers/tests while preserving protocol behavior and wire semantics.
- **Security invariants introduced/changed:**
  - No hard-coded key-like values in Rust helper/test code paths flagged by CodeQL.
  - Sentinels remain guarded and never consumed by crypto.
- **Implications for spec/impl/tests:**
  - tools/actors/refimpl_actor_rs/src/main.rs
  - tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs
  - tools/refimpl/quantumshield_refimpl/src/qsp/ratchet.rs
  - tools/refimpl/quantumshield_refimpl/src/qsp/state.rs
  - tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs
  - tests/CODEQL_hardcoded_crypto_value_cleanup_testplan.md

- **ID:** D-0103
- **Date:** 2026-01-18
- **Status:** Accepted
- **Goal IDs:** G2, G3
- **Decision:** Codify CodeQL as a continuous security regression gate with a fast local targeted query and CI as the authoritative gate.
- **PR:** PR #70
- **Rationale:** Local full-suite CodeQL can be slow; a fast targeted check catches regressions early while CI remains the source of truth.
- **Security invariants introduced/changed:**
  - Local CodeQL outputs must not dirty the repo; store under _forensics/ and use local-only excludes if desired.
  - Triage distinguishes real bugs vs guarded sentinels/test helpers with explicit evidence.
- **Implications for spec/impl/tests:**
  - START_HERE.md
  - docs/dev/DOC-DEV-002_CodeQL_Operating_Procedure_v1.0.0_DRAFT.md
  - tools/goal_lint.py

- **ID:** D-0104
- **Date:** 2026-01-18
- **Status:** Accepted
- **Goal IDs:** G2, G3
- **Decision:** Sequence execution roadmap: evidence gates → dumb relay → Linux TUI demo.
- **PR:** PR #72
- **Rationale:** Keep protocol work stable and audited before demo surfaces; prevent relay/UI work from driving protocol changes.
- **Security invariants introduced/changed:**
  - Fail-closed behavior remains mandatory.
  - Deterministic rejects and no-mutation-on-reject remain mandatory for stateful operations.
  - No secret logging in relay/UI demo paths.
- **Implications for spec/impl/tests:**
  - START_HERE.md
  - NEXT_ACTIONS.md
  - docs/test/DOC-TST-RELAY-TUI_Demo_Test_Plan_v0.1.0_DRAFT.md

- **ID:** D-0105
- **Date:** 2026-01-18
- **Status:** Accepted
- **Goal IDs:** G2, G3
- **Decision:** NA-0050: add a harness/actors relay HTTP transport adapter (opaque bytes only; no protocol changes).
- **PR:** PR #74
- **Rationale:** Enable real-world demo transport via an external relay while keeping protocol semantics unchanged and CI offline-safe.
- **Security invariants introduced/changed:**
  - Relay transport is opaque; no protocol parsing or crypto in the harness adapter.
  - Remote relay use is opt-in; defaults stay local/offline-safe.
  - Deterministic transport errors; no payload logging.
- **Implications for spec/impl/tests:**
  - tests/harness/4b/runner.py
  - tests/harness/4b/lib/relay_http.py
  - tests/harness/4b/tests/test_relay_http_adapter.py
  - docs/test/DOC-TST-RELAY-TUI_Demo_Test_Plan_v0.1.0_DRAFT.md

- **ID:** D-0106
- **Date:** 2026-01-18
- **Status:** Accepted
- **Goal IDs:** G2, G3
- **Decision:** CodeQL operational model: fast local targeted check + CI authoritative gate.
- **PR:** PR TBD
- **Rationale:** Local full suites are slow; a targeted local query keeps regressions visible while CI remains authoritative.
- **Security invariants introduced/changed:**
  - Local CodeQL outputs must not dirty the repo; store under _forensics/ and use local-only excludes if desired.
  - Triage distinguishes real bugs vs guarded sentinels/test helpers with explicit evidence.
- **Implications for spec/impl/tests:**
  - START_HERE.md
  - docs/dev/DOC-DEV-002_CodeQL_Operating_Procedure_v1.0.0_DRAFT.md

- **ID:** D-0107
- **Date:** 2026-01-18
- **Status:** Accepted
- **Goal IDs:** G2, G3
- **Decision:** NA-0052: relay_http harness transport semantics for remote relay interop (opt-in remote).
- **PR:** PR #78
- **Rationale:** Enable relay-based interop over an external transport-only relay without protocol changes; keep remote usage explicit and offline-safe by default.
- **Security invariants introduced/changed:**
  - Relay transport remains opaque; no protocol parsing or crypto in the harness adapter.
  - Remote relay usage requires explicit opt-in (QSL_ALLOW_REMOTE=1).
  - Deterministic transport errors; no payload logging.
- **Implications for spec/impl/tests:**
  - tests/harness/4b/lib/relay_http.py
  - tests/harness/4b/runner.py
  - tests/harness/4b/tests/test_relay_http_adapter.py
  - docs/test/DOC-TST-RELAY-TUI_Demo_Test_Plan_v0.1.0_DRAFT.md
  - NEXT_ACTIONS.md

- **ID:** D-0108
- **Date:** 2026-01-19
- **Status:** Accepted
- **Goal IDs:** G2, G3
- **Decision:** NA-0051: Linux TUI demo client is a protocol consumer; relay remains transport-only; remote use is explicit opt-in.
- **PR:** PR TBD
- **Rationale:** Provide a minimal Linux TUI demo without altering protocol-core behavior or wire semantics.
- **Security invariants introduced/changed:**
  - Relay is opaque transport only; no protocol parsing or crypto in the relay.
  - Remote relay usage requires explicit opt-in (QSL_ALLOW_REMOTE=1).
  - No payload logging; only metadata is shown in the UI.
- **Implications for spec/impl/tests:**
  - apps/qsl-tui/**
  - docs/test/DOC-TST-RELAY-TUI_Demo_Test_Plan_v0.1.0_DRAFT.md
  - NEXT_ACTIONS.md

- **ID:** D-0109
- **Date:** 2026-01-19
- **Status:** Accepted
- **Goal IDs:** G2, G3
- **Decision:** Add a headless mode to qsl-tui so demos can run in non-interactive environments without PTY/TTY access.
- **PR:** PR #83
- **Rationale:** Demo evidence requires a deterministic, non-interactive path in restricted shells while preserving the interactive TUI for real terminals.
- **Security invariants introduced/changed:**
  - Headless mode only changes UI initialization/output; protocol behavior is unchanged.
  - Relay mode remains transport-only and opt-in gated by QSL_ALLOW_REMOTE=1.
- **Implications for spec/impl/tests:**
  - apps/qsl-tui/**
  - docs/test/DOC-TST-RELAY-TUI_Demo_Test_Plan_v0.1.0_DRAFT.md

## D-0001 — DEMO-0001 headless golden run evidence captured (local + relay)

Date: 2026-01-19
Goals: G2, G3, G5

Decision:

Record the successful DEMO-0001 headless “golden run” as the baseline reproducible demo proof for the Linux demo client,
including both local execution and relay transport execution.

Evidence pointers (local, outside repo):

- OUT=/home/victor/work/qsl/_forensics/demo0001_headless_resume_20260119T053032Z
- Protocol repo HEAD=be0f97e0f3343f0129004a3ccbeddae2a4c1fd9b
- Relay base URL=http://qsl.ddnsfree.com:8080

Operational note:

Relay mode requires explicit opt-in via QSL_ALLOW_REMOTE=1.


## D-0002 — NA-0053 demo: app-layer size padding to reduce metadata leakage

Date: 2026-01-19
Goals: G2, G3, G4

Decision:

Implement app-layer payload size padding in qsl-tui (inside the encrypted payload) using fixed bucket sizes to reduce ciphertext size
correlation.

Rationale:

- Low-risk: does not modify protocol core or wire format; implemented entirely in demo/client layer.
- Measurable: bucket sizes and overhead are explicit; reduces passive size inference.
- Honest: does not claim full metadata privacy; IP/timing remain visible without additional measures.

Constraints:

- Must preserve deterministic headless demo markers and exit codes.
- Must not regress CI or protocol behavior.
## D-0003 — Documentation entrypoint + docs inventory map (no deletions)

Date: 2026-01-19
Goals: G4

Decision:
- Add a mandatory “Working directory” note to governance docs for operators who start in ~/work/qsl.
- Add docs/DOCS_MAP.md as the first-pass documentation inventory/map.
- No deletions or moves in this change; inventory first.

Evidence:
- docs/DOCS_MAP.md
- Working directory note in START_HERE.md / CHAT_STARTER.md / AGENTS.md / DOC-DEV-003

## D-0004 — NA-0054 metadata visibility demo (client-only)

Date: 2026-01-20
Goals: G2, G3

Decision:
- Add a qsl-tui metadata visibility demo (client-only) that reports plaintext_len vs ciphertext_len, padding bucket, and privacy mode.
- Keep protocol core unchanged; demo behavior is strictly client-layer.

Rationale:
- Makes metadata tradeoffs explicit for public demos without overstating privacy.

Evidence:
- apps/qsl-tui demo output markers (QSL_TUI_META / QSL_TUI_META_NOTE)
- docs/test/DOC-TST-RELAY-TUI_Demo_Test_Plan_v0.1.0_DRAFT.md

## D-0005 — Public demo pack for metadata visibility (NA-0054)

<!-- D-0005 DEMO_PUBLIC_001 -->

Date: 2026-01-20
Goals: G4

Decision:
- Publish a simple demo doc + script that exposes metadata visibility tradeoffs clearly.
- Keep all changes client/demo-only; no protocol/crypto changes.

Evidence:
- docs/demo/DEMO-PUBLIC-001_Metadata_Visibility.md
- scripts/demo/demo_public_metadata_visibility.sh

## D-0006 — Public demo/client v1 (two-party + proxy + metadata outputs)

Date: 2026-01-21
Goals: G4, G2, G3

Decision:
- Ship a two-party public demo client in qsl-tui with explicit sender/receiver roles.
- Add optional relay HTTP proxy support (Tor-friendly) for demo usage only.
- Standardize machine-readable metadata outputs (QSL_TUI_META) for sender and receiver.

Rationale:
- Public can validate content encryption while seeing concrete metadata tradeoffs.
- Proxy support allows demonstrable transport privacy options without changing protocol.

Constraints:
- Protocol/wire/crypto unchanged.
- Demo-only and governance/docs changes in allowlisted paths.

Evidence:
- scripts/demo/* two-party scripts
- docs/test/DOC-TST-RELAY-TUI_Demo_Test_Plan_v0.1.0_DRAFT.md (NA-0056)

<!-- D-0006 NA-0056 -->

## D-0007 — Public Demo Runbook (authoritative execution discipline)

Date: 2026-01-22
Goals: G4, G5

Decision:
- Adopt DOC-DEV-004 as the authoritative runbook for the public demo/client track (qsl-tui + scripts/demo), enforcing:
  - strict scope discipline
  - slow-machine operational constraints
  - bounded CI waiting (no `--watch`)
  - “claims discipline” for metadata minimization statements
  - security-by-default posture guidance
  - privacy envelope framing (tick schedule + size buckets + bundle packing)
  - uniform rejects + no-mutation-on-reject testing expectations
  - logging/metrics privacy budget discipline

Rationale:
- Demo/client work is high-visibility and prone to drift; a single checklist reduces ambiguity and prevents accidental scope creep while encoding enforceable privacy posture.

Evidence:
- docs/dev/DOC-DEV-004_Public_Demo_Runbook_v0.1.0_DRAFT.md
- NEXT_ACTIONS.md NA-0057 entry
## D-0008 — Introduce QSC client scaffold workspace (qsl/qsl-client) (NA-0058 Step 1)

Date: 2026-01-22
Goals: G4, G5

Decision:
- Add a separate client workspace path `qsl/qsl-client/qsc` as the QSC CLI entrypoint scaffold.
- Wire the crate into the root workspace (minimal `Cargo.toml` member addition).
- Emit deterministic, machine-readable markers (`QSC_MARK/1 ...`) as the initial output contract for automation and demos.

Rationale:
- Separates client surface from protocol core while enabling incremental delivery under NA-0058.
- Deterministic markers and tests prevent flaky demos and provide a stable contract for downstream tooling.
- No protocol/wire changes are introduced by this scaffold step.

Alternatives considered:
- Embedding the client under existing `apps/` or overloading existing demo clients. Rejected to keep NA-0058 isolated and auditable.

Invariants:
- Fail-closed behavior and deterministic output; no secrets or timestamps in default outputs for this phase.
- No protocol/wire mutation as part of scaffold-only work.

References:
- NA-0058 (NEXT_ACTIONS.md)
- QSC design spec: docs/design/QSC_CLI_Client_Design_Spec_v0.1_2026-01-22.md

### D-0110 — QSC store safety policy: strict defaults; anchored checks for explicit QSC_CONFIG_DIR; doctor is diagnostic-only
Date: 2026-01-24  
Goals: G4, G5

Context:
- QSC enforces fail-closed local storage hardening (symlink rejection, strict permissions, atomic writes).
- Some environments have group/world-writable parents above workspace paths; rejecting based on those unrelated parents blocks explicit overrides and deterministic tests.
- HOME may be read-only; tooling/tests must not assume writable HOME.

Decision:
- Default HOME-based store remains strict: reject if any relevant existing parent is group/world-writable.
- If QSC_CONFIG_DIR is explicitly set, anchor parent-perms enforcement at the configured store root:
  - Enforce: no symlink traversal in any path component (unsafe_path_symlink wins).
  - Enforce: store root permissions hardening (0700) and config file (0600) on Unix; fail-closed if hardening fails.
  - Do not reject based on unrelated parents above the explicit root.
- `qsc doctor --check-only` is non-fatal: it emits deterministic markers with safety booleans (parent_safe/symlink_safe), rather than erroring on unsafe parents.

Rationale:
- Preserves strict-by-default safety while enabling explicit overrides in controlled environments without weakening path/symlink protections.

### D-0111 — QSC terminal output sanitization + marker discipline + bounded waits (NA-0059 Step 3)
Date: 2026-01-24
Goals: G4, G5

Decision:
- Enforce terminal-safe sanitization for any untrusted text before display (strip ANSI/control).
- Keep QSC_MARK/1 as the only machine-parseable marker line format; markers must never contain untrusted text.
- Ensure any CLI-exposed wait/retry loop is bounded with explicit timeouts (no infinite waits).

Rationale:
- Prevents ANSI/control injection and log-forging while keeping demo outputs deterministic and safe.
- Bounded waits reduce DoS risk and improve scriptability.

Scope:
- Client-only (qsc); no protocol/wire changes.

### D-0112 (2026-01-24) — Sequence QSC store hardening before vault expansion (G4, G5)

Context:
- The QSC design spec makes atomic writes, locking, strict perms (0700/0600), symlink-safe paths, deterministic error classes,
  and no-mutation-on-reject core client invariants.
  (docs/design/QSC_CLI_Client_Design_Spec_v0.1_2026-01-22.md)
- The client security checklist treats the same items as MUST requirements (perms/path/atomic/locking + deterministic rejects + tests).

Decision:
- NA-0060 is the next READY item and must land before we expand encrypted-at-rest vault features:
  - umask 077 + perms enforcement
  - symlink-safe paths and parent safety policy
  - atomic write protocol and locking
  - stable error codes and CI tests proving no-mutation-on-reject at storage boundaries

Consequences:
- Public demo posture becomes defensible: secure-by-default at the local storage layer with invariant tests.
- Vault expansion remains mandatory, but is scoped into a follow-on NA to avoid mixing concerns and to keep reviews fail-closed.

### D-0113 (2026-01-24) — Expand QSC client security backlog without violating single-READY queue invariant (G4, G5)

Context:
- NA-0060 is the single READY item and remains the only immediate execution target.
- Multiple additional client hardening features were identified (vault encryption, protocol-boundary reject invariants, resource limits,
  diagnostics redaction, output minimization, privacy envelopes, ACK camouflage, supply-chain controls, memory hygiene, send commit semantics).

Decision:
- Record the full set as BACKLOG NAs immediately after NA-0060, each with explicit invariants and CI proof requirements.
- Preserve queue discipline: only one READY item at a time; promotion occurs only when the current READY NA is DONE.

Consequences:
- Nothing is “lost in chat”; the roadmap is pinned in NEXT_ACTIONS with testable acceptance criteria.
- Execution remains fail-closed and reviewable, avoiding scope creep while enabling systematic implementation.

Addendum:
- The backlog artifacts include an appendix mapping client_suggestions.txt items into BACKLOG NAs (no READY changes).
