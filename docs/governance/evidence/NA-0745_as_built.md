# NA-0745 — AS BUILT (D-1384) — THE NA-0744 CLOSE-OUT RECORDS ACT

**Lane:** NA-0745 · **Decision:** D-1384 · **Rulings executed:** **R355**, **R356** (both NA-0744's)
· **Operator ruling consumed:** `ENG-0199` grade → **P3** [O] 2026-08-18 · **Base:** main
`043400729a00503b1e85d16c7a313774cb3a3ce1`, verified UNMOVED **bare and unpiped by URL** against the
NAMED GitHub remote **at the moment of assertion** and not inherited from the brief, with the
open-PR set **MEASURED 0** against a positive control that returned rows.

⚠⚠ **THIS PR IS RECORDS ONLY. ZERO PRODUCT SOURCE BYTES. NOTHING IS REPAIRED AND NOTHING IS
DESIGNED.** `ENG-0199` and `ENG-0200` stay **FILED-not-fixed**; `ENG-0142`'s remainder, `ENG-0194`,
`ENG-0196`'s disposition, `ENG-0197` and `ENG-0198` stay **OPEN**.

⚠ **Why this document exists.** NA-0744's terminal turn produced findings, an incident, a
ratification and a fact that existed **only in sealed `/srv` artifacts and the operator's mailbox**.
The Director cannot read `/srv`, and the tree did not know any of it. This act moves it into repo
truth; PART 5 carries STOP 015 §§1–4's substance so it no longer lives only under `/srv`.

---

## 1. THE PREMISES THIS SEAT OWNS (R288), MEASURED BEFORE ANY EDIT

### 1.1 Base and the open-PR set

    $ git ls-remote origin refs/heads/main          # unpiped, bare, the NAMED github remote
    043400729a00503b1e85d16c7a313774cb3a3ce1        refs/heads/main

Identical to the base the brief states ⇒ **main UNMOVED**. Open PRs: **0**. ⚠ A count of zero is
evidence only if the instrument could have returned rows, so the **positive control** was run in the
same breath: `--state all --limit 5` returned **5** rows (#1770, #1767, #1766, #1765, #1762, each
with a populated `mergedAt`). ⇒ the query discriminates, and **main is the whole id space.**

### 1.2 Every predecessor artifact verified BEFORE it was read

Each sha256 (first 16) was computed from the sealed file and compared against **both** NA-0744's
`LATEST.md` stop-history table **and** the brief's own header list. All six agree on both routes:

| artifact | sha256 (16) | lines | agrees with LATEST.md | agrees with the brief |
|---|---|---|---|---|
| `STOP_NA0744_010_TERMINAL_FROZEN.md` | `5508a330b746d1b9` | 31 | ✅ | ✅ (named, no sha) |
| `STOP_NA0744_011_IMPL_PR_OPEN.md` | `d61d07447e4e3410` | 147 | ✅ (147 l) | ✅ `d61d0744…` |
| `STOP_NA0744_012_SELF_CONTAINMENT_CLOSED.md` | `d9d1dfc89fa21901` | 371 | ✅ (371 l) | ✅ `d9d1dfc8…` |
| `STOP_NA0744_015_TERMINAL_FROZEN.md` | `a8ece897ec41bac8` | 173 | ✅ (173 l) | ✅ (named, no sha) |
| `RULING_…_R355_…20260818.md` | `f6734f7dfc894555` | 38 | ✅ | ✅ `f6734f7d…` |
| `RULING_…_R356_…20260818.md` | `2bdec747a9906f04` | 31 | ✅ | ✅ `2bdec747…` |

### 1.3 Ids re-derived AT THE EDIT (WF-0068), every space with BOTH controls

The sweep is a sealed script, `/srv/qbuild/operator/NA-0745/id_sweep.sh`, so it can be re-executed
where it sits. **Declaring forms only**, `git grep` over tracked files throughout — never the shell
`grep` wrapper, which execs `ugrep --ignore-files` and therefore honours `.gitignore`.

| space | declaring form | max at base | positive control | negative control | taken |
|---|---|---|---|---|---|
| NA | `^### NA-[0-9]{4}` | **0744** | `### NA-0744` = 1 file | `### NA-0745` = **0** | **NA-0745** |
| D | **all four forms** (below) | **1383** | `## D-1383` = 1 file | D-1384 in any form = **0** | **D-1384** |
| ENG | `^### ENG-[0-9]{4}` | **0199** | `### ENG-0199` = 1 | `### ENG-0200` = **0** | **ENG-0200** |
| WF | `^### WF-[0-9]{4}` | **0087** | `### WF-0087` = 1 | `### WF-0088` = **0** | *none minted* |
| SR-16 rows | `^\| *[0-9]+ *\|` | **93** | row 93 present | row 94 absent | **94–97** |
| R | union of routes (below) | **R356** | R356 = TAKEN | R357 = **0** on all routes | *none minted* |

**The four D-record forms, swept together** — a form-specific needle gets the right answer only by
luck, because the dominant historical form stopped before the recent one took over:
`- **ID:** D-####` **1300** / max **1312** · `## D-####` **137** / max **1383** ·
`### D-####` **7** / max **0116** · `**D-####` **4** / max **1340** ⇒ **union max 1383.**

⚠ **`ENG-0200`'s token already appears in sealed `/srv` prose, and only the declaring form in the
TREE decides freeness.** Measured: `### ENG-0200` = **0 declaring**, and `ENG-0200` = **0 mentions
anywhere in the tree**, so the sweep could not have been misled either way. The brief's expected
value is confirmed by re-derivation, not adopted from it.

⚠⚠ **The R-space has no declaring form, so neither route is sound alone and the UNION is the
instrument** — and it must be split by PATH CLASS or a lane's own banking inverts its own verdict
within minutes. Three classes excluded: **(1)** this lane's dir `/srv/qbuild/operator/NA-0745/`,
**(2)** its relay mirror `BRIEF_NA0745_*`, **(3)** the shared mutable pointer `relay/LATEST.md`.
Route A (filename) max **356** · route B (content) **raw max 391** → **classified**: `R391` is
**binary noise inside a bundled blob**, already classified as such by NA-0744's STOP 001
(*"the naive max is 999 and is wrong"*), **not a declaration** ⇒ route B classified max **356** ·
route C (repo tree) max **354**. **Union = R356.** ⇒ *classify declarations before taking a maximum.*

**All three input sets were measured**, because main alone is blind to a parallel unmerged branch by
construction: **main** (above) · the **open-PR set** (0, positive control returned rows) · the
**operator lane dirs** (max `NA-0744` before this lane created its own).

### 1.4 SR-15 — NOT TRIGGERED, stated so it can be challenged

Docs only; no code; no lock or crypto region touched; no safety mechanism retired; seven files, of
which five are governance registers, one is a gitignored evidence doc and one is a new legal record.

### 1.5 `docs_only` MEASURED, never assumed — and the control that mattered

    subject (the 7 authorized paths)              -> docs_only=true   scope_class=docs_only
    control A: same set + one product-source path -> docs_only=false  scope_class=runtime_critical
    control B: docs/legal/EXPORT_NOTIFICATION.md alone -> docs_only=true  scope_class=docs_only

Control A proves the classifier discriminates. **Control B is the one the brief demanded**, because
`docs/legal/` is a **NEW path** that did not exist at base: had it read as anything but docs, that
would have been a surprise and an SR-02 STOP. It reads as docs — `is_docs_path`'s `docs/*` case arm
matches the whole subpath — so no STOP was owed.

---

## 2. THE SEVEN LANDINGS

| # | file | what landed |
|---|---|---|
| 1 | `docs/ops/IMPROVEMENT_LEDGER.md` | `ENG-0200` **verbatim** + 3 companion bullets; `ENG-0199`'s re-grade bullet **beside** its heading |
| 2 | `DECISIONS.md` | `D-1384`, carrying the ratification transcription in a column-0 fence |
| 3 | `NEXT_ACTIONS.md` | NA-0744 `MERGING` → **DONE** + class · prior-STATE record · NA-0745 born · STATE advance |
| 4 | `TRACEABILITY.md` | one row, artifact chain ending at `D-1384` |
| 5 | `docs/ops/PREDICTION_LEDGER.md` | SR-16 rows **94**, **95**, **96**, **97** |
| 6 | `docs/governance/evidence/NA-0745_as_built.md` | this document (**gitignored — force-added**) |
| 7 | `docs/legal/EXPORT_NOTIFICATION.md` | **NEW file and directory** — the 15 CFR 742.15(b) fact |

⚠ **`docs/governance/evidence/*` is gitignored** (`.gitignore` carries a broad `**/evidence/` rule
meant for CI artifact dirs) — `git add` **silently no-ops** and `git status` never lists the file.
It is force-added and its presence confirmed in `git diff --cached --name-only`, never inferred from
`git status`.

### 2.1 The no-drift proof, per verbatim block

Every block that had to land byte-for-byte was **extracted programmatically from its sealed source's
own bytes**, landed, then **extracted back out of the landed file and compared** — and each
comparison was proven non-vacuous by a **last-character tamper control that must fail**:

    ENG-0200 (STOP 015 §2)        round-trip IDENTICAL   tamper control FAILS as required   2701 B
    §7(b) re-grade bullet         round-trip IDENTICAL   tamper control FAILS as required    861 B
    STOP 012 handoff closure      round-trip IDENTICAL   tamper control FAILS as required    938 B
    STOP 012 companions           round-trip IDENTICAL   tamper control FAILS as required    638 B
    EXPORT_NOTIFICATION body      cmp rc 0 IDENTICAL     tamper control FAILS as required    597 B

⚠ **The export-notification body was verified by COMPARING, not by hashing the destination** — a
sha of the destination proves the file exists, never that your bytes are in it.

⚠ **One assert in this lane was wrong and the tree was right.** The first ledger landing asserted
`count("- Status: open — **FILING ONLY.**") == 1`; it fired, because that phrase is **house-standard
and already present 7×** at base. The instrument was too strict for an APPEND, not the tree defective:
corrected to a **delta of exactly one** on the generic phrase plus uniqueness of the full line, which
carries this entry's own originating-lane clause. Enumerated and classified rather than refined by
guesswork, and recorded here because a silently loosened assert is worse than a failed one.

### 2.2 The §7(b) bullet, diffed against the BANKED BRIEF

Extracted from `BRIEF_NA0745_…md` (`61f40c39…`) lines **119–128**, de-indented by six columns, with
**exactly two placeholders resolved at the edit** (`<lane>` → `NA-0745`, `<D-id>` → `D-1384`). A
unified diff of the verbatim text against the resolved text reports **one changed line — the
placeholder line, and nothing else**:

    -  was ruled worth more than either argument. Landed by <lane> (<D-id>).
    +  was ruled worth more than either argument. Landed by NA-0745 (D-1384).

### 2.3 Bracket-first, and the anchor that exists twice

The `- Severity:` transcription anchor occurs **twice** in the ledger (ENG-0134 first, ENG-0142
second), so no landing may anchor on it. This landing went **bracket-first**: the bracket is
`ENG-0199`'s entry; its **heading line was read out of the file's own bytes**, never retyped;
`heading + "\n\n"` was asserted **unique-or-fail**; the bullet was inserted after that blank line.
Post-condition asserted: the heading still occurs **exactly once** and is **not rewritten**.
⚠ `ENG-0199` carried **no bullets at all** before this act — its grade existed **only** in its
heading, which is precisely why row 94 exists.

---

### 2.4 Gates executed, each with a discriminating control, all bare

| gate | subject | control | verdict |
|---|---|---|---|
| **scope class** (`classify_ci_scope.sh`) | the 7 staged paths → `docs_only` | +1 product-source path → `runtime_critical`; the NEW `docs/legal/` path ALONE → `docs_only` | ✅ subject green, both controls discriminate |
| **goal-lint** (`tools/goal_lint.py`, the linter CI's `goal-lint` job runs) | `Goals: G4` in the PR body → rc **0**, *"OK: goal compliance checks passed."* | Goals line removed → rc **1** | ✅ discriminates |
| **queue / one-READY** (`preflight_governance.sh`) | rc **1**, `FAIL: READY_COUNT=2 (>1)` | anchored re-measure: `^Status: READY` = **exactly 1** | ⚠ **THE GATE IS WRONG, THE TREE IS RIGHT — see §6.2** |
| **hygiene sentinel** (`--require-clean --fail-on-tmp --fail-on-main-pin`) | clean committed tree → **OK** | (ran dirty before the commit → failed closed on `--require-clean`) | ✅ fails closed as designed |

⚠ The queue row is reported as it measured. **It was not made green**, by any route: the script is
outside the edit set and the historical comment its needle falsely matches must not be reworded to
satisfy an instrument. The property the gate exists to protect **does** hold, measured directly.

## 3. THE HANDOFF-CLOSURE RATIFICATION — TRANSCRIBED, NOT RATIFIED

**The seat transcribes; it does not ratify.** The ratifying authority is **R355 §3**, sealed and
banked verbatim (`f6734f7dfc894555…`, 38 lines), which reads:

> §3 THE HANDOFF CLOSURE is RATIFIED into the records at lane close: a stop that hands an
> action to someone else carries what performing it requires — the third closure form
> (references → promises → handoffs), found by asking what the reader must DO next.

**No ruling id is minted and no numbered standing rule is minted** ⇒ this lane's restraint budget of
one is **UNSPENT**, and the deliberate no-mint posture is unbroken. The drafted text lands in
`D-1384` inside a **column-0 fence** (the house technique — raw `###` headings in pasted source
otherwise become siblings of the `## D-` heading instead of its content), byte-verbatim from sealed
**STOP 012** (`d9d1dfc89fa21901…`, 371 lines), lines **39–50** and **14–21**.

⚠⚠ **THE THIRD COMPANION WAS MEASURED ALREADY DISCHARGED.** R355 §3 enumerates three items; the
third is *"finding 2's executed counterfactual … Director's chair, one row"*. Measured at this base,
that row **already exists** — `docs/ops/PREDICTION_LEDGER.md` **row 85**, attributed `[Directive]`,
landed by NA-0744's own implementation PR. ⇒ **a ruling's enumeration is a description of what is
owed, and what is owed is measured against the record, not adopted from the sentence.** Re-landing it
would have manufactured a duplicate in the register whose entire point is that entries are unique.

---

## 4. CONTROL-A's DEATH, AND ITS DURABLE REPLACEMENTS

**STOP 010's warning** (the formalization seat's last act): *"The queue gate's control A is now DEAD
— main carries the same `READY` block, so a successor must rebuild it; control B, which perturbs the
property itself, survives."*

**STOP 011's execution** (the implementing seat), measured with the gate bare:

| arm | perturbation | rc | `READY_COUNT` |
|---|---|---|---|
| subject | none | **0** | **1** |
| **control A′** | the `READY` line removed | **2** | **0** |
| **control B** | a second `READY` planted | **2** | **2** |

⇒ **A′ and B bracket the property from BELOW and ABOVE and survive any base**, which is exactly what
the inherited control A did not: once the promotion merged, main carried the same `READY` block and
control A returned **the subject's own answer**. *A control is only a control at the base it was
designed for.* This act re-measured the same property on its own tree: `Status: READY` occurs
**exactly once** (`Status: READY (D-1384)`), asserted before the file was written.

---

## 5. STOP 015 §§1–4, CARRIED SO THEY NO LONGER LIVE ONLY UNDER `/srv`

### 5.1 The terminal state, verbatim from STOP 015

        main            043400729a00503b1e85d16c7a313774cb3a3ce1   GREEN (public-safety success)
    #1767 impl      MERGED 2026-08-18T14:35:55Z -> 726c3c8d
    #1770 fix       MERGED 2026-08-18T15:12:57Z -> 04340072  (operator admin-merge under R356)
    macOS on main   run 32153095986  ALL 8 JOBS SUCCESS incl. macos-qsc-shard-manifest-gate
                    and shards 0-4 and the aggregate
    E7              run 32149418050  workflow_dispatch, FIRST DISPATCH, 14:36:56Z -> 14:40:21Z
                    = 3m25s, conclusion success — **GREEN**
    sentinel        #1768 CLOSED · #1769 CLOSED  (operator-closed; the sentinel never closes)
    open PRs        0
    rulings         R355 `f6734f7d…` · R356 `2bdec747…`  both banked verbatim, id in BOTH routes

### 5.2 E7's verdict, scored clause by clause FROM THE ARTIFACT (STOP 015 §1)

The artifact `remote-invite-roundtrip-artifacts` (21937 B) was downloaded and read, and the
`summary.txt` figures were then **re-derived from the `markers` stream itself**, because a summary is
the script's own claim while the marker file is the evidence. Carried **byte-verbatim**, with no
transformation of any kind:

| sealed clause | required | measured from the artifact | verdict |
|---|---|---|---|
| `invite_finish=ok` | `ok` | `invite_finish=ok` | ✅ |
| `handshake_complete` INCREASED | strictly up | `handshake_complete_before=1` → `after=2` | ✅ |
| `qsp_unpack ok=true` BOTH directions | both | `qsp_unpack_ok=true both_directions`, count 5 | ✅ |
| `recv_commit` ≥ 1 BOTH peers | ≥1 each | `recv_commit_alice=1` · `recv_commit_bob=1` | ✅ |
| zero-residue close **with its clock antecedent** | clean AND elapsed < 30 s | `residue_verdict=clean` · `residue_elapsed_secs=11` vs `residue_lease_bound_secs=30` · `residue_attempts=1` · `recv_none_alice=1 recv_none_bob=1 recv_frame_skipped_alice=0 recv_frame_skipped_bob=0` | ✅ **and the antecedent HOLDS** — 11 < 30, so this is a RESULT, not a non-result |
| `relay_pull_diagnostic` count ≥ 1 | ≥1, **with the gate's own setting beside it** | `relay_pull_diagnostic_gate=redacted` · `relay_pull_diagnostic_count=17`; re-counted in `markers`: **17 = 11 `op=pull` + 6 `op=ack`** | ✅ |
| *antecedent:* count 0 + gate UNSET = instrument fault; count 0 + gate SET = named failure | — | count **17** with the gate **SET** ⇒ neither branch fires; a genuine result | ✅ |
| *antecedent:* each step's rc captured independently | — | `status=pass`; every required step is invoked under `if ! run_qsc_step …` and exits by name, so no later checkpoint is a non-result | ✅ |

⛳ **E7 CLOSED THE COVERAGE GAP NA-0744 HAD ITSELF ENUMERATED AS OPEN.** STOP 011 recorded that **no
in-tree Rust test can reach the ACK wrapper's HTTP 200/404 arms** (a fixture frame is `left_leased`,
never committed). E7 produced **six live `op=ack` lines** — `api=relay_pull_ack_v1 status_code=200
acked_count=1 max=1` — so the `max`-as-request-bound reading the seat flagged as an interpretation
reads correctly in a real log. *An honestly enumerated gap is a prediction about which arm will close
it, and here the arm the directive reserved for it did.*

