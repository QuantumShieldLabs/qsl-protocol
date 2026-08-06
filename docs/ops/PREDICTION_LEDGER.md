# QSL PREDICTION LEDGER (SR-16) — seeded 2026-08-05
**Rule:** the Director appends predicted-vs-measured rows at every lane close. Predictions
are quoted from the directive/block as written BEFORE execution; measurements from the
stop-file of record. Purpose: after enough rows, ceremony tiering (SR-17) gets decided
with data instead of feel. Scoring: HIT (as written), MISS (direction wrong), PARTIAL.

| lane | prediction (as written) | measured | score | lesson row |
|---|---|---|---|---|
| NA-0695 | "expected calm" (D-shape, one stop at PR) | six stops, one gate surprise disposed pre-push | PARTIAL | calm ≠ stop-count; the surprise was the directive's own contradiction |
| NA-0695 | CodeQL WF-0047 class "EXPECT IT TO FIRE" on new salt consumers | ZERO alerts fired; dismissals carried | MISS (safe direction) | taint model narrower than assumed; salt→account-name not a crypto sink |
| NA-0695 | suite 596/0/2 across 129, by-name +1 binary | exact | HIT | — |
| NA-0695 | goal-lint green (§7.5) with TRACEABILITY absent from §6 | RED first run; directive internally inconsistent | MISS | became SR-07 |
| NA-0695 promo | fmt/clippy/structural base rows | all exact | HIT | — |
| NA-0696 probe | banked D1 mechanism assumptions (drop-order, panic-safety, path-key) | all held; zero contradictions | HIT | probe-before-draft pays |
| NA-0696 probe | "7 call sites" (banked) | 8 textual = 7 calls + 1 fn-value | PARTIAL | became SR-09/SR-10 |
| D630 draft | preflight GREEN at promotion (count 0 at base) | count 1 at base (:71 prose); promotion → 2 → RED mask | MISS | inference from exit code, not from the count itself; A1.1 corrected |
| D630 draft | "NO test asserts on stderr" | vault.rs:165 not-contains predicate exists (harmless to the emit) | MISS | census grep shape too narrow; A1.2 corrected |
| D630 draft | banked-block reconstruction faithful | 3 substantive divergences (D5a shape, D4 boundary, D4 comment) | MISS | became SR-14 (R-BANK) |
| NA-0696 promo | preflight RED (mask, unanchored 2 / anchored 1) | exact | HIT | corrected prediction held |
| NA-0696 promo | §7.1 base rows incl. two-needle split | all exact (12 + 1) | HIT | SR-10 applied |

<!-- Append below at each lane close. NA-0696 execution rows pending: suite 606/0/2/130
prediction; six control red sets; sweep zero-unclassified; e2e guards green; CodeQL
re-anchor expectation; ProVerif pass. -->
| NA-0696 exec | §2d.5: "NO test pins the old collapse" | FALSIFIED — na0694:155 pinned it; 1-red suite | MISS | became SR-18; third census-class instance |
| NA-0696 exec | "expects its R16 stops" (the lane framing) | exactly one R16 (STOP 006), fully diagnosed | HIT | the framing prediction held |
| NA-0696 exec | suite 606/0/2 across 130, by name | exact, on the amended commit (mandatory re-run) | HIT | identity-carry correctly refused across a test edit |
| NA-0696 exec | six E-C control red sets, committed exact | all EXACT incl. two pre-named sketch deviations | HIT | E-C discipline at full scale |
| NA-0696 exec | sweep: zero unclassified; 7 retired sites in held locks | exact, bidirectional | HIT | — |
| NA-0696 exec | CodeQL "may re-anchor" (hedged) | 0 open alerts on the ref | HIT (hedged) | second consecutive non-fire; WF-0047 set still 4 |
| NA-0696 exec | preflight A1.1 corrected trajectory (RED window → green post-flip) | exact | HIT | the corrected prediction, not the drafted one |
| NA-0696 exec | ProVerif pass (no formal/ touch) | pass | HIT | — |
