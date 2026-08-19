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
