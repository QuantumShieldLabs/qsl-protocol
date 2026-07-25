# NA-0676 — operator-infrastructure TIER-1 sanitize — test plan (D612, D-1307)

The instrument is a scan. What makes its clean result evidence rather than an absence of output is
that **it was made to return positive first**, on every class it claims to cover.

## A. The instrument's positive control — run BEFORE anything was trusted (D612 §5.2, §9.4)

| # | class seeded | found? |
|---|---|---|
| A1 | personal email | ✅ |
| A2 | `192.168/16` | ✅ |
| A3 | `172.16/12` | ✅ |
| A4 | `10/8` | ✅ |
| A5 | tailnet identifier | ✅ |
| A6 | host — LAN relay | ✅ |
| A7 | host — build box | ✅ |
| A8 | ssh alias | ✅ |
| A9 | seed removed; worktree clean; fresh scan finds no trace | ✅ 0 occurrences |

⚠ **A4 is the most informative row.** `10/8` matched the seed **and nothing else in the tree** —
independently confirming the census's "no `10.x` anywhere", a result the scan could only produce by
actually working. Record: `/srv/qbuild/evidence/NA-0676/positive_control.txt`.

## B. Baseline vs the census (D612 §9.3 — report any drift BEFORE editing)

| class | D612 §1 | measured | agree |
|---|---:|---:|:--:|
| personal email | 7 | 7 | ✅ |
| `192.168/16` | 5 | 5 | ✅ |
| `172.16/12` | 16 | 16 | ✅ |
| `10/8` | 0 | 0 | ✅ |
| tailnet id | 1 | 1 | ✅ |
| host — LAN relay | 35 | 35 | ✅ |
| host — build box | 7 | 7 | ✅ |
| ssh alias | 2 | 2 | ✅ |
| **subtotal** | **73** | **73** | ✅ |
| host — laptop (added by the OBS-AK ruling, §E) | — | **3** | new |
| **corrected total** | — | **76** | |

**No drift.** The three additional hits are a class-list correction, not a change in the tree.

## C. The priority class, proven separately and first (operator ruling)

| # | check | result |
|---|---|---|
| C1 | personal-email hits before | 7, in 6 tracked files |
| C2 | after | **0** |
| C3 | across all five trees (`qsl-protocol`, `qsl-desktop`, `qsl-server`, `qsl-attachments`, `.github`) | **0 / 0 / 0 / 0 / 0** |
| C4 | the instrument can detect this exact class | ✅ proven at A1 |
| C5 | meaning preserved in all six files | ✅ every passage still says the seat inherited the machine's personal identity instead of GH007 |

## D. The full Tier-1 result

| # | check | result |
|---|---|---|
| D1 | Tier-1 hits in `qsl-protocol` | **0** (from 76) |
| D2 | `qsl-desktop` · `qsl-server` · `qsl-attachments` · `.github` | **0 · 0 · 0 · 0** |
| D3 | scan exit code across all five trees | **0** |
| D4 | files changed | 19, all `qsl-protocol` |
| D5 | insertions / deletions | +69 / −68 — line-for-line substitution |
| D6 | `git diff --check` | clean |
| D7 | per-file delta recorded by **field name only** | ✅ `tier1_delta.md` — never `original → replacement` (OBS-H) |

## E. ⚠ The census class-list gap (OBS-AK, ruled in-scope)

| # | check | result |
|---|---|---|
| E1 | a private host name used as a network target was absent from the class list | ✅ confirmed — Tier 1 by D612 §2's own definition |
| E2 | added to the instrument, sweep re-run across all five trees | ✅ 3 further hits, 3 files |
| E3 | sanitized in the account@host form | ✅ account name (Tier 2b) survives, host name redacted |
| E4 | corrected pattern set recorded for NA-0677 | ✅ in the instrument's header comment and the as-built |

**The finding, stated so Lane C inherits it:** the definition was right and the enumeration was
short. **A pattern set assembled by listing known names is only as complete as the enumerator's
memory** — and Lane C is about to install one as a merge gate.

## F. ⚠ B3 dropped — the premise, measured (§4 of the as-built)

| # | enumerated satellite hit | class | fails Lane C's ruled gate? |
|---|---|---|---|
| F1 | build-path evidence pointer (`qsl-desktop`) | Tier 2a | **No** — not scanned at all |
| F2 | retired rig host name (`qsl-desktop`) | Tier 2b | **No** — added-line scoped |
| F3 | home-path reference (`qsl-server`) | Tier 2a | **No** — not scanned |
| F4 | Tier-1 instrument on both satellites, **unfixed** | — | **exit 0 — they already pass** |
| F5 | source of the error | — | D612 §2 as drafted listed three Tier-2-class hits under a Tier-1 definition |
| F6 | disposition | — | operator ruled option (a): **dropped**, recorded, Lane B ships as one PR |

## G. Convention (ruled)

| # | check | result |
|---|---|---|
| G1 | one convention applied uniformly (D612 §4.2) | ✅ descriptive placeholder everywhere |
| G2 | no fabricated literal inside a quotation | ✅ **no TEST-NET address introduced anywhere** |
| G3 | rationale surfaced before acting, then ruled | ✅ *"quotations must never contain fabricated observations"* |

## H. Meaning preserved — by reading, not by diffing (D612 §5.3)

| # | check | result |
|---|---|---|
| H1 | every changed passage re-read | ✅ |
| H2 | grammatical breaks introduced by the backstop substitution | **4 found, 4 fixed before commit** |
| H3 | exhaustive artifact re-scan over all changed files | 5 further matches |
| H4 | those 5 classified | **all PRE-EXISTING in `HEAD`** (enumeration letters), correctly left alone |
| H5 | any document's assertion changed | **none** |

## I. Boundaries held

| # | boundary | result |
|---|---|---|
| I1 | Tier 2 untouched (B1) | ✅ 0 Tier-2 substitutions |
| I2 | history not rewritten (§6) | ✅ no force-push, no filter-branch, no amend of a merged commit |
| I3 | satellites untouched (B3 dropped) | ✅ both trees `0 changes`, still on `main` |
| I4 | no source, test-code, workflow, `Cargo.*` or dependency change | ✅ |
| I5 | no allowlist / suppression file added to make a count look clean | ✅ residue published in prose instead |
| I6 | credentials encountered | **none** — no rotation event |
| I7 | GH007 on every object, trailers empty | ✅ |

## J. What this plan does NOT establish

- **Nothing about git history**, which retains every original literal by design (§I2).
- **Nothing about Tier 2**, which is published as recorded residue rather than cleaned.
- **Nothing about the satellites' Tier-2 content**, which stays by ruling.
- ⚠ This closeout is `docs_only`, so the behavioural suites correctly **SKIP** on its merge.
  **Its green proves the governance edit is well-formed and nothing else** — the positive control and
  the per-file delta are the evidence.
