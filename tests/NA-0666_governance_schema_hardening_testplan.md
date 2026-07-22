Goals: G4

Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-22

# NA-0666 — Governance-schema hardening: test plan (D602 / D-1292)

**This lane writes MARKDOWN and executes nothing.** There is no code path to
exercise, no vector to add, and no regression to guard — so this plan is a
**verification plan over documents**, not a software test plan. Saying that
plainly is part of the plan: a testplan that implied executable coverage where
none exists would be the same class of overclaim the lane exists to prevent.

**Nothing here asserts that any filed defect is fixed.** Every check below is
satisfied by an entry *existing and being internally consistent*. The defects
remain broken at merge; the entries say so themselves.

---

## 1. Mechanical scope gate (the cheapest and most direct check)

| # | check | method | expected |
|---|---|---|---|
| 1.1 | the full changed set is docs-only | `bash scripts/ci/classify_ci_scope.sh <every changed path>` | `docs_only=true`, `scope_class=docs_only` |
| 1.2 | no file outside the D602 allowed-path list | `git diff --stat origin/main...HEAD` | every path present in the allowed list; **zero** others |
| 1.3 | no forbidden path in the diff, for any reason | grep the diff name-list for `scripts/`, `.github/`, `CLAUDE.md`, `src/`, `Cargo.`, `tests/*.rs`, `formal/`, `specs/`, `schemas/`, `.claude/`, `qsl-desktop` | zero matches |

**Any result other than `docs_only=true` at 1.1 means a forbidden path entered
the diff — STOP, do not adjust the classifier.**

## 2. Ledger filings

| # | check | expected |
|---|---|---|
| 2.1 | each new ID appears exactly once as a heading | `grep -c '^### <ID> ' ` = 1 for WF-0026, WF-0027, WF-0028, WF-0029, WF-0030, WF-0031, WF-0032, ENG-0062 |
| 2.2 | no ID was reused | each of the eight was absent at Phase 0; WF-0025 was the highest WF and ENG-0061 the highest ENG |
| 2.3 | every new entry carries the filing status | each has `Status: open — filed 2026-07-22 by NA-0666 (D-1292)` and the **FILING ONLY** marker |
| 2.4 | the OBS-A collision is recorded on **both** sides | WF-0027 and WF-0028 **each** state that they collided on the word "FAIL" and are unrelated |
| 2.5 | blocked items name their blocker | WF-0027, WF-0029, WF-0030 each carry `Sequencing:` naming **WF-0031**; ENG-0062 names **WF-0030** |
| 2.6 | WF-0025 amended, not rewritten | its `Severity`, `Description`, `Proof gap`, and `Status` lines are byte-unchanged; the cluster note retains its original census sentence and gains only the mapping |
| 2.7 | latency of claim | WF-0026 states the defect is **LATENT, not live**, and that all seven invisible headings are `Status: DONE` |

## 3. The four §4a conventions

| # | check | expected |
|---|---|---|
| 3.1 | all four are present in `DOC-OPS-006 §4a` | relay file; proactive observations; queue verification; relay-vs-response |
| 3.2 | the queue rule carries its own limitation | §4a states the helper exits 2 at `READY_COUNT 0`, that a closeout records `READY_COUNT 0` + `Status: DONE` agreeing, and that `--allow-nonready-count` must not be passed |
| 3.3 | **the queue rule is SATISFIABLE at this lane's own closeout** | see §6 — the self-consistency gate |
| 3.4 | the relay-vs-response determination is evidence-backed | §4a cites the cadence/key/scope/mandate contrast and the three post-adoption directives that still name a `Response file target:` |
| 3.5 | the response file is recorded as **still owed** | §4a says so explicitly, and names NA-0664 and NA-0665 |
| 3.6 | **no supersession is manufactured** | §4a does **not** claim the relay file retired the response file |
| 3.7 | the split authority is stated in **both** places | the `§4a`-vs-`CLAUDE.md:47-50` split appears in the §4a text **and** in the lane response |

## 4. Schema work

