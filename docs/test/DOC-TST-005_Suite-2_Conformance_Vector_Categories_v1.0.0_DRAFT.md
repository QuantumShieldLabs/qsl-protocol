# DOC-TST-005 — Suite-2 Conformance Vector Categories
**Version:** v1.0.0 (DRAFT)  
**Status:** Draft  
**Last Updated:** 2025-12-28
Goals: G4 (supports G1–G3)

## 0. Purpose
This document defines the initial **vector categories** required to implement and gate QuantumShield **Suite-2** (DOC-CAN-003) and **SCKA** (DOC-CAN-004).

These categories are intended to be representable using the Phase 4 vector set schema:
- `schemas/qshield.phase4.vector_set.schema.v1.json`

## 1. Vector pack layout (initial)
Vector files are placed under:
- `inputs/suite2/vectors/`

Initial files (implemented):
1) `inputs/suite2/vectors/qshield_suite2_kdf_vectors_v1.json`
2) `inputs/suite2/vectors/qshield_suite2_scka_logic_vectors_v1.json`
3) `inputs/suite2/vectors/qshield_suite2_scka_kem_vectors_v1.json`
4) `inputs/suite2/vectors/qshield_suite2_downgrade_vectors_v1.json`
5) `inputs/suite2/vectors/qshield_suite2_transcript_vectors_v1.json`
6) `inputs/suite2/vectors/qshield_suite2_mk_hybrid_vectors_v1.json`
7) `inputs/suite2/vectors/qshield_suite2_pq_reseed_vectors_v1.json`
8) `inputs/suite2/vectors/qshield_suite2_ooo_replay_vectors_v1.json`

Planned protocol-level files (defined in this document; implemented under NA-0006):
9) `inputs/suite2/vectors/qshield_suite2_crash_restart_vectors_v1.json`

## 2. Category registry
Each category includes:
- **Category ID**
- **Spec mapping**
- **Required cases**
- **Notes**

### CAT-S2-KDF-001 — Suite-2 KDFs (G1)
**Spec mapping:** DOC-CAN-003 §3.3  
**Required cases:**
- `KDF_EC_CK` produces `(CK_ec', ec_mk)`
- `KDF_PQ_CK` produces `(CK_pq', pq_mk)`
- `KDF_HYBRID` produces `mk`
- `KDF_RK_DH` produces `(RK', CK_ec0)`
- `KDF_RK_PQ` produces `RK'`
- `KDF_PQ_RESEED` produces directional `CK_pq_dir0` (A->B and B->A)

**Notes:**
- These vectors are deterministic and do not require a PQ KEM library.

### CAT-SCKA-LOGIC-001 — SCKA monotonicity + one-time consumption logic (G2)
**Spec mapping:** DOC-CAN-004 §§3–6  
**Required cases:**
- Peer ADV monotonic acceptance/rejection:
  - accept if `peer_adv_id > peer_max_adv_id_seen`
  - reject otherwise
- CTXT targeting:
  - reject unknown `pq_target_id`
  - reject consumed `pq_target_id`
  - reject tombstoned `pq_target_id`
  - accept known unconsumed target (logic-only; does not validate KEM output)

**Notes:**
- These are logic-only vectors; cryptographic correctness is covered separately by CAT-SCKA-KEM-001.



### CAT-SCKA-KEM-001 — SCKA KEM cryptographic correctness (G1)
**Spec mapping:** DOC-CAN-004 §§3.3–3.4  
**Required cases:**
- Deterministic ML-KEM-768 keygen + deterministic encapsulation + decapsulation roundtrip:
  - `pq_epoch_ss_in == pq_epoch_ss_out`
  - `pq_ct` deterministic for identical inputs
- Wrong decapsulation key:
  - `pq_epoch_ss_in != pq_epoch_ss_out` (implicit rejection behavior; still fail-closed at higher layers)
- Tampered ciphertext:
  - `pq_epoch_ss_in != pq_epoch_ss_out`
- Invalid input sizes:
  - reject if `d_enc`, `z_enc`, or `m` are not 32 bytes (reason codes)

**Notes:**
- Vectors use deterministic fixtures via `MlKem768::generate_deterministic(d,z)` and `encapsulate_deterministic(m)`.
- This category validates KEM correctness without introducing any production-only debug hooks.

