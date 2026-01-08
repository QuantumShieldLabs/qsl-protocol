# QuantumShield Cross-Document Errata + ChangeLog
Doc ID: DOC-CTRL-002  
Version: v1.0  
Status: DRAFT  
Last Updated: 2025-12-26  
Audience: Internal (governance), External (read-only publication optional)  
Normative: **YES** (for documentation governance only)

## 0. Purpose
This document is the authoritative record of:
- **Errata** (post-publication corrections and clarifications), and
- **ChangeLog** entries (intentional, versioned changes)

across the QuantumShield document set—especially the canonical specifications:
- **QSP_4.3.x** (Protocol Specification)
- **QSE_1.8.x** (Envelope / Wire Specification)

It exists to ensure the project can evolve without silent divergence between:
specifications, reference implementations, test harnesses, vectors, and public-facing materials.

## 1. Scope and authority
### 1.1 Documents covered by this Errata + ChangeLog
**Canonical (source of truth)**
- DOC-CAN-001: QSP Protocol Specification (v4.3.x)
- DOC-CAN-002: QSE Envelope / Wire Specification (v1.8.x)

**Spec closure pack (normative support)**
- DOC-SCL-001: Suite / Parameter Registry & Profiles
- DOC-SCL-002: Error / Reason Code Registry
- DOC-SCL-003: Conformance Checklist
- DOC-SCL-004: State Persistence & Crash Safety Spec

**Evidence and operational documents**
- DOC-TST-001..004 (vectors, negative tests, interop plan, artifact packaging)
- DOC-OPS-001..006 (topology, storage, KMS, OpSec, abuse controls, observability)
- Public-facing docs (whitepaper, quickstart, FAQ) are covered for consistency, but are non-normative.

### 1.2 Conflict rule
If any supporting document conflicts with QSP/QSE, **QSP/QSE govern**, and the conflict MUST be resolved via an Errata entry (and corresponding document updates).

## 2. Definitions
### 2.1 Errata vs ChangeLog
- **Errata**: Corrections or clarifications needed after a document has been published or tagged. Errata may be:
  - *Clerical*: spelling, formatting, broken references (no technical effect)
  - *Clarifying*: removes ambiguity without changing protocol behavior
  - *Corrective*: fixes a technical mistake; may require implementation changes
- **ChangeLog**: A planned, intentional change to behavior, requirements, formats, or policy. A ChangeLog entry MUST correspond to a version bump.

### 2.2 Impact classifications
**Wire impact**
- **WIRE-NONE**: no wire-format change
- **WIRE-CLARIFY**: no change, but clarifies parsing/encoding expectations
- **WIRE-COMPAT**: wire-compatible extension (version negotiation/profile-gated)
- **WIRE-BREAK**: incompatible wire-format change (major version bump required)

**Security impact**
- **SEC-NONE**, **SEC-LOW**, **SEC-MED**, **SEC-HIGH**, **SEC-CRITICAL**

**Interop impact**
- **INT-NONE**: no interop changes expected
- **INT-RISK**: interop risk; requires at least one interop run
- **INT-REQ**: interop required before release

## 3. Governance and publication rules
### 3.1 Evidence requirement
An Errata/ChangeLog entry MUST include one or more of:
- a failing test (vector / negative / harness),
- an interop mismatch,
- a durability failure (rollback/replay/state restore),
- a security review finding, or
- an implementation defect traced to spec ambiguity.

### 3.2 Approval requirement
Any entry that is **WIRE-COMPAT**, **WIRE-BREAK**, or **SEC-HIGH/CRITICAL** MUST be explicitly approved by project maintainers and MUST trigger:
- conformance checklist review,
- vector updates (as applicable),
- and an interop re-run (minimum).

### 3.3 Traceability requirement
Each entry MUST reference one or more:
- Git commit(s) and/or tag(s),
- PR number(s),
- test run ID(s) / artifact IDs, and
- affected QSP/QSE section identifiers.

This ensures the “source of truth” remains auditable.

## 4. Entry format
### 4.1 Errata entry template
Use the following template per entry:

- **Errata ID**: E-YYYYMMDD-XXX  
- **Date**: YYYY-MM-DD  
- **Type**: Clerical | Clarifying | Corrective  
- **Affected documents**: (Doc IDs + versions)  
- **Affected sections**: (e.g., QSP §3.3.1; QSE §2.1)  
- **Summary**: one sentence  
- **Details**: what was wrong / ambiguous  
- **Resolution**: the new normative text or rule  
- **Wire impact**: WIRE-NONE | WIRE-CLARIFY | WIRE-COMPAT | WIRE-BREAK  
- **Security impact**: SEC-*  
- **Interop impact**: INT-*  
- **Implementation impact**: what must change in implementations  
- **Test impact**: vectors/negative tests/harness changes required  
- **References**: commits/tags/PRs/run IDs

### 4.2 ChangeLog entry template
- **Change ID**: C-YYYYMMDD-XXX  
- **Version bump**: (e.g., QSP 4.3.2 → 4.3.3)  
- **Summary**: one sentence  
- **Motivation**: why the change exists  
- **Compatibility**: wire and behavioral compatibility statement  
- **Migration notes**: required changes for implementers  
- **Test/interop requirements**: what must pass prior to publish  
- **References**: commits/tags/PRs/run IDs

## 5. Errata register
> No entries have been recorded yet for this document version. Add entries here as they arise.

### 5.1 Open errata (unresolved)
_None._

### 5.2 Resolved errata (published)
_None._

## 6. ChangeLog register
### 6.1 Document-level ChangeLog (this file)
- **C-20251225-001** — v1.0 — Initial creation of cross-document errata + changelog governance.
- **C-20251226-001** — v1.0 — Phase 4D documentation alignment: add DOC-SCL-003/004 and update spec-closure references for durability gates (IT-DUR-001..005).

### 6.2 Canonical spec ChangeLog (QSP/QSE)
> Populate this section when QSP/QSE are patched (e.g., 4.3.3 / 1.8.3) or when errata are incorporated.

_None._

## 7. Release gating implications
Any of the following automatically escalates required gating prior to publish:
- **WIRE-COMPAT / WIRE-BREAK**
- **SEC-HIGH / SEC-CRITICAL**
- **INT-REQ**
- Any change affecting: transcript binding, replay/rollback semantics, key schedule, canonical encoding, or reject rules.

Minimum evidence expectations for escalated items:
- Updated vectors (positive and/or negative as applicable)
- Updated harness assertions
- At least one interop run (two implementations strongly preferred)
- Durability regression run (rollback/replay/state restore)

## 8. Appendices
### Appendix A: Suggested ID conventions
- Errata: E-YYYYMMDD-### (chronological, per day)
- ChangeLog: C-YYYYMMDD-### (chronological, per day)

### Appendix B: Suggested section naming in QSP/QSE
To maximize traceability, QSP and QSE sections referenced in errata should use stable identifiers (section numbers and anchors), and any renumbering should be captured as a ChangeLog entry.

