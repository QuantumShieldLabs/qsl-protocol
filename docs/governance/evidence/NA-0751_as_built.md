# NA-0751 — AS BUILT: SLICE 4 PHASE 1, THE GATEWAY SPINE

**Lane:** NA-0751 · **Decision:** `D-1393` · **Date:** 2026-08-20
**Bases (re-derived bare and unpiped at the NAMED `github` remotes, measured UNMOVED at the edit):**
qsl-protocol `3293c39adbee76cb09e3d91b71495e9f58eb9e4f` · qsl-desktop `ee03edad27c9bc3214bb868e2612b76601b30d50`.
Open-PR set EMPTY in both, with a positive control that returned rows.

## 1. THE AUTHORITY CHAIN, EACH LINK BANKED BEFORE IT WAS CONSUMED

| artifact | sha256 | size |
|---|---|---|
| Director's formalization brief | `bec195bfaea2850c02c27c8530466ac1c415e404d8b84e4903c88c199d970bef` | 143 l / 11374 B |
| banked design (operator-blessed) | `f036af804b0ea5f28ec10a6f6fba1cce8c81c24713199146f8a382aecffebc0d` | 32 l / 2300 B |
| `R367` — seven asks ruled, v1 sealed | `2677764834dec5839ebe245e6508db46abd63bf284c871a8dcfdeb497977afa5` | 89 l / 7103 B |
| SR-15 read #1 findings | `507fec5a5060e0e881e4ddbc987a72835afec490490f0bb80d2ba60f988fc554` | 796 l / 59998 B |
| `R368` — v1 REFUSED, v2 specified | `a6a1d0294a53ddfbf977497311d40035a6917fe333810ad741b5666b70a56b6e` | 153 l / 12121 B |
| `R369` — six deltas ruled, v2 sealed | `7d7b1f22502be328cb342dc66044ef194acaad2dc32fbd006fa303ccf9cb3b7e` | 50 l / 3607 B |
| SR-15 re-read #2 findings | `7f61c182376538fb014f45697f6df3a47c9512d02d7b1ee1748ba301e5b60b8e` | 945 l / 72135 B |
| `R370` — v2 error layer REFUSED, v3 specified | `3ec3d63a4596763e8ce7784fdb7a2e4bc563cafa8a51bc93090b2f475957325a` | 125 l / 9928 B |
| SR-15 read #3 findings | `4604d6962fe8d7929b39c74e7e67e593824a8f222a0973d6c7c89ef3398fab14` | 659 l / 49606 B |
| `R371` — v3 REFUSED, v4 = retreat to tree conventions | `ac01f2f27e36bb1b9413511cb032d777b32f1e236b8c06438da2a22ba6238958` | 106 l / 8483 B |
| `R372` — v4 verified against the tree, BUILD authorized | `c388c35c954df2956229dcf84d689ab865f698bb56b5d1e420cc6a49e56fd0e3` | 41 l / 3395 B |

Every ruling was banked with the **id sweep run BEFORE the banking** (`WF-0087`), both controls per space,
and each placed by COMPARISON with a tamper control that returned non-zero.

## 2. THE EDIT SET, AND NOTHING OUTSIDE IT

    qsl/qsl-client/qsc/src/facade/mod.rs        NEW — the module
    qsl/qsl-client/qsc/src/lib.rs               the ONE `pub mod facade;` registration line
    qsl/qsl-client/qsc/tests/na0751_facade_connect_reason_totality.rs   NEW
    qsl/qsl-client/qsc/tests/na0751_facade_invite_surface.rs            NEW
    qsl/qsl-client/qsc/tests/na0751_facade_fact_fidelity.rs             NEW
    qsl/qsl-client/qsc/tests/na0751_facade_locked_control.rs            NEW
    scripts/ci/QSC_SHARD_MANIFEST.txt           +4 rows (shard 7)
    scripts/ci/QSC_SHARD_MANIFEST_MACOS.txt     +4 rows (shard 1)
    NEXT_ACTIONS.md · DECISIONS.md · TRACEABILITY.md
    docs/ops/IMPROVEMENT_LEDGER.md · docs/ops/PREDICTION_LEDGER.md
    docs/governance/evidence/NA-0751_as_built.md   (gitignored; `git add -f`)

