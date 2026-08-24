# NA-0757 — AS BUILT — THE ENG-0239 REPAIR: THE HANDSHAKE SESSION STORE

Spine decision **D-1399** · ruled at **`R388`** · SR-15 cold read commissioned and consumed
Base: qsl-protocol `f98af5cc0d22009fd73644f835a637d1fe338484` · qsl-desktop `5eb64c2e371906af1533804a542bdc119661d960` (#37 open and held)

## 1. What shipped

A session directory that is **born correct**, and a store refusal that **names its cause**.

`ENG-0239` was neither a session-store-key defect nor a process-shape defect. `qsp_sessions`
was created by a bare `fs::create_dir_all` with **no explicit mode**
(`protocol_state/mod.rs:982`), so its permissions were the **calling process's umask**. The
`qsc` binary sets one (`main.rs:59`); **no in-process library consumer does** — `qsl-desktop`
has zero `set_umask` occurrences and its umask is measured **002**. A directory born with any
write bit beyond the owner is refused for good by `enforce_safe_parents`
(`perms_group_or_world_writable` = `mode & 0o022 != 0`). **That refusal is correct.** The birth
mode was the defect, and the flattening is why nobody could see it.

Three source edits, two of them one-liners in substance:

| file | change |
|---|---|
| `fs_store/mod.rs` | `ensure_dir_secure` creates **mode-explicitly** (`DirBuilder::mode(0o700)`, recursive). Windowless: `0o700` has no group/world bits, so no umask can widen it. **17 pre-existing call sites (15 production, 8 files) inherit it.** |
| `protocol_state/mod.rs` | `:981-982` collapses onto that primitive. Written fully qualified (`quarantine:486` precedent) so **no other byte in the file moves**. |
| `handshake/mod.rs` | both store sites emit `store_code=<wire name>` on the error marker; the OUTER string is **deliberately unchanged**. |

## 2. The measurements

| instrument | baseline | after |
|---|---|---|
| qsc `cargo test` (suites / passed / failed) | **143 suites · 697 passed · 0 failed · 2 ignored** (rc 0, 3h12m) | **144 suites · 700 passed · 0 failed · 2 ignored** (rc 0, 3h10m) |
| qsl-desktop `cargo test` @ pin `d3fefd12` | 18 suites / 159 passed / 0 failed / 12 ignored | unchanged by this PR (pin bump is a separate PR) |
| shard gate, linux (`census/manifest/missing/unknown`) | 143 / 143 / 0 / 0 | **144 / 144 / 0 / 0**, rc 0 |
| shard gate, macOS | 143 / 143 / 0 / 0 | **144 / 144 / 0 / 0**, rc 0 |
| shard gate negative control (row removed) | — | **rc 1**, `FAIL: MISSING from manifest`, on **each** manifest |
| `cargo fmt --check` hunks (whole crate) | **309** | **309** — my files contribute **zero** |

⚠ **`cargo fmt --check` is RED at base and is not a CI gate in this repo** (zero `fmt`/`clippy`
occurrences across all 16 workflows). The honest statement is not "fmt is clean" but "this lane
adds no fmt diff": per-file deltas measured `fs_store` 0→0, `protocol_state` 0→0, `handshake`
6→6 (all pre-existing), `na0756` 2→2 after correcting the two hunks **my** block introduced —
never a pre-existing one.

## 3. The seals, and every control shown RED

| arm | property | control |
|---|---|---|
| **A** | a fresh session dir is **born `0700` under `umask 002`**, asserted BY EQUALITY on the observed mode; the in-process facade finish completes and the peer reads ACTIVE | **product tamper** — revert the `protocol_state:981` collapse → **rc 101, RED**. ⚠ The FIRST control (reverting the `DirBuilder`) returned **rc 0 — it passed**; see §3.1 |
| **B** | an already-poisoned `0775` dir is **still refused**, and the marker carries `store_code=unsafe_parent_perms` **by equality**, with `code=` still `handshake_session_store_failed` | **product tamper** — revert the augmentation at both sites → **rc 101, RED**; the marker then showed `code=handshake_session_store_failed` with **no `store_code`** |
| **C** | the env-gated seed fallback still requires **both** gates | assertion-side only (see the limit below) → **rc 101, RED** |

### 3.1 ⚠⚠ THE FIRST ARM-A CONTROL PASSED AGAINST A REVERTED PRODUCT

`A1(a)` shipped as **two edits of unequal effect**, and the control is what proved it. Tampering
the `DirBuilder` mode-explicit creation left arm A **green (rc 0)** — because `ensure_dir_secure`
already calls `enforce_dir_perms` one line after creation, which chmods `0775 → 0700`. So the
directory is correct by the time `write_atomic` checks it, with or without the `DirBuilder`.

⇒ **The `protocol_state` collapse onto `ensure_dir_secure` carries the OBSERVABLE repair** — the
primitive's `enforce_dir_perms` heals a just-created directory one line after creation, which the
old inline sequence (`enforce_safe_parents` + bare `create_dir_all`, no perms enforcement) never
did. **The `DirBuilder` closes the creation WINDOW** (the read's `F-8`): proven **BY
CONSTRUCTION** — `0o700` carries no group or world bits, so `mode & ~umask` cannot widen it and no
wrong mode ever exists on disk — and **UNPROVABLE BY TEST, because a window is not an
observable.** Both edits stand; the seal is aimed at the line that carries the behaviour; the
window edit's evidence is its bytes.

This is `D-1398`'s vacuous-seal property firing one lane later — *"a control that passes looks
exactly like a working control"* — caught, diagnosed to a **real property** rather than a harness
error, and re-aimed. Had the first control been banked, this lane would have shipped a seal that
passes with its product reverted.

⚠ **Arm C's control is weaker than A's and B's, and the reason is recorded rather than hidden.**
Its product line is the seed-fallback gate, which the `R388` A2 pre-commitment forbids this lane
from touching **at all**, temporarily included. So its control proves the assertion *executes*;
it does **not** prove the gate is load-bearing. A and B tamper the real product because NA-0756
measured a seal that passed with its gate deleted — an assertion-side control cannot catch that.

## 4. What is driven, and what is not

**Driven.** Both process shapes now complete a real handshake against `qsl-server`'s real router
in-process: the existing subprocess arm (byte-unchanged) and a **new in-process facade arm**.
Each drives its **own** inviter — a finish CONSUMES the reply it acts on, so sharing one flow
would have demoted the subprocess arm's assertion from `ok` to `none` rather than standing
beside it.

**Not driven.** No GUI. No relay beyond the in-process test server. The **field** half — a real
desktop-born vault on real machines — is the operator's, and `R388` S3 re-plans it: one fresh
invite pair end to end on the repaired build, in a native umask-002 environment, with **no
chmod and no pre-made dirs**, expecting `qsp_sessions` born 700 by `stat` and both sides
completing untouched.

## 5. What went wrong, kept for the next lane

- ⚠⚠ **I invented a number and nearly shipped it into repo truth.** A source comment claimed the
  flattening "took 187 days to localize". There is no such measurement — `ENG-0239` was filed the
  day before. Removed at both sites. **A figure that cannot be pointed at a measurement does not
  belong in a comment any more than in a record.**
- ⚠⚠ **My first post-edit suite was testing stale bytes.** Three files were edited AFTER `cargo
  test` had already compiled its binaries. The deltas were comments and whitespace, so the result
  would have been identical — which is exactly why it was tempting to keep. Killed at 4 minutes
  and restarted against source frozen and **sha-pinned**, so the suite's result names bytes that
  can be verified. **A suite result is a claim about specific bytes.**
- ⚠ **I conflated two counts.** "9 modules" was the cold read's figure for `write_atomic` (23
  sites), not for `ensure_dir_secure`. Re-measured: **17 pre-existing call sites, 15 production,
  8 files.** A number carried from an adjacent finding is not a measurement.
- ⚠ **`rc=$?` after a pipe read `tail`, not the gate** — the NA-0753 trap, fired again and caught
  by re-measuring unpiped. The gate's FAIL *text* was the real signal.
- ⛳ **The `R391` id trap did not fire, because a predecessor had already classified it.** The
  R-space's raw maximum reads `R391`, which is **binary noise inside `.tar.gz` blobs under
  `NA-0603/`**; every text hit is a prior lane's own sentence classifying that noise. Reading the
  predecessor's classification cost seconds and saved a wrong id.

## 6. Filings

| id | what |
|---|---|
| `ENG-0239` | **amended beside the entry, mark-don't-rewrite; NOT closed** — the heal-policy axis is deferred and the close is the Director's |
| `ENG-0240` | every directory in `qsc` is born at the ambient umask; no `DirBuilder` anywhere in `src/`; two bare creations OUTSIDE the config dir named and unfixed (`lib.rs:2805`, `output/mod.rs:363`); the deeper property is **repair-without-disclosure**, already shipped |
| `ENG-0241` | the G2 anti-rollback record is **unauthenticated JSON inside the session directory**; `Ok(None)` skips the guard, so one unlink disables it |
| `ENG-0242` | a facade doc-comment points at a `store_code_from_wire` arm that does not exist, and the arm above it would shadow one anyway — the trap that would have made the obvious un-flattening ship a wrong diagnosis |
| `ENG-0142` | cross-referenced, **not edited, not closed** |

## 7. The property this lane hands forward

⚠⚠ **A FLATTENED ERROR IS COMPATIBLE WITH EVERY CODE IT FLATTENS.** The field capture was the
*flattened* string, and seven distinct `ErrorCode`s reach that flattening — so the field evidence
discriminated **none** of them. The lab proved a cause **sufficient**; only the **intervention
experiment** (one variable changed, the outcome flipped) proved the **instance**. Two other codes
produced a byte-identical capture and stayed live until a `chmod 700` completed a handshake that
neither of them is cured by.

This is why NA-0756's open question closes here rather than in NA-0756: that lane could not have
answered it, because the only evidence it had was the string that names nothing.
