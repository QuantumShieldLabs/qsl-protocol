# NA-0746 — AS BUILT

**Lane NA-0746 · promotion `D-1385` (PR #1772, merged `11ff0debb9cf4ca3f5f131fecf0964ebf58883b0`) ·
implementation `D-1386` · ruling `R358`.**

This document exists so that nothing produced by this lane lives only in an upload. It carries the
as-built record, **the substance of the SR-15 adversarial cold read** whose findings governed the
outcome, and the facts the Director tagged as **CI-MIGRATION LANE INPUTS**.

---

## 1. WHAT THE LANE SET OUT TO DO, AND WHAT IT ACTUALLY SHIPPED

`ENG-0200` recorded that a red main has no admission path for its own fix unless the failure matches
one hardcoded historical profile, measured live on PR #1770 where both bounded attempts refused the
correct one-line fix and the operator admin-merged under R356. NA-0746 was formalized to design a
**generic, strictly-bounded** replacement.

**The design was REFUSED.** An SR-15 adversarial cold read (`FINDINGS_SR15_NA0746_READ_20260818T195721Z.md`,
sha256 `b1cc32e54c321493427d369ea63dd6257c957540390dab486ea4943e6a87b1fa`, 825 lines) returned
**7 BLOCKER / 13 MAJOR / 3 minor**; the Director ruled every blocker **ACCEPTED and independently
fatal**, ordered **no amended design**, and **RATIFIED R356** as the recorded procedure instead.
`ENG-0200` closes under that ratification.

**What shipped** is the one repair the ruling ordered — the mention-counting queue needle, repaired
at **both** of its sites (`ENG-0201`) — plus three findings filed and not fixed (`ENG-0202`,
`ENG-0203`, `ENG-0204`).

**Not built, and absent from the tree:** the generic profile, any widened queue regex, any fixture
change, any `public_safety_gate.py` byte, any `.github/**` byte.

---

## 2. THE SR-15 FINDINGS, IN SUBSTANCE

### Blockers — each ruled independently fatal

- **B1** — E1, the sealed **positive control**, is itself refused by the design's own settled-main
  clause when replayed at the incident's real timing. The seal could not have passed.
- **B2** — the "no other failure on the PR head" containment is evaluated on an **unsettled board**,
  and the checks it cannot yet see are exactly the ones branch protection does not block on.
- **B3** — the design's central input (`pr["checks"]`) has a provenance the design does not control:
  its shape is chosen by an `if/else` driven by the **historical** profile's prelims, and on the happy
  path it **structurally excludes both #1770 contexts**. The delta list had been declared complete.
- **B4** — a PR that **deletes, neuters or renames** the failing check is admissible, and the evidence
  it presents is **identical** to a genuine repair.
- **B5** — E4's second proof has **no instrument**, and its obvious candidate cannot return what the
  arm demands.
- **B6** — E7 and §6 seal **two contradictory expected values for one instrument** (21 vs 33).
- **B7** — the negative arms cannot enforce their own antecedents through the seam §6 names: the
  scorer tests **polarity only** and never asserts the refusal *grounds*.

### Majors

- **M1** — the reachability measurement "that decides the lane's shape" is narrower than claimed, and
  C3's stated price rests on a premise that measures false: a **third** bounded admission path
  (`validate_self_repair_bootstrap_pr`) was wired the whole time, its root set containing both the
  workflow and the gate file, eligible only when main's `advisories` is red **and** the PR classifies
  `workflow_security`. On #1770 `advisories` was green, so the incident-scoped claims stand.
- **M2** — the derived red set is read from an **unpaginated 100-item call**; main measured at **92**.
  → filed as `ENG-0203`.
- **M3** — the queue proof binds a **count, never an identity**, and the identity flag is passed by no
  workflow. → part of `ENG-0202`.
- **M4** — the widened `(READY|MERGING)\b` reader **accepts annotations that negate the status**. The
  refutation is recorded inside `ENG-0202` so the eventual repairer does not rebuild it.
- **M5, M6, M10** — see §3, CI-MIGRATION LANE INPUTS.
- **M7** — the transient hazard is **constructible**, and nothing in the design ties the admitted PR
  to the repair. This became the ruling's **reopening condition**.
- **M8** — the cap of 3 refuses **the program's own mandated PR shape**, measured at **83%** of recent
  merges.
- **M9** — row 97's defect has a **second, byte-identical site** that the design left unfixed, and
  fixing one makes two governance instruments disagree about one file. → this lane's repair covers
  **both**; became `ENG-0201`.
- **M11** — the `public-safety` exclusion is necessary and its refusal correct, but the state it
  leaves uncovered — main red **only** on `public-safety` with `advisories` green — has **no bounded
  path at all**. Recorded in the `ENG-0200` amendment as the **unpriced state, now priced**.
- **M12** — the settled-main clause's liveness cost measured at **~50 minutes** and unbounded on a
  wedged run. Died with the design that carried it.
- **M13** — `fixture_required_contexts()` has drifted from live branch protection (**14 vs 15**,
  missing `infra-literal-scan`). → filed as `ENG-0204`.

### Minors

- **m1** — the seat's rc-2 claim was scoped to two routes but read as a property of the script; the
  sites `:19/:26/:33` do exist. The durable observables remain **COUNT for A′, RC for B**.
- **m2** — the deferred measurement is answerable now: **no workflow runs** the fixture proofs, the
  preflight, or the post-merge verifier.
- **m3** — two small counting notes, both in the stop's favour.

---

## 3. CI-MIGRATION LANE INPUTS

*Tagged by the Director's ruling §R3. These bear directly on the required-set decision that lane
owns, and are recorded here rather than acted on — making a gate required is branch protection, and
therefore the operator's.*

### 3.1 A required-looking check that is `skipped` on every PR **by construction** (M6)

`.github/workflows/macos-qsc-sharded-suite.yml` triggers on `pull_request: {}`, and the aggregate job
that mints the check name `macos-qsc-sharded-suite` is gated:

```yaml
  aggregate:
    name: macos-qsc-sharded-suite
    if: >-
      always() &&
      (github.event_name != 'pull_request' ||
      startsWith(github.head_ref, 'na0724-'))
```

⇒ **on every PR whose branch name does not start with `na0724-`, the check is `skipped` — always.**
That is why #1770's head showed it `skipped`. It is a property of **every PR in the repo**, not of
that incident. Any future rule that reasons over "the checks red on main are green on the PR" must
know that this member is satisfied on **100%** of PR heads by a skip no PR can avoid and none earns.

### 3.2 `success` minted without validation on a docs-only PR (M6, second instance)

`macos-qsc-shard-manifest-gate` reports **`success` without running any validation** when
`docs_only == 'true'`: its real steps are guarded `if: needs.classify.outputs.docs_only != 'true'`
and the docs branch is a bare echo. ⇒ **`success` does not imply the check ran.** Any design that
tests a conclusion *string* inherits this.

### 3.3 A `failure` deny-list where the file's own precedent uses an accepted allow-list (M5)

GitHub's conclusion vocabulary is `success | failure | neutral | cancelled | timed_out |
action_required | stale | skipped`. A containment worded as *"no check is `failure`"* treats
**`cancelled`, `timed_out`, `action_required` and `stale` as passing** — and this repo's own workflow
bytes record real incidents of the first two (a later merge cancelling an earlier merge's suite; a
watchdog exhausting its budget while a suite was still `in_progress`, taking main red with both
suites green). The gate already owns the correct polarity twice: `ACCEPTED_CHECK_CONCLUSIONS` and
`check_completed_non_failing`. ⚠ Stated honestly: across 12 recent merged PR heads, **594 check-runs**
sampled showed only `success` (520), `skipped` (72), `failure` (2) — **no live instance** of the four
leaked conclusions. The finding rests on the documented vocabulary, the repo's own recorded
incidents, and the file's own precedent.

### 3.4 The emission namespace is not as separate as a prefix suggests (M10)

An AST walk over `public_safety_gate.py` finds **143 emitted string sites** (108 `print`, 28
flattenable `.append`, 7 `SystemExit`), of which **44** carry an `ERROR:`/`ALLOW:`/`WARN:` prefix —
against a census that had counted **32**. Two entire live validators and **two of the gate's three
ALLOW lines** (`:1210`, `:1390`) were outside it. And prefixing does **not** confer distinctness under
substring matching: old and new refusals reach the log through the same `print(f"ERROR: {error}")`,
so a prefixed string can **contain an existing refusal verbatim**, and a proposed queue refusal shared
its first **43 characters** with an existing one — the `established` / `established_recv_only` prefix
defect, reproduced.

---

## 4. THE REPAIR AS BUILT (`ENG-0201`)

Two sites, one line each, pattern only:

| file | before | after |
|---|---|---|
| `scripts/ci/preflight_governance.sh:39` | `rg -n 'Status:\s*READY' NEXT_ACTIONS.md` | `rg -n '^Status: READY\b' NEXT_ACTIONS.md` |
| `scripts/ci/post_merge_verify.sh:106` | `rg -n 'Status:\s*READY' "$NEXT_ACTIONS_TMP"` | `rg -n '^Status: READY\b' "$NEXT_ACTIONS_TMP"` |

Both line numbers were **re-derived by content anchor**, not adopted. `preflight_governance.sh:91`
carries a different needle (`Status:\s*(READY|DONE|BACKLOG)`, a display helper ending `|| true`) and
is deliberately untouched.

### V1 / V2 — five COMMITTED tree states, both instruments

| tree state | V1 `preflight` COUNT / rc | V2 `post_merge_verify` COUNT | sealed | verdict |
|---|---|---|---|---|
| settled main | 0 / 0 | 0 | 0 / rc 0 | MET |
| promotion, annotated `READY (D-…)` | 1 / 0 | 1 | 1 / rc 0 | MET |
| promotion, bare `READY` | 1 / 0 | 1 | 1 / rc 0 | MET |
| B — two READY | 2 / 1 | 2 | 2 / rc 1 | MET |
| A′ — READY removed | 0 / 0 | 0 | 0 / rc 0 | MET |

**Counts agree on every arm.** The historical `<!-- prior: STATE… -->` comment is matched by the
**pre-repair** needle in **all five** states and by the **repaired** needle in **none**.

⚠ **ANTECEDENT honoured:** every perturbation was **committed**. A dirty tree exits **rc 1** at
`hygiene_sentinel.sh --require-clean` — the *same* rc as the count failure — so an uncommitted
perturbation is confounded and proves nothing.

⚠ **A contract difference, recorded rather than smoothed.** `preflight` fails when the count exceeds
1; `post_merge_verify` fails when the count differs from `--expect-ready`, and its validator
**rejects any expectation above 1** (`error: --expect-ready must be 0 or 1`, rc 2). The two-READY
state therefore **cannot be stated** to the second instrument; its count on that arm is read from its
own printed `ready_count=` output. ⇒ **`V1 == V2` is an equality of COUNTS, not of exit codes.**

### V3 / V4

**V3** — the `ENG-0200` amendment round-tripped against its banked source: **1716 bytes, sha256[:16]
`fbcad35d43f5b2e9`** on both sides, DIFF EMPTY, last-character tamper control NON-EMPTY, `<R-id>`
resolved to `R358`. **V4** — every derived id **0-declaring at base** with a positive control per
space; row max re-derived (99 → 103).

---

## 5. IDS, AND HOW THEY WERE DERIVED

`D-1386` · `ENG-0201`–`ENG-0204` · SR-16 rows **100–103** · ruling `R358`. All swept at the edit on
**declaring forms** with `git grep` (never a `.gitignore`-honouring recursive search), `D-` across
**all four** record forms, **counted by line** — a `git grep -o` count split on whitespace scores one
`## D-####` heading as two, which is how this lane first mis-measured its own declaration count.

⚠ **`R358`'s route A is blind by construction:** the v2 ruling was banked as
`DIRECTIVE_NA0746_V2_…md`, whose **filename carries no R-id**, so the declaring-filename route cannot
see it. This is WF-0087's recorded route-A weakness, live again. Recorded, not fixed — the banked file
is sealed 444 and is never renamed.

---

## 6. CLAIM BOUNDARY

`scripts/ci/public_safety_gate.py` is **not edited by this lane at all**; the refused profile exists
nowhere in this tree; no fixture is touched; no queue regex is widened. No `.github/**`, no product
source, no test, no dependency, no lock. No test weakened, skipped or deleted. No standing rule
minted. No sealed artifact or fenced ruling edited, and the historical comment is **not reworded** —
the needle was fixed instead. `ENG-0142`'s remainder, `ENG-0194`, `ENG-0197`, `ENG-0198`, `ENG-0199`
and `preflight_qsc_impl.sh` stay OPEN and untouched.

⚠ **Neither repaired script is run by any workflow** — `git grep` over `.github/` returns 0 for both —
so **no CI behaviour changes with this repair**. The instruments are operator- and seat-run, and that
is precisely who the wrong number misled.
