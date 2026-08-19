# NA-0748 — AS BUILT (PROMOTION)

**Phase 1 Lane 1: the `qsc` pin bump (`ENG-0207`'s disposition), carrying two operator rulings.**
Promotion `D-1389`. Governing document: the Director's Formalization Brief of 2026-08-19, banked
verbatim under SR-14 as this lane's FIRST ACT.

Bases: qsl-protocol `1ed3232960bdc52b84237ec49c6f2ec771a91e7f`, qsl-desktop
`c52fd51bbaff5882741620a7774f2253814ddaa7` — both re-derived **bare and unpiped** against the named
`github` remotes at the edit, both matching the brief's stated bases. Open-PR sets **0** in both
repos, each against a positive control returning rows.

⚠ **WHAT THIS ACT DID NOT DO: move the pin.** No product source byte, no manifest byte, no lock
byte, in either repo. The bump is the impl act and it is gated behind a formalization stop, an
SR-15 cold read and a Director ruling. `src-tauri/Cargo.toml:23` still pins
`32e572c763a7437a73c3ca865397cef37ae38fd4` at the close of this act.

## 1. THE TWO OPERATOR RULINGS, AND WHERE THEY LANDED

Both are landed **beside** their ledger entries under mark-don't-rewrite. Neither entry is edited;
each entry's own `- Status: open — FILING ONLY` bullet stands, true of the act that wrote it.

| ruling | entry | disposition | what stays open |
|---|---|---|---|
| `ENG-0205` | the fingerprint-format contradiction | the **RATIFIED** two-tier format is the product format; the shipped `qsc` format moves to match | the entry stays **OPEN**; the repair is the **named successor `qsc` lane**, owing an SR-15 read and a consumer census of every fingerprint-string reader |
| `ENG-0207` | the 147-commit pin gap | **BUMP-THE-PIN**, executed by this lane | the entry is **ruled but not closed**; it closes when the bump lands. Six other shapes stand as **alternatives-not-taken, none refused on merits** |

Provenance for both: the operator's words of 2026-08-19, quoted in each record as
[O, chat, 2026-08-19, Director-relayed] — *"A, and bump"*.

⚠ **The two rulings are separable and the separation is load-bearing.** `ENG-0205` says the format
changes; `NA-0748` seals **V3 FINGERPRINT INVARIANCE**, which requires the fingerprint string and
verification code to be **byte-identical across the bump**. The bump provably does not move the
format; the successor lane provably does. Attribution stays clean only because this lane can prove
a negative.

## 2. NA-0747's OWED CARGO, DISCHARGED FROM ITS OWN BYTES

Two records were drafted by NA-0747's terminal stop and owed to this promotion. Neither was
retyped: each was **substituted from the drafted block's own bytes**, its placeholders filled by
program and the residual set asserted **empty**.

| block | source | placeholders filled | residual |
|---|---|---|---|
| NA-0747 `MERGING` → `DONE` | `M1_successor_done_flip.txt` (1583 B) | `<promotion>`×2 → `1389`, `<successor>` → `NA-0748` | **0** |
| the `ENG-0195` reconciling clause | `M2_successor_reconciling_clause.txt` (1305 B) | `<date>` → `2026-08-19`, `<successor lane>` → `NA-0748`, `<promotion>` → `1389` | **0** |

The class, `SLICE4_SEAM_MEASURED_HARNESS_GREEN_PASS`, is the Director's own, declared at §4 of the
banked STOP-005 clearance (sha256 `c54a2a91ba8cd6dabaefeee9dfaa87d4a4defbae763be70f8560b6ba5e3247ec`,
32 lines / 2445 bytes, mode 444) and carried into the flip **with its declaring document cited**.

⚠ **The packet's sha256 in the reconciling clause is carried AS CITED by the Director, truncated in
the source.** The artifact is off-box; this seat has no route to re-derive it and does not present
the value as a measured figure.

## 3. IDS — DERIVED BEFORE THE BRIEF WAS BANKED, ON DECLARING FORMS, WITH BOTH CONTROLS

Sweep run at `1ed32329` **before** the brief was banked and before any record was drafted, so this
lane's own artifacts could not be counted as evidence about its own ids.

| space | declaring form | max at base | candidate | positive control | negative control | verdict |
|---|---|---|---|---|---|---|
| `NA-` | `^### NA-####` in `NEXT_ACTIONS.md` | `0747` (`HIGHEST_NA=0747`) | `NA-0748` = **0** decl / **0** mentions | `NA-0747` = 1 | `NA-0749` = 0 | FREE |
| `D-` | union of **all four** record forms in `DECISIONS.md` | `1388` | `D-1389` = **0** decl | `D-1388` = 1 | `D-1391` = 0 | FREE |
| `ENG-` | `^### ENG-####` in `IMPROVEMENT_LEDGER.md` | `0207` | `ENG-0208` = **0** decl / **0** mentions | `ENG-0207` = 1 | — | *not minted here* |
| `WF-` | `^### WF-####` | `0087` | — | — | `WF-0088` = 0 decl | *not minted; `WF-0088` was derived free and WITHDRAWN at R353 §9* |
| SR-16 rows | `^\| N ` in `PREDICTION_LEDGER.md` | `109` (count 109) | rows **110–112** | row 109 = 1 | row 110 = 0 pre-edit | TAKEN by this act |
| `R-` | union of banked-filename and content routes | `R359` | — | `R359` present both routes | `R360` = 0 repo, operator hits **all this lane's own sweep** | *no ruling id minted* |

⚠⚠ **THE PLANT HAZARD FIRED TWICE, FROM TWO DIFFERENT DIRECTIONS, AND NEITHER COUNT WAS ADOPTED.**

1. **This lane's own sweep planted every id it swept.** Operator-tree counts before any record was
   written: `NA-0748` 10, `D-1389` 12, `D-1390` 9, `ENG-0208` 10. Enumerated by file, **6/5/5/6 of
   them are the sweep script and its own captured output**; the rest are NA-0747's stops. **Zero
   declarations anywhere.**
2. ⚠⚠ **`DECISIONS.md` ITSELF ALREADY CARRIED `D-1389` AND `D-1390`** — one mention each, both at
   `DECISIONS.md:43582`, inside **`D-1388`'s own IDS paragraph**, which recorded them as *that*
   lane's negative controls. A mention-counting sweep would have reported both **TAKEN**, with a
   plausible reason attached, and moved to `D-1391`.

⇒ **The freeness evidence is the declaring-form count.** A mention count is never a freeness verdict
— not in the operator tree, and **not in the counter file either**, because a negative control is a
record and the tree is a plant surface like any other.

## 4. THE FILE SET, AND WHAT WAS DELIBERATELY NOT TOUCHED

Six files, all docs. CI scope class **`docs_only`**, `scripts/ci/classify_ci_scope.sh` **executed on
the real path set**, with two discriminating controls: the same set plus one `.rs` member returns
`runtime_critical`; a set carrying one `.github/workflows/` member returns `workflow_security`.

Not touched, by rule: any `.rs` in either repo · `src-tauri/Cargo.toml` and `Cargo.lock` · every
test and every `#[ignore]` · `.github/**` in both repos · every mockup byte · every
fingerprint-format byte · every fenced ruling and every sealed artifact. No dependency, no feature
enablement, no standing rule minted.

⚠ **`ENG-0202`..`ENG-0206`, `ENG-0142`'s remainder, `ENG-0194` and `ENG-0197`..`ENG-0199` stay
OPEN.** `ENG-0205` is ruled, not repaired. `ENG-0207` is ruled, not closed.

## 5. ⚠ WHAT READS FALSE IN REPO TRUTH, DELIBERATELY

`D-1387`'s four sentences saying the Phase-0 census *"re-measures ENG-0195's figure exactly"* are
**superseded, not rewritten**, in `DECISIONS.md`, `TRACEABILITY.md` and twice in `NEXT_ACTIONS.md`.
Anyone reading `D-1387` alone will be wrong. The correction is the `ENG-0195` amendment, and every
record this act writes **cites the amendment, never `D-1387`**.

## 6. OBSERVED, NOT ACTED ON

⚠ **The SR-16 table's header declares four columns; every row since well before #98 carries five**
(`| # | assertion | measurement | verdict | lesson |`). This act matched the **rows**, which are the
form in use. The header is a historical record and was not edited — but a fresh seat reading the
header rather than the rows would author a malformed row, and nothing would catch it.
