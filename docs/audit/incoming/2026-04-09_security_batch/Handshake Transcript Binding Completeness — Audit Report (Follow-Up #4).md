## **Handshake Transcript Binding Completeness — Audit Report (Follow-Up \#4)**

### **1\. Executive Summary**

Both `pq_rcv_a_pub` (A's PQ receive key, in HS1) and `pq_rcv_b_pub` (B's PQ receive key, in HS2) are included in their respective transcript inputs and are covered by cryptographic signatures that a MITM cannot forge. A MITM who replaces either key on the wire would break the corresponding signature verification step before any key material derived from the substituted key is consumed or committed. However, four secondary concerns emerge: the hs2 transcript includes the full (non-zeroed) hs1 — correct but undocumented; there is no explicit length pre-check on `pq_rcv_a_pub` before `encap()` in `responder_process`; there is no targeted unit test for the MITM pq\_rcv substitution pattern; and the Suite-2 `pq_kem_pub_id` binding is a harness-level consistency check, not an independent cryptographic derivation from the raw transcript.

---

### **2\. Transcript Construction — Field-by-Field Verification**

#### **2.1 HS1 Transcript (`hs1_transcript`)**

**Defined at:** `tools/refimpl/quantumshield_refimpl/src/qsp/types.rs:229–236`

Rust  
pub fn hs1\_transcript(\&self, hash: \&dyn crate::crypto::traits::Hash) \-\> \[u8; 64\] {  
    let mut tmp \= self.clone();  
    tmp.sig\_ec\_a \= vec\!\[0u8; SZ\_ED25519\_SIG\];  
    tmp.sig\_pq\_a \= vec\!\[0u8; SZ\_MLDSA65\_SIG\];  
    let mut m \= b"QSP4.3/HS1".to\_vec();  
    m.extend\_from\_slice(\&tmp.encode());  
    hash.sha512(\&m)  
}

`HandshakeInit::encode()` writes the following fields in order (`types.rs:146–172`):

| Field | Included in hs1\_transcript | Notes |
| ----- | ----- | ----- |
| `protocol_version` | ✓ | 2 bytes |
| `suite_id` | ✓ | 2 bytes |
| `session_id` | ✓ | 16 bytes |
| `user_id_b` | ✓ | varbytes\_u16 |
| `device_id_b` | ✓ | u32 |
| `ek_dh_a_pub` | ✓ | 32 bytes |
| `ct1` | ✓ | 1088 bytes |
| `opk_used` | ✓ | u16 flag |
| `ct2` / `opk_dh_id` / `opk_pq_id` | ✓ | conditional on opk\_used |
| **`pq_rcv_a_id`** | **✓** | u32 |
| **`pq_rcv_a_pub`** | **✓** | 1184 bytes |
| `ik_sig_ec_a_pub` | ✓ | 32 bytes |
| `ik_sig_pq_a_pub` | ✓ | 1952 bytes |
| `sig_ec_a` | Zeroed | not signed (signature placeholder) |
| `sig_pq_a` | Zeroed | not signed (signature placeholder) |

**`pq_rcv_a_pub` is bound in hs1\_transcript.** A's signature `sig_ec_a` and `sig_pq_a` are computed over `hs1_hash = hs1.hs1_transcript(hash)` at `handshake.rs:188–192`. Any wire alteration of `pq_rcv_a_pub` changes `hs1_hash`.

#### **2.2 HS2 Transcript (`hs2_transcript`)**

**Defined at:** `tools/refimpl/quantumshield_refimpl/src/qsp/types.rs:313–326`

Rust  
pub fn hs2\_transcript(\&self, hs1: \&HandshakeInit, hash: \&dyn crate::crypto::traits::Hash) \-\> \[u8; 64\] {  
    let mut tmp \= self.clone();  
    tmp.conf\_b.fill(0);  
    tmp.sig\_ec\_b \= vec\!\[0u8; SZ\_ED25519\_SIG\];  
    tmp.sig\_pq\_b \= vec\!\[0u8; SZ\_MLDSA65\_SIG\];  
    let mut m \= b"QSP4.3/HS2".to\_vec();  
    m.extend\_from\_slice(\&hs1.encode());  // ← full hs1 including REAL sig bytes  
    m.extend\_from\_slice(\&tmp.encode());  
    hash.sha512(\&m)  
}

`HandshakeResp::encode()` writes the following fields (`types.rs:257–271`):

| Field | Included in hs2\_transcript | Notes |
| ----- | ----- | ----- |
| `protocol_version` | ✓ |  |
| `suite_id` | ✓ |  |
| `session_id` | ✓ |  |
| `dh0_b_pub` | ✓ |  |
| **`pq_rcv_b_id`** | **✓** | u32 |
| **`pq_rcv_b_pub`** | **✓** | 1184 bytes |
| `ct3` | ✓ | 1088 bytes — the KEM ciphertext encapsulated to pq\_rcv\_a\_pub |
| `conf_b` | Zeroed | MAC placeholder |
| `ik_sig_ec_b_pub` | ✓ |  |
| `ik_sig_pq_b_pub` | ✓ |  |
| `sig_ec_b` | Zeroed | signature placeholder |
| `sig_pq_b` | Zeroed | signature placeholder |

Additionally, `hs2_transcript` includes `hs1.encode()` **with A's actual signature bytes** (not zeroed). This means `hs2_hash` transitively binds the full content of HS1 including `pq_rcv_a_pub` AND includes A's signed attestation.

**`pq_rcv_b_pub` is bound in hs2\_transcript.** B's signatures are computed over `hs2_hash` at `handshake.rs:335–342`.

---

### **3\. MITM Resistance Analysis — Can pq\_rcv Keys be Replaced?**

#### **3.1 Scenario: MITM replaces `pq_rcv_a_pub` in HS1**

**Attack path:** MITM intercepts A's HS1, substitutes `pq_rcv_a_pub` with a MITM-controlled key `MITM_pub`, forwards to B. B then runs:

Rust  
// handshake.rs:262  
let hs1\_hash \= hs1.hs1\_transcript(deps.hash);  
// handshake.rs:263-267  
if \!deps.ed25519.verify(\&hs1.ik\_sig\_ec\_a\_pub, \&hs1\_hash, \&hs1.sig\_ec\_a) {  
    return Err(HandshakeError::BadSignature);  
}  
// handshake.rs:269-273  
if \!deps.pq\_sig.verify(\&hs1.ik\_sig\_pq\_a\_pub, \&hs1\_hash, \&hs1.sig\_pq\_a)? {  
    return Err(HandshakeError::BadSignature);  
}

Since `pq_rcv_a_pub` is part of `hs1.encode()`, changing it changes `hs1_hash`. A's actual signature is over the original `hs1_hash`. The modified `hs1_hash` will not verify under A's keys → `BadSignature` → B rejects before proceeding to `encap` at line 315\.

**Verdict:** ✓ **Detected.** A MITM cannot replace `pq_rcv_a_pub` without invalidating A's signature (both Ed25519 and ML-DSA-65).

#### **3.2 Scenario: MITM replaces `pq_rcv_b_pub` in HS2**

**Attack path:** MITM intercepts B's HS2, substitutes `pq_rcv_b_pub` with a MITM-controlled key, forwards to A. A then runs:

Rust  
// handshake.rs:389  
let hs2\_hash \= hs2.hs2\_transcript(\&init.hs1, deps.hash);  
// handshake.rs:391-395  
if \!deps.ed25519.verify(\&hs2.ik\_sig\_ec\_b\_pub, \&hs2\_hash, \&hs2.sig\_ec\_b) {  
    return Err(HandshakeError::BadSignature);  
}  
// handshake.rs:397-401  
if \!deps.pq\_sig.verify(\&hs2.ik\_sig\_pq\_b\_pub, \&hs2\_hash, \&hs2.sig\_pq\_b)? {  
    return Err(HandshakeError::BadSignature);  
}

Since `pq_rcv_b_pub` is part of `hs2.encode()`, changing it changes `hs2_hash`. B's signatures are over the original `hs2_hash`. The verification fails → `BadSignature` → A rejects.

**Verdict:** ✓ **Detected.** A MITM cannot replace `pq_rcv_b_pub` without invalidating B's signature.

#### **3.3 Scenario: MITM replaces `ct3` in HS2 (targeting ss3)**

`ct3` is included in `hs2.encode()` (line 265 in types.rs) and therefore in `hs2_hash`. Replacing `ct3` invalidates B's signature. Even if a MITM could substitute `ct3` and somehow keep B's sig valid (impossible without B's private key), A would decap the wrong `ct3` under `pq_rcv_a_priv`, obtain a wrong `ss3`, derive a wrong `rk0`, compute a wrong `conf`, and fail the confirmation check at `handshake.rs:409–411`:

Rust  
let conf \= kmac32(deps.kmac, \&rk0, "QSP4.3/CONF", \&hs2\_hash);  
if conf \!= hs2.conf\_b {  
    return Err(HandshakeError::BadConfirmation);  
}

**Verdict:** ✓ **Doubly protected** — first by B's signature over `hs2_hash` which includes `ct3`, second by the confirmation MAC which binds `ss3 = decap(ct3, pq_rcv_a_priv)`.

---

### **4\. Design Detail: hs2\_transcript Includes Full (Non-Zeroed) HS1**

**File:** `tools/refimpl/quantumshield_refimpl/src/qsp/types.rs:323`

Rust  
m.extend\_from\_slice(\&hs1.encode());  // NOT a zeroed copy of hs1

`hs2_transcript` calls `hs1.encode()` directly on the original `hs1`, including the actual `sig_ec_a` and `sig_pq_a` bytes (not the zeroed copies used for hs1\_transcript). This means B's signature over `hs2_hash` transitively includes A's actual sig bytes. This is the correct design — it means B's signature authenticity depends on having seen the real, signed HS1. However, this design choice is undocumented in the spec comment at `types.rs:311–312`:

"HS2 transcript per QSP §5.3.1: SHA-512("QSP4.3/HS2" || HandshakeInit || HS2\_input)"

The comment says "HandshakeInit" without specifying whether it is the zeroed or non-zeroed version. This ambiguity could cause interoperability failures with alternative implementations that interpret the spec differently.

---

### **5\. Identified Gaps**

#### **5.1 No explicit length pre-check on `pq_rcv_a_pub` before `encap`**

**File:** `tools/refimpl/quantumshield_refimpl/src/qsp/handshake.rs:315` **Compared with:** pre-flight checks at lines 249–257

Rust  
// Pre-flight checks:  
if hs1.sig\_ec\_a.len() \!= SZ\_ED25519\_SIG { return Err(Invalid("sig\_ec\_a len")); }  
if hs1.sig\_pq\_a.len() \!= SZ\_MLDSA65\_SIG { return Err(Invalid("sig\_pq\_a len")); }  
if hs1.ik\_sig\_pq\_a\_pub.len() \!= SZ\_MLDSA65\_PUB { return Err(Invalid("ik\_sig\_pq\_a\_pub len")); }

// BUT: no check:  
// if hs1.pq\_rcv\_a\_pub.len() \!= SZ\_MLKEM768\_PUB { return Err(Invalid("pq\_rcv\_a\_pub len")); }

// Direct use:  
let (ct3, ss3) \= deps.pq\_kem.encap(\&hs1.pq\_rcv\_a\_pub)?;  // line 315

A wrong-length `pq_rcv_a_pub` would only be caught by `encap()` returning `CryptoError::InvalidKey`, not by a proactive guard. In practice, the signature check at lines 262–274 would have already rejected a tampered `pq_rcv_a_pub` (changing any field changes `hs1_hash`). But the pre-flight validation pattern for field lengths is inconsistent — 3 fields get explicit length guards while `pq_rcv_a_pub` does not. There is no corresponding length check for `pq_rcv_b_pub` in `HandshakeResp` either.

#### **5.2 No targeted unit test for MITM pq\_rcv substitution**

**File:** `tools/refimpl/quantumshield_refimpl/src/qsp/handshake.rs` — test section lines 436–1166

The existing tests cover:

* Malformed HS1 (wrong `sig_ec_a` length) → `BadSignature` not called before length check (`issue27_malformed_hs1_rejects_before_verify_and_is_deterministic`)  
* Malformed HS2 (wrong `sig_ec_b` length) → similar  
* OPK partial bundle rejection  
* `ss3` mix determinism  
* `ss3` decap failure determinism

**Not covered:** a test that constructs a valid HS1 with `pq_rcv_a_pub = A_pub`, mutates it to `pq_rcv_a_pub = MITM_pub` after signing, and asserts that `responder_process` returns `BadSignature`. Similarly, no test constructs a valid HS2 with `pq_rcv_b_pub = B_pub`, mutates to `MITM_pub`, and asserts that `initiator_finalize` returns `BadSignature`. The structural protection exists in code, but the specific invariant is not tested.

The existing `handshake_rejects_tampered_transcript_no_mutation` test at `qsl/qsl-client/qsc/tests/handshake_security_closure.rs:131` is at the qsc layer (older wire format), tampers byte 8 of the B1 message (the `session_id` field), and tests the MAC path — not the pq\_rcv substitution path.

#### **5.3 Suite-2 `pq_kem_pub_id` binding is a harness consistency check, not a derivation from transcript**

**File:** `tools/actors/refimpl_actor_rs/src/main.rs:2815–2929`

The `s2.establish` endpoint checks that `pq_kem_pub_id` (top-level params) equals `bound.pq_kem_pub_id` (the `bound` object). Both values are supplied by the test harness orchestrator. The actor does NOT independently compute `SHA-256(pq_rcv_a_pub)` and verify it equals the supplied `pq_kem_pub_id`. The vector `S2-ESTABLISH-REJECT-PQ-BIND-MISMATCH-0001` (in `inputs/suite2/vectors/qshield_suite2_establish_vectors_v1.json`) tests that a mismatch between `pq_kem_pub_id=0xa0...bf` and `bound.pq_prekey_id=7` (vs `pq_prekey_id=42`) is rejected — but both values come from the caller.

The canonical spec at `DOC-CAN-003 §6.6` (lines 371, 386–387) states:

"The base handshake transcript MUST bind `pq_kem_pub_id` and `pq_prekey_id` to the authenticated transcript. ... `pq_kem_pub_id` MUST be 32 bytes, else reject."

This binding is verified to be consistent at the actor layer, but the chain from actual `pq_rcv_a_pub` bytes (in HS1) to the 32-byte `pq_kem_pub_id` passed to `s2.establish` is not enforced within the actor — it is the responsibility of the orchestrating harness to supply `SHA-256(pq_rcv_a_pub)` as `pq_kem_pub_id`. If the harness passes an incorrect hash, the actor's check would not detect it.

The NA-0025 test plan (`docs/archive/testplans/NA-0025_pq_binding_testplan.md`) documents CI-gated positive and negative cases for this, but the coverage relies on the harness passing consistent values, not on cryptographic enforcement within the actor.

#### **5.4 `rk0_pre` does not directly include `pq_rcv_a_pub` or `pq_rcv_b_pub`**

**File:** `tools/refimpl/quantumshield_refimpl/src/qsp/handshake.rs:37–61`

Rust  
fn derive\_rk0(..., ss1, ss2, dh1, dh2) \-\> \[u8; 32\] {  
    let mut m \= b"QSP4.3/MS".to\_vec();  
    m.extend\_from\_slice(ss1);   // from ct1  
    if let Some(s) \= ss2 { m.extend\_from\_slice(s); }  // from ct2  
    m.extend\_from\_slice(dh1);  
    if let Some(d) \= dh2 { m.extend\_from\_slice(d); }  
    let ms \= hash.sha512(\&m);  
    kmac.kmac256(\&ms, "QSP4.3/RK0", session\_id, 32\)  
}

`derive_rk0` mixes the secret outputs (`ss1`, `ss2`, `dh1`, `dh2`) but does not directly include `pq_rcv_a_pub` or `pq_rcv_b_pub`. These keys influence `rk0` only via `ss3 = KEM.encap(pq_rcv_a_pub)` → `mix_rk0_ss3(rk0_pre, ss3)`. This is correct: the KEM ciphertext `ct3` and the shared secret `ss3` are the relevant values. The keys are bound via the transcript signatures, not directly into the key schedule. This is the expected design but worth noting explicitly — the security argument is that the signature binds `pq_rcv_a_pub`, and `ct3` binds `ss3` to the real `pq_rcv_a_pub` holder.

---

### **6\. Summary Table**

| Property | Status | Evidence |
| ----- | ----- | ----- |
| `pq_rcv_a_pub` in HS1 transcript | ✓ Present | `types.rs:167` via `hs1.encode()` |
| `pq_rcv_b_pub` in HS2 transcript | ✓ Present | `types.rs:264` via `hs2.encode()` |
| HS1 signature covers `pq_rcv_a_pub` | ✓ Yes | `handshake.rs:188–192`: sign over `hs1_hash` |
| HS2 signature covers `pq_rcv_b_pub` | ✓ Yes | `handshake.rs:335–342`: sign over `hs2_hash` |
| MITM pq\_rcv\_a\_pub replacement → B rejects | ✓ Verified | Sig check at `handshake.rs:262–273` |
| MITM pq\_rcv\_b\_pub replacement → A rejects | ✓ Verified | Sig check at `handshake.rs:389–401` |
| MITM ct3 replacement → A rejects | ✓ Doubly protected | Sig (hs2\_hash includes ct3) \+ conf\_b check |
| hs2\_transcript includes full non-zeroed hs1 | ⚠️ Correct but undocumented | `types.rs:323` — spec comment ambiguous |
| `pq_rcv_a_pub` length check before `encap` | ⚠️ Missing | `handshake.rs:315` — no proactive guard |
| Unit test for MITM pq\_rcv substitution | ⚠️ Absent | No test in `handshake.rs` tests section |
| Suite-2 `pq_kem_pub_id` derived from raw pub key | ⚠️ Not enforced in actor | `actor/main.rs:2815–2929` — harness-supplied values |
| `pq_rcv_a/b_pub` directly in key schedule | — N/A | Keys influence only via ss3; correct by design |

No code changes were made during this task — the problem statement explicitly constrained this to a read-only audit ("Do not make code changes"). The deliverable was the security analysis report above, not a code patch.

There is no PR to describe. If you'd like, I can:

1. Write a PR description for a *future* remediation PR once the gaps identified in the audit are addressed (e.g., adding the missing length pre-check, the MITM pq\_rcv substitution unit test, the hs2\_transcript spec comment clarification, and the actor-level `pq_kem_pub_id` derivation enforcement).  
2. Draft the actual code changes to fix those gaps, then write the PR description.

Let me know which direction you'd like to proceed.

You asked

I'll let you decide, 1 or 2\.

![][image1]  
Implementing feature selection for options 1 or 2  
1m 42s

I'll go with **option 1** — the original problem statement still carries the "Do not make code changes" constraint, and a well-structured PR description is the appropriate deliverable for an audit finding.

---

## **Draft PR Description — Remediation for HS1/HS2 Transcript Binding Gaps (Follow-Up Audit \#4)**

**Title:** `fix: close HS1/HS2 transcript binding gaps identified in follow-up audit #4`

---

### **Summary**

Follow-up audit \#4 ("Handshake Transcript Binding Completeness") confirmed that both `pq_rcv_a_pub` and `pq_rcv_b_pub` are structurally bound into their respective transcripts and that a MITM substitution of either key is detected before any key material is committed. However, the audit identified four secondary gaps, each addressed here.

---

### **Changes**

#### **1\. Add explicit length pre-check for `pq_rcv_a_pub` in `responder_process`**

**File:** `tools/refimpl/quantumshield_refimpl/src/qsp/handshake.rs`

`responder_process` guards `sig_ec_a`, `sig_pq_a`, and `ik_sig_pq_a_pub` with explicit length checks before use (lines 249–257), but not `pq_rcv_a_pub`, which is passed directly to `deps.pq_kem.encap()` at line 315\. A wrong-length value falls through to `CryptoError::InvalidKey` rather than a descriptive `HandshakeError::Invalid("pq_rcv_a_pub len")`. This PR adds the guard to maintain consistent fail-closed pre-flight validation and make rejections auditable.

Similarly, `pq_rcv_b_pub` as supplied to `responder_process` and `HandshakeResp::pq_rcv_b_pub` get equivalent guards.

#### **2\. Add targeted unit tests for MITM pq\_rcv substitution**

**File:** `tools/refimpl/quantumshield_refimpl/src/qsp/handshake.rs` (test section)

Two new tests are added:

* **`pq_rcv_a_pub_replacement_in_hs1_rejected_by_responder`**: constructs a valid HS1 (with `pq_rcv_a_pub = A_pub`), mutates `hs1.pq_rcv_a_pub` to a MITM key after signing, calls `responder_process`, and asserts the return is `Err(HandshakeError::BadSignature)`. Demonstrates that the hs1\_transcript change from the substitution propagates into signature verification failure.

* **`pq_rcv_b_pub_replacement_in_hs2_rejected_by_initiator`**: constructs a valid HS2 (with `pq_rcv_b_pub = B_pub`), mutates `hs2.pq_rcv_b_pub` to a MITM key after signing, calls `initiator_finalize`, and asserts `Err(HandshakeError::BadSignature)`. Demonstrates that the hs2\_transcript change propagates into signature verification failure.

#### **3\. Clarify `hs2_transcript` spec comment re: non-zeroed HS1**

**File:** `tools/refimpl/quantumshield_refimpl/src/qsp/types.rs`

The comment at line 311–312 says `"SHA-512("QSP4.3/HS2" || HandshakeInit || HS2_input)"` without clarifying that `HandshakeInit` is the **full, non-zeroed encoding** (including A's actual `sig_ec_a` and `sig_pq_a` bytes). This is the correct design — but "non-zeroed" is the opposite of what implementors familiar with TLS 1.3-style transcript hashes (where you typically zero signing fields) might expect. The comment is updated to say: `"HandshakeInit is encoded as-is, with signature fields intact (not zeroed)"`.

#### **4\. Actor-level derivation of `pq_kem_pub_id` from raw `pq_rcv_a_pub`**

**File:** `tools/actors/refimpl_actor_rs/src/main.rs`

The `s2.establish` endpoint (lines 2815–2929) checks that caller-supplied `pq_kem_pub_id` matches `bound.pq_kem_pub_id`, but does not cryptographically verify that `pq_kem_pub_id == SHA-256(pq_rcv_a_pub)`. If the orchestrating harness passes an incorrect hash, the consistency check passes incorrectly.

This PR adds an optional `pq_rcv_raw_pub` input field to the `suite2.establish.run` operation. When supplied, the actor independently computes `SHA-256(pq_rcv_raw_pub)` and asserts it matches the incoming `pq_kem_pub_id`, emitting `REJECT_S2_ESTABLISH_PQ_BIND_MISMATCH` on failure. For backward compatibility with existing vectors that do not supply `pq_rcv_raw_pub`, the check is skipped when the field is absent. New positive and negative interop vectors are added to `inputs/suite2/vectors/qshield_suite2_establish_vectors_v1.json` under IDs `S2-ESTABLISH-REJECT-PQ-PUB-HASH-MISMATCH-0001` and `S2-ESTABLISH-ACCEPT-PQ-PUB-HASH-MATCH-0001`.

---

### **Test Coverage**

| New test / vector | Asserts |
| ----- | ----- |
| `pq_rcv_a_pub_replacement_in_hs1_rejected_by_responder` | MITM pq\_rcv\_a\_pub substitution → `BadSignature` at responder |
| `pq_rcv_b_pub_replacement_in_hs2_rejected_by_initiator` | MITM pq\_rcv\_b\_pub substitution → `BadSignature` at initiator |
| `S2-ESTABLISH-REJECT-PQ-PUB-HASH-MISMATCH-0001` | SHA-256(pq\_rcv\_raw\_pub) ≠ pq\_kem\_pub\_id → `REJECT_S2_ESTABLISH_PQ_BIND_MISMATCH` |
| `S2-ESTABLISH-ACCEPT-PQ-PUB-HASH-MATCH-0001` | SHA-256(pq\_rcv\_raw\_pub) \== pq\_kem\_pub\_id → accept |

---

### **References**

* `tools/refimpl/quantumshield_refimpl/src/qsp/types.rs` — `hs1_transcript` (line 229), `hs2_transcript` (line 313), `HandshakeInit::encode` (line 146), `HandshakeResp::encode` (line 257\)  
* `tools/refimpl/quantumshield_refimpl/src/qsp/handshake.rs` — `responder_process` pre-flight checks (lines 249–257), `encap` call (line 315), `initiator_finalize` sig verification (lines 389–401)  
* `tools/actors/refimpl_actor_rs/src/main.rs` — `s2.establish` binding check (lines 2815–2929)  
* `inputs/suite2/vectors/qshield_suite2_establish_vectors_v1.json` — existing PQ-BIND vectors  
* `docs/archive/testplans/NA-0025_pq_binding_testplan.md` — original NA-0025 binding requirement  
* `docs/canonical/DOC-CAN-003_QSP_Suite-2_True_Triple_Ratchet_v5.0.0_DRAFT.md` — §6.6 (`pq_kem_pub_id` binding requirement)

[image1]: <data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABgAAAAYCAIAAABvFaqvAAAAOElEQVR4XmP4QCXAgC5ALhg1iDAYNYgwwGLQnll3CSJ0PSPBIFyKiBcfWQZhKiJGzfA3iHJANYMA8tXyLB4w2TwAAAAASUVORK5CYII=>