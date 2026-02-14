# NA-0135 Ongoing PQ Ratchet Roadmap Decision

## Scope
This is a docs-only decision package. No protocol/client/server code changes are included.

## A) Current-State Summary (Grounded)

### Current posture from NA-0133
The NA-0133 audit found:
- Ongoing PQ-ratchet equivalence to SPQR: Not established.
- PQ material in handshake/session derivation: present.
- Classical ratchet replay/ordering semantics: established with deterministic reject behavior.

Evidence and labels are documented in:
- `docs/audit/PROTOCOL_SECURITY_AUDIT_NA-0133.md`:
  - "Security Property Table" (FS/PCS classical and PQ rows)
  - "SPQR / Triple Ratchet Gap Analysis"
  - "Findings and Follow-on NAs"

Interpretation for roadmap decisions:
- QSL has meaningful PQ-related handshake/session ingredients today.
- QSL does not yet have an evidence-backed, explicit ongoing PQ ratchet construction with SPQR-equivalent property claims and vector-backed validation.

## B) What SPQR / Triple Ratchet Provides (with Sources)

### Summary
Signal's published direction combines:
- PQXDH for post-quantum-capable initial key agreement.
- SPQR as an ongoing PQ ratchet layer.
- "Triple Ratchet" framing: traditional ratchet plus a PQ ratcheting mechanism to improve long-horizon resilience.

Primary references:
- Signal SPQR announcement: <https://signal.org/blog/spqr/>
- Signal PQXDH specification: <https://signal.org/docs/specifications/pqxdh/>
- NIST PQC conference overview (external): <https://csrc.nist.gov/csrc/media/events/2025/sixth-pqc-standardization-conference/post-quantum%20ratcheting%20for%20signal.pdf>

### Why this matters for QSL
Relative to handshake-only PQ measures, ongoing PQ ratcheting targets stronger long-term compromise recovery and future decryption resistance properties by periodically refreshing PQ contribution during session life.

## C) Candidate Ongoing PQ Strategies

### Candidate 1: SPQR-like sparse PQ ratchet layered onto existing ratchet (Triple Ratchet style)
- Description:
  - Keep current ratchet cadence for day-to-day traffic.
  - Inject scheduled/triggered PQ updates that refresh root state and are transcript-bound.
- Target properties:
  - Classical FS/PCS: preserved/improved.
  - PQ-resilient FS/PCS: strongest path among listed candidates when correctly implemented.
- Bandwidth/latency:
  - Moderate overhead from periodic PQ payloads and extra handshake-like substeps.
- Implementation complexity/risk:
  - High complexity (state machine extensions, compatibility rules, downgrade protections).
- Compatibility/upgrade:
  - Requires explicit capability signaling/versioning and strict downgrade prevention.

### Candidate 2: Periodic PQ rekey epochs (N-messages or time-window)
- Description:
  - Trigger full or mini-rekey events at bounded intervals.
  - PQ refresh is epoch-based rather than continuous sparse updates.
- Target properties:
  - Classical FS/PCS: good if cadence is disciplined.
  - PQ-resilient FS/PCS: improved over handshake-only, weaker granularity than sparse ongoing refresh.
- Bandwidth/latency:
  - Burstier overhead at rekey points; simpler runtime between epochs.
- Implementation complexity/risk:
  - Medium complexity; easier to reason about than sparse interleaving.
- Compatibility/upgrade:
  - Clearer rollout boundaries via epoch/version markers; still requires downgrade hardening.

### Candidate 3: Hybrid staged model (epoch rekey first, then sparse SPQR-like updates)
- Description:
  - Phase 1: ship bounded periodic PQ rekey with strict fail-closed semantics.
  - Phase 2: add sparse SPQR-like updates once vectors/perf posture are mature.
- Target properties:
  - Classical FS/PCS: improved in phase 1, improved further in phase 2.
  - PQ-resilient FS/PCS: partial in phase 1, stronger in phase 2.
- Bandwidth/latency:
  - Controlled initial cost; later incremental cost as sparse layer is introduced.
- Implementation complexity/risk:
  - Medium-high overall, but reduced near-term execution risk via staged delivery.
- Compatibility/upgrade:
  - Best operational control: feature flags/versioning can be staged with explicit gates.

## D) Decision and Rationale

## Decision
Adopt Candidate 3 (hybrid staged model):
- Phase 1 (near-term): periodic PQ rekey epochs as a bounded, test-first MVP.
- Phase 2 (target architecture): SPQR-like sparse ongoing PQ ratchet layered onto the ratchet once phase-1 vectors/operational envelopes are proven.

## Why this direction
Security benefits:
- Improves current "Not established" ongoing PQ status with a concrete, auditable bridge.
- Preserves strict deterministic reject/no-mutation discipline while adding PQ refresh cadence.
- Creates an explicit runway to stronger PQ-resilient PCS/FS claims rather than indefinite handshake-only posture.

Operational costs:
- Avoids immediate full-complexity SPQR-style rollout risk in one jump.
- Enables bounded deployment and performance measurement before committing to full sparse interleaving.

Why not direct full SPQR in one step:
- Higher implementation and verification risk for one NA lane.
- Larger downgrade/versioning surface without intermediate evidence checkpoints.

Goal alignment:
- Not "second best": this is a staged path to full-strength architecture, with hard acceptance gates that prevent a permanent half-state.

## E) Acceptance Criteria for NA-0136 (Implementation Plan NA)

NA-0136 must deliver a concrete design-to-tests plan including:

1. State machine definition:
- Exact session states and transitions for PQ rekey events.
- Clear trigger rules (message-count/time-based) and failure transitions.

2. Message/schedule rules:
- Wire/message semantics for PQ rekey steps (or explicit encapsulation if internal-only first).
- Deterministic cadence and conflict resolution rules for concurrent events.

3. Transcript binding requirements:
- Every PQ refresh step must be transcript/context-bound.
- Binding labels and reject reasons must be explicit and testable.

4. Downgrade/versioning strategy:
- Capability advertisement and negotiation behavior.
- Fail-closed handling for peers lacking required PQ-refresh support when policy requires it.

5. Test-first vector plan (mandatory before code):
- Vectors for claimed classical + PQ FS/PCS boundaries.
- Deterministic reject/no-mutation vectors for invalid/duplicate/out-of-order PQ refresh steps.
- Replay/rollback/rollback-after-crash protections.

6. Performance guardrails:
- Baseline budget targets (message overhead, latency impact bands, client CPU/battery envelope).
- Explicit pass/fail criteria for progressing from phase 1 to phase 2 planning.

7. Stop conditions for implementation:
- If downgrade prevention cannot be made fail-closed, stop.
- If reject paths mutate persistent state, stop.
- If property claims cannot be tied to vectors/evidence, mark "Not established" and stop.

## Follow-on NA framing (non-binding)
- NA-0136 should produce the implementation plan and vector strategy with no code.
- Subsequent implementation NAs should separate:
  - phase-1 periodic PQ rekey implementation,
  - phase-2 sparse SPQR-like layer,
  - independent performance and interoperability validation.
