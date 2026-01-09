# QuantumShield — Conformance Checklist (Protocol + Envelope + Harness Gates)

Doc ID: DOC-SCL-003  
Version: v1.0  
Status: DRAFT  
Last Updated: 2025-12-26  
Audience: Internal (engineering/QA/release), External (implementers; optional publication)  
Normative: **YES** (defines minimum conformance evidence required to claim a conformant implementation)  

Supersedes: Phase3 P3-03 v1.1 (Conformance Checklist)  
**Artifact lineage:** P3-03  
**Category:** Supporting (atomic)  
**Canonical refs (alignment only):** DOC-CAN-001 (QSP 4.3.2), DOC-CAN-002 (QSE 1.8.2)  

---

## 0. Purpose

This checklist defines the **minimum conformance requirements** and the **evidence artifacts** required for a QuantumShield implementation to be considered:

- protocol-conformant (QSP semantics),
- wire-conformant (QSE encoding/decoding), and
- test-conformant (able to pass harness gates as a regression baseline).

This document is designed to be **fail-closed**: if an item is marked MUST and no evidence is available, the implementation is **non-conformant**.

---

## 1. Conformance statement (normative)

An implementation MAY claim “QuantumShield conformant” only if:

1. It implements at least one registered suite/profile from DOC-SCL-001.
2. It implements all MUST requirements in DOC-CAN-001 (QSP) and DOC-CAN-002 (QSE) relevant to the suite/profile.
3. It passes the mandatory evidence gates defined in Section 2 for the claimed profile.

If any supporting document conflicts with the canonical specs, the canonical specs govern.

---

## 2. Mandatory evidence gates (normative)

### 2.1 Baseline CI gates (required)

For the reference repo baseline, the following gates are REQUIRED before merge/tag:

- **ci-4a** — Protocol/Envelope baseline gates (parsing, canonical encoding, harness sanity)
- **ci-4b** — Vector execution (positive + negative vectors)
- **ci-4c** — Interop smoke and expanded interop behaviors
- **ci-4d** — Security profile (ratchet/replay/resource bounds)
- **ci-4d-dur** — Durability profile (snapshot/restore/rollback semantics)

A release tag MUST correspond to a commit where all of the above are green.

### 2.2 Durability case requirements (required)

Durability conformance requires passing, at minimum, the following case families:

- **IT-DUR-001** — snapshot → restart → restore → continue (replay before snapshot rejected; forward progress preserved)
- **IT-DUR-002** — multiple snapshots + restores; post-snapshot replays rejected; forward progress preserved
- **IT-DUR-003** — rollback replay + forward progress correctness
- **IT-DUR-004** — ratchet/epoch boundary rollback does not re-enable acceptance of post-boundary ciphertext
- **IT-DUR-005** — epoch-mismatch behavior under rollback (non-durable replay path) rejects, and forward progress preserved

Implementations MAY exceed this set, but MUST NOT claim durability conformance without at least these.

---

## 3. Checklist items (normative)

### 3.1 Canonical encoding and strict parsing

- MUST reject non-canonical base64url (padding, invalid alphabet, non-minimal representations) for all base64url-encoded inputs.
- MUST enforce QSE envelope length invariants (no length smuggling; header/body lengths match actual bytes).
- MUST fail-closed on parse errors (do not attempt best-effort recovery).

**Evidence (minimum):**
- Passing **ci-4a** and **ci-4b**.

### 3.2 Handshake transcript binding

- MUST bind transcript hashing/confirmation to the correct transcript contents for the selected suite/profile.
- MUST reject any handshake confirmation that does not authenticate the transcript.

**Evidence (minimum):**
- Passing **ci-4b** vectors and any handshake-negative vectors associated with transcript binding.

### 3.3 Message ordering, out-of-order tolerance, and replay rejection

- MUST enforce the suite/profile out-of-order window (e.g., MAX_SKIP) and reject gaps beyond the allowed window.
- MUST reject replay of already-accepted ciphertexts within the accepted window.
- MUST bound skipped-key caches and evict deterministically; eviction MUST fail closed (reject) rather than accept.

**Evidence (minimum):**
- Passing **ci-4c** and **ci-4d**.

### 3.4 Durability and crash safety

- MUST persist session state such that, after crash/restart, the implementation either:
  - restores to a safe state and continues without weakening security properties, or
  - fails closed and forces re-key/re-handshake.
- MUST ensure rollback/snapshot restore cannot be used to re-accept ciphertexts that would otherwise be rejected (replay/epoch mismatch).
- MUST preserve forward progress after restore (new messages must still decrypt successfully).

**Evidence (minimum):**
- Passing **ci-4d-dur** and all required IT-DUR cases.

---

## 4. Evidence artifacts and reporting (normative)

### 4.1 Required artifact outputs

A conformant test run MUST produce artifacts that include:

- run ID
- git commit hash
- per-case results, including reject codes/messages for negative stages
- totals: required prefixes, passed cases count, and ok=true/false at top level

### 4.2 Minimum reporting fields

Reports SHOULD include (and the reference harness does include):

- `required_prefixes`
- `passed_cases`
- `errors` / `warnings`
- per-case `details.stages` showing checkpoint steps and where rejection occurred

---

## 5. Test hooks policy (normative)

Test hooks are **not production features**.

- Implementations MUST ensure that any debug or snapshot/restore hooks are **disabled by default**.
- Debug hooks MUST be gated behind an explicit enable flag intended for CI/testing only (e.g., an environment variable).
- Production builds MUST NOT enable test hooks.

Durability conformance testing may enable these hooks in CI to exercise snapshot/restore behavior.

---

## 6. Appendix: Recommended external validation

For higher assurance beyond baseline conformance:

- Independent implementation interop (at least two distinct codebases)
- Fuzzing of QSE parsers and QSP state machines (coverage-guided)
- Formal invariants for persistence atomicity and monotonic counters (model checking)

