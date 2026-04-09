## **Audit \#6 — Parser Fuzzing**

### **Scope statement**

**Goal:** Design `cargo-fuzz` harnesses for five decode functions in the refimpl crate — `Envelope::decode`, `decode_suite2_wire`, `HandshakeInit::decode`, `HandshakeResp::decode`, and `PrekeyBundle::decode` — with special focus on the `suite2_wire_prefix_len` bucket-mode arithmetic. **No code changes.**

---

### **1\. Target function inventory**

All five decoder targets live in the `quantumshield_refimpl` crate at `tools/refimpl/quantumshield_refimpl/`.

| Target | File | Line | Public access path |
| ----- | ----- | ----- | ----- |
| `Envelope::decode` | `src/qse/envelope.rs:57` | L57 | `quantumshield_refimpl::Envelope::decode` |
| `suite2_wire_prefix_len` | `src/qse/envelope.rs:129` | L129 | Called internally by `Envelope::decode` in bucket mode (L79); not separately exported |
| `decode_suite2_wire` | `src/suite2/parse.rs:129` | L129 | `quantumshield_refimpl::suite2::decode_suite2_wire_canon` (L20 of `suite2/mod.rs`) |
| `HandshakeInit::decode` | `src/qsp/types.rs:175` | L175 | `quantumshield_refimpl::qsp::decode_handshake_init_canon` (`qsp/mod.rs:22`) |
| `HandshakeResp::decode` | `src/qsp/types.rs:274` | L274 | `quantumshield_refimpl::qsp::decode_handshake_resp_canon` (`qsp/mod.rs:26`) |
| `PrekeyBundle::decode` | `src/qsp/types.rs:61` | L61 | `quantumshield_refimpl::qsp::decode_prekey_bundle_canon` (`qsp/mod.rs:18`) |

Note: `HandshakeInit`, `HandshakeResp`, and `ProtocolMessage` (the QSP-layer equivalent of `decode_suite2_wire`) are exported from `src/lib.rs:18` via `pub use qsp::{HandshakeInit, HandshakeResp, ...}`. `PrekeyBundle` is not directly re-exported at the crate root but is accessible through `qsp::PrekeyBundle`.

---

### **2\. Harness placement**

The existing libFuzzer infrastructure lives at `qsl/qsl-client/qsc/fuzz/` targeting the `qsc` crate. The refimpl-targeting harnesses belong in a separate `cargo-fuzz` workspace:

Code  
tools/refimpl/quantumshield\_refimpl/fuzz/  
  Cargo.toml                     \# \[package.metadata\] cargo-fuzz \= true; \[\[bin\]\] entries  
  fuzz\_targets/  
    refimpl\_envelope.rs           \# Envelope::decode (both modes)  
    refimpl\_suite2\_wire.rs        \# decode\_suite2\_wire / suite2\_wire\_prefix\_len  
    refimpl\_handshake\_init.rs     \# HandshakeInit::decode  
    refimpl\_handshake\_resp.rs     \# HandshakeResp::decode  
    refimpl\_prekey\_bundle.rs      \# PrekeyBundle::decode  
  corpus/  
    refimpl\_envelope/             \# seed001–seed004 (see §5 below)  
    refimpl\_suite2\_wire/          \# seed001–seed003  
    refimpl\_handshake\_init/       \# seed001–seed002  
    refimpl\_handshake\_resp/       \# seed001  
    refimpl\_prekey\_bundle/        \# seed001

The `Cargo.toml` mirrors `qsl/qsl-client/qsc/fuzz/Cargo.toml` (which uses `libfuzzer-sys = "0.4"`, `cargo-fuzz = true` metadata, and `path = ".."` dependency), substituting `quantumshield_refimpl = { path = "..", features = ["stdcrypto"] }`.

---

### **3\. Detailed anatomy of each target**

#### **3.1 `Envelope::decode` — `src/qse/envelope.rs:57`**

The function has two structurally distinct branches controlled by `FLAG_BUCKET_PADDED = 0x0001` (L4):

