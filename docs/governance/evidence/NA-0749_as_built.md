# NA-0749 — AS BUILT: PHASE 1 LANE 2, THE FINGERPRINT FORMAT REPAIR (`ENG-0205`) — ACT 0, THE PROMOTION

**Lane:** NA-0749 · **Decision:** D-1390 · **Date:** 2026-08-19
**Base:** qsl-protocol main `e917e7e8a761b22034eabeb760a10c38b1c0fe30`, re-derived **bare and unpiped**
against the named `github` remote on a clone made for this lane.
**Directive:** `BRIEF_NA0749_fingerprint_format_repair_2026-08-19.md`, sha256
`b35592acd87cbf9536544e6d65116b371f251ccdadae003cff7193e38f2f3446`, 145 lines / 10806 bytes, mode 444,
banked verbatim under SR-14 as the lane's **FIRST ACT**, before anything consumed it.
**Scope of this act:** RECORDS ONLY. Zero product source bytes; the fingerprint format is **not moved**
by this PR. qsl-desktop is out of scope entirely.

---

## §1. IDS — DERIVED AT THE EDIT (WF-0068), SWEPT **BEFORE** THE BRIEF WAS BANKED

| space | declaring form | max at base | candidate | declaring | mentions | positive control | negative control | verdict |
|---|---|---|---|---|---|---|---|---|
| `NA-` | `^### NA-####` in `NEXT_ACTIONS.md` | `0748` | `NA-0749` | **0** | 2 | `NA-0748` = 1 | `NA-0750` = 0 | **FREE** |
| `D-` | union of **all four** forms | `1389` | `D-1390` | **0** | 7 | `D-1389` = 1 | `D-1391` = 0 decl | **FREE** |
| `D-` | union of all four forms | `1389` | `D-1391` | **0** | 4 | `D-1389` = 1 | — | **FREE, not minted** |
| `ENG-` | `^### ENG-####` in the ledger | `0207` | `ENG-0208` | **0** | 3 | `ENG-0207` = 1 | — | **FREE** |
| SR-16 rows | `^\| N \|` in `PREDICTION_LEDGER.md` | `112` | rows `113`–`116` | — | — | row 112 = 1 | row 113 = 0 | **FREE** |
| `R` | filename ∪ content | `360` | `R361` | — | 0 | `R360` = 1 file | — | **FREE, not minted** |
| `WF-` | — | `0087` live | `WF-0088` | **0** | 7 | — | — | **FREE, not minted** |
| `SR-` | — | `24` live | `SR-25` | **0** | 12 | — | — | **FREE, not minted** |

⚠⚠ **THE PLANT HAZARD (WF-0087) FIRED ON EVERY SINGLE CANDIDATE.** `NA-0749`, `D-1390`, `D-1391` and
`ENG-0208` all return a **non-zero tree-wide mention count at a base where all four are free** — the hits
sitting at `DECISIONS.md:43604`, `docs/ops/PREDICTION_LEDGER.md:432`/`:434` and
`docs/governance/evidence/NA-0748_as_built.md:62-78`, **where the predecessor recorded them as its own
negative controls.** A mention-counting sweep would have refused all four, each with a plausible reason
attached. ⇒ the sweep was run **before** the brief was banked, every hit was enumerated by file and path
class, and the maximum was taken **only over declaring forms**.

