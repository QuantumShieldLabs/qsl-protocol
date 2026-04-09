## **Audit \#5 — Nonce Uniqueness Regression Fuzzing**

### **Scope statement**

**Goal:** Design a libFuzzer harness targeting `send_wire` and `recv_wire` in the Suite-2 refimpl that drives arbitrary sequences of messages and asserts that no `(key, nonce)` pair appears twice across the entire sequence.

**Constraint:** No code changes. All file references point to primary sources in the repo.

---

### **1\. What constitutes a (key, nonce) pair in Suite-2**

The spec (DOC-CAN-003 §5.2) defines two distinct nonce constructions, both in `tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs` at lines 145–167:

Code  
nonce\_hdr(session\_id, dh\_pub, N)  \= SHA-512("QSP5.0/HDR-NONCE"  || session\_id || dh\_pub || u32be(N))\[0:12\]  
nonce\_body(session\_id, dh\_pub, N) \= SHA-512("QSP5.0/BODY-NONCE" || session\_id || dh\_pub || u32be(N))\[0:12\]

These functions have **three** free variables — `session_id`, `dh_pub`, and `N` — and they are keyed by distinct AEAD keys:

| Operation | AEAD key | Nonce inputs |
| ----- | ----- | ----- |
| Header seal/open | `hk_s` / `hk_r` | `(session_id, dh_pub, ns)` |
| Body seal/open | `mk` (per-message hybrid) | `(session_id, dh_pub, ns)` |

`dh_pub` is the sender's current X25519 public key (fixed within a DH epoch; changes only on a boundary). `N = st.ns` for send (line 804, 810), advancing monotonically via `saturating_add(1)` (line 836). For the header key `hk_s`, the tuple `(hk_s, nonce_hdr(session_id, dh_pub, N))` must never repeat. For the body key `mk`, the pair `(mk, nonce_body(session_id, dh_pub, N))` must never repeat.

**Critical nuance for the harness:** `mk` is itself derived from `(CK_ec, CK_pq)` at send time via `derive_mk_step` (lines 171–191). A fresh `mk` is produced for every `N` as long as the chain keys advance. The nonce being derived from the same `N` as the key derivation creates an *implicit* uniqueness guarantee — but `saturating_add` silently stalls at `u32::MAX` rather than rolling over or failing (line 836), which means the property can break if `ns` reaches `u32::MAX` without any separate guard. No such guard currently appears in `send_wire`.

The harness needs to exercise this precisely.

---

### **2\. Target functions and their public API**

**Primary targets** (both are the public wire-level entry points):

* **`send_wire`** — `tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs:768`  
   Signature: `fn send_wire(hash, kmac, aead, st: Suite2SendState, flags: u16, plaintext: &[u8]) -> Result<SendWireOutcome, &'static str>`  
   State type: `Suite2SendState` (lines 748–758) — holds `session_id`, `dh_pub`, `hk_s`, `ck_ec`, `ck_pq`, `ns`, `pn`.

* **`recv_wire`** — `tools/refimpl/quantumshield_refimpl/src/suite2/ratchet.rs:846`  
   Signature: `fn recv_wire(hash, kmac, aead, st: Suite2RecvWireState, wire, pq_epoch_ss, peer_adv_id) -> Result<RecvWireOutcome, &'static str>`  
   State type: `Suite2RecvWireState` (lines 719–737) — holds `session_id`, `dh_pub`, `hk_r`, `rk`, `ck_ec`, `ck_pq_send`, `ck_pq_recv`, `nr`, `mkskipped`, plus SCKA tracking sets.

Both are re-exported from `tools/refimpl/quantumshield_refimpl/src/suite2/mod.rs` (lines 26–48) as `send_wire_canon` / `recv_wire_canon`, which return `RefimplError` instead of `&'static str`.

**Supporting functions that emit the nonce values:**

* `nonce_hdr` / `nonce_body` — lines 145–167 — these produce the 12-byte nonces given `(session_id, dh_pub, N)`.  
* `derive_mk_step` — lines 171–191 — derives the per-message body key `mk` from `(ck_ec, ck_pq)`.

---

### **3\. Harness placement**

The three existing libFuzzer targets live in:

