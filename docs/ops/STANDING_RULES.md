# QSL STANDING RULES — v1 (drafted 2026-08-05, Slice-E execution window)
**Purpose:** one file consolidating the method rules that currently live scattered across
stop-files, D-records, and packet addenda — so the constitution is a document, not an
archaeology exercise. **Status discipline:** rules marked BINDING are already law by their
cited origin record; this file consolidates and cites, it does not re-legislate. Rules
marked PROPOSED become binding when this file lands in repo truth via the post-NA-0696
governance errand and the operator merges it. Future rules are ADDED by directive/D-record
citing this file; entries are never silently rewritten (mark-don't-rewrite).

## A. BINDING (consolidated; origin cited; the origin record governs on any conflict)
- **SR-01 — Read the file, not the summary.** Rulings issue against the stop-file
  artifact, never a chat paraphrase. (Root-cause record, succession packet.)
  **EXTENSION (ruled NA-0698 RBANK 004 R15; recorded in D-1338, whose GENERAL
  form governs per §D's conflict rule — delta from RBANK 004's Director-facing
  phrasing flagged per SR-09):** A finding of fact about specific lines of
  executable text requires a sealed artifact and a measured sha, not a chat
  rendering at an approval prompt. Observations at a prompt are observations;
  findings require the file.
- **SR-02 — R16, widened.** Any surprise = stop with diagnosis + proposal, never
  self-fix. An edit to any file OUTSIDE a directive's enumeration is a scope expansion
  and stops BEFORE the edit — even pre-push, even obvious. (NA-0695 STOP 006 §3.)
- **SR-03 — Stop-file convention.** Immutable, timestamped, self-contained; the stop
  CONTAINS its documents; LATEST.md is a pointer only; corrections to ruled stops go in
  new files. (Standing.) **Every stop-file states in one plain sentence, before the
  machinery, what the work buys.** (Added at D-1339 from operator direction 2026-08-07;
  origin recorded in D-1339. Folded in here rather than numbered, because its content
  already binds through operator communication preferences and the gap it closes is
  seat-side reporting.)
- **SR-04 — D-1330 two-PR shape.** Promotion PR (predecessor finalized DONE, block born,
  STATE advanced, named gates once at base) then impl PR (record + counter + flip as the
  only post-PR commit). Merge commits only; operator merges everything.
- **SR-05 — Suite three-part policy.** Targeted tests mid-development; base-run skip only
  EARNED (endpoints named, compiled-diff empty — identity or docs-only); ONE full suite
  on the exact committed tree, unpiped, own exit, reconciled BY NAME. (Operator-agreed
  2026-08-03.)
- **SR-06 — E-C control discipline.** Every negative control's EXACT red set committed in
  writing at Phase 0; one edit per control; cmp-identical restores; a one-test deviation
  is a STOP. (D-1335 elevation E-C.)
- **SR-07 — Core-path doc enumeration.** Any directive touching goal-lint core paths
  enumerates TRACEABILITY.md + DECISIONS.md by default, and formalization MEASURES
  REQUIRED_DOCS_FOR_CORE_CHANGES at drafting. (NA-0695 STOP 006 §5, binding from D630.)
- **SR-08 — Never-pushed amends.** Legitimate (the no-rewrite property protects
  PUBLISHED history) — and any figure carried across an amend is EARNED endpoints-named,
  the base-skip bar. (NA-0695 STOP 006 §2.)
- **SR-09 — Enumeration-is-not-the-record.** Measured census supersedes any prose count,
  including our own banked text; the superseding record says so explicitly. (Q6 ruling,
  NA-0696 STOP 002.)
- **SR-10 — Needle independence.** Distinct grep needles are measured and gated
  separately, never summed into one row; a directive states each needle's own base→after.
  (NA-0696 STOP 005 item 1.)
- **SR-11 — The WF-0044 mask arithmetic, stated once.** Unanchored `Status: READY` = the
  :71 prose (1) + real READY lines; count 2 while a lane is READY → preflight exit 1 =
  the mask, pre-named, never chased; count 3 = the forbidden literal was written = STOP.
  The anchored queue helper is the signal. (A1.1; WF-0044.)
- **SR-12 — Vocabulary separation.** emit_marker strings are the user-facing claim
  vocabulary; debug diagnostics (stderr, stable prefixes) are a separate vocabulary; no
  test or grep reads one as the other. (Q4 ruling, NA-0696 STOP 002.)
- **SR-13 — Distinct causes, distinct names.** A new cause gets its own marker; no
  existing cause loses one; claim-adjacent strings are measured free before mint.
  (D-1333 mapping discipline, applied through D630.)

## B. ADOPTED AND PROPOSED RULES (status per entry; adoption from this list one per checkpoint)
**RESTRAINT (convention, unnumbered).** At most ONE new NUMBERED rule is
minted per governance lane. Transcriptions of already-ruled law, ruled
extensions recorded in D-records, and unnumbered conventions do not count
against it. (Codified at D-1339, correcting a mis-citation: D-1338 and prior
invoked this as "SR-17 restraint" — SR-17's text contains no such clause; the
practice was real, its citation was not. Distinct from this file's
per-CHECKPOINT adoption and scheduling headers, which govern different acts.
The convention's own birth-breach is recorded, not hidden: WF-0048's single
merge activated SR-14/15/16 together.)

- **SR-14 — R-BANK: banked decisions land on disk immediately.** The moment the operator
  blesses a design block, the receiving CC session files it VERBATIM as an immutable
  timestamped file under /srv/qbuild/operator/<lane>/ BEFORE any probe, formalization, or
  directive consumes it. Chat is never the canonical home of a blessed decision. (Origin:
  the R8 reconstruction episode, NA-0696 STOPs 003/004.)
- **SR-15 — Adversarial second read.** Any directive that touches lock or crypto regions,
  exceeds five source files, or retires a safety mechanism receives a red-team pass by a
  FRESH instance (no conversational investment) against the banked record before the
  Director rules. The read produces findings, not rulings. (Origin: the D629
  internal-contradiction miss; the D630 R8 catches.)
- **SR-16 — Prediction ledger.** The Director appends predicted-vs-measured rows to
  PREDICTION_LEDGER.md at every lane close (gate predictions, calm/stop-count, CodeQL,
  suite figures). Quarterly read decides ceremony-tier adjustments with data. (Seeded
  2026-08-05.)
- **SR-17 — Tiered ceremony (design pending data).** A defined micro-lane tier with a
  minimum gate set (structural rows + one control + targeted tests + the full-suite
  identity rule) for single-file, non-lock, non-crypto changes; ANY surprise
  auto-upgrades the lane to full ceremony. NOT active until SR-16 has enough rows to
  draw the tier line; first candidate class: docs-only and ENG-0048-class one-liners.
  **⚠ ANNOTATION (D-1339, 2026-08-07): the data this entry deferred to has
  arrived, and the FIRST tiering design drafted on it (D633 STOP 001 §6.3)
  FAILED its operator-mandated SR-15 adversarial read (findings sha
  570e546ef264ae63535220da2c92f84ba1fdcb215b578bdfc0b0d5172d87d3d3: 2 BLOCKER /
  11 MAJOR — tiers named with no defined content; "blast radius" announced but a
  domain-keyed enumeration delivered; the substrate arithmetic wrong in the
  direction favouring the rule) and is REMANDED. The findings are BINDING DESIGN
  CONSTRAINTS on any successor design, which requires its own SR-15 read before
  it binds. This entry's own text — the minimum gate set and the
  any-surprise-auto-upgrades clause — remains the operative interim guidance.**

- **SR-18 — Observable-remap census (ADOPTED by Director ruling, NA-0696 STOP 007; effective D631).**
  Any directive that changes what an existing OBSERVABLE maps to — a marker string, an
  exit class, an on-disk name or layout — MUST, at drafting, run a mechanical
  corpus-wide census of every test pinning that observable, classify each pin against
  the changed path, and place every affected file in the authorized edit set WITH its
  new expected value. A census's needle must be as wide as its claim. (Origin: NA-0696
  STOP 006 — the na0694 key_source-tamper pin meeting the D5 load split; the third
  census-narrower-than-its-claim instance.)

