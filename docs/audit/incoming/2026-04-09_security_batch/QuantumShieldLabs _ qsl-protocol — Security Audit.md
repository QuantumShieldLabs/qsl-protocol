# **QuantumShieldLabs / qsl-protocol — Security Audit**

## **1\. Repository Overview**

| Attribute | Details |
| ----- | ----- |
| **Language** | Rust (100%) |
| **Build system** | Cargo workspaces (multiple crates) |
| **Major components** | `tools/refimpl/quantumshield_refimpl` — cryptographic reference implementation (library); `qsl/qsl-client/qsc` — production TUI client; `apps/qshield-cli` — demo CLI; `qsl/qsl-server` — relay service |
| **Nature** | Production client \+ protocol reference implementation with demo scaffolding. The refimpl ships as a production dependency of `qsc` (`Cargo.toml: quantumshield_refimpl = { path = "...", features = ["pqcrypto"] }`). |
| **Protocols** | QSP v4.3 (classic X3DH-style \+ Double Ratchet); QSP v5.0 / Suite-2 (upgraded PQ-hybrid ratchet); QSE envelope (traffic-analysis mitigation) |
| **PQ primitives** | ML-KEM-768 (KEM), ML-DSA-65 (signatures), X25519 (classic DH), Ed25519 (classic signatures), AES-256-GCM or ChaCha20-Poly1305 (AEAD), KMAC-256 (KDF/MAC), Argon2id (vault KDF) |

---

## **2\. Cryptography Audit**

### **2.1 Primitive Selection**

The protocol uses modern, post-quantum-safe primitives throughout:

| Primitive | Alg | Status |
| ----- | ----- | ----- |
| KEM | ML-KEM-768 (`pqcrypto-mlkem 0.1.1`) | ✅ Standardised (NIST FIPS 203\) |
| PQ Sig | ML-DSA-65 (`ml-dsa 0.1.0-rc.7`) | ⚠️ Pre-release RC; RUSTSEC-2025-0144 timing bug (see §5) |
| AEAD | AES-256-GCM (QSP) / ChaCha20-Poly1305 (vault) | ✅ |
| Hash | SHA-512 (`sha2 0.10`) | ✅ |
| MAC/KDF | KMAC-256 (`tiny-keccak 2`) | ✅ |
| DH | X25519 (`x25519-dalek 2`) | ✅ |
| Classic Sig | Ed25519 (`ed25519-dalek 2`) | ✅ |
| Vault KDF | Argon2id (`argon2 0.5`) | ⚠️ Low memory parameter (see §4) |

No deprecated primitives (MD5, SHA-1, ECB mode, RSA-PKCS1v1.5) were found.

### **2.2 Nonce / IV Generation**

**QSP legacy protocol** (`qsp/ratchet.rs`):

* **Header nonce**: Random 12 bytes from `OsRng` via `Rng12::random_nonce12()` — correct.  
* **Body nonce**: Deterministic — `SHA-512("QSP4.3/BODY-NONCE" || session_id || dh_pub || n)[0..12]`. Since `(session_id, dh_pub, n)` is unique per session epoch and per-message counter, this is collision-safe.

**Suite-2** (`suite2/ratchet.rs:145-167`):

* Both header and body nonces are deterministic, derived from `SHA-512(label || session_id || dh_pub || n)`. Labels differ (`QSP5.0/HDR-NONCE` vs `QSP5.0/BODY-NONCE`), preventing cross-cipher nonce reuse.  
* `dh_pub` used in Suite-2 nonce derivation is the **receiver's stored state `st.dh_pub`**, not the wire message's `dh_pub`. This is consistent but means the nonce doesn't bind to the sender-supplied key in the incoming message envelope (body layer only; header layer does bind via the AD).

**Vault** (`vault/mod.rs:241, 267, 352`):

* Nonce generated fresh via `ChaCha20Poly1305::generate_nonce(&mut OsRng)` on every write — correct, no nonce reuse.

### **2.3 Key Derivation**