Code  
qsl/qsl-client/qsc/fuzz/fuzz\_targets/  
  qsc\_route\_http.rs  
  qsc\_payload\_boundaries.rs  
  qsc\_vault\_envelope.rs

Each is a `cargo-fuzz` binary wired via `qsl/qsl-client/qsc/fuzz/Cargo.toml`.

The new harness must target the **refimpl crate** (`quantumshield_refimpl`), not `qsc`. It therefore belongs in a **new** `fuzz/` directory under the refimpl crate:

Code  
tools/refimpl/quantumshield\_refimpl/fuzz/  
  Cargo.toml               \# cargo-fuzz manifest, depends on quantumshield\_refimpl  
  fuzz\_targets/  
    suite2\_nonce\_uniqueness.rs  
  corpus/  
    suite2\_nonce\_uniqueness/  
      seed001              \# minimal valid 2-message sequence (non-boundary)  
      seed002              \# minimal valid boundary sequence

The refimpl crate already exposes the needed types and functions as `pub` via `lib.rs`. The crate has `stdcrypto` as its default feature, so `StdCrypto` (from `src/crypto/stdcrypto.rs`) is available to the harness with no feature flags beyond `default`.

---

### **4\. Fuzzer input encoding**

The libFuzzer input must encode a **session configuration plus a variable-length sequence of send/recv operations**. The fuzzer controls:

1. The initial send-side state (fixed 32-byte seeds for `ck_ec`, `ck_pq`, `hk_s`, `dh_pub`, `session_id`; `ns = 0`).  
2. A byte-slice sequence of **operation descriptors** that the harness parses step-by-step.

Each operation descriptor is minimally:

* 1 byte: opcode (`0x00 = send`, `0x01 = recv_self` — receiver replays its own wire bytes)  
* Variable: plaintext length \+ bytes (for send)

The harness maintains:

* The current `Suite2SendState` (advanced after each successful `send_wire`)  
* The current `Suite2RecvWireState` (advanced after each successful `recv_wire`)  
* A `HashSet<([u8;32], [u8;12])>` — the global seen-pair tracker

For **send** operations, before calling `send_wire`, the harness derives and records:

* `(hk_s, nonce_hdr(session_id, dh_pub, ns))` — the header (key, nonce) pair  
* `(mk, nonce_body(session_id, dh_pub, ns))` — the body (key, nonce) pair

`mk` at this point equals what `derive_mk_step` will produce. The harness must reconstruct it from the pre-send `ck_ec`/`ck_pq` before the state advances. Both `nonce_hdr` and `nonce_body` are already `pub` so the harness can call them directly.

The assertion is:

Code  
assert\!(seen.insert((key, nonce)), "nonce reuse detected");

If any pair is already in `seen`, the fuzzer triggers a panic, which libFuzzer reports as a crash.

---

### **5\. Invariants to assert**

**I-1: No `(hk_s, nonce_hdr)` repetition across the send sequence.** `hk_s` is fixed for the lifetime of a `Suite2SendState` (line 753). `nonce_hdr` depends on `(session_id, dh_pub, ns)`. If `dh_pub` never changes (non-boundary sequence), `ns` must be strictly monotone. Reuse is possible if `ns` is not incremented (e.g., state roll-back or `saturating_add` stall at `u32::MAX`).

**I-2: No `(mk, nonce_body)` repetition across the send sequence.** `mk` changes every message because `ck_ec` advances via `KMAC32(ck_ec, "QSP5.0/CK", [0x01])` and `ck_pq` advances via `KMAC32(ck_pq, "QSP5.0/PQCK", [0x01])` (lines 179–183). Even if `mk` varies, the harness must also check that the pair `(mk, nonce)` is unique, since a malicious or buggy sender could produce the same `mk` on different `N` values if a KDF collapses.

**I-3: No `(hk_r, nonce_hdr)` repetition across the recv sequence.** Derived symmetrically. `hk_r` is the counterpart to `hk_s`; `nonce_hdr` is computed the same way from the sender's `dh_pub` and `N`. The receiver's `nr` must always advance past successfully-consumed message numbers (line 490: `new_state.nr = header_n.saturating_add(1)`).

