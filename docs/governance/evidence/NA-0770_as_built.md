# NA-0770 Lane A — AS BUILT: the legacy retirement

Base: qsl-protocol main `3b685e7933d7a6dcabf0feeb7817833b25de6914`, re-derived **bare and unpiped**
at the NAMED `github` remote with all 40 digits compared. ⚠ The seat's local `main` ref read
`241eec97c82763e4b5d03819faf1fafa2017255b` — **stale, by exactly the mirror trap this program has
recorded before.** Measured rather than assumed: `3b685e79` **is** an ancestor of this branch, and
`git log HEAD..3b685e79` is **empty**, so the branch contains all of main and adds two commits.

**Client-only. Zero relay bytes, zero desktop bytes, no pin moved, no `.github/**`, no secrets.**
Decision `D-1411`. Nothing merged — the operator merges.

## 1. THE PREMISE, MEASURED — NOT INFERRED

The brief's sec 3(a) asked **why `AckMode::Legacy` exists** and required the instruments that failed
to find a rationale be named. A rationale **was** found — in the comment written when the enum was
born — and the amendment (which ruled that a found rationale is no longer a stop) required it
carried verbatim beside the two grounds for retiring it anyway:

1. **delete-on-pull is the wrong delivery guarantee** — at-most-once, where the product needs
   at-least-once;
2. **it is structurally incompatible with the delivery ladder's rung 3.**

Both grounds stand independent of the rationale. The rationale explains the past; it does not
defend the future. ⚠ This is recorded because the honest answer to "why does this exist?" was
**"for a reason, and the reason is no longer good"** — not the easier "for no reason".

## 2. THE SHAPE: RETIRE THE MODE, TOMBSTONE THE KEY

A key that is **silently ignored is worse than one that is refused**, because the operator who set
it believes it is in force. So:

- the **writer refuses by name** — `config set ack-mode` emits `config_set_refused` carrying
  key/reason/file/remedy, then returns an error;
- the **reader reports a third state** — `AckModeConfigState::{Nothing, RetiredKeyPresent(String)}`,
  announced through `announce_retired_ack_mode_key()` with the **raw value the operator wrote**, so
  a stale config is visible rather than inert;
- `config_get` reports `state=retired_present|absent`, `retired=true`, `effect=ignored`.

⚠ **`Option<T>` WAS REFUSED DELIBERATELY.** It cannot distinguish "no key" from "key present,
ignored" — which is exactly the distinction the tombstone exists to make.

## 3. THE TEN PRODUCT FILES, AND WHY THERE IS NO ELEVENTH

`fs_store/mod.rs` (the tombstone choke point) · `cmd/mod.rs` (the `AckMode` enum and the
`--ack-mode` flag) · `lib.rs` (resolution replaced; refusal; reporting) · `transport/mod.rs` (the
two-arm URL collapsed to one `format!` carrying `&ack=lease`; the S-13 `recv_ack_mode` marker KEPT
with its gate removed) · `invite/mod.rs` · `handshake/mod.rs` · `main.rs` · `store/mod.rs` ·
`dedup/mod.rs` · `msgqueue/mod.rs`.

⚠ **THE NEAR-TRIGGER WAS MEASURED, NOT OBEYED.** The refusal appeared to need a new `ErrorCode`
variant in `model/mod.rs` — an eleventh file against an authorised set of ten, which the armed
trigger would have made a STOP. Reading `CliError::code`'s signature first showed it takes
`impl Into<String>`, so no new variant was needed. **The instrument was reading the constructor's
bytes before designing against a remembered shape.** Ledger row 364.

## 4. THE TWO CLAIM-ONLY FILES — THE FIGURE, WITH ITS INSTRUMENT (`WF-0087`)

`dedup/mod.rs` and `msgqueue/mod.rs` are in the edit set for **stale claims**, not for code.

| needle (the token set, stated because a figure carries its instrument) | dedup | msgqueue | positive control `lib.rs` |
|---|---|---|---|
| **IDENTIFIER needle** — `(AckMode\|ack_mode\|ack-mode)` | **0** | **0** | **19** |
| **UNION needle** — the above **plus `[Ll]egacy`** | **1** (`:11`, *"Legacy mode never constructs this store"*) | 0 | — |

