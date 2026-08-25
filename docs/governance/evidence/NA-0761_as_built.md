# NA-0761 — AS BUILT — THE RECORDS CONSOLIDATION: EVERY ACCUMULATED RECORD DEBT PAID IN ONE DOCS-ONLY PR

Spine decision **D-1402** · class **not declared at landing** (this lane's result class awaits the Director's
close-out; the line is marked to be amended when it is declared)
Order `ORDER_records_na0761_20260825.md` (sha256 `75e6376565c0559b9d5faf4c0a67246dcff078b42c25889f105e277ca8d8afcf`,
69 l / 4494 B, 444), sha-VERIFIED against its own bytes BEFORE being read.
Inputs, all four sha-VERIFIED before being read:
`CLOSEOUT_NA0759_20260825.md` (`e318aeaa…4df42`, 60 l / 4008 B) ·
`CLOSEOUT_NA0758_NA0760_20260825.md` (`d00d0e32…ece280`, 66 l / 4425 B) ·
`CLOSEOUT_NA0756_NA0757_20260825.md` (`15417b3c…78c5d`, 84 l / 6098 B) ·
`STOP_NA0760_002_20260825T194500Z.md` (`d066df2f…160818`, 702 l / 53636 B) — its banked sha confirmed against
the pointer that carries it (`relay/LATEST.md:61`), not merely recomputed from the file in isolation.
Base: qsl-protocol `35c80c006e9ee9a273cd5b3b03e2fc3cd10e9ee5`, re-derived **bare and unpiped** at the NAMED
github remote. ⚠ `origin` in these seats is the local mirror at `/srv/qbuild/mirrors/`, measured **STALE at
`241eec97`** — it does not contain this base at all — and was never used as a source of truth.

## 1. What shipped

**Six paths, docs and records only.** Zero code, zero tests, zero `Cargo` bytes, zero `.github` bytes, zero
`formal/` bytes, zero manifests, zero `STANDING_RULES.md` bytes.

| # | path | change |
|---|---|---|
| 1 | `NEXT_ACTIONS.md` | five edits: NA-0759 status AMENDED; NA-0758 block BORN AT CLOSE; NA-0760 FLIPPED; NA-0761 block born READY; STATE advanced |
| 2 | `docs/ops/IMPROVEMENT_LEDGER.md` | `ENG-0246` filed (filing only, no cure) |
| 3 | `docs/ops/PREDICTION_LEDGER.md` | 35 rows: `245-252` verbatim, `253-279` renumbered |
| 4 | `DECISIONS.md` | `D-1402`, carrying both close-outs WHOLE and byte-verbatim |
| 5 | `TRACEABILITY.md` | one row |
| 6 | `docs/governance/evidence/NA-0761_as_built.md` | this file (gitignored path; force-added) |

## 2. The three classes, transcribed no-drift

| lane | class, from the close-out's own bytes | how it landed |
|---|---|---|
| NA-0759 | `MACOS_FIXTURE_RACE_CURED_AND_VACUOUS_GREEN_FILED` | in-place AMENDMENT to a status line that named its own debt |
| NA-0758 | `PUBLIC_CLAIMS_MEASURED_TRUE_AND_FRONT_DOOR_LAUNCHED` | block **BORN AT CLOSE** |
| NA-0760 | `LICENSE_CANONICAL_AND_SELF_ENFORCING_EVERYWHERE` | ordinary READY → DONE flip |

**Mark-don't-rewrite was honoured on the amendment.** NA-0759's line keeps its original
`**CLASS NOT YET DECLARED.**` clause *and* its `it is owed an amendment carrying the class when the close-out
lands` sentence; the supersession is marked in place and the amendment is appended, so a reader still sees what
was true at the flip.

## 3. The seals

**Z1 — NO-DRIFT.** Each class string measured present in its close-out's bytes **and** in the landed line, both
sides printed. Counts: close-out 1 / `NEXT_ACTIONS.md` 2 / `DECISIONS.md` 2 for each of the three (the second
`NEXT_ACTIONS` hit is the prior-STATE comment; the second `DECISIONS` hit is inside the verbatim fence).
**Negative control:** a one-character tamper (`…GREEN_FILEX`) returns **0** on both sides, so the comparison
discriminates.

**Z2 — ROW CONTINUITY.** Numbered tail **244 before**, **279 after**; count == unique == max, contiguous from 1,
verified by `diff` against `seq`. **The instrument was proven able to fail, three independent ways:** a duplicated
number → RED (unique 278), a deleted row → RED (count 278), a tail renumbered 279→999 → RED (max 999). ⚠ The
instrument was **validated against the file's own structure first**: this ledger carries **two row formats** (244
numbered rows plus 82 legacy lane-keyed rows that carry no number), and the legacy rows cannot collide with the new
numbering — measured, not assumed, because a prior lane was caught by exactly this file's two formats.