### CAT-S2-DOWNGRADE-001 — Fail-closed Suite-2 selection (G3)
**Spec mapping:** DOC-CAN-003 §§1–2  
**Required cases (implemented):**
- Reject mismatched `protocol_version`/`suite_id` when Suite-2 expected
- Reject negotiation tampering reflected in AD mismatch
- Reject silent fallback paths

**Vectors:** `inputs/suite2/vectors/qshield_suite2_downgrade_vectors_v1.json`


## 2.1 Protocol-level composition categories (implemented/remaining; NA-0006 target)

These categories are intentionally defined *before* implementation so that NA-0006 has a stable, fail-closed test contract.
CAT-S2-TRANSCRIPT-001 and CAT-S2-MK-001 are now implemented and CI-gated.

### CAT-S2-TRANSCRIPT-001 — Transcript binding + negotiation enforcement (G3)
**Spec mapping:** DOC-CAN-003 §§2.2–2.3, §5.1; DOC-CAN-004 §2.4 (binding requirements)

**Required cases:**
- Handshake transcript binding:
  - any mismatch between the encoded message and the AEAD associated data (protocol_version, suite_id, session_id, msg_type, header/body length) MUST reject.
- Negotiation enforcement:
  - if Suite-2 is mutually supported and required, any fallback or suite mismatch MUST reject.
  - if a peer claims Suite-2 support in the transcript but encodes a different suite on-wire, MUST reject.
- SCKA binding:
  - any mismatch between SCKA public fields (ADV/CTXT/target identifiers) and the transcript-bound locations defined in DOC-CAN-004 MUST reject.

**Notes:**
- CAT-S2-DOWNGRADE-001 provides initial fail-closed coverage for suite/version mismatches. CAT-S2-TRANSCRIPT-001 expands this into full end-to-end transcript enforcement across handshake + ratchet messages.

### CAT-S2-MK-001 — Per-message KDF_HYBRID correctness at protocol level (G1)
**Spec mapping:** DOC-CAN-003 §§3.3.5, 7.3

**Required cases:**
- Sender and receiver derive the same per-message body key `mk` for deterministic transcripts.
- `mk` MUST change when either the EC chain key or PQ chain key changes (ratchet step or reseed).
- If either component contribution is mutated (EC or PQ) for a given message number, decrypt MUST fail (fail-closed).

**Vector expectations:**
- Deterministic state seeds and message sequences.
- Either:
  - expected ciphertext/tag for each message, or
  - an expected digest of `mk` (test-only) for each message.

### CAT-S2-RESEED-001 — PQ reseed integration (epoch → pq_chain) (G2, supports G1)
**Spec mapping:** DOC-CAN-003 §§3.3.6, 7.5.3, 8.2; DOC-CAN-004 §§3–6

**Required cases:**
- Accepting a valid new SCKA epoch MUST trigger a PQ reseed event exactly as specified (directional chains, ordering, and labels).
- Reseed MUST be transactional: if the enclosing message fails authentication/decryption, no PQ chain reseed is committed.
- One-time CTXT targeting + tombstoning MUST hold under reseed sequences (no re-use of targets).

### CAT-S2-OOO-001 — Out-of-order, replay, and deterministic reject behavior (G2, supports G3)
**Spec mapping:** DOC-CAN-003 §§6–7 (ratchet accept rules); DOC-CAN-004 §6 (commit gating)

**Required cases:**
- In-window out-of-order messages are accepted exactly per spec.
- Duplicate/replayed messages are rejected deterministically.
- Messages outside the receive window are rejected deterministically.
- Reject outcomes MUST imply no persistent state change (fail-closed).

### CAT-S2-BOUNDARY-001 — Boundary messages + epoch integration (G2, supports G4)
**Spec mapping:** DOC-CAN-003 §§4.3–4.4, 5.1–5.2, 7.5, 8.2; DOC-CAN-004 §§1–3

**Vectors:** `inputs/suite2/vectors/qshield_suite2_boundary_vectors_v1.json`

**Actor op:** `suite2.boundary.run`

**Scope:**
- In-order boundary only (`N == Nr`).
- Fail-closed parsing of PQ_PREFIX fixed sizes and flag invariants.
- Transactional commit after header+body decrypt and SCKA reseed apply.

