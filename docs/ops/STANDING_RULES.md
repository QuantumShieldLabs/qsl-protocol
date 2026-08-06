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
- **SR-02 — R16, widened.** Any surprise = stop with diagnosis + proposal, never
  self-fix. An edit to any file OUTSIDE a directive's enumeration is a scope expansion
  and stops BEFORE the edit — even pre-push, even obvious. (NA-0695 STOP 006 §3.)
- **SR-03 — Stop-file convention.** Immutable, timestamped, self-contained; the stop
  CONTAINS its documents; LATEST.md is a pointer only; corrections to ruled stops go in
  new files. (Standing.)
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

## B. PROPOSED (become binding when this file lands in repo truth; adopt one per checkpoint)
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

- **SR-18 — Observable-remap census (ADOPTED by Director ruling, NA-0696 STOP 007; effective D631).**
  Any directive that changes what an existing OBSERVABLE maps to — a marker string, an
  exit class, an on-disk name or layout — MUST, at drafting, run a mechanical
  corpus-wide census of every test pinning that observable, classify each pin against
  the changed path, and place every affected file in the authorized edit set WITH its
  new expected value. A census's needle must be as wide as its claim. (Origin: NA-0696
  STOP 006 — the na0694 key_source-tamper pin meeting the D5 load split; the third
  census-narrower-than-its-claim instance.)

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