**Z3 — ONE READY.** `grep -c '^Status: READY'` = **1** after the edit, and it is NA-0761's `Status: READY (D-1402)`.
The anchored needle is the repaired one (`preflight_governance.sh:39`); the historical unanchored needle is not used.

**Z4 — TRAILERS AND IDENTITY.** Commit trailers **0**, with a positive control proving the needle can return
non-zero. Author/committer identity confirmed as the house noreply **before** the commit, not after.

**Z5 — THE FIXED CHECKLIST**, discharged line by line at the stop; checks APPEAR then SETTLE.

**The close-out fences round-trip.** Both close-outs are carried WHOLE inside `D-1402`, ASCII armor included and
deliberately not de-armored — which is what makes the cited shas checkable. Extracted back out of the landed
`DECISIONS.md` by two independent routes (python string equality and `cmp`), each fence is **byte-identical** to
its banked file and its recomputed sha **equals** the cited sha. Tamper controls on both fences fired.

**The renumber is arithmetic, not retyping.** Old `207-233` → new `253-279`, a uniform **+46**. Proven by
stripping the number cell from both the source rows and the landed rows and comparing the remainders for
byte-identity — **identical**. The seven rows `227-233` were `cmp`-proven byte-identical between STOP 007 and
STOP 008, so the choice of stop of record is immaterial.

## 4. Measured and reported — NOT acted on

A seat does not widen its own order. Three conditions were measured, are real, and were left alone:

1. **`D-1401`'s `**Class:**` field is now owed its amendment by its own sentence** — it reads *"not declared at
   landing … this line is to be amended when it is declared"*, and the class **is** now declared. It is the only
   such line in `DECISIONS.md`, established by sweep. After this landing NA-0760's status line carries the class
   while `D-1401` still says it has none. Not in this lane's enumeration; not edited.
2. **NA-0759 carries two class strings in repo truth**, each correctly attributed to its own act:
   `MACOS_FIXTURE_RACE_NOT_A_PRODUCT_REGRESSION` — the interim diagnostic class ruled at the MACOSREG STOP-001
   ruling — at `DECISIONS.md:43986`, `docs/governance/evidence/NA-0759_as_built.md:3`,
   `docs/ops/IMPROVEMENT_LEDGER.md:6235` and one frozen historical `<!-- prior: STATE … -->` comment; and the
   close-out's RESULT class on the amended status line. Both true of the act each names; a census of *"NA-0759's
   class"* returns two answers.
3. **`D-1400`'s close-out fence is a 67-line de-armored excerpt beneath a whole-file sha citation** of an 84-line
   document, so that sha cannot be verified against that fence. The predecessor is left exactly as it stands.

Two corrections to the order's own premises are recorded rather than absorbed:

- **The `ENG-0246` cure is not `.github/**`.** Only the step's *name* lives there (`public-ci.yml:476`); the logic
  is `scripts/ci/public_safety_gate.py` (`:796-801`, `:814-815`, `:1339-1343`) — an ordinary tracked file.
- **Neither of `ENG-0246`'s two statements is a first sighting.** `ENG-0189` already carries both, added
  2026-08-15 by NA-0734 (`D-1370`) at R332.3 and R332.2 — the latter having deliberately minted **no id**, on the
  WF-0029 precedent. The id is minted here because two governing documents order it by name; the tension is
  recorded inside the entry so it can be folded into `ENG-0200` with one ruling.

## 5. Id derivation

`NA-0761` **0** occurrences tree-wide before this landing · `D-1402` **0**, derived across **all four**
`DECISIONS.md` record forms (max `D-1401` from the `## D-` form; a form-specific needle would have been right only
by luck) · `ENG-0246` **0** declarations against a positive control of `ENG-0245` present · prediction rows
`245-279` free against a measured tail of 244.
⚠ **Open-PR control:** `gh pr list --state open` returned **zero** on **all five** repositories — protocol,
desktop, server, attachments, `.github` — so no live lane could collide.
⚠ Out-of-range mentions (`NA-0791`/`NA-0799`, `ENG-0266`/`ENG-0299`) were **classified, not adopted**: each is a
prior lane's own negative control quoted in prose. The WF-0087 plant hazard, counted as mentions and not
declarations.

## 6. Claim boundary

This lane proves **no product property**. It claims that three Director-declared classes now appear in repo truth
byte-equal to their close-outs' own bytes; that `NA-0758` has a governance block honestly dated at its close; that
`ENG-0246` is filed and explicitly **not** cured; that the prediction ledger's numbered tail is contiguous 1..279;
and that both close-outs are recoverable byte-verbatim from `DECISIONS.md`. It does **not** claim that the program's
record debt is zero on every axis — three named exceptions are listed in §4 above, and the `NA-0756`/`NA-0757`
axis needed nothing here because those blocks already carried their classes, measured at the base.
