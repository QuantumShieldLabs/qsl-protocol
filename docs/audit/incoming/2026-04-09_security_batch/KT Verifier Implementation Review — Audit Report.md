## **KT Verifier Implementation Review — Audit Report**

### **1\. Executive Summary**

The Key Transparency (KT) subsystem is the highest-impact unimplemented security component in the repository. Every production `KtVerifier` implementation in the codebase is a stub that either unconditionally passes all KT material or only accepts a trivially empty "KT disabled" shape. No production code verifies STH signatures, Merkle inclusion proofs, consistency proofs, or enforces a log ID pinning policy. The security specifications (`specs/00_security_objectives.md`) declare KT verification **normative and mandatory** for authenticated sessions, but the code enforces none of it. This report audits the current state with the specific focus areas requested: STH signature verification, Merkle inclusion proof, and log ID pinning policy.

---

### **2\. KT Verifier Interface — What Must Be Implemented**

**File**: `tools/refimpl/quantumshield_refimpl/src/kt/mod.rs`

The entire KT subsystem is expressed through a single trait:

Rust  
pub trait KtVerifier {  
    /// Implementations MUST enforce:  
    /// \- log id pinning policy,  
    /// \- STH signature verification,  
    /// \- inclusion proof for the bundle leaf,  
    /// \- consistency proof when provided.  
    fn verify\_bundle(  
        \&self,  
        kt\_log\_id: &\[u8; 32\],  
        kt\_sth: &\[u8\],  
        kt\_inclusion\_proof: &\[u8\],  
        kt\_consistency\_proof: &\[u8\],  
    ) \-\> Result\<(), KtError\>;  
}

The comment on the trait method enumerates all four required sub-checks in the docstring but the actual body is empty. The module docstring explicitly says:

"Wire formats for STH/inclusion/consistency proofs may vary by KT system; therefore this reference skeleton defines an interface and defers the concrete encoding until the project finalizes KT serialization."

This means **zero** of the four checks are implemented anywhere in production paths.

---

### **3\. All `KtVerifier` Implementors — Complete Inventory**

There are exactly four implementors across the entire repository:

| Implementor | File | Behavior | Used in |
| ----- | ----- | ----- | ----- |
| `StubKt` | `tools/refimpl/quantumshield_refimpl/src/kt/mod.rs:36` | Always returns `Err(KtError::NotImplemented)` | Nothing (library stub) |
| `AllowKt` | `tools/refimpl/quantumshield_refimpl/src/qsp/handshake.rs:600` | Always returns `Ok(())` — no checks at all | Unit tests only |
| `KtAllowEmptyOnly` | `tools/actors/refimpl_actor_rs/src/main.rs:1127` | Accepts only if all fields are zero/empty | Refimpl actor CI harness |
| `KtAllowEmptyOnly` | `main.rs:318` | Same pattern as above | Top-level integration harness |

**Critical finding**: None of these perform any cryptographic verification. The two production-adjacent implementations (`KtAllowEmptyOnly`) only check that all KT fields are zeroed/empty — meaning they accept the **absence** of any KT material as valid, and fail-close (returning `Err(KtError::NotImplemented)`) if any non-empty KT material is ever provided. This is a "KT-disabled" mode, not a KT implementation.

---

### **4\. Call Sites for `verify_bundle` — Where KT Must Fire in Production**

#### **4.1 Initiator path — verifying B's prekey bundle**

**File**: `tools/refimpl/quantumshield_refimpl/src/qsp/handshake.rs:99-105`

Rust  
// KT verification of B's identity keys (Authenticated mode)  
deps.kt.verify\_bundle(  
    \&bundle\_b.kt\_log\_id,  
    \&bundle\_b.kt\_sth,  
    \&bundle\_b.kt\_inclusion\_proof,  
    \&bundle\_b.kt\_consistency\_proof,  
)?;

This is the **first thing** called in `initiator_build()`. The `?` propagates failure correctly — if a real `KtVerifier` returns an error, the handshake aborts before any key material is consumed. This is structurally correct and fail-closed.

#### **4.2 Responder path — KT for A's identity**

**File**: `tools/refimpl/quantumshield_refimpl/src/qsp/handshake.rs:258-261`

