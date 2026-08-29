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
  ⚠ **CORRECTING APPEND, per §D's conflict rule — SR-05's text above is NOT edited (mark-don't-rewrite).**
  [Recorded by NA-0768 (`D-1409`), 2026-08-29, from `CLOSEOUT_NA0771_20260829.md` sec 7, sha256
  `a3d2392af9a8b912ae14e15ff9a4517ad1fd2ba3664e97d3f21f249604a0c401`, verified digit-by-digit with a
  negative control. **RESTRAINT honoured: this mints NO new numbered rule** — it clarifies the wording of
  an existing binding one.]
  **THE OCCASION.** NA-0771 ran its end-of-lane full suite **CONCURRENTLY** at the operator's direction and
  DECLARED the deviation rather than absorbing it. The Director **RULED IT COMPLIANT** and amended the
  wording, because the property SR-05 protects was never serialism.
  **"FULL" MEANS, IN TERMS:** EVERY target of the census, **ONCE**, on the **EXACT committed tree**, with
  **each target's own exit recorded and never piped**, and **any nonzero target re-run ALONE** under a rule
  written BEFORE the re-run — *passes alone = contention; fails alone = real*.
  ⇒ **CONCURRENCY UNDER A LOAD GOVERNOR IS PERMITTED. SERIAL WAS NEVER THE PROPERTY.**
  ⚠⚠ **AND THE CLARIFICATION ARRIVES WITH THE DEFECT THAT MAKES IT LOAD-BEARING.** Concurrency moves the
  risk from wall-clock to **COMPLETENESS**: on that same run a runner keyed its results by target NAME while
  two targets share the name `qsc`, one silently OVERWROTE the other, and **the run reported GREEN while
  incomplete** — caught ONLY by reconciling the executed count (148) against the manifest's census (149).
  ⇒ **THE "EVERY TARGET, ONCE" CLAUSE IS A COUNT TO RECONCILE, NOT A SENTENCE TO BELIEVE**, and a
  concurrent runner that cannot state its own cardinality cannot satisfy this rule. Filed as `WF-0091`;
  the property is `PR-7` (`D-1412`), *a green run is not a complete run*; the general form is already
  `SR-26` (a''): an instrument that reports a verdict without its count cannot report that it looked at the
  wrong number of things.

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
  D-1349).** ⚠ **Never write a figure about an artifact in the same pass that creates the
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