⚠⚠ **THE "0 TOKENS" FIGURE IS TRUE OF THE IDENTIFIER NEEDLE AND NOT OF THE UNION NEEDLE, AND
SAYING WHICH IS THE WHOLE POINT.** The specification's row read *"ANY token needle — measures 0"*.
That is **false as written** for the union needle. This is a **precision item, not a defect**: both
files are correctly in the edit set, and both stale lines were found and corrected. Recorded here
because a bare "0" with no token set named is precisely the kind of figure that cannot be checked.

## 5. THE DISPOSITIONS — 41 BREAKS OF 68 TESTS ACROSS 14 FILES

`3 D-RETIRE + 25 D-REAIM-MECH + 1 D-REAIM-VALUE + 1 D-REAIM-NOASSERT + 11 D-NAMED-LOSS = 41`.

**Measured, not asserted:** net `#[test]` delta **−4** against the pre-edit tree, and **every file's
delta landed on its predicted row**. The four removals are the three D-RETIRE plus `r3b`, retired as
a CAPABILITY. Spot-check per the checklist: **no `assert` line was deleted in any D-REAIM-MECH
file** — the only assert-line losses are at named-loss sites, and one apparent anomaly resolved on
inspection (`NA_0640`'s deleted test delegates every assertion to a shared helper, so it genuinely
carried none of its own).

## 6. WHAT WAS REBUILT RATHER THAN MOURNED

The legacy control legs were this suite's **negative capability** — the demonstration that a
negative result was obtainable at all. Where it could be rebuilt without the retired mode, it was.

**THE IN-LEASE PROBE (`q4a`/`q4b`).** Under lease a pulled-but-unacked item is held **invisible**
until expiry. So each arm probes the SAME plant twice: once **inside** the window, where it is
genuinely unrecoverable, and once **after** expiry, where it must return. Mode-free, no test-only
seam, only shipped behaviour.

⚠⚠ **IT IS NOT THE SAME FACT, AND THE FILES SAY SO.** The old leg demonstrated **DESTRUCTION**; the
new one demonstrates **INVISIBILITY**. Both make the probe return `false`, which is what the arm
needs — but they are not interchangeable evidence, and no reader may cite these files as showing a
collateral pull can still destroy anything.

⚠ **THE WINDOW IS ASSERTED, NOT ASSUMED.** Each arm times its in-lease probe against
`LEASE_DURATION` and fails **loudly, with the remedy in its message**, if the probe overran — rather
than silently converting a slow box into a false negative-capability result. The clock starts
**before** the command, deliberately: the relay's lease begins when it serves the pull, somewhere
inside that command's runtime, so timing from before it **overstates** elapsed lease and the
assertion errs in the safe direction.

This reconstruction is the third option the Director's two-member option set could not contain
(ledger row 362).

## 7. THE SEVEN NAMED LOSSES — RECORDED AT THEIR SITES, NOT ONLY IN THE RECORD

`L1` `ratchet_step:265`'s hard-exit assertion (⚠ **:265, not :266** — the specification's line was
corrected by the Director) · `L1b` the same loss in a second file (`aws…:507`) · `L1c` **the same
loss in a THIRD file**, found only because the seat **stopped** at a contested disposition rather
than reclassifying under momentum (`file_transfer_mvp:901`) · `L2` `na0741`'s **sealed E5** · `L3`
**the drain** · `L4` `NA_0671`'s ACK−PULL **marginal**, structurally 0 once re-aimed · `L5` `t7`'s
Legacy half, the only empirical pin on the `1` of the 1→16 floor · `L6` the tree's last **negative**
pull-URL observation.

⚠⚠ **`L3` OUTLIVES ITS TEST AND IS THE ONE TO READ TWICE.** `--ack-mode legacy` was the tree's only
way to evict a poison frame from a relay mailbox. Measured across the whole CLI: `QuarantineCmd` is
a **local** store, `Outbox Discard` is the **outbound** queue, `relay_inbox_ack` is unreachable for
a frame the client cannot persist. ⇒ **an undecodable frame at a relay-mailbox head now redelivers
forever with no operator remedy.** This is a **consequence of removing a mode that was standing in
for an eviction verb**, not a defect this lane introduced — the verb never existed. It strengthens
the case for scheduling the `ENG-0142`/`ENG-0198` wedge repair, and any eviction verb built for it
must add itself to `G-B`'s asserted caller set or turn it red.

