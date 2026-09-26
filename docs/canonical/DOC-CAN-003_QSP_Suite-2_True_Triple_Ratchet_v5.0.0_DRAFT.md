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

### 4.1.1 Bucketed padding confidentiality (QSE)

When a bucketed envelope profile is selected, implementations MUST NOT expose exact payload length through cleartext envelope length fields. Only bucket/profile-level size information may be visible.

Receivers MUST parse the canonical inner protocol payload deterministically and treat trailing bytes as padding. Encodings that do not provide a unique deterministic payload/padding split MUST be rejected fail-closed.

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

- A boundary message header that applies an epoch transition (DH ratchet advancement and/or an SCKA reseed event) MUST be encrypted under the sender’s `NHK_s` derived from the **pre-boundary** `RK`; an advertisement-only boundary (`FLAG_PQ_ADV` without an epoch transition) advances no root and its header remains under the sender’s current `HK_s`.
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

After computing `dh_out = X25519(DHs_priv, msg.DH_pub)`, the implementation MUST reject the message with `REJECT_S2_DH_NONCONTRIBUTORY` and MUST NOT commit any state if `dh_out` is the all-zero value (RFC 7748 §6.1 contributory-behaviour check); the same check MUST be applied to the DH output of the combined DH+PQ boundary (§8.5.3) and to the sending side before the root advances.

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

If an inbound message’s `(DH_pub, N)` matches an entry in `MKSKIPPED`, the implementation MUST use that stored `mk` and MUST delete it upon successful use. If the stored `mk` fails to decrypt, the message MUST be rejected and the stored entry MUST be preserved (no mutation on reject).

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
- `REJECT_S2_COUNTER_OVERFLOW` — a symmetric message counter (`ns`/`nr`) would advance past
  `u32::MAX`. The send and receive paths MUST fail closed (terminate the session; no state
  mutation) rather than saturate the counter, since a frozen counter with static header keys
  would reuse a header nonce/ciphertext. A well-behaved sender never originates a message at
  the saturating counter.
- `REJECT_S2_DH_NONCONTRIBUTORY` — the X25519 DH output is the all-zero value, i.e. the peer's
  `DH_pub` lies in the small subgroup and contributes no entropy to the root (RFC 7748 §6.1). The
  receiver MUST reject and MUST NOT commit any state.

Additional reason codes are permitted but MUST be documented and registered (see DOC-SCL-002).

---

## 11. Conformance requirements

Implementations MUST provide:
- **CAT-S2-KDF-001** vectors and execution (Suite-2 KDF conformance)
- **CAT-S2-DOWNGRADE-001** vectors and execution (fail-closed negotiation)
- SCKA logic and KEM vectors as required by DOC-TST-005 and DOC-CAN-004
- goal-lint compliance for documentation changes (Goals line + governance updates)

---

## 12. Successor identifier allocation (normative table)

Goals: G4

This section is the canonical allocation table for the successor profile, as ruled for PLAN card F01 contract C01
(versions and boundaries): `docs/ops/contracts/C01_versions_and_boundaries.md`, ACCEPTED WITH NAMED FIXES and recorded
by D-1426. Allocation takes effect when this section merges; until then nothing here is allocated. The contract is the
rationale; this table is the allocation.

