## **Audit \#7 — Vault File Format Adversarial Testing**

### **Scope statement**

**Goal:** Design a fuzzing harness for `parse_envelope`/`parse_vault_envelope` with attacker-controlled KDF parameters, and determine whether the "validation floor added in M-1's fix" correctly rejects all below-threshold configurations. **No code changes.**

---

### **1\. Target function anatomy**

The vault file format parsing has two layers.

#### **1.1 Public adversarial entrypoint — `parse_vault_envelope`**

**File:** `qsl/qsl-client/qsc/src/adversarial/vault_format.rs:13`

This is the function already exposed for fuzzing via `qsl/qsl-client/qsc/fuzz/fuzz_targets/qsc_vault_envelope.rs`. It performs a pure structural parse of vault file bytes and returns a `VaultEnvelopeView`:

Code  
QSCV01       \[0..6\]   magic (6 bytes)  
key\_source   \[6\]      u8  
salt\_len     \[7\]      u8 (must be \== 16\)  
nonce\_len    \[8\]      u8 (must be \== 12\)  
kdf\_m\_kib    \[9..13\]  u32 LE  
kdf\_t        \[13..17\] u32 LE  
kdf\_p        \[17..21\] u32 LE  
ct\_len       \[21..25\] u32 LE  
salt         \[25..41\] 16 bytes  
nonce        \[41..53\] 12 bytes  
ciphertext   \[53..\]   ct\_len bytes

Minimum valid size: `6 + 1 + 1 + 1 + (4 * 4) = 25` bytes (before the salt/nonce/ciphertext body). Guards present:

* L14–16: reject if `bytes.len() < 25`  
* L18–20: reject if magic ≠ `QSCV01`  
* L24–26: reject if `salt_len ≠ 16` or `nonce_len ≠ 12`  
* L38–40: reject if `bytes.len() < off + salt_len + nonce_len + ct_len` (i.e., if body is truncated)

**No KDF parameter validation is performed here.** The fields `kdf_m_kib`, `kdf_t`, and `kdf_p` are accepted as arbitrary `u32` values and placed directly into `VaultEnvelopeView`.

#### **1.2 Internal entrypoint — `parse_envelope`**

**File:** `qsl/qsl-client/qsc/src/vault/mod.rs:697`

This is a thin wrapper that calls `parse_vault_envelope` and maps the result to `VaultRuntimeEnvelope`:

Rust  
fn parse\_envelope(bytes: &\[u8\]) \-\> Result\<VaultRuntimeEnvelope, &'static str\> {  
    let parsed \= crate::adversarial::vault\_format::parse\_vault\_envelope(bytes)?;  
    Ok(VaultRuntimeEnvelope {  
        key\_source: parsed.key\_source,  
        salt: parsed.salt,  
        kdf\_m\_kib: parsed.kdf\_m\_kib,   // raw u32, no validation  
        kdf\_t: parsed.kdf\_t,             // raw u32, no validation  
        kdf\_p: parsed.kdf\_p,             // raw u32, no validation  
        ciphertext: parsed.ciphertext,  
    })  
}

Again, no KDF parameter validation.

#### **1.3 KDF parameter consumption — `derive_runtime_key`**

**File:** `qsl/qsl-client/qsc/src/vault/mod.rs:709`

KDF parameters are only consumed here, and only for `key_source == 1` (Passphrase):

Rust  
let params \= Params::new(env.kdf\_m\_kib, env.kdf\_t, env.kdf\_p, Some(32))  
    .map\_err(|\_| "vault\_parse\_failed")?;   // line 725–726  
let argon2 \= Argon2::new(Algorithm::Argon2id, Version::V0x13, params);  
let res \= argon2.hash\_password\_into(\&pass\_bytes, \&env.salt, out);

For `key_source == 2` (Keychain), the function calls `keychain_load_key(out)` without ever touching the KDF params. For `key_source == 4` (MockProvider), it sets `*out = [0x42u8; 32]` — also without touching KDF params.

