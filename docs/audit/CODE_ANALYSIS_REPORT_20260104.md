# QSL Protocol Code Analysis Report

**Date:** 2026-01-04
**Scope:** Reference implementation security and correctness analysis
**Codebase:** QuantumShield (QSL) Protocol - Suite-2 / QSP v5.0

---

## Executive Summary

This report presents findings from a comprehensive code analysis of the QSL Protocol reference implementation. The analysis focused on:

1. **Cryptographic implementation security** - Key handling, randomness, side channels
2. **State machine correctness** - Ratchet logic, SCKA, error recovery
3. **Error handling and validation** - Panic vectors, input validation, fail-closed semantics

**Total Issues Identified: 28**

| Severity | Count | Risk Level |
|----------|-------|------------|
| CRITICAL | 3 | Immediate security/stability risk |
| HIGH | 6 | Significant security/correctness concern |
| MEDIUM | 14 | Should be addressed before production |
| LOW | 5 | Code quality improvements |

---

## Project Context

**QuantumShield (QSL)** implements a "True Triple Ratchet" messaging protocol:
- Classical Diffie-Hellman ratchet (X25519) for forward secrecy
- Classical symmetric ratchet per message
- Post-quantum symmetric ratchet via SCKA (Sparse Continuous Key Agreement)
- Hybrid combiner: `mk = KDF_HYBRID(ec_mk, pq_mk)` for per-message AEAD keys

The reference implementation is ~3,095 lines of Rust code with comprehensive test vectors and CI gating.

---

## CRITICAL Issues

### Issue #1: Signature Verification Default Fallback

**File:** `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs`
**Line:** 77

```rust
fn verify(&self, pubk: &[u8], msg: &[u8], sig: &[u8]) -> bool {
    use ed25519_dalek::{VerifyingKey, Signature, Verifier};
    let pk = match VerifyingKey::from_bytes(pubk.try_into().unwrap_or(&[0u8;32])) {
        Ok(v)=>v, Err(_)=>return false };
    let sig = match Signature::from_slice(sig) { Ok(s)=>s, Err(_)=>return false };
    pk.verify(msg, &sig).is_ok()
}
```

**Vulnerability:** If `pubk` is not exactly 32 bytes, `unwrap_or(&[0u8;32])` silently substitutes an all-zero public key. An attacker who can control the public key length could potentially forge signatures using the zero key.

**Impact:** SIGNATURE FORGERY - Violates fail-closed semantics; could accept invalid signatures.

**Recommendation:** Return `false` or propagate error when key length is invalid:
```rust
let pk_bytes: &[u8; 32] = pubk.try_into().map_err(|_| ())?;
// or simply: if pubk.len() != 32 { return false; }
```

---

### Issue #2: nr Overflow in Skip Key Derivation Loop

**File:** `tools/refimpl/quantumshield_refimpl/src/qsp/ratchet.rs`
**Lines:** 313-320

```rust
// Derive skipped keys up to n
while tmp.nr < n {
    let (mk, ck_next) = kdf_ck(hash, &tmp.ck_r);
    tmp.mk_skipped.insert((tmp.dh_peer.clone(), tmp.nr), mk);
    tmp.nr += 1;  // No overflow check
    tmp.ck_r = ck_next;
}
```

**Vulnerability:** If `tmp.nr` approaches `u32::MAX` and `n` is larger, the increment `tmp.nr += 1` will overflow. In debug builds this panics; in release builds it wraps to 0, causing:
- Infinite loop (if n > wrapped value)
- Incorrect skip key derivation
- Potential nonce reuse

**Impact:** DENIAL OF SERVICE / STATE CORRUPTION - Could cause infinite loop or message decryption failures.

**Recommendation:** Use checked arithmetic:
```rust
tmp.nr = tmp.nr.checked_add(1).ok_or("REJECT_NR_OVERFLOW")?;
```

---

### Issue #3: Panic on Invalid Ed25519 Key Length

**File:** `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs`
**Line:** 70

```rust
fn sign(&self, privk: &[u8], msg: &[u8]) -> Vec<u8> {
    let sk = SigningKey::from_bytes(privk.try_into().expect("ed25519 priv 32 bytes"));
    sk.sign(msg).to_bytes().to_vec()
}
```