**Non-bucket branch (L89–L104):**

* Reads `pad_len` as `u16 → usize` (L70) and `payload_len` as `u32 → usize` (L71).  
* Guards allocation at L90: `r.remaining() < payload_len + pad_len`. This addition is *not* checked for overflow. On 64-bit targets `pad_len ≤ 65535` and `payload_len ≤ 4294967295`, so `payload_len + pad_len ≤ 4,295,032,830` — within `usize` on 64-bit. On 32-bit it could overflow, but the `remaining()` bound in practice caps both to actual input size, preventing OOM amplification.  
* Calls `r.finish()` (L95) to reject trailing bytes. This is the only branch with trailing-byte enforcement.  
* The `read_varbytes_u16` at L68 (route token) allocates up to 65535 bytes, bounded by actual remaining input.

**Bucket branch (L73–L88):**

* Enforces `pad_len == 0 && payload_len == 0` as sentinels at L75–77 (rejects `bucket_len_fields`).  
* Slurps all remaining bytes at L78 (`r.read_bytes(r.remaining())`), then passes them to `suite2_wire_prefix_len` (L79).  
* If `suite2_wire_prefix_len` succeeds, calls `remaining.split_at(payload_len)` (L80).  
* **Does NOT call `r.finish()`** — correct for bucket mode, as all remaining bytes are consumed.  
* **Does NOT validate** that the embedded payload is actually a Suite-2 wire message (no `protocol_version` / `suite_id` check). `suite2_wire_prefix_len` only reads bytes at offsets 6–9 for `header_len` / `body_len`.

**Invariants for the harness:**

* I-Env-1: `decode` never panics on any input.  
* I-Env-2: If `flags & !FLAG_BUCKET_PADDED != 0` the function must return `Err(Invalid("flags"))`.  
* I-Env-3: If `env_version != QSE_ENV_VERSION_V1 (0x0100)` the function must return `Err(Invalid("env_version"))`.  
* I-Env-4: In bucket mode with non-zero `pad_len` or `payload_len`, the function must return `Err(Invalid("bucket_len_fields"))`.  
* I-Env-5: All allocations are bounded by input size (no amplification).

#### **3.2 `suite2_wire_prefix_len` — `src/qse/envelope.rs:129`**

This is the **primary target** for the bucket-mode arithmetic audit.

Code  
fn suite2\_wire\_prefix\_len(buf: &\[u8\]) \-\> Result\<usize, CodecError\> {  
    if buf.len() \< 10 { return Err(Truncated); }  
    let header\_len \= u16::from\_be\_bytes(\[buf\[6\], buf\[7\]\]) as usize;  
    let body\_len   \= u16::from\_be\_bytes(\[buf\[8\], buf\[9\]\]) as usize;  
    let total \= 10usize  
        .checked\_add(header\_len)  
        .and\_then(|n| n.checked\_add(body\_len))  
        .ok\_or(CodecError::LengthOutOfRange)?;  
    if total \> buf.len() { return Err(LengthOutOfRange); }  
    Ok(total)  
}

**Arithmetic analysis:**

* `header_len` and `body_len` are each at most `u16::MAX = 65535`.  
* `10 + 65535 + 65535 = 131080` — this can never overflow `usize` on any supported platform (16-bit platforms aside). The `checked_add` calls are therefore defensive rather than strictly necessary, but their presence is correct.  
* Maximum `total` returned: 131,080 bytes — this is the safe upper bound on the payload that will be split from the remaining bytes in bucket mode.

**Validation gap:** `suite2_wire_prefix_len` does not inspect bytes 0–5 of `buf`. In particular:

* `buf[0..2]` (would be `protocol_version` in a Suite-2 wire message) — ignored.  
* `buf[2..4]` (would be `suite_id`) — ignored.  
* `buf[4]` (would be `msg_type`) — ignored.  
* `buf[5]` (would be `_env_flags`) — ignored.