Rust  
// KT verification of A identity keys (Authenticated mode) – carried in A's bundle in real deployments.  
// In HS1 we only have A's IK pubs; KT proof carriage is in PrekeyBundle, not HS1.  
// Therefore this skeleton expects the caller to have performed KT pinning for A out-of-band or via service.  
// We \*do\* enforce that signature verification occurs.

This is the most significant **architectural gap**: the responder's `responder_process()` function does **not** call `deps.kt.verify_bundle()` at all. The comment acknowledges this and pushes KT for A's identity to the caller "out-of-band." In the existing actor harness (`main.rs:742-763`), the caller constructs A's bundle using `build_prekey_bundle_for()` and never verifies KT proofs on it before passing A's public keys to the responder. This means the responder has **no code path** to enforce KT on the initiator's identity — it relies entirely on the caller.

---

### **5\. STH Signature Verification — Audit**

#### **5.1 Current state**

There is **no STH signature verification** anywhere in the codebase.

The `kt_sth` field is opaque `Vec<u8>` in `PrekeyBundle` (`tools/refimpl/quantumshield_refimpl/src/qsp/types.rs:22`). No parsing, decoding, or signature check is performed on it.

The schema (`docs/schemas/DOC-SCL-002_Shared_Schemas_v1.0.json:141`) names the field:

JSON  
"description": "Canonical STH bytes per KT profile."

But the canonical encoding format, signature algorithm, and signing key are undefined in code. The KTL API schema (`docs/schemas/DOC-SCL-002_Shared_Schemas_v1.0.json:82-96`) shows the service returns `sth_b64` alongside `inclusion_proof_b64`, but neither the key that signs the STH nor the STH wire format is specified.

#### **5.2 What a real implementation must check**

When written, STH signature verification must:

1. **Parse** the `kt_sth` blob into a structure containing at minimum: tree size, timestamp, root hash, and the log's signature over those fields.  
2. **Verify the STH signature** using the pinned KT log public key associated with `kt_log_id`. The signing algorithm must be defined (the system uses Ed25519 and ML-DSA-65 elsewhere; which applies to the KT log's STH is unspecified).  
3. **Enforce freshness**: the STH timestamp must not be older than the configured maximum staleness (`ktl.proof_cache_ttl_seconds` is set to 300s in `docs/spec-closure/DOC-SCL-001_Suite_Parameter_Registry_Deployment_Profiles_v1.0_DRAFT.md:162`).  
4. **Enforce monotonicity**: if the verifier has previously accepted an STH for this log, the new STH's tree size must be ≥ the last seen tree size.

#### **5.3 Key audit risks when implemented**

* **STH key binding**: The log's public signing key must be pinned per `kt_log_id`. If the STH signing key is not pinned to a specific `log_id`, a compromised log can substitute an STH from a different log.  
* **Timestamp handling**: The STH timestamp must be checked against wall clock, not the bundle's `valid_from`/`valid_to` fields. Using the bundle's validity window instead of the STH timestamp would allow stale proofs paired with fresh-looking bundles.  
* **Downgrade of STH signature algorithm**: If the KT log supports multiple STH signing algorithms, the verifier must reject any attempt to negotiate down to a weaker algorithm than the pinned expectation.

---

### **6\. Merkle Inclusion Proof Verification — Audit**

#### **6.1 Current state**

The `kt_inclusion_proof` field is opaque `Vec<u8>` in `PrekeyBundle` (`types.rs:22`). No Merkle computation is performed anywhere.

The schema defines the KTL API:

JSON  
"KtlInclusionProofRequest": { "leaf\_hash\_b64": ..., "tree\_size": ... }  
"KtlInclusionProofResponse": { "sth\_b64": ..., "inclusion\_proof\_b64": ... }

But the **leaf hash computation** is not specified in any code file. Specifically:

* The canonical leaf binding bytes are referenced in the schema (`docs/schemas/DOC-SCL-002_Shared_Schemas_v1.0.json:72-73`) but not defined.  
* It is not specified whether the leaf is `H(bundle_tbs)`, `H(kt_log_id || bundle_tbs)`, or some other construction.  
* The audit path structure (sibling hashes list, proof length) is not defined.

The actor (`tools/actors/refimpl_actor_rs/src/main.rs:1271`) explicitly notes:

"OPKs and KT proofs are not exercised by the current CI harness (it only runs baseline handshake smoke tests)."

The `prekey_bundle_tbs()` function in the actor (`main.rs:1223`) includes `kt_log_id`, `kt_sth`, `kt_inclusion_proof`, and `kt_consistency_proof` in the TBS hash — meaning the bundle signatures cover these fields. This is correct for integrity but does not verify the proof content.

#### **6.2 What a real implementation must check**

1. **Compute the bundle leaf hash**: Hash the bundle's leaf binding bytes according to the defined construction (e.g., `H("leaf:" || bundle_tbs_canonical_bytes)`).  
2. **Verify the Merkle path**: For each sibling hash in the inclusion proof, combine with the current hash per the left/right direction bit, until the root is reconstructed.  
3. **Compare the reconstructed root against the STH root hash** from step 5 above. The STH must have already been signature-verified before this step.  
4. **Verify the leaf index is within the STH tree size**: The proof must not reference a leaf index ≥ tree\_size.

#### **6.3 Key audit risks when implemented**

* **Leaf hash domain separation**: If the leaf hash does not include a prefix (e.g., `0x00 || leaf_data`), a second-preimage attack can substitute a branch node for a leaf node in a proof (RFC 6962 style). The separator convention is unspecified.  
* **Proof length validation**: No maximum proof depth is specified. A malicious server could supply an unbounded proof list; the implementation must cap it at `ceil(log2(max_tree_size))`.  
* **Root extraction from STH**: The STH blob must be correctly parsed to extract the Merkle root hash. If the STH parsing uses a different field offset than the proof computation expects, root comparison silently fails.  
* **Cross-log proof substitution**: The inclusion proof must be tied to the specific STH (and therefore specific log) from which it was obtained. If `kt_log_id` is not threaded through both the proof verification and the STH pinning, a proof from log A could be accepted against an STH from log B.

---

### **7\. Log ID Pinning Policy — Audit**

#### **7.1 Current state**

The `kt_log_id` field is `[u8; 32]` in `PrekeyBundle` (`types.rs:20`). The trait signature exposes it as `&[u8; 32]`.

In both `KtAllowEmptyOnly` implementations:

Rust  
let all\_zero \= kt\_log\_id.iter().all(|\&b| b \== 0);  
if all\_zero && kt\_sth.is\_empty() && ... { Ok(()) } else { Err(KtError::NotImplemented) }

The all-zero `log_id` is the "KT disabled" sentinel. There is **no real pinning policy**: no set of accepted log IDs, no key material bound to any log ID, and no rejection of an unexpected `log_id`.

The schema (`docs/schemas/DOC-SCL-002_Shared_Schemas_v1.0.json:420-422`) lists `kt_log_id_b64` as an **optional preferred** parameter in prekey fetch requests — meaning the client can request a bundle with a specific log's proof. This implies a multi-log architecture is contemplated.

#### **7.2 What a real implementation must check**

1. **Maintain a pinned set of trusted log IDs** and their associated public keys. This set must be hardcoded or distributed via a secured configuration channel (not from the remote peer).  
2. **Reject any `kt_log_id` not in the pinned set** before attempting proof verification.  
3. **Use the public key associated with the pinned `kt_log_id`** to verify the STH signature — not a generic or peer-supplied key.  
4. **Enforce that `kt_log_id` is consistent** between the bundle's field and the STH (the STH must reference the same log it was retrieved from).

#### **7.3 Key audit risks when implemented**

* **Peer-controlled log selection**: If the initiator or responder can influence which log ID is accepted, they can route the verifier to a compromised log they control. The accepted log set must be client-side pinned, not peer-negotiated.  
* **Empty log ID accepted as valid**: The existing `KtAllowEmptyOnly` pattern — accepting all-zero `log_id` — must not persist into any production path. An all-zero log ID must be a hard reject, not an accept.  
* **Log key rotation**: When a KT log rotates its signing key, if the client's pinned key is not updated, all subsequent STH verifications will fail. The key rotation protocol is unspecified; this is an operational risk area.  
* **Single-log vs. multi-log**: If only one log is supported, the pinning is a single 32-byte ID. If multiple logs are allowed, each must have its own independently pinned public key, and the verifier must never mix up keys across logs.

---

### **8\. Bundle Signature Coverage of KT Fields**

The bundle signatures (`sig_ec`, `sig_pq` in `PrekeyBundle`) already cover the KT fields in the TBS encoding:

**Actor `prekey_bundle_tbs()`** (`tools/actors/refimpl_actor_rs/src/main.rs:1254-1258`):

Rust  
// KT material is also authenticated by the bundle signatures.  
w.write\_bytes(\&bundle.kt\_log\_id);  
w.write\_varbytes\_u32(\&bundle.kt\_sth);  
w.write\_varbytes\_u32(\&bundle.kt\_inclusion\_proof);  
w.write\_varbytes\_u32(\&bundle.kt\_consistency\_proof);

**Refimpl `PrekeyBundle::encode()`** (`tools/refimpl/quantumshield_refimpl/src/qsp/types.rs:53-57`) also includes all four KT fields in the canonical encoding, meaning they are covered by `sig_ec`/`sig_pq` when those bundle sigs are verified.

**Gap**: The `refimpl/quantumshield_refimpl` `initiator_build()` does **not** verify `bundle_b.sig_ec` or `bundle_b.sig_pq` before using the bundle's public keys. The comment in `handshake.rs:216` explicitly defers this:

"NOTE: This skeleton defers full bundle signature semantics until BundleTBS is finalized."

This means even the bundle-level signatures — the outer wrapper that authenticates KT fields — are not verified. KT proofs could be swapped in any bundle without detection by the current code.

---

### **9\. Pre-existing Audit Issue \#27 — Signature-Before-KT Ordering**

The 2026-01-04 code analysis (`docs/audit/CODE_ANALYSIS_REPORT_20260104.md:556-560`) identified:

"Signature verified before KT (Key Transparency) verification. Order not enforced by code — relies on caller diligence."

The test plan for this (`docs/archive/testplans/AUDIT-20260104_issue27_sig_verify_order_testplan.md`) addresses this as a cheapness-of-rejection concern (structural checks before expensive crypto). However, the deeper ordering issue is different: KT should be verified **before** trusting any public key material from the bundle, because the bundle's public keys are what get signed in the handshake transcript. If KT is bypassed, the responder in `responder_process()` accepts a caller-supplied `ik_sig_ec_b_pub`/`ik_sig_pq_b_pub` for a principal whose log inclusion has never been verified.

---

### **10\. Normative Requirement Status**

The security specification (`specs/00_security_objectives.md:100-106`) states:

"Implementations claiming conformance to QSP v4.3.2 MUST:

* Perform KT verification for identity bindings required by QSP.  
* Enforce rollback/freshness policies at least as strict as QSP's minimum requirements.  
* Treat KT validation failures as **fatal** for authenticated sessions."

**Current status**: Zero of these three requirements are satisfied by any non-test, non-stub code in the repository. The `KtAllowEmptyOnly` pattern used in all live harnesses accepts the absence of KT material as valid — the opposite of treating failures as fatal.

---

### **11\. Summary Table**

| KT Check | Required by spec | Implemented | Code location | Gap |
| ----- | ----- | ----- | ----- | ----- |
| Log ID pinning | ✓ (`kt/mod.rs:22`) | ✗ | `KtAllowEmptyOnly`: only checks all-zero | No pinned log set, no key binding |
| STH signature verification | ✓ (`kt/mod.rs:22`) | ✗ | Not called anywhere | STH format undefined; no signing key; no parse |
| Freshness / monotonicity | ✓ (`specs/00_security_objectives.md:105`) | ✗ | Not checked | STH timestamp not parsed or compared |
| Merkle inclusion proof | ✓ (`kt/mod.rs:22`) | ✗ | Not computed | Leaf hash undefined; audit path format undefined |
| Consistency proof | ✓ (`kt/mod.rs:22`) | ✗ | Not computed | Format undefined |
| Bundle sig covers KT fields | ✓ (tbs encoding) | Partially | `types.rs:53-57` encoding includes them | Bundle `sig_ec`/`sig_pq` not verified in initiator path |
| Responder KT check for initiator | ✓ | ✗ | `handshake.rs:258-261` (comment only) | No `deps.kt.verify_bundle()` call; deferred to caller |
| KT failure is fatal | ✓ (`specs/00_security_objectives.md:106`) | ✗ | `KtAllowEmptyOnly` accepts empty — not fatal | Empty KT is accepted, non-empty errors as unimplemented |

