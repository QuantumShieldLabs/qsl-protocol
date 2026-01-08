# DOC-CAN-003 — QSP Suite-2: True Triple Ratchet (Suite-2 / QSP v5.0)
Goals: G2, G3, G4
**Version:** v5.0.0 (DRAFT)  
**Status:** Draft (normative intent; implementable)  
**Last Updated:** 2026-01-04  
**Authority:** Canonical for `protocol_version = 0x0500` (Suite-2 lane)  
**Goals:** G1, G2, G3, G4

---

## 0. Scope, goals, and non-goals

Suite-2 defines a “True Triple Ratchet” messaging lane that combines:

- a **classical DH ratchet** (X25519) for ongoing forward secrecy,
- a **classical symmetric ratchet** (`CK_ec` → `ec_mk`) per message,
- a **PQ symmetric ratchet** (`CK_pq` → `pq_mk`) per message,
- a **hybrid combiner** `mk = KDF_HYBRID(ec_mk, pq_mk)` used as the per-message AEAD key for the message body.

Suite-2 achieves **always-hybrid per-message message keys** without per-message PQ bandwidth by seeding and reseeding the PQ ratchet using **sparse PQ events**:
- an **initial PQ KEM** contribution at session setup, and
- ongoing **SCKA** (Sparse Continuous Key Agreement) reseed events carried in message prefix fields (DOC-CAN-004).

### 0.1 Goals (normative intent)

Suite-2 is engineered to satisfy:

- **G1:** `mk = KDF_HYBRID(ec_mk, pq_mk)` for every encrypted message body.
- **G2:** PQ reseed events are monotonic, fail-closed, and crash-safe (DOC-CAN-004).
- **G3:** Fail-closed negotiation and downgrade resistance, transcript-bound.
- **G4:** Conformance-first: deterministic KDFs, explicit reject conditions, and CI-gated vector categories.

### 0.2 Non-goals

- Suite-2 does **not** change the transport envelope wire format (QSE 1.8.x); it defines how Suite-2 interprets the existing envelope fields.
- Suite-2 does **not** redefine identity, key transparency, or device authentication. Those system layers are assumed to exist and MUST provide authenticated peer identity before Suite-2 state is committed. Suite-2 defines the **Suite-2-specific transcript binding** and **key schedule** once the base handshake has produced required secrets.

### 0.3 Terms and roles

- **A** = initiator, **B** = responder. Role assignment is fixed for a session.
- **Direction labels:** `A->B` and `B->A` are used for directional KDF domain separation.
- `session_id` is a 16-byte unique session identifier chosen by A (see §4.2).

---

## 1. Versioning, suite namespace, and compatibility (normative)

### 1.1 Protocol and suite identifiers

Suite-2 is identified by:

- `protocol_version = 0x0500` (QSP v5.0 lane)
- `suite_id = 0x0002` (Suite-2 “True Triple Ratchet”)

**Namespace rule (normative):** `suite_id` is interpreted **within** a `protocol_version` namespace.  
`(protocol_version=0x0403, suite_id=0x0002)` (Suite-1B) is distinct from `(protocol_version=0x0500, suite_id=0x0002)` (Suite-2).

### 1.2 Fail-closed compatibility rule

Implementations MUST reject any inbound message where:
- `protocol_version` is unknown or unsupported, or
- `protocol_version` is supported but the given `suite_id` is unknown or unsupported for that `protocol_version`.

No silent fallback is permitted.

---

## 2. Downgrade resistance and capability commitment (normative)

This section is the normative basis for **CAT-S2-DOWNGRADE-001**.

### 2.1 Capability advertisement model

Each endpoint maintains:
- `supports_suite2` (local implementation capability), and
- `policy_require_suite2` (local policy; when true, Suite-2 is mandatory if the peer supports it).

Peer capability **MUST** be determined from an authenticated source (e.g., a signed prekey bundle, authenticated directory, or authenticated handshake extension). If peer capability cannot be authenticated, treat it as **unknown** and fail closed when policy requires Suite-2.

### 2.2 Negotiation check (fail-closed)

Let:
- `local.supports_suite2 ∈ {true,false}`
- `local.policy_require_suite2 ∈ {true,false}`
- `peer.supports_suite2 ∈ {true,false,unknown}`
- `negotiated.protocol_version`, `negotiated.suite_id` be the on-wire negotiated values

