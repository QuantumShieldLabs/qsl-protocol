# NA-0747 — AS BUILT

**Slice 4 Phase 0: the seam measurement, the mockup reconciliation, and the harness verification.**
Promotion `D-1387` (PR #1774, merged `e069db87807b7e6ffc3398754dff06b8171d15f3`) · records `D-1388` ·
ruling `R359` and its two dispositions of 2026-08-19.

Bases: qsl-protocol `bb7e6b9aaa688320a661ca353eff37187977914c` (Phase 0's measurements) and
`e069db87807b7e6ffc3398754dff06b8171d15f3` (this act); qsl-desktop
`c52fd51bbaff5882741620a7774f2253814ddaa7` throughout. Open-PR sets **0** in both repos at every
measurement, each against a positive control returning rows.

⚠ **What this lane did NOT build: anything.** No product source byte in either repo, no gateway
command, no screen, no mockup edit. Phase 0 measures; Phase 1 designs.

## 1. THE THREE MEASUREMENTS

### 1.1 M-C — the harness: **6 of 6**

All six `na0701_gui_a..f` PASS at desktop `c52fd51b` via the documented runner
(`cargo test --test gui_driver -- --ignored --test-threads=1`), **83.53 s**, corroborated from the
harness's own artifacts and not the cargo line alone:

| scenario | run root | result | steps | jsonl rows | manifest artifacts |
|---|---|---|---|---|---|
| `na0701_gui_a_read_census` | `20260819T003406Z` | PASS | 96 | 97 | 63 |
| `na0701_gui_b_onboarding` | `20260819T003417Z` | PASS | 20 | 21 | 52 |
| `na0701_gui_c_lock_unlock` | `20260819T003423Z` | PASS | 28 | 29 | 57 |
| `na0701_gui_d_settings_persistence` | `20260819T003434Z` | PASS | 25 | 26 | 61 |
| `na0701_gui_e_erase_ceremony` | `20260819T003440Z` | PASS | 52 | 53 | 56 |
| `na0701_gui_f_menu_event_substitutes` | `20260819T003523Z` | PASS | 21 | 22 | 58 |

**242 steps, six terminal `PASS` rows, a `MANIFEST.json` each.**

⚠ **One precondition had drifted and the repair is stated, not hidden.** `tauri-driver` was **ABSENT**
on PATH with `QSLD_TAURI_DRIVER` unset; everything else matched the `gui_driver.md` box record exactly
(rustc `1.95.0 (59807616e 2026-04-14)`; webkit2gtk-driver `2.52.3-0ubuntu0.24.04.1`; xvfb, Xvfb and
dbus-run-session present). Repaired with `cargo install tauri-driver --version 2.0.6 --locked` — no
`sudo`, no repo byte, no manifest or lock change. ⚠ **`tauri-driver --version` is not a valid route** —
the binary has no such flag and errors; the version was read from cargo's own install receipt
(`.crates.toml`: `tauri-driver 2.0.6 (registry+…)`; `.crates2.json` agreeing) against a
`2.0.6` = 1 / `2.0.7` = 0 discriminating control. **Per R332.1 this is a new measurement after a
stated environment repair, not a re-run to green: there was no prior red run.**

### 1.2 M-A — the seam

**A1, the command census: 27, by two agreeing routes.** Route A, the declaring attribute
`#[tauri::command]`: **27**, all in `src-tauri/src` (26 in `commands.rs`, 1 in `lib.rs`), **0** in
tests. Route B, the `generate_handler!` registration list at `lib.rs:333`: **27**, the same names,
**symmetric difference EMPTY both ways**. ⚠ Route B first read **28** — the seat's parser split a
two-line `//` comment inside the macro as an entry. **The tree was right and the instrument was not**;
stripping comments line-by-line before splitting gives 27. Categories: vault lifecycle 7 · identity 2 ·
protection 3 · settings 2 · diagnostics 3 · relay configuration 9 · UI 1. **Not one is invite, contact,
handshake or messaging.**

**A2, the four screens' needs.** Mockups consulted per screen, from their own `<title>` bytes:
fingerprint → the RATIFIED reference (governing) plus 07, 07b, 13, 13a · invite approval gate → 14, 15 ·
connect banner → the channel-established banner (its own title reads *"Slice 4 candidate"*) and the
failure-states companion · compose-disabled → 11, 12. Surfaces measured with path:line —
`identity_fingerprint_from_identity` `identity/mod.rs:137` and
`format_verification_code_from_fingerprint` `identity/mod.rs:606`, **already called** by the desktop at
`commands.rs:144-145`; `contacts_request_list/_accept/_ignore/_block` `contacts/mod.rs:1542/1562/1610/1621`;
`invite_create/_redeem/_accept/_finish/_list/_revoke` `invite/mod.rs:800/932/1195/1310/924/907`;
`qsp_status_tuple` `protocol_state/mod.rs:79`; `handshake_status` `handshake/mod.rs:1315`.
⚠ **The compose-disabled driver is ENUMERATED, NOT ANSWERED**, with the banked constraint quoted
(R334.3: never key off `send_ready`) — and one new fact for the design: `qsp_send_ready_tuple`
(`protocol_state/mod.rs:108`) is **`pub(crate)`**, so the value is not merely unreliable but not typed
API. Filed as `ENG-0206`.

**A3, linkability — the question was already closed by the tree.** `qsc` declares no `[lib]` and no
`[[bin]]`; `src/lib.rs` and `src/main.rs` both exist, so cargo infers both targets; it is a workspace
member of qsl-protocol, and the desktop is a separate single-member workspace. **It has been linked as
a library since NA-0705**: `src-tauri/Cargo.toml:23`, git rev `32e572c7`, resolved in `Cargo.lock`, with
**42 lines carrying `qsc::` / 44 occurrences** in four files — one property, two instruments, differing
by exactly the two double-token lines `commands.rs:289` and `lib.rs:308`. The pin is **147 commits /
10 days** behind main and **is an ancestor of main**. The module set is **identical** at both revs (21
`pub mod`); the delta is inside them (`transport` +679/−59, `invite` +248/−12, `handshake` +195/−16,
across 19 changed files). Filed as `ENG-0207`.

### 1.3 M-B — the mockups

All four sha256 values authenticated **against the commit's own blobs** (`git cat-file`), not the
worktree; expected-vs-measured written to two files and `diff`ed unpiped, **rc 0**, against a negative
control (a fifth file's sha matching nothing in the expected set).

Markers were **lifted from the RATIFIED reference's own bytes**, never modelled, and the census runs a
**provenance check first** — which caught the seat's own conflation: three markers read `ref=0` because
they are *corroborating* markers about the targets, not reference-derived ones, and they are reported
separately rather than counted. Negative-control file `mockup-08-create-vault.html` reads **0 on every
marker**.

| file | tier-1 voice form | tier-2 full form | verdict |
|---|---|---|---|
| `mockup-07-identity-pane.html` | PRESENT, byte-identical to the reference | PRESENT, behind `<details>` | **measured no-op** |
| `mockup-07b-onboarding-identity.html` | PRESENT, byte-identical | **deliberately ABSENT** | **forbidden** |
| `mockup-09-vault-security-pane.html` | absent | absent | **no target exists** |

`07b`'s tier-2 absence is a **standing operator ruling** — NA-0680 Finding 3 (2026-07-26), reaffirmed at
NA-0703 R179, recorded in the file's own header and in an in-body comment **sitting exactly where tier 2
would go**, and guarded **in both directions** by `design_polish.rs:785-810`. `09` is the
Vault-and-Security pane and carries no fingerprint presentation of any kind.

⚠ **Two candidates RECORDED, not acted on:** `mockup-07` carries a **dead `.reveal` CSS rule** (declared
once, zero elements), and both it and the reference use `var(--text-accent)`/`var(--text-muted)` while
**neither defines either property**.

## 2. THE PINS AND THE INFRA-LITERAL POSTURE

**Frozen-needle pins: 0**, both repos. No pin, test, script or workflow references a `docs/mockups`
path (**0** inside `.github`/`src-tauri`/`scripts`/`tools`/`tests`, against positive controls returning
258 desktop / 7698 protocol hits) or any of the four content shas (**0** each, whole-tree). The token
`mockups` outside `docs/` appears **13** times, **all** prose in desktop `DECISIONS.md`. ⇒ the lockstep
rule does not bind. ⚠ Adjacent, reported not acted on: desktop `DECISIONS.md:1628-1629` cites two
mockups **by line number**, so any future insertion above those points staleifies two records.

**Infra-literal posture: clean at base, and the gate is proven live on this very path.** The scanner's
own selftest passes **13 checks, 0 failed**; the Tier-1 tree scan is **clean — 76 files, 22872 lines
examined**, rc 0. ⚠ **Scope was proven, not assumed** (SR-21): planting one line into `mockup-07` moved
the examined count **22872 → 22873**, so the file is inside the scanned set. That plant did not red
Tier-1 because CGNAT is a **Tier-2b added-line** class — a mode/claim mismatch in the seat's own probe;
re-run correctly, diff mode **REFUSED** with rc **1**, naming
`docs/mockups/mockup-07-identity-pane.html: [added-line:tailnet_cgnat]`. Tree restored to base, sha
verified, throwaway branch deleted.

## 3. IDS, AND HOW THEY WERE DERIVED

Declaring forms, `git grep` (never `grep -r`, which honours `.gitignore` and is blind to the tracked
evidence tree), fence-aware, both controls per space, **re-seeded at each base rather than copied**.

⚠⚠ **RE-SEEDING WAS NOT CEREMONIAL.** Between `bb7e6b9a` and `e069db87`, three mention counts moved
because `D-1387` landed: `D-1388` 0→**2**, `ENG-0205` 1→**3**, `ENG-0207` 0→**1** — every one planted by
`D-1387`'s own IDS paragraph, every one classified as a MENTION with **0** declarations. **WF-0087
reading true a fourth time in this lane.**

⚠ **A stale control failed loudly and was re-derived.** The prediction-ledger negative controls were
seeded from the predecessor lane's script (rows 98/99/100 expected 0); all three returned **1**. They
were correct at row-max 97 and meaningless at row-max 103. *A control is only a control at the base it
was designed for.*

⚠⚠ **THE RULING-ID FILENAME ROUTE IS BLIND.** Route A returned max **R357** while `R358` was **consumed**
— NA-0746's v2 ruling, present in four tracked files and banked under a filename not containing the
token. **Ruling-id derivation MUST include the content route.**

## 4. WHAT WENT WRONG, AND IT IS THE MOST USEFUL THING HERE

**`ENG-0195`'s figure is 26; the census is 27; the promotion attributed 27 to the entry.** The gateway
registers 27 (26 in `commands.rs` + `ui_surface_changed` in `lib.rs:252`); the entry states 26 twice
while enumerating 27 names — a phantom (`vault_version_state`, a plain `pub fn` with 0 registration
hits) and an omission (`ui_surface_changed`). **The errors cancel in the count**, so every half
reconciles when checked alone. `D-1387`'s four attributions are superseded by the amendment beside
`ENG-0195`; the substantive claim re-measures TRUE, weakened by zero cases.

**Root cause, on the seat:** the brief's §0 paraphrase (*"ENG-0195: 27 gateway commands"*) was taken as
the entry's figure and the entry was never opened. **STOP 001 re-measured three of the brief's premises
against the tree and refuted all three; the one premise it did not re-measure — a premise about what a
RECORD SAYS — is the one that landed false.**

**Caught by the ordered next act.** Drafting the `ENG-0195` amendment R359 §6 ordered is what opened the
entry. Nothing else in the sequence would have.

## 5. CLAIM BOUNDARY

**Measured:** both mains and both PR sets · every id space with both controls at both bases · the four
mockup shas against the commit's blobs · the two-tier markers against the reference's own bytes · the
27-command census by two agreeing routes · `qsc`'s crate structure, link status, call-site figures and
pin distance · the fingerprint-format disagreement · the harness preconditions and a 6-of-6 green run
with per-scenario artifacts · the infra-literal posture and the gate's liveness on the target path · the
existence and delta of the NA-0703 packet copies · `ENG-0195`'s three defects and the provenance chain.

**NOT claimed, and only Phase 1 or the operator can decide:** which linkability shape Slice 4 adopts
(**no option is ruled**; the set is non-exhaustive) · what drives compose-disabled (**not answered**) ·
whether the ratified fingerprint format or the shipped one wins (**an operator design gate**) · whether
the `qsc` pin is bumped and on what gate · whether NA-0680 Finding 3 is revisited (**it is not**) · any
colour, token or hex claim (none is derived from any mockup) · that mockups match any shipped screen.
**Also not claimed:** that the harness green predicts CI green — the run is this box only, and
`gui_driver.md` records that the CI job re-measures its own producer identity.