This means that in bucket mode, `Envelope::decode` will successfully "parse" an embedded payload whose first 4 bytes are any values, as long as bytes 6–9 produce a `total ≤ remaining.len()`. The payload consumer downstream (e.g., `recv_wire_canon` at `qsl/qsl-client/qsc/src/main.rs:2271`) will then fail on protocol/suite mismatch rather than at the QSE layer. The audit property for the fuzzer: **any input for which `suite2_wire_prefix_len` returns `Ok(n)` must satisfy `n ≤ buf.len()`** — this holds by construction, but the fuzzer should confirm `split_at(n)` never panics.

**Bucket-mode edge cases the fuzzer must explore:**

* `buf.len() == 10` exactly: `header_len = 0, body_len = 0` → `total = 10`, `remaining.split_at(10)` on a 10-byte slice.  
* `header_len = 0xFFFF, body_len = 0xFFFF` with `buf.len() ≥ 131080` → large but safe.  
* `header_len + body_len + 10 > buf.len()` → must return `LengthOutOfRange`.  
* `buf.len() = 9` → must return `Truncated`.  
* Non-Suite-2 bytes at positions 0–5 (arbitrary protocol\_version, suite\_id, msg\_type).

#### **3.3 `decode_suite2_wire` — `src/suite2/parse.rs:129`**

This function additionally validates:

* `protocol_version == SUITE2_PROTOCOL_VERSION (0x0500)` — L150.  
* `suite_id == SUITE2_SUITE_ID (0x0002)` — L150.  
* `msg_type == 0x02` — L152.  
* `buf.len() == off + header_len + body_len` (exact length, no trailing bytes) — L163.

The inner `parse_ratchet_header` (`src/suite2/parse.rs:17`) uses manual offset tracking with explicit bounds checks before each access (L22, L52, L71, L93). The harness must confirm these guards prevent panics under all inputs.

**Specific boundary conditions:**

| Condition | Expected reject code |
| ----- | ----- |
| `buf.len() < 10` | `REJECT_S2_PARSE_PREFIX` |
| `protocol_version != 0x0500` or `suite_id != 0x0002` or `msg_type != 0x02` | `REJECT_S2_PARSE_PREFIX` |
| `buf.len() != off + header_len + body_len` (trailing bytes) | `REJECT_S2_PARSE_PREFIX` |
| `header.len() < 34` (\< 32 dh\_pub \+ 2 flags) | `REJECT_S2_PARSE_PREFIX` |
| Unknown flag bits in `flags` | `REJECT_S2_PARSE_FLAGS` |
| `FLAG_PQ_ADV` or `FLAG_PQ_CTXT` set without `FLAG_BOUNDARY` | `REJECT_S2_PARSE_FLAGS` |
| `FLAG_PQ_ADV` set but header too short for `4 + 1184` bytes | `REJECT_S2_PQPREFIX_PARSE` |
| `FLAG_PQ_CTXT` set but header too short for `4 + 1088` bytes | `REJECT_S2_PQPREFIX_PARSE` |
| `used != header.len()` after parsing PQ prefix \+ hdr\_ct | `REJECT_S2_PARSE_HDR_LEN` |
| `body.len() < 16` | `REJECT_S2_PARSE_BODY_LEN` |

**Interaction with `suite2_wire_prefix_len`:** The `decode_suite2_wire` parses `header_len` and `body_len` from bytes 6–7 and 8–9 as `u16 → usize` at L145–148 and checks `buf.len() < off + header_len + body_len` (L157). This is the *non-`checked_add`* version of the same arithmetic done in `suite2_wire_prefix_len`, but on 64-bit systems the inputs are bounded by `u16` so no overflow is possible. **The fuzzer tests the consistency between these two parallel arithmetic paths**: `suite2_wire_prefix_len` (used in the QSE envelope layer) and the inline arithmetic in `decode_suite2_wire` (used in the Suite-2 ratchet layer). For the same byte sequence, they must agree on the payload boundary.

#### **3.4 `HandshakeInit::decode` — `src/qsp/types.rs:175`**

Structure:

Code  
protocol\_version (u16) | suite\_id (u16) | session\_id \[16\] | user\_id\_b (varbytes\_u16) |  
device\_id\_b (u32) | ek\_dh\_a\_pub \[32\] | ct1 \[1088\] | opk\_used (u16) |  
\[if opk\_used: ct2 \[1088\] | opk\_dh\_id (u32) | opk\_pq\_id (u32)\] |  
pq\_rcv\_a\_id (u32) | pq\_rcv\_a\_pub \[1184\] | ik\_sig\_ec\_a\_pub \[32\] | ik\_sig\_pq\_a\_pub \[1952\] |  
sig\_ec\_a \[64\] | sig\_pq\_a \[3309\] | (finish — no trailing bytes)

* **Minimum valid size (no OPK):** 7,693 bytes (computed above).  
* **Maximum valid size (with OPK):** 8,789 bytes.  
* `opk_used` is a `u16` boolean: `0 = false, non-zero = true` (L190). The fuzzer should probe values like `0x0001`, `0x00FF`, `0xFFFF`, and `0x0002` — all non-zero values branch into the OPK path.  
* `user_id_b` is a `varbytes_u16` (L186): maximum allocation is 65,535 bytes, bounded by `r.remaining()`.  
* `protocol_version` must equal `QSP_PROTOCOL_VERSION = 0x0403` (L179) and `suite_id` must equal `QSP_SUITE_ID = 0x0001` (L182); other values return `Invalid("protocol_version")` / `Invalid("suite_id")`.  
* `r.finish()` at L205 enforces no trailing bytes.  
* **Structural conditional:** the `opk_used` branch creates two parse graphs with different total sizes. The fuzzer must discover both paths. The key invariant: **`opk_used = true` with truncated bytes must produce `Truncated` not a panic**.

#### **3.5 `HandshakeResp::decode` — `src/qsp/types.rs:274`**

Structure: fixed-size (no variable-length fields except KT proofs are absent here):

Code  
protocol\_version (u16) | suite\_id (u16) | session\_id \[16\] | dh0\_b\_pub \[32\] |  
pq\_rcv\_b\_id (u32) | pq\_rcv\_b\_pub \[1184\] | ct3 \[1088\] | conf\_b \[32\] |  
ik\_sig\_ec\_b\_pub \[32\] | ik\_sig\_pq\_b\_pub \[1952\] | sig\_ec\_b \[64\] | sig\_pq\_b \[3309\]

* **Minimum (and only) valid size:** 7,717 bytes.  
* No conditional branches. Simpler parse graph; the fuzzer coverage is essentially linear.  
* Same `protocol_version` / `suite_id` gating as HandshakeInit.  
* `r.finish()` at L294 enforces no trailing bytes.

#### **3.6 `PrekeyBundle::decode` — `src/qsp/types.rs:61`**

The most complex decoder. Fixed large fields plus three unbounded `varbytes_u16` (KT proofs at L95–97) and two optional sections (`opk_dh`, `opk_pq`) controlled by presence flags at L74, L83.

* **Minimum valid size (no OPK, 0-length varbytes):** 7,817 bytes.  
* **With OPK (both present):** adds `4 + 32` (DH OPK) \+ `4 + 1184` (PQ OPK) \= 1,224 bytes → \~9,041 minimum with OPK.  
* `kt_sth`, `kt_inclusion_proof`, `kt_consistency_proof` are `varbytes_u16` — each can claim up to 65,535 bytes, bounded by input.  
* `opk_dh_present` and `opk_pq_present` are `u16` booleans (L74, L83) — same `non-zero = true` semantics.  
* `r.finish()` at L98 enforces no trailing bytes.  
* **Allocation note:** three `varbytes_u16` KT proof fields simultaneously could claim up to 3 × 65,535 \= 196,605 bytes in excess of the fixed minimum — bounded by input size.

---

### **4\. Harness structure for each target**

Each harness follows the same pattern as the existing `qsc/fuzz/fuzz_targets/qsc_vault_envelope.rs`:

Rust  
\#\!\[no\_main\]  
use libfuzzer\_sys::fuzz\_target;  
use quantumshield\_refimpl::...; // target import

fuzz\_target\!(|data: &\[u8\]| {  
    let \_ \= TargetFunction::decode(data);  
    // Additional structural checks (see per-target notes below)  
});

The `_` discard is intentional: libFuzzer catches panics and memory safety violations (via AddressSanitizer when run with `cargo fuzz run --sanitizer address`). An `Err` result is an accepted outcome; a panic is a crash report.

**Additional assertions per harness:**

* **`refimpl_envelope.rs`:** After `Ok(env)`, assert `env.payload` and `env.padding` together consume the correct number of bytes relative to the encoded length (in bucket mode). This catches a `split_at` arithmetic defect.

* **`refimpl_suite2_wire.rs`:** After `Ok(parsed)`, assert `parsed.hdr_ct.len() == 24` and `parsed.body_ct.len() >= 16`. These are the fixed size invariants from `parse_ratchet_header` at L18 (`HDR_CT_LEN = 24`) and `decode_suite2_wire` at L172 (`body.len() < 16`). The harness should also call `suite2_wire_prefix_len` on the same input directly (via an accessible wrapper or by duplicating the 10-byte prefix logic) and verify the returned length matches what `decode_suite2_wire` consumes — testing the two parallel arithmetic paths for consistency.

* **`refimpl_handshake_init.rs`:** After `Ok(msg)`, assert `msg.ct2.is_some() == msg.opk_dh_id.is_some()` and `msg.ct2.is_some() == msg.opk_pq_id.is_some()`. These must hold because the decode path sets all three together (L192–196) or none (L197–199). A decode bug that populates some but not all would violate these.

* **`refimpl_prekey_bundle.rs`:** After `Ok(bundle)`, assert `bundle.opk_dh.is_some() == (opk_dh_present != 0)` (reconstructed from the bundle) — the decode must not populate `opk_dh` when `opk_dh_present == 0`.

---

### **5\. Corpus seeds**

All seeds should be binary files (not JSON), matching the existing `qsc/fuzz/corpus/` style.

**`refimpl_suite2_wire/seed001`** — Minimal valid non-boundary wire message:

* 84-byte blob: `050000020200003a0010` \+ `[0x22]*32` (dh\_pub) \+ `[0x00, 0x00]` (flags \= 0\) \+ `[0x33]*24` (hdr\_ct) \+ `[0x55]*16` (body\_ct)  
* Source: `inputs/suite2/vectors/qshield_suite2_parse_vectors_v1.json` vector `S2-PARSE-ACCEPT-NONBOUNDARY-0001` (`data` hex)

**`refimpl_suite2_wire/seed002`** — Minimal reject: bad protocol\_version:

* Same as seed001 but bytes 0–1 changed from `0x0500` to `0x0403`. Must return `REJECT_S2_PARSE_PREFIX`.

**`refimpl_suite2_wire/seed003`** — Wire with `FLAG_BOUNDARY | FLAG_PQ_CTXT` set (SCKA prefix present):

* `header_len` in bytes 6–7 encodes `32 + 2 + 4 + 1088 + 24 = 1150` (`0x047E`), `body_len = 16`.  
* Source: `inputs/suite2/vectors/qshield_suite2_parse_vectors_v1.json` vector `S2-PARSE-REJECT-PQPREFIX-CTLEN-0001` (exercise the PQ-prefix path).

**`refimpl_envelope/seed001`** — Non-bucket QSE envelope containing Suite2 wire:

* `env_version=0x0100, flags=0x0000, route_token="test-token" (10 bytes), timestamp_bucket, pad_len=0, payload_len=84, payload=seed001_suite2_wire`

**`refimpl_envelope/seed002`** — Bucket-mode QSE envelope at Standard profile (1024 bytes):