**Algorithm (normative):**

1) If `local.policy_require_suite2 = true`:
   - If `local.supports_suite2 != true`, **REJECT** (`REJECT_S2_LOCAL_UNSUPPORTED`).
   - If `peer.supports_suite2 != true`, **REJECT** (`REJECT_S2_PEER_UNSUPPORTED`).
   - If `negotiated.protocol_version != 0x0500` or `negotiated.suite_id != 0x0002`, **REJECT** (`REJECT_S2_SUITE_MISMATCH`).

2) If `local.policy_require_suite2 = false`:
   - If `negotiated.protocol_version = 0x0500` and `negotiated.suite_id = 0x0002`, proceed only if `local.supports_suite2 = true` and `peer.supports_suite2 = true`.
   - Otherwise, negotiation is outside Suite-2 scope; this document makes no claims about other lanes.

**No implicit downgrade:** If both endpoints support Suite-2 and policy requires it, any negotiation outcome other than `(0x0500,0x0002)` MUST be rejected.

### 2.3 Associated-data check for negotiation integrity

All Suite-2 AEAD operations MUST bind `protocol_version` and `suite_id` into associated data (see §5.1).

If a receiver observes an inconsistency between:
- the on-wire negotiated values used to route parsing, and
- the `protocol_version/suite_id` values bound into AEAD AD,
the receiver MUST reject the message and MUST NOT commit any state (`REJECT_S2_AD_MISMATCH`).

---

## 3. Suite-2 key schedule and KDFs (normative)

This section is the normative basis for **CAT-S2-KDF-001**.

### 3.1 Cryptographic primitives

Suite-2 uses the same primitive set as QSP-SUITE-1 unless explicitly replaced:

- Classical DH: **X25519**
- PQ KEM: **ML-KEM-768** (used for initial PQ seed and SCKA)
- AEAD: **AES-256-GCM** with 12-byte nonce, 16-byte tag
- Hash: `H(m) = SHA-512(m)`
- KDF/MAC: **KMAC-256** with domain separation labels

All secrets in this document are 32 bytes unless stated otherwise.

### 3.2 Notation

- `KMAC(key, label, data, outlen)` is KMAC-256 with customization string = `label`.
- `KMAC32(key, label, data)` means `KMAC(key, label, data, 32)`.
- `KMAC64(key, label, data)` means `KMAC(key, label, data, 64)`.
- `u32be(x)` is a 4-byte big-endian encoding of `x`.

### 3.3 KDF definitions

#### 3.3.1 Classical per-message chain KDF (EC)

Input: `CK_ec` (32 bytes)  
Output: `(CK_ec', ec_mk)` (each 32 bytes)

- `CK_ec' = KMAC32(CK_ec, "QSP5.0/CK", [0x01])`
- `ec_mk  = KMAC32(CK_ec, "QSP5.0/MK", [0x02])`

After deriving a message key, the sender/receiver MUST update:
- `CK_ec := CK_ec'`

#### 3.3.2 Root update from DH ratchet

Input: `RK` (32 bytes), `dh_out` (32 bytes from X25519)  
Output: `(RK', CK_ec0)` (each 32 bytes)

- `tmp = KMAC64(RK, "QSP5.0/RKDH", dh_out)`
- `RK'    = tmp[0:32]`
- `CK_ec0 = tmp[32:64]`

`CK_ec0` is the initial classical chain key for the *new* DH epoch (direction determined by which party’s DH public key was advanced).

#### 3.3.3 Root update from PQ shared secret

Input: `RK` (32 bytes), `pq_ss` (bytes)  
Output: `RK'` (32 bytes)

- `RK' = KMAC32(RK, "QSP5.0/RKPQ", pq_ss || [0x01])`

`pq_ss` is a PQ KEM shared secret from either:
- the initial session PQ seed exchange, or
- an SCKA ciphertext event (DOC-CAN-004).

#### 3.3.4 PQ per-message chain KDF

Input: `CK_pq` (32 bytes)  
Output: `(CK_pq', pq_mk)` (each 32 bytes)

- `CK_pq' = KMAC32(CK_pq, "QSP5.0/PQCK", [0x01])`
- `pq_mk  = KMAC32(CK_pq, "QSP5.0/PQMK", [0x02])`