**ZERO** bytes in `handshake/`, `transport/`, `identity/`, `contacts/`, `invite/`, `protocol_state/`,
`vault/`, `output/` or `model/` module bodies. **ZERO** visibility widenings. No `.github/**`, no `ui/**`,
no mockup, no dependency, no lock, no test weakened, skipped or deleted.

## 3. THE IN-CRATE REACHABILITY PREMISE — MEASURED TRUE END TO END

`lib.rs:112-142` declares `contacts`, `identity`, `handshake`, `protocol_state`, `store`, `invite` and now
`facade` all at the CRATE ROOT, so `super` of `contacts` IS the crate root. Every fact the facade needs is
therefore reachable with no visibility change:

| item | site | visibility |
|---|---|---|
| `qsp_status_tuple` | `protocol_state/mod.rs:79` | `pub` |
| `contacts_list_entries` | `contacts/mod.rs:493` | `pub(super)` at root |
| `contact_request_list` | `contacts/mod.rs:462` | `pub(super)` at root |
| `contact_state` | `contacts/mod.rs:498` | `pub(super)` at root |
| `identity_read_pin` | `identity/mod.rs:744` | `pub(super)` at root |
| `identity_voice_form` | `identity/mod.rs:679` | `pub` |
| `ContactRecord` / `ContactRequestRecord` | `store/mod.rs:211` / `:198` | `pub(crate)` |
| `ErrorCode` + `ErrorCode::as_str` | `model/mod.rs:20` / `:43` | `pub` |
| the six invite entry points | `invite/mod.rs:800 :932 :1195 :1310 :924 :907` | `pub` |

## 4. THE SINGLE FINGERPRINT RESOLUTION — WHY A TRUST SCREEN CANNOT LIE

`identity_peer_status` (`lib.rs:242`) is a thin wrapper over `identity_read_pin` that substitutes the literal
`"untrusted"` when there is no pin. The facade therefore calls **`identity_read_pin` directly, ONCE per
contact**, and derives BOTH `fingerprint` and `pinned` from that one result:

    let fp = primary_device(&v).map(|d| d.fp.as_str()).unwrap_or(v.fp.as_str());
    if fp.is_empty() || fp.eq_ignore_ascii_case("UNSET") { None } else { Some(fp.to_string()) }

⇒ the displayed full fingerprint IS the exact string the pin comparison consumes, and the `"untrusted"`
sentinel never reaches the surface at all. The 64-hex guard additionally refuses `UNSET` and the `""` that
`identity_voice_form` returns for a non-64-hex input — the refusal its own doc (`identity/mod.rs:672-679`)
says the caller MUST perform.

## 5. WHAT THE SEALS MEASURED (figures inserted after measurement, never in the pass that produced them)

- **Clippy delta vs base: ZERO.** `-D warnings` fails on this tree AT BASE (rc 101, 27 lib errors), proven by
  stashing the entire change. First run with the change: 29 — a delta of exactly TWO `doc_lazy_continuation`
  errors in one doc block, repaired. Re-measured: 27 = 27, facade errors 0.
- **`W7(b)` found-side control:** the value-shape needle finds **22** error consts, exactly the set the mapping
  declares, and their lines are exactly the taxonomy block's. The NAME needle returns **25** in the same test —
  the over-capture (`DS_COMMIT`, `DS_SIG`, `INVITE_CODE_PREFIX`) that made the earlier seal red on day one. A
  synthetic lowercase const makes the scrape return 23 and reds the seal: the control discriminates BOTH ways.
- **`W4` pinned discriminant set: 38** — 25 non-`Store` variants plus `Store`'s 13-member fan-out over
  `ErrorCode::as_str`, all distinct, `lock_upgrade_refused` among them.
- **Shard gate (`qsc_shard_check.py`), the gate that redded NA-0749:** `census 140 targets / manifest 140 rows /
  12 shards / missing 0 / unknown 0 / doc shard 11 with 0 co-tenant(s)`, `OK: manifest covers the census
  exactly`, rc 0. Linux shard 7 14→18, macOS shard 1 29→33; `doc:qsc` still ALONE in Linux 11 / macOS 4.

## 6. THE BUILD-TIME MISSES, RECORDED BEFORE THEY WERE TOUCHED

1. The totality file's `session_invalid` arm returned `VaultLocked`. **The facade was right and the test was
   wrong:** that binary never unlocks, so the process flag is default-false and the override fired exactly as
   specified. ⛳ The other five arms passed untouched in the same run — independent evidence, from a test not
   trying to prove it, that the override is scoped to EXACTLY ONE arm.