Status vocabulary: EXISTING (allocated elsewhere, cited); ALLOCATED (exact value fixed by this table); OPEN (the
dimension is allocated, the value is not; named by the contract's open cell); RESERVED (no bytes and no names
allocated; allocation belongs to the named contract); RETIRED (refused, never reused). O-numbers are the contract's
open cells (O2 profile string and TAG, O5 contact-id and namespace strings, O9 distinct error codes, O11 QHSM bump,
O12 candidate-record reconciliation, O13 C07 lineage fields); F-numbers are THE PLAN's cards
(`docs/ops/PLAN_QSL_successor_rev3.md`). "C" is the NA-0780 candidate (PR #1831 head ffc8fc52), cited as evidence only.
Refusal spellings not already registered in DOC-SCL-002 are registered there by the implementing PR (O9); this
section allocates identifiers, not error codes.

### 12.1 Allocation table

| Row | Namespace | Exact allocation | Refusal when absent / different | Status |
|---|---|---|---|---|
| A01 | Suite tuple | (protocol_version 0x0500, suite_id 0x0002), sec 1.1 | REJECT_QSC_HS_SUITE_UNSUPPORTED / _DOWNGRADE (sec 1.2) | EXISTING (sec 1.1), cited |
| A02 | Successor profile | ONE ASCII string, bytes 0x21-0x7E except 0x3D '=' (no whitespace: 0x20 and control bytes lie outside the range), length 1..50; not equal to NA0780-DIR-INTEGRATION-01, -02, -03 or NA0780-OWNER-FREE-01. VALUE: OPEN (O2). Consumers: A03 value; A13 `protocol`; delivery Transaction.version; QueuedIntent.profile; A17 packed marker; A18 marker; receipt-key KDF binding and receipt AD (CRYPTO INPUT) | per consumer: REJECT_QSC_HS_INTEGRATION_PROFILE; vault_version_unsupported; TRANSACTION_PROFILE; intent code (O9); successor_dir_foreign | ALLOCATED (rule, charset, length, consumers); VALUE OPEN (O2) |
| A03 | Handshake critical parameter | id 0x7f80, flag 0x01, value length = len(A02), value = A02; exactly once; in canonical order with 0x0001 | missing -> REJECT_QSC_HS_INTEGRATION_REQUIRED; value/flag -> REJECT_QSC_HS_INTEGRATION_PROFILE; repeated -> REJECT_QSC_HS_DUPLICATE_PARAMETER; order -> REJECT_QSC_HS_NONCANONICAL_ORDER; length -> REJECT_QSC_HS_MALFORMED_LENGTH | ALLOCATED |
| A04 | QHSM version | 2; version 1 refused. Bump question OPEN (O11, C02). C02 owns invite/handshake labels QSL.invite.payload.v1, QSL.invite.identity-commitment.v1, QSC.HS.SID, QSC.HS.ROOT.COMBINE.v1 | v1 -> REJECT_QSC_HS_INTEGRATION_REQUIRED | ALLOCATED; bump question OPEN (O11) |
| A05 | Core KDF profile | NA0780-DIR-EPOCH-CORE-01 (unchanged bytes) | n/a (label) | ALLOCATED (existing bytes kept) |
| A06 | KMAC customization prefix | NA0780.DE1/ (unchanged bytes) | n/a (label) | ALLOCATED (existing bytes kept) |
| A07 | Directional epoch frame | NDE1; kind 0 ordinary, 1 boundary | magic -> MAGIC; kind > 1 -> TYPE | ALLOCATED (existing bytes kept) |
| A08 | Core typed kind | 0 body, 1 reserved, 2 target advertisement | > 2 -> TYPED; 1 -> INTEGRATION_KIND | ALLOCATED (existing bytes kept) |
| A09 | Exact-wire receipt | NDR1, 113 bytes | per receipt parser | ALLOCATED (existing bytes kept) |
| A10 | Inner typed body | NDI2; kind 0 application, 1-4 reserved file kinds, 5 maintenance, 6-255 unassigned | 1-4 -> INTEGRATION_FILE_GATED (on the kind byte); 6-255 -> INTEGRATION_KIND; magic -> code OPEN (O9) | ALLOCATED; magic refusal code OPEN (O9) |
| A11 | Vault envelope magic | QSCV04; recognised-old QSCV01, QSCV02, QSCV03 | old -> vault_version_unsupported; other -> vault_parse_failed | ALLOCATED |
| A12 | Vault KDF | Argon2id, version 0x13, m = 262144 KiB, t = 3, p = 1, output 32 bytes, salt 16 bytes; exact match before derivation. Conditional on a held derived key (O4c, F05). Value per RULING round2 E5+E6; applies at all three Argon2 call sites (init, unlock, provider) | mismatch -> vault_parse_failed today; distinct code OPEN (O9) | ALLOCATED (measured; conditional on a held derived key, F05) |
| A13 | Vault payload | version 5; fields {version, protocol, mode, secrets} plus the RESERVED C07 lineage fields (O13); the field list is RESERVED, NOT FROZEN until F02 settles them, and no successor vault is written outside tests before then; protocol = A02; duplicate keys refused at every map level | vault_version_unsupported; parse -> vault_parse_failed | ALLOCATED (version 5); FIELD LIST RESERVED (O13) |
| A14 | Vault mode | field `mode`: "messaging" or "storage-only"; fixed at init; local only, never on the wire. RESERVED beside it: field name protection_mode, a separate discriminator for the C07 protection profile (O13) | vault_mode_unsupported; storage-only with owner/peer keys -> directional_owner_binding; storage-only establishing a session -> directional_mode_storage_only | ALLOCATED; protection_mode RESERVED (O13) |
| A15 | Vault owner/peer namespaces | owner key and peer prefix strings OPEN (O5); peer suffix = contact id | directional_schema_incompatible | ALLOCATED (rule); strings OPEN (O5) |
| A16 | Init selector | --mode messaging / --mode storage-only; the second axis (C07 protection profile at fresh-identity creation) RESERVED, no spelling (O13) | absent/unknown -> refuse before any write | ALLOCATED; second axis RESERVED (O13) |
| A17 | Queue family | directory msgqueue_v2; per-contact subdirectory = contact id; record version 2 (checked); record AAD label qsc.msgqueue.v2; store-key secret msgqueue_store_key_v2; packed marker protocol = A02 | record version -> refuse (code OPEN, O9); AEAD -> msgqueue_record_tampered | ALLOCATED; record-version refusal code OPEN (O9) |
| A18 | Store marker | store.meta lines store_version=2 and profile=<A02>; read at open | successor_dir_foreign | ALLOCATED; profile value rides A02 (O2) |
| A19 | Store directory leaf | "qsc-" + TAG; TAG = [a-z0-9]{1,16}, allocated with A02 (O2); never "qsc" | n/a (location) | ALLOCATED (rule); TAG OPEN (O2) |
| A20 | Store location override | environment variable "QSC_" + uppercase(TAG) + "_CONFIG_DIR"; QSC_CONFIG_DIR is not honoured by a successor build | legacy_config_override_ignored (marker, not a refusal) | ALLOCATED (rule); TAG OPEN (O2) |
| A21 | RETIRED (refused, never reused) | profiles NA0780-DIR-INTEGRATION-01, -02, -03, NA0780-OWNER-FREE-01; NDI1; QSCV01, QSCV02, QSCV03; payload versions 1-4; selector and marker spelling directional-v1, owner-free-v1; QSE envelope 0x0100 and the legacy Suite-2 message path; QSSV01 session store; legacy payload versions FILE_XFER_VERSION 1, ATTACHMENT_DESCRIPTOR_VERSION 1, CTRL_VERSION_MAX 2; the C appended reservation rows (C :663-671) | as A02-A13 | RETIRED |
| A22 | Reserved file formats | NDI2 kinds 1-4 (reserved, refused); binary file queue row tag (name reserved, no byte); NIF format (not allocated); allocation by C06 only | INTEGRATION_FILE_GATED | RESERVED |
| A23 | Protection-file format line | vault_security.txt and vault_unlock_failures.txt (CENSUS D7, D8): the FIRST line is exactly protection_version=2, split once at the first '='; the key=value lines after it keep today's exact key set (attempt_limit; failed_unlocks, last_failure_unix_s). An absent (today's unversioned) or any other format line -> refuse | refuse, fail closed, nothing counted, delayed or written (O7); today's unreadable shape is vault_attempt_limit_io (protection.rs:150); distinct code OPEN (O9) | ALLOCATED |
| A24 | C07 reserved dimensions | RESERVED, no bytes and no names allocated: the prepared-successor vault slot; the freshness checkpoint location (outside the store directory); the per-lineage lock; the C07 lineage fields of A13, protection_mode (A14) and the selector's second axis (A16). Allocation by C07 / F02 only (O13) | n/a | RESERVED |