After deriving a message key, the sender/receiver MUST update:
- `CK_pq := CK_pq'`

#### 3.3.5 Hybrid combiner KDF (per-message AEAD key)

Input: `ec_mk` (32 bytes), `pq_mk` (32 bytes)  
Output: `mk` (32 bytes)

- `mk = KMAC32(ec_mk, "QSP5.0/HYBRID", pq_mk || [0x01])`

`mk` is the Suite-2 per-message AEAD key used for the message **body** encryption/decryption.

#### 3.3.6 PQ chain reseed KDF from SCKA ciphertext events

Inputs:
- `RK` (32 bytes) — **the RK value before applying `KDF_RK_PQ` for this event**
- `pq_target_id` (u32) — identifies which local PQ receive key was targeted
- `pq_ct` (bytes) — the on-wire ML-KEM ciphertext
- `pq_epoch_ss` (bytes) — the decapsulation output / encapsulation shared secret for this event

Define:

- `ct_hash = H(pq_ct)[0:32]`
- `ctx = "QSP5.0/SCKA/CTXT" || u32be(pq_target_id) || ct_hash || pq_epoch_ss`

Outputs (32 bytes each):
- `CK_pq_seed_A2B = KMAC32(RK, "QSP5.0/PQSEED/A->B", ctx)`
- `CK_pq_seed_B2A = KMAC32(RK, "QSP5.0/PQSEED/B->A", ctx)`

**Application rule (normative):**
- Compute `(CK_pq_seed_A2B, CK_pq_seed_B2A)` using the current `RK` (call it `RK_old`).
- Then update `RK := KDF_RK_PQ(RK_old, pq_epoch_ss)` (see §3.3.3).
- Then apply the directional PQ seeds according to roles (see §8.5.3).

This ordering is required for conformance with **CAT-S2-KDF-001** vectors.

### 3.4 Directional header keys (ratchet header confidentiality)

Suite-2 uses directional **header keys** derived from `RK` to encrypt/decrypt the ratchet header ciphertext.

Define:

- `HK_A->B(RK)  = KMAC32(RK, "QSP5.0/HK/A->B",  [0x01])`
- `HK_B->A(RK)  = KMAC32(RK, "QSP5.0/HK/B->A",  [0x01])`
- `NHK_A->B(RK) = KMAC32(RK, "QSP5.0/NHK/A->B", [0x01])`
- `NHK_B->A(RK) = KMAC32(RK, "QSP5.0/NHK/B->A", [0x01])`

`HK` is used for non-boundary header encryption. `NHK` is used for **boundary** header encryption (see §8.5.1).

After any update to `RK`, the implementation MUST recompute `HK`/`NHK` for both directions from the new `RK`.

---

## 4. Wire format integration (normative)

Suite-2 uses the existing QuantumShield Envelope (QSE) framing. Only Suite-2 interpretation rules are defined here.

### 4.1 Envelope summary (QSE)

On wire (big-endian):
```
Envelope {
  u16  protocol_version;   // 0x0500 for Suite-2
  u16  suite_id;           // 0x0002 for Suite-2
  u8   msg_type;           // 0x01 handshake, 0x02 ratchet
  u8   flags;              // reserved at envelope layer
  u16  header_len;
  u16  body_len;
  opaque header[header_len];
  opaque body[body_len];
}
```

`protocol_version` and `suite_id` MUST be treated as public metadata and MUST be included in Suite-2 associated data (§5.1).

### 4.2 Session identifier

The initiator MUST generate `session_id` as 16 uniformly random bytes and include it in the authenticated transcript and associated data. `session_id` MUST be unique with overwhelming probability and MUST NOT intentionally be reused.

### 4.3 Ratchet prefix layout (Suite-2)

The Suite-2 ratchet prefix is carried in the ratchet message’s outer (unencrypted) portion and MUST be parsed before header AEAD:

```
RatchetPrefix {
  opaque DH_pub[32];   // X25519 sender public key
  u16 flags;           // Suite-2/QSP ratchet flags (not envelope.flags)
  // optional PQ prefix fields in canonical order:
  // if FLAG_PQ_ADV:  u32 pq_adv_id;  opaque pq_adv_pub[...];
  // if FLAG_PQ_CTXT: u32 pq_target_id; opaque pq_ct[...];
}
```

