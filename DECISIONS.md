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

- **ID:** D-0227
- **Date:** 2026-02-15
- **Status:** Accepted
- **Goal IDs:** G2, G5
- **Decision:** For NA-0138 follow-up, qsc command execution must not mutate `lock_state` except through explicit lock transitions (`/lock`, successful `/unlock`, auto-lock timeout, init completion), and command parser coverage is enforced by a catalog-based invariant harness.
- **Rationale:** A command-path activity timestamp bug allowed command execution near inactivity boundaries to violate lock/timer invariants and present as relock/wedge behavior.
- **Security invariants introduced/changed:**
  - Benign/read-only/config command paths do not transition lock state.
  - Invalid command/argument rejects are deterministic and no-mutation.
  - Locked allowlist enforcement remains fail-closed while preserving UI responsiveness.
- **Alternatives considered:**
  - Add one-off regression tests per command (rejected: brittle and incomplete coverage).
  - Rely on manual command smoke tests (rejected: insufficient guardrail for parser drift).
- **Implications for spec/impl/tests:**
  - Command activity uses current monotonic TUI time (`current_now_ms`) for timeout bookkeeping.
  - Added command-catalog category invariants in `qsl/qsl-client/qsc/tests/tui_command_catalog_invariants.rs`.
  - TRACEABILITY records NA-0138 follow-up evidence in PR #359.

- **ID:** D-0226
- **Date:** 2026-02-15
- **Status:** Accepted
- **Goal IDs:** G2, G5
- **Decision:** For NA-0138, qsc adds an optional fixed-interval polling mode in TUI (`/poll set fixed <seconds>`), with adaptive mode unchanged as the default.
- **Rationale:** A constant poll cadence reduces timing leakage versus opportunistic polling while preserving explicit operator control and avoiding default regressions.
- **Security invariants introduced/changed:**
  - Fixed polling is explicit-only; default remains adaptive.
  - Poll interval bounds are deterministic and fail-closed (`2..=300` seconds).
  - Reject paths for invalid polling settings perform no persistence mutation.
  - Scheduler cadence is deterministic and does not inject extra out-of-cadence polls after receive.
- **Alternatives considered:**
  - Make fixed cadence the default immediately (rejected: higher latency/cost risk without rollout data).
  - Add jitter in MVP mode (rejected: reduced determinism for initial test-backed rollout).
- **Implications for spec/impl/tests:**
  - qsc TUI command/status paths include polling mode/interval visibility.
  - Added deterministic tests in `qsl/qsl-client/qsc/tests/tui_fixed_polling.rs`.
  - TRACEABILITY records NA-0138 implementation artifacts in PR #358.

- **ID:** D-0225
- **Date:** 2026-02-14
- **Status:** Accepted
- **Goal IDs:** G1, G2, G3, G4, G5
- **Decision:** For NA-0135, QSL adopts a staged ongoing-PQ-ratchet roadmap: Phase 1 uses bounded periodic PQ rekey epochs with strict fail-closed semantics; Phase 2 targets a SPQR-like sparse PQ ratchet layered onto the existing ratchet model after phase-1 evidence gates are met.
- **Rationale:** NA-0133 identified ongoing PQ FS/PCS as not established. A direct one-step SPQR-like rollout carries high state-machine and verification risk. The staged approach improves near-term security posture while preserving deterministic reject/no-mutation invariants and creating an explicit, non-optional path to stronger long-horizon PQ-resilient properties.
- **Security invariants introduced/changed:**
  - Ongoing PQ refresh claims must remain evidence-backed; otherwise status is Not established.
  - Downgrade/version behavior for PQ refresh must be explicit and fail-closed when policy requires PQ refresh support.
  - Invalid/replayed/out-of-order PQ refresh steps must reject deterministically with no persistent state mutation.
- **Alternatives considered:**
  - Immediate full SPQR-like sparse ratchet in one implementation phase (rejected: highest delivery and verification risk in a single lane).
  - Handshake-only PQ posture with no ongoing refresh (rejected: does not close NA-0133 ongoing-PQ gap).
  - Periodic PQ rekey only as a terminal design (rejected: insufficient end-state ambition for strongest PQ-resilient PCS/FS goals).
- **Implications for spec/impl/tests:**
  - NA-0136 must produce a test-first implementation plan covering state machine, transcript binding, downgrade prevention, replay/rollback handling, and reject/no-mutation vectors before code changes.
  - Follow-on implementation NAs should split phase-1 periodic rekey from phase-2 sparse SPQR-like extension to keep risk bounded and evidence auditable.
  - TRACEABILITY must map NA-0135 decision artifacts to roadmap and decision documents.

- **ID:** D-0204
- **Date:** 2026-02-14
- **Status:** Accepted
- **Goal IDs:** G2, G5
- **Decision:** For NA-0131 UX cleanup, lock transitions (manual and auto-lock) must use one shared full-redraw path, nav movement while unlocked must update main content immediately, and lock/unlock copy must remain minimal and deterministic.
- **Rationale:** Remaining UX friction and occasional stale-text artifacts came from inconsistent redraw semantics and delayed nav-driven inspector updates; one deterministic lock transition path and immediate nav-to-main coupling remove ambiguity.
- **Security invariants introduced/changed:**
  - Unlocked->locked transition must fully redraw the frame and remove stale prior-view text.
  - Auto-lock and `/lock` use the same lock-state transition path.
  - Locked/unlock panels remain explicit-intent and low-noise without expanding pre-unlock disclosure.
- **Alternatives considered:**
  - Keep mixed clear mechanisms (terminal clear plus partial redraw) (rejected: can leave visual remnants).
  - Keep Enter-required main update after nav movement (rejected: slower operator flow and inconsistent nav semantics).
- **Implications for spec/impl/tests:**
  - Updated qsc TUI render/lock/nav behavior in `qsl/qsl-client/qsc/src/main.rs`.
  - Extended invariants in `qsl/qsl-client/qsc/tests/tui_lock_unlock_polish.rs`.
  - TRACEABILITY updated with NA-0131 cleanup evidence entry.

- **ID:** D-0203
- **Date:** 2026-02-14
- **Status:** Accepted
- **Goal IDs:** G2, G5
- **Decision:** Standardize post-unlock home focus to Nav with `Lock` selected, simplify unlocked Lock panel content to a concise status view, and require a forced full redraw on unlocked→locked transitions to prevent stale terminal remnants.
- **Rationale:** Operators need deterministic, low-noise post-unlock navigation and an unambiguous lock-state view; lock transitions must fully overwrite prior screen content to avoid partial-text artifacts.
- **Security invariants introduced/changed:**
  - Unlock completion lands on Nav and defaults selection to `Lock` with empty command input.
  - Lock panel omits non-essential/debug-style text and preserves explicit-intent controls.
  - Manual lock and auto-lock both trigger the same full-redraw lock transition path.
- **Alternatives considered:**
  - Keep prior unlock landing behavior (rejected: inconsistent operator starting context).
  - Continue diff-only redraw on lock (rejected: allowed stale visual remnants under some terminal redraw paths).
- **Implications for spec/impl/tests:**
  - qsc TUI lock/unlock state handling and renderer updated in `qsl/qsl-client/qsc/src/main.rs`.
  - Added invariants in `qsl/qsl-client/qsc/tests/tui_lock_unlock_polish.rs`.
  - TRACEABILITY updated for NA-0131 follow-up evidence.

- **ID:** D-0202
- **Date:** 2026-02-12
- **Status:** Accepted
- **Goal IDs:** G2, G5
- **Decision:** Enforce a locked-first TUI shell with zero-leak pre-unlock rendering and fail-closed command gating: while locked, nav is restricted to `Unlock`/`Exit`, main shows only locked/init-required text, and only `/init` (no vault) or `/unlock` (vault present) plus `/exit` are accepted.
- **Rationale:** Prior locked rendering exposed broader UI structure and command surface before unlock. Restricting both render and command paths removes pre-unlock metadata leakage and keeps security state truthful and deterministic.
- **Security invariants introduced/changed:**
  - Pre-unlock UI must not render aliases, IDs, counts, protocol status, or domain data.
  - Locked command gate is deterministic and fail-closed (`locked_unlock_required`) for disallowed commands, including `/help`.
  - `/init` requires alias validation, strong passphrase, confirmation match, and exact acknowledgement (`I UNDERSTAND`), with reject paths no-mutation.
  - Post-init state remains locked until explicit `/unlock`.
- **Alternatives considered:**
  - Keep prior redaction-in-place model with full nav/domain visibility while locked (rejected: leaks non-secret but sensitive metadata and broadens pre-unlock behavior).
  - Permit `/help` in locked mode (rejected: expands pre-unlock disclosure surface).
- **Implications for spec/impl/tests:**
  - qsc TUI renderer and command router updated for locked-shell gating and deterministic rejects.
  - Added locked-first invariants in `qsl/qsl-client/qsc/tests/tui_locked_first.rs`.
  - Existing TUI headless tests aligned with explicit unlocked test mode where domain-level assertions require post-unlock state.

- **ID:** D-0201
- **Date:** 2026-02-11
- **Status:** Accepted
- **Goal IDs:** G2, G5
- **Decision:** Standardize home-nav selection UX to a single deterministic selector (`>` on one row only), with Up/Down moving nav selection only while Nav is focused, and Enter activating only navigation targets.
- **Rationale:** Multiple persistent `>` glyphs in nav made selection ambiguous and slowed operator flow. A single-marker model preserves explicit-intent UX and removes ambiguity without changing command safety or protocol behavior.
- **Security invariants introduced/changed:**
  - Exactly one nav row is selected at a time in home mode.
  - Nav activation (`Enter`) is navigation-only and must not execute command actions.
  - Nav movement is focus-scoped (no selection changes unless Nav has focus).
- **Alternatives considered:**
  - Keep per-header `>`/`v` markers plus item selection markers (rejected: ambiguous active row).
  - Auto-activate on arrow movement (rejected: implicit actions, weaker operator control).
