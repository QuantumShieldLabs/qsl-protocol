# NA-0675 — as-built (D611, D-1306) — claim corrections + the response-file retirement

Result class: **`CLAIM_CORRECTION_PASS`**. Single LITE-CEREMONY spine closeout (DOC-OPS-006 §9).

## 1. What shipped

| repo | PR | merge | class |
|---|---|---|---|
| `qsl-desktop` | #12 (D-0012) | `29eef441` | code+docs — `[rust]` **GREEN** |
| `.github` | #5 | `30244e4a` | docs — ⚠ **no CI exists in that repo** |
| `qsl-protocol` | this closeout (D-1306) | — | docs_only |

Base: desktop `b4ea47e7`, spine `000f99e2`. Seat `qwork NA-0675 qsl-desktop qsl-protocol`,
`19:45:05Z`, `qwork_version_or_sha 620a98ff…`; spine seat proof `ready_count=1`,
`queue_top_ready=NA-0675`, `requested_lane_status=READY`; both seats clean at `origin/main`.

## 2. The defect, and the correction

Both pages said the desktop build **"makes no network connections at all"**. That stopped being
true at NA-0673 (the Server pane) and was redesigned at NA-0674. The app's own surfaces were swept
at NA-0673; **the pages about the app were not.**

**The corrected claim was proven before it was written** (D611 §7.4, run at Phase 0 on the seat):

```
grep -c 'invoke("relay_test"' ui/main.js   ->  1
```

…and that single call site sits inside the Test-connection handler; every other frontend `invoke`
is local (vault, settings, identity, protection). So the published sentence is:

> **The app opens a network connection only when you press Test connection.** Nothing connects at
> launch, in the background, or on a timer.

with the still-true boundary kept: **no messaging, no release**. The desktop README's
claim-boundary paragraph is byte-unchanged. On the profile page, **"The desktop client cannot send
a message" is kept verbatim** — it remains true and is the load-bearing half of the sentence.

**The redesign is noted by behaviour, not by lane number** (Director ruling, 2026-07-25): a public
page's reader learns more from *"Save commits the pane; Test saves first"* than from `NA-0674`.

## 3. The four census corrections (all folded as binding)

| | correction | disposition |
|---|---|---|
| **C1** | the profile README's Repositories table **already listed all four repos** | the intent's item 2 was a **no-op**; table byte-unchanged, zero rows touched |
| **C2** | the same file carried the identical false claim three lines below that table | **the real work there** — substituted under FLAG-A1 and **recorded**, in the commit and here |
| **C3** | its "Working today" desktop line was understated | now names server configuration and the connection test |
| **C4** | the intent's open question on the idle-autolock default | **already settled — 60 SUPERSEDED 15** by the operator's own qsl-desktop D-0005 decision (`81cad54`); residue filed as **WF-0024**; reverting was forbidden and **did not happen** (`settings.rs` byte-untouched) |

## 4. The A2 pin, and its positive control

`claim_discipline_five_surfaces_swept` now asserts `README.md` alongside the app surfaces.
**Why it was missing is the whole point:** that block covered `index.html`, `main.js`, `lib.rs` and
`commands.rs` — the app — and nothing covered the page about the app, which is why one sentence
survived two lanes that were explicitly retiring it.

`repo_file()` resolves `CARGO_MANIFEST_DIR/../<rel>` and **panics** when the read fails, so the pin
cannot pass by silently reading nothing — the flag's worry was already answered by the existing
helper.

**Positive control** (`/srv/qbuild/evidence/NA-0675/a2_pin_positive_control.txt`):

```
A. BEFORE — panicked at src-tauri/tests/server_pane.rs:429:5:
   README status section still says makes no network connections at all
   test result: FAILED. 0 passed; 1 failed
B. AFTER  — test result: ok. 1 passed; 0 failed
```

*A negative result is only evidence if the instrument could have returned positive.*