Suite-2 uses these flag bit assignments:
- `0x0001` = `FLAG_PQ_ADV` (includes new PQ receive key advertisement)
- `0x0002` = `FLAG_PQ_CTXT` (includes PQ encapsulation ciphertext)
- `0x0004` = `FLAG_BOUNDARY` (message performs a DH ratchet boundary)

**Flag invariants (normative):**
- A message with `FLAG_PQ_ADV` MUST also set `FLAG_BOUNDARY`.
- A message with `FLAG_PQ_CTXT` MUST also set `FLAG_BOUNDARY`.
- If `FLAG_PQ_CTXT` is set, the message MUST include `pq_target_id` and `pq_ct` in canonical order.
- Implementations MUST reject any message that violates these invariants.

### 4.4 Header plaintext layout

The decrypted header plaintext MUST be:

```
HeaderPlaintext {
  u32 PN;   // number of messages in sender’s previous chain
  u32 N;    // message number in sender’s current chain (epoch)
}
```

All integer fields are big-endian.

---

## 5. Associated data and nonces (normative)

### 5.1 Associated data (Suite-2)

Associated data MUST bind Suite-2 negotiation and the optional PQ control-plane prefix.

Let `PQ_PREFIX` be the concatenation (in canonical on-wire order) of optional PQ fields present in the ratchet prefix:
- If `FLAG_PQ_ADV` is set: `pq_adv_id || pq_adv_pub`
- If `FLAG_PQ_CTXT` is set: `pq_target_id || pq_ct`

Define:
- `pq_bind = H("QSP5.0/PQ-BIND" || u16(flags) || PQ_PREFIX)[0:32]`

Then:
- `AD_hdr  = session_id || protocol_version || suite_id || DH_pub || u16(flags) || pq_bind`
- `AD_body = session_id || protocol_version || suite_id || pq_bind`

**Integrity requirement (normative):**
- Implementations MUST compute `pq_bind` from the received on-wire `flags` and PQ prefix fields.
- Any modification of PQ prefix fields MUST cause AEAD verification failure and MUST NOT be committed to ratchet state.

Envelope metadata outside the ratchet prefix MUST NOT be included in Suite-2 AEAD AD.

### 5.2 Nonces

Nonces are deterministic and MUST be derived as:

- `nonce_hdr  = H("QSP5.0/HDR-NONCE"  || session_id || DH_pub || u32be(N))[0:12]`
- `nonce_body = H("QSP5.0/BODY-NONCE" || session_id || DH_pub || u32be(N))[0:12]`

Where `N` is the header-plaintext message number for this DH epoch (§8.3).

---

## 6. Suite-2 session establishment and negotiation mapping (normative)

This section defines how Suite-2 establishment is mapped onto the existing QuantumShield Envelope (QSE) and what the base handshake MUST provide to initialize Suite-2. No new wire formats are introduced.

### 6.1 Preconditions and negotiation gating

- A Suite-2 session MUST NOT be established unless the negotiation check in §2 passes for `(protocol_version=0x0500, suite_id=0x0002)`.
- Negotiation inputs MUST be authenticated as required by §2.1. If peer capability is unknown when local policy requires Suite-2, the session MUST be rejected (see §2.2).
- The `negotiated.protocol_version` and `negotiated.suite_id` used for routing MUST match the values bound into Suite-2 AEAD associated data (§5.1). Any mismatch MUST be rejected (`REJECT_S2_AD_MISMATCH`).

### 6.2 Envelope mapping for establishment messages

- `msg_type = 0x01` denotes the base handshake lane used for Suite-2 establishment (within the existing QSE framing).
- `msg_type = 0x02` is used only after establishment for Suite-2 ratchet messages.
- Suite-2 does not change the QSE envelope structure (§4.1); establishment uses the same envelope fields.
- Any inbound message with `(protocol_version=0x0500, suite_id=0x0002)` and an unknown `msg_type` MUST be rejected (`REJECT_S2_ESTABLISH_BAD_MSG_TYPE`).

### 6.3 Base handshake interface contract (Suite-2)

The base authenticated session setup MUST provide the following outputs for Suite-2 initialization:

- `session_id` — 16 bytes, chosen by initiator A (§4.2), and included in the authenticated transcript.
- `dh_init` — 32-byte classical shared secret (X25519 output).
- `pq_init_ss` — 32-byte ML-KEM-768 shared secret.
- `pq_kem_pub_id` — 32-byte identifier for the peer’s PQ KEM public key (e.g., `SHA-256(pq_kem_pub)`).
- `pq_prekey_id` — u32 identifier for the PQ prekey / bundle entry that supplied `pq_kem_pub_id`.
- `dh_self_pub` — 32-byte X25519 public key for the local role (A or B).
- `dh_peer_pub` — 32-byte X25519 public key for the peer role (B or A).

Normative requirements:

- The base handshake MUST authenticate peer identity at the system layer before Suite-2 state is committed (§0.2).
- The base handshake transcript MUST commit to `(protocol_version=0x0500, suite_id=0x0002)` and the `session_id`. If authenticated commitment cannot be provided, the session MUST be rejected (`REJECT_S2_ESTABLISH_UNAUTHENTICATED`).
- The base handshake transcript MUST bind `dh_self_pub` and `dh_peer_pub` to the authenticated transcript; attackers MUST NOT be able to swap these values without detection.
- The base handshake transcript MUST bind `pq_kem_pub_id` and `pq_prekey_id` to the authenticated transcript. Missing or mismatched bindings MUST be rejected (`REJECT_S2_ESTABLISH_PQ_BIND_MISSING` / `REJECT_S2_ESTABLISH_PQ_BIND_MISMATCH`).

### 6.4 Role assignment and initial counters

- Role A (initiator) and Role B (responder) are fixed for the session (§0.3).
- Initiator A chooses `session_id` and the default profile is “initiator sends first,” matching the asymmetric initialization in §8.2.
- Initial `PN/N` counters and any “unset until first boundary” semantics MUST follow §8.2–§8.4.

### 6.5 Initialization algorithm (normative)

Suite-2 MUST initialize state using §8.2 exactly. For clarity, the required input validation is:

- `session_id` MUST be 16 bytes, else reject (`REJECT_S2_ESTABLISH_BAD_INPUT_LEN`).
- `dh_init` MUST be 32 bytes, else reject (`REJECT_S2_ESTABLISH_BAD_INPUT_LEN`).
- `pq_init_ss` MUST be 32 bytes, else reject (`REJECT_S2_ESTABLISH_BAD_INPUT_LEN`).
- `pq_kem_pub_id` MUST be 32 bytes, else reject (`REJECT_S2_ESTABLISH_BAD_INPUT_LEN`).
- `pq_prekey_id` MUST be present, else reject (`REJECT_S2_ESTABLISH_PQ_BIND_MISSING`).
- `dh_self_pub` MUST be 32 bytes, else reject (`REJECT_S2_ESTABLISH_BAD_INPUT_LEN`).
- `dh_peer_pub` MUST be 32 bytes, else reject (`REJECT_S2_ESTABLISH_BAD_INPUT_LEN`).

### 6.6 Fail-closed reject rules for establishment

Implementations MUST reject the establishment attempt if any of the following occur:

- Negotiation mismatch or policy violation (reuse §2.2 rejects): `REJECT_S2_LOCAL_UNSUPPORTED`, `REJECT_S2_PEER_UNSUPPORTED`, `REJECT_S2_SUITE_MISMATCH`.
- AD inconsistency between negotiated routing values and bound values (`REJECT_S2_AD_MISMATCH`).
- Unknown `msg_type` for `(protocol_version=0x0500, suite_id=0x0002)` (`REJECT_S2_ESTABLISH_BAD_MSG_TYPE`).
- Missing or invalid base-handshake outputs (length or presence) (`REJECT_S2_ESTABLISH_BAD_INPUT_LEN`).
- Base handshake cannot provide authenticated commitment to Suite-2 negotiation and `session_id` (`REJECT_S2_ESTABLISH_UNAUTHENTICATED`).
- Missing or mismatched PQ KEM public key / prekey binding (`REJECT_S2_ESTABLISH_PQ_BIND_MISSING`, `REJECT_S2_ESTABLISH_PQ_BIND_MISMATCH`).

## 7. Suite-2 ratchet state (normative)

Suite-2 extends the standard DH + symmetric ratchet with an always-on PQ symmetric ratchet.

Per session:
- `role ∈ {A,B}`
- `session_id` (16 bytes)

Per direction (send/recv), the implementation maintains:

- `RK` (32 bytes) — current root key

Header keys (derived from `RK` per §3.4):
- `HK_s`, `HK_r`, `NHK_s`, `NHK_r` (32 bytes)

DH ratchet:
- `DHs` — current local X25519 keypair (send DH)
- `DHr` — current peer X25519 public key (last received send DH)
- `CK_ec_send`, `CK_ec_recv` (32 bytes each, may be unset before first use)
- counters: `Ns`, `Nr`, `PNs` (u32)

PQ symmetric ratchet:
- `CK_pq_send`, `CK_pq_recv` (32 bytes each, may be unset before first use)

Skipped stores (for out-of-order handling):
- `MKSKIPPED`: map keyed by `(DH_pub, N)` to `mk` with bounds (§9.3)
- `HKSKIPPED`: optional map keyed by `old_DH_pub` to `(HK_r_old, NHK_r_old)` with bounds (§9.3)

**Commit rule (normative):** No durable state is committed unless **both header and body decrypt succeed** and all validations pass.

---

## 8. Suite-2 ratchet algorithms (normative)

### 8.1 Directional header key selection

Derive `HK_A->B`, `HK_B->A`, `NHK_A->B`, `NHK_B->A` from the current `RK` (see §3.4), then map to local send/recv as:

- If `role = A`:
  - `HK_s = HK_A->B`, `HK_r = HK_B->A`
  - `NHK_s = NHK_A->B`, `NHK_r = NHK_B->A`
- If `role = B`:
  - `HK_s = HK_B->A`, `HK_r = HK_A->B`
  - `NHK_s = NHK_B->A`, `NHK_r = NHK_A->B`

### 8.2 Initialization from a base handshake

The base authenticated session setup MUST provide:

- `session_id`
- an initial classical shared secret `dh_init` (X25519 output)
- an initial PQ shared secret `pq_init_ss` (ML-KEM-768 output)

Suite-2 initialization (normative):

1) Set an initial root:
   - `RK0 = KMAC32(dh_init, "QSP5.0/RK0", session_id || [0x01])`
2) Mix in the initial PQ seed:
   - `RK = KDF_RK_PQ(RK0, pq_init_ss)`  (see §3.3.3)
3) Derive `HK_s/HK_r/NHK_s/NHK_r` from `RK` and role (§8.1).
4) Initialize DH state:
   - `DHs_pub := dh_self_pub`
   - `DHr_pub := dh_peer_pub`
5) Initialize chain keys depending on role:

- If `role = A` (initiator):
  - set `CK_ec_send = KMAC32(RK, "QSP5.0/CK0/A->B", [0x01])`
  - set `CK_pq_send = KMAC32(RK, "QSP5.0/PQ0/A->B", [0x01])`
  - `CK_ec_recv`, `CK_pq_recv` are unset until the first inbound ratchet boundary from B establishes them.
- If `role = B` (responder):
  - set `CK_ec_recv = KMAC32(RK, "QSP5.0/CK0/A->B", [0x01])`
  - set `CK_pq_recv = KMAC32(RK, "QSP5.0/PQ0/A->B", [0x01])`
  - `CK_ec_send`, `CK_pq_send` are unset until B performs a send boundary.

This asymmetric initialization matches the typical “initiator sends first” profile.

### 8.3 Sending a message (non-boundary)

Preconditions:
- `CK_ec_send` and `CK_pq_send` MUST be set; otherwise **REJECT/FAIL**.

Algorithm:

1) `(CK_ec_send, ec_mk) = KDF_EC_CK(CK_ec_send)`  (§3.3.1)  
2) `(CK_pq_send, pq_mk) = KDF_PQ_CK(CK_pq_send)`  (§3.3.4)  
3) `mk = KDF_HYBRID(ec_mk, pq_mk)`                 (§3.3.5)  
4) Encrypt body with `mk`, `nonce_body` (§5.2), and `AD_body` (§5.1).  
5) Construct header plaintext containing `PNs` and `Ns` (both u32).  
6) Encrypt header plaintext under `HK_s` with `nonce_hdr` (§5.2) and `AD_hdr` (§5.1).  
7) Send message with `FLAG_BOUNDARY = 0`.

After sending:
- increment `Ns := Ns + 1`.

### 8.4 Receiving a message (non-boundary)

The receiver MUST:

1) Compute `AD_hdr` and `nonce_hdr`; attempt header decryption under `HK_r`, then `NHK_r`, then `HKSKIPPED` candidates (bounded by `MAX_HEADER_ATTEMPTS`).  
2) Recover `(PN, N)` from header plaintext.
3) Enforce bounds (§9.3) and handle out-of-order / skipped keys (§9.1).
4) Derive the correct `(ec_mk, pq_mk, mk)` for this `(DH_pub, N)` by advancing `CK_ec_recv` and `CK_pq_recv` as required, storing intermediate skipped `mk` values in `MKSKIPPED`.
5) Decrypt body with `mk`, `nonce_body`, and `AD_body`.
6) Commit state only after success (§7).

---

### 8.5 Boundary handling

A “boundary” message is any message with `FLAG_BOUNDARY = 1`. Boundaries are the only points where Suite-2 permits:
- DH ratchet advancement, and/or
- application of SCKA reseed events to PQ chain keys.

#### 8.5.1 Boundary header key rule (anti-spoof)

- A boundary message header MUST be encrypted under the sender’s `NHK_s` derived from the **pre-boundary** `RK`.
- The receiver MUST accept a boundary epoch transition only if the header decrypt source is `CURRENT_NHK` (i.e., decrypted using its current `NHK_r`). If the boundary header decrypts under any other candidate key, the receiver MUST reject.

This rule prevents forged “new epoch” transitions.

#### 8.5.2 DH ratchet (boundary without PQ)

When a sender performs a DH ratchet boundary:
1) Save `boundary_hk = NHK_s` (derived from the current `RK`).
2) Generate a new local X25519 keypair and update the on-wire `DH_pub` accordingly.
3) Set `PNs := Ns`, then set `Ns := 0`.
4) Compute `dh_out = X25519(DHs_priv_new, DHr_pub_current)`.
5) `(RK, CK_ec_send) = KDF_RK_DH(RK, dh_out)` (§3.3.2).
6) Reinitialize the PQ send chain from the updated root:
   - If `role = A`, `CK_pq_send := KMAC32(RK, "QSP5.0/PQ0/A->B", [0x01])`
   - If `role = B`, `CK_pq_send := KMAC32(RK, "QSP5.0/PQ0/B->A", [0x01])`
7) Recompute `HK/NHK` from the updated `RK` (§8.1).
8) Encrypt the boundary message header under `boundary_hk` (not the post-update key schedule).
9) Set `FLAG_BOUNDARY = 1`.

Receiver processing when `msg.DH_pub != st.DHr`:
1) Require `hdr_source == CURRENT_NHK` (see §8.5.1).
2) Skip-message processing for the prior epoch using `PN` (header plaintext) and bounded skipped-key derivations.
3) Set `st.DHr := msg.DH_pub`, set `st.Nr := 0`.
4) Compute `dh_out = X25519(st.DHs_priv_current, st.DHr)`.
5) `(st.RK, st.CK_ec_recv) = KDF_RK_DH(st.RK, dh_out)`.
6) Reinitialize PQ recv chain from the updated root:
   - If `role = A`, `CK_pq_recv := KMAC32(RK, "QSP5.0/PQ0/B->A", [0x01])`
   - If `role = B`, `CK_pq_recv := KMAC32(RK, "QSP5.0/PQ0/A->B", [0x01])`
7) Recompute `HK/NHK` from the updated `RK` (§8.1).
8) Commit state only after body decrypt success.

#### 8.5.3 Applying SCKA reseed (boundary with PQ ciphertext)

If a boundary message carries `FLAG_PQ_CTXT`, it additionally carries:
- `pq_target_id`
- `pq_ct`

Receiver MUST:
1) Require `hdr_source == CURRENT_NHK` (see §8.5.1).
2) Process SCKA ciphertext per DOC-CAN-004 and obtain `pq_epoch_ss`. If decapsulation fails or violates monotonicity/one-time rules, reject and do not commit state.
3) Let `RK_old = RK`.
4) Compute `(CK_pq_seed_A2B, CK_pq_seed_B2A) = KDF_PQ_RESEED(RK_old, pq_target_id, pq_ct, pq_epoch_ss)` (§3.3.6).
5) Update `RK := KDF_RK_PQ(RK_old, pq_epoch_ss)` (§3.3.3).
6) Apply directional PQ chain keys:
   - If `role = A`:
     - `CK_pq_send := CK_pq_seed_A2B`
     - `CK_pq_recv := CK_pq_seed_B2A`
   - If `role = B`:
     - `CK_pq_send := CK_pq_seed_B2A`
     - `CK_pq_recv := CK_pq_seed_A2B`
