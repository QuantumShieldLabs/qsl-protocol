# NA-0669 — as built

**Lane:** NA-0669 · **Decisions:** D-1295 (implementation), D-1296 (closeout) · **Directive:** QSL-DIR-2026-07-23-605 (D605), sha256 `4852f471e7bf10d91bd9a8d95d7e08f6af46b8276d816b081e71a74de9274b73`, 374 lines
**Base:** spine `62118874cac9ef54e90e507211d95a2dc4bbdfb6` (PR #1629, the NA-0669 queue promotion)
**Result class:** `AUDIT_CLIENT_SECURITY_THREE_FIXES_PASS`

---

## ⚠ 1. THE CI GREEN ON THIS PR *IS* EVIDENCE — AND THAT IS THE POINT

Read this before anything else, because it is the exact inverse of the note that opened the last four as-builts.

`scripts/ci/classify_ci_scope.sh` over this PR's file set returns:

```
docs_only=false
workflow_security=false
runtime_critical=true
scope_class=runtime_critical
```

**Both full suites RUN.** The change set alters behavior the suites exercise — one of the three fixes changes the output of a function a test independently reimplements and feeds back through the CLI — so **a green result here could have been red.**

That is the property NA-0668's standing method demands: *a negative result is only evidence if the instrument could have returned positive.* It is discharged here **by construction rather than by argument**, and the discharge was itself checked with a positive control rather than assumed:

```
$ bash scripts/ci/classify_ci_scope.sh NEXT_ACTIONS.md DECISIONS.md docs/ops/ROLLING_OPERATIONS_JOURNAL.md
docs_only=true          <- the classifier CAN still return docs_only; this negative is meaningful
```

**Contrast with the four lanes immediately preceding.** NA-0664, NA-0667 and NA-0668 all shipped spine diffs that classified `docs_only=true`, so both full suites SKIPPED and their greens proved nothing about the work; NA-0668's real validating evidence had to come from watching a check fail on purpose, because no CI system could see the tooling it changed. This lane needs no such substitute. The suites are the instrument, they are live, and they are pointed at the change.

### ⚠ 1.1 — THE INSTRUMENT WAS DEMONSTRATED TO REJECT A REAL DEFECT BEFORE IT WAS TRUSTED TO ACCEPT

This is the strongest single fact in the as-built, and it is stronger than a first-try green would have been. **The first full local `cargo test -p qsc` run went RED.** It caught a real incompleteness in the C-1a fix that both the directive's census and this executor's Phase 0 had missed: a *second* byte-identical shadow copy of the verification-code formatter, in a test file the directive did not name (see §4.4a). The failing test, `verification_code_pin_preserves_handshake_contract` in `tests/identity_foundation_contract_na0217d.rs`, pinned a code computed by the stale shadow (`QSCF-P82F-…`, old format) while production recomputed it in the new format (`82F6-…`), producing `peer_mismatch`.

A docs-only lane could not have surfaced this. The runtime suite could, and did. So the eventual clean re-run is trustworthy for a reason the four preceding greens never were: **we watched this exact instrument fail correctly, on a real defect, before we trusted it to pass.** That is the positive-control property discharged not by argument but by observation — the negative result the method demands actually occurred, was diagnosed, and was fixed under an explicit operator ruling (A) that extended §5a to the one file, for the one lockstep edit, with the assertion unchanged.

*Recording convention honored:* this as-built does not assert "both suites green as evidence" on the strength of the aborted first run. `cargo test` is fail-fast across binaries, so that run stopped at the failure and left ~71 later binaries unexecuted. The green claim in §6/§7 rests only on the **re-run to completion**, and is written only once that run has exited 0.

**Bounded honestly, because the boundary is not the same for all three fixes.** The suite green is genuine evidence for **C-1a** (behavioral, tested, lockstep-verified) and for **C-4** (behavioral, and the CLI before/after is recorded in §5). It is **NOT** evidence for **C-6**: a missing `fsync` on a directory entry has no observable effect in any test that does not cut power mid-write. For C-6 the green proves **non-regression only**, and §3 states that boundary rather than letting the green imply more than it earns.

---

## 2. What landed

Three defects from the 2026-07-22 independent audit, fixed. **Nothing else.** No new ledger entries were filed — observations went to the operator relay, per the directive's scope discipline.

```
 qsl/qsl-client/qsc/src/identity/mod.rs                          | 13 ++++++++++++-
 qsl/qsl-client/qsc/src/vault/mod.rs                             | 14 +++++++++++++-
 qsl/qsl-client/qsc/tests/identity_binding.rs                    |  9 ++++++++-
 qsl/qsl-client/qsc/tests/identity_foundation_contract_na0217d.rs| 10 +++++++++-
 4 files changed
```

The fourth file, `tests/identity_foundation_contract_na0217d.rs`, was **added to §5a by explicit operator ruling (A)** after the first full run caught its stale second shadow copy (§1.1, §4.4a). The ruling is bounded to the one lockstep prefix-strip at `:133`, duplicate kept, assertion at `:294` unchanged — the identical treatment the directive already blessed for shadow #1. Anything else in that file remains a STOP.

The functional change is **three lines**; the rest is the comment each one needs to survive the next reader, plus the lockstep test update.

| Fix | Site | Change |
|---|---|---|
| **C-6** | `vault/mod.rs` `write_vault_atomic`, after the rename | add `crate::fsync_dir_best_effort(parent);` |
| **C-1a** | `identity/mod.rs:527` `format_verification_code_from_fingerprint` | strip `IDENTITY_FP_PREFIX` before the alphanumeric filter |
| **C-4** | `vault/mod.rs` `read_passphrase_file` | `String::from_utf8(bytes).map_err(…)` replacing `from_utf8_lossy(&bytes).to_string()` |

**Phase 0 — every anchor re-verified at the seating base, not trusted from the directive.** The directive's anchors were exact at `14a2464c`; the seat was `62118874`, two commits later. `git diff --name-status 14a2464c 62118874` returns **`M NEXT_ACTIONS.md` and nothing else**, and each anchor was then confirmed individually by `grep -n`:

| Anchor | Directive | At `62118874` |
|---|---|---|
| `IDENTITY_FP_PREFIX = "QSCFP-"` | `identity/mod.rs:30` | `:30` ✓ |
| `format_verification_code_from_fingerprint` | `:527` | `:527` ✓ |
| alphanumeric filter | `:531` | `:531` ✓ |
| `identity_pin_matches_seen` (**forbidden path**) | `:562` | `:562` ✓ |
| `write_vault_atomic` / `parent` bound / `fs::rename` | `:845` / `:847` / `:870` | `:845` / `:847` / `:870` ✓ |
| `vault_init_core` fsync (parity reference) | `:587` | `:587` ✓ |
| `from_utf8_lossy` / stdin `read_to_string` | `:1109` / `:1122` | `:1109` / `:1122` ✓ |
| shadow copy of the code formatter | `tests/identity_binding.rs:37` | `:37` ✓ |

No drift, so no re-anchoring was required and nothing was edited at a shifted line.

---

## 3. C-6 — the vault write path now fsyncs its parent directory

**Done first, as directed: unconditional, unflagged, no compat surface.**

```rust
    fs::rename(&tmp, path).map_err(|_| "vault_write_failed")?;
    crate::fsync_dir_best_effort(parent);
    Ok(())
```

Byte-parallel to the pre-existing call in `vault_init_core`, shown side by side:

```
--- vault_init_core (pre-existing, the parity reference) ---
        fs::rename(&tmp, &vault_path).map_err(|_| ())?;
        crate::fsync_dir_best_effort(parent);
        Ok(())
--- write_vault_atomic (NA-0669 C-6) ---
    fs::rename(&tmp, path).map_err(|_| "vault_write_failed")?;
    crate::fsync_dir_best_effort(parent);
    Ok(())
```

`parent` was already bound at `:847`. `crate::fsync_dir_best_effort` is already re-exported (`lib.rs:106`) and is a `#[cfg(unix)]` `File::open(dir).sync_all()` with a `#[cfg(not(unix))]` no-op (`fs_store/mod.rs:359-364`) — **best-effort by construction, so it introduces no new error path** and no new failure mode for a caller to handle.

**Why one line matters here.** `sync_all()` on the temp file persists its *contents*; it says nothing about the *directory entry* the rename creates. Losing that entry loses the write while the data it points at is durable. `write_vault_atomic` has **three callers**, verified rather than assumed:

```
vault/mod.rs:240   secret_set
vault/mod.rs:273   secret_set_with_passphrase
vault/mod.rs:385   persist_session
```

That is every steady-state vault mutation in the product — contacts, ratchet state, timeline, and identity secrets after init. It was also the **only** rename-then-nothing site in the crate; the other durability-relevant paths (`vault_init_core:587`, `protection.rs:281/:430/:449`, `fs_store/mod.rs:123`, `lib.rs:2311`) all already fsync.

**⚠ EVIDENCE BOUNDARY, STATED PLAINLY.** No test in the suite can observe this fix. Directory-entry durability is only visible across a power failure, and the suite does not cut power. **The suite green proves C-6 did not regress anything; it does not prove C-6 works.** What supports the fix is the parity argument above — the same call, after the same rename, in the same crate, against the same failure mode — not the green check.

---

## 4. C-1a — the verification code recovers 20 bits

### 4.1 The change

`format_verification_code_from_fingerprint` filtered its input to alphanumerics, which **drops the hyphen of `QSCFP-` but keeps its five letters**. Five of the sixteen body characters were therefore constant, and every verification code the product has ever displayed began `QSCF-P`. The fix strips the prefix before the filter. **Target width stays 16. Grouping stays `4-4-4-4-checksum`. The Crockford alphabet and the `% 32` fold are untouched** — the checksum input changes only because the body changes, which is intended.

A fingerprint that does not carry the prefix is left alone (`strip_prefix(…).unwrap_or(fingerprint)`). The function is `pub` and reachable from qsl-desktop, so it must not assume its input shape even though both in-crate call sites guard with `starts_with`. The match is case-sensitive, which is **exact parity with those two guards** (`:554`/`:570` pre-change).

### 4.2 Measured, from the real code — five real identities

Five identities created with the real built binary (`qsc vault init` + `qsc identity rotate --confirm`), then their real fingerprints run through **the real production function** — the `qsc` library linked as an external crate, built once from the base tree and once from the fixed tree. Not a model:

```
FINGERPRINT (real)                        BEFORE                   AFTER
QSCFP-62da0931a8fa6bff1f766b1108896fc6    QSCF-P62D-A093-1A8F-P    62DA-0931-A8FA-6BFF-Y
QSCFP-8fa9e770a177f30e65a8f28443a12800    QSCF-P8FA-9E77-0A17-1    8FA9-E770-A177-F30E-9
QSCFP-6106dbe02fa997c392c3abd7ffa08d51    QSCF-P610-6DBE-02FA-Y    6106-DBE0-2FA9-97C3-0
QSCFP-0a9e07d057c27039beb2b269a98e4d1c    QSCF-P0A9-E07D-057C-P    0A9E-07D0-57C2-7039-Y
QSCFP-8253cc956d42273fa8e271c2a0f24605    QSCF-P825-3CC9-56D4-H    8253-CC95-6D42-273F-8
```

The constant `QSCF-P` is gone. This matches the directive's predicted §3 shape exactly.

### 4.3 The 44 → 64 claim, measured through the real function at 4000 samples

The directive's §1a table was produced against a *model* of the derivation. It is reproduced here through **the real production function**, over 4000 fingerprints in the exact real shape (`QSCFP-` + 32 hex, i.e. `hex(sha512(kem||sig)[..16])`):

```
BEFORE  n=4000 width=21  constant display positions [0,1,2,3,4,5,9,14,19] -> "QSCF-P---"
        varying hex in body = 11  ->  44 bits
AFTER   n=4000 width=21  constant display positions [4,9,14,19]           -> "----"
        varying hex in body = 16  ->  64 bits
```

**44 → 64 bits**, at unchanged width and zero UX cost. Identical to the directive's modeled table, position for position.

**⚠ 64 bits at width 16 — NOT the 88/22 the intent stated.** 16 hex characters is 64 bits; 88 bits needs 22 characters, which needs the target-width change this lane explicitly excludes, and 22 is not divisible by the four-character display grouping. The directive corrected this before execution and the operator confirmed the correction; it is restated here so the number in the record is the right one.

**⚠ One arithmetic correction to the directive itself, recorded rather than quietly fixed.** §1a describes the result as *"a 20-fold reduction in collision probability."* It is a **20-bit** reduction — a factor of **2²⁰ ≈ 1,048,576**. The bit figures (44, 64) and every ruling resting on them are correct and unaffected; only that one descriptive phrase understates the gain by five orders of magnitude. The as-built uses the correct figure.

### 4.4 ⚠ The shadow copy, updated in lockstep

`tests/identity_binding.rs:37` holds a **byte-identical private reimplementation** of the production function. The test computes its own expected code and feeds it to the CLI, so changing only `src/` makes the test fail — correctly, by detecting the divergence.

Both were updated. The duplicate was **kept as a duplicate and deliberately not replaced with an import**, with a comment now recording why: its separateness is exactly what makes the test a real check that the CLI and the formula agree, rather than a tautology that would pass no matter what either side did.

**No assertion was weakened, and none needed its expected value edited.** This is worth stating precisely, because the directive anticipated "a pinned literal" and the reality is stronger: the test's expected value is **computed, not pinned**, so updating the formula *is* the value update. There was no literal to change and no assertion to touch. `handshake_accepts_verification_code_pin_without_peer_mismatch` (`:413`) — which runs `contacts add --fp <code>` with no keys, completes a full handshake, and asserts `!combined.contains("peer_mismatch")` — still passes unchanged under F1(a).

### 4.4a ⚠ THE SECOND SHADOW COPY — caught by the suite, not by the census (root-cause finding)

There were **three** implementations of the formatter, not the two the directive accounted for. The third — a **second test shadow copy** — is why the first full run went RED (§1.1).

| # | Location | Name | Fixed by |
|---|---|---|---|
| 1 | `src/identity/mod.rs:527` | `format_verification_code_from_fingerprint` (production) | the C-1a change |
| 2 | `tests/identity_binding.rs:37` | `format_verification_code_from_fingerprint` (shadow #1, directive-named) | lockstep, §4.4 |
| 3 | `tests/identity_foundation_contract_na0217d.rs:133` | `verification_code_from_fingerprint` (shadow #2, **unnamed**) | lockstep, under operator ruling (A) |

Shadow #2 drives `verification_code_pin_preserves_handshake_contract`: it pins a code computed by the stale local formatter, so after C-1a it pinned the old format while production produced the new one → `peer_mismatch` → the test's `assert!(!text.contains("handshake_reject"))` at `:294` fired. The fix is **identical to shadow #1** — strip the prefix before the filter, keep the duplicate (not an import, same §3a reasoning), **assertion unchanged**. This is "update the derivation," which the directive endorsed for shadow #1; it is not "weaken an assertion," which stays a STOP. §5a was extended by explicit operator ruling to permit exactly this one edit in this one file.

**⚠ ROOT-CAUSE FINDING, because it generalizes.** Shadow #2 is named `verification_code_from_fingerprint` — **without** the `format_` prefix that the production symbol and shadow #1 both carry. So every sweep keyed on the **symbol name** missed it: the directive's authoring grep, and this executor's Phase-0 caller-sweep (which keyed on `format_verification_code_from_fingerprint` and correctly found the three *callers* of that symbol, but not a differently-named fourth *definition*). Only a sweep for the **function body** — the Crockford alphabet constant `0123456789ABCDEFGHJKMNPQRSTVWXYZ` — finds all three. **The lesson: when hunting duplicate implementations, sweep for a distinctive body fragment or constant, not the symbol name — names diverge, bodies do not.** This is "sweep, don't trust the list" applied to *definitions* rather than *call sites*, and it is the same class as the NA-0668 costume set: an instrument pointed slightly off the question it was asked. It is recorded as a finding here and in the journal; **no new ledger entry is filed** (scope discipline holds).

**⚠ CROSS-REPO, REPORT-ONLY (no fourth copy). qsl-desktop was checked against mirror main `02cc9b9` using the body sweep, and it holds NONE of its own** — the Crockford constant appears nowhere in the desktop tree. Desktop calls the qsc production symbol directly (`src-tauri/src/commands.rs:82` renders the code; `src-tauri/tests/slice_a_flows.rs:50` tests it and asserts only `!code.is_empty()`, pinning no format literal). So there is no fourth instance, and desktop CI will not break on the pin bump — confirming §5b by direct inspection. The already-known §5b staleness (`DESIGN_SPEC_AppendixD.md:100`/`:221`, `ui/style.css:280`) still lands whenever desktop next bumps its qsc pin; that is the desktop pin-bump lane's concern, not this one.

### 4.5 The other test anchors: checked, not assumed

- **`tests/identity_ux.rs:141-142`** — flagged in the directive as short 16-hex pins that currently pad. Anchors exact. **Unaffected:** the pins are 16 hex characters *after* the prefix, so they neither pad before nor after, and the test's assertions (`:150-156`) only check `peer=alice`/`peer=bob` ordering and `identity_fp=QSCFP-` at `:128` — the **fingerprint**, never the code. No change needed.
- **`tests/NA_0634_full_identity_provisioning.rs:89-95`** — mirrors the fingerprint, not the code, and its `:90` comment says so explicitly. **Confirmed unaffected**, as the directive asked rather than assumed.
- **`tests/NA_0649_gui_surface.rs:311-312`** — **a third call site the directive did not name.** Found by sweeping all callers rather than working from the directive's list. It calls the `pub` function directly and asserts only **shape and determinism** (five groups, four of length four, one checksum character, alphanumeric-or-hyphen) — every one of which survives C-1a unchanged. **Unaffected.** Recorded because the directive's file list was, on this point, incomplete, and an executor working only from it would not have looked.

### 4.6 What was NOT touched: `identity_pin_matches_seen:562`

**F1 = (a), ruled by the operator before execution. The branch is untouched, and it is a forbidden path in this lane.** Verified untouched: the only changes to `identity/mod.rs` are inside `format_verification_code_from_fingerprint`.

The reasoning is on the record and is not re-litigated here. In short: verification codes **are** stored as pins today (`contacts/mod.rs:908`/`:918` store the operator-supplied `--fp` verbatim; the `(None, None)` arm at `:886` performs no key check at all), the behavior is covered by a named test, and deleting the branch is a product decision rather than a one-line security fix. C-1a alone takes that comparison from 44 to 64 bits, which is the win available without a product decision. Option (b) — an observability marker — was declined explicitly: one new marker is not worth a vocabulary change in a lane scoped to three fixes.

**Recorded because it cuts the other way and will matter to the successor:** `src/adversarial/binding_fuzz.rs:396 trusted_pin_matches_seen` is `pinned.eq_ignore_ascii_case(seen_fp)` and nothing else — no code branch — and under `qsc_binding_fuzz_helper` the production result is computed into `_canonical_pin_matches` and **discarded** (`handshake/mod.rs:990`, `:1046`, `:1093`) in favor of that strict oracle. The strict semantics are therefore already the modeled and attested ones, and the production leniency is a **known, unasserted divergence from the model**. The oracle was not edited (a STOP condition); it is cited as evidence, not as a target.

### 4.7 ⚠ The accepted upgrade break

**Every existing code-pinned contact will yield `peer_mismatch` on every subsequent handshake.** A contact pinned by code holds the old-format string on disk; the code recomputed from the peer's fingerprint is now the new format, and they will never compare equal.

This follows from changing the **format**, not from changing the matcher, so it happens with `:562` completely untouched. **Ruled accepted, no migration** — pre-release, zero contacts in the field. Recorded so the decision stands as a knowing one rather than a field discovery.

---

## 5. C-4 — the passphrase file no longer discards entropy

### 5.1 The change, and why reject rather than raw bytes

```rust
    let bytes = fs::read(path).map_err(|_| "vault_passphrase_file_read_failed")?;
    let mut passphrase =
        String::from_utf8(bytes).map_err(|_| "vault_passphrase_file_read_failed")?;
```

Existing error code, no new vocabulary, no signature change, one line.

**Raw-bytes-end-to-end is the better design and was rejected as uncontained**, per the directive's ruling: the `&str` runs the whole passphrase spine — `read_passphrase_file`, `unlock_with_passphrase_file:180`, `unlock_with_passphrase:187` (**pub**), `set_process_passphrase:1067` (**pub**), `clone_process_passphrase`, the `PROCESS_PASSPHRASE` global, `derive_runtime_key`, `load_vault_runtime_with_passphrase`, the init path, and two sites in `protection.rs` — roughly ten sites across two files including **two `pub` signature changes on the surface qsl-desktop links against.** That is a passphrase-pipeline refactor, and it belongs with the vault-format lane where the pipeline is already being opened.

**Reject is a genuine fix, not a dodge.** The defect is that the operator *believes* they have 256 bits and silently has ~144. Rejecting converts a silent degradation into a loud, diagnosable failure at the moment of use, and removes the two-ingress asymmetry that was the tell — `read_passphrase_from_stdin:1122` already errors on invalid UTF-8 via `read_to_string`, so file and stdin now behave identically.

### 5.2 The entropy loss, measured through the real `from_utf8_lossy`

Measured by enumerating the **complete** single-byte input space through Rust's real `String::from_utf8_lossy` — the exact function the pre-fix code called — not by reasoning about it:

```
single random byte through from_utf8_lossy:
  distinct outcomes = 129 (of 256 inputs)
  Shannon H         = 4.500 bits of 8
  destroyed         = 3.500 bits/byte (43.8%)
  32-byte file (independent-byte extrapolation) = 144.0 bits of 256
```

This confirms the directive's figures exactly (129 outcomes, H = 4.500, ≈144 bits) and the audit's "145–160" range. The mechanism: the 128 bytes `0x00-0x7F` map to themselves, and **all 128 bytes `0x80-0xFF` collapse to the single character U+FFFD.**

Corroborated on whole files, 200,000 random 32-byte trials:

```
  valid UTF-8 (would survive untouched) = 0 (0.000000%)
  mean U+FFFD replacements per file     = 13.35
```

**Not one** random 32-byte file in 200,000 was valid UTF-8. The lossy path fired on every single one.

**⚠ The 144-bit figure is an independent-byte extrapolation (32 × 4.5), stated as such.** The true value is slightly higher, because adjacent bytes can form valid multi-byte sequences that survive. It is nowhere near 256, which is the point, and the direction of the approximation is recorded rather than glossed.

### 5.3 Observed CLI behavior, with the positive control

The same `head -c 32 /dev/urandom > raw.txt` file (confirmed invalid UTF-8), run against binaries built from the base tree and the fixed tree:

```
1. BEFORE: raw 32-byte /dev/urandom file — SILENTLY ACCEPTED
   QSC_MARK/1 event=vault_init path=<redacted>
     -> exit=0                              <- a ~144-bit vault the operator believes is 256-bit

2. AFTER: same raw file — REJECTED
   QSC_MARK/1 event=error code=vault_passphrase_file_read_failed
     -> exit=1

3. MIGRATION HAZARD (F2, accepted): the vault built at step 1, opened under AFTER
   QSC_MARK/1 event=error code=vault_passphrase_file_read_failed
     -> exit=1

4. AFTER: base64-encoded — the SAME 32 bytes — round-trips
   vault init      -> exit=0
   identity rotate -> event=identity_rotate ok=true
   identity show   -> event=identity_show ok=true
```

**Step 1 is the positive control.** The instrument demonstrably returns "accepted" against the pre-fix binary, so step 2's rejection is a measured behavior change and not an artifact of a broken rig.

Byte-for-byte preservation into Argon2id, on the same 32 bytes:

```
raw.txt                      : 32 bytes, 256 bits on disk
from_utf8_lossy(raw)         : 64 bytes reaching Argon2id BEFORE — NOT the file's bytes
raw survives verbatim?         False
b64.txt                      : 44 bytes
String::from_utf8(b64)       : 44 bytes reaching Argon2id AFTER
b64 survives verbatim?         True
b64 decodes to the original 32 bytes?  True
=> all 256 bits reach Argon2id, unmodified
```

Note the pre-fix path handed Argon2id **64 bytes** for a 32-byte file — each U+FFFD is three bytes in UTF-8 — so the derived key was not merely weaker, it was computed over a longer string that carried less information. `derive_key` already takes `pass_bytes: &mut [u8]` straight into `hash_password_into`, so nothing downstream needed to change.

### 5.4 ⚠ The accepted migration hazard

**Every existing vault created from a non-UTF-8 passphrase file is now permanently unopenable.** The lossy transform previously ran at *both* init and unlock, so such vaults were self-consistent and openable; both candidate fixes break that self-consistency.

**F2 ruled accepted, no migration** — pre-release, no recovery path. Demonstrated live at step 3 above rather than asserted. Reject is the kinder of the two candidate fixes: it fails with the named `vault_passphrase_file_read_failed` at the moment of use, whereas raw-bytes would have failed as an indistinguishable wrong key (`vault_locked`). This hits precisely the `/dev/urandom` operator who was trying hardest to be secure, which is why it is recorded here in full rather than buried.

### 5.5 The trailing-newline stripper: reported, not fixed

`vault/mod.rs` strips trailing terminators in a `while` loop, so a passphrase legitimately ending in newlines is ambiguous. **Ruled not trivial and deliberately left alone:** the same loop runs at init and at unlock, so it is self-consistent today, and changing it to strip a single terminator would silently change the derived key for any existing vault whose passphrase file ended in more than one newline. `read_passphrase_from_stdin` carries the identical loop, so any fix must move both together. It belongs with the vault-format lane.

---

## 6. Acceptance

| § | Item | Result |
|---|---|---|
| 7.1 | C-1a, real derivation, five random identities, before/after from built code, prefix absent, **16** hex varying where 11 did, 44 → 64 bits | **PASS** — §4.2, §4.3 |
| 7.2 | **Both** shadow copies updated in lockstep, assertions unchanged in strength; `handshake_accepts_verification_code_pin_without_peer_mismatch` and `verification_code_pin_preserves_handshake_contract` both pass | **PASS** — §4.4, §4.4a (shadow #2 caught by the suite, then fixed) |
| 7.3 | C-6 call present after the rename, shown side by side with the `vault_init_core` parity reference, evidence boundary stated | **PASS** — §3 |
| 7.4 | C-4 random-bytes file rejects with `vault_passphrase_file_read_failed`; base64 32-byte file round-trips with entropy preserved byte-for-byte; before-figure recorded | **PASS** — §5.2, §5.3 |
| 7.5 | Both full suites green **to completion**, stated as evidence, contrasted with the last four lanes | **PASS** — clean full run, `CARGO_EXIT=0`, 434 tests, 0 failed (§7). The first run went RED (§1.1) and caught shadow #2; after the ruling-(A) fix, this run completed clean. |
| 7.6 | No unrelated file touched | **PASS** — four files, all in §5a as extended by ruling (A) |

**STOP conditions:**

| Condition | Status |
|---|---|
| Any file outside §5a needs to change | **TRIGGERED and correctly stopped** — the first run showed `tests/identity_foundation_contract_na0217d.rs` (shadow #2) needed the lockstep edit; the executor STOPPED and requested a ruling rather than editing it. Operator ruling (A) then extended §5a to that one file for that one edit. No file was touched outside authorization. |
| F1 or F2 unruled at execution time | Not triggered — both ruled before seat |
| `identity_binding.rs`/`na0217d.rs` assertions need **weakening** | Not triggered — no assertion touched in either; both expected values are computed, not pinned, so the formula update *is* the value update |
| `binding_fuzz.rs` trusted oracle needs editing | Not triggered — cited as evidence only, byte-unchanged |
| `Cargo.toml`/`Cargo.lock` move, or a new dependency appears | Not triggered — verified untouched, including after building an external consumer against the tree |
| Suites red for a reason **not** traceable to the three intended changes | Not triggered — the one red was **fully traceable** to C-1a (a stale shadow of the changed formatter); it was not an unrelated failure |
| `docs_only=true` on the implementation PR | **Not triggered — `docs_only=false`, `runtime_critical=true`** (§1) |

**rustfmt:** `cargo fmt --check` reports pre-existing drift across 42 files in this crate — the known owed micro-lane, present identically on the base. Verified by diffing the full `cargo fmt --check` output before and after the change: **identical apart from two line-number shifts caused by this lane's own insertion.** `src/identity/mod.rs` appears in neither list. **This lane introduces zero new drift and repairs none**, which is out of scope.

---

## 7. Suite results

**Full local `cargo test -p qsc` — CLEAN GREEN TO COMPLETION.** Run to the end, no fail-fast abort, cargo's **real** exit code captured (not a pipe's):

```
CARGO_EXIT=0
110 result lines (integration binaries + lib unit tests + doc-tests)
434 tests passed, 0 failed, 0 ignored-as-failure
0 "test result: FAILED" · 0 panics · 0 compile errors
```

The two verification-code-pin workflow tests — the direct proof of the C-1a lockstep across both shadow copies — are green in this same run:

```
tests/identity_binding.rs
  handshake_accepts_verification_code_pin_without_peer_mismatch ... ok      (binary: 3 passed, 0 failed)
tests/identity_foundation_contract_na0217d.rs
  verification_code_pin_preserves_handshake_contract           ... ok      (binary: 2 passed, 0 failed)
```

**This green is evidence, and it is a stronger evidence story than a first-try green would have been (§1, §1.1).** The instrument (both full suites, runtime-critical scope) is live, it was pointed at the change, and it was **demonstrated to reject a real defect** — shadow #2 — on the first run before it was trusted to accept on this one. Contrast the four immediately preceding lanes (NA-0664/0667/0668), whose `docs_only=true` diffs SKIPPED both suites and whose greens carried no information about the work.

The slowest binaries are the debug-build file-transfer and e2e suites (top: ~1277 s / ~21 min for `aws_file_medium_boundary_na0192a`, ~964 s, ~445 s), all dominated by debug-build Argon2id across many spawned `qsc` subprocesses — inherent to those tests, not this lane. Total wall time ≈ 1 h 40 m.

**Boundary restated (§3):** this green proves C-1a and C-4 behaviorally, and proves **non-regression** for C-6; it does not prove C-6's durability, which no non-power-cut test can.

---

## 8. What this lane did NOT do

Recorded so the successor does not have to re-derive the boundary:

- **`identity_pin_matches_seen:562`** — untouched (F1(a)). The delete/gate product decision rides the vault-format lane.
- **C-1b and the target-width decision (20/24)** — explicitly excluded; the same successor lane owns them.
- **The C-4 raw-bytes form** — the better design, deferred as uncontained (§5.1).
- **The trailing-newline stripper** — reported, ruled not trivial (§5.5).
- **C-2 (constant-time bearer compare)** — a separate `qsl-server` lane. Not touched.
- **qsl-desktop** — not touched. C-1a changes what the GUI displays, **but only when desktop bumps its qsc pin**, which is a separate deliberate lane. Two desktop artifacts go stale at that moment: `docs/DESIGN_SPEC_AppendixD.md:100`/`:221` (the mockup literal `QSCF-P26A-0C0B-3B40-4`) and `ui/style.css:280` (a comment asserting the format). **No desktop test pins the format, so desktop CI will not break**, and no spine doc pins it either. Reported; nothing filed.
- **Nothing new was filed in `IMPROVEMENT_LEDGER.md`.** Scope discipline was the lane's second deliverable: four of the last five lanes were infrastructure, and this one ships code, fixes exactly three defects, and files nothing. Observations went to the operator relay.

**The successor is the vault-format lane**, which now owns C-1b, the target-width decision, the `identity_pin_matches_seen` product ruling, the C-4 raw-bytes form, and the newline stripper — a coherent set, worth sizing sooner than planned.