## 8. THE GUARDS — BOTH ARMS PRINTED

`tests/na0770_legacy_retirement_guards.rs`. **G-A**: the pull URL has exactly ONE construction and
it carries `&ack=lease` — **singleness is the assertion**. **G-B**: `transport::producer_ack` is
reached through exactly TWO wrappers, by name — **identity, not a budget.**

| arm | tamper | result |
|---|---|---|
| baseline | none | both green |
| G-A singleness | a second `format!("{}/v1/pull?…")` added | **RED** — *"SINGLENESS IS THE ASSERTION … Found 2"* |
| G-A lease | `&ack=lease` → `&ack=legacy` | **RED** — *"…no longer carries `&ack=lease`"* |
| G-B identity | a third caller added in `store/mod.rs` | **RED** — permitted vs measured printed, naming the intruder |

Each tamper was **verified to have landed** before its result was believed, and the tree was
confirmed clean after each (`git status --porcelain` empty).

⚠ **THE EXCLUSIONS ARE CHARACTERIZED, NOT LINE-PINNED.** `adversarial/route.rs`'s `/v1/pull?`
occurrences are raw `b"GET ` HTTP request bytes driving the relay's own parser. Excluding them by
**line number** would have rotted the moment a line moved — this lane already produced one such
stale cite. They are excluded by **shape**, anything else in that file still fails the guard, and
the guard asserts those fixtures still exist so its own exclusion cannot become silently vacuous.

⚠ **G-A WENT RED ON ITS FIRST RUN AGAINST ITS OWN DESCRIPTION** — the comment written beside the
construction it guards matched its needle. Prose cannot construct a URL; comment lines are now
skipped. Recorded because it is the **fail-closed** direction.

## 9. WHAT THIS LANE DID NOT MEASURE, STATED PLAINLY

- **No relay was contacted for traffic; no field machine was touched.** The relay figures
  (`RETENTION_TTL_SECS=604800`, `PULL_LEASE_SECS=60`, and **neither field machine carries an
  ack-mode config**) are the **operator's word of 2026-08-28**, recorded as such — not this seat's
  measurement. They are what makes `ENG-0253`/`ENG-0254` **latent, never live**.
- **The desktop repository was not opened.**
- **`ENG-0255` was filed from source reading**, both lines read whole with their enclosing
  signatures. **It was not reproduced at runtime** and this file does not claim it was.
- ⚠⚠ **TWO GOVERNING RULINGS WERE NEVER BANKED AS FILES.** The product-deletion checkpoint ruling
  and the contested-disposition ruling arrived as pastes. `RULING_NA0770_001`–`005` are banked 444;
  those two are not. ⇒ **the Director's two independent sweep needles, which that ruling reports as
  finding NO further contested sites, exist only in the paste and cannot be quoted from an
  artifact here.** Stated as a gap rather than reconstructed from memory. Banking them is owed.

## 10. THE METHOD FAILURES THIS LANE PAID FOR

- **The launcher defect.** `qsc_shard_check.py --emit-args` writes headers to **stderr** and args to
  **stdout**; the launcher suppressed stderr then trimmed the one remaining line to **empty**, and
  an empty arg list makes cargo select **everything** — 147 targets, twelve-wide on six cores.
  ⚠ The tell was printed for a full cycle before it was read: every shard reported *"6 targets, 1
  FAILED"*, **including the shard assigned exactly one target**. Cured with an explicit non-empty
  assertion that aborts the shard (`rc=97`) rather than silently running the suite.
- **The stop that failed its own audit.** STOP 004 was believed complete; the mechanical
  self-containment audit found **five gaps** on its **sixth** asking in one lane, one of them on an
  item the governing brief **orders**. Re-assembled whole as STOP 005 — *a supplement chain is not a
  stop file*. This is the measurement `SR-26` was minted on.
- **Flag-removal residue.** Removing a flag leaves three kinds of residue and only two are
  compiler-visible: the emitted args, the now-unused **parameter**, and the `mut` that existed only
  to `push` it. ⚠ **Neither the compiler nor a signature sweep sees a caller still passing a
  now-ignored literal** — `na0741` was passing `"legacy"` into a dead parameter, so that arm would
  have run under **lease** and failed at runtime having compiled cleanly. The flag sweep and the
  value sweep are two separate passes.
