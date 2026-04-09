## **PQ KEM Decapsulation Failure Handling — Audit Report**

### **1\. Executive Summary**

The repository uses two distinct `PqKem768` implementations that wrap two different underlying ML-KEM-768 libraries, each with different decapsulation error semantics. The `pqcrypto-mlkem 0.1.1` path is provably implicit-rejection-safe (decapsulate is infallible). The `ml-kem 0.2.1` path wraps a fallible `decapsulate()` and maps all errors to a single `CryptoError::AuthFail`, which is good, but the path that should never return an error for valid-length ciphertext — a tampered but correctly-sized CT — is untested for the `ml-kem` implementation. The SCKA canonical spec explicitly requires constant-time decapsulation (`DOC-CAN-004_QSP_SCKA_Sparse_Continuous_Key_Agreement_v1.0.0_DRAFT.md:162`) but neither library carries `subtle` as a dependency. Several secondary oracle concerns exist: distinguishable error codes leaked to caller-observable output, `CryptoError` variant strings included verbatim in wire-visible JSON error messages, and one existing timing oracle in header key selection (previously noted in the 2026-01-04 code analysis) that interacts with the ratchet's PQ path.

---

### **2\. Inventory of All `decap()` Call Sites**

There are exactly five production `pq_kem.decap()` call sites and one internal test-harness call site:

| \# | Location | KEM Library Used | Error Disposition |
| ----- | ----- | ----- | ----- |
| 1 | `tools/refimpl/quantumshield_refimpl/src/qsp/handshake.rs:277` | Caller-injected (`PqKem768` trait) | `?` → `HandshakeError::Crypto(CryptoError::...)` |
| 2 | `tools/refimpl/quantumshield_refimpl/src/qsp/handshake.rs:279` | Caller-injected | `?` → same |
| 3 | `tools/refimpl/quantumshield_refimpl/src/qsp/handshake.rs:405` | Caller-injected | `?` → `HandshakeError::Crypto(CryptoError::...)` distinct from `HandshakeError::BadConfirmation` |
| 4 | `tools/refimpl/quantumshield_refimpl/src/qsp/ratchet.rs:569` | Caller-injected | `?` → `RatchetError::Crypto(CryptoError::...)` |
| 5 | `qsl/qsl-client/qsc/src/handshake/mod.rs:845` | `StdCrypto` / `pqcrypto-mlkem 0.1.1` | `match Err(_) → emit_marker("pq_decap_failed")` then `return Ok(())` |
| 6 | `tools/actors/refimpl_actor_rs/src/main.rs:3435` | `ml-kem 0.2.1` (direct call, `scka.kem.check` harness) | `map_err(REJECT_SCKA_KEM_DECAP_FAIL)` |

The actor's main ratchet path (calls 1–4) uses the `MlKemDet` struct (`tools/actors/refimpl_actor_rs/src/main.rs:1056–1062` and `main.rs:253–258`), which wraps `ml-kem 0.2.1`.

---

### **3\. Implicit Rejection Safety — Two Libraries, Two Behaviors**

#### **3.1 `pqcrypto-mlkem 0.1.1` (used by `StdCrypto`, call site \#5)**

**File**: `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs:163–169`

Rust  
fn decap(\&self, privk: &\[u8\], ct: &\[u8\]) \-\> Result\<Vec\<u8\>, CryptoError\> {  
    let sk \= mlkem768::SecretKey::from\_bytes(privk).map\_err(|\_| CryptoError::InvalidKey)?;  
    let ct \= mlkem768::Ciphertext::from\_bytes(ct).map\_err(|\_| CryptoError::InvalidKey)?;  
    let ss \= mlkem768::decapsulate(\&ct, \&sk);  // ← infallible, no ? or map\_err  
    Ok(ss.as\_bytes().to\_vec())  
}

`mlkem768::decapsulate()` has return type `SharedSecret` — not `Result<SharedSecret, _>`. It is **infallible by design**. This is ML-KEM's built-in implicit rejection: for a tampered-but-correctly-sized ciphertext, it returns a pseudorandom shared secret derived from a re-randomized path, not an error. The `pqkem768_tamper_changes_secret` test at `stdcrypto.rs:325–338` and `tests/pqkem768.rs:16–25` confirms this: a bit-flipped ciphertext produces `Ok(different_ss)`, not `Err`.

**Possible failure from `pqcrypto-mlkem`:**

