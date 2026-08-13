# DIRECTOR_OWED RE-AUDIT — 2026-08-13 (NA-0718, D-1354)

**What this is.** A status re-audit of the Director's owed-acts record
`DIRECTOR_OWED_AT_NA0698_CLOSE.md` (operator-side, written 2026-08-06T15:44Z), ordered as
the NA-0717-arc governance errand (ruling packet R277-R279, ORDER 3 leg 2). Every status
below is established from ON-BOX ARTIFACTS ONLY, with the instrument stated; where no
on-box record exists the item is marked UNDISCHARGED — per the source document's own
principle: *"an obligation living only in a chat window is not an obligation."* Nothing
here is reconstructed from memory.

## Status table

| item (verbatim heading from the OWED record) | status | evidence |
|---|---|---|
| 1. "THE GOVERNANCE ERRAND — OPERATOR-APPROVED 2026-08-06" | **DISCHARGED** by NA-0699 (D-1339) | TRACEABILITY.md 2026-08-07 entry: `docs/ops/STANDING_RULES.md` gained the SR-01 R15-extension, SR-03's payoff sentence, the RESTRAINT convention, SR-19, SR-20 (with the environment-is-identity extension "carried byte-exact"), and SR-21; and item 1b's mandated red-team ran — *"SR-17 IS NOT REPLACED: the drafted tiering rule was WITHDRAWN ENTIRE after the operator-mandated SR-15 read returned 2 BLOCKER / 11 MAJOR / 13 MINOR / 9 NOTE."* |
| 2. "THE CACHE AFTER FIGURE — THE LANE IS NOT CLOSED WITHOUT IT" | **UNDISCHARGED** | No on-box record of the post-merge keyed-entries measurement exists: recursive search of `/srv/qbuild/operator/` for the instrument string `v0-rust-qsc-sharded` (2026-08-13) matches only NA-0698's own stop-files and the OWED record itself — no AFTER figure anywhere. |
| 3. "THE SR-16 PREDICTION-LEDGER ROWS + THE PACKET REFRESH" | **UNDISCHARGED — and wider than the OWED record knew** (see the finding below) | Ledger measurement + on-box search, both instruments stated below. No `QSL_HANDOFF_*.zip` exists on-box either (find over `/srv/qbuild/operator`, 2026-08-13) — the packet-refresh half is equally open; the NA-0717-arc closeout owes a rebuild. |
| 4. "THE PROMOTION/RETIREMENT DECISION — OPERATOR-RATIFIED 2026-08-07" | **PENDING (no act taken)** | `qsc-sharded-suite` is ABSENT from main's required-context set (15 contexts, read from branch protection 2026-08-12, banked in the NA-0717 lane record); the monolith `qsc-linux-full-suite` is undemoted and measured at ~220 min per push across five banked runs (91% of its 240-min ceiling). ⚠ INTERACTION, stated openly: the NA-0717-arc ceiling micro-lane raises that monolith's ceiling 240→330 as a BRIDGE — it does not replace the ratified demote-after-three-green-lanes plan, whose green-lane tally is the Director's, not this seat's. |
| 4b. "RUST-ANALYZER LSP PLUGIN — OPERATOR-DEFERRED 2026-08-07" | **PARTIALLY EVIDENCED** | The binary IS installed (`~/.cargo/bin/rust-analyzer`, measured 2026-08-13). The OWED half-acts around it — the toolchain-match verification record and the corroborating-instrument ruling (*"rust-analyzer is a CORROBORATING instrument, never sole evidence for a census claim"*) — are found in no on-box record; the ruling remains owed. |
| 4c. "THE AUDIT SCHEDULE — OPERATOR-APPROVED 2026-08-07 ('yes!')" | **UNDISCHARGED** | No audit-schedule proposal artifact exists in repo docs or the operator dirs (search 2026-08-13; the only hit is NA-0389's old ops-cadence doc, a different subject). The four anchored tracks (negative-control audit after the GUI driver · claim audit before external review · the external review after Slice 4 · legal-readiness pulled forward) remain unscheduled. |
| 5. "THE QUEUE AFTER NA-0698" | **EVOLVED — superseded by events** | The named sequence largely executed: polish + GUI driver (NA-0700–0702), Slice-4 probes (NA-0704/0706), crypto-adjacent and CI work through NA-0717. The live queue is `NEXT_ACTIONS.md`; the OWED copy is history, not authority. |
| Stale-line notice (operator project-instructions say qsl-desktop has no public-safety scan) | **UNVERIFIABLE FROM THIS SEAT** | The operator's instruction block is not an on-box artifact; the underlying fact (the scanner exists since NA-0686/D-1325) is repo truth. The one-line operator edit remains flagged. |

## The central finding: eighteen lanes of close rows were never written anywhere

Repo truth (`docs/ops/PREDICTION_LEDGER.md` at `e4cb73dc`) carries close rows through
**NA-0698**, then NOTHING until NA-0717's six drafting rows. For the eighteen lanes
**NA-0699 through NA-0716** the ledger holds ZERO rows — and the on-box record holds no
source to harvest them from:

- **Instrument:** recursive grep of `/srv/qbuild/operator/` for lines matching
  `^\| NA-0(699|70[0-9]|71[0-6])` (any pipe-table row led by a lane token), 2026-08-13.
- **Result:** six files matched; each classified a FALSE POSITIVE on read — divergence
  tables (NA-0700 STOP_001/STOP_010), a suite-census citation (NA-0705 STOP_001), a
  ladder citation in a directive and its stop (NA-0706), and an id-sweep table (NA-0713
  STOP_005). **Not one is an SR-16 prediction row.**
- **Conclusion:** the enter-at-the-next-close deferral chain never discharged; the rows
  do not exist to be transcribed. Per the governing packet's rule they are **LISTED AS
  STILL-OWED — never reconstructed from memory**:

**STILL-OWED close rows (18 lanes):** NA-0699 · NA-0700 · NA-0701 · NA-0702 · NA-0703 ·
NA-0704 · NA-0705 · NA-0706 · NA-0707 · NA-0708 · NA-0709 · NA-0710 · NA-0711 · NA-0712 ·
NA-0713 · NA-0714 · NA-0715 · NA-0716 — each owed by the Director's chair at that lane's
retrospective close, sourced from its lane record at writing time, entered under the
ledger's own append-at-close rule. NA-0717's execution rows (a lane still open at this
writing) enter at ITS close per the standing deferral and are not part of this debt.

## Boundary

This note records status; it closes nothing by re-reading (the NA-0709 lesson). "Not
found on-box" is the measured fact; whether an act occurred in a chat window is outside
this instrument's scope — and outside the record's, by the OWED document's own rule.