**Vulnerability:** `expect()` will panic if `privk` is not exactly 32 bytes. Since this function takes a slice (not `[u8; 32]`), callers can pass invalid lengths causing a crash.

**Impact:** DENIAL OF SERVICE - Attacker-controlled input can crash the application.

**Recommendation:** Return Result instead of panicking:
```rust
fn sign(&self, privk: &[u8], msg: &[u8]) -> Result<Vec<u8>, CryptoError> {
    let sk_bytes: [u8; 32] = privk.try_into()
        .map_err(|_| CryptoError::InvalidKeyLength)?;
    let sk = SigningKey::from_bytes(&sk_bytes);
    Ok(sk.sign(msg).to_bytes().to_vec())
}
```

---

## HIGH Severity Issues

### Issue #4: Weak RNG Initialization

**File:** `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs`
**Lines:** 50-51, 86-88

```rust
// Keypair generation
let mut sk_bytes = [0u8; 32];
rand::thread_rng().fill_bytes(&mut sk_bytes);

// Nonce generation
fn random_nonce12(&mut self) -> [u8; 12] {
    let mut n = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut n);
    n
}
```

**Concern:** Uses `rand::thread_rng()` without explicit verification that it's properly seeded from a cryptographically secure source. While `rand` typically uses OS entropy, there's no explicit use of `getrandom` or verification of entropy quality.

**Impact:** WEAK CRYPTOGRAPHY - Poor randomness compromises key material and nonce uniqueness.

**Recommendation:**
- Use `getrandom` crate directly for critical operations
- Add entropy quality verification
- Consider using `rand::rngs::OsRng` explicitly

---

### Issue #5: Panic on AEAD Operations

**File:** `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs`
**Lines:** 36-39

```rust
fn seal(&self, key32: &[u8; 32], nonce12: &[u8; 12], ad: &[u8], pt: &[u8]) -> Vec<u8> {
    let cipher = Aes256Gcm::new_from_slice(key32).expect("key length");
    cipher.encrypt(nonce12.into(), Payload { msg: pt, aad: ad }).expect("encrypt")
}
```

**Concern:** Both `expect()` calls will panic on failure. While the first is safe (key32 is always 32 bytes), the second could theoretically fail.

**Impact:** DENIAL OF SERVICE - Panics can be exploited for service disruption.

**Recommendation:** Return `Result<Vec<u8>, AeadError>` instead of panicking.

---

### Issue #6: ck_pq_recv Not Updated on Boundary

**File:** `tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs`
**Line:** 365

```rust
let (ec_mk, _ck_ec_p) = kdf_ck(hash, &st.ck_ec_recv);
let (pq_mk, _ck_pq_p) = derive_mk_step(hash, &st.ck_pq_recv);  // Result discarded
let mk = kdf_hybrid(hash, &ec_mk, &pq_mk);
```

**Concern:** The `_ck_pq_p` result from `derive_mk_step` is discarded. The `ck_pq_recv` counter is never advanced per boundary message - only updated via `scka::apply_pq_reseed`.

**Impact:** STATE DIVERGENCE - If boundaries are received out-of-order or duplicated, `ck_pq_recv` could become out-of-sync with sender's expectations.

**Recommendation:** Verify this is intentional per protocol spec. If not, update `st.ck_pq_recv = ck_pq_p` after successful decryption.

---

### Issue #7: State Mutation Before Send Completion

**File:** `tools/refimpl/quantumshield_refimpl/src/qsp/ratchet.rs`
**Lines:** 211-223

```rust
if request_pq_mix && peer_id.is_some() && peer_pub.is_some() {
    let ss = pq_kem.encap(peer_pub.unwrap());
    // ... compute rk_pq ...
    st.rk = rk_pq;
    st.hk_s = nhk_s;
    st.hk_r = nhk_r;
    st.nhk_s = new_nhk_s;
    st.nhk_r = new_nhk_r;
}
// Then return the message
```

**Concern:** State (RK, header keys) is updated BEFORE the message is returned/sent. If message serialization fails or the caller doesn't send the message, state has already been updated.