**I-4: Skipped-key replay is detected.** When `recv_nonboundary_ooo` stores a skipped `MkSkippedEntry` (lines 459–463), it records `(dh_pub, n, mk)`. If the same entry is consumed twice, the second consume will not find it in `mkskipped` (it is removed at line 383). The harness does not need to assert this separately — it is covered by I-2 once `(mk, nonce)` pairs are globally tracked.

**I-5: `u32::MAX` saturation does not silently produce nonce reuse.** `ns` saturates at `u32::MAX` (line 836). After saturation, every subsequent `send_wire` call uses `ns = u32::MAX`, which produces the same `nonce_hdr` and the same `nonce_body`, and the same `hk_s` — so both I-1 and I-2 fire. The harness must be able to fast-forward `ns` to near-saturation using a seeded state rather than executing `u32::MAX` real iterations.

---

### **6\. Saturation fast-forward: the key complexity**

Fuzzing `u32::MAX` iterations is not feasible under libFuzzer's time budget. The harness needs to be able to accept a **synthetic initial state** from the fuzzer input where `ns` is already set to a large value (e.g., `u32::MAX - 2`). This is safe because:

* `Suite2SendState` is a plain struct with all fields `pub` (lines 748–758).  
* The harness can construct it directly from fuzzer-supplied bytes without going through the real handshake.  
* The domain-separation labels in the KDF include no sequence-number commitment, so a synthetic starting state is a valid test.

The fuzzer input encoding should include a 4-byte `ns_seed` field. The harness reads it and initializes `st.ns = ns_seed`, so libFuzzer can discover the `u32::MAX` boundary by mutation.

---

### **7\. Corpus seeds**

Seed entries must be valid byte sequences parsable by the harness's own input decoder. Two seeds are sufficient to bootstrap coverage:

**seed001** (non-boundary, 3 sends \+ 3 recv-self):

* `ns_seed = 0x00000000`  
* 3× send opcodes with 16-byte plaintexts  
* 3× recv opcodes replaying the 3 wire blobs produced

**seed002** (saturation boundary, 2 sends at ns \= u32::MAX \- 1):

* `ns_seed = 0xFFFFFFFE`  
* 2× send opcodes — second one should trigger the I-1/I-2 assertion because `ns` saturates to `u32::MAX` on both

Existing Suite-2 KDF vectors (`inputs/suite2/vectors/qshield_suite2_kdf_vectors_v1.json`) and MK-hybrid vectors (`inputs/suite2/vectors/qshield_suite2_mk_hybrid_vectors_v1.json`) can be cross-referenced to verify that the harness's inline key derivations match the reference values, providing an independent correctness check for the tracker.

---

### **8\. Infrastructure placement**

Following the pattern of the existing `qsc/fuzz/` setup:

| Artifact | Path |
| ----- | ----- |
| `cargo-fuzz` manifest | `tools/refimpl/quantumshield_refimpl/fuzz/Cargo.toml` |
| Harness source | `tools/refimpl/quantumshield_refimpl/fuzz/fuzz_targets/suite2_nonce_uniqueness.rs` |
| Corpus seeds | `tools/refimpl/quantumshield_refimpl/fuzz/corpus/suite2_nonce_uniqueness/` |
| CI script (qbuild-first) | `scripts/ci/refimpl_fuzz_nonce.sh` (mirrors `scripts/ci/qsc_adversarial.sh` structure) |
| CI workflow | `.github/workflows/refimpl-fuzz.yml` (mirrors `.github/workflows/qsc-adversarial.yml`) |

The existing `qsc-fuzz` manifest (`qsl/qsl-client/qsc/fuzz/Cargo.toml`) uses `libfuzzer-sys = "0.4"` and `cargo-fuzz = true` metadata. The new manifest must follow the same pattern, adding `quantumshield_refimpl` as a path dependency with `features = ["stdcrypto"]`.

Per DOC-G4-001 §3 (adversarial program rules): "keep qsl-server transport-only and qsl-attachments opaque ciphertext-only" — this harness stays within `tools/refimpl` and does not touch either sibling repo.

---

### **9\. Known limitations and residual gaps**