- **Implications for spec/impl/tests:**
  - qsc TUI nav renderer/input paths updated for single-row selection.
  - Added deterministic nav invariants in `qsl/qsl-client/qsc/tests/tui_nav_selection.rs`.
  - Traceability recorded in TRACEABILITY changelog (PR #316).

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
- **PR:** PR https://github.com/QuantumShieldLabs/qsl-protocol/pull/102 (merge b32f0d8d7c46c7d53b9ba97a9697563783b2e715)
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
- **PR:** PR https://github.com/QuantumShieldLabs/qsl-protocol/pull/102 (merge b32f0d8d7c46c7d53b9ba97a9697563783b2e715)
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

### D-0114 (2026-01-24) — QSC store hardening: locking + atomic writes + keyslot-ready metadata (NA-0060)

Context:
- NA-0060 requires fail-closed storage hardening with deterministic error classes and no-mutation-on-reject tests.
- Future hardware unlock factors (e.g., YubiKey) must be possible without re-encrypting all client data.

Decision:
- Implement store locking (exclusive for mutation, shared for read) and atomic write helpers as mandatory primitives.
- Enforce strict permissions (umask 077, dirs 0700, files 0600) and symlink-safe paths.
- Add keyslot-ready metadata layout (VMK + wrapped keyslots) as a forward-compatible placeholder; do not integrate hardware factors yet.

Consequences:
- Store mutations are serialized and fail-closed under deterministic error codes.
- Future unlock factors can be added by populating keyslots without changing core store layout.
### D-0115 — QSC vault command wiring + noninteractive fail-closed markers (NA-0061 skeleton)

Date: 2026-01-25  
Goals: G4, G5

Decision:
- Introduce a qsc vault command surface for NA-0061 and enforce deterministic, fail-closed behavior in noninteractive mode.
- Error paths emit stable QSC_MARK codes and exit nonzero (no silent success on reject).
- Vault init remains a skeleton pending Argon2id + encrypted-at-rest envelope completion in NA-0061.

Invariants protected:
- Noninteractive mode never prompts; rejects deterministically with stable marker.
- Reject paths do not mutate vault state (tests cover no-mutation-on-reject).

Evidence:
- PR: https://github.com/QuantumShieldLabs/qsl-protocol/pull/104
### D-0116 — NA-0061 Phase 2: encrypted-at-rest vault default (Argon2id + AEAD) with deterministic noninteractive

Date: 2026-01-25  
Goals: G4, G5

Decision:
- Vault secrets must be encrypted-at-rest by default (no plaintext mode).
- Passphrase-derived keys use Argon2id with explicit parameters and a versioned envelope.
- Noninteractive mode never prompts; it fails closed with stable markers when passphrase material is missing.
- Key source integration points are defined (keychain preferred, passphrase fallback, YubiKey stub) but device integration remains out of scope.

Rationale:
- Enforces deterministic, testable client-side confidentiality without changing protocol wire behavior.

Consequences:
- Vault file format becomes versioned and forward-upgradable; reject paths remain no-mutation.
Evidence: PR #107 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/107) merged (merge SHA 4e0cc3af7b49224c1b3ac72224d4375219e56088).
- **ID:** D-0110
  - **Status:** Accepted
  - **Date:** 2026-01-25
  - **Goals:** G5
  - **Decision:** Stage QSC YubiKey support as “plumbing now, enforce later” via an extensible vault keyslot model.
  - **Rationale:**
    - Hardware-backed unlock improves data-at-rest posture without protocol wire changes.
    - Keyslots allow migration/recovery and avoid irreversible lockout.
    - Enforcement must be explicit and policy-driven to avoid surprise/unsafe defaults.
  - **Invariants:**
    - Encrypted-at-rest remains default; no silent plaintext storage.
    - Noninteractive mode never prompts; fails closed with a stable marker.
    - Hardware token requirements are never silently enabled; must be explicit configuration/policy.
  - **References:**
    - PR #106 (YubiKey roadmap governance update)
    - docs/design/QSC_CLI_Client_Design_Spec_v0.1_2026-01-22.md (YubiKey roadmap section)
    - NEXT_ACTIONS.md (NA-0061 roadmap note; NA-0062 BACKLOG)
  - **Evidence:** PR #110 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/110) merged (merge SHA aded11b95b81fcbcc89139960a949845ad6f8c78).
- **ID:** D-0117
  - **Status:** Accepted
  - **Date:** 2026-01-25
  - **Goals:** G5
  - **Decision:** QSC vault defaults to encrypted-at-rest storage using Argon2id for passphrase-derived keys, with keychain preferred when available and deterministic passphrase fallback.

- **ID:** D-0118
  - **Status:** Accepted
  - **Date:** 2026-02-08
  - **Goals:** G3, G4, G5
  - **Decision:** NA-0101 requires PQ signature-bound handshake identity using ML-DSA and deterministic fail-closed verification.
  - **Rationale:**
    - TOFU pinning alone detects changes but does not cryptographically bind transcript origin.
    - Signature binding over canonical handshake transcript hardens active MITM resistance for pinned peers.
  - **Invariants:**
    - Signature secret material is vault-backed and never persisted plaintext in identity records.
    - Pinned peer handshake rejects on signature verification failure with deterministic markers.
    - Reject paths do not mutate session/pin state.
  - **References:**
    - PR #237 (NA-0101 implementation)
    - `qsl/qsl-client/qsc/src/main.rs`
    - `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs`
  - **Rationale:**
    - Default encryption prevents silent plaintext storage.
    - Keychain-backed unlock reduces passphrase exposure when the platform supports secure storage.
    - Deterministic noninteractive behavior enables safe automation and CI validation.
  - **Invariants:**
    - Noninteractive mode never prompts; if passphrase is required and not explicitly provided, it fails closed with a stable marker.
    - Reject paths do not create/overwrite the vault file (atomic commit only on success).
    - Vault secrets never appear in plaintext on disk.
  - **Parameters:**
    - Argon2id parameters are explicit and fixed in code (deterministic and testable).
  - **References:**
    - NA-0061 (NEXT_ACTIONS.md)
    - QSC vault tests (qsl/qsl-client/qsc/tests/vault.rs)

- **ID:** D-0118
  - **Status:** Accepted
  - **Date:** 2026-01-25
  - **Goals:** G5
  - **Decision:** Mandate quoting-safe directive templates (no nested heredocs, no python3 heredoc piped to tee).
  - **Rationale:** Prevent recurring execution failures and governance drift.
  - **References:** DOC-DEV-004 (Quoting-safe directive template)

- **ID:** D-0119
  - **Status:** Accepted
  - **Date:** 2026-01-25
  - **Goals:** G5
  - **Decision:** Introduce a vault keyslot provider abstraction with a YubiKey stub and mock provider for CI.
  - **Rationale:**
    - A provider abstraction allows key-source evolution without protocol changes.
    - The YubiKey provider is stubbed to fail closed until hardware integration is explicitly approved.
    - A mock provider enables deterministic CI coverage of invariants without hardware.
  - **Invariants:**
    - Noninteractive mode never prompts; failures emit stable markers.
    - Reject paths do not mutate vault state.
    - YubiKey selection fails closed deterministically (no hardware deps).
  - **References:**
    - NA-0062 (NEXT_ACTIONS.md)
- **ID:** D-0120
- **Date:** 2026-01-25
- **Status:** Accepted
- **Goal IDs:** G5
- **Decision:** Enforce bounded resource limits and deterministic retries/timeouts in the QSC client (NA-0063).
- **Rationale:** Prevent unbounded growth or infinite waits at client boundaries and preserve deterministic, testable behavior.
- **Security invariants introduced/changed:**
  - Queue/history sizes are bounded; overflow is a deterministic reject with stable markers.
  - Retry loops and timeouts are bounded; no infinite waits.
  - Reject paths do not mutate state.
- **Alternatives considered:**
  - Unbounded queues/retries for “flexibility” (rejected: unsafe and nondeterministic).
- **Implications for spec/impl/tests:**
  - Add bounded queue/history helpers and retry/timeout helpers in qsc.
  - Add CI tests: queue_limit_enforced, retry_bound_enforced, timeout_marker_stable.
  - **Evidence:** PR #112 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/112) merged (merge SHA 85508a2bd9f8c0567ae9856db775a838a6a1f593).
- **ID:** D-0121
- **Date:** 2026-01-25
- **Status:** Accepted
- **Goal IDs:** G5
- **Decision:** Add a marker schema v1 with optional JSONL output and a redacted doctor export for diagnostics (NA-0064).
- **Rationale:** Deterministic, secret-free diagnostics and markers enable CI checks and safe automation without leaking sensitive data.
- **Security invariants introduced/changed:**
  - Diagnostics output never includes secrets; doctor exports are redacted.
  - Logging is disabled by default; when enabled, outputs are redacted.
  - Marker schema v1 is deterministic; JSONL is opt-in.
- **Alternatives considered:**
  - Ad-hoc printf diagnostics (rejected: nondeterministic and secret-risky).
- **Implications for spec/impl/tests:**
  - Add JSONL marker option and redacted doctor export.
  - Add CI tests: diagnostics_no_secrets, markers_schema_stable, logs_off_by_default.
- **ID:** D-0122
  - **Status:** Accepted
  - **Date:** 2026-01-25
  - **Goals:** G5
  - **Decision:** Default QSC CLI output is redacted to minimize endpoints/timestamps/high-cardinality identifiers; explicit reveal requires a flag.
  - **Rationale:** Public demo posture requires conservative output; explicit reveal reduces accidental leakage.
  - **Invariants:**
    - Default output redacts sensitive or high-cardinality values.
    - Explicit reveal is opt-in and must be passed as a flag.
    - No protocol wire changes.
  - **References:** NA-0065 (NEXT_ACTIONS.md)
  - **Evidence:** PR #116 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/116) merged (merge SHA 71ef24c6b92bb600c0e12eb900bedeeec573f4b6).
- **ID:** D-0123
  - **Status:** Accepted
  - **Date:** 2026-01-25
  - **Goals:** G5
  - **Decision:** Define QSC privacy envelopes with deterministic tick schedule, size buckets, and bundle packing (NA-0066).
  - **Rationale:** Deterministic envelope shaping supports measurable privacy posture and CI-verifiable behavior without protocol wire changes.
  - **Invariants:**
    - Tick schedule is deterministic and bounded; no infinite delays.
    - Bucket sizing is deterministic and testable; no overclaims.
    - Bundle packing is deterministic with explicit size/count bounds.
  - **Implications for spec/impl/tests:**
    - Add envelope contract + scheduler + packing implementation in qsc.
    - Add tests: tick_schedule_stable_and_bounded; bucket_sizes_match_spec; bundle_packing_rules.
  - **References:** NA-0066 (NEXT_ACTIONS.md)
  - **Evidence:** PR #118 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/118) merged (merge SHA 6a8fcd9268dceb6b9bf9abd2f64c9e988521d6fb).
- **ID:** D-0124
  - **Status:** Accepted
  - **Date:** 2026-01-25
  - **Goals:** G5
  - **Decision:** Require a “State Ledger” proven from main in every session before issuing directives to prevent wrong-NA / wrong-PR actions.
  - **Rationale:** Eliminates drift and confusion across long-running, multi-PR workflows and carries into new chats via required docs.
  - **References:** DOC-DEV-003 (Mandatory State Ledger), CHAT_STARTER (state reset bullet)
- **ID:** D-0125
  - **Status:** Accepted
  - **Date:** 2026-01-25
  - **Goals:** G5
  - **Decision:** ACK/receipt envelopes are mapped into the small-message size/tick class to avoid ACK distinguishability (NA-0067).
  - **Rationale:** Aligning ACKs with the smallest message class prevents ACKs from forming a unique observable size/timing class.
  - **Invariants:**
    - ACK size class matches the small-message envelope bucket.
    - ACK planning uses the same bounded tick schedule as regular envelopes.
    - Deterministic planning; no wall-clock dependence in tests.
  - **References:** NA-0067 (NEXT_ACTIONS.md)
  - **Evidence:** PR #121 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/121) merged (merge SHA aceedd34da242722f8f57844f0e3394de33b4732).
- **ID:** D-0126
- **Date:** 2026-01-26
- **Status:** Accepted
- **Goal IDs:** G5
- **Decision:** Enforce locked dependency builds, add advisory scanning, and require release provenance/checksums for QSC artifacts.
- **Rationale:** Supply-chain integrity depends on deterministic dependency resolution, early advisory detection, and verifiable release artifacts without inventing signing keys.
- **Security invariants introduced/changed:**
  - CI builds/tests are fail-closed on lockfile drift (use --locked).
  - Advisory scanning fails closed on known vulnerabilities.
  - Release artifacts are accompanied by SHA256 checksums and provenance attestations.
- **Alternatives considered:**
  - Best-effort checks without CI enforcement (rejected: allows drift).
  - Key-based signing with ad hoc keys (rejected: no secret material should be invented in CI).
- **Implications for spec/impl/tests:**
  - CI workflows include advisory and release-auth policy checks.
  - Release workflow generates checksums and provenance attestations.
- **Addendum (2026-01-26):** Temporary RustSec advisory DB pin (RUSTSEC_DB_PIN=3c3cbe8838d5c1a23ca31592353142aa78100d64) to avoid cargo-audit CVSS 4.0 parse failure (RUSTSEC-2026-0003). Remove once upstream supports CVSS4.
- **Addendum:** Upgraded cargo-audit to 0.22.0+ to support CVSS 4.0; removed temporary RustSec DB pin.
- **ID:** D-0127
  - **Status:** Accepted
  - **Date:** 2026-01-26
  - **Goals:** G5
  - **Decision:** In blocked/unclear situations, require a Codex read-only diagnosis pass before making changes.
  - **Rationale:** Prevents guesswork; uses repo-local evidence (workflows/logs/diffs) to identify root causes quickly.
  - **References:** DOC-DEV-003 (Codex diagnosis rule), CHAT_STARTER bullet
- **ID:** D-0128
  - **Status:** Accepted
  - **Date:** 2026-01-26
  - **Goals:** G5
  - **Decision:** Enforce QSC secret hygiene in memory: zeroize passphrase/key material and avoid secret exposure in errors/markers.
  - **Rationale:** Reduce crash surface and accidental leakage without changing protocol behavior.
  - **Invariants:**
    - Secrets never appear in stdout/stderr/markers.
    - Passphrase and key buffers are zeroized before exit on failure paths.
    - Deterministic error markers for new reject paths.
  - **Notes:** CI proves redaction and no-mutation-on-reject; memory zeroization is enforced by code but not directly observable in tests.
- **ID:** D-0129
  - **Status:** Accepted
  - **Date:** 2026-01-26
  - **Goals:** G5
  - **Decision:** Implement QSC send commit semantics with durable outbox (prepare→send→commit) so send state advances only on confirmed success.
  - **Rationale:** Prevents state advancement on transport failure and preserves no-mutation-on-reject invariants.
  - **Invariants:**
    - Send state is unchanged on transport failure.
    - Outbox is removed only after successful commit.
    - Deterministic markers for prepare/send/commit outcomes.
  - **References:** NA-0070 (NEXT_ACTIONS.md)
  - **Evidence:** PR #128 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/128) merged (merge SHA d0f3801d3d020ec2b65c73dabf95283202b1a327).
- **ID:** D-0130
  - **Status:** Accepted
  - **Date:** 2026-01-26
  - **Goals:** G1, G3, G4, G5
  - **Decision:** Prioritize NA-0071 to harden QSP v4.3 header key derivation with KMAC-based KDF; placeholders are release-blocking.
  - **Rationale:** Placeholder/static derivation undermines header confidentiality/integrity and domain separation; correctness must be proven in refimpl before release.
  - **Invariants:**
    - Header keys MUST be derived from RK via KMAC (QSP4.3/HK and QSP4.3/NHK labels).
    - Placeholders/static labels must never ship in protocol lanes.
    - Rejected inputs must not mutate session state.
  - **References:** NA-0071 (NEXT_ACTIONS.md); tools/refimpl/quantumshield_refimpl/src/qsp/state.rs, handshake.rs, ratchet.rs; tests/NA-0071_qsp_header_key_derivation_testplan.md
- **ID:** D-0131
  - **Status:** Accepted
  - **Date:** 2026-01-26
  - **Goals:** G1, G3, G4, G5
  - **Decision:** Require SessionState initialization and recomputation to derive HK/NHK via KMAC from RK; remove placeholder/static derivation from non-test builds.
  - **Rationale:** Header key correctness and domain separation depend on RK; placeholders undermine confidentiality/integrity and must not ship.
  - **Invariants:**
    - HK/NHK are derived via KMAC from RK at session init and when recomputed.
    - Wrong RK fails to decrypt boundary headers deterministically.
    - Reject paths do not mutate session state.
  - **References:** NA-0071; tools/refimpl/quantumshield_refimpl/src/qsp/state.rs, handshake.rs; tools/refimpl/quantumshield_refimpl/tests/na_0071_header_key_derivation.rs; tests/NA-0071_qsp_header_key_derivation_testplan.md
- **ID:** D-0132
  - **Status:** Accepted
  - **Date:** 2026-01-27
  - **Goals:** G1, G3, G4, G5
  - **Decision:** Housekeeping must be provably safe: no silent deletions, reference integrity required, CI green required.
  - **Rationale:** Prevents loss of reproducibility and avoids misleading duplicates/deprecated artifacts.
  - **References:** NA-0072 plan stub (tests/NA-0072_repo_housekeeping_plan.md)
- **ID:** D-0133
  - **Status:** Accepted
  - **Date:** 2026-01-27
  - **Goals:** G3, G4, G5
  - **Decision:** qsc TUI is a Security Lens with explicit-only interaction; charter is enforceable by tests.
  - **Rationale:** Prevents UX creep from weakening invariants while enabling a demonstrable, safe interface.
  - **Invariants:**
    - No implicit sends/retries/recovery.
    - No secrets in UI/markers/logs (redaction enforced).
    - Deterministic markers for all state changes.
  - **References:** NA-0074; docs/qsc/DOC-QSC-001_TUI_Charter_Security_Lens_v1.0.0_DRAFT.md; tests/NA-0074_qsc_security_lens_mvp_plan.md
- **ID:** D-0134
  - **Status:** Accepted
  - **Date:** 2026-01-27
  - **Goals:** G3, G4, G5
  - **Decision:** Implement NA-0074 Phase 1 via a read‑mostly TUI skeleton with deterministic markers and explicit commands; enforce charter rules with headless tests; keep qsc clippy -D warnings clean.
  - **Rationale:** Establishes a safe, observable UI surface without implicit actions and prevents regression in warning-free builds.
  - **References:** NA-0074; qsl/qsl-client/qsc/src/main.rs; qsl/qsl-client/qsc/tests/tui_charter.rs; tests/NA-0074_qsc_security_lens_mvp_plan.md
- **ID:** D-0135
  - **Status:** Accepted
  - **Date:** 2026-01-28
  - **Goals:** G3, G4, G5
  - **Decision:** Complete NA-0074 Phase 1 by wiring real TUI status fields, adding an explicit per‑peer session panel, and introducing a deterministic receive‑reject no‑mutation regression test aligned to the Security Lens charter.
  - **Rationale:** Closes Phase 1 acceptance gaps while preserving explicit‑only interaction and deterministic markers.
  - **Invariants:**
    - No implicit send/retry/recovery.
    - Deterministic markers for status updates and rejects.
    - No mutation on reject; no secrets in output.
  - **References:** NA-0074; PR #142 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/142); docs/qsc/DOC-QSC-001_TUI_Charter_Security_Lens_v1.0.0_DRAFT.md; tests/NA-0074_qsc_security_lens_mvp_plan.md; D-0133
- **ID:** D-0136
  - **Status:** Accepted
  - **Date:** 2026-01-28
  - **Goals:** G3, G4, G5
  - **Decision:** Relay demo transport is explicitly-controlled and charter-enforced; fault injection must be seedable and deterministic; no implicit retries/recovery.
  - **Rationale:** Enables realistic demo conditions without weakening Security Lens invariants or observability.
  - **References:** NA-0075; docs/qsc/DOC-QSC-002_Relay_Demo_Transport_Contract_v1.0.0_DRAFT.md; tests/NA-0075_qsc_relay_demo_transport_plan.md; docs/qsc/DOC-QSC-001_TUI_Charter_Security_Lens_v1.0.0_DRAFT.md
- **ID:** D-0137
  - **Status:** Accepted
  - **Date:** 2026-01-28
  - **Goals:** G3, G4, G5
  - **Decision:** Implement NA-0075 Phase 1 relay demo transport in qsc with explicit-only send/receive, seeded deterministic fault injection (drop/dup/reorder/delay), and regression tests proving no mutation on failure/reject and deterministic markers.
  - **Invariants:**
    - No implicit sends; no automatic retries; no background recovery.
    - No mutation on failure/reject.
    - Deterministic markers for relay events and send lifecycle.
  - **References:** NA-0075; PR #145 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/145); docs/qsc/DOC-QSC-002_Relay_Demo_Transport_Contract_v1.0.0_DRAFT.md; tests/NA-0075_qsc_relay_demo_transport_plan.md; D-0136; docs/qsc/DOC-QSC-001_TUI_Charter_Security_Lens_v1.0.0_DRAFT.md
- **ID:** D-0138
  - **Status:** Accepted
  - **Date:** 2026-01-28
  - **Goals:** G3, G4, G5
  - **Decision:** Adopt proactive-improvement reporting and deterministic tooling defaults as enforced workflow policy for governance and engineering tasks.
  - **Rationale:** Ensures higher-quality outcomes and reproducible diagnostics without drive-by changes.
  - **References:** NA-0076; docs/dev/DOC-DEV-003_Assistant_Operating_Rules_v1.0.0_DRAFT.md; tests/NA-0076_quality_workflow_hardening_plan.md
- **ID:** D-0139
  - **Status:** Accepted
  - **Date:** 2026-01-28
  - **Goals:** G3, G4, G5
  - **Decision:** Demo packaging must be deterministic, safe-to-share, and charter-enforced; scripts/runbook are treated as security artifacts.
  - **Rationale:** Ensures demos reflect real behavior without weakening Security Lens invariants.
  - **References:** NA-0077; docs/qsc/DOC-QSC-003_Demo_Packaging_Runbook_v1.0.0_DRAFT.md; tests/NA-0077_demo_packaging_plan.md; docs/qsc/DOC-QSC-001_TUI_Charter_Security_Lens_v1.0.0_DRAFT.md
- **ID:** D-0140
  - **Status:** Accepted
  - **Date:** 2026-01-29
  - **Goals:** G3, G4, G5
  - **Decision:** Demo full-run must be deterministic, safe-to-share, and charter-enforced; CI smoke must prevent rot without requiring sudo.
  - **Rationale:** Ensures reproducible demos under hostile conditions without weakening invariants.
  - **References:** NA-0078, DOC-QSC-004, tests/NA-0078_demo_fullrun_plan.md, DOC-QSC-001
- **ID:** D-0141
  - **Status:** Accepted
  - **Date:** 2026-01-29
  - **Goals:** G3, G4, G5
  - **Decision:** TUI relay integration must remain explicit-only; event stream is a security artifact; determinism via seed; no implicit retry/recovery.
  - **Rationale:** Preserve Security Lens guarantees while surfacing hostile transport events safely.
  - **References:** NA-0079, DOC-QSC-005, tests/NA-0079_qsc_tui_relay_integration_plan.md, DOC-QSC-001, DOC-QSC-002
- **ID:** D-0142
  - **Status:** Accepted
  - **Date:** 2026-01-29
  - **Goals:** G3, G4, G5
  - **Decision:** Implement NA-0079 Phase 1 wiring of the Security Lens TUI to relay transport with explicit-only selection, an in-TUI events stream for hostile relay events, and headless seeded determinism tests.
  - **Invariants:**
    - Explicit-only transport selection.
    - No implicit send/retry/recovery.
    - Deterministic markers and event stream with seed.
    - No mutation on failure (as evidenced by tests/markers where applicable).
  - **References:** NA-0079; PR #157 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/157); docs/qsc/DOC-QSC-005_TUI_Relay_Integration_Security_Lens_v1.0.0_DRAFT.md; tests/NA-0079_qsc_tui_relay_integration_plan.md; docs/qsc/DOC-QSC-001_TUI_Charter_Security_Lens_v1.0.0_DRAFT.md; docs/qsc/DOC-QSC-002_Relay_Demo_Transport_Contract_v1.0.0_DRAFT.md
- **ID:** D-0143
  - **Status:** Accepted
  - **Date:** 2026-01-30
  - **Decision:** Remote relay testing must be non-flaky (nightly/manual), charter-enforced, safe-to-share, and never a required PR gate.
  - **Rationale:** Preserves merge stability while still exercising real network conditions.
  - **References:** NA-0080, DOC-QSC-006_Remote_Relay_Testing_Contract_v1.0.0_DRAFT.md, tests/NA-0080_remote_relay_testing_plan.md
- **ID:** D-0144
  - **Status:** Accepted
  - **Date:** 2026-02-01
  - **Goals:** G3, G4, G5
  - **Decision:** Add `qsc send abort` to clear stale outbox state deterministically so `outbox_exists` recovery does not require manual file deletion.
  - **Invariants:**
    - Explicit-only action; no implicit recovery.
    - No secrets/payload contents in output.
    - Safe-parent checks enforced.
    - Idempotent behavior (second run reports absent).
  - **References:** PR #163 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/163); qsl/qsl-client/qsc/tests/outbox_abort.rs
- **ID:** D-0145
  - **Status:** Accepted
  - **Date:** 2026-02-01
  - **Decision:** Doctor output must be unambiguous: checked_dir and dir_writable_required fields are mandatory; safe-to-share determinism preserved.
  - **Rationale:** Prevents ambiguity in operator diagnostics while keeping outputs safe to share and stable for tests.
  - **References:** NA-0082; qsc doctor
- **ID:** D-0146
  - **Status:** Accepted
  - **Date:** 2026-02-01
  - **Goals:** G3, G4, G5
  - **Decision:** Implement NA-0082 by adding doctor marker fields checked_dir and dir_writable_required, and regression tests to enforce deterministic, safe-to-share output.
  - **Invariants:**
    - Doctor markers include checked_dir and dir_writable_required.
    - No secrets in doctor output.
    - Deterministic marker structure.
  - **References:** NA-0082; PR #165 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/165); qsl/qsl-client/qsc/tests/cli.rs
- **ID:** D-0147
  - **Status:** Accepted
  - **Date:** 2026-02-01
  - **Decision:** qsc must honor XDG roots for lock/store paths; lock failures must be unambiguous (open vs contention) to preserve fail-closed diagnostics.
  - **Rationale:** Enables harness isolation without HOME hacks and makes lock failure causes explicit in markers.
  - **References:** NA-0083; qsc lock/store paths
- **ID:** D-0148
  - **Status:** Accepted
  - **Date:** 2026-02-01
  - **Goals:** G3, G4, G5
  - **Decision:** Implement NA-0083 by honoring XDG_CONFIG_HOME for lock/store paths and splitting lock errors into lock_open_failed vs lock_contended, with regression tests.
  - **Invariants:**
    - XDG roots are honored for lock/store paths.
    - Lock open failures and contention are distinguishable.
    - Safe-parent checks remain enforced.
    - No secrets in marker output.
  - **References:** NA-0083; PR #168 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/168); tests/NA-0083_qsc_xdg_lock_plan.md
- **ID:** D-0150
  - **Status:** Accepted
  - **Date:** 2026-02-01
  - **Goals:** G3, G4, G5
  - **Decision:** qsc send is the primary sender; sending requires explicit transport selection; relay-backed send is test-driven.
  - **Rationale:** Removes ambiguity between `qsc send` and `qsc relay send` while preserving explicit-only transport semantics.
  - **References:** NA-0084; tests/NA-0084_qsc_send_semantics_plan.md
- **ID:** D-0152
  - **Status:** Accepted
  - **Date:** 2026-02-01
  - **Goals:** G3, G4, G5
  - **Decision:** Implement NA-0084 by requiring explicit relay transport for `qsc send`, delegating to relay send plumbing, and adding regression tests for refusal, happy-path, failure no-commit, and outbox recovery.
  - **Invariants:**
    - Explicit-only transport selection; no implicit send.
    - No secrets or payload contents in output.
    - Failure does not commit; outbox recovery via `qsc send abort`.
  - **References:** NA-0084; PR #171 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/171); qsl/qsl-client/qsc/tests/send_semantics.rs; qsl/qsl-client/qsc/tests/send_commit.rs
- **ID:** D-0153
  - **Status:** Accepted
  - **Date:** 2026-02-01
  - **Decision:** TUI /help must render a deterministic in-app command list; headless tests enforce it to prevent UX regressions.
  - **Rationale:** Ensures help is visible and stable in the TUI without relying on implicit markers.
  - **References:** NA-0085; qsc TUI help rendering
- **ID:** D-0154
  - **Status:** Accepted
  - **Date:** 2026-02-02
  - **Goals:** G3, G4, G5
  - **Decision:** Implement NA-0085 by rendering a deterministic in-app command list for `/help`; headless tests enforce rendering and determinism.
  - **Invariants:**
    - Deterministic ordering and strings for the help list.
    - Help derived from the command registry/parser to avoid drift.
    - No secrets in help output.
  - **References:** NA-0085; PR #174 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/174); qsl/qsl-client/qsc/tests/tui_help_render.rs
- **ID:** D-0155
  - **Status:** Accepted
  - **Date:** 2026-02-02
  - **Goals:** G3, G4, G5
  - **Decision:** Interactive TUI must never print QSC_MARK to the terminal; markers are routed in-app, while headless mode retains stdout markers.
  - **Rationale:** Prevents framebuffer corruption in interactive TUI while preserving deterministic marker output for tests/CI.
  - **References:** NA-0086; qsc TUI marker routing
- **ID:** D-0156
  - **Status:** Accepted
  - **Date:** 2026-02-02
  - **Goals:** G3, G4, G5
  - **Decision:** Implement NA-0086 by routing interactive TUI markers in-app (no stdout) while keeping headless stdout markers; add headless tests to enforce both modes.
  - **Invariants:**
    - Interactive TUI emits no QSC_MARK to stdout/stderr.
    - Headless mode continues to emit QSC_MARK to stdout deterministically.
    - No secrets in marker output.
  - **References:** NA-0086; PR #177 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/177); qsl/qsl-client/qsc/tests/tui_marker_routing.rs

- **ID:** D-0157
  - **Status:** Accepted
  - **Date:** 2026-02-02
  - **Decision:** /help is full-screen help mode; deterministic and registry-derived; prevents UX drift.
  - **References:** NA-0087; qsc TUI help full-screen mode
- **ID:** D-0158
  - **Status:** Accepted
  - **Date:** 2026-02-02
  - **Goals:** G3, G4, G5
  - **Decision:** Implement NA-0087 by adding full-screen help mode (/help + F1) with deterministic list+details rendering and headless tests.
  - **Invariants:**
    - Help content is deterministic and registry-derived.
    - Interactive mode does not emit QSC_MARK to stdout.
    - No secrets in help output.
  - **References:** NA-0087; PR #180 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/180); qsl/qsl-client/qsc/tests/tui_help_fullscreen.rs
- **ID:** D-0159
  - **Status:** Accepted
  - **Date:** 2026-02-02
  - **Decision:** TUI uses full-screen Focus modes for deep inspection; keymap avoids F1 interception; deterministic and test-backed.
  - **References:** NA-0088; qsc TUI focus modes
- **ID:** D-0160
  - **Status:** Accepted
  - **Date:** 2026-02-02
  - **Goals:** G3, G4, G5
  - **Decision:** Implement NA-0088 by adding full-screen Focus modes (Events/Status/Session/Contacts) with /focus commands, F2–F5 keybinds, and deterministic headless tests.
  - **Invariants:**
    - Focus mode switches are explicit-only and deterministic.
    - Interactive mode does not emit QSC_MARK to stdout.
    - No secrets in focus output.
  - **References:** NA-0088; PR #183 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/183); qsl/qsl-client/qsc/tests/tui_focus_modes.rs
- **ID:** D-0161
  - **Status:** Accepted
  - **Date:** 2026-02-02
  - **Decision:** Demo artifacts must be self-explanatory: include deterministic hostile counts; safe-to-share; no new required PR gates.
  - **References:** NA-0089; demo artifact counts
- **ID:** D-0162
  - **Status:** Accepted
  - **Date:** 2026-02-02
  - **Decision:** Remote demo scenarios must be meaningful; enforce via client-side deterministic fault injection; safe-to-share markers.
  - **References:** NA-0090; remote scenario enforcement

- **ID:** D-0163
  - **Status:** Accepted
  - **Date:** 2026-02-02
  - **Goals:** G3, G4, G5
  - **Decision:** Implement client-side deterministic relay fault injection markers (drop/reorder) keyed by seed+scenario to make remote scenarios meaningful without server changes.
  - **Invariants:**
    - explicit-only; no implicit retries
    - no secrets in markers
    - drop-reorder retains deliver events
  - **References:** NA-0090; PR #189 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/189); qsl/qsl-client/qsc/tests/remote_fault_injection.rs

- **ID:** D-0164
  - **Status:** Accepted
  - **Date:** 2026-02-02
  - **Goals:** G3, G4, G5
  - **Decision:** Receive must be explicit-only; two-way exchange is test-backed; TUI integrates /receive without background polling.
  - **References:** NA-0091; qsc receive; TUI /receive
- **ID:** D-0165
  - **Status:** Accepted
  - **Date:** 2026-02-02
  - **Goals:** G3, G4, G5
  - **Decision:** Implement NA-0091 with relay-backed receive (CLI + TUI), deterministic recv markers, and two-way E2E tests using an embedded inbox server.
  - **Invariants:**
    - explicit-only receive; no background polling
    - no secrets or payloads in markers/UI
    - deterministic marker schema for recv_start/recv_item/recv_commit
  - **References:** NA-0091; PR #192 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/192); qsl/qsl-client/qsc/tests/receive_e2e.rs

- **ID:** D-0166
  - **Status:** Accepted
  - **Date:** 2026-02-02
  - **Decision:** QSP/QSE on-wire enforcement: pack/encrypt before push; verify/decrypt/unpack after pull; truthy ACTIVE/INACTIVE status derived from runtime behavior.
  - **References:** NA-0092; QSP/QSE on-wire enforcement

- **ID:** D-0167
  - **Status:** Accepted
  - **Date:** 2026-02-02
  - **Goals:** G3, G4, G5
  - **Decision:** Implement NA-0092 by wiring qsc send/receive to refimpl QSP/QSE envelopes and exposing truthy ACTIVE/INACTIVE status.
  - **Invariants:**
    - Outbound on-wire bytes are encrypted envelopes (not raw payload).
    - Inbound bytes are verified/unpacked before write; failures are deterministic and do not mutate state.
    - No payloads/secrets in markers or UI output.
  - **References:** NA-0092; PR #195 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/195); qsl/qsl-client/qsc/tests/qsp_qse_onwire.rs

- **ID:** D-0168
  - **Status:** Accepted
  - **Date:** 2026-02-02
  - **Decision:** Protocol status must be truthy and deterministic: QSP/QSE ACTIVE/INACTIVE with explicit reasons, in CLI and TUI.
  - **References:** NA-0093; QSP/QSE status truthy requirement

- **ID:** D-0169
  - **Status:** Accepted
  - **Date:** 2026-02-02
  - **Goals:** G3, G4, G5
  - **Decision:** Implement NA-0093 by deriving QSP/QSE ACTIVE/INACTIVE from a local pack+unpack self-check and emitting explicit reasons in status markers.
  - **Invariants:**
    - No `reason=none`; all INACTIVE states carry explicit reason.
    - No secrets in status output.
  - **References:** NA-0093; PR #198 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/198); qsl/qsl-client/qsc/tests/qsp_status_truthy.rs

- **ID:** D-0170
  - **Status:** Accepted
  - **Date:** 2026-02-02
  - **Goals:** G3, G4, G5
  - **Decision:** Enforce a hard protocol gate: qsc send/receive must refuse unless QSP/QSE status is ACTIVE, with deterministic protocol_inactive errors.
  - **Invariants:**
    - No insecure override by default.
    - No payloads/secrets in markers.
  - **References:** NA-0094; protocol gate requirement

- **ID:** D-0171
  - **Status:** Accepted
  - **Date:** 2026-02-04
  - **Goals:** G3, G4, G5
  - **Decision:** Implement NA-0094 by enforcing a hard protocol gate on send/receive and adding deterministic tests for ACTIVE/INACTIVE behavior.
  - **Invariants:**
    - protocol_inactive errors include explicit reason.
    - No outbox mutation on inactive refusal.
  - **References:** NA-0094; PR #201 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/201); qsl/qsl-client/qsc/tests/qsp_protocol_gate.rs

- **ID:** D-0172
  - **Status:** Accepted
  - **Date:** 2026-02-04
  - **Goals:** G3, G4, G5
  - **Decision:** Prioritize NA-0095 handshake MVP over inbox; ratchet advancement deferred to NA-0096; server remains blind and fail-closed.
  - **Invariants:**
    - Deterministic transcript verification; tamper/replay rejected.
    - No mutation on reject.
    - No secrets in markers/UI/logs.
  - **References:** NA-0095; NA-0096; handshake MVP decision

- **ID:** D-0173
  - **Status:** Accepted
  - **Date:** 2026-02-05
  - **Goals:** G3, G4, G5
  - **Decision:** Implement PQ KEM (ML-KEM-768) in refimpl to enable PQ or PQ-primary hybrid handshake; X25519-only is forbidden.
  - **Invariants:**
    - Handshake must derive its primary shared secret from PQ KEM.
    - No secrets in markers/UI/logs.
  - **References:** NA-0095; PQ KEM prerequisite

- **ID:** D-0174
  - **Status:** Accepted
  - **Date:** 2026-02-05
  - **Goals:** G3, G4, G5
  - **Decision:** Implement ML-KEM-768 (PqKem768) in refimpl StdCrypto with deterministic tests for roundtrip and tamper behavior.
  - **Invariants:**
    - PQ KEM encapsulate/decapsulate roundtrip yields identical shared secret.
    - Tampered ciphertext does not yield the same shared secret.
  - **References:** NA-0095; PR #207 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/207); tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs

- **ID:** D-0175
  - **Status:** Accepted
  - **Date:** 2026-02-05
  - **Goals:** G3, G4, G5
  - **Decision:** Implement NA-0095 handshake using ML-KEM-768 shared secret (StdCrypto:PqKem768); forbid X25519-only key agreement; transcript MAC is keyed from pq_init_ss; markers include PQ length evidence only.
  - **Invariants:**
    - Session establishment derives its primary secret from PQ KEM.
    - Rejected handshake messages do not mutate state.
    - No secrets in markers/UI/logs.
  - **References:** NA-0095; PR #205 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/205); qsl/qsl-client/qsc/src/main.rs; qsl/qsl-client/qsc/tests/handshake_mvp.rs

- **ID:** D-0176
  - **Status:** Accepted
  - **Date:** 2026-02-05
  - **Goals:** G3, G4, G5
  - **Decision:** Implement first ratchet step with send+recv chains and bounded skipped handling; correctness proven by deterministic tests; fail-closed on tamper/replay/out-of-order.
  - **Invariants:**
    - No message key reuse; chains advance per message.
    - Skipped keys are bounded with deterministic eviction.
    - Rejects do not mutate persistent state.
    - No secrets in markers/UI/logs.
  - **References:** NA-0096; ratchet step governance

- **ID:** D-0177
  - **Status:** Accepted
  - **Date:** 2026-02-05
  - **Goals:** G3, G4, G5
  - **Decision:** Implement NA-0096 ratchet send/recv advancement with bounded skipped handling and deterministic markers/tests.
  - **Invariants:**
    - No message key reuse; chains advance per message.
    - Skipped keys are bounded with deterministic eviction.
    - Rejects do not mutate persistent state.
    - No secrets in markers/UI/logs.
  - **References:** NA-0096; PR #211 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/211); qsl/qsl-client/qsc/tests/ratchet_step.rs

- **ID:** D-0178
  - **Status:** Accepted
  - **Date:** 2026-02-05
  - **Goals:** G3, G4, G5
  - **Decision:** Implement A2 confirm so the responder commits only after explicit transcript confirmation; improve handshake correctness while remaining PQ-primary and fail-closed.
  - **Invariants:**
    - Responder commits session only after valid A2.
    - Tamper/replay/out-of-order rejected deterministically with no mutation.
    - No secrets in markers/UI/logs.
  - **References:** NA-0099; tests/NA-0099_handshake_a2_confirm_plan.md

- **ID:** D-0179
  - **Status:** Accepted
  - **Date:** 2026-02-06
  - **Goals:** G3, G4, G5
  - **Decision:** Fix NA-0099 A2 replay test to deliver A2 once, reject replay deterministically, and prove no mutation on replay.
  - **Invariants:**
    - A2 replay is rejected deterministically.
    - Session state is unchanged after replay.
    - No secrets in markers/UI/logs.
  - **References:** NA-0099; PR #214 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/214); qsl/qsl-client/qsc/tests/handshake_mvp.rs

- **ID:** D-0180
  - **Status:** Accepted
  - **Date:** 2026-02-06
  - **Goals:** G3, G4, G5
  - **Decision:** Prioritize identity binding MVP (TOFU) now; PQ signature identity deferred to NA-0101; forbid silent accept on mismatch.
  - **Invariants:**
    - Peer identity mismatch is deterministically rejected.
    - No secrets in markers/UI/logs.
  - **References:** NA-0100; NA-0101; tests/NA-0100_identity_binding_tofu_plan.md; tests/NA-0101_pq_signature_identity_plan.md

- **ID:** D-0181
  - **Status:** Accepted
  - **Date:** 2026-02-06
  - **Goals:** G3, G4, G5
  - **Decision:** Bind peer identity via TOFU pin of PQ KEM fingerprint; reject mismatches deterministically and avoid session mutation.
  - **Invariants:**
    - Fingerprint uses peer PQ KEM public key; no raw key output.
    - Mismatch rejects without overwriting existing pin or session state.
    - No secrets in markers/UI/logs.
  - **References:** NA-0100; PR #217 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/217); qsl/qsl-client/qsc/tests/identity_binding.rs

- **ID:** D-0182
  - **Status:** Accepted
  - **Date:** 2026-02-06
  - **Goals:** G3, G4, G5
  - **Decision:** Add identity UX commands (show/rotate/peers list) with deterministic markers and explicit rotate confirmation; no silent identity changes.
  - **Invariants:**
    - Rotate requires explicit confirmation; no mutation on reject.
    - Outputs contain only fingerprints; no secrets.
    - Deterministic markers for show/rotate/list.
  - **References:** NA-0102; tests/NA-0102_identity_ux_plan.md

- **ID:** D-0183
  - **Status:** Accepted
  - **Date:** 2026-02-07
  - **Goals:** G3, G4, G5
  - **Decision:** Implement NA-0102 identity UX commands and tests with explicit rotate confirmation and deterministic peer list ordering.
  - **Invariants:**
    - Rotate requires --confirm; no mutation on reject.
    - Outputs contain only fingerprints; no secrets.
    - Deterministic markers for show/rotate/list.
  - **References:** NA-0102; PR #220 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/220); qsl/qsl-client/qsc/tests/identity_ux.rs

- **ID:** D-0184
  - **Status:** Accepted
  - **Date:** 2026-02-07
  - **Goals:** G3, G4, G5
  - **Decision:** Add a metadata minimization lane (explicit-only) with bounded polling, padding/bucketing, and batching; deterministic demo mode for CI.
  - **Invariants:**
    - Explicit-only features; no hidden background behavior.
    - Deterministic mode is seeded and safe-to-share.
    - Padding/bucketing and batching are bounded.
    - No secrets in markers/logs.
  - **References:** NA-0103; tests/NA-0103_metadata_minimization_plan.md

- **ID:** D-0185
  - **Status:** Accepted
  - **Date:** 2026-02-07
  - **Goals:** G3, G4, G5
  - **Decision:** Implement NA-0103 metadata-minimization controls (bounded polling and explicit padding/bucketing) with deterministic markers and tests.
  - **Invariants:**
    - Explicit-only polling and padding; bounded parameters.
    - Deterministic markers for cadence and padding length.
    - No secrets in outputs.
  - **References:** NA-0103; PR #223 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/223); qsl/qsl-client/qsc/tests/meta_min.rs

- **ID:** D-0186
  - **Status:** Accepted
  - **Date:** 2026-02-07
  - **Goals:** G3, G4, G5
  - **Decision:** Adopt TUI Layout v2 (H3 inspector drawer) to reduce clutter and improve inspectability with deterministic headless tests.
  - **Invariants:**
    - Home screen has a single scroll region (timeline/chat).
    - Inspector is a single switchable pane (Status/Events/Session/Contacts).
    - No QSC_MARK to stdout in interactive mode; no secrets in UI.
  - **References:** NA-0104; tests/NA-0104_tui_layout_h3_plan.md

- **ID:** D-0187
  - **Status:** Accepted
  - **Date:** 2026-02-07
  - **Goals:** G3, G4, G5
  - **Decision:** Implement NA-0104 H3 home layout with switchable inspector, responsive contact auto-hide, and Enter-to-focus mapping backed by deterministic headless tests.
  - **Invariants:**
    - Home mode keeps timeline/chat as the only scroll region.
    - Inspector switching is explicit (F2-F5 or `/inspector ...`).
    - Interactive mode keeps marker routing in-app (no stdout marker spam).
    - No secrets in markers or UI output.
  - **References:** NA-0104; PR #227 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/227); qsl/qsl-client/qsc/tests/tui_layout_h3.rs

- **ID:** D-0188
  - **Status:** Accepted
  - **Date:** 2026-02-07
  - **Goals:** G3, G4, G5
  - **Decision:** Clarify inspector/focus controls: home F2-F5 switches inspector summary only, while Ctrl+F2-F5 jumps directly to full-screen focus; add `/ins` alias for terminal portability.
  - **Invariants:**
    - Home inspector remains summary-only (no extra home scroll panes).
    - Ctrl+F2-F5 maps to existing focus panes deterministically when delivered by terminal.
    - No secrets in markers or UI output.
  - **References:** NA-0104; PR #229 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/229); qsl/qsl-client/qsc/tests/tui_inspector_keys.rs

- **ID:** D-0189
  - **Status:** Accepted
  - **Date:** 2026-02-07
  - **Goals:** G3, G4, G5
  - **Decision:** Prioritize truthful protocol ACTIVE semantics by requiring validated stored sessions and removing production seed/synthetic fallback; defer identity-secret-at-rest hardening to a dedicated follow-on lane.
  - **Invariants:**
    - ACTIVE requires validated session state, not synthetic derivation.
    - Missing/invalid session remains deterministic INACTIVE with explicit reason.
    - Send/receive remain fail-closed when INACTIVE.
  - **References:** NA-0105; NA-0106; tests/NA-0105_truthful_active_session_only_plan.md; tests/NA-0106_identity_secret_at_rest_plan.md

- **ID:** D-0190
  - **Status:** Accepted
  - **Date:** 2026-02-07
  - **Goals:** G3, G4, G5
  - **Decision:** Implement NA-0105 by making protocol status peer-scoped and ACTIVE only on validated session load; keep seed synthetic session behind explicit test-only override.
  - **Invariants:**
    - Seed-only paths cannot claim ACTIVE in production status markers.
    - Invalid/corrupt session files resolve to deterministic INACTIVE `session_invalid`.
    - Peer-scoped send/receive gates remain fail-closed unless ACTIVE.
  - **References:** NA-0105; PR #231 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/231); qsl/qsl-client/qsc/tests/qsp_status_truthy.rs

- **ID:** D-0191
  - **Status:** Accepted
  - **Date:** 2026-02-07
  - **Goals:** G3, G4, G5
  - **Decision:** Prioritize NA-0106 as release-blocking security hygiene so identity private-key material is not stored plaintext at rest, with deterministic migration from legacy storage.
  - **Invariants:**
    - Identity secret material must not remain plaintext on disk.
    - Migration must be explicit, deterministic, and fail-closed.
    - No secret leakage in logs, markers, or UI output.
  - **References:** NA-0106; tests/NA-0106_identity_secret_at_rest_plan.md

- **ID:** D-0192
  - **Status:** Accepted
  - **Date:** 2026-02-07
  - **Goals:** G3, G4, G5
  - **Decision:** Implement NA-0106 by moving identity `kem_sk` storage to encrypted vault secrets with deterministic legacy migration and fail-closed behavior when vault access is unavailable.
  - **Invariants:**
    - `kem_sk` is not persisted in plaintext identity files.
    - Legacy plaintext identities are migrated only after successful vault import.
    - Failed migration leaves legacy files unchanged (no-mutation on reject).
    - No secret leakage in markers, logs, or UI output.
  - **References:** NA-0106; PR #234 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/234); qsl/qsl-client/qsc/tests/identity_secret_at_rest.rs

- **ID:** D-0193
  - **Status:** Accepted
  - **Date:** 2026-02-08
  - **Goals:** G3, G4, G5
  - **Decision:** Prioritize NA-0101 so PQ signature identity (ML-DSA) and signed handshake binding are the next ready security lane, while preserving existing TOFU fail-closed behavior.
  - **Invariants:**
    - ML-DSA identity signing must not weaken current fail-closed session gating.
    - TOFU mismatch refusal remains in force until signed-identity validation is fully implemented.
    - No silent downgrade to unsigned identity acceptance.
  - **References:** NA-0101; tests/NA-0101_pq_signature_identity_plan.md

- **ID:** D-0194
  - **Status:** Accepted
  - **Date:** 2026-02-08
  - **Goals:** G3, G4, G5
  - **Decision:** Prioritize NA-0107 as release-blocking for remote relay reliability by integrating optional bearer auth in qsc relay inbox client, without weakening fail-closed status guarantees.
  - **Invariants:**
    - Bearer token handling is optional and env-driven; open relay behavior remains unchanged when unset.
    - Tokens are never emitted in markers/logs/UI/artifacts.
    - Unauthorized responses are deterministic and non-mutating.
  - **References:** NA-0107; tests/NA-0107_remote_relay_auth_header_plan.md

- **ID:** D-0195
  - **Status:** Accepted
  - **Date:** 2026-02-08
  - **Goals:** G3, G4, G5
  - **Decision:** Implement NA-0107 by adding optional bearer authorization headers for relay inbox push/pull and deterministic unauthorized error handling, while preserving no-secret output invariants.
  - **Invariants:**
    - Token precedence remains deterministic (`QSC_RELAY_TOKEN` then `RELAY_TOKEN`) and never leaks to logs/markers.
    - Relay 401/403 maps to deterministic `relay_unauthorized`.
    - Unset token environment keeps open-relay behavior unchanged.
  - **References:** NA-0107; PR #243 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/243); qsl/qsl-client/qsc/tests/relay_auth_header.rs

- **ID:** D-0196
  - **Status:** Accepted
  - **Date:** 2026-02-08
  - **Goals:** G3, G4, G5
  - **Decision:** Add NA-0108 as a separate remote handshake evidence lane, distinct from seed-fallback relay smoke, to prove real handshake-established ACTIVE behavior before bidirectional send/receive checks.
  - **Invariants:**
    - Lane never sets `QSC_ALLOW_SEED_FALLBACK`.
    - Workflow trigger policy remains manual/nightly only (no `pull_request` trigger).
    - Lane is fail-closed for `protocol_inactive`, `relay_unauthorized`, missing `qsp_pack`/`qsp_unpack`, or zero receive commits.
    - Artifacts remain safe-to-share with deterministic redaction/normalization.
  - **References:** NA-0108; tests/NA-0108_remote_handshake_lane_plan.md; docs/qsc/DOC-QSC-006_Remote_Relay_Testing_Contract_v1.0.0_DRAFT.md

- **ID:** D-0197
  - **Status:** Accepted
  - **Date:** 2026-02-08
  - **Goals:** G3, G4, G5
  - **Decision:** Expand NA-0108 scope minimally to include `qsl/qsl-client/qsc/**` for receive-path mailbox/peer separation so relay mailbox routing and protocol session lookup are explicit and fail-closed.
  - **Invariants:**
    - Relay inbox pull channel selection is explicit and distinct from protocol peer/session key.
    - Remote handshake lane remains no-seed-fallback and manual/nightly only.
    - Deterministic error/no-mutation guarantees remain unchanged.
  - **References:** NA-0108; CODEX DIRECTIVE 0265

- **ID:** D-0198
  - **Status:** Accepted
  - **Date:** 2026-02-08
  - **Goals:** G3, G4, G5
  - **Decision:** Implement `qsc receive --mailbox` as explicit relay inbox selector while retaining `--from` for protocol peer/session context, then update remote-handshake lane to use mailbox/peer separation with fail-closed checks.
  - **Invariants:**
    - Relay pull channel and protocol decrypt/session peer are modeled as separate inputs.
    - Backward compatibility is preserved when `--mailbox` is omitted (default self label when available, else previous `--from` behavior).
    - No seed fallback usage is introduced in remote-handshake lane.
    - Repo-root vault artifacts are prevented by scoped per-peer config roots.
  - **References:** NA-0108; CODEX DIRECTIVE 0265; qsl/qsl-client/qsc/src/main.rs; scripts/demo/qsc_remote_handshake_smoke.sh

- **ID:** D-0199
  - **Status:** Accepted
  - **Date:** 2026-02-08
  - **Goals:** G3, G4, G5
  - **Decision:** Session/ratchet state at rest is release-blocking and must be encrypted with integrity verification; plaintext session/ratchet key material on disk is not allowed.
  - **Invariants:**
    - Session/ratchet key material is never persisted plaintext.
    - Load path must verify integrity before use; tamper is deterministically rejected with no mutation.
    - Vault/secret-unavailable path is fail-closed (no ACTIVE-by-handshake promotion; send/receive deterministic refusal).
    - Legacy migration is safe and idempotent.
  - **References:** NA-0109; tests/NA-0109_session_state_at_rest_plan.md; CODEX DIRECTIVE 0266

- **ID:** D-0200
  - **Status:** Accepted
  - **Date:** 2026-02-08
  - **Goals:** G1, G2
  - **Decision:** Implement NA-0109 by storing session/ratchet state only as encrypted, integrity-protected blobs with deterministic fail-closed load/migration behavior, and prevent vault path fallback to current working directory to avoid repo-root artifacts.
  - **Invariants:**
    - Session/ratchet state at rest is persisted in encrypted `.qsv` blobs only; plaintext secret-bearing session files are not retained.
    - Decrypt/integrity failures are deterministic and non-mutating.
    - Legacy plaintext migration is idempotent and blocked deterministically when vault access is unavailable.
    - Vault storage path defaults to `QSC_CONFIG_DIR` or XDG/HOME config path; no implicit `.` fallback.
  - **References:** NA-0109; PR #255 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/255); qsl/qsl-client/qsc/tests/session_state_at_rest.rs; qsl/qsl-client/qsc/tests/vault.rs

- **ID:** D-0201
  - **Status:** Accepted
  - **Date:** 2026-02-08
  - **Goals:** G4, G5
  - **Decision:** Adopt a proof-first provenance light-touch baseline by adding `NOTICE`, `PROVENANCE.md`, and a signed-releases runbook so users can verify source and release authenticity without introducing code or CI behavior changes.
  - **Invariants:**
    - Provenance guidance is documentation-only and does not alter protocol/client/server behavior.
    - Official proof references distinguish handshake proof (`remote-handshake-tests`) from transport health (`remote-relay-tests`, `seed_fallback_test`).
    - Guidance must not require trusting unaudited binaries and must point users to traceable source + CI evidence.
  - **References:** NA-0110; tests/NA-0110_provenance_lighttouch_plan.md

- **ID:** D-0202
  - **Status:** Accepted
  - **Date:** 2026-02-08
  - **Goals:** G1, G2, G5
  - **Decision:** Client lifecycle hardening for qsc is release-blocking: secrets must not leak through runtime output or crash paths, and client state handling must remain fail-closed with no CWD artifacts from startup through shutdown.
  - **Invariants:**
    - Secrets are never emitted on stdout/stderr/markers/logs, including panic and error paths.
    - Lifecycle reject paths are deterministic and non-mutating (fail-closed by default).
    - Panic/backtrace posture is hardened for release operation and secret-bearing panic text is disallowed.
    - Safe-parent-verified config/state roots are mandatory; writes to CWD/repo root are rejected.
    - Secret lifetime is minimized (zeroize ASAP, avoid unnecessary copies), and shutdown removes transient plaintext artifacts.
  - **References:** NA-0111; tests/NA-0111_client_lifecycle_hardening_plan.md

- **ID:** D-0203
  - **Status:** Accepted
  - **Date:** 2026-02-09
  - **Goals:** G1, G2, G5
  - **Decision:** Implement NA-0111 lifecycle hardening in qsc with a startup panic redaction hook, explicit panic-demo verification path, and regression coverage that enforces no secret leakage and no CWD artifact creation for common client commands.
  - **Invariants:**
    - Panics emit only deterministic redacted marker `event=panic code=panic_redacted`; panic payload and backtrace content are not emitted.
    - Lifecycle verification command `qsc util panic-demo` is explicit-only and test-focused.
    - Common read/diagnostic command paths do not write artifacts into current working directory.
    - Output scanning tests reject secret sentinel and token-like leakage across stdout/stderr.
  - **References:** NA-0111; PR #261 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/261); `qsl/qsl-client/qsc/tests/lifecycle.rs`; `tests/NA-0111_client_lifecycle_hardening_plan.md`

- **ID:** D-0204
  - **Status:** Accepted
  - **Date:** 2026-02-09
  - **Goals:** G2, G5
  - **Decision:** Metadata minimization Phase 2 in qsc is enforced through explicit, bounded mechanisms (deterministic scheduling, size bucketing, bounded batching, and optional marked cover traffic), with no silent background cover traffic.
  - **Invariants:**
    - Deterministic mode uses fixed-interval polling and explicit marker emission for each tick/bucket/batch decision.
    - Payload-size shaping is explicit and bounded; no unbounded padding behavior.
    - Batch controls enforce max count/max latency bounds fail-closed.
    - Cover traffic is opt-in, bounded, deterministic, and visibly marked when enabled.
    - No implicit retries or hidden recovery behavior in TUI/CLI paths.
  - **References:** NA-0112; `tests/NA-0112_metadata_minimization_phase2_plan.md`

- **ID:** D-0205
  - **Status:** Accepted
  - **Date:** 2026-02-09
  - **Goals:** G2, G5
  - **Decision:** Implement NA-0112 in qsc with explicit `meta plan` dry-run controls plus deterministic tick/batch/bucket marker emission for receive and marker-only bucket reporting for send/relay, while keeping cover traffic explicit plan-only and preserving wire-format behavior.
  - **Invariants:**
    - `qsc meta plan` performs no network I/O and no disk writes; it emits deterministic markers only.
    - Receive metadata scheduling is bounded and deterministic under explicit flags, with fail-closed bound validation.
    - Send/relay metadata bucketing in Phase 2 is marker-only and does not alter ciphertext/wire format.
    - Regression tests enforce deterministic marker ordering, bounded reject/no-mutation behavior, and no-secret output guarantees.
  - **References:** NA-0112; PR #264 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/264); `qsl/qsl-client/qsc/tests/meta_phase2.rs`; `tests/NA-0112_metadata_minimization_phase2_plan.md`

- **ID:** D-0206
  - **Status:** Accepted
  - **Date:** 2026-02-09
  - **Goals:** G2, G5
  - **Decision:** Delivered receipts are client-generated ACKs emitted only after successful peer decrypt/unpack; server-generated delivered receipts are forbidden. Receipt behavior is explicit-only, camouflaged within bounded small-message buckets, and deterministic/test-backed.
  - **Invariants:**
    - `delivered_to_relay` and `delivered_to_peer` remain distinct states.
    - Receiver emits ACK only post-`qsp_unpack ok=true`; relay never synthesizes delivered receipts.
    - ACK traffic uses standard encrypted message flow and bounded bucket camouflage; no receipt-only observable class.
    - Receipts default OFF and require explicit CLI/TUI opt-in.
    - Reject/tamper/replay paths are deterministic fail-closed with no silent state mutation.
  - **References:** NA-0113; `tests/NA-0113_delivered_receipts_plan.md`

- **ID:** D-0207
  - **Status:** Accepted
  - **Date:** 2026-02-09
  - **Goals:** G2, G5
  - **Decision:** Implement NA-0113 delivered receipts in qsc as encrypted client ACK control messages with explicit send/receive flags, deterministic receipt markers, and ACK control-message consumption that does not create `recv_*.bin` artifacts.
  - **Invariants:**
    - `qsc send --receipt delivered` requests delivery ACK; default remains receipt-disabled.
    - `qsc receive --emit-receipts delivered` emits ACK only after successful decrypt/unpack of receipt-requested data payloads.
    - ACK messages are camouflaged via standard small-class padding policy (`bucket=small`) and bounded output length.
    - ACK consumption emits `receipt_recv`/`delivered_to_peer` markers and skips normal file output write path.
    - Output remains secret-safe; marker fields keep `msg_id` redacted.
  - **References:** NA-0113; `qsl/qsl-client/qsc/tests/receipts_delivered.rs`; `tests/NA-0113_delivered_receipts_plan.md`

- **ID:** D-0208
  - **Status:** Accepted
  - **Date:** 2026-02-09
  - **Goals:** G2, G5
  - **Decision:** TUI is a security lens: readability and deterministic behavior are prioritized over raw information density. Timestamp rendering must be deterministic in headless tests, and focus panes are full-screen by design.
  - **Invariants:**
    - Home view remains uncluttered with H3 inspector as summary-only and Timeline as the only home scroll region.
    - Focus panes (`Events`, `Status`, `Session`, `Contacts`) use full-screen scrollable layouts; search/filter can be added only where deterministic and bounded.
    - Deterministic/headless coverage must not depend on wall-clock time and must validate stable keybindings and no overflow/panic under small terminal breakpoints.
    - Interactive TUI mode emits no `QSC_MARK` to stdout.
  - **References:** NA-0114; `tests/NA-0114_tui_readability_h3_plan.md`

- **ID:** D-0209
  - **Status:** Accepted
  - **Date:** 2026-02-09
  - **Goals:** G2, G5
  - **Decision:** Implement NA-0114 in qsc with deterministic focus-pane timestamp tokens, full-height focus scrolling markers, and concise stable key-hint text while preserving interactive marker silence on stdout.
  - **Invariants:**
    - Focus-pane render markers include deterministic timestamp bounds (`ts_start`, `ts_end`) and full-viewport/scroll metadata (`viewport=full`, `scroll`, `view_rows`).
    - Focus panes (`Events`, `Status`, `Session`, `Contacts`) share consistent Up/Down/PgUp/PgDn navigation behavior in focus mode.
    - Home/focus hints remain concise and stable for `F2-F5`, `Ctrl+F2-F5`, `Enter`, `Esc`, `/help`.
    - Interactive test mode remains marker-silent on stdout (`QSC_MARK` not printed).
  - **References:** NA-0114; `qsl/qsl-client/qsc/tests/tui_readability.rs`; `tests/NA-0114_tui_readability_h3_plan.md`

- **ID:** D-0210
  - **Status:** Accepted
  - **Date:** 2026-02-09
  - **Goals:** G2, G5
  - **Decision:** P0 usable-client sequence is unlock -> contacts -> timeline -> message states -> file transfer; security invariants are not weakened for UX.
  - **Invariants:**
    - Queue discipline is preserved by keeping NA-0115..NA-0119 in BACKLOG (no READY promotion in this governance step).
    - Each phase is client-only and fail-closed by default with deterministic, test-backed behavior.
    - No phase may bypass lock, trust, at-rest protection, truthful state semantics, or bounded integrity checks.
  - **References:** NA-0115; NA-0116; NA-0117; NA-0118; NA-0119

- **ID:** D-0211
  - **Status:** Accepted
  - **Date:** 2026-02-10
  - **Goals:** G2, G5
  - **Decision:** NA-0115 enforces a local unlock gate in qsc: sensitive operations are locked-by-default, require explicit unlock per invocation, and fail-closed with deterministic `vault_locked` markers.
  - **Invariants:**
    - Sensitive operations (`send`/`receive`/`handshake`/`identity rotate` and relay send path) terminate fail-closed when not explicitly unlocked.
    - Lock rejects are deterministic and non-mutating for session/outbox/identity/vault state.
    - Unlock validation is local-only and non-mutating (`vault unlock`), with no relay/server presence signaling.
    - TUI exposes lock posture (`LOCKED`) and blocks sensitive actions while locked.
  - **References:** NA-0115; `qsl/qsl-client/qsc/src/main.rs`; `qsl/qsl-client/qsc/src/vault.rs`; `qsl/qsl-client/qsc/tests/unlock_gate.rs`; `tests/NA-0115_local_unlock_gate_plan.md`

- **ID:** D-0212
  - **Status:** Accepted
  - **Date:** 2026-02-10
  - **Goals:** G2, G5
  - **Decision:** NA-0116 introduces explicit, vault-backed contacts management in qsc (`add/show/list/verify/block/unblock`) with deterministic mismatch/block refusal markers and no plaintext contact pin files.
  - **Invariants:**
    - Trust changes are explicit-only; verify updates require explicit confirmation and reject deterministically otherwise.
    - Blocked peers are fail-closed for handshake/send paths with deterministic `peer_blocked` refusal behavior and no session mutation.
    - Pinned fingerprint mismatch is fail-closed with deterministic `peer_mismatch` markers and no session mutation.
    - Contact state is stored only through vault secret APIs and rendered deterministically (stable list ordering + explicit state markers).
  - **References:** NA-0116; PR #277 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/277); `qsl/qsl-client/qsc/tests/contacts_verify_block.rs`; `tests/NA-0116_contacts_verify_block_plan.md`

- **ID:** D-0213
  - **Status:** Accepted
  - **Date:** 2026-02-10
  - **Goals:** G2, G5
  - **Decision:** NA-0117 introduces a vault-backed encrypted timeline store in qsc with deterministic per-entry counters, explicit timeline CLI operations, and timeline ingestion only on successful send/receive commit paths.
  - **Invariants:**
    - Timeline records are persisted only via vault secret APIs; no plaintext timeline file is written in config storage.
    - Timeline order and timestamps are deterministic (`next_ts` monotonic counter) and list rendering is deterministic.
    - Receive reject/tamper and send no-commit paths do not mutate timeline state.
    - Timeline clear requires explicit confirmation and is fail-closed when vault is unavailable.
  - **References:** NA-0117; `qsl/qsl-client/qsc/tests/timeline_store.rs`; `tests/NA-0117_encrypted_timeline_store_plan.md`

- **ID:** D-0214
  - **Status:** Accepted
  - **Date:** 2026-02-10
  - **Goals:** G2, G5
  - **Decision:** Implement NA-0118 with an explicit honest-delivery message state model in qsc timeline storage (`CREATED`, `SENT`, `RECEIVED`, `DELIVERED`, `FAILED`) where `DELIVERED` requires explicit client ACK, transitions are monotonic and deterministic, and reject paths do not mutate persisted state.
  - **Invariants:**
    - `DELIVERED` is never claimed without an explicit ACK that maps to a known outbound message id for the peer timeline.
    - Invalid/duplicate/unknown transitions emit deterministic `message_state_reject` markers and do not mutate timeline state.
    - Successful transitions emit deterministic `message_state_transition` markers with stable `from/to/id` fields.
    - Timeline state remains vault-backed and encrypted at rest; no plaintext timeline store is introduced.
  - **References:** NA-0118; `qsl/qsl-client/qsc/src/main.rs`; `qsl/qsl-client/qsc/tests/message_state_model.rs`; `tests/NA-0118_message_state_model_plan.md`

- **ID:** D-0215
  - **Status:** Accepted
  - **Date:** 2026-02-10
  - **Goals:** G2, G5
  - **Decision:** Implement NA-0119 file transfer MVP in qsc as client-only chunk+manifest control payloads over existing encrypted transport, with explicit bounded limits, deterministic integrity markers, fail-closed reject behavior, and vault-backed encrypted persistence only.
  - **Invariants:**
    - File transfer send path enforces explicit bounds (`max_file_size`, `chunk_size`, `max_chunks`) before network send and rejects deterministically.
    - Integrity is verified per chunk and per manifest using deterministic hashes bound to peer/session context (`peer`, `file_id`, `size`, `chunk_count`, `chunk_hashes`).
    - Receive-side file transfer reject paths do not commit ratchet/session mutation or timeline transfer mutation.
    - No plaintext file-transfer store is written to config disk outside vault-backed `timeline.json` secret storage.
    - Truthful completion markers and timeline ingest occur only after full manifest verification.
  - **References:** NA-0119; `qsl/qsl-client/qsc/src/main.rs`; `qsl/qsl-client/qsc/tests/file_transfer_mvp.rs`; `tests/NA-0119_file_transfer_mvp_plan.md`

- **ID:** D-0216
  - **Status:** Accepted
  - **Date:** 2026-02-10
  - **Goals:** G2, G5
  - **Decision:** Implement NA-0121 by adopting a unified qsc TUI home layout (left navigation + main panel + full-width command bar) with explicit home focus cycling and no marker-string regressions.
  - **Invariants:**
    - Home layout is deterministic and inspectable: nav is always rendered, exactly one domain is expanded (mapped to inspector selection), and command bar remains full-width.
    - Focus is explicit (`Nav`, `Main`, `Command`) and changes only on explicit key input (`Tab`/`Shift+Tab`), with no focus stealing.
    - Existing deterministic marker contracts remain stable; home render fields preserve prior ordering and append nav/focus metadata without renaming legacy keys.
    - Interactive mode marker-silence and existing security behavior (no implicit send/retry/recover) remain unchanged.
  - **References:** NA-0121; PR #293 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/293); `qsl/qsl-client/qsc/src/main.rs`; `qsl/qsl-client/qsc/tests/tui_unified_layout.rs`

- **ID:** D-0217
  - **Status:** Accepted
  - **Date:** 2026-02-10
  - **Goals:** G2, G5
  - **Decision:** Implement NA-0123 by completing unified TUI Messages + Contacts behavior in qsc: conversation-aware nav/main wiring, explicit unread buffering when Main is unfocused, and command-bar-only action model with deterministic invariant tests.
  - **Invariants:**
    - Messages domain keeps truthful NA-0118-aligned state text and never over-claims delivery state.
    - Unfocused updates are bounded: incoming message events increment unread counters and do not auto-append/scroll the main conversation view until Main focus resumes.
    - Contacts domain presents verification/pinning posture as inspection output; trust-changing and blocking operations remain explicit command-bar actions.
    - Existing deterministic `QSC_MARK/1` marker names are preserved; added TUI markers are additive and deterministic.
  - **References:** NA-0123; PR #300 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/300); `qsl/qsl-client/qsc/src/main.rs`; `qsl/qsl-client/qsc/tests/tui_messages_contacts.rs`

- **ID:** D-0218
  - **Status:** Accepted
  - **Date:** 2026-02-10
  - **Goals:** G2, G5
  - **Decision:** Implement NA-0124 by completing unified TUI Files behavior in qsc with Files-only multi-select, truthful NA-0119-aligned state rendering, and deterministic headless invariant coverage.
  - **Invariants:**
    - Multi-select is enabled only in Files domain; attempts from other domains are blocked deterministically.
    - Files view state text is truthful and does not claim `VERIFIED/COMPLETE` before transfer state is verified.
    - Files update buffering is bounded and focus-safe (no focus stealing, no implicit action execution).
    - Existing deterministic `QSC_MARK/1` marker names remain unchanged; added Files markers are additive and deterministic.
  - **References:** NA-0124; PR #303 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/303); `qsl/qsl-client/qsc/src/main.rs`; `qsl/qsl-client/qsc/tests/tui_files_domain.rs`

- **ID:** D-0219
  - **Status:** Accepted
  - **Date:** 2026-02-10
  - **Goals:** G2, G5
  - **Decision:** Implement NA-0125 by completing unified TUI Keys + Activity + Status behavior in qsc with command-bar-only dangerous operations, bounded activity buffering, and locked-state status redaction backed by deterministic invariant tests.
  - **Invariants:**
    - Keys domain remains inspection-first and command-driven; multi-select is not available outside Files domain.
    - Activity updates are bounded and focus-safe: when Main is unfocused, activity increments unread counters and does not auto-append visible ledger content.
    - Status domain presents snapshot structure while redacting sensitive values in locked state.
    - Existing deterministic `QSC_MARK/1` marker names are preserved; new NA-0125 markers are additive and deterministic.
  - **References:** NA-0125; PR #306 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/306); `qsl/qsl-client/qsc/src/main.rs`; `qsl/qsl-client/qsc/tests/tui_keys_activity_status.rs`

- **ID:** D-0220
  - **Status:** Accepted
  - **Date:** 2026-02-10
  - **Goals:** G2, G5
  - **Decision:** Implement NA-0126 by adding unified TUI Settings + Lock domains in qsc, explicit command-bar lock/unlock UX, and locked-state leakage guardrails validated by deterministic headless invariants.
  - **Invariants:**
    - Settings domain is read-only inspection plus explicit command listings; no inline dangerous action execution is introduced.
    - Lock domain exposes explicit lock/unlock state and command-only transitions; locked state redaction applies across Messages/Files/Keys/Contacts render paths.
    - Left-nav preview leakage is prevented via preview-free domain summaries; view markers explicitly declare `preview=none` for sensitive domains.
    - Existing deterministic `QSC_MARK/1` marker names are preserved; added settings/lock markers are additive and deterministic.
  - **References:** NA-0126; PR #309 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/309); `qsl/qsl-client/qsc/src/main.rs`; `qsl/qsl-client/qsc/tests/tui_settings_lock.rs`

- **ID:** D-0221
  - **Status:** Accepted
  - **Date:** 2026-02-12
  - **Goals:** G2, G5
  - **Decision:** Implement NA-0129 by simplifying qsc TUI chrome to a `QSC` nav brand with minimal command bar hints, adding post-unlock Help/About/Legal inspector panes, and suppressing internal marker/debug lines from normal activity/main rendering paths.
  - **Invariants:**
    - Locked-first NA-0128 behavior remains fail-closed: locked nav remains `Unlock/Exit`, locked command gating rejects `/help` and non-allowlisted commands deterministically.
    - Command bar remains explicit-intent only; unlocked chrome is minimal (`Cmd: /help` when unfocused, `Cmd:` input when focused) without verbose hint banners.
    - Help/About/Legal panes are visible only post-unlock and contain non-secret informational content.
    - Existing deterministic `QSC_MARK/1` marker names remain unchanged; NA-0129 marker fields are additive only.
  - **References:** NA-0129; PR #322 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/322); `qsl/qsl-client/qsc/src/main.rs`; `qsl/qsl-client/qsc/tests/tui_chrome_simplification.rs`

- **ID:** D-0222
  - **Status:** Accepted
  - **Date:** 2026-02-12
  - **Goals:** G2, G5
  - **Decision:** Implement NA-0130 by introducing inactivity auto-lock in qsc TUI with default 10-minute timeout, command-bar timeout controls, and a shared lock transition that clears UI buffers before rendering the locked shell.
  - **Invariants:**
    - Auto-lock is enabled by default while unlocked and transitions to locked state deterministically on inactivity timeout.
    - Any keypress/command input resets the inactivity timer; timeout behavior is deterministic in headless tests via scripted `wait <ms>` clock advancement.
    - Manual `/lock` and inactivity auto-lock share the same lock transition path and emit deterministic `tui_buffer_clear` markers.
    - Locked shell remains fail-closed (`Unlock/Exit` only) and existing deterministic `QSC_MARK/1` marker names are preserved.
  - **References:** NA-0130; PR #325 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/325); `qsl/qsl-client/qsc/src/main.rs`; `qsl/qsl-client/qsc/tests/tui_autolock.rs`

- **ID:** D-0223
  - **Status:** Accepted
  - **Date:** 2026-02-13
  - **Goals:** G2, G5
  - **Decision:** Implement NA-0131 by hardening locked-mode TUI command UX in qsc with explicit locked key allowlisting, visible command echo with steady block cursor, deterministic unlock/init prompt flows, and a non-blocking `/init` wizard state machine.
  - **Invariants:**
    - While locked, only allowlisted keys are accepted (`Up/Down`, `Enter`, `Tab`, `Esc`, `/`, and command editing keys when command focus is active); focus hotkeys (`Ctrl+F*` and inspector/function shortcuts) are ignored.
    - Locked command bar behavior is deterministic: unfocused shows `Cmd:`, focused shows echoed input with a steady block cursor; passphrase entry is masked.
    - `/init` no longer requires all arguments on one line; it advances through visible deterministic wizard steps (`alias` -> `ack` -> `passphrase` -> `confirm`) and remains fail-closed on validation errors.
    - Existing deterministic `QSC_MARK/1` event names are preserved; NA-0131 adds marker fields/events without renaming prior markers.
  - **References:** NA-0131; PR #329 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/329); `qsl/qsl-client/qsc/src/main.rs`; `qsl/qsl-client/qsc/tests/tui_locked_cmd_init_ux.rs`

- **ID:** D-0224
  - **Status:** Accepted
  - **Date:** 2026-02-13
  - **Goals:** G2, G5
  - **Decision:** Implement the NA-0131 follow-up by refining locked init UX into a form-style 4-step wizard in Main (`alias` -> `passphrase` -> `confirm` -> `ack`), gating nav selection markers to Nav focus only, and preserving uppercase command input echo (including headless `/key` simulation).
  - **Invariants:**
    - Nav selection marker rendering is focus-safe and deterministic: exactly one `>` only when `Nav` is focused; zero markers when focus is `Cmd`/`Main`.
    - Wizard content remains pre-unlock safe while visible: explanatory text and field labels are shown, passphrase fields are masked, validation errors are deterministic and rendered directly under the input line.
    - Locked command bar remains explicit and deterministic: wizard-active labels (`Alias/Passphrase/Confirm/Ack`) with steady block cursor when focused; `Esc` cancels to locked shell with no mutation.
    - Existing deterministic `QSC_MARK/1` event names remain unchanged; added marker fields for locked-shell rendering are additive only.
  - **References:** NA-0131; PR #330 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/330); `qsl/qsl-client/qsc/src/main.rs`; `qsl/qsl-client/qsc/tests/tui_locked_cmd_init_ux.rs`

- **ID:** D-0225
  - **Status:** Accepted
  - **Date:** 2026-02-15
  - **Goals:** G2, G5
  - **Decision:** Implement NA-0140 by enforcing deterministic routing of read-only show commands to the Status view (`/status`, `/poll show`, `/autolock show`), keeping set commands on the current view (`/poll set`, `/autolock set`), and replacing dump-style Settings rendering with grouped user-facing sections.
  - **Invariants:**
    - Show commands never render into Settings accidentally and do not alter lock state or focus; they deterministically switch only the active inspector pane to `Status`.
    - Set commands mutate only their own configuration fields and keep the current inspector view; deterministic command feedback is recorded in Status via `last_command_result`.
    - Settings content is constrained to user-meaningful groups (`Lock`, `Auto-lock`, `Polling`, `Commands`) and excludes removed internal-ish fields such as `status_containment`.
    - Existing deterministic `QSC_MARK/1` event names remain unchanged; updated marker fields for settings/status are additive only.
  - **References:** NA-0140; PR #363 (https://github.com/QuantumShieldLabs/qsl-protocol/pull/363); `qsl/qsl-client/qsc/src/main.rs`; `qsl/qsl-client/qsc/tests/tui_command_output_routing.rs`; `qsl/qsl-client/qsc/tests/tui_settings_lock.rs`