* Wrong-length `privk` bytes → `CryptoError::InvalidKey` (from `from_bytes`)  
* Wrong-length `ct` bytes → `CryptoError::InvalidKey` (from `from_bytes`)  
* Valid-length tampered `ct` → `Ok(wrong_ss)` (implicit rejection)

This is the correct behavior. The library (`pqcrypto-mlkem 0.1.1`) wraps the NIST reference C implementation via `pqcrypto-internals` (the Cargo.lock shows a `cc` dependency), which the NIST specification defines to be constant-time.

#### **3.2 `ml-kem 0.2.1` (used by `MlKemDet` in actor and top-level harness, call sites \#1–\#4, \#6)**

**Actor file**: `tools/actors/refimpl_actor_rs/src/main.rs:1056–1062` **Top-level file**: `main.rs:253–258`

Rust  
fn decap(\&self, privk: &\[u8\], ct: &\[u8\]) \-\> Result\<Vec\<u8\>, CryptoError\> {  
    let dk \= Self::dk\_from\_bytes(privk)?;                          // CryptoError::InvalidKey if wrong length  
    let ct\_enc \=  
        ml\_kem::Ciphertext::\<MlKem768\>::try\_from(ct)  
            .map\_err(|\_| CryptoError::AuthFail)?;                  // CryptoError::AuthFail if wrong length  
    let ss \= dk.decapsulate(\&ct\_enc)  
        .map\_err(|\_| CryptoError::AuthFail)?;                      // CryptoError::AuthFail if decap fails  
    Ok(ss.as\_slice().to\_vec())  
}

