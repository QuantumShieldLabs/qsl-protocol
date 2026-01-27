# QuantumShield Documentation Master Index & Release Packet
Version: v1.0.1 (Draft)
Last Updated: 2025-12-28
Goals: G4

Scope: This document is the authoritative index of QuantumShield/QSL project documents and the minimum release packet required to claim implementation, interop, and security readiness. It defines (a) which documents are authoritative for governance and protocol semantics, and (b) which artifacts constitute a minimum “ship-quality” release.

Classification: Governance (authoritative)

---

## 1. Normative Authority

This program has two distinct “normative” layers:

A) Governance / execution authority (process, ordering, fail-closed workflow)  
B) Protocol authority (wire, semantics, cryptographic behavior)

### 1.1 Governance / Execution Authority (repo-root)

The following repo-root documents are authoritative for *how work is executed* and how drift is prevented:

1) START_HERE.md  
2) GOALS.md  
3) AGENTS.md  
4) PROJECT_CHARTER.md  
5) NEXT_ACTIONS.md  
6) DECISIONS.md  
7) TRACEABILITY.md  

Rule: If any supporting plan/checklist conflicts with this governance spine, the spine governs.  
If ambiguity remains, STOP and fail-closed.

### 1.2 Protocol Authority (wire + semantics)

The following documents are the sole normative sources of protocol truth (wire + semantics):

1) QSP_4_3_2_REVIEWED_FULL.md (Protocol Specification)  
2) QSE_1_8_2_REVIEWED_FULL.md (Envelope / Wire Specification)  
3) DOC-CAN-003_QSP_Suite-2_True_Triple_Ratchet_v5.0.0_DRAFT.md (Suite-2 v5.0 lane; normative intent)  
4) DOC-CAN-004_QSP_SCKA_Sparse_Continuous_Key_Agreement_v1.0.0_DRAFT.md (SCKA; normative intent)  

Rule: If any supporting document conflicts with protocol authority, protocol authority governs.  
Supporting documents must be updated via Errata/ChangeLog where applicable.

### 1.3 Test Authority (what must be tested)

The following document is authoritative for what constitutes adequate testing coverage for Suite-2:

- DOC-TST-005_Suite-2_Conformance_Vector_Categories_v1.0.0_DRAFT.md

Rule: Vector category definitions are binding for CI gating expectations. If the canonical spec evolves, DOC-TST-005 must be updated to remain aligned (fail-closed).

### 1.4 Inputs / Vectors (authoritative test data)

The following inputs are authoritative test data sources:

- inputs/suite2/vectors/*.json  
- inputs/suite2/vectors/README.md  

Rule: When vectors are updated, CI must be updated (if necessary) to parse and gate them fail-closed.

---

## 2. Document Spine Map (Mandatory Reading)

If you read only 8–10 documents, read these in order:

| Priority | Document | Purpose | Normative Scope |
|---:|---|---|---|
| 1 | START_HERE.md | Workflow + fail-closed execution rules | Governance |
| 2 | GOALS.md | Goal taxonomy (G1–G5) | Governance |
| 3 | AGENTS.md | Agent rules / enforcement intent | Governance |
| 4 | PROJECT_CHARTER.md | Scope + claims/constraints | Governance |
| 5 | NEXT_ACTIONS.md | Single ordered work queue | Governance |
| 6 | DECISIONS.md | Binding decisions | Governance |
| 7 | TRACEABILITY.md | Goal → spec → tests → evidence linkage | Governance |
| 8 | DOC-CAN-003 | Suite-2 protocol lane | Protocol (normative intent) |
| 9 | DOC-CAN-004 | SCKA spec | Protocol (normative intent) |
| 10 | DOC-TST-005 | Suite-2 test categories | Test authority |

Rule: No other document may define “what to do next.” NEXT_ACTIONS.md is the only authoritative execution queue.

---

## 3. Release Packet (Minimum Required for a Ship-Quality Release)

A release is considered “ship-quality” only when all items below are present, versioned, and internally consistent:

### 3.1 Governance completeness
- START_HERE.md + NEXT_ACTIONS.md exist and are current.
- GOALS.md / AGENTS.md / PROJECT_CHARTER.md define scope and constraints.
- DECISIONS.md captures all binding security/interop decisions.
- TRACEABILITY.md maps goals → specs → tests → evidence.

### 3.2 Canonical specs completeness
- QSP (Protocol Spec) and QSE (Envelope Spec) are complete, self-contained, and internally consistent.
- Suite-2 (DOC-CAN-003) and SCKA (DOC-CAN-004) are complete enough to implement without using tests as substitute specs.

### 3.3 Test plan + vectors + gates
- DOC-TST-005 enumerates required categories and gating expectations.
- inputs/suite2/vectors contains required vectors with stable schemas.
- CI lanes are green and fail-closed:
  - goal-lint
  - qshield-ci (4a–4d + durability lane as applicable)
  - suite2-ci
- Evidence artifacts are retained for each CI run.

### 3.4 Implementation and interop readiness (as applicable to the release)
- Reference implementation supports the claimed lanes.
- Interop evidence exists (once interop is claimed).
- Durability/rollback defenses are proven by tests and CI evidence.

---

## 4. Document Registry

This section lists project documents grouped by classification. “Status” refers to maturity and release-readiness, not “existence.”

### 4.1 Program Control (Governance)

Repo-root governance spine:

- REPO-ROOT: START_HERE.md — Operational Constitution — Normative (governance) — Status: Active
- REPO-ROOT: NEXT_ACTIONS.md — Authoritative Execution Queue — Normative (governance) — Status: Active
- REPO-ROOT: GOALS.md — Goal taxonomy — Normative (governance) — Status: Active
- REPO-ROOT: AGENTS.md — Agent policy / enforcement intent — Normative (governance) — Status: Active
- REPO-ROOT: PROJECT_CHARTER.md — Scope/claims/constraints — Normative (governance) — Status: Active
- REPO-ROOT: DECISIONS.md — Decision log — Normative (governance) — Status: Active
- REPO-ROOT: TRACEABILITY.md — Goal/spec/test/evidence map — Normative (governance) — Status: Active

Supporting repo-root items:

- REPO-ROOT: CHAT_STARTER.md — Legacy chat starter (do not use; replaced by NEXT_ACTIONS.md) — Supporting — Status: Deprecated
- ARCHIVE: docs/archive/START_HERE_2.md — Historical (superseded by START_HERE.md; intentionally non-operative) — Supporting — Status: Deprecated
- REPO-ROOT: ALL_CHATS.md — Conversation index + starter discipline (non-authoritative) — Supporting — Status: Active
- REPO-ROOT: CHECKLIST_PROTOCOL_CHANGE.md — Change-control checklist (supporting) — Supporting — Status: Active

Developer / tooling notes (supporting):
- REPO-ROOT: README_PHASE4.md — Local/CI quick start — Supporting — Status: Active
- REPO-ROOT: CODEX_RULES.md — Codex-only operational guardrails — Supporting — Status: Active
- REPO-ROOT: FORMAL_VERIFICATION_PLAN.md — Verification roadmap (G4) — Supporting — Status: Active

Master control docs:

- DOC-CTRL-001: Documentation Master Index & Release Packet — v1.0.1 — Normative (governance) — Status: Draft
- DOC-CTRL-002: Cross-Document Errata + ChangeLog — v1.0 — Normative (governance) — Status: Draft

### 4.2 Canonical Specs (Protocol Semantics)

- DOC-CAN-001: QSP Protocol Specification — v4.3.2 — Normative — Status: RC  
  Location: docs/_external/PHASE2_CANONICAL.pointer.json (points to Phase2 canonical bundle)

- DOC-CAN-002: QSE Envelope Specification — v1.8.2 — Normative — Status: RC  
  Location: docs/_external/PHASE2_CANONICAL.pointer.json (points to Phase2 canonical bundle)

- DOC-CAN-003: QSP Suite-2 (True Triple Ratchet) — v5.0.0 — Normative intent — Status: Draft (Implementable)  
  Location: docs/canonical/DOC-CAN-003_QSP_Suite-2_True_Triple_Ratchet_v5.0.0_DRAFT.md

- DOC-CAN-004: QSP SCKA (Sparse Continuous Key Agreement) — v1.0.0 — Normative intent — Status: Draft (Implementable)  
  Location: docs/canonical/DOC-CAN-004_QSP_SCKA_Sparse_Continuous_Key_Agreement_v1.0.0_DRAFT.md

### 4.3 Spec Closure (Support for Shipping / Formalization)

- DOC-SCL-001: Suite Parameter Registry & Deployment Profiles — v1.0 — Supporting — Status: Draft  
  Location: docs/spec-closure/DOC-SCL-001_Suite_Parameter_Registry_Deployment_Profiles_v1.0_DRAFT.md

- DOC-SCL-002: Shared Schemas + Error/Reason Code Registry — v1.0 — Supporting — Status: Draft  
  Locations:
  - docs/spec-closure/DOC-SCL-002_Shared_Schemas_Error_Reason_Code_Registry_v1.0_DRAFT.md
  - docs/schemas/DOC-SCL-002_Shared_Schemas_v1.0.json
  - docs/schemas/DOC-SCL-002_Reason_Codes_v1.0.json

- DOC-SCL-003: Conformance Checklist — v1.0 — Supporting — Status: Draft  
  Location: docs/spec-closure/DOC-SCL-003_Conformance_Checklist_v1.0_DRAFT.md

- DOC-SCL-004: State Persistence & Crash Safety — v1.0 — Supporting — Status: Draft  
  Location: docs/spec-closure/DOC-SCL-004_State_Persistence_Crash_Safety_v1.0_DRAFT.md

### 4.4 Test Plan / Categories (Authoritative for coverage)

- DOC-TST-005: Suite-2 Conformance Vector Categories — v1.0.0 — Test Authority — Status: Draft  
  Location: docs/test/DOC-TST-005_Suite-2_Conformance_Vector_Categories_v1.0.0_DRAFT.md

### 4.5 External bundles (pointer-only; content is authoritative per pointer)

- PHASE2_CANONICAL bundle — referenced by docs/_external/PHASE2_CANONICAL.pointer.json
- PHASE3_SUPPORTING bundle — referenced by docs/_external/PHASE3_SUPPORTING.pointer.json

Rule: pointer targets must be preserved; do not fork “shadow copies” inside the repo without recording a decision and updating traceability.

---

## 5. Governance rules for document changes (summary)

- Any doc change must comply with GOALS.md and AGENTS.md.
- Any change that affects interoperability or security invariants requires updates to:
  - DECISIONS.md (what/why)
  - TRACEABILITY.md (where it is tested and evidenced)

If uncertain, treat the change as governance-relevant and record it.

---

## 6. Deprecation and consolidation policy (fail-closed)

Purpose: prevent drift by ensuring there is **one** authoritative place for workflow, queues, and normative meaning.

### 6.1 What “Deprecated” means
A deprecated document:
- MUST NOT be used to determine “what to do next,” protocol semantics, or compliance requirements.
- MUST begin with an explicit deprecation header:
  - `# DEPRECATED — <replacement>`
  - and a `Goals:` line near the top.
- MUST contain a short “where to look now” redirect to the authoritative replacement.

### 6.2 Allowed deprecation actions
- Mark as deprecated in this registry with a replacement pointer.
- Remove or minimize operational instructions from deprecated docs (preferred), to reduce the risk of stale guidance being followed.
- Retain full historical content only if it is explicitly labeled as a non-operative snapshot.

### 6.3 Consolidation rules (avoid competing roadmaps)
- **NEXT_ACTIONS.md** is the only ordered execution queue.
- **START_HERE.md** is the only workflow constitution.
- Any other “plan,” “starter,” or “roadmap” must be treated as supporting and MUST NOT define a competing queue.

If a supporting document needs to convey an ordering of work, it must instead link to NEXT_ACTIONS.md.

---
End of DOC-CTRL-001