⚠ **E6, and the instrument that was wider than its claim — recorded because it recurred in the same
seat's hands one act later.** Over the **17** diagnostic lines: 318 field instances, 20 distinct
keys, **0 keys containing `token`** and **0 values `<redacted>`**. A first, wider grep over all 167
marker lines returned 2 token-keys and 37 redactions — all in OTHER, pre-existing markers.
⇒ **state the claim's scope in the same breath as the needle.**

### 5.3 The structural finding (STOP 015 §2) → `ENG-0200`

Landed verbatim in `docs/ops/IMPROVEMENT_LEDGER.md`; see §2.1 for the round-trip proof. Its shape in
one line: **`public_safety_gate.py`'s `RED_MAIN_REPAIR_PROFILES` holds exactly ONE profile bound to
one historical defect**, so a main-red caused by anything else has no bounded path that can admit its
own fix. ⛳ **The profile's `failure_check` matched the workflow that was actually red — and it was
the only field that matched, and the only one that could not help.** The circularity is total:
"add a profile" is itself a `scripts/ci/` change facing the same gate (**self-blocking**), and
"restore main another way" **did not exist**. Resolved by **operator admin-merge under R356**.

### 5.4 The queue-flip interaction and the gate-scope facts (STOP 015 §3)

Both landed as `ENG-0200`'s own bullets. In brief: the ordered `READY → MERGING` flip **removes a
precondition** of the red-main-repair path (`queue proof does not show exactly one READY item`), so
**a lane that follows its instructions cannot satisfy that clause afterwards** — a property any
generic profile must handle, **not a defect in the flip**; and `macos-qsc-shard-manifest-gate` is
**visible on PRs and NOT required**, which is the correction to the premise offered at STOP 013.
Making it required is branch protection, therefore the operator's, and deliberately deferred to the
CI-migration lane: **recorded, not requested.**