**Residual occurrences of the retired phrase in `qsl-desktop`: five, and none is a live claim** —
three in `server_pane.rs` (two guard needles + the failure message) and two in `DECISIONS.md`
quoting what was retired.

## 5. §2a — the response-file retirement (operator ruling, added at approval)

`docs/ops/DIRECTOR_OPERATIONS.md` §4a.4 now ends with a dated retirement note.

- **Marked, not rewritten.** §4a.4's evidence table, its measured discrepancy, and its explicit
  *"the relay file did NOT supersede the response file"* are all left legible. **A reversal that
  erases the reasoning it reverses reads as drift.**
- **The operator's rationale is recorded as given:** relay files have de facto superseded it and
  are the better instrument — every turn rather than per milestone, self-contained for a reader who
  never saw the terminal, and actually written.
- **The note also states what the old text got right:** refusing to invent a supersession from a
  lapse was correct. *An executor must not retire an operator's convention by noticing it has
  lapsed.* The lapse was evidence for a decision; it was never a decision.
- **Five obligations discharged BY the retirement, by name:** NA-0664, NA-0665, and the
  `Response file target:` lines of **D611, D612, D613**.
- **Consequences applied in the same document:** §1's template element retired; §3's path and its
  **575 archived files kept as readable history** (nothing renamed or removed); §4a.1 marked as the
  **sole** required reporting artifact.
- ⚠ **`CLAUDE.md` step 6 deliberately NOT edited** (WF-0032 — not a docs path; touching it fires
  both full suites and costs this lane its `docs_only` class). **Its staleness has flipped
  direction:** it previously stated a live requirement while omitting the relay convention; it now
  states a **dead** one. DOC-OPS-006 is the authority; the edit rides WF-0032's sequencing.

## 6. Not done, deliberately

- **OBS-P — the spine's own public claim documents** (`docs/public/**`,
  `RELEASE_READINESS_EVIDENCE_MAP.md`, `EXTERNAL_REVIEW_PACKAGE.md`,
  `SUITE2_TRIPLE_RATCHET_CLAIM_BOUNDARY.md`) still carry "desktop GUI guided prototype readiness"
  language about the qsc-desktop prototype retired at NA-0651 (D-1274), and never mention the real
  client. **Operator ruling: a real fourth-lane candidate — file nothing, fix nothing, no ledger
  entry; the Director drafts a Lane D intent after Lanes A–C.** Nothing was touched.
- **The autolock default** (C4) — reverting 60→15 would have silently reversed a sanctioned
  operator decision on a docs lane.
- **`src-tauri/src/**`** — 0 files, verified.

## 7. ENG-0064, reproduced again and captured for Lane C

The NA-0675 spine seat came up pointing at the **desktop's** target dir
(`CARGO_TARGET_DIR=…/targets/qsl-desktop/…`, `source=preexisting-env`,
`explicit_target_preserved=yes`). **Third consecutive sighting** — NA-0670 filed it, NA-0674
reproduced it, NA-0675 captured it — and the first two were noticed only because an executor
checked by hand.

At the operator's instruction the baseline is preserved for **NA-0677 / D613 §2d (FLAG-C4)** at
`/srv/qbuild/evidence/NA-0677/`: the spine env (the defect), the desktop env (the correct
contrast), the startup proof (which also shows it carries **no** target-dir assertion and **no**
commit-identity line — ENG-0064's and ENG-0074's proof gaps side by side), and a README explaining
what it proves. **Nothing in `/srv/qbuild/tools` was touched**; this lane used the same explicit
`CARGO_TARGET_DIR` workaround its predecessors did.

## 8. Claim boundary

A PASS asserts that two published pages now match the shipped binary and that a pin exists to keep
one of them matching. It asserts **nothing** about messaging, about any security property, or about
release readiness. ⚠ **This closeout is `docs_only`, so the behavioural suites correctly SKIP on its
merge — its green proves the governance edit is well-formed and nothing more.** The evidence is
#12's `[rust]` run, the §7.4 measurement, and the §4 positive control.