### 12.2 The candidate's appended reservation, carried as RETIRED rows

The NA-0780 candidate branch (C) appended a section "NA-0780 first-release directional profile reservation" to this
document (C :653-685) that describes itself as "not an external registry allocation" and disagrees with the
candidate's own code in four places (:663, :665, :669, :670). That section was never merged to main. This table
SUPERSEDES it: each of its rows is carried here as RETIRED, and a value this table keeps is allocated only by its own
row above. Whether the candidate section's TEXT is carried anywhere stays OPEN (O12); this section does not carry it.

| Candidate row (C :line) | Candidate value | Status in this table | Successor row |
|---|---|---|---|
| :663 Handshake critical parameter | 0x7f80, flag 1, length 25, ASCII NA0780-DIR-INTEGRATION-01 | RETIRED value (A21); the parameter id 0x7f80 continues, with length = len(A02) | A02, A03 |
| :664 Directional frame | NDE1 | reservation row RETIRED; the value is allocated by A07 | A07 |
| :665 Inner delivery body | NDI1 | RETIRED value (A21); refused | A10 (NDI2) |
| :666 Exact-wire receipt | NDR1 | reservation row RETIRED; the value is allocated by A09 | A09 |
| :667 Core profile | NA0780-DIR-EPOCH-CORE-01 | reservation row RETIRED; the value is allocated by A05 | A05 |
| :668 KMAC customization prefix | NA0780.DE1/ | reservation row RETIRED; the value is allocated by A06 | A06 |
| :669 Encrypted transaction key | na0780_directional_transaction/{peer} (the candidate's code uses na0780_directional_transaction_v2/) | RETIRED (the record's spelling); the code's spelling is not the successor's either (A15: a fresh pair); the record-vs-code disagreement is O12 | A15 (strings OPEN, O5) |
| :670 Development vault envelope | QSCV03, schema 3 (the candidate's code: payload version 4) | RETIRED value (A21); recognised-old under A11, refused | A11, A13 |
| :671 Packed queue record | schema 1, protocol directional-v1 | RETIRED value (A21) | A17 |

### 12.3 C02 allocations (invitation authentication) and the A04 amendment

Added for PLAN card F01 contract C02 (invitation authentication): `docs/ops/contracts/C02_invitation_authentication.md`,
ACCEPTED WITH NAMED FIXES and recorded by D-1427. The contract is the rationale (T1a-T8 are its sections); this table is
the allocation, which takes effect when this subsection merges. Sections 12.1 and 12.2 are not edited: row A04-AM1
below supersedes row A04's value, as C01's appended amendment AM-2 records. Refusal spellings the contract marks NEW are
registered in DOC-SCL-002 by the implementing PR (the contract's OC12, as C01's O9); this subsection allocates
identifiers, not error codes. Values that depend on A02 (the profile string) stay OPEN with it (O2).

| Row | Namespace | Exact allocation | Refusal when absent / different | Status |
|---|---|---|---|---|
| A04-AM1 | QHSM version (AMENDS A04; C01 O11 RESOLVED) | 3; versions 1 and 2 refused on the successor path. Frame layouts (C02 T1c), each after the header "QHSM" ++ u16 3 ++ type ++ u16 block_len ++ block, block = A01 suite block ++ A03 parameter: A1 = sid 16, kem_pk 1184, sig_pk 1952, dh_pub 32, resp_kem_ct 1088, eph_kem_pk 1184 (a fresh ML-KEM-768 key per attempt); B1 = sid 16, kem_ct 1088, eph_kem_ct 1088, mac 32, sig_pk 1952, sig 3309, dh_pub 32; A2 = sid 16, mac 32, sig 3309. The PQ combiner input list is [ss_eph, ss_pq, resp_kem_ss]. In A04's label list, QSC.HS.SID is a test-seam label, not a domain label (C01 AMENDMENTS AM-3) | v1, v2 -> REJECT_QSC_HS_INTEGRATION_REQUIRED; other -> handshake_version | ALLOCATED (amendment; supersedes A04's value 2) |
| C02-01 | Invite code | prefix "QSLI-2-" ++ base64url without padding (canonical trailing bits) of the C02-02 payload; raw input at most 2048 bytes before trim or decode; trimmed code at most 859 characters | over 2048 -> invite_code_too_long; "QSLI-1-" -> invite_version_unsupported; other "QSLI-" -> invite_version_newer; else invite_malformed | ALLOCATED |
| C02-02 | Invite payload v2 | ver 0x02, type 0x01, profile (u8 length 1..50 ++ A02 bytes), invite_id 16, expiry u64, relay_ep (u16 length 1..512 ++ bytes in the contract's canonical relay_ep grammar, T1a), bearer_secret 16, commit 32 (SHA-256 of DS_COMMIT ++ the canonical bundle); no trailing bytes; at most 639 bytes. Signed ML-DSA-65 over "QSL.invite.payload.v1" ++ payload (label bytes kept) | ver 0x01 -> invite_version_unsupported; other ver -> invite_version_newer; type -> invite_type_unknown; profile -> invite_profile_unsupported; else invite_malformed | ALLOCATED; profile value OPEN (O2) |
| C02-03 | Handshake envelope | magic "QSLH" (51 53 4C 48), env_ver 0x02, env_type 0x01 (A1) / 0x02 (B1); the fields in the fixed order of C02 T1b; relay_ep equal to the invitation payload's; signature ML-DSA-65, 3309 bytes, over DS_ENV_T ++ envelope[0 .. offset(sig)); a received envelope over ENV_MAX = 12288 bytes is refused before any parse | QSLH-1 first bytes 01 01 / 01 02 -> handshake_envelope_version_retired; other env_ver -> handshake_envelope_version_unsupported; env_type -> handshake_envelope_type; size -> handshake_envelope_too_large; structure -> handshake_envelope_malformed; profile -> handshake_envelope_profile; form -> handshake_envelope_noncanonical; binding -> handshake_envelope_binding; A1 bearer tag -> invite_bearer_invalid; signature -> handshake_envelope_signature_invalid | ALLOCATED; profile value OPEN (O2) |
| C02-04 | Domain labels (NEW) | QSL.invite.redeem-cap.v1 (redeem_cap = KMAC<16>(bearer_secret, label, invite_id)); QSL.invite.bearer-key.v1 (K_bearer = KMAC<32>(bearer_secret, label, invite_id)); QSL.handshake.bearer.v1 (the A1 bearer tag, KMAC<32> keyed by K_bearer); QSL.handshake.envelope.A1.v1 and QSL.handshake.envelope.B1.v1 (DS_ENV_A1, DS_ENV_B1) | n/a (labels) | ALLOCATED |
| C02-05 | Domain labels (KEPT) | QSL.invite.identity-commitment.v1 (DS_COMMIT); QSL.invite.payload.v1 (DS_SIG); QSC.HS.ROOT.COMBINE.v1; QSC.HS.PQ; QSC.HS.DHINIT; QSC.HS.TRANSCRIPT; QSC.HS.TRANSCRIPT.H; QSC.HS.CONFIRM; QSC.HS.A2; QSC.HS.SIG.B1; QSC.HS.SIG.A2 | n/a (labels) | EXISTING (bytes kept) |
| C02-06 | RETIRED by C02 (refused, never reused) | "QSLI-1-" codes and invite payload v1; the QSLH-1 envelope (ENVELOPE_VER 0x01, TAG_BUNDLE 0x01, TAG_ROUTE_TOKEN 0x02, TAG_A1 0x03, TAG_B1 0x04, types 0x01/0x02 of that version); QHSM versions 1 and 2; the bare (unenveloped) A1/B1 on the successor mailbox | per rows C02-01, C02-03 and A04-AM1; a bare A1/B1 is not accepted on the successor mailbox (contract OC7) | RETIRED |

### 12.4 C03 allocations (relay authority and recovery)

Added for PLAN card F01 contract C03 (relay authority and recovery): `docs/ops/contracts/C03_relay_authority_and_recovery.md`,
ACCEPTED WITH NAMED FIXES and recorded by D-1428. The contract is the rationale (T1-T11 are its sections); this table is
the allocation, which takes effect when this subsection merges. Sections 12.1-12.3 are not edited. The relay derivation
LABELS are NOT allocated here: their spellings stay PROPOSED and the row is OPEN until the Director rules C3-O1. Values
the contract leaves to measurement or to the Director stay OPEN (C3-O2 H_REC; C3-O3 the bucket, ring, idle and ceiling
values). Refusal spellings the contract marks NEW (relay ERR_V2_* and the client relay_v2_* codes) are registered by the
implementing PRs (C3-O10: the qsl-server contract document, DRAFT DOC-SRV-008, and DOC-SCL-002); this subsection
allocates identifiers, not error codes. The relay's own contract for v2 is qsl-server `docs/server/DOC-SRV-008` (DRAFT);
DOC-SRV-007 stays the v1 contract.

| Row | Namespace | Exact allocation | Refusal when absent / different | Status |
|---|---|---|---|---|
| C03-01 | Relay derivation labels (CRYPTO-TOUCHING) | the set, each followed by fixed-length input and prefix-free: QSL.relay.deposit.v1 (D = H(label ++ R)), QSL.relay.key.v1 (K = H(label ++ D)), QSL.relay.log.v1, QSL.relay.slot-locator.v1 (L = H(label ++ invite_id)), QSL.relay.recovery.v1 (H_S), QSL.relay.ticket.v1 (H_T), QSL.relay.request.redeem.v1 (Q_R), QSL.relay.request.push.v1 (P), QSL.relay.request.create.v1 (Q_C); H = SHA-256 | n/a (labels) | OPEN (C3-O1): spellings PROPOSED, not allocated |
| C03-02 | Relay v2 request headers | X-QSL-Read-Cap = hex32(R); X-QSL-Deposit-Cap = hex32(D); both EXACTLY 64 lowercase hex, no trim, decoded before hashing. X-QSL-Operation-Id = hex16(op_id). X-QSL-Invite-Locator = hex16(invite_id). X-QSL-Invite-Ticket (EXISTING header name) carries hex32(T) on v2. On any v2 request X-QSL-Route-Token and x-msg-id are refused | malformed -> 400 (contract V06); v1 token present -> 400; capability missing -> 400 (contract T2 N2) | ALLOCATED |
| C03-03 | Relay v2 endpoints | POST /v2/mailbox/open; POST /v2/push; GET /v2/pull?max=N; POST /v2/pull/ack; POST /v2/invite/create; POST /v2/invite/redeem; POST /v2/invite/deliver; POST /v2/invite/settle; POST /v2/invite/revoke. Lease-only (no delete-on-pull arm); JSON bodies carry "v":2 and an exact field set | per the contract's T3 and T6 | ALLOCATED |
| C03-04 | Relay capability advertisement | GET /v1/server-info (existing, additive per DOC-SRV-006 rule 1): api gains "relay_v2"; new object "v2" with fields max_body_bytes, pull_max_items, lease_secs, recovery_secs, max_ack_ids, push_receipts_per_mailbox, max_mailboxes. The successor client requires all of them and v2.recovery_secs equal to its H_REC | absent or different -> relay_v2_unsupported (client), no request of any kind to /v1/ mailbox or invite paths | ALLOCATED |
| C03-05 | Relay store | mailbox kinds "mailbox" and "invite_slot", fixed at creation; qsl-server SCHEMA_VERSION 3 adds the v2 tables (mailboxes_v2, messages_v2, slots_v2, claims_v2, push_receipts_v2); no v1 row is migrated into v2 | a version-2 binary refuses a version-3 store (ERR_STORE_VERSION, existing); kind conflict -> 409 | ALLOCATED |
| C03-06 | v2 protocol constants | H_REC (recovery horizon, a protocol constant the client checks; PROPOSED 259200 s); V2_PULL_MAX_ITEMS (PROPOSED 32); PUSH_RECEIPTS_PER_MAILBOX (PROPOSED 1024); MAILBOX_IDLE_SECS (PROPOSED = retention TTL, a mailbox pulled at least once); the never-pulled horizon 3600 s (the pull lease ceiling); MAX_V2_MAILBOXES (PROPOSED 4096 default, 65536 ceiling); the global open bucket and the per-L/per-K lookup bucket (PROPOSED); CLOCK_SLACK (PROPOSED 3600 s). The A1 deliver body cap is C02 ENV_MAX 12288 (row C02-03); relay_v2 requires max_body_bytes >= 12288 | per the contract's T6-T8 | OPEN (C3-O2, C3-O3): dimensions allocated, values not |
| C03-07 | RETIRED on the successor path (never reused) | tui.relay.inbox_token as a mailbox secret (R is fresh CSPRNG output per mailbox; never a v1 token's bytes); the v1 ACK mapping of 404 to LegacyComplete on any /v2/ path; the relay-minted invite ticket and revoke_token on v2 (client-minted T; revoke by R_slot). The v1 relay endpoints themselves stay for old clients (retirement: C3-O9, operator) | a successor client never sends them | RETIRED (successor path) |

### 12.5 Amendment rows from the F01 combined audit (C01-C03 as a set)

Added for PLAN card F01 after the combined adversarial audit of C01, C02 and C03 (AUDIT_F01_C01_C03_FINDINGS, sha256
512d42751d7a7dc3b26fa68fb100a7bf5c812f646e44fad47adc80bb142001c2), as ruled (RULING_NA0783_F01_audit_2026-09-24) and
recorded by D-1429; the contract amendments are the AMENDMENTS sections of the three contracts. Sections 12.1-12.4 are
not edited (as 12.3 and 12.4 state for their predecessors); rows are appended here only. SUPERSEDED, used in this
subsection only, means: the named later row replaces the value, and the value is NOT allocated. This subsection
allocates identifiers, not error codes; refusal spellings the amendments mark NEW are registered by the implementing
PRs (C02 OC12, C03 C3-O10).

| Row | Namespace | Exact allocation | Refusal when absent / different | Status |
|---|---|---|---|---|
| C03-04-AM1 | Relay capability advertisement (AMENDS C03-04; C03 AMENDMENTS AM-6) | the successor client also requires v2.max_body_bytes >= 12288 (= C02 ENV_MAX, row C02-03), v2.pull_max_items >= 1 and v2.lease_secs >= 1 | a value below its floor -> relay_v2_unsupported (client), before any v2 call; a 401 on server-info -> relay_unauthorized (existing client code), not relay_v2_unsupported | ALLOCATED (amendment; adds to C03-04) |
| A04-ST1 | QHSM version (STATUS OF A04) | A04 value 2 SUPERSEDED by A04-AM1; not allocated. The successor QHSM version is 3 (row A04-AM1) | per A04-AM1 | SUPERSEDED |

### 12.6 C04 allocations (ownership and accounting)

Added for PLAN card F01 contract C04 (ownership and accounting): `docs/ops/contracts/C04_ownership_and_accounting.md`,
ACCEPTED WITH NAMED FIXES and recorded by D-1431. The contract is the rationale (T1-T9 are its sections). Sections
12.1-12.5 are not edited (as 12.3-12.5 state for their predecessors); rows are appended here only. C04 allocates NO wire
or local-schema identifier and no VaultPayload field (C01 O13): its rows fix reservation DIMENSIONS and their owners and
apply rows A13 and A17 without changing them. Values the contract leaves to measurement stay OPEN or HYPOTHESIS; its
word PROPOSED stays PROPOSED here. Refusal spellings the contract marks NEW (msgqueue_queue_full,
msgqueue_queue_bytes_full, history_full and the Q11 code) are PROPOSED and are registered in DOC-SCL-002 by the
implementing PR (C01 O9); this subsection allocates identifiers, not error codes. Row C03-04-AM2 amends row C03-04-AM1
as C03 AMENDMENTS AM-18 records.

| Row | Namespace | Exact allocation | Refusal when absent / different | Status |
|---|---|---|---|---|
| C04-01 | Reservation dimensions and owners | R01-R14 of the contract's T1, each with exactly ONE durable owner: the paired owner (CapacityOwner, PeerReserve, SessionControlReserve, OwnerEntry) inside the successor vault's owner secret, the Transaction for R04/R05, the msgqueue_v2 row for R08/R09 before PREPARED; no counter file and no second database; the owner key strings stay OPEN (C01 O5, row A15) | an admission that would exceed a reservation refuses before effects (contract T3 V-R2) | OPEN: dimensions fixed, values not (C4-O1..C4-O5, C4-O8, C4-O13); no identifier allocated |
| C04-02 | Aggregate and session-reserve limits | B_V 16777216 (L2), H_W 524288, CORE_B 288920, CTRL_B 36775 and the DERIVED per-session reserve 1699966; the ordinary-material L2 ceiling B_V - H_W - n x 1699966 (contract T6 L29); byte-field encoding of the vault and queue OPEN (C4-O7) | directional_aggregate_waiting (EXISTING at the candidate) | HYPOTHESIS (contract T6 L01-L08, L29; F06 proves or re-derives); not allocated |
| C04-03 | Queue family application (APPLIES row A17; A17 unchanged) | the counted pool = QUEUED plus PREPARED-not-yet-relay-accepted rows across all contacts; admission by slots Q_S, per-peer share Q_P and bytes Q_B with MAXPACK charged at enqueue, under one hold of the store lock, before any directory, file or temporary; record version 2 checked at reopen | msgqueue_queue_full, msgqueue_queue_bytes_full, history_full (PROPOSED spellings, NEW; DOC-SCL-002) | OPEN: values HYPOTHESIS (Q_S 64, Q_B 4194304, Q_P OPEN; C4-O1); spellings PROPOSED |
| C03-04-AM2 | Relay capability advertisement (AMENDS C03-04-AM1; C03 AMENDMENTS AM-18) | the successor client requires v2.max_body_bytes >= 65536 (= MAX_WIRE, the largest NDE1 wire), replacing the 12288 floor of C03-04-AM1; the pull_max_items and lease_secs floors of C03-04-AM1 are unchanged | a value below 65536 -> relay_v2_unsupported (client), before any v2 call | ALLOCATED (amendment; its max_body_bytes floor replaces C03-04-AM1's, which is not allocated) |

### 12.7 C05 allocations (dispatcher and GUI)

Added for PLAN card F01 contract C05 (dispatcher and GUI contract): `docs/ops/contracts/C05_dispatcher_and_gui.md`,
ACCEPTED WITH NAMED FIXES subject to the operator's final approval, and recorded by D-1433. The contract is the rationale
(T1-T9 and T1a are its sections). Sections 12.1-12.6 are not edited (as 12.3-12.6 state for their predecessors); rows
are appended here only. C05 allocates NO wire or local-schema identifier: its rows fix dispositions, budgets and
meanings. Every spelling it marks NEW (the disposition names; the MessageState, ConversationNotice and ContactTrust
spellings; text_empty, text_too_long, text_invalid_utf8, action_id_conflict, redeem_alias_conflict) is PROPOSED and is
registered in DOC-SCL-002 by the implementing PR (C01 O9; C05 C5-O16); this subsection allocates identifiers, not
error codes. Values the contract leaves to measurement stay HYPOTHESIS (C05 C5-O1, C5-O4, C5-O5, C5-O7). Row
C03-04-AM3 amends row C03-04-AM2 as C03 AMENDMENTS AM-25 and AM-26 record; row C04-01-AM1 amends row C04-01 as C04
AMENDMENTS AM-1 records.

| Row | Namespace | Exact allocation | Refusal when absent / different | Status |
|---|---|---|---|---|
| C05-01 | Dispatcher dispositions | ACK, ACK-DUP, DISPOSE, DEFER-P, DEFER-C, FAULT, HOLD, RESP-REFUSED (contract T1), with the code-to-disposition table T1a and the deferral reasons P1-P7 and C1-C5 (T2); an item is ACKed only after the durable commit that admits it or after full dispatch concludes DISPOSE (D-R1) | n/a (dispositions, not codes) | OPEN: meanings fixed, spellings PROPOSED; no identifier allocated |
| C05-02 | Facade DTO and status meanings | MessageState M1-M6, ConversationNotice C1-C8, ContactTrust, ChatRow, ConversationPage, Notify, Lock, HistoryUsage (contract T4), drawn per AMENDMENT A5 to THE PLAN (Queued and Prepared no tick; relay accepted an outline tick; delivered a solid tick; never a read mark) | n/a | OPEN: meanings fixed, spellings PROPOSED; no identifier allocated |
| C05-03 | IPC action id | action_id = 16 bytes CSPRNG per user gesture for submit_text and invite_create, stored with H(canonical body) in the commit that creates the record (contract T5) | action_id_conflict (PROPOSED spelling, NEW; DOC-SCL-002) | OPEN: field placement and spelling decided by the implementing PR; not allocated |
| C05-04 | Dispatcher budgets and the text limit | the per-pass budgets BU1-BU12 (N from B_PULL and v2.max_body_bytes; PUSH_MAX, PUSH_PEER, SLOT_PULLS, OPEN_MAX, SETTLE_MAX, T_PASS, LEASE_MARGIN, DEFER_MAX, BACKOFF_REPORT, REPUSH_AFTER) and TEXT_MAX 3913 bytes at the default padding ceiling | text_empty, text_too_long, text_invalid_utf8 (PROPOSED spellings, NEW) | HYPOTHESIS (C05 C5-O1, C5-O4, C5-O5); not allocated |
| C04-01-AM1 | Reservation dimensions and owners (AMENDS C04-01; C04 AMENDMENTS AM-1) | R07 gains the per-peer history share H_P (H_P < H_N), charged in the same paired commit as the R07 unit | history_full (PROPOSED spelling) on a submit; DEFER-C on an arrival | OPEN: the dimension fixed, its value HYPOTHESIS (C05 C5-O7); no identifier allocated |
| C03-04-AM3 | Relay capability advertisement (AMENDS C03-04-AM2; C03 AMENDMENTS AM-25, AM-26) | the successor client also requires v2.lease_secs >= T_PASS + the ack deadline (15 s) + LEASE_MARGIN, which supersedes C03-04-AM1's lease_secs >= 1 floor, and v2.max_body_bytes <= 1048576 beside C03-04-AM2's floor of 65536 | a value outside its bound -> relay_v2_unsupported (client), before any v2 call | ALLOCATED (amendment) for the max_body_bytes ceiling; the lease floor's form is allocated, its value HYPOTHESIS (C05 C5-O1) |

### 12.8 C07 allocations (rollback and durability)

Added for PLAN card F02 contract C07 (rollback and durability): `docs/ops/contracts/C07_rollback_and_durability.md`,
ACCEPTED WITH NAMED FIXES and recorded by D-1437. The contract is the rationale (T0-T11 are its sections; T6 the names,
types and bytes, T7 this route); this table is the allocation, which takes effect when this subsection merges. Sections
12.1-12.7 are not edited (as 12.3-12.7 state for their predecessors); rows are appended here only. By this merge C01 O13
(the C07 lineage fields) and C04 C4-O12 (their bytes inside R01: at most 287 bytes per vault for local-checkpoint, 474
for tpm, generation funded at 20 digits; contract T6.2) are CLOSED, and rows A13, A14, A16 and A24 are amended as the
rows below state. The vault envelope magic stays QSCV04 and the payload version stays 5: no new version is minted,
because no successor vault has been written outside tests (C01 O13). The marker spellings of the contract's T3.6
(freshness_*, anchor_*, committed_state_missing, commit_indeterminate, storage_durability_failed,
vault_protection_mode_unsupported, tpm_owner_auth_set) are registered in DOC-SCL-002 by the implementing PR (C01 O9);
this subsection allocates identifiers, not error codes.

| Row | Namespace | Exact allocation | Refusal when absent / different | Status |
|---|---|---|---|---|
| C07-01 | Freshness checkpoint (AMENDS A24: the checkpoint location) | QSLFRESH v1, 147 bytes, no trailing bytes: magic "QSLFRESH" @0+8, version u16 BE = 1 @8+2, profile u8 (1 local-checkpoint, 2 tpm) @10+1, vault_id @11+32, generation u64 BE @43+8, digest d @51+32, resulting anchor A @83+32, HMAC-SHA-256(checkpoint_mac_key, bytes 0..114) @115+32 (contract T6.5). Location (D31): `<STATE>/qsl/freshness/<64 lowercase hex of vault_id>/head`, STATE = $XDG_STATE_HOME if set and absolute, else `<HOME>/.local/state`; directory 0700, file 0600; temporary `head.tmp.<pid>`; test override variable `QSC_<TAG>_STATE_DIR` (rides O2, as A20) | missing -> freshness_checkpoint_missing; MAC failure, bad length or trailing bytes -> freshness_checkpoint_corrupt; unsupported version or profile, or a profile byte that disagrees with protection_mode -> freshness_checkpoint_unsupported | ALLOCATED |
| C07-02 | Vault payload C07 members (AMENDS A13: FIELD LIST RESERVED -> ALLOCATED; C01 O13 CLOSED) | payload version 5, field list FROZEN as {version, protocol, mode, protection_mode, vault_id, generation, predecessor_anchor, checkpoint_mac_key, tpm_enrollment, secrets}. vault_id: 32 bytes (CSPRNG at genesis, immutable, never reused); generation: u64 JSON number, 0 at genesis, +1 per commit, canonical; predecessor_anchor: 32 bytes (32 zero bytes at genesis); checkpoint_mac_key: 32 bytes (CSPRNG at genesis); tpm_enrollment: JSON null iff protection_mode = "local-checkpoint", else the object {nv_public: the 16-byte TPM2B_NV_PUBLIC as defined, nv_auth: 32 bytes, primary_template: string, primary_name: the 34-byte Name} with deny_unknown_fields. Every persisted byte field is canonical padded standard base64, strictly decoded; deny_unknown_fields kept; no serde(default) (contract T6.1) | a null/object combination other than the stated one -> vault_parse_failed (distinct code per O9); generation + 1 beyond u64 -> freshness_generation_exhausted; otherwise as A13 | ALLOCATED |
| C07-03 | Protection profile discriminator (AMENDS A14: protection_mode RESERVED -> ALLOCATED) | field protection_mode: exactly "local-checkpoint" or "tpm"; a separate discriminator from `mode`; immutable from genesis, no in-place change or downgrade | unknown value -> vault_protection_mode_unsupported | ALLOCATED |
| C07-04 | Init selector second axis (AMENDS A16) | `--protection local-checkpoint` / `--protection tpm`, REQUIRED at fresh-identity creation beside `--mode`; `--mode storage-only` is permitted with either; no in-place change later (contract T6.3) | absent or unknown -> refuse before any write; `--protection tpm` on a TPM whose (manufacturer, firmware version, spec revision) tuple is not on the qualified list -> anchor_unqualified; with ownerAuthSet = 1 -> tpm_owner_auth_set; each before any write | ALLOCATED |
| C07-05 | Reserved locations (AMENDS A24: the prepared slot and the lineage lock) | D30 the prepared-successor vault slot `vault.qsv.prepared` in the successor store directory (same filesystem as the vault), its temporary `vault.qsv.prepared.tmp.<pid>`, at most one, mode 0600; D32 the per-lineage lock `<STATE>/qsl/freshness/<hex vault_id>/lock` (exclusive flock, never unlinked, inode re-checked after flock), distinct from .qsc.lock (contract T6.4, T4.2 FN1) | n/a (locations) | ALLOCATED |
| C07-06 | Vault digest domain label (CRYPTO INPUT; new) | ASCII "QSL-C07-VAULT-v1" in d = SHA-256(label \|\| V \|\| B); A_next = SHA-256(A \|\| d), which equals the TPM extend rule (contract T4.1 C8) | n/a (label) | ALLOCATED |
| C07-07 | TPM NV usage profile (new) | the NV1 template: one index per vault lineage, TPM_NT_EXTEND, nameAlg SHA-256, dataSize 32, attributes exactly AUTHWRITE, NT_EXTEND, AUTHREAD, NO_DA (TPMA_NV 0x02040044 as defined, 0x22040044 once written; measured), authPolicy empty; production handles only in the owner range 0x01800000-0x01BFFFFF, drawn at random excluding present handles, at most 8 tries (H1, H2); the enrollment fields of contract T3.3 E2; the storage-primary template spelling "qsl-srk-ecc-p256-v1" (QQ6), whose template bytes the implementing PR records | template or owner-bypass mismatch at provisioning -> anchor_template_mismatch; no free handle or NV space -> anchor_capacity_exhausted; unlisted TPM -> anchor_unqualified; ownerAuthSet = 1 -> tpm_owner_auth_set | ALLOCATED (the primary template's bytes recorded by the implementing PR, QQ6) |

### 12.9 C01 O2 and O5 values (PLAN card F03 formalization)

Added for PLAN card F03 (reconcile integration and repair representative fixtures) from the operator-approved values of
C01 open cells O2 and O5 (RBANK_F03_C01_O2_O5_values_2026-09-25, banked; the Director proposed, the operator's word
"ACCEPT"), recorded by D-1439. Sections 12.1-12.8 are not edited (as 12.3-12.8 state for their predecessors); rows are
appended here only. By this merge C01 O2 and O5 are CLOSED and the rows below fix the values that rows A02, A03, A15,
A18, A19, A20, C02-02, C02-03 and C07-01 left OPEN. Row A02 is a CRYPTO INPUT (C01 T1 row 2: a direct input of the
session-root KDF through hs_root_combine, of the receipt-key KDF and of the receipt AD); its IMPLEMENTING PR carries an
SR-15 independent review. This subsection allocates identifiers, not error codes, and implements nothing.

| Row | Namespace | Exact allocation | Refusal when absent / different | Status |
|---|---|---|---|---|
| A02-V1 | Successor profile VALUE (AMENDS A02: VALUE OPEN -> ALLOCATED; C01 O2 CLOSED) | ASCII `QSL-SUCCESSOR-01`, 16 bytes, hex 51 53 4C 2D 53 55 43 43 45 53 53 4F 52 2D 30 31; every byte in 0x21-0x7E, none 0x3D; not equal to NA0780-DIR-INTEGRATION-01, -02, -03 or NA0780-OWNER-FREE-01. It is the one value of every A02 consumer (A03 value; A13 `protocol`; Transaction.version; QueuedIntent.profile; A17 packed marker; A18 marker; receipt-key KDF binding and receipt AD; C02-02 and C02-03 profile fields) | as A02, per consumer | ALLOCATED (CRYPTO INPUT; SR-15 at the implementing PR) |
| A03-V1 | Handshake critical parameter length (APPLIES A03) | parameter 0x7f80, flag 0x01, value length 16 (u16 big-endian 00 10), value = A02-V1; with the 9-byte suite block the parameter block is 30 bytes (<= HS_PARAM_BLOCK_MAX 64). The literal length 25 of the NA-0780 candidate is RETIRED with its value | as A03 (length -> REJECT_QSC_HS_MALFORMED_LENGTH) | ALLOCATED (derived from A02-V1) |
| A19-V1 | Store directory TAG (AMENDS A19: TAG OPEN -> ALLOCATED) | TAG = `succ01` (matches [a-z0-9]{1,16}); store directory leaf `qsc-succ01` | n/a (location) | ALLOCATED |
| A20-V1 | Store location override (APPLIES A20) | environment variable `QSC_SUCC01_CONFIG_DIR`; QSC_CONFIG_DIR stays unhonoured by a successor build | legacy_config_override_ignored (marker; spelling registered by the implementing PR, C01 O9) | ALLOCATED |
| A18-V1 | Store marker profile line (APPLIES A18) | store.meta lines `store_version=2` and `profile=QSL-SUCCESSOR-01` | successor_dir_foreign | ALLOCATED |
| C07-01-V1 | Freshness state test override (APPLIES C07-01's variable rule) | `QSC_SUCC01_STATE_DIR` (test override only, as C07-01 states) | as C07-01 | ALLOCATED |
| A15-V1 | Vault owner/peer namespaces (AMENDS A15: strings OPEN -> ALLOCATED; C01 O5 CLOSED for these strings) | owner key `qsl_successor_owner_v1`; peer key = `qsl_successor_transaction_v1/` ++ contact id (O5-01) | directional_schema_incompatible | ALLOCATED |
| O5-01 | Contact id (C01 O5; applies to A15, A17 and every per-contact artifact of C01 T5) | 16 bytes from the CSPRNG, written as 32 lowercase hex characters [0-9a-f]{32}, minted locally when a contact is created and never derived from the typed label. The typed label is a local note that lives ONLY inside the encrypted vault as a label -> contact-id mapping (the resolver); it never appears in a filename, directory name, vault key suffix or AAD. Renaming a contact changes only that mapping (no re-keying; the peer never sees the label) | a per-contact key or path whose suffix is not 32 lowercase hex -> refused by the consumer's own code (codes registered by the implementing PR, C01 O9) | ALLOCATED (format and placement); the resolver's storage row stays with CENSUS V7 (C01 O6) |
| A21-AM1 | RETIRED (ADDS to A21) | the candidate's parameter-length literal 25 (with its value); the candidate's owner/peer strings na0780_directional_owner_v1 and na0780_directional_transaction_v2/ and the record spelling na0780_directional_transaction/{peer} (12.2 :669); the "R02 approved identifier allocation" of NA-0780 TASK (2026-09-20), which was never a registry allocation (C01 O1); the identifier statements of the candidate qsc README sections "Directional first-release development draft", "NA-0780 successor development profile (Stage A)" and "R02 successor integration" (profile -01/-02/-03, schema 3/4, NDI1) | as A02-A13 | RETIRED |

---
End of DOC-CAN-003


## NA-0780 first-release directional profile reservation

Goals: G4. Status: draft integration; no release acceptance.
The project reserves the following exact identifiers for its single intended
first-release directional protocol. The project-local allocation check found no
conflicting use or separate allocator in the project namespaces at the reviewed
base. This is not an external registry allocation.

| Namespace | Exact allocation | Meaning |
| --- | --- | --- |
| Handshake critical parameter | `0x7f80`, flag `1`, length `25` | Exact ASCII `NA0780-DIR-INTEGRATION-01`; mandatory with the existing Suite-2 tuple |
| Directional frame | `NDE1` | Existing directional epoch frame encoding |
| Inner delivery body | `NDI1` | Existing application/control and closure encoding |
| Exact-wire receipt | `NDR1` | Existing authenticated receipt encoding |
| Core profile | `NA0780-DIR-EPOCH-CORE-01` | Existing core KDF context |
| KMAC customization prefix | `NA0780.DE1/` | Existing label-specific customization, unchanged |
| Encrypted transaction key | `na0780_directional_transaction/{peer}` | One transaction per single-channel peer alias |
| Development vault envelope | `QSCV03` | First-release-only vault, authenticated schema 3 and exact integration profile |
| Packed queue record | schema `1`, protocol `directional-v1` | Exact ciphertext committed in the authoritative vault before queue transport |

These reservations do not change candidate KDF bytes, activation, capacity or
receipt rules. Unknown, missing, duplicate or changed handshake selection and
unsupported stored schema/profile MUST refuse without reset or fallback. New
vault creation requires explicit `directional-v1` selection and an empty development
configuration. Existing development vaults and fixtures MUST remain untouched.
Alias/channel divergence MUST refuse before preparation or discard. Fault hooks
are permitted only in an explicitly enabled non-shipping test configuration.

Actual relay and macOS acceptance remain release gates. This draft establishes
neither backup-rollback detection nor power-loss resistance. PQ recovery requires
an unexposed honest target and a delivered honest transition; indefinite denial
can prevent progress. Canonical security and formal-verification requirements
remain in force, and no release, default activation or migration is authorized.