**Impact:** STATE DIVERGENCE - Sender and receiver RK states diverge if send fails after state mutation.

**Recommendation:** Return new state values with the message; let caller commit after successful send.

---

### Issue #8: expect() Calls on Struct Invariants

**File:** `tools/refimpl/quantumshield_refimpl/src/qsp/types.rs`
**Lines:** 138-140, 293-298

```rust
if self.opk_used {
    w.write_bytes(self.ct2.as_ref().expect("ct2"));
    w.write_u32(self.opk_dh_id.expect("opk_dh_id"));
    w.write_u32(self.opk_pq_id.expect("opk_pq_id"));
}
```

**Concern:** These `expect()` calls panic if the struct invariant is violated (e.g., `opk_used=true` but `ct2=None`). No type-level guarantee prevents invalid states.

**Impact:** DENIAL OF SERVICE - Malformed internal state causes crashes.

**Recommendation:** Add validation method or use a type-state pattern to enforce invariants at compile time.

---

### Issue #9: Missing Key Zeroization

**Files:** All crypto modules
**Observation:** `Cargo.toml` includes `zeroize` dependency but it's never used.

```toml
zeroize = "1"
```

**Concern:** Sensitive data (private keys, shared secrets, derived keys) are not explicitly zeroized after use. Rust's drop semantics don't guarantee memory clearing.

**Impact:** INFORMATION DISCLOSURE - Key material may remain in memory, recoverable via memory dumps or side channels.

**Recommendation:**
```rust
use zeroize::Zeroize;

// After use:
secret_key.zeroize();
shared_secret.zeroize();
```

---

## MEDIUM Severity Issues

### Issue #10: Timing Side-Channel in Header Decryption

**Files:**
- `tools/refimpl/quantumshield_refimpl/src/qsp/ratchet.rs:147-166`
- `tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs:213-227`

```rust
if let Some(v) = try_key(&st.hk_r, HeaderSource::CurrentHk) { return Ok(v); }
if let Some(v) = try_key(&st.nhk_r, HeaderSource::CurrentNhk) { return Ok(v); }
// ... more attempts with early exit
```

**Concern:** Early-exit optimization leaks timing information about which key succeeded. Attacker observing response times can determine header key state.

**Impact:** INFORMATION DISCLOSURE - Timing oracle reveals internal key state.

**Recommendation:** Process all candidates and use constant-time selection.

---

### Issue #11: Nonce Reuse on Counter Overflow

**File:** `tools/refimpl/quantumshield_refimpl/src/qsp/ratchet.rs:67-76`

```rust
fn nonce_body(hash: &dyn Hash, session_id: &[u8;16], dh_pub: &[u8;32], n: u32) -> [u8;12] {
    // ... derives nonce from n ...
}
```

**Concern:** Message counter `n` uses `saturating_add()` which stops at `u32::MAX`. If counter saturates, subsequent messages reuse the same nonce.

**Impact:** CATASTROPHIC CRYPTO FAILURE - AES-GCM nonce reuse allows plaintext recovery.

**Recommendation:** Reject messages when counter reaches MAX or force ratchet advancement.

---

### Issue #12: take_mk_skipped Leaves Stale mk_order Entry

**File:** `tools/refimpl/quantumshield_refimpl/src/qsp/state.rs:111-114`

```rust
pub fn take_mk_skipped(&mut self, dh: &[u8;32], n: u32) -> Option<[u8;32]> {
    self.mk_skipped.remove(&(dh.clone(), n))
    // mk_order is NOT updated
}
```

**Concern:** Removes from `mk_skipped` HashMap but not from `mk_order` VecDeque. FIFO queue accumulates stale entries.

**Impact:** MEMORY LEAK - Long-lived sessions accumulate dead queue entries.

**Recommendation:** Track and remove stale entries from `mk_order`.

---

### Issue #13: SCKA Monotonicity Check Insufficient

**File:** `tools/refimpl/quantumshield_refimpl/src/suite2/scka.rs:51-52`

