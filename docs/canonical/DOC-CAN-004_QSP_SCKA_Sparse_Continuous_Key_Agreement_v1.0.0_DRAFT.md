# DOC-CAN-004 — QSP SCKA: Sparse Continuous Key Agreement (Suite-2 PQ Control Plane)
**Version:** v1.0.0 (DRAFT)  
**Status:** Draft (normative intent; implementable)  
**Last Updated:** 2026-01-04  
**Authority:** Canonical SCKA engine for Suite-2 (`protocol_version = 0x0500`)  
**Goals:** G2, G4 (supports G1)

---

## 0. Scope and goals

SCKA (Sparse Continuous Key Agreement) is the **post-quantum control plane** used by Suite-2 (DOC-CAN-003) to introduce periodic fresh PQ shared secrets (`pq_epoch_ss`) without paying PQ encapsulation overhead on every message.

SCKA provides:

- **Peer ADV monotonicity (G2):** strict acceptance rule for peer advertisements (`pq_adv_id`).
- **One-time CTXT targeting (G2):** each locally advertised receive key may be targeted at most once; re-targeting is rejected.
- **Rollback/crash safety (G2):** persistence invariants are monotonic and fail-closed.
- **Conformance-first behavior (G4):** explicit reason codes and CI-gated vectors (DOC-TST-005).

SCKA outputs `pq_epoch_ss` which is then integrated into Suite-2’s PQ reseed and per-message PQ chain as specified in DOC-CAN-003 (see §3.3.6, §7.5.3).

Non-goals:

- This document does not change Envelope framing or Suite-2 wire semantics; it specifies how to process optional PQ prefix fields carried in Suite-2 ratchet messages.
- This document does not mandate an advertisement cadence; cadence is policy-driven. It specifies **what to do** when ADV/CTXT fields appear and how to do so safely.

---

## 1. Wire-level fields and transcript/AD binding (normative)

SCKA uses two optional PQ prefix features carried in the Suite-2 ratchet prefix (DOC-CAN-003 §4.3):

### 1.1 Advertisement (ADV)

If `FLAG_PQ_ADV` is set, the prefix carries:

- `pq_adv_id` (u32, big-endian): sender-chosen strictly increasing identifier
- `pq_adv_pub` (ML-KEM-768 public key, fixed size)

### 1.2 Ciphertext (CTXT)

If `FLAG_PQ_CTXT` is set, the prefix carries:

- `pq_target_id` (u32, big-endian): identifier of a previously advertised receiver key
- `pq_ct` (ML-KEM-768 ciphertext, fixed size)

### 1.3 Fixed sizes (Suite-2)

The following sizes are normative for Suite-2:

| Item | Size |
|---|---:|
| `pq_adv_id` / `pq_target_id` | 4 bytes |
| ML-KEM-768 public key `pq_adv_pub` | 1184 bytes |
| ML-KEM-768 ciphertext `pq_ct` | 1088 bytes |
| ML-KEM-768 shared secret `pq_epoch_ss` | 32 bytes |

If the implementation cannot parse expected fixed-size fields, it MUST reject (fail-closed).

### 1.4 Transcript / associated-data binding (`pq_bind`)

The PQ prefix fields are public but MUST be integrity-bound via Suite-2 associated data (`pq_bind`), defined in DOC-CAN-003 §5.1.

Normative requirement (SCKA):

- Any SCKA processing MUST treat `(pq_adv_id, pq_adv_pub, pq_target_id, pq_ct)` as transcript-bound inputs via `pq_bind`.
- No SCKA-derived state (peer ADV acceptance, consumption/tombstones) may be committed unless Suite-2 message processing commits (DOC-CAN-003 §8.2).

---

## 2. State (normative)

SCKA state is maintained per session.

### 2.1 Local send state

- `local_next_adv_id` (u32): next local advertisement id to allocate (strictly increasing; never reused)
- `peer_adv` (optional):
  - `peer_adv_id` (u32): most recently accepted peer advertisement id
  - `peer_adv_pub` (1184 bytes): peer ML-KEM-768 public key

### 2.2 Local receive state (advertised key store)

- `advkeys`: mapping `pq_adv_id -> { adv_priv, consumed }`
  - `adv_priv`: ML-KEM-768 secret key (encoding is implementation-defined; MUST be protected and zeroized on delete)
  - `consumed` (bool): whether this local receive key has been targeted/consumed
- `tombstones`: set of `pq_adv_id` that MUST be rejected if targeted again

### 2.3 Peer advertisement state (monotonicity)

- `peer_max_adv_id_seen` (u32): maximum peer `pq_adv_id` accepted so far

All state updates MUST be transactional as described in §5 and DOC-CAN-003 §8.2.

---

## 3. Algorithms (normative)

This section is the normative basis for CAT-SCKA-LOGIC-001 and CAT-SCKA-KEM-001 (DOC-TST-005).

### 3.1 Advertise (local)

`SCKA_AdvertiseLocal()`:

1) Allocate `pq_adv_id := local_next_adv_id + 1`.
   - If allocation would overflow u32, the session MUST be terminated (fail-closed).
2) Generate `(pq_adv_pub, adv_priv) = MLKEM768.KeyGen()` (production randomized keygen).
3) Persist to durable storage **before sending**:
   - `advkeys[pq_adv_id] := { adv_priv, consumed=false }`
   - `local_next_adv_id := pq_adv_id`
   Persistence MUST be atomic with respect to crashes.
4) Emit `FLAG_PQ_ADV` with `(pq_adv_id, pq_adv_pub)`.

If durable storage cannot be updated in step (3), the implementation MUST NOT send ADV.

### 3.2 Process peer advertisement (receive)

`SCKA_ProcessPeerADV(peer_adv_id, peer_adv_pub)`:

- If `len(peer_adv_pub) != 1184`: REJECT with `REJECT_SCKA_ADV_BAD_PUB_LEN`.
- If `peer_adv_id <= peer_max_adv_id_seen`: REJECT with `REJECT_SCKA_ADV_NONMONOTONIC`.
- Otherwise (strictly greater):
  - Stage updates:
    - `peer_max_adv_id_seen := peer_adv_id`
    - `peer_adv := (peer_adv_id, peer_adv_pub)`
  - Do not commit until Suite-2 commit rule succeeds (§5, DOC-CAN-003 §8.2).

### 3.3 Encapsulate to peer (send)

`SCKA_EncapToPeer()`:

Precondition: `peer_adv` is present.

1) Let `(peer_adv_id, peer_adv_pub) := peer_adv`.
2) Compute `(pq_ct, pq_epoch_ss_out) = MLKEM768.Encap(peer_adv_pub)`.
3) Emit `FLAG_PQ_CTXT` with `(pq_target_id = peer_adv_id, pq_ct)` and return `pq_epoch_ss_out` to Suite-2 as the sender-side PQ reseed input (DOC-CAN-003 §7.5.3).

**Deterministic fixtures (conformance only; CAT-SCKA-KEM-001):**

Vectors define deterministic ML-KEM-768 fixtures:

- `MLKEM768.generate_deterministic(d_enc[32], z_enc[32]) -> (pk, sk)`
- `MLKEM768.encapsulate_deterministic(pk, m[32]) -> (pq_ct, pq_epoch_ss)`

Input validation (normative for vector execution):

- If `len(d_enc) != 32`: REJECT with `REJECT_SCKA_KEM_BAD_D`.
- If `len(z_enc) != 32`: REJECT with `REJECT_SCKA_KEM_BAD_Z`.
- If `len(m) != 32`: REJECT with `REJECT_SCKA_KEM_BAD_M`.

Production implementations MUST NOT require deterministic fixtures.

### 3.4 Decapsulate ciphertext targeted to local advertisement (receive)

`SCKA_DecapLocal(pq_target_id, pq_ct)`:

1) If `len(pq_ct) != 1088`: REJECT with `REJECT_SCKA_CTXT_BAD_CT_LEN`.
2) If `pq_target_id` is in `tombstones`: REJECT with `REJECT_SCKA_TARGET_TOMBSTONED`.
3) Lookup `entry = advkeys[pq_target_id]`; if missing: REJECT with `REJECT_SCKA_TARGET_UNKNOWN`.
4) If `entry.consumed == true`: REJECT with `REJECT_SCKA_TARGET_CONSUMED`.
5) Compute `pq_epoch_ss_in = MLKEM768.Decap(entry.adv_priv, pq_ct)` (constant-time).
6) Stage updates (do not commit yet):
   - mark `entry.consumed := true` in the staged state
   - schedule secure deletion/zeroization of `entry.adv_priv` on commit
   - schedule addition of `pq_target_id` to `tombstones` on commit
7) Return `pq_epoch_ss_in` to Suite-2 as the receiver-side PQ reseed input.

**Transactional rule:** Steps (6) and any resulting durable changes MUST commit only if Suite-2 message processing commits (DOC-CAN-003 §8.2). If the message is rejected at any stage (including AEAD failure), all staged SCKA updates MUST be discarded and the durable state MUST remain unchanged.

**CAT-SCKA-KEM-001 negative cases:**
- Wrong decapsulation key or tampered `pq_ct` will (with overwhelming probability) yield `pq_epoch_ss_in != pq_epoch_ss_out`. Protocol-level fail-closed behavior is enforced by the Suite-2 commit rule and transcript binding (DOC-CAN-003 §5.1, §8.2).

### 3.5 Initial epoch mapping from PQXDH-style bundle outputs (normative)

This section defines how PQXDH-style bundle outputs from the base handshake initialize the **SCKA epoch 0** state. This mapping is self-contained and does not require external references.

#### 3.5.1 Inputs (from base handshake)

The base handshake MUST provide:
- `session_id` (16 bytes)
- `dh_init` (32 bytes)
- `pq_init_ss` (32 bytes)
- `dh_self_pub` (32 bytes)
- `dh_peer_pub` (32 bytes)
- `role` (`A` or `B`)
- `authenticated` flag (true only if the transcript binds Suite-2 negotiation + identity)

