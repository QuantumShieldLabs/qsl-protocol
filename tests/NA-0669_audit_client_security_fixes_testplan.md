# NA-0669 — the three audit client security fixes: test plan and executed results

**Lane:** NA-0669 · **Decisions:** D-1295, D-1296 · **Directive:** QSL-DIR-2026-07-23-605 (D605), sha256 `4852f471e7bf10d91bd9a8d95d7e08f6af46b8276d816b081e71a74de9274b73`
**Base:** spine `62118874cac9ef54e90e507211d95a2dc4bbdfb6`

---

## ⚠ 0. UNLIKE THE LAST FOUR LANES, CI IS THE INSTRUMENT HERE

The four preceding lanes shipped `docs_only=true` spine diffs, so both full suites SKIPPED and their green checks carried no information about the work. Their test plans existed precisely because CI could not stand in.

**This lane is different and the difference is verified, not asserted:**

```
$ bash scripts/ci/classify_ci_scope.sh <this PR's file set>
docs_only=false   workflow_security=false   runtime_critical=true   scope_class=runtime_critical

$ bash scripts/ci/classify_ci_scope.sh NEXT_ACTIONS.md DECISIONS.md docs/ops/ROLLING_OPERATIONS_JOURNAL.md
docs_only=true                        <- POSITIVE CONTROL: the classifier can still say docs_only
```

Both full suites run, and one of the three fixes changes the output of a function that a test independently reimplements and feeds back through the CLI — so **the suite could have gone red.**

**This document therefore covers what CI cannot see**, which is exactly two things: the durability fix (§3, unobservable without cutting power) and the quantitative claims about entropy (§2, §4), which are measurements rather than pass/fail assertions.

---

## 1. Harness