**Open-PR set: EMPTY**, measured with `gh pr list --state open` against a **positive control** of 4
merged rows (#1776/#1775/#1774/#1773) proving the query returns rows when rows exist. #1776's
`mergeCommit.oid` independently corroborates the base sha.

---

## §2. PREMISES THE SEAT OWNS (§2 of the brief) — MEASURED AT THIS BASE

### (a) main UNMOVED — **HIT**
`e917e7e8a761b22034eabeb760a10c38b1c0fe30`, matching the brief's stated base exactly. Open-PR set empty
with a positive control. Ids re-derived (§1).

### (d) THE CURRENT CONSTRUCTOR'S EXACT BYTES — **MEASURED, AND THE BRIEF'S SINGULAR IS AN UNDERCOUNT**

⚠ **The brief's §2(d) says "the current constructor" — singular. There are TWO in `identity/mod.rs`, and
a THIRD construction of the same shape in `handshake/mod.rs`.**

- `identity/mod.rs:125` — `pub(super) fn identity_fingerprint_from_pk(pk: &[u8]) -> String`
  ⇒ `IDENTITY_FP_PREFIX ‖ hex(sha512(pk)[..16])`
- `identity/mod.rs:137` — `pub fn identity_fingerprint_from_identity(kem_pk, sig_pk) -> String`
  ⇒ `IDENTITY_FP_PREFIX ‖ hex(sha512(kem_pk ‖ sig_pk)[..16])`
- `handshake/mod.rs:933` — `fn hs_sig_fingerprint(sig_pk: &[u8]) -> String`
  ⇒ `IDENTITY_FP_PREFIX ‖ hex(sha512(sig_pk)[..16])` — **byte-identical in construction to
  `identity_fingerprint_from_pk`**, in a different module.

`IDENTITY_FP_PREFIX = "QSCFP-"` (`identity/mod.rs:35`). Truncation is **the first 16 bytes** of a
sha512 digest (128-bit), hex-encoded. **There is no domain string anywhere in any of the three.**

### (b) THE CONSUMER CENSUS — measured; dispositions are STOP 2's deliverable

Live-code (non-`.md`) occurrence counts, `git grep` (never `grep -r`: the wrapper honours `.gitignore`
and is blind to 483 tracked files under `docs/governance/evidence/`):

| token | live lines | live files |
|---|---|---|
| `QSCFP` | 16 | 10 |
| `identity_fingerprint_from_pk` | 7 | 4 |
| `identity_fingerprint_from_identity` | 15 | 7 |
| `format_verification_code_from_fingerprint` | 10 | 4 |
| `identity_marker_display` | 8 | 3 |
| `identity_pin_matches_seen` | 12 | 5 |
| `CROCKFORD` | 6 | 3 |
| `IDENTITY_FP_PREFIX` | 9 | 3 |

**The desktop-reachable (`pub`) surface is exactly two functions** — `identity_fingerprint_from_identity`
and `format_verification_code_from_fingerprint`. The second is the one C5 retires, which is precisely
why the desktop consumes this at pin-bump-2 and not here.

⚠ **`formal/model_qsc_handshake_authentication_bounded.py` is a consumer OUTSIDE §1's enumeration.** It
names both identity constructors and `hs_sig_fingerprint`, and it is executed by the **REQUIRED**
`formal-scka-model` job. It models them abstractly as injective structured tokens, so it may need no
edit — but that is a STOP-2 disposition, not a self-authorized one.

### (c) THE PINNING SEMANTICS — **MEASURED, NOT ASSUMED**

`identity_pin_matches_seen(pinned, seen_fp)` (`identity/mod.rs:652`) accepts **either** form: it compares
the pin case-insensitively against the fingerprint, and — **only if `seen_fp` starts with `QSCFP-`** —
against the derived verification code. Pins persist **inside the vault**, as a `serde_json` blob under
`CONTACTS_SECRET_KEY` via `vault::secret_set` (`contacts/mod.rs:367`/`:389`), not as a plaintext file.

⛳ **What a stored old-format pin does against a new-format computation: it FAILS CLOSED.** Under the new
format there is no `QSCFP-` prefix, so the second branch is never taken and the comparison returns
`false` — a refusal, not a silent accept. That is the correct failure mode and is sealable.

⚠ **§2(c)'s STOP condition, answered precisely:** **no pin persists in the repository outside test
fixtures and historical records** (§2(f)). Pins do persist **at runtime**, in the user's vault-backed
contacts store — which is outside "test fixtures" in the literal sense while being exactly the
pre-release "regenerate, no migration" case the brief anticipates. **Surfaced for the Director rather
than resolved by this seat.**

### (e) THE KEY SIZES — **PROVEN FIXED, by two independent routes**

- `tools/refimpl/quantumshield_refimpl/src/qsp/constants.rs:12` — `SZ_MLKEM768_PUB = 1184`
- `tools/refimpl/quantumshield_refimpl/src/qsp/constants.rs:15` — `SZ_MLDSA65_PUB  = 1952`
- Independently, `qsl/qsl-client/qsc/fuzz/fuzz_targets/qsc_binding_semantics.rs:56,58` —
  `KEM_PUBLIC_KEY_LEN = 1184`, `SIG_PUBLIC_KEY_LEN = 1952`.

The handshake reaches them through `hs_kem_pk_len()` → `runtime_pq_kem_public_key_bytes()` →
`<MlKem768Ek as EncodedSizeUser>::EncodedSize::USIZE`, and `hs_sig_pk_len()` →
`runtime_pq_sig_public_key_bytes()` → `EncodedVerifyingKey::<MlDsa65>::default().len()`. **Despite the
`runtime_` prefix these are type-level constants for a single hard-coded parameter set** — fixed per
build, not negotiated. ⇒ **§2(e) measures TRUE**, and C2's no-separator argument stands **on that
dependency, which is now stated so it can be challenged**.

### (f) GOLDEN FILES / FIXTURES PINNING OLD-FORMAT VALUES — **NONE THAT ARE REAL**

**17 literal `QSCFP-<hex>` values tree-wide, 11 distinct.** Every live-code occurrence is **synthetic**:
all-zeros and all-`f`s in `src/adversarial/binding_fuzz.rs:519,521`; `QSCFP-bbbb…`/`QSCFP-aaaa…` in
`tests/identity_ux.rs:141,142`; an all-zeros `WRONG_SIG_FP` in
`tests/kem_signature_transcript_binding_negative.rs:16`; and a probe payload
`identity_fp=QSCFP-na0700-payload-probe-0123456789` in `src/output/mod.rs:441`. The remaining literals
are in historical records (`NA-0669`, `NA-0633`). ⇒ **no golden pins a real computed fingerprint**, so
no golden regeneration is forced by the format change.

---

## §3. ⚠ TWO FINDINGS THE DESIGN MUST ANSWER — raised now, not adapted around silently

**F-A — C1/C2 SPECIFY THE COMBINED FINGERPRINT AND LEAVE THE SINGLE-KEY PATH UNDETERMINED.** C1 fixes
`sha512(domain ‖ kem_pk ‖ sig_pk)`. But `identity_fingerprint_from_pk` takes **one** key and is live at
three call sites, in **two different semantic roles**: `contacts/mod.rs:852` and `:928` pass **`sig_pk`**
and store the result as `sig_fp`, while `contacts/mod.rs:936` passes **`kem_pk`** for the KEM-only legacy
path (`contacts_kem_pk_fp_mismatch`). `hs_sig_fingerprint` computes the same construction again in the
handshake module. **The brief's C1–C8 say nothing about any of them.** Under §4's own rule — *an
impossible or underdetermined constraint is a finding, never a silent adaptation* — this is FILED, and
STOP 2 will carry a proposal rather than an improvisation.

**F-B — THE FORMAL MODEL ASSERTS A DOMAIN SEPARATION THE CODE DOES NOT IMPLEMENT.**
`formal/model_qsc_handshake_authentication_bounded.py:20-22,91,96` describes `CODE1(kem_id)` and
`SIGFP(sig_id)` as having **"distinct domain"**. Measured, both are computed by the *same*
un-domain-separated construction — `hex(sha512(x)[..16])` — over inputs that differ only in **content and
length**. Today `1184 ≠ 1952` keeps them apart **by accident of parameter choice, not by construction**.
⇒ the model is **stronger than the implementation**, and C2's versioned domain string is the first thing
that would make the model's own assumption true — **if** the design extends it to the single-key path
(F-A). A versioned domain that separates versions but not *roles* leaves this gap exactly where it is.

---

## §4. WHAT THIS ACT LANDED

1. **NA-0748 `READY (D-1389)` → `DONE`**, class **`PIN_BUMP_VAULT_FP_INVARIANT_PASS`** — text
   **extracted programmatically from NA-0748 STOP 004 §6's own bytes**, three documented placeholders
   filled, round-trip diffed. The clearance citation it carries was **re-verified**, not inherited:
   sha256 `debb980e…c0aa`, 46 lines / 3369 bytes, mode 444, and its §5 does declare that class.
2. **`ENG-0207` CLOSED**, beside the entry, mark-don't-rewrite — likewise extracted, not recomposed.
3. **`ENG-0208` FILED** (`F-11`, re-measured and **repo-scoped**; see §5).
4. **SR-16 rows 113–116** — `R360` §6's four.
5. **`D-1390`**, the STATE advance, the prior-STATE comment, TRACEABILITY, and NA-0749 born `READY`.

**Assembly proof:** all **8** landed blocks were extracted back out of the finished files and compared
against their sources — **8/8 contained**, each with a **negative control** (a tampered copy) proving the
comparison can fail. No block was typed twice.

---

## §5. ⚠⚠ A RE-MEASUREMENT THAT CHANGED A FINDING'S MEANING

`F-11` was carried forward as *"the stale `ci.yml` required-set comment"* with **no repository named**.
Re-measured at this base rather than inherited:

- **qsl-desktop** protection = `["rust","advisories","infra-literal-scan"]`, and its `ci.yml:73-79`
  claims three required contexts *including* `public-safety` and says `infra-literal-scan` *"is NOT in
  the required set yet"*. ⇒ **stale in both directions. F-11 is TRUE — of qsl-desktop.**
- **qsl-protocol** protection is a **different set of fifteen** in which **`public-safety` IS required**
  and neither `rust` nor `advisories` exists at all.

⇒ **a reader who took `F-11` to be about the repository it is FILED IN would have checked qsl-protocol,
found the claim false, and had no way to tell whether the finding or their reading was wrong.**
*A finding inherits the scope of the tree it was measured on, not the scope of the record that carries it
forward.* `ENG-0208` names the repository in its heading.

---

## §6. ⚠ ONE ITEM OWED AND DELIBERATELY NOT LANDED

NA-0748 STOP 004 §5.3 owes a **fifth** SR-16 row that is **its own, not `R360` §6's**: the `461`-vs-`518`
lock census — *a census whose dedup key is not the thing counted*, certifying "0 version-changed" on a key
that could not have seen one. The brief's §3 enumerates **four** rows by naming `R360` §6, whose own
sentence lists four; the fifth **post-dates** that ruling. `docs/ops/PREDICTION_LEDGER.md` is inside §1's
file set, so landing it would break no file bound — but **an authorization predating a finding does not
cover it**. Surfaced at STOP 1 for the Director with its text ready, rather than self-authorized.

## §7. ⚠ AN INHERITED "KNOWN-BAD GATE" THAT MEASURES **REPAIRED** — the seat's own premise, refuted

This seat carried forward, from its own standing notes, that `scripts/ci/preflight_governance.sh`
counts `Status:\s*READY` **unanchored** and therefore fails correct promotion trees. **Re-measured at
this base, that is FALSE.** The live needle is `scripts/ci/preflight_governance.sh:39` —
`rg -n '^Status: READY\b' NEXT_ACTIONS.md` — **anchored**, repaired at NA-0746 (`ENG-0201`,
`D-1387`) at **both** sites, the second being `scripts/ci/post_merge_verify.sh`. The gate reads this
tree correctly.

⚠⚠ **AND THE INHERITED FIGURES WERE STALE TOO, IN A WAY THAT IS ITS OWN LESSON.** The note said the
unanchored needle reads *"1 on every settled main, 2 on a promotion tree"*. Measured here it returns
**4 — at the unedited base AND on this promotion tree alike**, so this act does not move it. Enumerated,
the four are: `NEXT_ACTIONS.md:97`, the historical prior-STATE comment (the one the note remembered as
line 92 — **the line number had moved**); `:37370` **twice** and `:37372` once, all inside **NA-0746's
own block, which quotes the token while documenting and closing this very bug**; and `:37387`, the one
real declaration. ⇒ **three of the four false matches are the records that DESCRIBE the mention-counting
defect.** *Documenting a mention-counting bug is itself a mention* — the plant hazard, arriving in the
one place built to warn about it.

⇒ **A STANDING NOTE IS A PREMISE, NOT A MEASUREMENT.** It was right about the shape, wrong about the
state, wrong about the line number, and wrong about the count — and the only thing that caught it was
re-running the instrument instead of citing the note. The anchored truth on this tree is **1**, the
correct value: NA-0749's, and NA-0748's flipped to `DONE` in the same act.

---

# NA-0749 — AS BUILT, PART 2: THE IMPLEMENTATION (`D-1391`)

**Authorization:** `R363` §3, banked verbatim under SR-14 before acting
(`a22ae514ade83ff3290eb86b657391d2a1f4a2cc98eba15684306071063f4e40`, 64 lines / 4933 bytes, 444).
**Base:** main `f181c367151ac6dc3f85c277a2d8d77056fc0205`. **Built in the order `R363` §3 sets, and no
other.**

⚠ **Part 1 above is the ACT-0 record and is NOT edited.** Its §3 named a design (`F-A`/`F-B`) that
`R361` subsequently refused and `R362` replaced; those sentences stand as written, superseded here
rather than rewritten. The construction that landed is the one locked at `R362` §1.

## §I1. W1 — RED-FIRST, CAPTURED BEFORE THE IMPLEMENTATION LANDED

The prediction was written **before** the run: the old constructor must return
`QSCFP-` + `hex(sha512(kem‖sig)[..16])` = **`QSCFP-9069d8689203a5a1576fbc88a44a525e`**.

```
thread 'na0749_identity_fingerprint_matches_the_sealed_independent_vector' panicked at
qsl/qsl-client/qsc/tests/na0749_redfirst_armA.rs:8:5:
assertion `left == right` failed: the combined identity fingerprint does not match the sealed
independent vector
  left: "QSCFP-9069d8689203a5a1576fbc88a44a525e"
 right: "d67b4a10510394ca268c9e8cfde8980fd6280dc8c379d4ea8c8642ac9a750349"
test result: FAILED. 0 passed; 1 failed
```

⛳ **The left-hand value is byte-identical to the sealed prediction.** ⚠ **Stated precisely: this is an
ASSERTION red on the real old code, not a compile red.** The voice-form arm could not be run against
the old tree at all — `identity_voice_form` did not exist there — which is a *compile* red and a weaker
one; the distinction is recorded rather than blurred.

## §I2. THE SEALED VECTOR, ASSERTED BY THE IMPLEMENTATION

| | sealed value | asserted by |
|---|---|---|
| identity FULL | `d67b4a10510394ca268c9e8cfde8980fd6280dc8c379d4ea8c8642ac9a750349` | `tests/na0749_fingerprint_conformance.rs` |
| identity VOICE | `187363336018275058094178831816` | same |
| sig FULL | `c7251cb68ab0db6416e4ef3b3e9c372a6b63222587f22027ef12efbd75d58bab` | `identity/mod.rs` unit tests |
| kem FULL | `f5f23dadc0acee52fb4da4528d7bbe49aa5e7ecd77b9ea6962b2981040246d98` | same |

**All four HIT.** The Rust and the independent Python tool agree exactly — which is the whole point of
computing the vector outside the codebase first.

## §I3. W7 — THE BLOCKER-1 REGRESSION

`na0749_no_resplit_of_the_key_material_collides_with_the_true_pair` walks **every** split position
`k = 0..=3136`, excluding the true split, and asserts no re-split reproduces the true fingerprint.
**GREEN: 0 collisions of 3136 re-splits.** The identical property against the rejected construction
returns **3136/3136**, so the test can go red.

## §I4. W6 — THE COMPARATOR ARMS AND THEIR MUTATION CONTROLS

| arm | property | result |
|---|---|---|
| A | old-format `QSCFP-` pin refused | **HIT** |
| B/C | correct full and voice forms accepted | **HIT** |
| D | a wrong but well-formed 30-digit voice form refused | **HIT** |
| E | an identity voice form refused against a **sig** fingerprint | **HIT** |
| F | empty / short / non-hex / 64-char-non-hex `seen_fp` never authenticates | **HIT** |
| G | whitespace-padded **full AND voice** forms both accepted | **HIT** |
| H | the plain comparator refuses its own derivable voice form | **HIT** |

⛳ **EVERY MUTATION CONTROL FIRED — this, not the passes, is the seal:**

| mutant | change | arm expected red | measured |
|---|---|---|---|
| m1 | tier 2 → `pinned.len() == 30` | D | **RED** |
| m2 | shape-check removed; short `seen_fp` zero-pads | F | **RED** |
| m3 | `trim()` moved back off tier 2 (design v2 as first written) | G | **RED** |
| m4 | the plain comparator delegates to `_identity` | H | **RED** |

⚠⚠ **m3 reproduced MAJOR-B's asymmetry exactly**: the failing assertion is the **padded VOICE** case at
`identity/mod.rs:885`, while the padded FULL case on the line above **passed** — the precise divergence
the SR-15 re-read found on paper, now reproduced as an executable control. **Arms B, C and G are
accepting arms and prove nothing alone;** a comparator that accepts everything passes all three.

## §I5. W2 — THE RETIREMENT, PROVEN BY A **BODY** SWEEP

| needle | BASE `f181c367` | NOW |
|---|---|---|
| Crockford alphabet literal `0123456789ABCDEFGHJKMNPQRSTVWXYZ` | **3** | **0** |
| byte-sum-mod-32 idiom (`fold(0u32 … saturating_add`) | **3** | **0** |
| `IDENTITY_FP_PREFIX` | 9 | **0** |
| `format_verification_code_from_fingerprint` | 10 | **0** |
| `verification_code_from_fingerprint` (the shadow short name) | 12 | **0** |
| `identity_marker_display` | 8 | **0** |
| **positive control** `identity_fingerprint_from_identity` | 15 | **14** (non-zero) |

**THE THREE RESIDUALS, CLASSIFIED — none is a live mechanism:**
1. **`QSCFP` × 1**, `identity/mod.rs`, inside W6 arm A: the old-format pin used as the **refused input**.
   Removing it would delete the test proving old pins fail closed.
2. **`identity_fingerprint_from_pk` × 2 and `hs_sig_fingerprint` × 2**, all four in
   `formal/model_qsc_handshake_authentication_bounded.py` **docstrings** (`:21`, `:22`, `:91`, `:96`) —
   the file `R362` §3 keeps **outside the enumeration**. Comments naming renamed symbols; **no edit**,
   and the model needed none: it abstracts a fingerprint as a tuple, for which injectivity is free — a
   *false* abstraction of the old code and a *faithful* one of the new.
⚠ **Instrument note:** `identity_pin_matches_seen` reads 12 → 24 as a **substring** count, because the
new `_identity` name contains it. Exact: `identity_pin_matches_seen(` = **7**,
`identity_pin_matches_seen_identity(` = **14**.

## §I6. W3 — THE INVENTORY, KEYED `(file, name)`

Instrument (the corrected one, per `R362` §3 / MINOR-F): for each own-line `^\s*#\[test\]\s*$`, scan
**forward, unbounded**, to the next `fn`; key `(file, name)`.

| | BASE `f181c367` | NOW |
|---|---|---|
| entries | **679** | **695** |
| files | 148 | 150 |
| distinct names | 676 | 692 |

**REMOVED: 0. ADDED: 16** — ten unit tests in `identity/mod.rs` and six in
`tests/na0749_fingerprint_conformance.rs`, each enumerated in the stop. ⛳ **No test was retired**, as
the census predicted: the two "deliberate private reimplementation" tests protect *"a user can pin
using the second rendering they were shown"*, a property the ratified design **keeps** — only the
comparand changed, 16-character code → 30-digit voice form, updated in lockstep.
⚠ **An instrument trap caught in flight:** the first inventory run used `git ls-files` for the NOW side
and was blind to the **untracked** new test file, reporting +10 instead of +16. Staged first, then
recounted. *A census of tracked files cannot see the file you just wrote.*

## §I7. W3 — THE FULL SUITE ON THE EXACT COMMITTED TREE

`cargo test -p qsc`, run **bare** (the gating command's own exit status, never a pipe's), on the tree
carrying every compiled change.

**rc = 0 · 136 targets · 660 passed · 0 failed · 2 ignored · 0 `error` lines.**
All **16** of this lane's tests ran and passed.

⚠ **660 EXECUTED AGAINST 695 INVENTORIED — RECONCILED BY ENUMERATION, NOT ATTRIBUTED.** The 33
inventoried names that never executed are, by file: `rng_failure_residual_surfaces` (6),
`rng_failure_behavior` (4), `cli_identity_rotation_provider_rng_failure` (3), `kem_provider_rng_failure`
(3), `lazy_identity_provider_rng_failure` (3), `legacy_identity_public_record_provider_rng_failure` (3),
`na0695_vault_keychain_addressing` (3), `a2_signature_provider_rng_failure` (2),
`b1_signature_provider_rng_failure` (2), `na0696_vault_honesty` (2),
`na0742_invite_finish_scan_producer_acks` (1), `src/vault/mod.rs` (1).
**Two controls prove this is a base property and not this lane's doing:**
1. **ZERO of the 33 are in this lane's changed-file set.**
2. They sit behind `#[cfg(qsc_rng_failure_test_seam)]` and siblings, which the default build does not
   set — the fault-injection seam NA-0742 recorded as unbuilt by any workflow.
⇒ *a suite that runs 660 of 695 is reporting a cfg boundary, not a gap this act opened; the count is
stated with its cause rather than rounded to "green".*

## §I8. THE CLAIM BOUNDARY

- **The vector proves** the implementation agrees with an artefact computed **outside** this codebase,
  by a tool whose SHA-512 was validated against the published NIST value **before** any lane value
  existed, corroborated across four engines. It proves nothing about whether the *design* is the right
  design — two SR-15 reads and three rulings did that.
- **The suite proves** the crate's own behaviour on this tree, in the default cfg. It does **not**
  exercise the 33 `cfg`-gated fault-seam tests, nor the `cfg`-gated `binding_fuzz` shadow whose split
  landed here — that shadow's fidelity is checked by inspection plus CI's adversarial job, and that
  limit is stated rather than glossed.
- **Neither proves anything about qsl-desktop**, which links `qsc` and **meets this format only at
  pin-bump-2**. `format_verification_code_from_fingerprint` was published API; its retirement is a
  **published-API break**, marked as such and met at that pin bump.
- **What is NOT fixed:** the entry paths still accept keys of any length — the **class** behind
  BLOCKER-1's instance. Filed as `ENG-0209` with its trade.
