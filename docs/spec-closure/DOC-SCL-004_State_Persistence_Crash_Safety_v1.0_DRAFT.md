# QuantumShield — State Persistence & Crash Safety Specification

Doc ID: DOC-SCL-004  
Version: v1.0  
Status: DRAFT  
Last Updated: 2025-12-26  
Audience: Internal (engineering/ops), External (implementers; optional publication)  
Normative: **YES** (defines durability, rollback detection, and crash safety requirements)  

Supersedes: Phase3 P3-07 (State Persistence & Crash Safety)  
**Artifact lineage:** P3-07  
**Category:** Supporting (atomic)  
**Canonical refs (alignment only):** DOC-CAN-001 (QSP 4.3.2), DOC-CAN-002 (QSE 1.8.2), DOC-SCL-001 (profiles)  

---

## 0. Purpose

This document specifies **durability requirements** for QuantumShield session state so that crashes, restarts, and snapshot restores:

- do **not** weaken replay protection,
- do **not** re-enable acceptance of post-ratchet ciphertext after rollback, and
- preserve forward progress (the session continues safely) where supported.

Where a platform cannot provide the required guarantees, the implementation MUST fail closed and force a re-key/re-handshake.

---

## 1. Threat model and failure modes

### 1.1 Covered events
- Power loss, process crash, node restart
- Partial writes / torn writes
- Storage reordering without `fsync`/barriers
- VM snapshot/restore rollback (state regression)
- Operator error restoring an older backup

### 1.2 Security goals (normative)
After any covered event, the implementation MUST ensure:

- **G1 (Fail-closed safety):** if state integrity or monotonicity is uncertain, reject and require re-key.
- **G2 (Replay safety):** ciphertext already accepted MUST NOT become acceptable again due to rollback.
- **G3 (Epoch/ratchet safety):** ciphertext from a later ratchet/epoch MUST NOT become acceptable after rollback to an earlier state.
- **G4 (Forward progress):** if state is restored to a consistent checkpoint, new messages MUST decrypt successfully.

---

## 2. Durable state requirements (normative)

### 2.1 Minimum persisted state (per session)
An implementation MUST persist, at minimum:

- session identifier and role (initiator/responder)
- suite/profile identifiers and negotiated parameters
- current ratchet/epoch state sufficient to decrypt next valid message
- skipped-key caches (or a safe representation sufficient to enforce bounds)
- anti-replay state sufficient to reject previously accepted ciphertexts within the configured replay window
- a **monotonic rollback generation / epoch marker** (see Section 3)

### 2.2 Confidentiality and privacy of persisted state
Persisted state MUST:
- avoid writing plaintext message contents,
- avoid writing raw ciphertext bytes when a digest suffices,
- be scoped per session and protected by OS-level access controls,
- be eligible for secure wipe/rotation when a session is closed.

---

## 3. Rollback detection and monotonicity (normative)

### 3.1 Monotonic generation marker
Each persisted checkpoint MUST include a **generation marker** that is monotonic across valid progress.

- The marker MUST be updated atomically with ratchet state (Section 4).
- On startup/restore, if the marker regresses relative to a previously observed marker, the implementation MUST treat this as rollback and **fail closed** (G1), unless an explicit, higher-level recovery protocol is used.

### 3.2 Recommended marker design
A practical marker design is:
- a counter stored in a small, separate “monotonic file” updated with `fsync`, or
- a platform monotonic counter / secure element counter, when available.

If no monotonic storage is available, implementations SHOULD force a re-key on any restore where rollback cannot be ruled out.

---

## 4. Atomic persistence and crash safety (normative)

### 4.1 Atomic commit
Checkpoint persistence MUST be atomic with respect to the durable state set in Section 2.

Acceptable patterns include:
- write new state to a temp file → `fsync` → atomic rename → `fsync` directory
- copy-on-write database transaction with explicit durability settings

### 4.2 Crash recovery
On crash/restart:
- if the last committed checkpoint is complete and integrity-checked, restore it and continue;
- if any invariant check fails (truncation, checksum mismatch, missing components), the implementation MUST fail closed and require re-key.

### 4.3 Integrity checks
Implementations MUST use integrity checks over persisted state, such as:
- checksums (CRC32 is acceptable for corruption detection) plus
- cryptographic MAC/signature if the state is attacker-modifiable in the threat model.

---

## 5. Durable anti-replay (normative)

### 5.1 Requirement
To satisfy G2, an implementation MUST prevent rollback from re-enabling acceptance of previously accepted ciphertexts.

At minimum, it MUST maintain a **replay set** over a bounded window. The replay set MAY be:

- a persistently stored set of ciphertext digests, or
- a persistently stored set of message identifiers derived from authenticated headers.

### 5.2 Digest construction (recommended)
If digests are used:

- use a collision-resistant hash (e.g., SHA-256),
- include session scoping (e.g., H(session_id || ciphertext)),
- store only the digest, not raw ciphertext,
- bound size via eviction policies consistent with QSP resource caps.

### 5.3 Windowing and eviction
Replay retention MUST be at least as long as the expected offline window for the deployment profile.
Eviction MUST be deterministic and MUST fail closed: if the replay window cannot be enforced safely, the implementation MUST reject rather than accept.

---

## 6. Ratchet/epoch rollback safety (normative)

To satisfy G3, implementations MUST ensure that rollback cannot cause acceptance of messages from later epochs.

Two compliant approaches exist:

1. **Monotonic epoch enforcement:** persist a monotonic epoch marker (Section 3) such that restoring an older checkpoint is detected and fails closed; or
2. **Durable replay + epoch binding:** persist sufficient anti-replay and epoch-binding state to ensure post-epoch ciphertexts are rejected after restore.

In either approach, post-restore decryption MUST NOT succeed for ciphertext generated after the restored checkpoint.

---

## 7. Test conformance mapping (informational)

The Phase 4D durability suite exercises the above requirements:

- IT-DUR-001 / IT-DUR-002: consistent restore + forward progress (G4) and replay rejection around snapshots (G2)
- IT-DUR-003: rollback replay rejection across restore (G2) with continued forward progress (G4)
- IT-DUR-004: ratchet/epoch boundary rollback does not re-enable post-boundary ciphertext acceptance (G3)
- IT-DUR-005: epoch-mismatch behavior rejects after rollback (G3), even when durable replay is not the rejection mechanism

---

## 8. Test hooks and operational gating (normative)

Snapshot/restore and debug hooks MUST be gated:

- Disabled by default
- Enabled only when an explicit test flag is set (e.g., `QSL_TEST_HOOKS=1`)
- Not permitted in production deployments

Durability-specific mechanisms used solely for CI (e.g., a disk-backed replay journal) MUST also be gated behind test hooks and MUST NOT be active in normal operation.

---

## 9. Reference implementation status (informational)

As of the Phase 4D durability gates, the reference actor includes a **test-only** disk-backed replay journal used to make rollback replays observable in CI. This is gated behind explicit test enablement and is not intended as a production storage design.

Production-grade deployments SHOULD implement Sections 3–6 with platform-appropriate durable storage and rollback detection.