* `env_version=0x0100, flags=0x0001, route_token, timestamp_bucket, pad_len=0 (sentinel), payload_len=0 (sentinel), [suite2_wire + random_padding to 1024]`  
* Source: `tools/refimpl/quantumshield_refimpl/tests/qse_bucket_confidentiality.rs` `make_bucketed` helper shows the exact encoding.

**`refimpl_envelope/seed003`** — Bucket-mode envelope with non-Suite2 bytes at positions 0–5 of payload (exercises `suite2_wire_prefix_len`'s non-validation of those bytes).

**`refimpl_envelope/seed004`** — Non-bucket with large (but valid) `payload_len` field and exactly that many bytes of payload (exercises the `r.remaining() < payload_len + pad_len` guard at L90).

**`refimpl_handshake_init/seed001`** — Minimum valid HandshakeInit (no OPK): `protocol_version=0x0403, suite_id=0x0001` \+ all required fixed fields filled with deterministic bytes; 7,693 bytes.

**`refimpl_handshake_init/seed002`** — HandshakeInit with OPK (8,789 bytes).

**`refimpl_handshake_resp/seed001`** — Minimum valid HandshakeResp: 7,717 bytes with all fixed fields.

**`refimpl_prekey_bundle/seed001`** — Minimum valid PrekeyBundle (no OPK, 0-length KT proofs): 7,817 bytes.

The existing `inputs/suite2/vectors/qshield_suite2_parse_vectors_v1.json` contains 6 parse vectors (`S2-PARSE-ACCEPT-NONBOUNDARY-0001` through `S2-PARSE-REJECT-PQPREFIX-CTLEN-0001`); their `input.msg.data` hex fields are directly usable as corpus seeds for `refimpl_suite2_wire`.

---

### **6\. The `suite2_wire_prefix_len` arithmetic: specific test cases**

The following input patterns are the highest-value cases for the bucket arithmetic:

| Input bytes (key fields) | `header_len` | `body_len` | `total` | `buf.len()` | Expected result |
| ----- | ----- | ----- | ----- | ----- | ----- |
| `[...] [00 00] [00 00]` at offsets 6–9 | 0 | 0 | 10 | ≥10 | `Ok(10)` |
| `[...] [FF FF] [00 00]` at offsets 6–9 | 65535 | 0 | 65545 | ≥65545 | `Ok(65545)` |
| `[...] [FF FF] [FF FF]` at offsets 6–9 | 65535 | 65535 | 131080 | ≥131080 | `Ok(131080)` |
| `[...] [FF FF] [FF FF]` at offsets 6–9 | 65535 | 65535 | 131080 | 131079 | `Err(LengthOutOfRange)` |
| `[...] [00 00] [00 00]` at offsets 6–9 | 0 | 0 | 10 | 9 | `Err(Truncated)` |
| Only 9 bytes total | — | — | — | 9 | `Err(Truncated)` |

The `checked_add` calls (L136–138) handle the arithmetic; the secondary guard at L139 (`total > buf.len()`) enforces that the claimed total does not exceed the actual available bytes. The `split_at(payload_len)` at `envelope.rs:80` will **never panic** because `suite2_wire_prefix_len` has already guaranteed `payload_len ≤ remaining.len()`.

**One subtle structural question for the fuzzer:** The `suite2_wire_prefix_len` function treats `buf[6..7]` as the Suite-2 `header_len` field and `buf[8..9]` as `body_len`. In the Suite-2 wire format (DOC-CAN-003 §4.1), these fields are at offsets 6–7 and 8–9 of the QSP wire envelope:

Code  
\[0..2\]  protocol\_version  u16  
\[2..4\]  suite\_id          u16  
\[4\]     msg\_type          u8  
\[5\]     flags             u8  
\[6..8\]  header\_len        u16   ← used by suite2\_wire\_prefix\_len  
\[8..10\] body\_len          u16   ← used by suite2\_wire\_prefix\_len

This matches the `decode_suite2_wire` parsing at `parse.rs:L145–148`. The arithmetic is **consistent** between the two functions. The fuzzer confirms this consistency property.

---

### **7\. Known gaps and residual questions**

**G-1: `suite2_wire_prefix_len` does not validate `protocol_version`, `suite_id`, or `msg_type`.** This is intentional design (QSE layer does not re-validate Suite-2 specifics; that's the caller's responsibility). However, the fuzzer should confirm that the mismatch is caught at the next layer (`decode_suite2_wire` or `recv_wire_canon`) and not silently accepted. The harness for `refimpl_envelope.rs` should optionally chain into `decode_suite2_wire_canon` on the extracted payload and assert that the combined decode is fail-closed: if `Envelope::decode` returns `Ok(env)` but `decode_suite2_wire_canon(&env.payload)` returns `Err`, the combination still terminates without panic or state mutation.

**G-2: `HandshakeInit::encode` with `opk_used=true` but missing OPK fields returns an empty `Vec<u8>` (line 160 of `types.rs`).** This is a silent failure in the encoder, not the decoder. The decoder (`decode`) would then receive an empty buffer and return `Truncated`. No panic. The fuzzer implicitly covers this via empty-input seeds.

**G-3: No decode target requires `read_varbytes_u32`.** The codec's `read_varbytes_u32` (which could claim up to 4 GiB if not guarded by `remaining()`) is present in `codec/mod.rs:70` but not used by any of these five targets. Only `read_varbytes_u16` is used. The `read_varbytes_u32` path remains untested by this harness set.

**G-4: `ProtocolMessage::decode` in `types.rs:379` is the QSP-layer analog of `decode_suite2_wire`.** It is not listed in the audit scope but shares the same `hdr_ct_len` exact-size check (`QSP_HDR_CT_LEN = 24` at L415) and `body_ct_len` minimum check (`QSP_BODY_CT_MIN = 16` at L424). A combined harness that calls both `decode_suite2_wire` and `ProtocolMessage::decode` on the same input would test whether both parsers agree on valid inputs.

**G-5: `PrekeyBundle::decode` does not validate signatures or KT proofs.** The decode is a pure structural parse. Signature verification happens in `handshake.rs:initiator_build` via `deps.kt.verify_bundle` and `deps.ed25519`. The parse-layer fuzzer exercises only structural correctness, not cryptographic integrity.

---

### **8\. Infrastructure placement summary**

| Artifact | Path |
| ----- | ----- |
| `cargo-fuzz` manifest | `tools/refimpl/quantumshield_refimpl/fuzz/Cargo.toml` |
| Envelope harness | `tools/refimpl/quantumshield_refimpl/fuzz/fuzz_targets/refimpl_envelope.rs` |
| Suite2 wire harness | `tools/refimpl/quantumshield_refimpl/fuzz/fuzz_targets/refimpl_suite2_wire.rs` |
| HandshakeInit harness | `tools/refimpl/quantumshield_refimpl/fuzz/fuzz_targets/refimpl_handshake_init.rs` |
| HandshakeResp harness | `tools/refimpl/quantumshield_refimpl/fuzz/fuzz_targets/refimpl_handshake_resp.rs` |
| PrekeyBundle harness | `tools/refimpl/quantumshield_refimpl/fuzz/fuzz_targets/refimpl_prekey_bundle.rs` |
| Corpus seeds | `tools/refimpl/quantumshield_refimpl/fuzz/corpus/<target>/` |
| CI script | `scripts/ci/refimpl_parse_fuzz.sh` (mirrors `scripts/ci/qsc_adversarial.sh`) |
| CI workflow | `.github/workflows/refimpl-fuzz.yml` (mirrors `.github/workflows/qsc-adversarial.yml`) |
| Primary seed source | `tools/refimpl/quantumshield_refimpl/vectors/parse_only.json` (QSP/QSE parse fixtures) |
| Secondary seed source | `inputs/suite2/vectors/qshield_suite2_parse_vectors_v1.json` (6 Suite-2 parse vectors) |

Per DOC-G4-001 §3: "keep qsl-server transport-only and qsl-attachments opaque ciphertext-only" — this harness set is entirely within `tools/refimpl/quantumshield_refimpl` and does not touch either sibling repo. Execution is qbuild-first; CI promotion follows after signal/noise is proven.

---

### **9\. Primary source reference map**

| Symbol / location | Role |
| ----- | ----- |
| `src/qse/envelope.rs:57` `Envelope::decode` | Target \#1 |
| `src/qse/envelope.rs:129` `suite2_wire_prefix_len` | **Primary arithmetic target** |
| `src/qse/envelope.rs:4` `FLAG_BUCKET_PADDED = 0x0001` | Flag driving the two parse branches |
| `src/qse/envelope.rs:73–88` Bucket branch | `suite2_wire_prefix_len` invocation and `split_at` |
| `src/qse/envelope.rs:89–104` Non-bucket branch | `r.finish()` trailing-bytes enforcement |
| `src/suite2/parse.rs:129` `decode_suite2_wire` | Target \#2 |
| `src/suite2/parse.rs:17` `parse_ratchet_header` | Inner parser; all index accesses guarded |
| `src/suite2/parse.rs:34–43` Flag validation | Unknown flags / PQ flags without BOUNDARY → reject |
| `src/qsp/types.rs:61` `PrekeyBundle::decode` | Target \#3 |
| `src/qsp/types.rs:175` `HandshakeInit::decode` | Target \#4 |
| `src/qsp/types.rs:274` `HandshakeResp::decode` | Target \#5 |
| `src/qsp/types.rs:190` `opk_used = r.read_u16()? != 0` | Conditional branch in HandshakeInit |
| `src/codec/mod.rs:62` `read_varbytes_u16` | Guarded allocation (bounded by remaining) |
| `src/codec/mod.rs:70` `read_varbytes_u32` | Unused by these targets but audited for completeness |
| `src/qsp/constants.rs:3` `QSP_PROTOCOL_VERSION = 0x0403` | Version gates in HandshakeInit/Resp |
| `src/qsp/constants.rs:4` `QSP_SUITE_ID = 0x0001` | Suite gate |
| `src/qsp/constants.rs:12` `SZ_MLKEM768_PUB = 1184` | Fixed-size PQ public key field |
| `src/qsp/constants.rs:13` `SZ_MLKEM768_CT = 1088` | Fixed-size PQ ciphertext field |
| `src/qsp/constants.rs:14` `SZ_MLDSA65_PUB = 1952` | Fixed-size PQ signature pub |
| `src/qsp/constants.rs:15` `SZ_MLDSA65_SIG = 3309` | Fixed-size PQ signature |
| `src/qsp/mod.rs:18` `decode_prekey_bundle_canon` | Public canon wrapper |
| `src/qsp/mod.rs:22` `decode_handshake_init_canon` | Public canon wrapper |
| `src/qsp/mod.rs:26` `decode_handshake_resp_canon` | Public canon wrapper |
| `src/suite2/mod.rs:20` `decode_suite2_wire_canon` | Public canon wrapper |
| `src/lib.rs:17–18` | Crate root exports |
| `tools/refimpl/quantumshield_refimpl/vectors/parse_only.json` | QSP/QSE parse fixtures (8 envelope, 8 messaging) |
| `inputs/suite2/vectors/qshield_suite2_parse_vectors_v1.json` | Suite-2 parse vectors (6 cases) |
| `tests/qse_bucket_confidentiality.rs:3` `make_suite2_wire` | Reference for valid Suite-2 wire construction |
| `tests/qse_bucket_confidentiality.rs:31` `qse_header_prefix_len` | Reference for QSE header offset arithmetic |
| `qsl/qsl-client/qsc/fuzz/Cargo.toml` | Existing `cargo-fuzz` manifest to mirror |
| `qsl/qsl-client/qsc/fuzz/fuzz_targets/qsc_vault_envelope.rs` | Existing harness pattern to follow |
| `DOC-CAN-003 §4.1` | Suite-2 wire format specification |
| `DOC-G4-001 §3` | Adversarial program placement rules |