**L-1: Boundary messages not yet in scope for this harness.** `recv_boundary_in_order` (line 514\) is called when `flags & FLAG_BOUNDARY != 0`. A boundary changes `dh_pub` and rekeys the chain. The full nonce-uniqueness invariant across boundaries would require the harness to also drive a paired `Suite2BoundaryState` with a real SCKA sequence, including a valid ML-KEM-768 ciphertext. This is feasible but makes the harness significantly more complex. The recommendation is to scope the first iteration to non-boundary messages only, with a follow-on audit item for boundary sequences.

**L-2: `nonce_hdr` for OOO header recovery is not tracked.** `recv_nonboundary_ooo` (line 193\) tries up to `MAX_HEADER_ATTEMPTS = 100` candidate `n` values under the same `hk_r`. Each trial produces a distinct nonce, but none of them are committed unless they decrypt successfully. Only the successfully-consumed `(hk_r, nonce_hdr(dh_pub, header_n))` pair needs to appear in the uniqueness tracker.

**L-3: `evict_mkskipped` discards old entries (line 40–55).** If the fuzzer evicts a skipped key and then the harness receives the corresponding message, the receiver will treat it as a forward-skip rather than a skipped-key cache hit. The nonce uniqueness tracker only catches reuse during successful decryption, so this edge case is benign from the tracker's perspective — the key was never used for decryption after eviction.

**L-4: `saturating_add` at `u32::MAX` for `nr` (receiver side).** `new_state.nr = header_n.saturating_add(1)` (line 490\) — same saturation concern on the receive side. A receiver that accepts message `u32::MAX` and then accepts another message at the same `N` would reuse the same `(hk_r, nonce_hdr)` pair. The harness's seed002 and the ns\_seed fast-forward mechanism should cover both sides.

---

### **10\. Primary source map (full reference)**

| Symbol / location | Role |
| ----- | ----- |
| `ratchet.rs:768` `send_wire` | Primary fuzz target (send path) |
| `ratchet.rs:846` `recv_wire` | Primary fuzz target (recv path) |
| `ratchet.rs:145` `nonce_hdr` | Nonce derivation (header) |
| `ratchet.rs:157` `nonce_body` | Nonce derivation (body) |
| `ratchet.rs:171` `derive_mk_step` | Body key derivation |
| `ratchet.rs:748` `Suite2SendState` | Fuzz-controllable send state |
| `ratchet.rs:719` `Suite2RecvWireState` | Fuzz-controllable recv state |
| `ratchet.rs:836` `ns.saturating_add(1)` | Overflow/saturation behavior |
| `ratchet.rs:490` `nr.saturating_add(1)` | Recv-side saturation |
| `ratchet.rs:12` `MAX_MKSKIPPED = 1000` | Eviction cap |
| `ratchet.rs:11` `MAX_SKIP = 1000` | OOO window |
| `suite2/mod.rs:26` `send_wire_canon` | Public re-export |
| `suite2/mod.rs:37` `recv_wire_canon` | Public re-export |
| `binding.rs:7` `pq_bind_sha512_32` | AD binding (needed for correct `aead.seal` calls in harness) |
| `binding.rs:18` `ad_hdr` | AD for header seal |
| `binding.rs:36` `ad_body` | AD for body seal |
| `crypto/traits.rs:23` `Aead` trait | Harness can inject an instrumented impl |
| `crypto/stdcrypto.rs:15` `StdCrypto` | Real AES-256-GCM impl (default feature) |
| `suite2/state.rs:25` `Suite2SessionState` | Snapshot/restore (useful for corpus creation) |
| `DOC-CAN-003 §5.2` | Normative nonce formula |
| `DOC-G4-001 §3` | Adversarial program placement rules |
| `qsl/qsl-client/qsc/fuzz/Cargo.toml` | Existing `cargo-fuzz` manifest to mirror |
| `qsl/qsl-client/qsc/fuzz/fuzz_targets/qsc_vault_envelope.rs` | Existing harness structure to follow |
| `inputs/suite2/vectors/qshield_suite2_mk_hybrid_vectors_v1.json` | Reference values for harness correctness check |
| `inputs/suite2/vectors/qshield_suite2_kdf_vectors_v1.json` | Reference KDF values |
| `qsl/qsl-client/qsc/tests/ratchet_durability_na0155.rs:147` | Existing `nonce_reuse` regression test (qsc layer, not refimpl) |