If any input is missing or has a wrong length, the mapping MUST reject (fail-closed).

#### 3.5.2 Derivation and state mapping

1) Compute the Suite-2 initial root keys (DOC-CAN-003 §8.2):
   - `RK0 = KMAC32(dh_init, "QSP5.0/RK0", session_id || 0x01)`
   - `RK  = KMAC32(RK0, "QSP5.0/RKPQ", pq_init_ss || 0x01)`
2) Initialize SCKA epoch 0 state:
   - `local_next_adv_id = 0`
   - `peer_max_adv_id_seen = 0`
   - `known_targets = {}` (empty set)
   - `consumed_targets = {}` (empty set)
   - `tombstoned_targets = {}` (empty set)
3) Initialize PQ chain seeds in the Suite-2 receive state from `RK` (DOC-CAN-003 §8.2):
   - `CK_pq_send` and `CK_pq_recv` are set based on `role`:
     - If `role = A`: `CK_pq_send = KMAC32(RK, "QSP5.0/PQ0/A->B", 0x01)`, `CK_pq_recv = 0x00..00`
     - If `role = B`: `CK_pq_send = 0x00..00`, `CK_pq_recv = KMAC32(RK, "QSP5.0/PQ0/A->B", 0x01)`

The mapping MUST reject if `authenticated != true`.

#### 3.5.3 Fail-closed rules

- If `session_id`, `dh_init`, `pq_init_ss`, `dh_self_pub`, or `dh_peer_pub` lengths are not exact, REJECT.
- If `role` is not `A` or `B`, REJECT.
- If `authenticated` is false, REJECT.

### 3.6 Reason codes (normative)

Implementations MUST use the following reason codes for conformance vectors:

- `REJECT_SCKA_ADV_NONMONOTONIC`
- `REJECT_SCKA_ADV_BAD_PUB_LEN`
- `REJECT_SCKA_TARGET_UNKNOWN`
- `REJECT_SCKA_TARGET_CONSUMED`
- `REJECT_SCKA_TARGET_TOMBSTONED`
- `REJECT_SCKA_CTXT_BAD_CT_LEN`
- `REJECT_SCKA_KEM_BAD_D`
- `REJECT_SCKA_KEM_BAD_Z`
- `REJECT_SCKA_KEM_BAD_M`

---

## 4. Epoch monotonicity and replay resistance (G2)

Receivers MUST reject:

- peer advertisements that do not strictly increase `peer_max_adv_id_seen` (§3.2)
- ciphertext targeting unknown IDs (§3.4)
- ciphertext targeting consumed IDs (§3.4)
- ciphertext targeting tombstoned IDs (§3.4)

All rejects are fail-closed and MUST produce no durable state changes.

---

## 5. Persistence and rollback safety (G2)

Implementations MUST ensure:

### 5.1 Atomicity

- Adv key creation (alloc + keygen + durable write) is atomic.
- Consumption/tombstoning updates for `pq_target_id` are atomic and commit only after Suite-2 commit (DOC-CAN-003 §8.2).
- Peer advertisement acceptance updates (`peer_max_adv_id_seen`, `peer_adv`) are atomic and commit only after Suite-2 commit.

### 5.2 Anti-rollback invariants

At minimum, the following values MUST be monotonic across restarts:

- `peer_max_adv_id_seen` MUST never decrease.
- `local_next_adv_id` MUST never decrease and IDs MUST never be reused.
- `tombstones` MUST be monotonic (entries only added, never removed) for the lifetime of the session.

If rollback is detected or cannot be ruled out for these values, the session MUST be terminated (fail-closed).

Mechanism is implementation-defined (e.g., authenticated append-only log, monotonic counter, hardware-backed rollback protection). The invariant is normative.

---

## 6. Bounded storage and eviction (normative)

To prevent memory DoS:

- `advkeys`, `peer_adv` history (if any), and `tombstones` MUST be bounded by implementation-defined maxima.
- If evicting an unconsumed `advkeys[pq_adv_id]`, the implementation MUST record a tombstone for `pq_adv_id` (so ciphertext targeting it is rejected).
- If adding a tombstone would exceed the tombstone bound, the session MUST be terminated (fail-closed).

Implementations SHOULD choose bounds such that normal operation never hits these limits.

---

## 7. Conformance categories (G4)

Implementations MUST satisfy SCKA categories in DOC-TST-005:

- **CAT-SCKA-LOGIC-001** (G2): peer ADV monotonicity + CTXT targeting logic.
- **CAT-SCKA-KEM-001** (G1): deterministic ML-KEM-768 fixture correctness.

Reference vectors (authoritative fixtures executed in CI):

- `inputs/suite2/vectors/qshield_suite2_scka_logic_vectors_v1.json`
- `inputs/suite2/vectors/qshield_suite2_scka_kem_vectors_v1.json`