2. The seventh reason returned `Inactive` with `session_integrity_failed`. `qsp_session_store_key_load`
   (`protocol_state:168-181`) returns the REAL stored key when `vault::secret_get` succeeds and the TEST
   FALLBACK key only when it fails; the subprocess wrote under the real key, and setting the process flag
   WITHOUT loading a passphrase took the fallback branch. Same directory, two different keys.
   ⇒ **setting the unlock flag is not the same as unlocking.**

3. ⚠⚠ **THE FULL SUITE ON THE COMMITTED TREE WENT RED, AND IT CAUGHT SOMETHING SUBSET-GREEN COULD
   NOT.** `COMMITTED_RC=101`: `na0751_facade_locked_control.rs` failed its pass-through arm —
   expected `NoSession` for a locked vault with no session blob, got `VaultLocked`, with a
   `session_unsupported_version` marker proving it had READ a blob. It read the **fabricated** blob
   its SIBLING test had just pointed the process at: cargo runs a binary's tests in PARALLEL
   THREADS of one process, and each of the three writes `QSC_CONFIG_DIR`, a process-global.
   ⇒ **Giving that file its own BINARY isolates it from other FILES, not from its own SIBLING
   TESTS.** That is `E5`'s finding — the control's subject is a process atomic — applied one level
   in and missed; files 1, 2 and 3 carry guards and this one did not.
   DIAGNOSIS TESTED RATHER THAN ASSERTED: `--test-threads=1` passes, and three consecutive parallel
   runs in isolation ALSO pass. **That does not refute the race — it shows the interleaving is rare
   on an idle machine, which is what made it dangerous.** Cured by a file guard taken by all three
   tests, which removes the possibility instead of relying on scheduling. The red run is preserved
   at 444 as `RED_COMMITTED_qsc_suite_95c9b72a.log`.

All three are SR-16 rows 150, 151 and 152. None was cured by adjusting an instrument to make a
seal hit; the third was cured by repairing a real defect in this lane's own test file, and the
full suite is what found it.

## 7. W8 AND W9 — SEALED, THEN FOUND UNINSTRUMENTED, THEN INSTRUMENTED

The full suite went green with **`W8` and `W9` having no test at all**. It was found by a
double-check of the seal list, not by the build — a green suite that never touches a seal is not
evidence about that seal. Both now have instruments, in
`tests/na0751_facade_contact_surface.rs`:

- **`W8`** — a contact whose `fp` is the `UNSET` sentinel lists as typed absence
  (`fingerprint == None`), with a mutation control showing that without the 64-hex guard the
  published voice tier would be `identity_voice_form`'s empty sentinel.
- **`W9`** — the displayed `fingerprint.full` EQUALS `identity_peer_status(..).0`, the exact
  string the pin comparison consumes, with a mutation control on an EARLY byte.

⚠ **The deviation, stated:** `R373` §2(a) asks for a freshly-accepted request. That path is
measured unreachable single-party (see `D-1393`), so the IDENTICAL state was driven and the verb
exercised on its reachable `NotFound` arm.

**FOUR FINDINGS CAME OUT OF THE FAILURES, none a defect in shipped code:**
1. `contacts_add` requires a route token — the auto-mint path was retired (D616 §2m). The rule is
   22..=128 chars of `[A-Za-z0-9_-]` (`adversarial/route.rs:24`).
2. `contact_request_block` does NOT behave like `accept`/`ignore`: on an unknown alias it
   CONSTRUCTS a `REVOKED`/`blocked` record and returns `Ok(())`. The test pins the asymmetry, and
   in doing so makes `M5`'s divergence visible — raw `status` REVOKED vs `contact_state()`'s
   CHANGED, and the facade ships the latter.
3. `contacts_add(.., verify = false)` pins by default, so a contact can be `PINNED` with no
   displayable fingerprint at all — trust state and fingerprint are independent.
4. The voice tier reduces the FIRST 20 BYTES only (≈100 bits), so a trailing-byte change leaves it
   IDENTICAL. That is the ratified design; the test pins BOTH directions.

## 8. W10's THREE EXCEPTIONS, WITH THE NINE CI-ONLY STEPS NAMED