Here `decapsulate()` returns a `Result`, so it can return an error. FIPS 203 mandates implicit rejection for ML-KEM, and the `ml-kem 0.2.1` crate is a pure-Rust implementation. For a correctly-sized but tampered ciphertext, the expected behavior is `Ok(wrong_ss)`. However, **there is no unit test in this codebase that exercises the `ml-kem 0.2.1` path with a tampered valid-length ciphertext and asserts `Ok(...)` rather than `Err(...)`**. The only tamper test (`pqkem768_tamper_changes_secret`) uses `pqcrypto-mlkem`. The `scka.kem.check` harness (call site \#6) accepts `tamper_ct=true` and returns `ss_match: false` rather than an error — this confirms the implicit rejection works in that path. But there is no equivalent test for the ratchet decap path (call site \#4 at `ratchet.rs:569`).

**Key risk**: If `ml-kem 0.2.1`'s `decapsulate()` returns `Err` for a valid-length tampered ciphertext (which would be a non-conformant implementation), the resulting `CryptoError::AuthFail` would short-circuit the ratchet, producing a distinguishable failure from the normal AEAD authentication failure. This would be an oracle.

#### **3.3 Error code asymmetry between the two libraries**

For the `pqcrypto` path (call site \#5 in `qsc/handshake/mod.rs`), the two failure modes map to **different CryptoError variants**:

| Failure mode | `pqcrypto-mlkem` error | `ml-kem 0.2.1` error |
| ----- | ----- | ----- |
| Wrong-length private key | `CryptoError::InvalidKey` | `CryptoError::InvalidKey` |
| Wrong-length ciphertext | `CryptoError::InvalidKey` | `CryptoError::AuthFail` |
| Valid-length tampered CT | `Ok(wrong_ss)` \[implicit\] | `Ok(wrong_ss)` \[expected, untested\] |

The `qsc/handshake/mod.rs:847` match arm uses `Err(_)` — all errors produce the same `"pq_decap_failed"` marker, collapsing the distinction. However, the refimpl/actor paths use `?` propagation, which preserves the variant. The `ActorError::Crypto` format string at `tools/actors/refimpl_actor_rs/src/main.rs:687` serializes `CryptoError` via `{:?}`:

Rust  
impl From\<CryptoError\> for ActorError {  
    fn from(e: CryptoError) \-\> Self {  
        ActorError::Crypto(format\!("{:?}", e))   // "InvalidKey" vs "AuthFail" in the message  
    }  
}

The `#[error(...)]` strings on `CryptoError` (traits.rs:7–13): `"authentication failed"` for `AuthFail`, `"invalid key material"` for `InvalidKey`. These become `err.to_string()` in the actor's JSON response at `tools/actors/refimpl_actor_rs/src/main.rs:3679`. This means any caller of the actor API can distinguish `CryptoError::InvalidKey` (wrong key/private key length) from `CryptoError::AuthFail` (wrong CT length or decap failure) from the JSON error `message` field.

---

### **4\. Decapsulation Error — Timing Oracle Analysis**

#### **4.1 SCKA spec requirement vs. library dependency chain**

The SCKA canonical spec at `docs/canonical/DOC-CAN-004_QSP_SCKA_Sparse_Continuous_Key_Agreement_v1.0.0_DRAFT.md:162` explicitly states:

`pq_epoch_ss_in = MLKEM768.Decap(entry.adv_priv, pq_ct)` **(constant-time)**

However, the `ml-kem 0.2.1` dependency chain (from the Cargo.lock) is:

Code  
ml-kem → hybrid-array, kem, rand\_core, sha3

**`subtle` is absent**. The `subtle` crate is the standard Rust mechanism for constant-time comparisons. Of the packages that do depend on `subtle` in this workspace, none are `ml-kem` itself:

Code  
subtle dependents: aes-gcm, curve25519-dalek, digest, ed25519-dalek, password-hash, rustls, universal-hash

The `pqcrypto-mlkem 0.1.1` path uses the reference C implementation (`pqcrypto-internals` via `cc`), which the NIST reference implementation specifies as constant-time and which has been independently analyzed. That library does not use `subtle` either, but the C implementation's constant-time properties come from compiler flags and NIST's implementation choices.

There is **no documented verification** in the repository that `ml-kem 0.2.1`'s decapsulation is constant-time at the Rust level. The spec's `(constant-time)` annotation is a normative requirement with no corresponding test or evidence artifact in the codebase.

#### **4.2 Pre-existing timing oracle in header key selection (adjacent risk)**

The 2026-01-04 Code Analysis Report (Issue \#11) at `docs/audit/CODE_ANALYSIS_REPORT_20260104.md:281` identifies an existing timing oracle in header key decryption:

"Early-exit optimization leaks timing information about which key succeeded. Attacker observing response times can determine header key state."

**File**: `tools/refimpl/quantumshield_refimpl/src/qsp/ratchet.rs` (header key decrypt path)

The report recommends constant-time selection across all candidate header keys. This is directly adjacent to the PQ decap path: in `ratchet_decrypt` (ratchet.rs), header key decryption (`header_decrypt`) at line 426 runs before the PQ decap at line 569\. If an attacker can exploit the header-key timing oracle, they can selectively trigger or avoid the PQ decap code path, potentially using timing differences to extract oracle information from the PQ decap itself.

---

### **5\. Error Type Observability Analysis per Call Site**

#### **5.1 Call sites \#1 and \#2: `responder_process` (handshake.rs:277, 279\)**

Both ss1 and ss2 decap errors propagate via `?` as `HandshakeError::Crypto(CryptoError::...)`. In the actor harness (`tools/actors/refimpl_actor_rs/src/main.rs:763`):

Rust  
.map\_err(|e| ActorError::Crypto(format\!("handshake\_respond failed: {e}")))

The `{e}` format uses the `Display` impl of `HandshakeError`, which delegates to `CryptoError`'s `Display` (which is the `#[error]` string: `"authentication failed"` vs `"invalid key material"`). This string appears in the JSON response's `message` field. **A caller can distinguish wrong-length CT (`InvalidKey`) from a decap algorithm failure (`AuthFail`).**

For `pqcrypto-mlkem`: wrong-length sk → `InvalidKey`; wrong-length ct → `InvalidKey`; tampered-valid-ct → `Ok(wrong_ss)` → auth failure downstream. For `ml-kem 0.2.1` (used in actor): wrong-length sk → `InvalidKey`; wrong-length ct → `AuthFail`; tampered-valid-ct → (expected) `Ok(wrong_ss)`.

#### **5.2 Call site \#3: `initiator_finalize` (handshake.rs:405) — critical ordering**

The ss3 decap at line 405 occurs **after** HS2 signature verification (lines 391–401) but **before** the confirmation check (lines 409–411):

Rust  
// Line 405  
let ss3 \= deps.pq\_kem.decap(\&pq\_rcv\_a\_priv, \&hs2.ct3)?;  
let rk0 \= mix\_rk0\_ss3(deps.kmac, \&init.rk0\_pre, \&init.session\_id, \&ss3);

// Line 409  
let conf \= kmac32(deps.kmac, \&rk0, "QSP4.3/CONF", \&hs2\_hash);  
if conf \!= hs2.conf\_b {  
    return Err(HandshakeError::BadConfirmation);  
}

For `pqcrypto-mlkem` with a valid-length tampered `ct3`: decap returns `Ok(wrong_ss)`, rk0 derivation succeeds with the wrong key, and the confirmation check fails → `HandshakeError::BadConfirmation`. For a wrong-length `ct3`: decap returns `CryptoError::InvalidKey` → `HandshakeError::Crypto(InvalidKey)`.

**These produce distinguishable errors.** The test `ss3_decap_failure_rejects_deterministically_and_no_mutation` at `handshake.rs:1077` only exercises the mock `PqKemFixed` with `fail_decap: true` → `CryptoError::InvalidKey`. It tests determinism and no state mutation, but does not test that the caller cannot distinguish this error from `BadConfirmation`.

#### **5.3 Call site \#4: ratchet PQ\_CTXT decap (ratchet.rs:569)**

The decap at `ratchet.rs:569` is inside the `if (msg.flags & FLAG_PQ_CTXT) != 0` block and uses `?` directly:

Rust  
let pq\_ss \= pq\_kem.decap(privk, ct)?;

This propagates as `RatchetError::Crypto(CryptoError::...)`. In the actor at `tools/actors/refimpl_actor_rs/src/main.rs:1781`:

Rust  
.map\_err(|e| ActorError::Crypto(format\!("decrypt failed: {e}")))

The error message includes the `CryptoError` string. The actor response at line 3679 sends this string to the caller. **If a network adversary can cause a wrong-length `pq_ct` to be processed, they receive a different error message from the actor than if the ciphertext is valid-length but tampered (which should trigger AEAD failure downstream).**

For the pqcrypto path: both wrong-length and wrong-key produce `CryptoError::InvalidKey` (from `from_bytes`) — they are **both** distinguishable from AEAD `AuthFail` but **not** distinguishable from each other. For the ml-kem path (in actor): wrong-length ct → `AuthFail`; wrong-length private key → `InvalidKey` — **distinguishable from each other**.

#### **5.4 Call site \#5: `qsc/handshake/mod.rs:845` — best current practice**

This is the most carefully written path. The `match c.decap(...) { Err(_) => ... }` discards the specific error variant, emitting only the opaque marker `"pq_decap_failed"` and returning `Ok(())`. This means:

* Wrong-length sk/ct → `"pq_decap_failed"` marker, no error distinction  
* No error code reaches the network peer

However, if a valid-length tampered ciphertext passes through, `pqcrypto-mlkem` decap succeeds with wrong\_ss, and the transcript MAC check at line 897 fails → `"bad_transcript"` marker. This is **observable at the process stdout level** as a different marker from `"pq_decap_failed"`. An attacker with process-level observability can distinguish wrong-length CT from valid-tampered CT.

#### **5.5 Call site \#6: `scka.kem.check` harness (actor main.rs:3435)**

Rust  
let ss\_in \= dk\_decap.decapsulate(\&ct\_enc).map\_err(|\_| {  
    ActorError::Invalid("reject: REJECT\_SCKA\_KEM\_DECAP\_FAIL".into())  
})?;  
let ss\_match \= ss\_in\_slice \== ss\_out\_slice;  
// Returns: { "ss\_match": { "bool": ss\_match } }

This is a **test vector harness**, not a production path. It explicitly returns `ss_match: false` when tampered. The `REJECT_SCKA_KEM_DECAP_FAIL` error would only fire if `ml-kem 0.2.1` fails to decapsulate a valid-length CT, which would indicate a non-conformant implementation.

---

### **6\. Error Type String Leakage in Wire-Visible Responses**

The `CryptoError` enum derives both `Debug` and `Display` (via `#[error(...)]`):

Rust  
// tools/refimpl/quantumshield\_refimpl/src/crypto/traits.rs:5–12  
\#\[derive(Debug, Error)\]  
pub enum CryptoError {  
    \#\[error("authentication failed")\]  AuthFail,  
    \#\[error("invalid key material")\]   InvalidKey,  
    \#\[error("not implemented")\]        NotImplemented,  
}

In `tools/actors/refimpl_actor_rs/src/main.rs:686–688`:

Rust  
impl From\<CryptoError\> for ActorError {  
    fn from(e: CryptoError) \-\> Self {  
        ActorError::Crypto(format\!("{:?}", e))   // "AuthFail", "InvalidKey", or "NotImplemented"  
    }  
}

This uses `{:?}` (Debug), which produces the variant name directly: `"AuthFail"`, `"InvalidKey"`, `"NotImplemented"`. The actor's JSON error response at line 3679 includes `err.to_string()` which uses the `Display` impl: `"crypto error: authentication failed"` or `"crypto error: invalid key material"`.

This means the actor's JSON response message distinguishes:

1. `"crypto error: invalid key material"` — private key or (for pqcrypto) ciphertext length wrong  
2. `"crypto error: authentication failed"` — (for ml-kem) ciphertext length wrong, or decap algorithm failure  
3. `"crypto error: not implemented"` — trait stub

A network caller can observe these distinctions. This is the actor protocol interface, not a direct network protocol, but any system that exposes actor responses over a network (or that logs them to a location readable by adversaries) creates a KEM decapsulation oracle differentiated by error type.

---

### **7\. Constant-Time Requirements vs. No `subtle` Dependency**

The SCKA spec (`DOC-CAN-004:162`) requires constant-time decapsulation. The `specs/00_security_objectives.md:112–113` also states:

"Avoid secret-dependent branching and memory access patterns in cryptographic primitives to the extent feasible (especially in signature/KEM operations and key schedule logic)."

**`ml-kem 0.2.1`** (Cargo.lock: `version = "0.2.1"`, deps: `hybrid-array`, `kem`, `rand_core`, `sha3`) carries **no `subtle` dependency**. The `sha3` implementation (`sha3 0.10.8`) does use `digest` which depends on `subtle`, but that is for digest finalization, not for the FO-transform comparison inside decapsulation. The critical constant-time operation in ML-KEM decapsulation is the re-encapsulation comparison (the FO-transform check). Without `subtle`, this comparison may be performed with native Rust `==` on byte slices, which compilers may or may not optimize to a branch.

**`pqcrypto-mlkem 0.1.1`** wraps the reference C implementation (`pqcrypto-internals` via `cc`). NIST's reference implementation of ML-KEM uses CMov-style constant-time operations in C. However, the Rust wrapper's `from_bytes` and `as_bytes` operations introduce data-dependent early exits on length mismatch, which are not in the decap hot path.

There is **no explicit test** in the repository that verifies constant-time behavior for either library (e.g., via timing analysis or a formal annotation).

---

### **8\. Summary Table**

| Risk | File(s) | Status |
| ----- | ----- | ----- |
| `pqcrypto-mlkem` decap infallible (correct) | `stdcrypto.rs:163–169` | ✓ Verified by tamper test |
| `ml-kem 0.2.1` decap implicit rejection (untested) | `actor/main.rs:1056–1062`, `main.rs:253–258` | ⚠️ No tamper test for ratchet path; `scka.kem.check` only |
| `CryptoError` variant string in JSON response | `actor/main.rs:686–688, 3679` | ⚠️ `"AuthFail"` vs `"InvalidKey"` observable in `message` field |
| `HandshakeError::Crypto` vs `BadConfirmation` distinguishable | `handshake.rs:405–411` | ⚠️ Different error types for wrong-length vs tampered-valid CT |
| `"pq_decap_failed"` vs `"bad_transcript"` markers | `qsc/handshake/mod.rs:845–899` | ⚠️ Process-level distinguishable; not wire-level |
| Constant-time requirement (spec) vs no `subtle` in `ml-kem` | `Cargo.lock`, `DOC-CAN-004:162` | ⚠️ Spec requires CT; no evidence of CT compliance in `ml-kem 0.2.1` |
| Pre-existing timing oracle in header key select | `ratchet.rs`, `CODE_ANALYSIS_REPORT_20260104.md:281` | ⚠️ Adjacent to PQ decap path; may compound oracle risk |
| Two libraries in same project with different error semantics | `stdcrypto.rs` vs `actor/main.rs` | ⚠️ Inconsistent: `from_bytes` ct failure → `InvalidKey` (pqcrypto) vs `AuthFail` (ml-kem) |
| SCKA pre-decap length check | `DOC-CAN-004:158`, `actor/main.rs:3428–3429` | ✓ Spec step 1 enforced in `scka.kem.check`; absent in ratchet path |