#### **1.4 Deployment constants**

**File:** `qsl/qsl-client/qsc/src/vault/mod.rs:29–31`

Rust  
const KDF\_M\_KIB: u32 \= 19456;   // 19 MiB (OWASP Argon2id category 2\)  
const KDF\_T: u32 \= 2;  
const KDF\_P: u32 \= 1;

These constants are used only for **vault init** (writing new vaults) — at line 408 (during `vault_init`), and at lines 500–502 (during `encode_envelope`). When **reading** a vault, the KDF parameters come from the file itself, not these constants.

---

### **2\. Central finding: the "validation floor from M-1's fix" does not exist**

The problem statement refers to "the validation floor added in M-1's fix." Based on a full reading of the codebase, **no such validation floor exists in `parse_vault_envelope` or `parse_envelope`.**

The `M-1` label in this repository's audit documents (`docs/audit/METADATA_LEAKAGE_AUDIT_NA-0134.md`) refers to "Optional fixed-interval polling mode" (a traffic-analysis mitigation), unrelated to vault KDF parameters. There is no separate documented fix to the vault format parser that adds a KDF minimum floor.

The only floor that exists is:

1. The `argon2` crate's `Params::new` minimum constraints, applied **lazily** at `derive_runtime_key` time (not at parse time).  
2. This guard only applies for `key_source == 1`.

This means the audit has two possible interpretations:

* **(A) The floor should exist but doesn't** — the audit is finding a security gap, and the fuzzer is designed to confirm that gap.  
* **(B) The floor was intended to be `Params::new` rejection** — the fuzzer validates that Argon2's constraints are the correct floor, applied correctly.

Both interpretations are audited below.

---

### **3\. Argon2id parameter boundary analysis**

From `argon2` crate v0.5.x behavior (confirmed by the `Params::new` call in the codebase):

| Parameter | Argon2id minimum | Argon2id maximum | Deployment constant | Result of below-minimum |
| ----- | ----- | ----- | ----- | ----- |
| `kdf_m_kib` (`m_cost`) | 8 KiB | 0x0FFFFFFF (268 GiB) | 19456 KiB | `Params::new → Err` → `vault_parse_failed` |
| `kdf_t` (`t_cost`) | 1 | uncapped in `Params` | 2 | `Params::new → Err` → `vault_parse_failed` |
| `kdf_p` (`p_cost`) | 1 | 0xFFFFFF | 1 | `Params::new → Err` → `vault_parse_failed` |
| `kdf_m_kib` | ≥ `8 * kdf_p` | — | — | `Params::new → Err` → `vault_parse_failed` |

**The gap between the Argon2id library floor and the deployment floor is the primary security concern:**

* `kdf_m_kib = 8` (argon2 minimum): `Params::new` **succeeds**. Argon2id runs with 8 KiB of memory — 2,432× weaker than the deployment standard of 19,456 KiB. The vault is "unlockable" with the correct passphrase, and the KDF computation is trivially fast, providing no brute-force resistance.  
* `kdf_m_kib = 19455` (one below deployment): `Params::new` succeeds, key derivation runs at slightly below the expected hardness.  
* `kdf_t = 0`: `Params::new` fails → `vault_parse_failed`. This case is correctly rejected.  
* `kdf_t = 1` with `kdf_m_kib = 19456`: `Params::new` succeeds, key derivation runs with one-half the expected iteration count.  
* `kdf_m_kib = u32::MAX` (4 GiB): `Params::new` may return `Err(InvalidM)` because it exceeds `MAX_M_COST = 0x0FFFFFFF`. The guard at line 726 catches this.  
* `kdf_t = u32::MAX`: `Params::new` may succeed, leading to an effectively infinite computation — a CPU-time DoS.

