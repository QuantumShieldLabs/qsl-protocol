# Audit Charter: Protocol + Metadata

## Purpose
Establish honest, evidence-backed claims for the protocol and metadata security posture.
This audit lane records what is proven, what is partial, and what is not yet established.

## Scope
In scope:
- Handshake properties and transcript binding behavior.
- Ratchet/session evolution behavior and state-machine transitions.
- Delivery/receipt semantics and state-claim correctness.
- Replay, reordering, downgrade, and DoS-relevant protocol surfaces.
- Key lifecycle controls (generation, persistence gates, lock/unlock effects).
- Transport patterns that affect metadata exposure (timing, size, routing identifiers, headers).

Out of scope:
- UI polish and non-security visual ergonomics.
- Server trust assumptions for confidentiality guarantees beyond protocol design intent.
- Product roadmap prioritization unrelated to protocol/metadata risk.

## Rules Of Engagement
- No code fixes during audit NAs.
- Findings must become follow-on NAs with bounded implementation scope.
- Every security claim must be labeled as one of:
  - Established
  - Partially established
  - Not established

## Required Evidence Format
Each claim must include:
- File/line pointers to implementation and/or governance artifacts.
- Tests that enforce the behavior (or explicit statement that no enforcing test exists).
- Deterministic markers used to verify behavior where applicable.
- CI lane names and outcome evidence.
- Reproduction command(s) that an auditor can run.

## Downstream Deliverables
NA-0133 must produce:
- Protocol property table.
- SPQR/Triple Ratchet gap analysis against current behavior.

NA-0134 must produce:
- Metadata leakage matrix.
- Mitigation options with cost/impact table.

## Audit-NA Acceptance Criteria
- Findings list with severity and rationale.
- Follow-on NA recommendations for every material finding.
- No ungrounded claims; all conclusions tied to explicit evidence.
- Explicit list of unknowns and assumptions.