```rust
if peer_adv_id <= peer_max_adv_id_seen {
    return Err("REJECT_SCKA_ADV_ID_NOT_MONOTONIC");
}
```

**Concern:** Uses `<=` which rejects duplicates. However, `peer_max_adv_id_seen` is only updated when `commit=true` (line 83-86). Non-committed boundaries with same `adv_id` may be accepted multiple times.

**Impact:** STATE INCONSISTENCY - Could cause peer state divergence.

---

### Issue #14: store_mk_skipped Silent Failure

**File:** `tools/refimpl/quantumshield_refimpl/src/qsp/state.rs:99-109`

```rust
pub fn store_mk_skipped(&mut self, dh: [u8;32], n: u32, mk: [u8;32]) {
    if self.mk_skipped.contains_key(&(dh.clone(), n)) {
        return;  // Silent return on duplicate
    }
    // ...
}
```

**Concern:** Duplicate `(dh, n)` pairs silently fail without updating the key.

**Impact:** DECRYPTION FAILURE - If key was updated between calls, the old key remains.

---

### Issue #15: DH Ratchet Corrupts pn on ns Overflow

**File:** `tools/refimpl/quantumshield_refimpl/src/qsp/ratchet.rs:80-83`

```rust
fn dh_ratchet_send(...) {
    st.pn = st.ns;  // No overflow check
    st.ns = 0;
    // ...
}
```

**Concern:** If `st.ns` is `u32::MAX`, the value is stored in `pn` without validation.

**Impact:** STATE CORRUPTION - Peer desynchronization if session sends MAX messages before boundary.

---

### Issue #16: DoS via Large Collection Deserialization

**File:** `tools/refimpl/quantumshield_refimpl/src/suite2/state.rs:162-181`

```rust
let known_len = c.u32()? as usize;
for _ in 0..known_len {
    recv.known_targets.insert(c.u32()?);
}
```

**Concern:** No bounds check on `known_len`. Attacker-controlled input could specify huge values causing:
- Memory exhaustion
- CPU exhaustion in loop
- No verification bytes are available

**Impact:** DENIAL OF SERVICE - Malicious state data crashes application.

**Recommendation:** Add reasonable bounds: `if known_len > MAX_KNOWN_TARGETS { return Err(...); }`

---

### Issue #17: Multiple Unwraps on header_pt

**File:** `tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs:356-359`

```rust
let header_pn = u32::from_be_bytes([
    header_pt.unwrap()[0],
    header_pt.unwrap()[1],
    header_pt.unwrap()[2],
    header_pt.unwrap()[3],
]);
```

**Concern:** Calls `unwrap()` 4 times on same Option. Redundant and poor style.

**Recommendation:** Extract once: `let pt = header_pt.unwrap();`

---

### Issue #18: Unsafe Unwraps in OPK Handling

**File:** `tools/refimpl/quantumshield_refimpl/src/qsp/handshake.rs:95, 97, 104`

```rust
let (opk_pq_id, opk_pq_pub) = bundle_b.opk_pq.as_ref().unwrap();
let (opk_dh_id, _opk_dh_pub) = bundle_b.opk_dh.as_ref().unwrap();
```

**Concern:** Guarded by prior `is_some()` check but brittle if code is refactored.

**Recommendation:** Use `if let Some(...)` pattern for safer extraction.

---

### Issue #19: State Cloning Proliferates Key Material

**File:** `tools/refimpl/quantumshield_refimpl/src/qsp/ratchet.rs:275`

```rust
let mut tmp = st.clone();  // Clones all sensitive key material
// ... operations on tmp ...
*st = tmp;  // Overwrites original
```

**Concern:** Creates full copy of sensitive state. Original remains in memory until overwritten.

**Impact:** INFORMATION DISCLOSURE - Multiple copies of keys in memory.

**Recommendation:** Use `zeroize` on overwritten state; consider swap semantics.

---

### Issue #20: Mutex::lock().unwrap() in CLI

**File:** `tools/refimpl/quantumshield_refimpl/apps/qshield-cli/src/commands/relay.rs`
**Lines:** 199, 220, 270, 310, 361, 399

```rust
let state = state.lock().unwrap();
```