### 5.5 The incident, in order (STOP 015 §4), verbatim

        13:55/13:58Z  macos-qsc-shard-manifest-gate FAILS on PR #1767 (both commits) -- VISIBLE, and
                  non-blocking; nobody read it, this seat included (SR-16 row 93)
    14:35:55Z     #1767 merged -> main 726c3c8d
    14:35:58Z     the same gate fails on main; sentinel opens #1768, then #1769 by propagation
    14:36:56Z     E7 dispatched (unaffected by the manifest defect) -> GREEN in 3m25s
    ~14:50Z       cause CONFIRMED from the log before anything was touched
    14:55Z        #1770 opened: one manifest row (shard 2, by the manifest's own re-mined seed)
                  + SR-16 rows 91-93. The repaired gate PASSES on the PR (7s).
    ~15:00Z       public-safety FAILS on #1770 -- the main-red blocker, both bounded paths
                  refusing correctly. Diagnosed, not bypassed; escalated as STOP 014.
    15:12:57Z     operator admin-merge under R356 -> main 04340072
    15:13:00Z     macOS push run 32153095986 GREEN, all 8 jobs
                  main public-safety success; #1768 and #1769 operator-CLOSED

---

## 6. WHAT THIS ACT FOUND ON ITS OWN, AND DID NOT TOUCH