**Reject codes:**
- `REJECT_S2_BOUNDARY_NOT_IN_ORDER`
- `REJECT_S2_PQPREFIX_PARSE`
- `REJECT_SCKA_CTXT_BAD_CT_LEN`
- `REJECT_S2_HDR_AUTH_FAIL`
- `REJECT_S2_BODY_AUTH_FAIL`
- `REJECT_S2_LOCAL_UNSUPPORTED`

### CAT-S2-ESTABLISH-001 — Suite-2 session establishment mapping (G3, supports G4)
**Spec mapping:** DOC-CAN-003 §6.1–§6.6; DOC-CAN-003 §8.2 (initialization)

**Vectors:** `inputs/suite2/vectors/qshield_suite2_establish_vectors_v1.json`

**Runner:** `scripts/ci/run_suite2_establish_vectors.py`

**Actor ops:** `suite2.establish.run`, `suite2.e2e.send`, `suite2.e2e.recv`

**Scope/constraints:**
- Suite-2 only; `msg_type` enforcement per DOC-CAN-003 §6.2.
- Fail-closed length checks per §6.3/§6.5.
- MUST NOT affect Suite-1/Suite-1B behavior.

**Required cases (minimum):**
- Accept: valid establishment inputs (`session_id`=16, `dh_init`=32, `pq_init_ss`=32, `dh_self_pub`=32, `dh_peer_pub`=32), authenticated prerequisite satisfied, negotiated `(0x0500,0x0002)`; then send 1 non-boundary message (`flags==0`) and receive it.
- Reject: `msg_type != 0x01` → `REJECT_S2_ESTABLISH_BAD_MSG_TYPE`.
- Reject: any bad input length (`session_id`/`dh_init`/`pq_init_ss`/`dh_self_pub`/`dh_peer_pub`) → `REJECT_S2_ESTABLISH_BAD_INPUT_LEN`.
- Reject: unauthenticated prerequisite not satisfied → `REJECT_S2_ESTABLISH_UNAUTHENTICATED`.
- Reject: negotiated `protocol_version/suite_id` not `(0x0500,0x0002)` when Suite-2 policy requires establishment → use DOC-CAN-003 §2/§6.1 rejects (`REJECT_S2_LOCAL_UNSUPPORTED`, `REJECT_S2_PEER_UNSUPPORTED`, `REJECT_S2_SUITE_MISMATCH`).

### CAT-S2-PARSE-001 — Suite-2 ratchet message strict parsing (G4)
**Spec mapping:** DOC-CAN-003 §§4.1–4.4, 5.1; DOC-CAN-004 §1

**Vectors:** `inputs/suite2/vectors/qshield_suite2_parse_vectors_v1.json`

**Actor op:** `suite2.parse.check`

**Scope:**
- Decode raw Suite-2 ratchet message bytes.
- Enforce prefix sizing, hdr/body ciphertext length constraints, and flag invariants → PQ_PREFIX exact sizing.
- Fail-closed: any mismatch rejects before crypto/state.

**Reject codes:**
- `REJECT_S2_PARSE_PREFIX`
- `REJECT_S2_PARSE_HDR_LEN`
- `REJECT_S2_PARSE_BODY_LEN`
- `REJECT_S2_PARSE_FLAGS`
- `REJECT_S2_PQPREFIX_PARSE`

### CAT-S2-E2E-RECV-001 — Suite-2 end-to-end receive on raw wire bytes (G2, supports G4)
**Spec mapping:** DOC-CAN-003 §§7.4–8.2; DOC-CAN-004 §§1–3

**Vectors:** `inputs/suite2/vectors/qshield_suite2_e2e_recv_vectors_v1.json`

**Actor op:** `suite2.e2e.recv`

**Scope:**
- Raw wire bytes → parse → header decrypt → body decrypt → transactional state commit.
- Covers non-boundary, out-of-order receive, and in-order boundary with reseed integration.
- Rejects on parse/flag invariants, auth failures, bounds, and SCKA rejects (fail-closed).

### CAT-S2-INTEROP-001 — Suite-2 interop send→wire→recv (flags==0) (G4)
**Spec mapping:** DOC-CAN-003 §§4.3–4.4, 5.1–5.2, 7.3–7.4, 8.2

**Vectors:** `inputs/suite2/vectors/qshield_suite2_interop_vectors_v1.json`

**Runner:** `scripts/ci/run_suite2_interop_vectors.py`

**Actor ops:** `suite2.e2e.send` + `suite2.e2e.recv`