**Concern:** Panics if mutex is poisoned (another thread panicked while holding lock).

**Impact:** DENIAL OF SERVICE - Crash propagation across threads.

**Note:** Acceptable for demo CLI; would be issue in production code.

---

### Issue #21: MKSKIPPED Removal Without Recovery

**File:** `tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs:234-250`

```rust
if let Some((pn, entry)) = st.mkskipped.iter()
    .find(|(k, _)| k.0 == hdr.pn && ...) {
    let mk = entry.1.clone();
    st.mkskipped.remove(&(*pn, ...));  // Removed before decrypt
    // Then try decrypt...
}
```

**Concern:** Key removed from cache before decryption is verified. If decryption fails, key is permanently lost.

**Impact:** MESSAGE LOSS - Cannot retry decryption if transient failure.

---

### Issue #22: Boundary Message Window Not Enforced

**File:** `tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs:361-362`

```rust
if n != st.nr {
    return Err("REJECT_S2_BOUNDARY_NOT_IN_ORDER");
}
```

**Concern:** `recv_boundary_in_order` requires exact `n == st.nr`, but header decryption tried up to `st.nr + 100` candidates (MAX_HEADER_ATTEMPTS). Resources wasted on known-to-fail boundaries.

**Impact:** RESOURCE EXHAUSTION - Attacker can cause computational waste.

---

### Issue #23: ss3 Entropy Discarded in Handshake

**File:** `tools/refimpl/quantumshield_refimpl/src/qsp/handshake.rs:237, 262`

```rust
let _ = ss3;  // Discarded
```

**Concern:** PQ shared secret `ss3` is computed via KEM but never integrated into session RK.

**Impact:** REDUCED SECURITY - Post-handshake key material doesn't include ss3 entropy.

**Note:** May be intentional per protocol spec - requires verification.

---

## LOW Severity Issues

### Issue #24: Hardcoded ZERO32 Initialization

**File:** `tools/refimpl/quantumshield_refimpl/src/suite2/establish.rs:9, 93-94`

```rust
const ZERO32: [u8; 32] = [0u8; 32];
// Used for uninitialized chains:
ck_ec: ZERO32,
ck_pq: ZERO32,
```

**Note:** Likely intentional per spec. Should be documented to prevent accidental use in crypto operations.

---

### Issue #25: Inconsistent Error Types

**Observation:**
- Suite2 uses: `Result<T, &'static str>` with codes like "REJECT_S2_PARSE_PREFIX"
- QSP uses: `Result<T, CodecError>` enum

**Impact:** Errors not composable across modules; context lost.

---

### Issue #26: Asymmetric Initial State

**File:** `tools/refimpl/quantumshield_refimpl/src/suite2/establish.rs:74-138`

**Concern:** No validation of `role_is_a` parameter. If incorrectly set, both sides initialize with zero chains, causing silent decryption failures.

---

### Issue #27: Signature Verification Order

**File:** `tools/refimpl/quantumshield_refimpl/src/qsp/handshake.rs:185-186`

**Concern:** Signature verified before KT (Key Transparency) verification. Order not enforced by code - relies on caller diligence.

---

### Issue #28: Redundant Safe Unwraps

**Various files**

Many `unwrap_or()` and guarded unwraps throughout. Correct but verbose - could use more idiomatic patterns.

---

## Positive Findings

The codebase demonstrates several strong security practices:

1. **Fail-closed semantics:** Suite2 parsing uses explicit rejection codes
2. **Input validation:** Length checks before crypto operations (establish.rs)
3. **Bounded storage:** MAX_SKIP (1000), MAX_MKSKIPPED (2000), MAX_HKSKIPPED (4)
4. **Overflow protection:** Uses `saturating_add()`, `saturating_sub()`, `checked_add()`
5. **Domain separation:** KDF functions use explicit labels
6. **Hybrid KDF:** Properly combines EC and PQ components
7. **Protocol versioning:** Explicit version checks throughout
8. **Comprehensive test vectors:** 15 vector categories covering all protocol aspects
9. **Strong governance:** DECISIONS.md, TRACEABILITY.md, goal-lint enforcement

---

## Recommendations Summary