### 6.1 A records defect in the precedent this lane was told to follow

⚠⚠ **A RECORDS DEFECT IN THE PRECEDENT THIS LANE WAS TOLD TO FOLLOW.** The brief's §7(b) bullet
cites *"the ENG-0142 re-grade precedent"*. Measured at this base, commit **`a1c6c969`** (NA-0742,
D-1378) inserted that re-grade bullet at `docs/ops/IMPROVEMENT_LEDGER.md:3720` — **between the first
and second physical lines of the `- Severity:` bullet it annotates**. `ENG-0142` therefore now reads:

    - Severity: **P1** — ⚠ **TRANSCRIBED FROM THIS ENTRY'S OWN HEADING**, which has read `⚠ P1` since the
    - ⚠ **Severity, RE-GRADED 2026-08-16 to `P2` by the OPERATOR …**
      entry was filed 2026-08-09. This bullet is a transcription of an existing declaration into the
      field a triage needle can read. **It is not a new judgment and must not be read as one.**

`ENG-0134` carries the **identical** transcription bullet **unsplit** at `:3666-3668`, which is the
only reason the break is visible at all. **The anchor existed twice and the landing anchored on a
FRAGMENT of one of them** — the exact hazard the brief's §2(d) warns about, demonstrated by the
precedent it names. ⚠⚠ **NOT REPAIRED HERE.** `ENG-0142`'s remainder is outside this lane's bounds
(brief §10), and a records act that quietly reaches into an entry it was told not to touch is worse
than the defect. **Filed as SR-16 row 96 and left for the act that owns that entry.**