| # | check | expected |
|---|---|---|
| 4.1 | `Sequencing` is a first-class field | present in the `DOC-AUD-001 §6` mandatory schema list with its minimum meaning |
| 4.2 | the severity-vs-sequencing distinction is stated | "severity is an impact scale; sequencing is a calendar" |
| 4.3 | four users migrated, content unchanged | ENG-0051, ENG-0053, ENG-0054, ENG-0058 — deadline, meaning and severity byte-equivalent; only ENG-0053's field shape changed |
| 4.4 | the field is scannable | `grep '^- Sequencing:'` over the ledger returns **10** bullets (4 migrated + 6 new) with no non-canonical `- **Sequencing` remaining |
| 4.5 | recovery marker present and forward-only | `DOC-OPS-003 §3a` defines `REC-<NNN> · <CLASS> · <class-key>` with `CLASS` ∈ `DEFECT`/`HAZARD`/`ONE-OFF`, and states the forward-only limitation |
| 4.6 | the marker distinguishes the three classes the journal could not | defect that bit / hazard correctly anticipated / unrelated one-off |
| 4.7 | NA-0664's analysis is referenced, not restated | §3a points at the journal section and does not reproduce it; the journal gains a forward pointer |
| 4.8 | **no retro-labeling** | zero `REC-` markers appear in journal entries above this lane's own |
| 4.9 | the template matches §3a | `TEMPLATE_Rolling_Operations_Journal` shows the marker in both the blank and worked-example `Failures / recoveries` sections |
| 4.10 | §2 counter source repaired | `DOC-OPS-006 §2` names `/srv/qbuild/operator/directives/`, records the drift as observed fact, and does not propose renaming the 575 files |

## 5. Governance patch

| # | check | expected |
|---|---|---|
| 5.1 | D-1292 present exactly once, dated, accepted, mapped to Goals | `grep -c '^- \*\*ID:\*\* D-1292$' DECISIONS.md` = 1 |
| 5.2 | D-1291 still present exactly once | unchanged |
| 5.3 | STATE flipped | `READY=NONE \| HIGHEST_NA=0666 \| HIGHEST_D=1292` |
| 5.4 | the NA-0666 section agrees | exactly one `### NA-0666` heading, `Status: DONE`, and **zero** `Status: READY` sections remain |
| 5.5 | owed list updated | the docs-only LITE lane discharged; the `Sequencing` count corrected **three → four**; the `CLAUDE.md` pointer and step-6 correction recorded as still owed under WF-0032; the NA-0664/NA-0665 response files recorded as owed |
| 5.6 | TRACEABILITY row present | one row for NA-0666 / D-1292 |
| 5.7 | journal entry uses the new marker | this lane's entry carries `REC-001` and `REC-002` in the §3a shape — **the marker's first use, which is its first honest test** |

## 6. THE SELF-CONSISTENCY GATE (the most important check in this lane)

**The lane's own closeout must pass the queue-verification rule the lane just
wrote.**

| # | check | expected |
|---|---|---|
| 6.1 | run the rule as written | `python3 scripts/ci/qsl_evidence_helper.py queue` → `READY_COUNT 0`, **exit 2** |
| 6.2 | the exit code is **not** treated as failure | exit 2 at `READY=NONE` is the WF-0025 behaviour, pre-recorded by D602's OBS-N, and correct — no lane is ready to start |
| 6.3 | both layers agree | STATE reads `READY=NONE`; the `### NA-0666` section reads `Status: DONE` |
| 6.4 | the flag was **not** used | `--allow-nonready-count` was not passed at any point |
| 6.5 | **the rule is satisfiable in this state** | §4a's closeout clause asks for `READY_COUNT 0` + `Status: DONE` agreeing, and **explicitly forbids requiring exit 0** — so the state at 6.1/6.3 satisfies it |

**Pass criterion: 6.5, not 6.1's exit code.** If the rule as written could *not*
be satisfied here, **the RULE would be wrong, not the closeout** — and the
correct action would be to STOP and report the contradiction, never to weaken
the closeout or manufacture a green exit code. NA-0665 established the
principle and the operator confirmed it: **a gate that cannot pass teaches
bypass.**

## 7. Claim boundary

| # | check | expected |
|---|---|---|
| 7.1 | no fix is claimed | the response and every entry state that zero defects were fixed |
| 7.2 | no derived-safety claim | **no** claim that the queue is correct, that `qwork` is safe, that the identity hazard is resolved, or that the journal is countable |
| 7.3 | suite-skip stated honestly | this merge is `docs_only=true`, so **both full suites SKIP**; a green merge is **not** evidence about suite health (ENG-0052 clause (d)) |
| 7.4 | no prohibited claims | no public/production/crypto-complete/attachment-complete/bug-free/vulnerability-free claims anywhere |
| 7.5 | raw private values | none published; proof-root-only |