### Immediate (Critical)
1. Fix signature verification fallback (stdcrypto.rs:77)
2. Add overflow check in skip key derivation loop (qsp/ratchet.rs:313-320)
3. Replace expect() with Result in sign() (stdcrypto.rs:70)

### Short-term (High)
4. Replace AEAD expect() with Result propagation
5. Verify ck_pq_recv update behavior against spec
6. Fix state mutation ordering in ratchet_encrypt
7. Add runtime validation for struct invariants
8. Implement key zeroization

### Medium-term
9. Add constant-time header key attempts
10. Implement nonce overflow prevention
11. Fix mk_order memory leak
12. Add bounds checks on deserialization
13. Standardize error types

---

## Appendix: Files Analyzed

| File | Lines | Issues Found |
|------|-------|--------------|
| `crypto/stdcrypto.rs` | 90 | #1, #3, #4, #5, #9 |
| `qsp/ratchet.rs` | 359 | #2, #7, #10, #11, #15, #19 |
| `suite2/ratchet.rs` | 615 | #6, #10, #17, #21, #22 |
| `qsp/state.rs` | 431 | #12, #14 |
| `suite2/scka.rs` | 103 | #13 |
| `suite2/state.rs` | 189 | #16 |
| `qsp/types.rs` | 347 | #8 |
| `qsp/handshake.rs` | 273 | #18, #23, #27 |
| `suite2/establish.rs` | 141 | #24, #26 |
| `apps/qshield-cli/src/commands/relay.rs` | ~500 | #20 |

---

 Overall Assessment

  Quality Rating: Strong reference implementation with notable crypto concerns

  Strengths

  1. Excellent protocol design - The Triple Ratchet architecture with hybrid PQ/classical keys is well-conceived. The SCKA (Sparse Continuous Key Agreement) approach achieves post-quantum security without per-message KEM overhead.
  2. Strong governance model - The DECISIONS.md, TRACEABILITY.md, and goal-lint enforcement show mature engineering discipline. Protocol changes require explicit approval through NEXT_ACTIONS.
  3. Comprehensive testing - 15 test vector categories covering KDF, SCKA logic, downgrade resistance, crash/restart, replay, interop, etc. CI gates (4A-4D) enforce conformance.
  4. Fail-closed semantics - The protocol rejects unknown versions, malformed inputs, and ambiguous states with explicit error codes. This is the correct security posture.
  5. Good cryptographic foundations - Proper domain separation in KDFs, hybrid key derivation, transcript binding, and bounded storage limits.

  Concerns

  1. Critical crypto implementation bugs - The signature verification fallback (Issue #1) is a serious vulnerability. Using a zero key on invalid input could allow signature forgery. This is the kind of bug that breaks real-world crypto.
  2. Panic vectors in crypto code - For a security-critical library, expect() and unwrap() in cryptographic paths are problematic. Production crypto code should never panic on attacker-controlled input.
  3. Missing zeroization - The zeroize crate is listed as a dependency but never used. Key material lingering in memory is a real attack surface (cold boot attacks, memory dumps).
  4. State machine edge cases - The nr overflow bug (Issue #2) could cause infinite loops or nonce reuse. These are the subtle bugs that survive code review but cause production incidents.

  Production Readiness

  Not production-ready in current state, which aligns with the project's own --demo-unauthenticated-override requirement. The reference implementation serves its purpose for:
  - Protocol specification validation
  - Test vector generation
  - Interoperability testing

  Before production use, I'd recommend:
  1. Fix the 3 CRITICAL issues
  2. Implement proper error propagation (no panics)
  3. Add key zeroization
  4. Security audit by crypto specialists
  5. Fuzz testing on parsing/state machines

  Verdict

  This is a well-designed protocol with a solid reference implementation that needs hardening before production deployment. The architecture is sound, the governance is excellent, and the test coverage is comprehensive. The issues I found are implementation bugs, not protocol flaws - they're fixable without redesign.

  For a cryptographic protocol at this stage of development, this is better than most. The governance model and traceability documentation suggest the team understands security engineering. The bugs are typical of reference implementations focused on correctness over hardening.