- **SR-19 — Delta symbol (ADOPTED by Director ruling, NA-0697 STOP 005 §D R6; effective
  D632).** Every red-capable BEHAVIORAL instrument in a directive names its DELTA SYMBOL —
  the specific symbol inside the authorized edit set whose change flips the instrument
  red→green — and formalization verifies at drafting that (a) the instrument compiles and
  runs at base and (b) the delta symbol is reachable from it. An instrument whose delta
  symbol cannot be named is redesigned at drafting, not discovered at execution.

- **SR-20 — Consumer-validated emission (ADOPTED by Director ruling, NA-0698 RBANK 003 R2;
  effective D632).** Any instrument, script, or manifest whose output is consumed by
  another tool (compiler, test runner, CI engine, shell) is validated at drafting AGAINST
  THAT TOOL — the drafting control EXECUTES the consumer on the emitted artifact and
  records its exit — never against the author's model of the tool. Evidence that an emitter
  produced SOMETHING is never evidence that what it produced is ACCEPTED. (Origin: NA-0698
  STOP 001 — E8 verified shard args non-empty while I3 claimed the emit path verified;
  SR-15 measured cargo rejecting shard 6's args outright, exit 101.)
  **EXTENSION (ruled NA-0698 RBANK 008 R41; recorded in D-1338): THE EMITTING STEP'S
  ENVIRONMENT IS PART OF THE ARTIFACT'S IDENTITY. A fixture borrowed from a
  differently-configured job is not the artifact under test.** SR-20 already requires the
  consumer to be executed on the emitted artifact; this says WHICH emitted artifact — the
  one produced by the step that will actually produce it in production, under its real
  configuration. (Origin of the extension: NA-0698's first Control G run went red across
  all twelve shards because the reconciler was validated against a runner log from the one
  `ci.yml` job that does not use `dtolnay/rust-toolchain`, and therefore the one job whose
  cargo output is not colour-wrapped.)

- **SR-21 — An instrument's scope must equal its claim's scope (ADOPTED by Director ruling,
  NA-0698 RBANK 010 R55; effective D633).** Wider confounds; narrower misses. **THE
  OPERATIVE CHECK, applied BEFORE any figure is offered as evidence: does what I measured
  span exactly what I am asserting?** Five measured instances, all of which fail that
  question in advance: a needle wider than its claim (a `shard:` key matching a job named
  `shard`) · a census narrower than its claim (a literal-only `env::var` needle missing the
  dynamic site) · a fixture from the wrong producer (SR-20's extension above) · a re-run
  wider than its side effects (re-running a sealed series overwrote its own evidence
  because the OUTPUT root was not re-pointed) · an instrument wider than its claim (a
  repo-wide cache total offered as proof about ONE workflow's cache behaviour).
  **Corollary, from the fifth: when re-running a sealed series, re-point the OUTPUT root
  FIRST and prove it by listing the sealed directory's mtimes before and after.**

- **SR-22 — Two-pass figures (ADOPTED by Director ruling, NA-0712 RBANK 009 R248; effective
  D-1348).** ⚠ **Never write a figure about an artifact in the same pass that creates the
  artifact.** Two passes, always: **(1) write with the figure ABSENT · (2) measure · (3) insert.**
  ⚠ **A figure that cannot be measured at the instant it is typed MUST NOT BE TYPED** — leave the
  slot empty and fill it, or point at the listing **without asserting a value**. ⚠ **A hedge is not
  a measurement:** *"(see listing)"* beside an asserted value **disguises the assertion rather than
  softening it**. **WHY ITS OWN CLAUSE:** six measured instances in one lane — a needle mismatching
  twice, two verification greps that "found" absences that did not exist (`FRESHLY` vs `freshly`,
  `20 shards` vs `≤20 concurrent shards`), a commission line-count (139 vs 153), and a banked
  ruling's line-count-and-sha **written into the header of the stop that catalogued the first
  five**. ⚠ **Diligence failed six times, so the cause is not diligence: a document that cites its
  own metrics is written in one pass, and at the instant the figure is typed the artifact does not
  finish existing.** The structure guarantees the defect. ⚠ **Every instance was in a GOVERNANCE
  artifact, and a governance artifact's numbers are the only thing a later reader can check
  cheaply — a wrong sha in a header is the field a successor uses to decide whether a document IS
  the document.** This is SR-09/SR-10's *build it from the bytes, not from your model of them*
  applied to the act of **WRITING** rather than **READING**; the reading half has been ratified for
  weeks and the writing half is the one that failed.

## C. ADOPTION LADDER (machinery items; one per checkpoint; each ships with its own red-capable proof)
1. **Post-NA-0696 governance errand** (docs-only PR, with the ENG-0048 pairing window):
   lands THIS FILE at docs/ops/STANDING_RULES.md + PREDICTION_LEDGER.md + a ledger WF
   entry for the ladder. Activates SR-14/15/16.
2. **Gate-manifest micro-lane** (next): per-lane machine-readable needle manifest
   (needle → expected base → expected after) + one runner script; §7.1 rows,
   pattern-asserts, and both sweep directions become push-button. Red-capable proof: a
   deliberately wrong manifest row must fail the runner.
3. **Control harness** (with or after 2): apply-named-revert → run targeted set → diff
   measured red set vs written → restore → cmp, as tooling. Enables the retroactive
   negative-control audit track cheaply.
4. **GUI evidence tooling** — folded into the input-driver lane intent, not separate:
   capture → perceptual diff vs reference markup → verbatim text extraction, so Slice-4
   acceptance is gateable.
5. **CI-migration** (already queued) — the suite wall-clock cure; folds ENG-0112,
   WF-0046, ENG-0092, WF-0047 durable fix.

## D. MAINTENANCE
This file rides every handoff packet. The repo copy (once landed) is canonical; the
packet copy mirrors it. Additions cite their origin record; status changes
(PROPOSED→BINDING) cite the merging PR. Nothing here overrides a D-record; on conflict
the D-record wins and this file gets a correcting append.