- **SR-25 — THE BETTER-PATH RETROSPECTIVE (ADOPTED by OPERATOR ORDER of 2026-08-19; landed
  NA-0750 / D-1392, ruled at `R365` §1).** Source banked at
  `/srv/qbuild/operator/method/RBANK_better_path_retrospective_20260819.md`, sha256
  `691fa3fdfbbdd10766175285bc1fa46e193257190aa5f04f26918008b2d0794f`. The rule body below is
  **transcribed byte-verbatim from that file's own bytes**, with its single placeholder `SR-XX`
  resolved to `SR-25` and a two-space list-continuation indent applied; both transforms were
  inverted and diffed back against the source extraction (**EMPTY**), against a negative control
  that returned non-empty.
  ⚠ **CORRECTING APPEND, per §D's conflict rule.** `R310` (in `D-1363`) recorded *"the
  constitution ends at SR-22 and that is now a decision, not a gap."* That sentence **disposed of
  the SR-23 reservation** and is **NOT edited** — it stands as issued, and this append supersedes
  it **in the open**. The superseding authority is the **OPERATOR'S ORDER of 2026-08-19**, which
  outranks a Director ruling. **`SR-23` stays permanently reserved-and-unminted** (R310) and
  **`SR-24` stays refused** (`R305` / `WF-0078`); **neither number is reused.**
  ⚠ **AND A NUMBERING COLLISION IS DISPOSED HERE RATHER THAN LEFT TO OUTLIVE THE LANE.** The
  operator-area proposal `PROPOSAL_CHAIR_BOUNDARIES_20260813.md` declares `SR-25` (*"Same sha is
  not the same world"*) and `SR-26` (*"No pairing without a pre-flight"*) in this file's exact
  declaring form, and its own `SR-24` names a **different** rule from directive 654's `SR-24`.
  Its **substance** was adopted at `R288`; its **numbers** were never ruled, and it collides with
  itself, so **it is not a numbering authority**. Those `SR-25`/`SR-26` number claims are
  **RETIRED AS NUMBERS**. If any rule in it is ever adopted it takes a **fresh id derived at that
  edit**.
  SR-25 — THE BETTER-PATH RETROSPECTIVE (id derived at the edit; operator-ordered 2026-08-19).
  At every lane close, the Director's clearance records one explicit answer to: "Knowing what
  we know now, was there a BETTER path — judged on all three axes: a better OUTCOME (stronger
  design, stronger evidence, more durable result), a better SEQUENCE (ordering, scope, or a
  different thing worth doing instead), or a materially CHEAPER route to the same outcome —
  and what would it have traded?" Each axis is answered, not skipped. Exactly one disposition
  per axis, stated never implied: NONE FOUND · CANDIDATE (named and filed to the backlog
  surface with its trade) · ADOPTED-FORWARD (the next brief changes, cited). A close without
  the line is a defective close. The retrospective PROPOSES; it never enacts — method and
  design changes ride the loop like everything else. INVARIANT no retrospective may trade
  away: at least two independent instruments touch every load-bearing claim.

- **SR-26 — STOP-FILE SELF-CONTAINMENT, AUDITED AS AN INSTRUMENT (ADOPTED by OPERATOR ORDER of
  2026-08-27, relayed verbatim as *"not just a kickoff... lets make it durable asap"*; landed
  NA-0770 / `D-1411`).** Source banked at
  `/srv/qbuild/operator/method/PROPOSAL_SR26_20260827.md`, sha256
  `15436299e5bb6b188dd184f5268a461bf0833510b45c4186574ea1a86a5c8f38`, 444.
  ⚠ **NUMBERING, RE-DERIVED AT THIS EDIT AND RECONCILED SO IT DOES NOT READ DOUBLE-SPENT:** the
  SR-25 note above records that `PROPOSAL_CHAIR_BOUNDARIES_20260813.md` declares an `SR-26`
  (*"No pairing without a pre-flight"*) in this file's exact declaring form, and that the proposal
  **is not a numbering authority** — its `SR-25`/`SR-26` claims are **RETIRED AS NUMBERS**. `SR-25`
  was re-measured as the highest live numbered rule in this file at this edit, so `SR-26` was FREE.
  The refused rule is not adopted, not renumbered and not revived; if it is ever adopted it takes a
  fresh id. **RESTRAINT honoured: this is the ONE new numbered rule in this act.**

  SR-26 — STOP-FILE SELF-CONTAINMENT, AUDITED AS AN INSTRUMENT (id derived at the edit;
  operator-ordered 2026-08-27).
  A stop file is not complete until its author has run a self-containment audit MECHANICALLY over
  the FINISHED file's own bytes and printed the result inside that same file. The audit is not a
  resolve, a checklist read, or a recollection; it is a run, and a stop file without its printed
  result is incomplete on its face.

  THE AUDIT, MINIMALLY:
  (a) PRINT THE ABSOLUTE PATH, filename and sha256 of the stop file itself, at the head of the
      file and again at its end.
  (a') ⚠⚠ **THE STAMP IS THE LAST ACT, AND IT IS VERIFIED FROM THE BANKED BYTES.** [DIRECTOR'S
      AMENDMENT, NA-0770 STOP-009 ruling, 2026-08-29 — marked as such so the operator may refuse
      it at merge.] The sha256 printed under (a) is the file's SELF-DIGEST under a construction
      that must be STATED IN THE FILE. The LAST act before banking is to recompute that digest
      FROM THE BANKED BYTES AT THE BANKED PATH and compare it to what is printed. Any of the
      following is a STOP, not a note:
        · the recomputed digest does not equal the stamped one;
        · ANY identity field still holds an unresolved placeholder (a filename, a commit, a path);
        · the audit was run over ANY path other than the banked one.
      ⚠ THE INSTANCES THIS PARAGRAPH IS MINTED ON, both in the lane that minted SR-26 itself:
      **STOP 008** carried an unresolved `@@FILENAME@@` in its head path line, and **STOP 009**
      carried BOTH that placeholder AND, in its head digest slot, **STOP 008's self-digest** —
      because 009's head was copied from 008 rather than regenerated, so the substitution found
      no placeholder to replace. Both files passed SR-26's own audit and NA-0756's section-map
      diff at zero gaps. ⇒ **NEITHER INSTRUMENT LOOKS AT THE IDENTITY FIELDS THEY PRINT**, and
      the audit in 009 ran over a scratchpad path, so it audited bytes that were never banked.
      A stop whose own identity line is wrong is unverifiable by the reader it exists for.

  (b) ENUMERATE every question the governing brief/ruling asks and every record it orders, and for
      EACH one quote the line in this file that answers it. An item with no quote is a GAP: cure
      it, or state plainly that it is unanswered and why.
  (c) CONFIRM, each as a measured property and not an assertion: every document the file's PROSE
      cites is CARRIED IN FULL, not pointed at (strip embeds before counting citations, so a
      filename appearing only inside a carried document is not miscounted) · every record PROPOSED
      is drafted as TEXT, not named as a destination · every control prints BOTH ARMS and the
      values they produced · every measured output is quoted VERBATIM, not summarised · what was
      NOT measured is stated plainly, with its reason, together with n= and the claim boundary.
  (d) PRINT the verdict line: "SELF-AUDIT COMPLETE — N items checked, M gaps found and cured, K
      stated unanswered." A run finding nothing says zero.

  THE FAILURE MODE THE RULE EXISTS AGAINST, named because naming it is what makes the audit
  non-vacuous: **AN AUTHOR CARRIES WHAT HE REASONED ABOUT AND MERELY CITES WHAT HE USED.** The
  document that gets cited rather than carried is precisely the one whose unquoted parts BOUND the
  claim being made — so the gap is not random, it is systematically located at the load-bearing
  sentence. Vigilance does not cure it; running the audit over the bytes does.

  SCOPE: every stop file, every lane, every seat, including the Director's own rulings and briefs
  where they are handed to a chair that cannot read `/srv`. The rule binds the AUTHOR, not the
  reader: a Director or operator who has to ask for the audit has already paid the cost the rule
  exists to remove.

  WHAT IT DOES NOT DO: it does not prescribe a tool, a script, or a file format — only that the
  audit be MECHANICAL over the finished bytes and its result printed. It does not weaken SR-01
  (the Director still rules from the artifact). **It does not license a seat to shorten a stop
  file: the audit's cure is always to CARRY more, never to claim less.**

  (a'') ⚠⚠ **A CARRIED DOCUMENT IS VERIFIED BY ITS OWN WHOLE-FILE DIGEST, AFTER STAMPING.**
      [DIRECTOR'S AMENDMENT, NA-0771 / `RULING_NA0771_004` sec 3 B-3, 2026-08-29 — marked
      as such so the operator may refuse it at merge. Landed by NA-0771 (`D-1412`).]
      A stop's SELF-DIGEST may MASK a class of bytes — the construction in use masks every
      64-hex run — and a digest that masks a class **cannot see an edit in that class**.
      Therefore the carry check must not use it. For every document a stop carries:
        · the stamping pass MUST NOT WRITE INSIDE a carried document at all; and
        · after stamping and BEFORE banking, each carried document is EXTRACTED from the
          finished bytes, hashed with a WHOLE-FILE sha256, and compared to the value
          printed beside it. **The comparison is printed in the same file.** A mismatch
          is a STOP, not a note.
      ⚠ THE INSTANCE THIS PARAGRAPH IS MINTED ON: **STOP_NA0771_004**. Its stamping pass
      filled every `@@SELFDIGEST@@` placeholder in the file with its own digest — INCLUDING
      the two literals inside the two carried SR-15 cold-read documents, corrupting each by
      +50 bytes (64 − 14) at the line stating that file's own construction. Both carries'
      printed `sha256` and `size` were wrong for the bytes actually carried. **The stop's
      seal verified anyway**, because the corruption is invisible to a construction that
      normalises hex; and its self-containment audit could not see it either, because that
      audit STRIPS embeds — so the one pass that writes into carries and the one pass that
      could have caught it were blind in exactly opposite directions.
      ⚠ AND THE FIRST CURE HAD THE SAME BLINDNESS: a fence-label pattern of `[^-]+` silently
      skipped the two documents whose labels contain a hyphen — the very two the defect was
      about. It was caught only because the instrument printed its own CARDINALITY (six
      carries found where eight were specified) rather than only its verdict.
      ⇒ **AN INSTRUMENT THAT REPORTS A VERDICT WITHOUT ITS COUNT CANNOT REPORT THAT IT
      LOOKED AT THE WRONG NUMBER OF THINGS.**

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