7) Recompute `HK/NHK` from the updated `RK` (§8.1).
8) Commit state only after body decrypt success.

Sender-side (encapsulator) MUST apply the same reseed computation using its encapsulation shared secret for `pq_epoch_ss` and the on-wire `pq_ct`, so that both parties converge on the same directional seeds.

#### 8.5.4 PQ advertisement (boundary with PQ ADV)

If a boundary message carries `FLAG_PQ_ADV`, it updates the peer’s view of the sender’s available PQ receive keys (DOC-CAN-004). The receiver MUST validate the advertisement and MUST fail closed on malformed or policy-violating advertisements.

---

## 9. Bounds and out-of-order handling (normative)

### 9.1 Out-of-order messages

Suite-2 supports out-of-order delivery using a skipped-key map keyed by `(DH_pub, N)`.

If an inbound message’s `(DH_pub, N)` matches an entry in `MKSKIPPED`, the implementation MUST use that stored `mk` and MUST delete it upon successful use. If the stored `mk` fails to decrypt, the message MUST be rejected and the stored entry MUST be deleted (fail-closed, replay-resistant).

#### 9.1.1 MKSKIPPED eviction and delete-on-use (normative)

- The MKSKIPPED store is bounded by `MAX_MKSKIPPED`. Implementations MUST NOT allow the store to exceed this bound.
- When inserting skipped keys would exceed `MAX_MKSKIPPED`, implementations MUST evict entries deterministically before completing the insert.
- Eviction order MUST be deterministic and stable. The reference policy is: evict the lowest `(N, DH_pub)` entries first (ascending `N`, then byte-lexicographic `DH_pub`) until within bounds.
- Any message that relies on an evicted skipped key MUST be rejected deterministically (e.g., `REJECT_S2_REPLAY` or `REJECT_S2_HDR_AUTH_FAIL`, depending on the header-candidate search).
- Delete-on-use is mandatory: once a skipped key successfully decrypts a message, it MUST be deleted and any later reuse MUST be rejected.

### 9.2 Commit rule

No ratchet or SCKA state may be persisted unless:
- header decrypt succeeded and all header-source boundary rules were satisfied (where applicable),
- all bounds and policy checks succeeded,
- body decrypt succeeded, and
- SCKA logic (if present) succeeded.

### 9.3 Bounds (normative constants)

Suite-2 adopts the base bounds used by the reference lane:

- `MAX_SKIP = 1000` (max derived skipped message keys per epoch)
- `MAX_MKSKIPPED = 1000` entries, TTL 7 days
- `MAX_HEADER_ATTEMPTS = 100` per message
- `MAX_HKSKIPPED = 4` entries, TTL 7 days

Implementations MUST reject any message that would exceed these bounds or their associated policy rules. For `MAX_MKSKIPPED`, implementations MUST evict deterministically per §9.1.1 to remain within bounds.

---

## 10. Error handling and reason codes (normative)

Implementations MUST map failures to stable reason codes. At minimum, the following reason codes MUST exist and be used in conformance vectors:

- `REJECT_S2_LOCAL_UNSUPPORTED`
- `REJECT_S2_PEER_UNSUPPORTED`
- `REJECT_S2_SUITE_MISMATCH`
- `REJECT_S2_AD_MISMATCH`
- `REJECT_S2_ESTABLISH_PQ_BIND_MISSING`
- `REJECT_S2_ESTABLISH_PQ_BIND_MISMATCH`

Additional reason codes are permitted but MUST be documented and registered (see DOC-SCL-002).

---

## 11. Conformance requirements

Implementations MUST provide:
- **CAT-S2-KDF-001** vectors and execution (Suite-2 KDF conformance)
- **CAT-S2-DOWNGRADE-001** vectors and execution (fail-closed negotiation)
- SCKA logic and KEM vectors as required by DOC-TST-005 and DOC-CAN-004
- goal-lint compliance for documentation changes (Goals line + governance updates)

---
End of DOC-CAN-003