### 6.2 A GATE THAT COUNTS MENTIONS — FOUND BY RUNNING IT ON THIS LANE'S OWN TREE

⚠⚠ **`scripts/ci/preflight_governance.sh` FAILED this correct tree**, on the committed, clean
commit, with:

    FAIL: READY_COUNT=2 (>1)

**Measured rather than worked around.** The gate's needle is `rg 'Status:\s*READY' NEXT_ACTIONS.md`
— **unanchored** — so besides this lane's genuine `Status: READY (D-1384)` it matches a **MENTION
inside a historical `<!-- prior: STATE… -->` comment** at `NEXT_ACTIONS.md:92`, whose prose reads
`` `Status: READY` becomes `DONE 2026-08-02` ``. That comment was landed by **`b76016b5`**, NA-0690's
promotion, on 2026-08-02.

**The gate has therefore not reported the true number since that day, and it is wrong in BOTH
directions:**

| base | gate needle | anchored truth | direction |
|---|---|---|---|
| `9d11e2bd` · `4c59ffda` · `21597277` · `ae2047e6` · `d484c065` · `04340072` (settled mains) | **1** | **0** | **permissive** — it reports one READY where there is none |
| `ea0ee23e` (NA-0744's promotion tree) | **2** | **1** | **blocking** — it reports a violation that does not exist |
| this lane's commit | **2** | **1** | **blocking** (the failure above) |

⛳ **At `4025eb44`, NA-0744's implementation PR head, the ONLY line the needle matches is that
comment** — so a `READY_COUNT 1` read at that head counted the comment and not a lane. *A count that
is right can still be right for the wrong reason; read WHICH lines matched, never how many.*

⚠⚠ **NOT REPAIRED — AND THE WORKAROUND WAS REFUSED TOO.** `scripts/ci/**` is outside this lane's
edit set. `NEXT_ACTIONS.md` **is** in the edit set, so rewording line 92 would have made the gate
pass — and that is exactly the antipattern this program exists to prevent: **editing a historical
record to satisfy an instrument**. Neither was done. **Recorded as SR-16 row 97**; whether it earns
a countable `ENG-` id is the Director's call, since a new id is outside the brief's enumeration.

⚠ **Blast radius MEASURED, not assumed.** `git grep preflight_governance -- .github/` returns **0
hits** ⇒ **no workflow runs this script**; nothing in CI has ever been gated on it. This is the same
shape NA-0744 recorded for `preflight_qsc_impl.sh` — *a script the house treats as a required gate
that no workflow executes*. The **admission** gate is a different instrument:
`scripts/ci/public_safety_gate.py` derives its queue proof from a structured `status` field
(`entry.get("status") == "READY"`), not by grepping prose, and its red-main path is not active
because main is green.

⛳ **The invariant itself HOLDS on this tree**, measured with an anchored needle: `^Status: READY`
matches **exactly one** line, `Status: READY (D-1384)`, asserted before the file was written and
re-measured after.

---

## 7. CLAIM BOUNDARY

**Records only.** No product source byte, no `.github/**`, no script, workflow, dependency or lock.
No test weakened, skipped or deleted. No sealed artifact or fenced ruling edited — STOPs 010, 011,
012 and 015 and rulings R355/R356 all stand byte-unchanged. No GitHub issue opened, closed or
commented on. No `sudo`, no secret read, no re-run-to-green. **Nothing merged by this seat: the
operator merges; the seat does not.**

⚠ **One ordering question is reported rather than silently reconciled.** The brief's §6(d) asks the
Director to rule the ratification *"before any PR opens"*, while its §9 asks the same stop to carry
*"the PR number and head sha"*. Both cannot be satisfied by one stop. It is resolved in the direction
that usurps nothing: the authority cited is the **pre-existing R355 §3**, **no ruling id and no rule
is minted**, the drafted text is quoted verbatim with its source sha in the stop so the Director
rules **from the artifact** (SR-01), and the **PR is open, not merged** — so nothing reaches repo
truth before that ruling. If the Director intends a distinct ruling on the landed wording, the
wording is in the stop and can be amended before the merge.