**Specifically for `key_source = 2` (Keychain) or `key_source = 4` (MockProvider):** Any values of `kdf_m_kib`, `kdf_t`, and `kdf_p` — including `0, 0, 0` — are accepted by `parse_vault_envelope` and `parse_envelope` without any guard. The KDF params are stored in `VaultRuntimeEnvelope` but never read. This is architecturally correct (keychain-backed vaults don't use passphrase KDF), but it means a tampered vault with `key_source=2` and intentionally malformed KDF params will parse successfully. If an attacker can write to the vault file and change `key_source` from 1 to 2, they eliminate the passphrase-KDF requirement entirely — but this requires write access to the vault file, which is a local privilege escalation (already known).

---

### **4\. Existing fuzz harness state**

**File:** `qsl/qsl-client/qsc/fuzz/fuzz_targets/qsc_vault_envelope.rs`

The existing harness is:

Rust  
fuzz\_target\!(|data: &\[u8\]| {  
    let \_ \= parse\_vault\_envelope(data);  
});

This harness exercises the structural parse path only. It will:

* Confirm no panics from structural violations.  
* Drive the fuzzer to produce structurally valid envelopes.  
* **NOT** test KDF parameter boundaries, because `parse_vault_envelope` accepts any `u32` values for those fields without validation.

**Existing corpus seeds** (`qsl/qsl-client/qsc/fuzz/corpus/qsc_vault_envelope/`):

* `magic_only.bin` (7 bytes): `QSCV01 0a` — triggers early length reject.  
* `minimal_valid.bin` (69 bytes): `key_source=1, kdf_m_kib=4096, kdf_t=3, kdf_p=1, ct_len=16` — note that `kdf_m_kib=4096` is well **below** the deployment floor of 19,456 and the fuzzer accepts this without complaint.

**The CI script** (`scripts/ci/qsc_adversarial.sh`) runs:

sh  
cargo \+nightly fuzz run qsc\_vault\_envelope "${run\_dir}" \-- \-max\_total\_time=10

with 10 seconds of fuzzing. This is a smoke-only run; it is not a thorough adversarial validation of KDF parameters.

---

### **5\. Detailed harness design for vault KDF adversarial testing**

The audit identifies three distinct harness requirements that are not met by the current `qsc_vault_envelope` harness.

#### **5.1 Harness 1: `qsc_vault_kdf_params` (extended KDF validation harness)**

**Location:** `qsl/qsl-client/qsc/fuzz/fuzz_targets/qsc_vault_kdf_params.rs` (new target)

**Entry point:** `parse_vault_envelope` (same as current harness)

**Additional invariants to assert post-parse:**

After `parse_vault_envelope` returns `Ok(env)`, assert the following:

* **I-KDF-1:** `env.kdf_m_kib >= KDF_MIN_M_KIB` (where `KDF_MIN_M_KIB` should be defined as the deployment floor, 19,456 — not the Argon2 library minimum of 8).

  * Current state: **NOT enforced**. The harness would catch that `kdf_m_kib=8` passes structural parse but violates policy.  
* **I-KDF-2:** `env.kdf_t >= KDF_MIN_T` (deployment floor: 2, not Argon2 library minimum of 1).

  * Current state: **NOT enforced** at parse time.  
* **I-KDF-3:** `env.kdf_p >= 1` (Argon2 library minimum is the acceptable floor here).

  * Current state: the library catches `kdf_p=0` only at `Params::new` time.  
* **I-KDF-4:** For `key_source == 1`, `env.kdf_m_kib >= 8 * env.kdf_p` (Argon2 lane constraint).

  * Current state: caught by `Params::new` only.

Without a **policy-level floor** added to `parse_vault_envelope`, I-KDF-1 and I-KDF-2 cannot be confirmed by the existing harness — the assertions would fire immediately on the existing `minimal_valid.bin` seed (which uses `kdf_m_kib=4096`). This confirms the gap.

**Post-parse chain harness variant:** A more thorough harness would also call `derive_runtime_key` on successfully-parsed envelopes, to verify that sub-policy KDF params are caught before key material is derived:

Rust  
fuzz\_target\!(|data: &\[u8\]| {  
    if let Ok(env) \= parse\_vault\_envelope(data) {  
        // For key\_source=1, check that sub-minimum params fail at Params::new  
        if env.key\_source \== 1 {  
            let result \= argon2::Params::new(env.kdf\_m\_kib, env.kdf\_t, env.kdf\_p, Some(32));  
            // Invariant: params within argon2 library bounds succeed; below-library-minimum fail  
            // POLICY FINDING: no check for deployment floor (19456, 2\) here or anywhere  
        }  
    }  
});

This is an **analysis-only** observation since code changes are out of scope. The harness design reveals the gap.

#### **5.2 Harness 2: specific KDF boundary corpus seeds**

These are exact binary inputs that probe the boundary conditions identified in §3. They should be added to `qsl/qsl-client/qsc/fuzz/corpus/qsc_vault_kdf_params/`.

**Seed layout** (all values little-endian; all seeds use `key_source=1` unless noted):

| Seed name | `kdf_m_kib` | `kdf_t` | `kdf_p` | Purpose | `parse_vault_envelope` result |
| ----- | ----- | ----- | ----- | ----- | ----- |
| `kdf_m_zero.bin` | `0` | `1` | `1` | Below Argon2 `MIN_M` | `Ok` (parse passes\!) |
| `kdf_m_seven.bin` | `7` | `1` | `1` | One below Argon2 minimum | `Ok` (parse passes\!) |
| `kdf_m_argon_min.bin` | `8` | `1` | `1` | Argon2 floor, below deployment | `Ok` (parse passes, KDF runs weak) |
| `kdf_m_deploy_minus1.bin` | `19455` | `2` | `1` | One below deployment floor | `Ok` (parse passes, KDF runs slightly weak) |
| `kdf_m_deploy.bin` | `19456` | `2` | `1` | Deployment-correct values | `Ok` |
| `kdf_t_zero.bin` | `19456` | `0` | `1` | `kdf_t=0` below Argon2 min | `Ok` (parse passes\!) |
| `kdf_t_one.bin` | `19456` | `1` | `1` | Below deployment `KDF_T=2` | `Ok` (parse passes, weaker) |
| `kdf_p_zero.bin` | `19456` | `2` | `0` | `kdf_p=0` below Argon2 min | `Ok` (parse passes\!) |
| `kdf_m_max_argon.bin` | `0x0FFFFFFF` | `1` | `1` | Max Argon2 `m_cost` | `Ok` if bytes present; DoS risk |
| `kdf_t_max.bin` | `19456` | `0xFFFFFFFF` | `1` | Max `t_cost` | `Ok` if bytes present; CPU DoS |
| `kdf_p_max.bin` | `19456` | `2` | `0xFFFFFF` | Max `p_cost` | `Ok` if bytes present; DoS risk |
| `kdf_all_zeros.bin` | `0` | `0` | `0` | All-zero params | `Ok` (parse passes\!) |
| `key_src2_kdf_zero.bin` | `0` | `0` | `0` | `key_source=2`, zero params | `Ok` (no KDF validation at all) |
| `kdf_m_lane_violate.bin` | `8` | `1` | `2` | `kdf_m < 8*kdf_p` | `Ok` (parse passes\!) |

**Key observation for the test author:** All seeds with `kdf_m_kib=0`, `kdf_t=0`, `kdf_p=0` will produce `parse_vault_envelope → Ok(...)` because the parser does no semantic validation. This is the central finding that confirms the absent validation floor.

The hex construction for each seed follows the format shown in §1.1. For example, `kdf_t_zero.bin`:

Code  
magic:    5153435630 31         QSCV01  
key\_src:  01                    1 (Passphrase)  
salt\_len: 10                    16  
nonce\_len:0c                    12  
kdf\_m:    004c000000            19456 LE  
kdf\_t:    0000000000            0 LE  
kdf\_p:    0100000000            1 LE  
ct\_len:   1000000000            16 LE  
salt:     \[16 bytes\]  
nonce:    \[12 bytes\]  
ct:       \[16 bytes\]

Total: 69 bytes.

#### **5.3 Harness 3: integration-level harness — `qsc_vault_unlock`**

A separate fuzzer or property test should combine structural parse with the key-derivation step for `key_source=1`, to confirm the layered rejection behavior. This is the closest equivalent to testing the full "validation floor" path:

Rust  
fuzz\_target\!(|data: &\[u8\]| {  
    if let Ok(env) \= parse\_vault\_envelope(data) {  
        if env.key\_source \== 1 {  
            // Drive into Params::new to see what the argon2 library accepts  
            let \_ \= argon2::Params::new(env.kdf\_m\_kib, env.kdf\_t, env.kdf\_p, Some(32));  
            // INVARIANT: if Params::new succeeds with kdf\_m\_kib \< 19456 or kdf\_t \< 2,  
            // this is a policy violation (not caught, would run a weak KDF on unlock)  
        }  
    }  
});

Since fuzzing is non-deterministic and `Argon2.hash_password_into` is too slow to call in a tight fuzz loop (it would take \>100ms per iteration at deployment parameters), this harness must **not** call the full KDF — only `Params::new`. Adding this as a separate target (`qsc_vault_kdf_rejection`) would provide coverage of the `Params::new` failure path.

---

### **6\. Specific test cases required for the "validation floor" assertion**

The following deterministic test cases (suitable for `adversarial_properties.rs` or `adversarial_miri.rs`) correspond directly to the audit mandate of verifying that below-threshold configurations are rejected. They are currently **absent** from the test suite.

| Test case | Input | Expected result | Currently tested? |
| ----- | ----- | ----- | ----- |
| `kdf_m_kib=0` at parse layer | `parse_vault_envelope(seed)` | `Ok(env)` — parse does not reject | **No** |
| `kdf_t=0` at parse layer | `parse_vault_envelope(seed)` | `Ok(env)` — parse does not reject | **No** |
| `kdf_m_kib=0` at derive layer (`key_source=1`) | `Params::new(0,1,1,Some(32))` | `Err` — Argon2 rejects | **No** |
| `kdf_t=0` at derive layer (`key_source=1`) | `Params::new(19456,0,1,Some(32))` | `Err` — Argon2 rejects | **No** |
| `kdf_m_kib=8` at parse layer | `parse_vault_envelope(seed)` | `Ok(env)` — library floor passes | **No** |
| `kdf_m_kib=8` at derive layer | `Params::new(8,1,1,Some(32))` | `Ok` — **policy violation not caught** | **No** |
| `kdf_m_kib=19455` at derive layer | `Params::new(19455,2,1,Some(32))` | `Ok` — **deployment floor not enforced** | **No** |
| `key_source=2, kdf_m=0` | `parse_vault_envelope(seed)` → `derive_runtime_key` | `Err(vault_locked)` (no keychain) | **Indirectly** (integration) |

The cases `kdf_m_kib=8` through `19455` are the most important: **these will pass `Params::new` and allow Argon2id to run with arbitrarily weakened parameters**, producing a usable key from the correct passphrase but with negligible brute-force cost.

---

### **7\. Structural findings**

#### **Finding 7.1 — CRITICAL gap: no policy-level minimum floor for KDF parameters at parse time**

**Severity:** P1 (per DOC-AUD-001 §6 schema)

* **File:** `qsl/qsl-client/qsc/src/adversarial/vault_format.rs:13` (`parse_vault_envelope`)  
* **Additional file:** `qsl/qsl-client/qsc/src/vault/mod.rs:697` (`parse_envelope`)  
* **Invariant violated:** The vault passphrase-KDF floor (deployment constants `KDF_M_KIB=19456`, `KDF_T=2`) is enforced only at write time (vault init, line 408\) — not at read time. An attacker who can write a tampered vault file can supply `kdf_m_kib=8, kdf_t=1` and the vault will parse and unlock (given the correct passphrase) while running a trivially fast Argon2id computation.  
* **Why it matters:** An attacker with local file access (or the ability to replace the vault file) can pre-compute a rainbow table against the minimum-parameters vault and brute-force the passphrase offline in seconds rather than days.  
* **Proof gap:** No test or fuzz assertion checks `env.kdf_m_kib >= KDF_M_KIB` after a successful parse. The existing corpus seed `minimal_valid.bin` uses `kdf_m_kib=4096` (one-fifth the floor) and this is never flagged.

#### **Finding 7.2 — MODERATE gap: KDF validation is lazy and conditioned on key\_source**

**Severity:** P2

* **File:** `qsl/qsl-client/qsc/src/vault/mod.rs:709–738` (`derive_runtime_key`)  
* **File:** `qsl/qsl-client/qsc/src/adversarial/vault_format.rs:13` (`parse_vault_envelope`)  
* **Invariant violated:** For `key_source ∉ {1}`, no KDF parameter validation ever occurs. A vault with `key_source=4` (MockProvider) and `kdf_m_kib=0, kdf_t=0, kdf_p=0` parses, "unlocks," and decrypts normally (MockProvider vaults use a hardcoded key).  
* **Why it matters:** In test/CI environments that use MockProvider (`key_source=4`), KDF parameter validation is entirely bypassed. If MockProvider vaults ever escape to production or are subject to trust-boundary confusion, the absence of any validation is a gap.  
* **Proof gap:** No test verifies that `parse_vault_envelope` rejects (or flags) nonsensical KDF params for non-passphrase key sources.

#### **Finding 7.3 — Informational: `ct_len` usize cast is safe on 64-bit but latently unsafe on 32-bit**

**Severity:** P3 (informational)

* **File:** `qsl/qsl-client/qsc/src/adversarial/vault_format.rs:35–36`  
* `ct_len = u32::from_le_bytes([...]) as usize`  
* On 64-bit platforms (qsc's only target): `need = 25 + 16 + 12 + ct_len` cannot overflow for any `ct_len ≤ u32::MAX`. Safe.  
* On 32-bit platforms: `need` would overflow for `ct_len > 4294967243`, causing the bounds check at line 39 to be bypassed, potentially leading to a slice-index panic. qsc is 64-bit only in practice.  
* **Proof gap:** No platform-width guard or `checked_add` in the `need` calculation.

#### **Finding 7.4 — Informational: large `kdf_t` values can cause CPU-time DoS via successful `Params::new`**

**Severity:** P3 (informational)

* **File:** `qsl/qsl-client/qsc/src/vault/mod.rs:725–728`  
* `Params::new(env.kdf_m_kib, env.kdf_t, ...)` may succeed for very large `kdf_t` values (if within `Params` bounds). A subsequent `argon2.hash_password_into` call would then block for an arbitrarily long time.  
* **Why it matters:** An attacker who can write a tampered vault file can set `kdf_t=0x7FFFFFFF` to cause the unlock operation to block for hours or indefinitely.  
* **Proof gap:** No upper-bound check on `kdf_t` exists at parse or derive time. The CI adversarial smoke test uses `-max_total_time=10` for the fuzzer run, so a 10-second block per fuzz iteration would cause the fuzzer itself to stall.  
* **Harness note:** Any fuzz harness that calls the full KDF (not just `Params::new`) must use a test-specific passphrase key source or mock to avoid stalling.

---

### **8\. Harness placement and CI integration**

The existing infrastructure handles this well:

| Component | Path | Notes |
| ----- | ----- | ----- |
| Fuzz workspace | `qsl/qsl-client/qsc/fuzz/` | Already has `cargo-fuzz` manifest at `fuzz/Cargo.toml` |
| Existing vault harness | `fuzz/fuzz_targets/qsc_vault_envelope.rs` | Target for structural parse only |
| **New KDF harness** | `fuzz/fuzz_targets/qsc_vault_kdf_params.rs` | Should add `Params::new` assertions post-parse |
| Existing corpus | `fuzz/corpus/qsc_vault_envelope/` | Two seeds: `magic_only.bin`, `minimal_valid.bin` |
| **New corpus** | `fuzz/corpus/qsc_vault_kdf_params/` | 14 seeds from §5.2 above |
| CI script | `scripts/ci/qsc_adversarial.sh` | Needs `run_fuzz_target qsc_vault_kdf_params` addition |
| CI workflow | `.github/workflows/qsc-adversarial.yml` | Runs the shell script; no structural change needed |
| Deterministic tests | `qsl/qsl-client/qsc/tests/adversarial_properties.rs` | Should add 8 property tests from §6 above |

The existing fuzz smoke pattern (`-max_total_time=10`) is appropriate for the structural parse harness. For the KDF-params harness, the `Params::new`\-only variant is safe at the same time budget. Any harness that invokes `hash_password_into` must use MockProvider credentials to avoid CI stalling.

---

### **9\. Primary source reference map**

| Symbol / location | Role |
| ----- | ----- |
| `qsl/qsl-client/qsc/src/adversarial/vault_format.rs:13` `parse_vault_envelope` | Primary fuzz target; structural parse only; **no KDF validation** |
| `qsl/qsl-client/qsc/src/adversarial/vault_format.rs:35–36` | `ct_len = u32 as usize` (safe on 64-bit) |
| `qsl/qsl-client/qsc/src/vault/mod.rs:697` `parse_envelope` | Internal wrapper; passes KDF params through without validation |
| `qsl/qsl-client/qsc/src/vault/mod.rs:709` `derive_runtime_key` | Only site of KDF param validation; lazy and key\_source-conditioned |
| `qsl/qsl-client/qsc/src/vault/mod.rs:725–726` `Params::new` | Argon2 library floor check (catches `< 8 KiB`, `t_cost < 1`); does NOT check deployment floor |
| `qsl/qsl-client/qsc/src/vault/mod.rs:29–31` `KDF_M_KIB`, `KDF_T`, `KDF_P` | Deployment constants (write-time only; not enforced at read time) |
| `qsl/qsl-client/qsc/src/vault/mod.rs:408–412` | Vault init: `Params::new(KDF_M_KIB, KDF_T, KDF_P, ...)` — enforces floor only on write |
| `qsl/qsl-client/qsc/fuzz/fuzz_targets/qsc_vault_envelope.rs` | Existing harness; no KDF assertion |
| `qsl/qsl-client/qsc/fuzz/corpus/qsc_vault_envelope/minimal_valid.bin` | `kdf_m_kib=4096` — below deployment floor, accepted without flagging |
| `qsl/qsl-client/qsc/fuzz/Cargo.toml` | `cargo-fuzz` manifest; add new `[[bin]]` entry for KDF harness |
| `qsl/qsl-client/qsc/tests/adversarial_miri.rs:74` `miri_vault_parser_rejects_short_input` | Only existing vault unit test; structural only |
| `qsl/qsl-client/qsc/tests/adversarial_properties.rs:208` `malformed_vault_envelope_rejects_without_panic` | Only existing vault property test; structural only |
| `scripts/ci/qsc_adversarial.sh` | Smoke runner; add `run_fuzz_target qsc_vault_kdf_params` |
| `.github/workflows/qsc-adversarial.yml` | CI job; already runs the shell script |
| `docs/design/DOC-G4-001_Adversarial_Validation_Fuzz_Chaos_Program_and_Post_Audit_Priority_Decision_v0.1.0_DRAFT.md` | Adversarial program design; §3 scopes to qsc vault/store surfaces |
| `docs/audit/DOC-AUD-001_qsc_Director_Ready_Crypto_and_Code_Audit_Program_v0.1.0_DRAFT.md:§4.4` | Names "parser-boundary fuzzing" and "targeted adversarial validation expansion" as high-value follow-on |

