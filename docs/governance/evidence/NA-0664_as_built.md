# NA-0664 as built — macOS CI capacity correction + vault-read instrumentation (D600; D-1288 + D-1289 + D-1290)

Directive: QSL-DIR-2026-07-21-600, final sha256
`18e7d606827ece57d15f1436e4a27ab18e2b47d92e6812d58d5fb26392fa3800`, 822 lines,
APPROVED 2026-07-21 (F1 macOS ceiling 180; F2 Linux ceiling IN at 240; F3 SHIP
NOTHING; F4 two PRs ceiling-first), **AMENDED IN PLACE THREE TIMES DURING
EXECUTION**. The amendment chain is recorded because the directive cited in
D-1288 is not the directive that governed the lane's end:

| stage | sha256 | lines | what it added |
|---|---|---|---|
| at promotion | `6b99f250…` | 532 | the approved four-flag directive |
| at D-1288 | `9a26ffd3…` | 558 | (as cited in D-1288's own text) |
| final | `18e7d606…` | 822 | AMENDMENT 1 (Phase 3 four-bucket acceptance), AMENDMENT 2 (method + framing rulings R1–R4, incl. R1-a), AMENDMENT 3 (Option-A scope widening that inserted PR-1b) |

Base: main == origin/main == `67110be7` at closeout; the lane began from the
seating merge `32215b75`. Phase 0 re-derived live at closeout by a fresh
session: main sha, `STATE` line count == 1, highest decision, highest ledger
item, zero open PRs, and a clean worktree — **all confirmed before any edit**.

## 1. WHAT SHIPPED — AND WHAT DELIBERATELY DID NOT

**THREE PRs, not the drafted two.** The lane broke main and repaired it in
flight.

- **PR #1613 → merge `ca6897fc` (D-1288)** — `macos-qsc-full-serial`
  `timeout-minutes` **120 → 180**; `qsc-linux-full-suite` gains an explicit
  **`timeout-minutes: 240`** replacing the inherited 360-minute GitHub default;
  plus the ENG-0052/ENG-0053 ledger amendment owed from NA-0663.
- **PR #1614 → merge `67110be7` (D-1289)** — the `public-safety` watchdog budget
  **DERIVED from the ceilings it waits on**, repairing a red main this lane
  itself shipped.
- **This closeout (D-1290)** — the Phase 3 verdict and the ledger disposition.

**NOTHING WAS FIXED AND NO REMEDY WAS CHOSEN.** No performance fix, no cache, no
cached key handle, no cached parsed store, no change to `vault::secret_get` or
`relay_ca_file()` behavior. **The F3 ship-nothing ruling means this lane's
product is a written verdict, not shipped timing code.**

## 2. THE CEILINGS — VERIFIED IN FORCE BY THE MERGES THAT LANDED THEM

The ceiling-first ordering was **load-bearing, not stylistic**: because
`.github/workflows/*` classifies `docs_only=false`
(`scripts/ci/classify_ci_scope.sh:20,61-62`), PR-1's own merge was **the first
genuine exercise of `macos-qsc-full-serial` since NA-0663 and the first ever
under the corrected ceiling**. Verified live on the PR: `docs_only=false,
workflow_security=true, runtime_critical=false`, with exactly the two
push-only suites skipping on the PR side and running on merge.

| suite | ceiling | observed | utilisation |
|---|---|---|---|
| `macos-qsc-full-serial` | **180** | **132m15s** job / 130m00s step | **73.5%**, 47m45s headroom — **PASSED** |
| `qsc-linux-full-suite` | **240** | **157m45s** | **65.7%** |

**Against the rejected alternatives**, the same observed run would have been
**110.2%** of the old 120 ceiling (cancelled again) and **88.2%** of a 150
ceiling — **numerically the identical thin margin the item was filed about**,
which is exactly why 150 was rejected at F1.

**THE EXTRAPOLATION THAT JUSTIFIED 180 UNDERSHOT, AND IT IS RECORDED AS A RESULT.**
Applying Linux's +17.3% pre/post delta to the macOS baseline projected a
**121m31s** step; the observed step was **130m00s** — **8m29s low**. The
cross-platform extrapolation was directionally right and **quantitatively
optimistic in the unsafe direction**. Absolute added cost was close on both
platforms (macOS step +26m24s vs Linux job +23m13s, **ratio 1.14**), consistent
with a **compute-bound, platform-invariant** regression. **A future reader must
not treat a cross-platform percentage extrapolation as a measurement.**

## 3. THE THIRD BUDGET — A BREAKAGE THIS LANE SHIPPED

D-1288 corrected two CI budgets and left a **third** untouched:
`public-safety`'s `--max-iterations 390` (~130 min at 20 s), which **appeared in
no filing, no directive, and no analysis**. It became binding the instant the
first two were corrected: on `ca6897fc` **both suites PASSED** while the
watchdog **exhausted 390/390 at 19:13:03Z** with Linux still `in_progress` —
that suite succeeded **20m52s later**. `public-safety` is REQUIRED, so **main
went RED on a merge where nothing was broken.**

**THE REPAIR IS DERIVATION, NOT A BUMP** (operator-directed: *"DO NOT SIMPLY BUMP
390 TO 696 — a hardcoded iteration count that must be manually kept in sync with
two independent ceilings is the identical defect one more time"*). The step now
**reads both ceilings from the checked-out workflow files at run time**, takes
the maximum, adds a stated **60-minute** queue/jitter margin, and computes
`--max-iterations`: **240 + 60 = 300m coverage = 900 iterations**. Raising either
ceiling now raises the watchdog automatically. Two failure modes fail **loudly
and immediately** rather than silently 130 minutes later: a **missing ceiling**
(the budget cannot be derived) and a **ceiling this job cannot cover** (derived
coverage exceeding its own inherited 360-minute default). Extraction is
dependency-free `awk`, cross-checked against a YAML parse; all three paths —
happy, ceiling-too-large, ceiling-removed — were executed locally before the PR.

## 4. THE MEASUREMENT VERDICT (ENG-0053)

**Release profile, `key_source = 1` (passphrase), REPS=12, median, two store
sizes so the fixed floor separates from the growth term.**

| bucket | Regime A (153 B) | Regime B (414,592 B, 4000 msgs) |
|---|---|---|
| wall-clock `secret_get` | **18.137 ms** | **18.554 ms** |
| (a) file read | 0.014 ms (0.1%) | 0.054 ms (0.3%) |
| **(b) Argon2id** | **17.675 ms (97.4%)** | **17.703 ms (95.4%)** |
| (c) AEAD decrypt | 0.004 ms (0.0%) | 0.289 ms (1.6%) |
| (d-outer) payload parse | 0.001 ms (0.0%) | 0.759 ms (4.1%) |
| **attributed** | **97.6%** | **101.4%** |
| (d-inner) timeline parse | 0.000 ms | **2.009 ms** (outside `secret_get`) |

- **FLOOR vs GROWTH, separately stated (the acceptance requirement):** the
  Argon2id floor is **FLAT at ~17.7 ms across a 2700x store-size increase**;
  the growth term (c)+(d-outer) moves **0.005 → 1.048 ms** — **~6% of the call**
  at 4000 messages.
- **The (c)/(d) split is REQUIRED and is not even: (d) outweighs (c) by ~2.6x.**
  Deserialization, not decryption, is the larger half. **(d-inner) at 2.009 ms
  exceeds (c)+(d-outer) combined** and has a different remedy.
- **Tier 1 multiplicity: EXACTLY 1** `secret_get` per `relay_http_client()`
  construction, **perfectly linear at N=10** — so **"call it less" is not the
  remedy**, and the per-call split is the whole story.
- **Counter validation PASSED before any counter-derived claim rested on it:**
  12 calls → `kdf=+12 reads=+12 decrypts=+12`, every run, both regimes, both
  profiles. **Timing never had to override the counters.** Recorded because
  `perf_snapshot()` had **zero prior consumers** and `PERF_VAULT_FILE_READS`
  counts **attempts, not successes**.
- **`key_source` is STATED and does not generalize:** under `key_source = 2`
  (keychain) no KDF runs at all, so bucket (b) ≈ 0 — **~95-97% of the measured
  cost disappears** and the successor's remedy differs.

### 4a. THE HEADLINE CORRECTION — ~350-400 ms IS A DEBUG FIGURE

**Release is ~18 ms per `secret_get`, ~95-97% of it Argon2id.** Confirmed three
independent ways: the NA-0663 probe binary carries `.debug_info` at 89,332,792 B
against a release `qsc` of 8,850,832 B (**10x**); **both CI suites run their
TESTS in debug** (`macos-qsc-full-serial` has no `--release`;
`qsc-linux-full-suite` builds release then tests debug); and the debug/release
ratio **401.310 / 18.137 = 22.1x** accounts for the observed ~21x gap **with
nothing left over**.

**This CLOSES the 20x discrepancy the apportionment run had left explicitly
open, and SUPERSEDES that artifact's "open problem" section.** Of its three
candidates, the non-release probe binary is **CONFIRMED**; the other two are
rendered **unnecessary, NOT refuted**.

**Consequences, stated plainly:** ENG-0053's **product-facing severity was
OVERSTATED** — by the filing, by the operator's ranking, and by the measuring
executor's own hypothesis. **NO READER MAY CARRY 350 ms INTO A PRODUCT
DECISION**, and the ledger heading now says so. **The debug figure remains the
correct figure for CI**, which genuinely runs debug — so **NO CEILING CHANGE
FOLLOWS FROM THIS VERDICT**; the 180/240 ceilings were derived from debug-profile
CI wall-clock and stand as set.

### 4b. WRITE PATH AND APPEND ARITHMETIC

`secret_set` costs **20.014 ms** (empty) / **22.142 ms** (414 KB) — the write
adds **~2.0-2.6 ms** over a read, **dominated by the atomic whole-file write at
~1.9-2.0 ms, which is FLAT with store size** (fsync-bound, not size-bound). **An
operator prediction that the whole-file write would matter at scale is
CORRECTED: store size does not punish the write path** at these scales. The
undercount concern was RIGHT; its **attribution** was wrong — the missing cost
was (e) serialize, (f) encrypt, and the inner timeline serialize.

A `timeline_append_entry` performs **TWO Argon2id derivations**, 2 outer parses,
1 inner parse, 1 inner serialize, and one each of (e)/(f)/(g). **Measured at
414 KB: ~43.2 ms.** Extrapolated to ~5 MB / ~50,000 messages: **~103-105 ms**.

**⚠ AN ARITHMETIC CORRECTION THAT CHANGES THE CONCLUSION: Argon2id is 81.9% of
an append at 414 KB, not ~42%.** The erroneous figure divided **one** derivation
by the append total; an append performs **two** (35.406 / 43.245 = **81.9%**). At
~5 MB the share falls to ~34%. **At today's realistic store sizes Argon2id does
not merely lead an append — it DOMINATES it**, which materially strengthens the
envelope-encryption option.

**ASSUMPTION, NOT MEASUREMENT:** extrapolating the atomic write as flat out to
5 MB is **unverified**; measure it there if a precise figure is needed.

### 4c. SUSPECTED ROOT CAUSE — LABELLED SUSPECTED, AND ITS FALSIFIER WAS TESTED

**Structural inspection plus inference, NOT the measured verdict.**
`secret_get` (`vault/mod.rs:213`) has **no session**: it is
`load_vault_runtime()` → `decrypt_payload()` → one `BTreeMap` lookup — a full
`fs::read` + envelope parse + complete Argon2id derivation (19456 KiB, t=2, p=1)
+ whole-store AEAD decrypt + whole-store `serde_json::from_slice`. **A COLD OPEN,
PER CALL**, to serve a hash lookup. **`VaultSession` (`:725`) already holds
exactly the right state with a correct wiping `Drop` (`:733`)** and is used by
the WRITE path and by `lock()` — **the READ path bypasses it entirely.** Probable
origin: CLI-era code whose **contract never changed but whose CALLER PROFILE
did** (D582 made `qsc` a linkable library; NA-0663 put it behind a constructor
used at all eight relay-client sites) — the same family as the other D582
residue.

**THE ACCOUNT CARRIED AN EXPLICIT FALSIFIER AND IT WAS TESTED.** The stated
condition was that a Tier 1 multiplicity of **3-4 rather than 1** would shift the
arithmetic and force revision. **Tier 1 measured exactly 1. The account
SURVIVED.** The measurement **corroborates the structure and refutes only its
implied emphasis**: the cold open is real, but its cost is dominated by the
**KDF**, not by the store-size-dependent reconstruction the account emphasised.

## 5. LEDGER DISPOSITION

- **ENG-0052 — clauses (a)/(b)/(c) DISCHARGED; clause (d) OWED.** The periodic
  scheduled exercise is **still absent**, so the **masking note is unretired**:
  governance merges still skip both suites and **main's green history is still
  not evidence about ceiling health.** The defect class is recorded with **ALL
  FIVE instances** — the macOS 120 ceiling, Linux's inherited 360 default, the
  watchdog's 390 budget, the incident-specific repair profile, and the
  bootstrap's advisories-only trigger (**RULED IN by the operator as the fifth**).
  The last two show the class is **not confined to numbers or to CI timing**.
  **Recorded as a limitation, not a success: only instance 3 was given a
  derivation; instances 1 and 2 were corrected to better LITERALS and remain
  literals today.**
- **ENG-0053** — updated with the verdict, the debug-vs-release correction (in
  the heading as well as the body), the corrected append arithmetic, and the
  **remedy space RECORDED AS OPTIONS WITH NOTHING DECIDED**: envelope encryption
  leads on the merits (it targets the ~95-97% term directly; its true cost is a
  **vault FORMAT change with migration**), and the prior objection that it
  "targets the wrong term" is **REFUTED by this lane's data**.
  **⚠ ITS DEADLINE IS SET BY THE FORMAT CHANGE, NOT BY THE MILLISECONDS.**
  Migration is **FREE TODAY — no users, no vaults in the field.** After public
  release the identical change needs **version detection, a migration path,
  testing against old vaults, and a failure mode where a user's vault will not
  open** — on a product whose premise is that **there is no recovery**. The cost
  **steps up sharply and permanently at first release**. Recorded explicitly
  because **"~18 ms, low priority" WILL READ AS "NEVER"**: on latency alone this
  item defers forever and each deferral looks reasonable. **The format argument
  is what makes it a PRE-RELEASE item rather than an indefinite one — the window
  in which it is cheap CLOSES AT PUBLIC RELEASE.**
- **ENG-0055** — a **RECORDING-ONLY** bullet widening the census scope.
  `vault/mod.rs` has **exactly one `impl Drop`**, on `VaultSession`, which
  **`secret_get` does not use**; `VaultRuntime` (`:717`) holds the 32-byte
  derived key and `VaultPayload` (`:74`) the full corpus, **both without `Drop`**,
  with **zero `ZeroizeOnDrop`** in the module — so **every `secret_get` abandons
  the entire plaintext secret corpus unwiped on the hot path.** That entry's own
  "the house DOES zeroize at the vault seam" claim is **`VaultSession`-ONLY and
  false for the read path**. The scope widens **independently of whether any
  cache ever lands**. **The tradeoff INVERTS:** today **N** abandoned unwiped
  copies versus **ONE** wipeable owned copy with a cache — **caching may IMPROVE
  memory hygiene**, and "cache = corpus in the clear" is **WITHDRAWN**.
  **The framing that matters: the hardened path and the hot path have DIVERGED —
  `lock()` wipes deliberately, `secret_get` wipes nothing, in the same module.**
- **ENG-0059 — FILED.** Both sanctioned routes from a red main back to green are
  unusable, for unrelated reasons, and **neither had ever been exercised**.

## 6. BOUNDS HONORED

**ZERO `src/` paths in all three PR diffs.** The F3 ship-nothing ruling meant the
measurement's product is a written verdict; the temporary in-crate measurement
edits were **reverted from byte-exact copies** and their absence proven by
`git diff --stat`, verified **before and after** the closeout patch.

No performance fix, no cache, no cached key handle, no cached parsed store, no
change to `vault::secret_get` or `relay_ca_file()` behavior, no change to
`docs_only` gating or the `pull_request` exclusions, no scheduled/periodic
trigger (clause (d) is **not** this lane), no protocol/wire/crypto change, no
`qsl-desktop` touch, no relay change. NA-0663's proof root was treated as
read-only. Raw private values remained proof-root-only.

**A green main-push under the new ceilings is NOT evidence that ENG-0053 is
resolved, and nothing in this lane asserts that it is.**

### 6a. ⚑ A STAGING HAZARD THE NEXT EXECUTOR WILL HIT

**This document was silently absent from the first staged diff.**
`docs/governance/evidence/` is ignored by `.gitignore:65` (`**/evidence/`), so it
required `git add -f` — **and it bit a SECOND time on the amend**, because the
ignore rule applies to **every** commit carrying the file, not only the first.

**Not a one-off: the journal records AT LEAST 31 prior instances** of this
footgun actually biting — hand-verified at the line level, spanning **NA-0245
through NA-0580**. **31 is a FLOOR, not a point estimate:** successive keyword
searches returned 41, 42, 175 and 35 depending on marker framing, so no single
figure survives scrutiny and none is cited as one. **The recurrence is
unambiguous; its exact magnitude is not.** **A convention forgotten at least 31
recorded times is a defect with a workaround, not a mitigation.** **⚠ THE
ARGUMENT IS UNCHANGED BY THE NUMBER — it was never rate-dependent:** a defect
recovered flawlessly every time generates no pressure to fix it, which holds at
31 as it did at the erroneous 41 and would hold at 5. **The mechanism is the
silence of the failure mode, not the frequency.**

**A SEPARATE FINDING CAME OUT OF COUNTING IT:** four reasonable marker framings
over the same journal returned **41 / 42 / 175 / 35**, because the journal has
**no consistent marker vocabulary for recovery events** — so it **cannot answer
"how often has X happened" for ANY class of event**, and prioritisation
arguments of the form "this has happened N times" are not supportable from it
without hand-inspection. **Recorded, not acted on** (see §7 item 5b).

Recorded against **WF-0016**, which names the footgun but frames it as specific
to design-lock handoffs — **this instance was an as-built**, so it covers every
artifact under `evidence/`. **And the reframing that makes it cheap: if the
convention is "always `git add -f`", the ignore rule is wrong** — a path
unconditionally force-added is not a path anyone intends to ignore. Either
`.gitignore:65` is too broad and wants a negation, or evidence should genuinely
not be tracked and the convention contradicts that intent. **Both are one-line
answers.** **RECORDING ONLY — `.gitignore` was not in this lane's scope.**

> **AFTER every `git commit` or `git commit --amend` that should carry an
> evidence document, re-run `git diff --stat` against the base and confirm the
> path is present. Staging success is not proof — the file disappears without an
> error.**

## 7. OWED OUT OF THIS LANE — STATED SO NOBODY MISTAKES IT FOR COMPLETE

1. **The MEASURED `secret_get` count per FULL relay operation (send/pull/ack).**
   Tier 1 measured per **construction** (= 1) and the debug verdict explains the
   **marginal** cost of the one call NA-0663 added — **together those settle the
   REGRESSION question. They do NOT give the TOTAL per-operation vault cost,
   which is what sizes the remedy.** The **16 call sites tree-wide** (transport 4,
   contacts 3, identity 2, protocol_state 2, timeline 1, attachments 1,
   handshake 1, lib 1) are **STATIC INSPECTION, NOT MEASUREMENT**, and are
   labelled so everywhere they appear.

   **⚠ INFERENCE — ARITHMETIC SHOWN, EXPLICITLY NOT MEASURED, NOT TO BE CITED AS
   A RESULT.** *If* an operation touches even **5–10** of the 16 sites, then at
   the measured **~18.1 ms** per `secret_get` the total is **~90–181 ms of vault
   cost per relay operation**, of which **~86–172 ms is Argon2id**. **The 5–10
   figure is an assumption about call-graph reachability, not an observation** —
   nothing in this lane bounds how many sites one operation reaches, and the true
   number may be lower (short-circuits, cached higher-level state) or higher
   (loops, retries, per-contact iteration). **If it holds, it MATERIALLY CHANGES
   REMEDY SIZING and strengthens BOTH leading options**: client reuse saves in
   proportion to constructions avoided rather than a flat ~18 ms, and envelope
   encryption removes the Argon2id term from **every** call in the operation
   rather than from one. **A remedy chosen against the per-call figure alone
   would be scoped against the smaller number.**

   **Judged NOT CHEAP and deliberately carried rather than guessed**, on three
   independent grounds: `perf_snapshot()` has **zero consumers**, so reading it
   needs a temporary source bridge; its counters are **process-global** while the
   only working full-operation harness (`tests/same_host_client_to_client_e2e.rs`)
   drives the CLI as a **subprocess**; and `relay_send` requires an **established
   two-party protocol session** (contact trust + `protocol_active`), so an
   in-process measurement means reproducing setup that exists only as CLI
   choreography. **It does not fall out of a counter read — budget for it.**
2. **The `key_source = 2` (keychain) profile** — bucket (b) ≈ 0 there.
3. **ENG-0052 clause (d)** — the periodic scheduled exercise.
4. **The choice of remedy** — options recorded, none chosen.
5. **The CLAUDE.md `## Session procedure` capture** of the operator-relay and
   proactive-observations conventions (a docs-only LITE lane).
5a. **Whether `Sequencing` should be FORMALIZED in DOC-AUD-001 §6 as a
   first-class ledger field**, so deadline-bearing items are findable by
   **scanning** rather than by reading every entry. The mechanism already exists
   as an informal bullet convention — **ENG-0054** ("resolve BEFORE reviewer
   outreach"), **ENG-0058** ("settle BEFORE GUI slice B's Logs pane is drafted"),
   and now **ENG-0053** (the pre-release envelope-encryption deadline) — **three
   items want it**, and it is neither first-class nor scannable. **A
   governance-schema question, RECORDED AS OWED, NOT ACTED ON**; plausibly the
   same LITE lane as item 5.
5b. **Whether the journal needs a CONTROLLED MARKER VOCABULARY for recovery
   events**, so that "how often has this class of failure recurred" is answerable
   by search rather than by hand-inspection. Found by accident while counting one
   hazard: four framings returned **41 / 42 / 175 / 35** over the same corpus.
   The journal records recoveries under at least a dozen headings with **no
   controlled field** distinguishing *a defect that bit* from *a hazard correctly
   anticipated* from *an unrelated one-off*. **Consequence: the journal is
   excellent as a narrative record and unreliable as a countable one.**
   **RECORDED AS OWED, NOT ACTED ON** — plausibly the same governance-schema LITE
   lane as items 5 and 5a.
6. **A precise 5 MB atomic-write figure**, if one is ever needed — the flat
   extrapolation is unverified.

## 8. METHOD — THE MOST VALUABLE RESULT

> **⚑ A measurement that lands suspiciously close to the expected value gets a
> second look UNDER CHANGED CONDITIONS before it is reported, not after.**

The first apportionment run was a **debug** build giving **401.3 ms/call**
against an expected **350-400 ms** — a near-exact match to the hypothesis.
**A result that CONFIRMS the hypothesis is the hardest kind to catch, because
nothing feels wrong.** Reporting it would have produced a **confident wrong
verdict** steering the successor toward the store-size framing and **away from
Argon2id, where the cost actually is.** What caught it was re-running under
**release — a changed condition, not extra scrutiny.** Recorded as a standing
house rule in `docs/ops/ROLLING_OPERATIONS_JOURNAL.md`.

### 8a. A RECORD-KEEPING DEFECT FOUND AT CLOSEOUT

**The operator-relay headers written during this lane run 8–26 minutes AHEAD of
their own file mtimes** (the session handoff is headed `22:05:00Z`, written
`21:43:52Z`), from composing a header time rather than reading the clock.
**CONSEQUENCE: every relay header timestamp in this lane's record is an ESTIMATE,
NOT AN OBSERVATION, and nothing may rest on one.** **PRACTICE CHANGE, inherited
by every lane: read the clock at write time (`date -u`); do not compose a header
time.**

**The re-run sanctions are unaffected**, because they were checked against the
filesystem: both pre-result justifications were on disk before their triggers
(by **35 s** and by **8 s**). **But the 8-second margin is thin and must not be
presented as decisive on its own.** **The primary anti-reconstruction evidence is
the CONTENT of the predictions, not their timestamps** — each names a
**falsifiable band** ("within the first poll iterations… well under a minute"
against an observed 18 s; "~24 s, single main-state query, not a poll" against an
observed 24 s), and **someone writing backwards from a known result names the
result.** The timestamps corroborate that reading; they do not carry it.

## 9. EVIDENCE

Proof root:
`/srv/qbuild/tmp/NA0664_macos_ceiling_and_vault_read_instrumentation_20260721T155548Z/`
— **22 artifacts**. Measurements: `VERDICT_DEBUG_PROFILE_CONFIRMED.md`,
`APPORTIONMENT_RESULTS.md` (whose "open problem" section is superseded by the
verdict), `WRITE_PATH_MEASURED.md`, `TIER1_MULTIPLICITY.txt`. CI facts:
`MACOS_OBSERVED_DURATION.txt`, `PR1_TERMINAL_RESULTS.txt`, `_watchdog_tail.txt`,
`PR1_*` diff/classifier/check artifacts, `seating_merge_runs.txt`. Governance:
`REMEDY_SPACE_AND_METHOD_RULE.md`, `ENG0059_PENDING_FILING.md`,
`BOOTSTRAP_INELIGIBLE_STOP.md`, `JOURNAL_ENTRY_PENDING_RERUN_SANCTION.md`, and
the four re-run sanction artifacts whose **pre-trigger mtimes were independently
re-verified at closeout**.