KDF hierarchy for QSP/Suite-2:

Code  
rk0 \= KMAC-256(H(ss1 || \[ss2\] || dh1 || \[dh2\]), "QSP4.3/RK0", session\_id, 32\)  
rk  \= KMAC-256(rk0, "QSP4.3/RK0/SS3", session\_id || ss3, 32\)  
(hk\_s, hk\_r, nhk\_s, nhk\_r) \= KMAC-256(rk, "QSP4.3/HK/A-\>B" | "B-\>A" | "NHK/...", 0x01, 32\)  
(rk', ck)  \= KMAC-256(rk, "QSP4.3/RKDH", dh\_out, 64)\[0..32, 32..64\]  
(ck', mk)  \= KMAC-256(ck, "QSP4.3/CK"/"MK", 0x01/0x02, 32\)

Domain separation via distinct string labels — good.

**Finding**: In `derive_rk0` (`qsp/handshake.rs:37-61`), the optional `ss2` and `dh2` are concatenated without a length prefix or presence indicator:

Rust  
let mut m \= b"QSP4.3/MS".to\_vec();  
m.extend\_from\_slice(ss1);  
if let Some(s) \= ss2 { m.extend\_from\_slice(s); }  
m.extend\_from\_slice(dh1);  
if let Some(d) \= dh2 { m.extend\_from\_slice(d); }

Since ML-KEM-768 shared secrets are always 32 bytes and X25519 outputs are always 32 bytes when present, the absence vs. presence of `ss2`/`dh2` creates different total lengths, making collisions impossible in practice. However, explicit length markers or a counter (`0x01`/`0x02`) for optional fields would be more principled.

### **2.4 Constant-Time Operations**

* `ed25519-dalek 2.x` uses `subtle` for constant-time comparisons ✅  
* `x25519-dalek 2.x` performs Curve25519 DH in constant time ✅  
* `aes-gcm 0.10` uses `aes` crate which is constant-time on platforms with AES-NI ✅

**Finding (RUSTSEC-2025-0144)**: `ml-dsa 0.1.0-rc.7` has a confirmed timing side channel in signature verification. This advisory is **explicitly suppressed** in `.cargo/audit.toml`:

TOML  
\[advisories\]  
ignore \= \["RUSTSEC-2025-0144"\]  \# timing-fix upgrade is not drop-in

The justification states "tooling-only," but `qsc` lists `quantumshield_refimpl` with `features = ["pqcrypto"]` as a **non-dev dependency**, meaning this ML-DSA implementation is in the production binary.

`is_zero32` in `suite2/ratchet.rs:36-38` uses a data-dependent branch, not constant-time:

Rust  
fn is\_zero32(v: &\[u8; 32\]) \-\> bool {  
    v.iter().all(|b| \*b \== 0\)  
}

Used as a "chain key is unset" sentinel check (not secret data comparison), so timing is not critical here.

### **2.5 Secure Randomness**

`OsRng` from `rand_core` / `rand 0.8` is used throughout for key generation. However:

**Finding — Deterministic RNG backdoor** (`qsl/qsl-client/qsc/src/handshake/mod.rs:266-287`):

Rust  
fn hs\_rand\_bytes(label: \&str, len: usize) \-\> Vec\<u8\> {  
    if let Some(seed) \= hs\_seed\_from\_env() {  // reads QSC\_HANDSHAKE\_SEED  
        let seed\_hash \= c.sha512(\&seed\_bytes);  
        return c.kmac256(\&seed\_key, label, b"", len);  
    }  
    OsRng.fill\_bytes(\&mut out);  
    out  
}

Setting `QSC_HANDSHAKE_SEED` to any integer replaces `OsRng` for all handshake randomness: ephemeral KEM keypairs, DH ephemeral keys, and session IDs. An attacker who can observe or control this variable in a production deployment can predict and reconstruct all handshake secrets for that session.

### **2.6 Signature Verification**

* `responder_process` verifies both Ed25519 and ML-DSA-65 signatures over the HS1 transcript before proceeding (`handshake.rs:262-273`) ✅  
* `initiator_finalize` verifies both signatures over the HS2 transcript before accepting (`handshake.rs:391-401`) ✅  
* `conf_b` is a KMAC-256 MAC over `rk0` and `hs2_hash`, providing an additional session binding check (`handshake.rs:409-412`) ✅  
* Transcript format: `SHA-512("QSP4.3/HS1" || HS1_bytes_with_zero_sigs)` — correct signature-over-hash-of-message pattern ✅

The handshake transcript in `qsc/src/handshake/mod.rs` (`hs_transcript_mac`, `hs_transcript_hash`) uses KMAC keyed on `pq_init_ss` — this binds the transcript authentication to the PQ KEM shared secret ✅.

**Finding**: KT verification is never performed. `StubKt::verify_bundle` always returns `Err(KtError::NotImplemented)` (`kt/mod.rs:42-47`). In `initiator_build` and `responder_process`, `deps.kt.verify_bundle(...)` is called, but if an `AllowKt` stub (which always succeeds, present in test code) is substituted in production, identity keys would never be verified against the transparency log.

### **2.7 X25519 Low-Order Points**

`StdCrypto::dh` uses `x25519_dalek::StaticSecret::diffie_hellman`:

Rust  
fn dh(\&self, privk: \&X25519Priv, pubk: \&X25519Pub) \-\> \[u8; 32\] {  
    let sk \= StaticSecret::from(privk.0);  
    let pk \= PublicKey::from(pubk.0);  
    (sk.diffie\_hellman(\&pk)).to\_bytes()  
}

`x25519-dalek 2.x` with `StaticSecret` does **not** perform a low-order-point check — it intentionally uses the "non-contributory" semantics per RFC 7748\. A peer supplying a low-order point (e.g., `[0u8; 32]`) will cause DH to return all-zeros. No all-zeros DH output check is present. In QSP, if `dh_out = [0; 32]`, then `KMAC-256([0;32], "QSP4.3/RKDH", [0;32], 64)` is the new ratchet step — predictable but still a KMAC output, not a zero key. Risk is low because the KDF absorbs it, but a small-subgroup attack against X25519 is worth guarding against.

---

## **3\. Protocol / Message Security**

### **3.1 Replay Protection**

**QSP ratchet**: Per-message counter `ns`/`nr` enforced; `MAX_SKIP=1000` bounds the OOO window. Skipped message keys are stored with `(dh_pub, n)` keys and consumed on use (`take_mk_skipped`) ✅.

**Suite-2**: Monotonically increasing `peer_adv_id` for SCKA events (`scka.rs:52-54`); `consumed_targets` and `tombstoned_targets` sets prevent PQ ciphertext reuse ✅.

**QSE envelope**: `timestamp_bucket` is set by the service edge but is not verified by the receiver in the parsed code path (`qse/envelope.rs`). There is no sequence number or nonce in the envelope itself. Replay at the envelope layer is possible if an adversary can replay packets before the timestamp bucket rolls over.

### **3.2 Parsing / Length Checks**

All parser code uses the `Reader` abstraction with bounds checking (`codec/mod.rs`). Fixed-length fields use `read_exact::<N>()`, variable-length use `read_varbytes_u16()` / `read_varbytes_u32()` with explicit remaining-bytes checks. `reader.finish()` rejects trailing bytes ✅.

**Finding — Silent truncation in `write_varbytes_u16`** (`codec/mod.rs:106-109`):

Rust  
pub fn write\_varbytes\_u16(\&mut self, b: &\[u8\]) {  
    self.write\_u16(b.len() as u16);  // truncates silently if len \> 65535  
    self.write\_bytes(b);             // writes full slice  
}

If `b.len() > 65535`, the length prefix is silently truncated. The wire becomes malformed: the reader will read `truncated_len` bytes from the field boundary and interpret the remainder as the next field, causing full message deserialization failure or worse, phantom-field aliasing. Fields using this path include `kt_sth`, `kt_inclusion_proof`, `kt_consistency_proof`, and `user_id` — KT proofs can legitimately exceed 64 KiB.

**Finding — Suite-2 wire format: 16-bit body length** (`suite2/parse.rs:146-148`):

Rust  
let body\_len \= u16::from\_be\_bytes(\[buf\[8\], buf\[9\]\]) as usize;

Maximum message body \= 65535 bytes. QSP legacy uses 32-bit body length. Large payloads silently fail with parsing errors rather than an explicit max-size rejection. No DoS protection via explicit max-body size check.

### **3.3 Session Binding**

The header AD includes `session_id || protocol_version || suite_id || dh_pub || flags || pq_bind` ✅.  
 The body AD in **Suite-2** (`suite2/binding.rs:36-43`) includes `session_id || protocol_version || suite_id || pq_bind` but **does not include `dh_pub`**. The `dh_pub` is present in the header AD but absent from body AD. While the body key (mk) is implicitly bound to the ratchet epoch via the chain key, the explicit AD omission means a body ciphertext could theoretically be tested against a different epoch's AD without triggering an AD mismatch (though the key would still differ). This is a defense-in-depth gap.

### **3.4 Downgrade / Version Negotiation**

Both QSP legacy and Suite-2 parsers check `protocol_version` and `suite_id` against hardcoded expected values and reject mismatches (`types.rs:177-185`, `parse.rs:150-155`). Unknown flags are rejected ✅. There is no multi-version negotiation in the wire protocol — the version is asserted, not negotiated, preventing downgrade.

---

## **4\. Secret Handling and Storage**

### **4.1 Vault Encryption**

The vault uses ChaCha20-Poly1305 with a fresh random nonce per write, keyed by an Argon2id-derived key ✅. `VaultSession` implements `Drop` with `zeroize()` on key bytes and secret string values ✅.

**Finding — Low Argon2id parameters** (`vault/mod.rs:29-31`):

Rust  
const KDF\_M\_KIB: u32 \= 19456; // 19 MiB  
const KDF\_T: u32 \= 2;  
const KDF\_P: u32 \= 1;

OWASP 2023 minimum recommendation: m=47104 (46 MiB), t=1, p=1. The memory cost here is \~41% of that minimum. These parameters are stored in the vault envelope and could be overwritten by an attacker who obtains the vault file to reduce brute-force cost on a stolen file.

**Finding — Process-level passphrase cache** (`vault/mod.rs:679, 985-1001`):

Rust  
static PROCESS\_PASSPHRASE: OnceLock\<Mutex\<Option\<String\>\>\> \= OnceLock::new();

The vault passphrase is cached in process memory for the lifetime of the TUI session. Any memory dump, `/proc/PID/mem` read, or core dump exposes the passphrase in plaintext. The passphrase is not wrapped in a `Zeroizing<String>` type.

**Finding — Hardcoded MockProvider key** (`vault/mod.rs:733-735`):

Rust  
4 \=\> {  
    \*out \= \[0x42u8; 32\];  
    Ok(())  
}

Key source tag `4` bypasses all KDF and uses a fixed key `0x42...42`. The function `unlock_if_mock_provider()` auto-unlocks without credentials if the vault was initialized with this source. This constitutes a hardcoded backdoor key.

### **4.2 Session State Serialization**

`SessionState::snapshot_bytes()` (`qsp/state.rs:171-305`) serializes the full session state including:

* `dh_self.0` — X25519 private key  
* `ck_s`, `ck_r` — chain keys  
* `hk_s`, `hk_r`, `nhk_s`, `nhk_r` — header encryption keys  
* `mk_skipped` entries — cached message keys

The output `Vec<u8>` is not zeroized after use. Private DH key in `X25519Priv` has `ZeroizeOnDrop`, but once extracted to a `[u8; 32]` and placed in the snapshot `Vec<u8>`, it lives until the allocator decides to reuse it.

PQ private keys in `pq_self: HashMap<u32, (Vec<u8>, Vec<u8>)>` are plain `Vec<u8>` without `ZeroizeOnDrop`. Same for `HandshakePending.kem_sk`.

### **4.3 Secret Logging**

No direct logging of key material found in the main code paths. Error codes use static strings without embedding runtime key values. Marker output functions (`emit_marker`) emit structured JSON with controlled keys. ✅

**Finding**: `establish.rs` logs hex-encoded `dh_init` and `pq_init_ss` to the relay server (`EstablishRecordRequest` at line 138-147). These are demo-only derived values (not real session secrets in the full handshake), but in any real deployment, transmitting KEM shared secret material to a relay constitutes secret leakage.

---

## **5\. Dependencies and Supply Chain**

| Crate | Version | Risk |
| ----- | ----- | ----- |
| `ml-dsa` | `0.1.0-rc.7` | **RUSTSEC-2025-0144** — timing side channel in ML-DSA-65 verify. Suppressed in audit.toml. |
| `pqcrypto-mlkem` | `0.1.1` | Pre-1.0 FFI wrapper around C reference implementation; interface stability risk |
| `rand` | `0.8` | Older minor version; `0.9.x` exists. No known vulnerabilities |
| `argon2` | `0.5` | Current stable |
| `ed25519-dalek` | `2` | Current stable |
| `x25519-dalek` | `2` | Current stable |
| `qsl-attachments` | git pin `59f632f...` | External git dependency pinned to a specific commit — good for reproducibility |

**RUSTSEC-2025-0144 detail**: ML-DSA-65 signature verification in `ml-dsa 0.1.0-rc.7` leaks timing information that may allow an attacker to extract signing key bits via a timing oracle. In `qsc/src/handshake/mod.rs:399-426`, `c.verify(sig_pk, msg, sig)` is called during handshake processing over a network connection. A network-accessible adversary sending crafted signatures could time the responses and eventually recover key material.

No vendored crypto or custom forks found. All crypto crates are standard crates.io dependencies.

---

## **6\. Fuzzing and Testing**

### **6.1 Existing Tests**

* Property/unit tests exist for ratchet OOO (`suite2_bounded_receive.rs`), SCKA replay rejection (`scka.rs`), handshake transcript binding (`suite2_handshake_security.rs`), MK-skipped management (`state.rs`), and parse-only vectors (`parse_only_vectors.rs`)  
* Integration tests in `qsc/tests/` including `desktop_gui_contract_na0215b.rs`  
* Test vectors for parsing (JSON format)

### **6.2 Gaps**

| Gap | Risk |
| ----- | ----- |
| No fuzz harness for `decode_suite2_ratchet_message` / `decode_suite2_wire` | Parser bugs, integer overflows |
| No fuzz harness for `Envelope::decode` / `suite2_wire_prefix_len` | Bucket-mode path length calculation |
| No property test for nonce uniqueness across sessions | Nonce reuse regression |
| No negative test for X25519 low-order point input to DH | Small-subgroup silent acceptance |
| No adversarial test for `write_varbytes_u16` overflow path | Truncation/desync |
| No test for vault with attacker-controlled KDF parameters | KDF downgrade |
| `DummyKmac` (returns constant zero for all inputs) is used in handshake security tests | False positive on auth property tests |

---

## **7\. Prioritized Findings**

### **CRITICAL**

**C-1: ML-DSA-65 Timing Side Channel — RUSTSEC-2025-0144 Active in Production Binary**

* **File**: `.cargo/audit.toml` (suppression); `qsc/Cargo.toml:21` (dependency); `qsc/src/handshake/mod.rs:399-426` (call site)  
* **Impact**: Network attacker can time ML-DSA-65 verify calls during handshake to recover signing key bits. The advisory is suppressed with a "tooling-only" justification that is factually incorrect.  
* **Remediation**: Upgrade to a fixed version of `ml-dsa` once available, or temporarily substitute a constant-time ML-DSA implementation. Remove the `audit.toml` suppression.

**C-2: `QSC_HANDSHAKE_SEED` Deterministic RNG Backdoor**

* **File**: `qsl/qsl-client/qsc/src/handshake/mod.rs:266-287`  
* **Impact**: Setting the environment variable replaces `OsRng` for all handshake randomness. Any CI pipeline log, environment leak, or attacker with shell access that sets `QSC_HANDSHAKE_SEED` can reconstruct ephemeral KEM keypairs and DH keys for that session.  
* **Remediation**: Remove the `QSC_HANDSHAKE_SEED` path from all non-test code. Gate it behind a `#[cfg(test)]` or `#[cfg(feature = "test-vectors")]` feature flag that is never enabled in production builds.

**C-3: Hardcoded Vault MockProvider Key (`key_source=4`)**

* **File**: `qsl/qsl-client/qsc/src/vault/mod.rs:733-735`; `vault/mod.rs:198-220` (`unlock_if_mock_provider`)  
* **Impact**: Any vault file initialized with `key_source=4` is trivially decryptable with the known key `[0x42; 32]`. `unlock_if_mock_provider` auto-unlocks without credentials. If any user accidentally initializes with this source (or a bug routes them there), all secrets including PQ private keys are exposed.  
* **Remediation**: Remove `key_source=4` from production code entirely. If needed for testing, move to a test-only binary or feature flag.

---

### **HIGH**

**H-1: `authenticated` Flag Logic Inversion in Demo CLI**

* **File**: `apps/qshield-cli/src/commands/establish.rs:127`  
* **Impact**: The JSON parameter `"authenticated": { "bool": demo_unauthenticated_override }` sends `authenticated=false` by default, which causes `init_from_base_handshake` to unconditionally reject all sessions. The only working path is to pass `--demo-unauthenticated-override`, which sets `authenticated=true`. The misleading flag name causes operators to believe they are explicitly allowing unauthenticated mode when in fact the flag enables authentication acceptance. This confuses the security model.  
* **Remediation**: Rename the flag to `--demo-force-authenticated` and/or invert the boolean: the default should be to attempt authenticated establishment; unauthenticated should be an explicit, differently-named override.

**H-2: KT (Key Transparency) Verification Never Performed**

* **File**: `tools/refimpl/quantumshield_refimpl/src/kt/mod.rs:37-47`; `qsp/handshake.rs:99-105`  
* **Impact**: `StubKt` always errors, so identity key binding to a KT log is never verified. Impersonation attacks on long-term identity keys are undetected. In a production deployment using a passthrough KT verifier, this completely negates the KT security property.  
* **Remediation**: Ship a real KT verifier before production deployment. Add a build-time or runtime assertion that `StubKt` is never used outside of tests. Mark `StubKt` with `#[cfg(test)]`.

**H-3: `write_varbytes_u16` Silent Length Truncation**

* **File**: `tools/refimpl/quantumshield_refimpl/src/codec/mod.rs:106-109`  
* **Impact**: Fields larger than 65535 bytes (KT inclusion proofs, consistency proofs) silently get a wrong length prefix. The encoded wire message is malformed. The peer's decoder will read the wrong number of bytes, causing subsequent fields to alias, potentially producing phantom message structures that authenticate as valid.  
* **Remediation**: Replace `b.len() as u16` with a checked cast: if `b.len() > u16::MAX`, return an error rather than silently truncating.

**H-4: PQ Private Keys Not Zeroized on Drop**

* **File**: `qsp/state.rs:64` (`pq_self`); `handshake/mod.rs:78` (`kem_sk`); `qsp/state.rs:171-305` (`snapshot_bytes`)  
* **Impact**: ML-KEM-768 private keys live in plain `Vec<u8>` without `ZeroizeOnDrop`. Session snapshots expose raw private key bytes in an unerased heap buffer. After `SessionState` is dropped, private key material remains in freed memory pages.  
* **Remediation**: Wrap PQ private keys in `Zeroizing<Vec<u8>>` throughout. Ensure `snapshot_bytes()` output is wrapped in `Zeroizing<Vec<u8>>` at the call site.

---

### **MEDIUM**

**M-1: Argon2id Memory Factor Below Recommended Minimum**

* **File**: `qsl/qsl-client/qsc/src/vault/mod.rs:29-31`  
* **Impact**: `KDF_M_KIB=19456` is \~41% of the OWASP minimum (47104 KiB). KDF parameters are stored inside the vault file and could be overwritten to weaker values in a stolen-file attack.  
* **Remediation**: Increase to at least `m=46080` (45 MiB). Add a minimum-parameter validation in `derive_runtime_key` that rejects envelopes with parameters below a hardcoded floor.

**M-2: Plaintext Passphrase in Process-Global Static**

* **File**: `qsl/qsl-client/qsc/src/vault/mod.rs:679, 995-1001`  
* **Impact**: The vault unlock passphrase is stored as `Option<String>` in a static `Mutex`. Memory dumps, `/proc` reads, or crash reports expose the passphrase in cleartext.  
* **Remediation**: Use `Zeroizing<String>` for the passphrase cache. Consider deriving and caching the key (`[u8; 32]`) rather than the passphrase, zeroizing on vault re-lock.

**M-3: Suite-2 Body AD Missing `dh_pub`**

* **File**: `tools/refimpl/quantumshield_refimpl/src/suite2/binding.rs:36-43`  
* **Impact**: Body AD does not include the sender's ephemeral DH public key. The body ciphertext is not explicitly bound to a specific ratchet epoch at the AD level (only implicitly, via the chain key). A cross-epoch confusion scenario (crafted message with same session\_id and epoch body key) is not closed at the AD layer.  
* **Remediation**: Include `dh_pub` in `ad_body` to match the coverage of `ad_hdr`.

**M-4: `secure_delete_file` Not Cryptographically Effective**

* **File**: `apps/qshield-cli/src/fsutil.rs:32-61`  
* **Impact**: Overwriting with zeros is not reliable on SSDs (wear-leveling), journaled filesystems, or COW filesystems (Btrfs, ZFS, APFS). The function implies secure erasure to callers while providing none.  
* **Remediation**: Document that this is best-effort on modern filesystems. Prefer key-wrapping (encrypt secrets, destroy only the wrapping key) over file erasure.

**M-5: Demo Session Secrets Are Deterministic / Predictable**

* **File**: `apps/qshield-cli/src/util.rs:35-73`  
* **Impact**: `demo_session_id_bytes`, `demo_dh_init_bytes`, `demo_pq_init_bytes` derive session material from SHA-256 of known-public IDs and public keys. Any observer who knows the participants' IDs can predict these values, undermining forward secrecy for demo sessions. These are labeled as demo-only but are used in the actual session establishment path.  
* **Remediation**: These should only be used in isolated test environments. Add a runtime assertion that these functions are never called when a real KEM/DH is available.

**M-6: `evict_mkskipped` May Evict Recently-Needed Keys**

* **File**: `tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs:40-56`  
* **Impact**: When `mkskipped.len() > MAX_MKSKIPPED`, the oldest (lowest `n`) entries are evicted. Under deliberate OOO message flooding, an attacker can force eviction of keys needed to decrypt legitimate messages, creating a targeted DoS.  
* **Remediation**: Implement per-epoch key count limits in addition to global limits, so a flooder in one epoch cannot starve other epochs.

**M-7: `dh_init` and `pq_init_ss` Sent to Relay Server**

* **File**: `apps/qshield-cli/src/commands/establish.rs:138-156`  
* **Impact**: `EstablishRecordRequest` transmits hex-encoded `dh_init` and `pq_init_ss` to the relay. In the demo these are synthetic deterministic values, but the relay stores them, creating a log of "session key material." Any relay compromise exposes this data.  
* **Remediation**: Remove these fields from the relay record. Session establishment should be relay-blind; only session identifiers (not key material) should be stored server-side.

---

### **LOW**

**L-1: X25519 Low-Order Point — No All-Zeros DH Output Check**

* **File**: `tools/refimpl/quantumshield_refimpl/src/crypto/stdcrypto.rs:145-151`  
* **Impact**: A malicious peer supplying a low-order point causes DH output to be all-zeros. This is absorbed into KMAC-based KDF, so the resulting session key is not literally zero, but the session's PFS is broken for that exchange. `x25519-dalek 2.x`'s `StaticSecret` API does not reject low-order points.  
* **Remediation**: After `dh()`, check that the output is not all-zeros and return `CryptoError::InvalidKey` if so.

**L-2: `DummyKmac` Returns Constant Zero in Auth Tests**

* **File**: `tools/refimpl/quantumshield_refimpl/src/qsp/handshake.rs:451-455` (test code)  
* **Impact**: `DummyKmac::kmac256` returns `vec![0u8; outlen]` regardless of input. Tests using this mock (including handshake confirmation tests) verify structural behavior but may give false positives on authentication properties (e.g., confirmation MACs).  
* **Remediation**: Replace `DummyKmac` with `TestKmac` (the accumulator version already defined) in tests that check authentication outcomes.

**L-3: Role Selection by Lexicographic String Comparison**

* **File**: `apps/qshield-cli/src/commands/establish.rs:84-89`  
* **Impact**: `if my_id.as_str() <= peer_id` determines protocol role A/B. Two users with equal IDs (highly unlikely but possible) would both select role A, causing a protocol mismatch. Not exploitable in practice.  
* **Remediation**: Add a specific check for `my_id == peer_id` to return an error.

**L-4: `is_zero32` as Presence Sentinel Has Negligible False-Positive Risk**

* **File**: `suite2/ratchet.rs:36-38`  
* **Impact**: If a legitimate KDF output happens to be all-zeros (probability 2^-256), the chain key is treated as unset and messages are rejected. Not exploitable but is a design smell.  
* **Remediation**: Use an explicit `Option<[u8; 32]>` for chain key presence instead of a sentinel value.

---

## **8\. Suggested Follow-Up Audits**

1. **ML-DSA-65 Timing Oracle Profiling**: Measure timing variance of `c.verify()` in `handshake/mod.rs` under a controlled network environment. Quantify exploitability against an attacker with 10^6 signature verification opportunities.

2. **KT Verifier Implementation Review**: When a real KT verifier is written, audit the STH signature verification, Merkle inclusion proof, and log ID pinning policy. This is the highest-impact unimplemented security component.

3. **PQ KEM Decapsulation Failure Handling**: Review all paths where `pq_kem.decap()` returns `CryptoError::InvalidKey` to ensure they do not provide oracle information (timing or error type). ML-KEM-768 is specified to be implicit rejection safe, but implementation-specific oracles may exist in `pqcrypto-mlkem`.

4. **Handshake Transcript Binding Completeness**: Verify that HS1 and HS2 transcripts (`hs1_transcript`, `hs2_transcript`) include all mandatory fields that must be bound to the session. Specifically, confirm that the PQ receive key advertisement (`pq_rcv_a_pub`, `pq_rcv_b_pub`) cannot be replaced by a MITM between HS1 and HS2 without triggering a transcript mismatch.

5. **Nonce Uniqueness Regression Fuzzing**: Create a libFuzzer harness for `send_wire` / `recv_wire` in Suite-2 that tracks all `(key, nonce)` pairs across a sequence of messages and asserts no pair appears twice.

6. **Parser Fuzzing**: Create fuzzing harnesses for `decode_suite2_wire`, `Envelope::decode`, `HandshakeInit::decode`, `HandshakeResp::decode`, and `PrekeyBundle::decode` using `cargo-fuzz`. The `suite2_wire_prefix_len` bucket-mode arithmetic is a particular target.

7. **Vault File Format Adversarial Testing**: Fuzz `parse_envelope` with attacker-controlled KDF parameters (sub-minimum `kdf_m_kib`, `kdf_t=0`, very large values), and verify that the validation floor added in M-1's fix correctly rejects all below-threshold configurations.

