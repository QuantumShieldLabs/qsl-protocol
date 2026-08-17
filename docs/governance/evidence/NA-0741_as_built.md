# NA-0741 — AS BUILT

**Lane:** NA-0741 · **Decision:** D-1376 · **Rulings:** R340 (interim), R341 (final)
**Base:** main `4c59ffdadef38d9a7f058b1b39387bd9a7298e44`
**This PR is RECORDS ONLY. Zero product source bytes. The repair is promoted, not implemented.**

## 1. WHAT WAS PROMOTED

The final directive `DIRECTIVE_NA0741_FINAL_NPRIME.md` (597 lines, sha256
`a8f25983a80d1bae8df78244772e5e50ff95abe9939beab087bb182c5c062d28`, mode 444), which a **fresh
seat** will execute under the SR-15 ceremony. It specifies **OPTION N-PRIME**: classify before
unpack, skip **exactly** Handshake / InviteInit / InviteResp, route Unknown-class frames to unpack
exactly as today, leave `transport/mod.rs:1186-1250` byte-unchanged, and gate the skip to
`AckMode::Lease` only.

## 2. INPUTS, ALL SEALED 444 AND BANKED BEFORE ANYTHING CONSUMED THEM

| artifact | sha256 | lines |
|---|---|---|
| formalization brief (verbatim) | `f643939273339ec6d982c9c751827f4729c8d44fe8f9a1d10d7df70ca9f0a8b4` | 153 |
| operator-blessed design block (extracted from the brief's own bytes) | `316b09acedee3221a7c429898ef84d4833b3ad7edaf40bd06898d7ec5b306e6b` | 56 |
| interim ruling (R340) | `4ade0ca7966984c0a8bf4d76ed388a59d7cc15dbe4d803c8c3a249c2897f60d1` | 65 |
| **SR-15 adversarial cold read — NOT this seat's work** | `98e266562d2797c06e1bfb1337a3501a0072f77fccdcbbf1ce31914ac42f95b0` | 842 |
| final ruling (R341) | `b5284c1d1da20836544d84251b4be942cc04e9b77582c2ec854d611ce92972ef` | 113 |
| Director's verification of the final directive | `f2877a61f0d3328356d9ff3dd553825f3dfc12d001cf30474a99289b0070cae9` | 22 |
| **final directive (the deliverable)** | `a8f25983a80d1bae8df78244772e5e50ff95abe9939beab087bb182c5c062d28` | 597 |

The design block was **extracted from the banked brief's own bytes by delimiter**, never retyped,
and diffed back **identical (rc 0)** against a **negative control returning rc 1**, non-vacuity
asserted. Every ruled text folded into the final directive was likewise **extracted from its
source's own bytes**: **6/6 positive** (region occurs exactly once in the finished document) and
**6/6 negative** (last-character-mutated copy occurs zero times).

## 3. THE MEASUREMENTS THIS LANE OWNS

- **Base UNMOVED**, bare and unpiped, against the **NAMED** `github` remote, re-verified at the
  edit. **Open-PR set MEASURED EMPTY** with a positive control returning merged rows.
- **Ids on DECLARING forms** via `git grep` (never a `.gitignore`-honouring recursive search —
  WF-0087 part 2, which is blind to 514 tracked files in this repo), `D-` across **all four**
  record forms, **every** id space swept: `NA-0741` / `D-1376` each **0 declaring, 0 mentions**,
  against three positive controls at `declaring=1`.
- **The ruling's premise, verified from the tree's own fixture bytes rather than adopted:** six of
  the seven breaking fixtures are Unknown class (`00 00`, `6e 6f`, `6e 6f`, `6e 6f`, `6e 61`,
  `7b 22`); the seventh, `timeline_store.rs:154`, is `01 02` — InviteResp. Controls confirmed the
  classifier returns Message / InviteInit / InviteResp / Handshake for the four real frame shapes
  and Unknown for 0- and 1-byte inputs, so it discriminates in both directions.
- **The per-item loop** spans `transport/mod.rs:521-1252` with **35 early exits** (7 `return Err`,
  28 `?`). Three of the seven abort on **frame content** after a successful unpack (`:642`,
  `:682`, `:720`) and ride the ENG-0142 closure amendment.
- **`qsp_unpack`'s Err channel carries three populations**, which is why the design block's third
  arm was ruled void: ten frame-content codes, plus `qsp_session_store_failed` (a store WRITE),
  `qsp_no_session` and `qsp_channel_invalid`, with `session_rollback_detected` riding the same
  failure.

## 4. THE SEAT'S OWN ERRORS, RECORDED RATHER THAN TIDIED

1. **The §2(d) census was narrower than its claim** — it counted rc idioms while the claim covered
   marker consumers. Caught by the SR-15 read; landed as SR-16 row 44.
2. **The first rc census missed 15 of 28 test functions** (helper-wrapped receives), and the miss
   was found **by reading a file, not by the instrument**. Widened before the stop.
3. **The R-id sweep took `max()` over MENTIONS** and would have taken `R341` instead of `R340` —
   the instrument committing the very hazard it was written to detect.
4. **Four line-wrap / model-built needle misses** across the lane (a banked clause, an extraction
   anchor, a status-line needle carrying a leading space that was an artifact of the tool output
   it was read from). Each time the **needle** was wrong and the **tree** was right.
5. **The assembler's anti-placeholder assert fired on its own proof table**, which had reproduced
   the placeholder literal. The assert was right; the table was wrong.

## 5. WHAT IS EXPLICITLY NOT DONE

ENG-0142 **not closed, not repaired** · ENG-0196 **untouched (lane 2)** · ENG-0191 (a)–(e)
**unruled** · ENG-0194 **not repaired** · ENG-0193 **not built** · WF-0086 and WF-0087 gates
**not built** · **#1745 — an ISSUE, not a PR — stays OPEN** · no prior findings swept · no test
weakened, skipped or deleted · no standing rule minted · no fenced ruling or sealed artifact
edited · **zero product source bytes.**

---

# ══════════════════════════════════════════════════════════════════
# PART 2 — THE IMPLEMENTATION (NA-0741 impl seat, D-1377)
# ══════════════════════════════════════════════════════════════════

⚠ **PART 1 ABOVE IS THE PROMOTION'S AS-BUILT AND IS UNEDITED.** It carries the input
provenance table — the sha256 of every sealed artifact this lane consumed — and destroying
it to make room for the implementation record would have deleted the only place those
hashes live in repo truth. **Mark, do not rewrite**, which is the same discipline this
program applies to fenced rulings and sealed directives. Part 1's bytes are preserved
verbatim: sha256 `fa165ceaa1daff1fb4f8e045a98de73955f54d30dab793363c67422630dc9c09`.

## NA-0741 — AS BUILT, PART 2: THE REPAIR
## RECEIVE-SIDE FRAME-CLASS DISPATCH, under the ruled option N-PRIME (R341)

**Executable document:** `DIRECTIVE_NA0741_FINAL_NPRIME.md`, 597 lines, 47342 bytes, mode 444,
sha256 `a8f25983a80d1bae8df78244772e5e50ff95abe9939beab087bb182c5c062d28` — verified against
`LATEST.md` before it was consumed.
**Base:** main `788c8de5fa29fabc0540df4d53bea48307b2ffde`, derived **bare and unpiped** against the
GitHub remote by URL (this seat's remotes were ENUMERATED, not assumed: `origin` IS
`https://github.com/QuantumShieldLabs/qsl-protocol.git`, `mirror` is the local mirror).
Local HEAD identical, worktree clean, and exactly **two first-parent merges** past the directive's
base `4c59ffda` — #1756 (promotion) then #1757 (queue-gate) — whose combined diff touches **six
records files and zero product source**.
**Knobs stated on every run:** `PULL_LEASE_SECS=60`, `MAX_BODY_BYTES=65536`.

---

## §1. THE EDIT SET, AS LANDED — 6 files, +92 / −3

| file | change |
|---|---|
| `qsc/src/frameclass.rs` | **NEW.** The classifier: `FrameClass{Handshake,Message,InviteInit,InviteResp,Unknown}`, `name()`, `is_known_foreign()`, `classify()`. Pure, total, no allocation, no I/O. **Every discriminator reached BY REFERENCE — no magic literal in the module.** |
| `qsc/src/lib.rs` | `mod frameclass;` (crate-private), placed alphabetically. |
| `qsc/src/handshake/mod.rs` | **VISIBILITY ONLY:** `const HS_MAGIC` → `pub(crate) const HS_MAGIC`. `handshake` is already `pub mod`, so this adds **no public surface**. The only visibility change the lane requires. |
| `qsc/src/transport/mod.rs` | §5's classify-and-skip block after the dedup block; `skipped` (per round) and `skipped_total` (per batch); §5.1's round condition; §5.2's end-of-batch summary. |
| `qsc/tests/timeline_store.rs` | **FIXTURE LINE ONLY** (§7.2). Three assertions byte-unchanged, hash-verified before and after. |
| `qsc/tests/na0741_frame_class_dispatch.rs` | **NEW.** T1–T7. |
| `scripts/ci/QSC_SHARD_MANIFEST{,_MACOS}.txt` | +1 line each (§7.7). |

**No new dependency, no lock change, no `.github/**`, no workflow, no qsl-server change, no
committed script edit, no quarantine-module use, no test weakened/skipped/deleted, zero secrets,
loopback only.**

### ⛳ THE `Err(code)` ARM IS BYTE-UNCHANGED, PROVEN MECHANICALLY RATHER THAN BY EYE

The arm was carved from BOTH trees by an anchor asserted unique and hashed. ⚠ The obvious anchor
`Err(code) => {` occurs **4 times** (`:1186`/`:1605`/`:1674`/`:3217`); rather than refine the needle
it was **enumerated and classified**, and the arm's own first statement
(`let from_alias = peer_alias_from_channel(ctx.from);`, exactly 1 occurrence) was used instead.

| tree | bytes | sha256 (16) |
|---|---|---|
| base `788c8de5` | 4281 | `6be034fe77f03006` |
| repaired | 4281 | `6be034fe77f03006` |

**IDENTICAL.** All four side effects present exactly once each in the repaired arm:
`contact_request_upsert` · `emit_cli_contact_request` · `emit_tui_contact_request` ·
`emit_file_integrity_fail` · `record_qsp_status(…, false, …)` · `emit_marker("qsp_unpack", Some(code))`.
A last-character-mutated copy compares DIFFERENT, so the comparison discriminates.

---

## §2. THE SEALED EXPECTATIONS — E1…E6

⚠ **E5 IS EVALUATED AGAINST ITS RE-SEALED TEXT, NOT THE BRIEF'S ORIGINAL.** The brief's §4 E5 read
*"REPAIRED, Legacy: foreign frame quarantined locally with witness marker, run continues, message
delivered, rc 0."* The FINAL directive §2 **re-sealed E5 verbatim from the ruling** as *"Legacy: the
same foreign-frame arrangement still aborts exactly as today, proven beside the Lease skip in one
discrimination test."* The original is **VOID** — §2 deleted the Legacy quarantine branch rather
than amending it. Byte-verbatim binds to the CURRENT governing text.

| # | expectation | verdict | measured |
|---|---|---|---|
| **E1** | RED CONTROL on the unrepaired tree: wedge reproduces, rc 1, `qsp_env_decode_failed`, nothing delivered; preserved, never re-run to green | **HIT** | rc **101**; 1 passed / 6 failed; **13 ×** `code=qsp_env_decode_failed`; **0 ×** `recv_frame_skipped`. Preserved (R332.1). |
| **E2** | REPAIRED, Lease: exits 0 and delivers the message behind the foreign frame IN THE SAME INVOCATION; both mailboxes of a completed invite receive cleanly | **HIT** | T1/T2 rc 0, `recv_item` present, payload **byte-equal** to what was sent. T6: inviter `class=handshake` rc 0, redeemer `class=invite_resp` rc 0. |
| **E3** | The foreign frame survives server-side: leased, unacked, undestroyed — accounting before/after, unexplained = 0 | ⚠ **PARTIAL — and this is a RESULT, not a tuning** | **(a) HIT:** after lease expiry a raw `GET /v1/pull?ack=lease` returns the **exact planted bytes** for every skipped class. **(b) NOT DISCHARGED:** no store-level row census was run in this lane; the arms assert FRAME RESIDENCY, not `unexplained = 0`. NA-0740's harness did that accounting; this lane did not repeat it, and the expectation's second half therefore stands unproven here. |
| **E4** | The `replay_reject` arm is byte-unchanged and its existing coverage still passes | **HIT** | Byte-identity proven above (the `qsp_replay_reject` block at `:1200-1248` lies inside the carved region). Coverage: full suite green — §6. |
| **E5** (re-sealed) | Legacy: the same arrangement still aborts exactly as today, proven beside the Lease skip in one discrimination test | **HIT** | T4, one test, both modes on the SAME fixture and SEPARATE mailboxes (a legacy pull DELETES what it returns): `--ack-mode lease` → rc 0 + `recv_frame_skipped class=invite_resp disposition=left_leased`; `--ack-mode legacy` → rc non-zero, **0 ×** `recv_frame_skipped`. |
| **E6** | NEGATIVE CONTROL: with the delta symbol reverted, the new tests go RED | **HIT** | Removing exactly the `is_known_foreign` continue block (**1229 bytes, 23 lines**; declarations, round condition, summary and classifier all retained, tree still compiles) → **6 of 7 RED**, T3 still passes. |

---

## §3. RED-FIRST — A PROOF, NOT A CLAIM (§7.1, SR-19 half (a))

**The run of record is against the EXACT BYTES that land.** An earlier red run (same six arms, same
rc 101, same abort code) preceded a **wording-only** fix to one assertion's FAILURE MESSAGE — no
assertion, fixture or arm changed. The red was re-established so the evidence matches the landed
file byte-for-byte; **it was never re-run to green.**

```
CMD    : cargo test -p qsc --test na0741_frame_class_dispatch
TREE   : 788c8de5, UNREPAIRED — only the new test file added, zero product source
KNOBS  : PULL_LEASE_SECS=60  MAX_BODY_BYTES=65536
RESULT : 1 passed; 6 failed  (rc 101)  — 216.51s
```

| arm | at base | after repair |
|---|---|---|
| T1 `invite_class_frames_at_head_do_not_abort_the_batch` | **RED** | ok |
| T2 `handshake_class_frame_at_head_does_not_abort_the_batch` | **RED** | ok |
| T3 `unknown_class_junk_still_reaches_unpack_and_still_rejects` | **ok** (red-first-EXEMPT) | ok |
| T4 `lease_skips_where_legacy_still_aborts` | **RED** | ok |
| T5 `the_skip_marker_leaks_nothing` | **RED** | ok |
| T6 `both_mailboxes_of_a_completed_invite_receive_cleanly` | **RED** | ok |
| T7 `foreign_litter_at_the_head_still_delivers_up_to_max` | **RED** (partial build — §4) | ok |

⚠ **T5 IS RED AT BASE AND §7.1 DID NOT LIST IT — a deviation reported rather than hidden.** The
directive lists T1/T2/T4/T6/T7 as the red-first set. T5 as built also asserts that the marker under
test **actually fired**, without which "the skip marker leaks nothing" would be vacuously true of a
run that emitted no marker at all. That assertion makes it necessarily red at base. The
strengthening is deliberate; the red was captured with the rest.

⚠ **T3's obligation is DIFFERENT AND STRONGER**, as the directive states: it asserts UNCHANGED
behaviour, so it must pass on BOTH trees. Measured: **ok at base AND ok after the repair.** It is the
arm that proves the classifier does not OVER-SKIP.

---

## §4. T7's RED-FIRST IS A ***PARTIAL BUILD***, AND THAT IS STATED

T7 is the only arm whose red-first control is a partial build: §5's insert and §5.2's summary
present, **§5.1's round condition absent**. Measured on that tree:

```
recv_frame_skipped class=invite_resp id=<redacted> bytes=63 disposition=left_leased   ×4
recv_skip_summary count=4
recv_none                       ← zero delivered, and the receive exited 0
```

**6 passed / 1 failed** — only T7. ⇒ §5.1 is the single thing standing between the repair and a
**silent** zero-delivery, proven by construction rather than argued. Without it the repair trades a
loud `rc 1` for a silent `rc 0`.

---

## §5. §7.2 — THE ONLY COMMITTED TEST TOUCHED, VERIFIED BY EXECUTION

`timeline_store.rs`: `vec![1,2,3,4,5,6,7]` (opens `01 02` = **InviteResp**) →
`vec![0x01,0x00,0xFF,0xFF,0x04,0x05,0x06]` (**Message** class).

- ⛳ **THE RE-AIM IS LOAD-BEARING, PROVEN BY DELIBERATE MEASUREMENT:** on the repaired tree with the
  fixture UNCHANGED, `timeline_not_written_on_receive_reject_no_mutation` **FAILS** at
  `timeline_store.rs:181` — *"receive reject must fail"* — because `01 02` is skipped and the
  receive exits 0. The re-aim is not cosmetic.
- ⛳ **THE EMITTED CODE WAS READ, NOT ASSUMED** (the directive requires verification by execution):
  `event=qsp_unpack code=qsp_env_decode_failed ok=false` → `event=error code=qsp_env_decode_failed`.
  **Still `qsp_env_decode_failed`.**
- **All three assertions byte-unchanged**, hash-verified identical before and after
  (`sha256 49d91492b68cf225`); the applying script REFUSES TO WRITE if they differ. The test's own
  NAME stays true.
- The comment beside it cites this directive and names the surviving authenticated home —
  `file_transfer_mvp.rs::tampered_chunk_reject_no_mutation` (`:413`/`:434`/`:443`), verified present
  — following `ratchet_step.rs:243-254`'s precedent: *a contract with two homes drifts.*

---

## §6. SUITE RECONCILIATION (§7.3)

Both runs are `cargo test -p qsc`, same seat, same toolchain (`rustc 1.95.0`), same knobs.
**BEFORE** = the unrepaired tree at `788c8de5`, worktree clean, run to COMPLETION before any edit
existed — so a red anywhere afterwards is attributable, by design rather than by timestamp forensics.

| metric | BEFORE | AFTER | delta | directive expected |
|---|---|---|---|---|
| targets reporting | 132 | **133** | **+1** | +1 binary |
| integration test binaries | 129 | **130** | **+1** | +1 |
| tests passed | 615 | **622** | **+7** | +7 |
| tests failed | 0 | **0** | 0 | 0 |
| tests ignored | 2 | **2** | 0 | — |
| red targets | 0 | **0** | 0 | — |
| assertions retargeted | — | **0** | — | 0 |
| fixture lines changed | — | **1** | — | 1 |
| tests removed | — | **0** | — | 0 |

`BASELINE_RC=0` (17:45:58Z → 20:26:29Z) · `AFTER_RC=0` (20:55:22Z → 23:44:51Z).
**EVERY FIGURE MATCHES THE DIRECTIVE'S PREDICTED DELTA.** No mismatch, so no STOP is owed here.

⛳ **The new binary is green INSIDE the full suite, not merely in isolation** — `7 passed; 0 failed`
in 431.20 s, which is the stronger claim: no interference from the other 132 targets.
⛳ **`timeline_store` is green with the re-aimed fixture** — `6 passed; 0 failed` — so the one
committed test this lane touched passes in the suite that owns it.
⛳ **`receive_e2e.rs` passes**, which is a live check on §8.1's static discharge of its `:598` open
flag: the directive resolved it as UNAFFECTED on two static grounds, and execution agrees.

---

## §7. §7.7 — BOTH SHARD MANIFESTS, AND AN HONEST NOTE ON THE PLACEMENT METHOD

The consumer was **EXECUTED**, with a control that discriminates:

| run | rc | output |
|---|---|---|
| `qsc_shard_check.py` BEFORE the manifest edit — **NEGATIVE CONTROL** | **1** | `FAIL: MISSING from manifest: tests/na0741_frame_class_dispatch.rs` · census 133 / manifest 132 |
| same, macOS manifest — **NEGATIVE CONTROL** | **1** | same, census 133 / manifest 132 |
| `qsc_shard_check.py` after | **0** | `OK: manifest covers the census exactly` · census 133 / manifest 133 / missing 0 / unknown 0 |
| same, macOS manifest | **0** | `OK` · census 133 / manifest 133 / doc shard 4 with 0 co-tenants |

`grep -c '^tests/'` moves **129 → 130 on BOTH**, symmetric as §7.7 requires.

⚠⚠ **§7.7 ASKS FOR THE LIGHTEST SHARD "BY EACH MANIFEST'S OWN SEED FIGURES", AND NEITHER MANIFEST
CARRIES PER-TARGET FIGURES.** Measured: the Linux manifest states none at all; the macOS header
states its four working shards are balanced **to within 0.153 s of each other** — i.e. equal by
construction. Target COUNT is an *inverted* proxy: Linux shards 0/1/2 hold exactly **one** target
each precisely because those are the heaviest (`aws_file_medium_boundary_na0192a`,
`attachment_streaming_na0197c`, `na0688_c2_passivation`). ⇒ **no shard is identifiable as "lightest"
from either manifest's own bytes.** The placement is therefore **arbitrary within the stated
constraints, and is recorded as such rather than dressed as compliance**: Linux **shard 3**, macOS
**shard 1**; never the doc shard (Linux 11 / macOS 4), whose sole-tenancy invariant the checker
re-verified at 0 co-tenants both times.

⚠ **THE PERTURBATION IS STATED WITH A NUMBER RATHER THAN HIDDEN:** the new binary is **SLOW** —
measured **434.80 s** for its seven arms (two invite flows, several lease waits, ~20 party setups
each doing a PQ identity rotation). That is a heavy target, not a light one, and on a macOS
partition balanced to 0.153 s it is a real perturbation.

---

## §8. FINDINGS — THREE, NONE OF THEM TUNING

### F-A ⚠⚠ §5.1 DOES NOT PREVENT THE SILENT UNDER-DELIVERY WHEN THE PULL LEASE IS SHORTER THAN A PULL ROUND — a residual the directive does not name

Measured on the **repaired** tree with `pull_lease_secs=1`: four InviteResp frames at the head,
two real messages behind, `--max 4` ⇒ **16 `recv_frame_skipped` emissions for 4 frames**
(4 × `RECV_CONTROL_ROUNDS_MAX`), `recv_skip_summary count=16`, `recv_none`, **zero delivered, rc 0**.
The lease expires between rounds, the same head is redelivered, `want` is refilled by it every
round, and the tail is never reached.

⇒ **§5.1 converts an unbounded silent under-delivery into a BOUNDED re-skip spin that is still a
silent zero-delivery.** The repair's delivery guarantee is **conditional on `PULL_LEASE_SECS`
exceeding a pull round's duration**, and that condition was not stated anywhere in the design, the
directive or the ruling.

- **The shipped configuration is UNAFFECTED:** production runs `PULL_LEASE_SECS=60` ([O] provenance,
  NA-0740). Re-measured at 60 s, everything else identical: **exactly 4 skips, tail delivered, rc 0.**
- ⛳ **The diagnosis was written BEFORE the act with its refutation condition** (*"if it still emits
  16 the diagnosis is wrong and I re-diagnose rather than tune"*) — SR-16 row 51.
- T7 therefore runs at the production figure, and pins `count=4` **exactly**, so a regression into
  the spin fails the arm instead of passing it.

### F-B ⚠ THE DIRECTIVE'S §6 CLAIM ABOUT `id=` IS FALSE AS MEASURED — and a recorded diagnostic dies with it

§6 justifies the marker's `id=` field: *"`item.id` is relay-assigned and already emitted unredacted
by `recv_dup_skipped` (:527), `recv_item` (:1059) and `ack_replay_unrecoverable` (:1225-1229)."*
**Measured on every emission: `id=<redacted>`.** The marker layer redacts by VALUE SHAPE, not by key
— `should_redact_value` → `looks_high_cardinality` (`len() >= 24` and contains a digit) — and a relay
item id is a UUID.

- **The safety conclusion is unaffected and is strictly stronger than claimed.**
- ⚠ **But a consequence is lost.** F-20's marker tax was recorded as *"every redelivery emits a fresh
  `recv_frame_skipped` carrying the SAME `id=`"*. **An operator cannot observe that** — every id
  renders identically — so the tax is real and its diagnostic is blind. The count in
  `recv_skip_summary` is the only available discriminator, which is why T7 asserts it exactly.
- ⚠ **No source census could have caught this:** the claim was about RENDERED output and every
  instrument aimed at it was reading SOURCE. It surfaced on the first EXECUTION of the marker.

### F-C ⚠ WF-0087's PLANT HAZARD REACHES MARKER NAMES, NOT ONLY ID SPACES

§6 states both marker names *"MEASURED FREE (0 files tree-wide), confirmed twice"*. Re-measured at
the edit against `788c8de5`: `recv_skip_summary` = **0 files**, but **`recv_frame_skipped` = 3** —
`DECISIONS.md`, `NEXT_ACTIONS.md`, `docs/ops/IMPROVEMENT_LEDGER.md`. **All three are this lane's own
promotion records (#1756), which merged AFTER the directive's measurement at base `4c59ffda`.**
Classified rather than escalated: **zero occurrences under any source, test or script path**, which
is what the claim is for.

⇒ **A record of a measurement is an occurrence of everything the measurement names — and that now
demonstrably reaches MARKER NAMES, not just ruling ids.** The cure stays in the instrument: split the
census by PATH CLASS before reporting a total.

---

## §9. OBSERVED, NOT FIXED — outside this lane's authorized cargo

- **`PR #TBD` survives elsewhere in the tree** after rider (a) filled TRACEABILITY.md's:
  `NEXT_ACTIONS.md:2663` (*"Completed: 2026-01-20 — PR #TBD (merge TBD)"*),
  `docs/ops/ROLLING_OPERATIONS_JOURNAL.md:3931`, three archived testplans, and
  `docs/audit/AUDIT_CODE_ANALYSIS_STATUS_20260104.md` (×3). All historical and unrelated to NA-0740.
  **Reported, not repaired** — out of the authorized edit set.

---

## §10. RESIDUALS — carried forward, every one named

- **Unknown-class frames still wedge a batch.** The ruled trade: an attacker chooses their own
  leading bytes, so skipping Unknown buys no adversarial ground, while it would cost six committed
  assertions and the NA-0187 onboarding surface. T3 pins this as current behaviour.
- **The Legacy wedge persists** — non-default, explicit-flag lab mode; byte-unchanged in lane 1.
- **The three post-unpack frame-content aborts** (`:642`, `:682`, `:720`) survive; a
  session-authenticated peer can still abort a batch. They ride the §9.6 ledger amendment.
- **F-20's marker tax**, now with F-B's correction: the repetition is real and **unobservable**.
- **F-A's lease-dependency**, new in this lane.
- **Lane 2 still owes the producer acks**; until then the residue is re-skipped once per lease
  period for up to `retention.ttl_secs = 604800`.

---

## §11. CLAIM BOUNDARY

Loopback plain HTTP; in-process `qsl-server` (dev-dependency rev `131d63f4`); Linux
`rustc 1.95.0`; **zero secrets read, no relay contacted, no `sudo`, no
`qwork`/`qstart`/`qresume`/`qnext` run by this seat**. **NOT a CI claim** — CI is the PR's own
gates. ENG-0142's **closure boundary is the one already amended into the ledger by #1756**: lane 1
closes the non-adversarial trigger and foreign/unknown-class junk from any sender; message-class
frames, the three post-unpack aborts and Legacy's wedge **REMAIN OPEN**. **Severity re-grade of the
remainder is the operator's act at lane close.** ENG-0196 is LANE 2 and untouched; ENG-0191's (a)–(e)
stay UNRULED; ENG-0194 not repaired; ENG-0193 not built; WF-0086's and WF-0087's gates not built;
**#1745 — an ISSUE, not a PR — stays OPEN.** Nothing is merged: the operator merges, the seat does not.