`CodeQL` — zero producer bytes across the repository's sixteen workflows (case-insensitive,
measured twice); post-push GitHub-side analysis; nothing local stands in.
`macos-qsc-qshield-build` — `runs-on: macos-latest` (`macos-build.yml:59`); the same cargo targets
ran on Linux inside `ci-4a` as a LABELED PROXY, platform gap stated.
`public-safety` — PARTIAL: 5 of 14 steps run locally (denylist filename scan · high-confidence
credential content scan · vector JSON parse sanity · markdown relative-link check · vector JSON
structure sanity). The NINE structurally CI-only steps are: two `actions/checkout` steps; `Require
classify success` and `Require advisories success`, which read `needs.*.result`; the two
`public_safety_gate.py` steps (`list-pr-files`, `scan-pr-changes`), which need
`$GITHUB_REPOSITORY` and a live PR number; the main-red blocking step; the push-only full-suite
waiter; and the docs/governance-only echo.

---

## 9. CORRECTION, APPENDED BY NA-0752 (D-1394; ordered at R374 §6) — TWO SECTIONS ABOVE WENT STALE

**MARK, DON'T REWRITE. Every byte of §§1-8 above is UNMODIFIED**; this section corrects them in
the open, beside them, exactly as a fenced ruling is never edited to match a later figure. A
successor reading §2 or §5 cold gets a wrong number for the tree that shipped, and this section
is how they find out.

**THE CAUSE, and it is the transferable part.** Commits `aa2af706` and `cfb58c90` *added* §6(3),
§7 and §8 to this document. **Adding a new section is NOT the same as revisiting the sections the
new work INVALIDATED.** §2 and §5 were left describing the 140-row tree while the shipped tree is
141. The document internally tells the story — §7 records the fifth test binary — but the two
sections whose entire job is to be exhaustive are the ones that went stale. Same family as
"an enumeration is a description of the record, not the record".

**§2, "THE EDIT SET, AND NOTHING OUTSIDE IT" — TWO CORRECTIONS.**

1. The list at lines 31-34 names **four** `na0751_facade_*` test files. **FIVE shipped.** The
   absent one is:

       qsl/qsl-client/qsc/tests/na0751_facade_contact_surface.rs           NEW

   (§7 of this document already records that fifth binary; §2 was never revisited.)

2. Lines 35-36 read `+4 rows` for both shard manifests. **The tree holds `+5` in each:**

       scripts/ci/QSC_SHARD_MANIFEST.txt           +5 rows (shard 7)
       scripts/ci/QSC_SHARD_MANIFEST_MACOS.txt     +5 rows (shard 1)

**§5's SHARD-GATE BULLET (lines 88-90) — THREE CORRECTIONS.**

| §5 says | the tree holds |
|---|---|
| `census 140 targets / manifest 140 rows` | **141 / 141** |
| `Linux shard 7 14→18` | `14→`**`19`** |
| `macOS shard 1 29→33` | `29→`**`34`** |

⚠ **A UNIT TRAP THAT MAKES 141 LOOK LIKE 140.** The 141st manifest row is `doc:qsc`, which prints
as `Doc-tests qsc` and **never** as a `Running` line. Checking `Running` lines against 141 reads a
false miss at 140. The reconciliation is `Running` + 1 doctest: base 135 + 1 = 136 rows; the
shipped tree 140 + 1 = **141**.

**BLAST RADIUS — MEASURED, NOT ASSUMED, at the time the defect was found:** **CI: NONE** —
`qsc_shard_check.py` reads the two manifests and the live census and never this document; it
measured **141 / 141, rc 0**, and both shard-manifest gates were PASS on #1780. **Code: NONE.**
**Records: REAL** — which is why this correction exists.

⚠ **RECORDED AND DELIBERATELY NOT FIXED (R374 §6, item 4's fifth finding):** three STALE
SELF-DESCRIPTIONS remain in the tree, all of them NAMES rather than assertions, nothing red:
the IPC test named `all_27_registered_commands_…` against 26 registered before NA-0751 and 38
after; `gui_driver.rs`'s `M = 6` module doc against 7 flows (8 after NA-0752); and the CI step
named *"the six flows"*, which runs `--ignored` unfiltered and therefore executes all of them.
They are cosmetic, they are owed, and they are **out of NA-0752's enumeration** — recorded here
so the debt is visible rather than forgotten.