**Scope:**
- Two-actor exchange using raw wire bytes; non-boundary only (`flags == 0`).
- Validates plaintext round-trip and state advancement symmetry across sender/receiver.

### CAT-S2-INTEROP-XIMPL-001 — Suite-2 cross-implementation wire interop (flags==0) (G4)
**Spec mapping:** DOC-CAN-003 §§4.3–4.4, 5.1–5.2, 7.3–7.4, 8.2

**Vectors:** `inputs/suite2/vectors/qshield_suite2_interop_ximpl_vectors_v1.json`

**Runner:** `scripts/ci/run_suite2_interop_ximpl_vectors.py`

**Actor ops:** `suite2.e2e.send` + `suite2.e2e.recv`

**Scope:**
- Cross-implementation exchange using raw wire bytes; non-boundary only (`flags == 0`).
- Validates plaintext round-trip and state advancement symmetry between independent actors.

**Expected rejects (minimum):**
- `REJECT_S2_LOCAL_UNSUPPORTED`
- `REJECT_S2_AD_MISMATCH`
- `REJECT_S2_MK_MISMATCH`
- `REJECT_S2_BODY_AUTH_FAIL`
- `REJECT_S2_HDR_AUTH_FAIL`

### CAT-S2-CRASH-001 — Crash/restart + rollback-aligned invariants (G2, supports G4)
**Spec mapping:** DOC-CAN-004 §5 (persistence + anti-rollback invariants); DOC-SCL-004 (persistence guidance)

**Vectors:** `inputs/suite2/vectors/qshield_suite2_crash_restart_vectors_v1.json`

**Runner:** `scripts/ci/run_suite2_crash_restart_vectors.py`

**Actor ops:** `suite2.e2e.send`, `suite2.e2e.recv`, `debug_snapshot`, `debug_restore`

**Scope:**
- Suite-2 sessions only; crash/restart via debug snapshot/restore.
- Fail-closed on rollback detection for monotonic SCKA invariants.
- Suite-2 handshake ops are not implemented; CAT-S2-CRASH-001 exercises the ratchet via raw-wire ops.

**Required cases:**
- Crash between “stage” and “commit” results in no committed state change after restart.
- Tombstones and monotonic ADV tracking persist across restart.
- Rollback to an older persisted state is detected and rejected (aligned to IT-DUR-* gates).

**Expected rejects (minimum):**
- durable replay reject (existing durability store)
- rollback/monotonic violation reject (Suite-2 specific; fail-closed)
- `REJECT_SCKA_ROLLBACK_DETECTED`
- `replay (durable)`

**Notes:**
- This category is defined to align vector-level expectations with the durability CI lane and to prevent “Suite-2 bypass” of rollback defenses.


## 3. Required reason code behavior
When vectors expect failure (`ok=false`), they MUST specify `reason_code`. Reason codes should be aligned with the program’s reject taxonomy registry as it stabilizes.

## 4. CI gating (implemented)
CI MUST:
- validate vector set schema correctness
- execute supported categories in the reference actor, fail-closed

**Suite-2 CI executes (fail-closed):**
- CAT-S2-KDF-001 (KDF vectors)
- CAT-S2-TRANSCRIPT-001 (transcript binding vectors)
- CAT-S2-MK-001 (per-message mk hybrid vectors)
- CAT-S2-PQRESEED-001 (PQ reseed vectors)
- CAT-S2-OOO-001 (OOO/replay vectors)
- CAT-S2-BOUNDARY-001 (boundary/epoch vectors)
- CAT-S2-ESTABLISH-001 (establishment mapping vectors)
- CAT-S2-PARSE-001 (strict parse vectors)
- CAT-S2-E2E-RECV-001 (e2e recv vectors)
- CAT-S2-INTEROP-001 (interop vectors)
- CAT-S2-INTEROP-XIMPL-001 (cross-implementation interop vectors)
- CAT-S2-CRASH-001 (crash/restart vectors)
- CAT-SCKA-LOGIC-001 (logic vectors)
- CAT-SCKA-KEM-001 (KEM correctness vectors)
- CAT-S2-DOWNGRADE-001 (downgrade vectors)

**Workflow:** `.github/workflows/suite2.yml` (job: `suite2-vectors`)

Planned next gating (after NA-0006 implements the protocol-level runner):