| | |
|---|---|
| Toolchain | `rustc 1.95.0-x86_64-unknown-linux-gnu` (the lane's pinned `qwork` target key) |
| Suite | `cargo test -p qsc`, run locally — `qsc-linux-full-suite` **skips on PRs**, so the local run is the real pre-merge check |
| Binaries | `qsc` built twice: once from the base tree, once from the fixed tree, both retained for before/after comparison |
| Library probe | An **external consumer crate**, outside the repository, with a path dependency on `qsc`, calling `qsc::identity::format_verification_code_from_fingerprint` directly. Built against the base tree and the fixed tree. It uses a **copy** of the workspace `Cargo.lock` so the 12-pin RustCrypto alignment holds; without it `quantumshield_refimpl` fails to compile on fresh resolution (the known ENG-0017-class trap). |
| Repo integrity | `git status --porcelain` re-checked after every build against the tree. `Cargo.toml`/`Cargo.lock` never moved. |

**Why an external consumer rather than a new test file.** §5a does not authorize a new `tests/*.rs`, and adding one would have been a STOP. The probe lives entirely in scratch space, links the real library, and calls the real function — so the evidence is real derivation, not a model, without touching a file the lane may not touch.

---

## 2. C-1a — verification code entropy

### 2.1 Real identities, real derivation (§7.1)

Five identities created with the real built binary:

```
qsc vault init --non-interactive --passphrase-file <file>
qsc identity rotate --confirm --unlock-passphrase-file <file>
```

Their real fingerprints run through the real production function, base tree vs fixed tree:

```
FINGERPRINT (real)                        BEFORE                   AFTER
QSCFP-62da0931a8fa6bff1f766b1108896fc6    QSCF-P62D-A093-1A8F-P    62DA-0931-A8FA-6BFF-Y
QSCFP-8fa9e770a177f30e65a8f28443a12800    QSCF-P8FA-9E77-0A17-1    8FA9-E770-A177-F30E-9
QSCFP-6106dbe02fa997c392c3abd7ffa08d51    QSCF-P610-6DBE-02FA-Y    6106-DBE0-2FA9-97C3-0
QSCFP-0a9e07d057c27039beb2b269a98e4d1c    QSCF-P0A9-E07D-057C-P    0A9E-07D0-57C2-7039-Y
QSCFP-8253cc956d42273fa8e271c2a0f24605    QSCF-P825-3CC9-56D4-H    8253-CC95-6D42-273F-8
```

**PASS** — the constant `QSCF-P` is absent from every AFTER code.

### 2.2 Position analysis at 4000 samples

4000 fingerprints in the exact real shape (`QSCFP-` + 32 hex), through the real function in both builds:

```
BEFORE  n=4000 width=21  constant display positions [0,1,2,3,4,5,9,14,19] -> "QSCF-P---"
        varying hex in body = 11  ->  44 bits
AFTER   n=4000 width=21  constant display positions [4,9,14,19]           -> "----"
        varying hex in body = 16  ->  64 bits
```

**PASS — 44 → 64 bits.** Position-for-position identical to the directive's modeled §1a table, now confirmed against the real function.

**⚠ 64 bits at width 16.** 16 hex characters = 64 bits. The intent's "88 bits / 22 hex chars" was an arithmetic error corrected in the directive before execution: 88 bits needs 22 characters, which needs the target-width change this lane excludes, and 22 is not divisible by the four-character display grouping.

**⚠ The directive's own "20-fold reduction" phrasing is a 20-BIT reduction** — a factor of 2²⁰ ≈ 1,048,576. The bit figures and every ruling resting on them are unaffected.

### 2.3 The shadow copies — BOTH of them (§7.2)

There were **three** implementations of the formatter, not two. The directive named only one shadow copy; the full local suite caught the second.

| # | Location | Name | Status |
|---|---|---|---|
| 1 | `src/identity/mod.rs:527` | `format_verification_code_from_fingerprint` | production, fixed |
| 2 | `tests/identity_binding.rs:37` | `format_verification_code_from_fingerprint` | shadow #1 (directive-named), fixed in lockstep |
| 3 | `tests/identity_foundation_contract_na0217d.rs:133` | `verification_code_from_fingerprint` | shadow #2 (**unnamed**), fixed in lockstep under operator ruling (A) |

Both shadows were updated in lockstep and **kept as duplicates** — not replaced by imports — because their separateness is what makes each test a real check rather than a tautology.

**No assertion was weakened, and no expected value needed editing in either:** each test *computes* its expected code rather than pinning a literal, so the lockstep formula update **is** the value update. `handshake_accepts_verification_code_pin_without_peer_mismatch` (`identity_binding.rs:413`) and `verification_code_pin_preserves_handshake_contract` (`na0217d.rs`, assertion at `:294`) both pass unchanged under F1(a).

**⚠ HOW SHADOW #2 WAS FOUND, AND THE LESSON.** The **first full `cargo test -p qsc` went RED** on `verification_code_pin_preserves_handshake_contract`: shadow #2 still emitted the old format, so its pinned code (`QSCF-P82F-…`) no longer matched production's new-format code (`82F6-…`) → `peer_mismatch`. It was missed by the directive's authoring grep and by this executor's Phase-0 caller-sweep because both keyed on the **symbol name** `format_verification_code_from_fingerprint`, and shadow #2 is named `verification_code_from_fingerprint` — no `format_` prefix. Only a sweep for the **function body** (the Crockford constant `0123456789ABCDEFGHJKMNPQRSTVWXYZ`) finds all three. **Lesson: hunt duplicate implementations by a distinctive body fragment, not the symbol name — names diverge, bodies do not.** This is the strongest evidence item in the lane: the suite was **demonstrated to reject a real defect** before it was trusted to accept — a stronger story than a first-try green. The clean re-run to completion (required before §7 is filled) is trustworthy precisely because of it.

**No fourth copy:** qsl-desktop (mirror main `02cc9b9`, body-swept) holds none of its own; it calls the qsc production symbol and pins no format literal, so its CI will not break on the pin bump. Report-only.

### 2.4 Other test anchors — checked, not assumed

| Anchor | Expectation | Result |
|---|---|---|
| `tests/identity_ux.rs:141-142` | directive flagged as possibly moving | **Unaffected.** 16 hex after the prefix, so no padding either way; assertions check ordering and `identity_fp=QSCFP-` (the fingerprint), never the code |
| `tests/NA_0634_full_identity_provisioning.rs:89-95` | mirrors the fingerprint, not the code | **Unaffected**, confirmed rather than assumed |
| `tests/NA_0649_gui_surface.rs:311-312` | **not named in the directive** — found by sweeping all callers | **Unaffected.** Asserts only shape and determinism, both preserved |

---

## 3. C-6 — durability. ⚠ WHAT THIS PLAN CANNOT PROVE

**The fix is unobservable by any test in this suite.** A missing `fsync` on a parent directory changes nothing a process can see; it changes what survives a power failure. The suite does not cut power, and this lane did not build a crash rig.

What is proved instead, mechanically:

1. **The call is present after the rename**, byte-parallel to the pre-existing `vault_init_core` reference — shown side by side in the as-built §3.
2. **`parent` was already bound**, so nothing new is computed.
3. **The helper cannot add an error path** — `fsync_dir_best_effort` is `#[cfg(unix)] File::open(dir).sync_all()` with a `#[cfg(not(unix))]` no-op, returning `()`.
4. **The three callers were enumerated, not assumed** — `secret_set:240`, `secret_set_with_passphrase:273`, `persist_session:385`: every steady-state vault mutation in the product.
5. **It was the only rename-then-nothing site in the crate** — the other durability paths (`vault_init_core:587`, `protection.rs:281/:430/:449`, `fs_store/mod.rs:123`, `lib.rs:2311`) already fsync.

**The suite green proves non-regression for C-6, and nothing more.** Stated here and in the as-built so no reader mistakes the green for durability evidence.

*(A crash-consistency rig — the NA-0639 `WF-0022` atomic-write-crash-window shape — would be the real instrument. It was not in scope and is not claimed.)*

---

## 4. C-4 — passphrase file entropy

### 4.1 The loss, measured through the real `from_utf8_lossy`

Complete enumeration of the single-byte input space through Rust's real function:

```
distinct outcomes = 129 (of 256 inputs)
Shannon H         = 4.500 bits of 8
destroyed         = 3.500 bits/byte (43.8%)
32-byte file (independent-byte extrapolation) = 144.0 bits of 256
```

Mechanism: `0x00-0x7F` map to themselves; **all 128 bytes `0x80-0xFF` collapse to the single character U+FFFD.**

Whole-file corroboration, 200,000 random 32-byte trials:

```
valid UTF-8 (would survive untouched) = 0 (0.000000%)
mean U+FFFD replacements per file     = 13.35
```

**Confirms the directive's ≈144 and the audit's 145–160.** ⚠ The 144 figure is an independent-byte extrapolation (32 × 4.5); the true value is slightly higher because adjacent bytes can form surviving multi-byte sequences. It is nowhere near 256, which is the point.

### 4.2 CLI behavior, with a positive control (§7.4)

Same `head -c 32 /dev/urandom > raw.txt` (confirmed invalid UTF-8), base binary vs fixed binary:

| # | Binary | Action | Result |
|---|---|---|---|
| 1 | BEFORE | `vault init --passphrase-file raw.txt` | `event=vault_init`, **exit 0** — silently accepted |
| 2 | AFTER | same | `event=error code=vault_passphrase_file_read_failed`, **exit 1** |
| 3 | AFTER | open the vault created at step 1 | `event=error code=vault_passphrase_file_read_failed`, **exit 1** |
| 4 | AFTER | `vault init` + `identity rotate` + `identity show` with base64 of the **same** 32 bytes | all **ok=true**, exit 0 |

**Step 1 is the positive control**: the rig demonstrably returns "accepted" against the pre-fix binary, so step 2's rejection is a measured change and not a broken harness. **PASS.**

**Step 3 is the accepted F2 migration hazard, demonstrated rather than asserted** — see §4.4.

### 4.3 Byte-for-byte preservation into Argon2id

```
raw.txt                                : 32 bytes, 256 bits on disk
from_utf8_lossy(raw)                   : 64 bytes reaching Argon2id BEFORE — not the file's bytes
raw survives verbatim?                   False
b64.txt                                : 44 bytes
String::from_utf8(b64)                 : 44 bytes reaching Argon2id AFTER
b64 survives verbatim?                   True
b64 decodes to the original 32 bytes?    True
```

**PASS — all 256 bits reach Argon2id unmodified.** Note the pre-fix path handed Argon2id **64 bytes** for a 32-byte file (each U+FFFD is 3 UTF-8 bytes): a longer string carrying less information. `derive_key(… pass_bytes: &mut [u8] …)` already takes bytes straight into `hash_password_into`, so nothing downstream changed.

### 4.4 ⚠ The accepted migration hazard

**Every existing vault created from a non-UTF-8 passphrase file is now permanently unopenable.** The lossy transform previously ran at both init and unlock, making such vaults self-consistent; both candidate fixes break that. **F2: accepted, no migration** — pre-release, no recovery path. Reject is the kinder candidate: it fails with the named `vault_passphrase_file_read_failed` rather than as an indistinguishable wrong key.

---

## 5. Regression

`cargo test -p qsc` — full local run. Results in §7.

**rustfmt:** `cargo fmt --check` reports pre-existing drift across 42 files in this crate (the known owed micro-lane). Verified by diffing the complete `cargo fmt --check` output before and after: **identical apart from two line-number shifts caused by this lane's own insertion.** `src/identity/mod.rs` appears in neither list. Zero new drift introduced; none repaired (out of scope).

---

## 6. Scope verification

```
qsl/qsl-client/qsc/src/identity/mod.rs
qsl/qsl-client/qsc/src/vault/mod.rs
qsl/qsl-client/qsc/tests/identity_binding.rs
qsl/qsl-client/qsc/tests/identity_foundation_contract_na0217d.rs   (added by operator ruling (A))
```

**Four** files. The first three are the directive's §5a; the fourth was added by explicit operator ruling (A) after the first full run caught shadow #2 (§2.3), bounded to the one lockstep prefix-strip at `:133`, duplicate kept, assertion at `:294` unchanged. `Cargo.toml`/`Cargo.lock` verified untouched — **including after building an external consumer crate against the tree**, which was the one step in this plan that could plausibly have moved them.

`binding_fuzz.rs` byte-unchanged (STOP condition). `identity_pin_matches_seen:562` byte-unchanged (F1(a), forbidden path). No file was edited outside §5a-as-extended; the one file that needed it triggered a STOP and a ruling before any edit.

---

## 7. Executed suite results

**`cargo test -p qsc` — CLEAN GREEN TO COMPLETION** (real exit code, no fail-fast abort):

```
CARGO_EXIT=0
110 result lines (integration binaries + lib unit + doc-tests)
434 tests passed · 0 failed · 0 panics · 0 compile errors
wall time ≈ 1h40m (top binary ~1277s: aws_file_medium_boundary, debug Argon2id)
```

The C-1a lockstep proof, both shadows, same run:

```
identity_binding.rs                       :: handshake_accepts_verification_code_pin_without_peer_mismatch ... ok  (3 passed)
identity_foundation_contract_na0217d.rs   :: verification_code_pin_preserves_handshake_contract           ... ok  (2 passed)
```

**Two runs, and the pair is the point.** Run 1 (before shadow #2 was fixed) went RED at
`verification_code_pin_preserves_handshake_contract` and aborted fail-fast (~71 binaries unrun). Run 2
(after the ruling-(A) lockstep fix) completed clean. The instrument was demonstrated to reject the real
defect before it was trusted to accept — the positive-control property discharged by observation.

---

## 8. What this plan does not cover

- **Crash consistency for C-6** (§3) — no power-cut rig; the fix rests on parity with `vault_init_core`, not on a test.
- **The exact entropy of a 32-byte file under `from_utf8_lossy`** — the 144-bit figure is an independent-byte extrapolation, labeled as such (§4.1).
- **qsl-desktop's displayed code** — changes only when desktop bumps its qsc pin, a separate lane. No desktop test pins the format, so desktop CI will not break.
- **`identity_pin_matches_seen` semantics** — untouched by ruling; the production/oracle divergence stands as a known, unasserted gap for the vault-format lane.
