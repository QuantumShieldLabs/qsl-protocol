# NA-0739 — AS BUILT: THE ENG-0142 AMENDMENT RECORDS LANE

**Lane:** NA-0739. **Decision:** D-1374. **Base:** main `64c60dc3ed5c6ca130567ab3048243bd214b86e7`.
**Directive:** the Director's formalization brief of 2026-08-16, banked verbatim under SR-14 before
anything consumed it — `BRIEF_NA0739_ENG0142_AMENDMENT_RECORDS_20260816T033408Z.md`, sha256
`0c93193a645921febe10014e87ba570474639c9abed67e38a12da753c9b318bc`, 118 lines, mode 444, mirrored to
`/srv/qbuild/operator/relay/`.

⚠ **THIS DOCUMENT EXISTS SO THAT NA-0738 STOP 002 AND STOP 003 STOP LIVING ONLY UNDER `/srv`**
(D-1 / R331.1). The ENG-0142 amendment landed in `docs/ops/IMPROVEMENT_LEDGER.md` rests entirely on
measurements recorded in those two stop-files; a reader of repo truth could not otherwise check it.
Their substance is carried here, cited by sha.

⚠⚠ **THIS LANE IS FILING ONLY. NOTHING IS REPAIRED.** ENG-0142's repair touches the receive region,
**SR-15 triggers for it**, and it is a separate lane the operator has not promoted.

---

## 1. PREMISES THE SEAT OWNS — EXPECTED VALUE STATED BEFORE EACH MEASUREMENT

| # | premise | measurement |
|---|---|---|
| a | main UNMOVED, **bare and unpiped**, against the **NAMED** github remote | `git ls-remote github refs/heads/main` → `64c60dc3ed5c6ca130567ab3048243bd214b86e7`, rc 0, exactly one line. ⚠ `origin` in this seat is a **local mirror**, never used for this check |
| a′ | open-PR set | **EMPTY** (`[]`, rc 0) in `qsl-protocol`, and **0** in each of `qsl-desktop` / `qsl-server` / `qsl-attachments`. ⚠ **With a positive control**, because an empty set from an instrument that cannot return rows is not evidence: the same `gh pr list` against `--state merged` returned **three** rows (#1753/#1752/#1751 with `mergedAt` values) |
| a″ | ⚠ **a carried premise CORRECTED** | The open item tracked in the program's notes as "#1745" is an **ISSUE, not a pull request**: `gh pr view 1745` fails with *"Could not resolve to a PullRequest"* while `gh issue view 1745` returns *"CI RED: remote-handshake-tests on main"*, **OPEN**. ⇒ it is **not** part of any open-PR id sweep, and the "derive counters against main AND every open PR" hazard has **no unmerged branch to be blind to** here. **Not closed by this lane** |
| b | NA-0738's three stop shas verified against its own `LATEST.md` **before reading them** | **All five inputs verified EXACT**, sha256 and line count both: STOP 001 `87c35f3c310fa52de374e256486e5109e427cd2d0098753539ad89f9499ff7d1` / 488 · STOP 002 `d2033fa577c07fadad40f8217d98b71af4ecfff11c14a727d588ddae2c21e712` / 229 · STOP 003 `d676c6b49326374e4a435cfb9737b7d624a87140e265a50d772084bced2abd91` / 122 · RULING R337 `29ca1c317f2628e432950d63366c5ba8627d8b38aa9c4dedaa0d478e095a471f` / 62 · brief `8ab993b93b660517ee95b7b6d2d2eb2c0178b91434cab188ecedd8d8ba8a43f3` / 149 |
| c | id derivation on **declaring forms**, with **both** controls | §2 below — the negative control **failed its expected value**, and that failure is why WF-0087 exists |
| d | `docs_only` **measured by executing the tree's own classifier**, never assumed from the file list | §6 below — `classify_ci_scope.sh` returns `scope_class=docs_only` for this lane's exact edit set and **discriminates three ways** |

---

## 2. THE ID INSTRUMENT, AND THE CONTROL THAT FAILED

The brief required a **positive** control (an id known taken must return a declaring hit) and a
**negative** control (`SR-24` measures 6 files and 0 declaring headings — free the whole time).

**Sweep on DECLARING forms**, not token counts: `### NA-####` · `### ENG-####` · `### WF-####` ·
`## D-####` / `### D-####` / `- **ID:** D-####` / `**D-####` (all four, per the NA-0732 finding) ·
`- **SR-##`.

### 2.1 Positive controls — the instrument CAN return positive, in every family swept

| id | declaring | files | verdict |
|---|---|---|---|
| `NA-0738` | 1 | 6 | taken |
| `D-1373` | 1 | 6 | taken |
| `ENG-0195` | 1 | 5 | taken |
| `WF-0086` | 1 | 7 | taken |
| `WF-0085` | 1 | 4 | taken |
| `SR-21` | 1 | 10 | taken |

Corroborated independently by the maximum of each declaring space: `### NA-` **0738** · `### ENG-`
**0195** · `### WF-` **0086** · `D-` **1373** across all four forms · `- **SR-` **22**; and by
`NEXT_ACTIONS.md`'s own `STATE:` line, `HIGHEST_NA=0738 | HIGHEST_D=1373`.

### 2.2 ⚠⚠ The negative control FAILED its expected value — and that is the control working

Expected `SR-24` = **6 files / 0 declaring**. **Measured 5 files / 1 declaring.** Per SR-21 the
disagreement was **enumerated and classified rather than the needle refined**, and it decomposed into
**two independent defects, both the seat's**:

1. **The lone "declaring" hit was a false positive.** `DECISIONS.md:38725` reads
   `**SR-24 itself is NOT yet adopted.**` — a **bold sentence opener**, matched by an over-broad
   `^\*\*SR-` alternative in the seat's own needle. A mention, not a declaration.
2. ⚠⚠ **The missing sixth file was invisible to the search, not absent from the tree.** It is
   `docs/governance/evidence/NA-0724_as_built.md` — **tracked and present on disk**. Cause: the
   recursive search honours `.gitignore`, and `.gitignore:65`'s broad `**/evidence/` rule matches
   **514 tracked files**:

| directory | tracked files matched by a `.gitignore` pattern |
|---|---|
| `docs/governance/evidence` | **483** |
| `tests` | 18 |
| `scripts/ci` | 3 |
| `inputs/metadata_runtime` | 3 |
| `qsl/qsl-client/qsc/tests` | 2 |
| `scripts/audit` · `inputs/local_ops/response_writer_fixtures` · `inputs/local_ops/director_state_index_fixtures` · `inputs/local_ops/directive_manifest_fixtures` · `docs/archive/testplans` | 1 each |
| **total** | **514** (the ten directories sum exactly; the list is complete, not truncated) |

**Proof of the blindness, by construction:** a root-anchored search for `--include='*_as_built.md'`
returns **0** files where `git grep` over the same glob returns **18**. *Same bytes, same tree, one
instrument silently blind, and no error anywhere* — the only tell is a file **count** that disagrees
with another route's count.

**Corrected on both counts, the sweep reproduces the brief's figure exactly:** `SR-24` = **6 files /
0 declaring, FREE.**

### 2.3 The result

| candidate | declaring | files | verdict |
|---|---|---|---|
| `NA-0739` | 0 | 0 | **FREE — taken by this lane** |
| `D-1374` | 0 | 0 | **FREE — taken by this lane** |
| `WF-0087` | 0 | **2** | **FREE — taken by this lane.** Both occurrences were MENTIONS inside NA-0738's own "ids swept free" sentences |
| `SR-25` | 0 | **2** | **FREE.** Same two sentences. **Not minted** |
| `ENG-0196` | 0 | 0 | **FREE. Not minted** — the primary act AMENDS ENG-0142 rather than filing a sibling (WF-0029) |

⇒ ⛳ **Had the brief not required a negative control, this lane would have derived correct ids from a
defective instrument — and filed a hazard entry while standing in it.**

---

## 3. THE PRIMARY ACT — THE AMENDMENT LANDED, WITH NO DRIFT PROVEN

### 3.1 Method: the ruled draft was landed FROM ITS OWN BYTES, not retyped

The amendment was drafted verbatim at **NA-0738 STOP 002 §5** (`d2033fa5…e712`, lines 175–195) and
RULED. It was **extracted programmatically from that file**, the stop-file's blockquote device
stripped, and the **two** `<lane>` and **two** `<D-id>` placeholders resolved to `NA-0739` / `D-1374`,
leaving **0** residual placeholders.

| artifact | sha256 |
|---|---|
| draft as extracted (21 lines, still quoted) | `0e786a85b2c78ad3c28ac6ed676434d77260480b3339d048bac478b601e380d1` |
| draft unquoted, placeholders **unresolved** | `898a93b2e2b8ae4a1fb9d7eab69dc387caf2884a234bd8ffdba97d5de68505b8` |
| draft unquoted, placeholders **resolved** | `8dd5d54019717ca6fec5722bbd30f429b7d31815280d11c918f4ddf142685c3d` |
| **the text as it now stands in the ledger** (`:3734`–`:3754`) | **`8dd5d54019717ca6fec5722bbd30f429b7d31815280d11c918f4ddf142685c3d`** |

**NO-DRIFT PROOF:** landed vs resolved → **identical, rc 0, zero differences, equal sha256**.
⚠ **And it is not a vacuous pass:** the same comparison against the **unresolved** draft **does**
report differences, so the diff instrument can return non-zero.
**PURE-INSERTION PROOF:** `git diff --numstat` on that edit = **24 insertions / 0 deletions**, a
single hunk `@@ -3732,0 +3733,24 @@` ⇒ **nothing above it was edited** (mark-don't-rewrite).

⇒ **the only differences between what was ruled and what landed are the two placeholder
substitutions and the removal of the quoting device.** This matters because the substitution defect
this program recorded at ENG-0191 — a "retarget" that silently became a "delete" — is invisible to
every gate the program owns.

### 3.2 ⛳ The coordinates were RE-VERIFIED at the landing base rather than trusted

The draft was measured at `62752adf`; it lands at `64c60dc3`. **A lane filing a stale-line-number
finding must not itself land stale line numbers.**

- `qsl/qsl-client/qsc/src/transport/mod.rs` is **byte-identical across the two bases** — blob
  `0c5e1bcc18fd523dd92824237290b9140547f4f8` at both. PR #1753 changed only six records files.
- Each cited site re-read at `64c60dc3`:

| site | content at `64c60dc3` |
|---|---|
| `:1186` | `Err(code) => {` — the `Err(code)` arm |
| `:1200` | `if code == "qsp_replay_reject" {` — **the escape branch opens** |
| **`:1210`** | ⚠ `// Ack it (loudly) to end the redelivery loop instead of hard-` — **a COMMENT, inside that branch** |
| `:1224`–`:1247` | `if ctx.ack_mode == AckMode::Lease {` … `continue;` at `:1246` — the quarantine-and-continue block |
| `:1249` | `return Err(CliError::code(code));` — **the abort, AFTER the escape branch closes** |

⇒ **the entry's own `:1210` does land a reader inside the one branch that does NOT abort — confirmed
by reading the ENCLOSURE, not merely the line.** That re-verification is recorded as its own note
attributed to NA-0739, **beside** the amendment; the amendment's own text was not edited to carry it.

---

## 4. NA-0738 STOP 002's SUBSTANCE — the basis the amendment rests on

Carried from `STOP_NA0738_002_20260816T025712Z.md`, sha256 `d2033fa5…e712`, 229 lines.

### 4.1 The identity with ENG-0142, verified clause for clause

| ENG-0142 as filed | what NA-0738 measured | identical? |
|---|---|---|
| "one bad frame aborts the whole pull" | a 6436 B handshake frame at the mailbox head → `qsp_env_decode_failed` → the whole receive aborts | **yes** |
| "only `qsp_replay_reject` under Lease escapes" | verified at source: the quarantine-and-continue arm is the **only** one; every other code falls through to `return Err(...)` | **yes** |
| "the failing item is never acked or quarantined" | store query after three aborted receives: the row still present, unacked, undestroyed | **yes** |
| "redelivers" | it did — three times, on a 1 s lease | **yes** |
| "re-aborts every `qsc receive`" | **3 attempts, 3 aborts, identical code** | **yes — and now MEASURED, not derived** |
| "until the relay's 7-day retention expires it" | the relay advertises `retention.ttl_secs = 604800` = exactly 7 days | **yes — independently confirmed** |

⇒ **no difference in mechanism.** Do not file `ENG-0196`; do not widen `ENG-0134`; **amend ENG-0142**
(WF-0029).

### 4.2 The trigger is by construction, and wider than the ruling stated

`relay_inbox_pull` (`transport/mod.rs:3010`) issues `GET /v1/pull?max=N&ack=lease` and returns the
items. **It contains no ack call at all** — acking lives in `relay_inbox_ack` (`:3068`), reached only
through the `receive` path's `pending_acks` / `flush_pending_acks`. ⇒ **every caller leases and never
acks, BY CONSTRUCTION** — strictly stronger than an argument from observed runs.

The tree's **own ratified census** (the doc comment above the function, NA-0688 C4 / D622, "SITE 2 of
2 — the flag-less pull") names three callers: **`invite accept`** (`--max 1`), ⚠ **`invite finish`**
(`--max 1`, the user's **ORDINARY inbox**), **`handshake poll`** (`--max 4`). The comment's own words:
*"⚠ `invite finish` matters most: it pulls the mailbox where a peer's ordinary messages sit, under a
command the user is required to run."*

⚠ **THE LIMIT, so the claim is not read wider than the measurement.** Undecodable residue is measured
**only** for `handshake poll`. For `invite accept` and `invite finish` **only the never-acks property
is established** — whether their residue is specifically undecodable is **UNMEASURED**.

### 4.3 "Re-aborts every `qsc receive`" — measured

Loopback relay restarted on the **same store** with `PULL_LEASE_SECS=1`; three consecutive
`qsc receive` against run X1's alice mailbox, ~2 s apart, on that run's own preserved vault and state.

| attempt | rc | markers |
|---|---|---|
| 1 | **1** | `event=qsp_unpack code=qsp_env_decode_failed ok=false` · `event=error code=qsp_env_decode_failed` |
| 2 | **1** | identical |
| 3 | **1** | identical |

Out dir **empty** throughout; **all five rows still present** afterwards — nothing acked, nothing
destroyed.

### 4.4 The sealed sub-check that MISSED, and why the miss was worth more

Sealed: delete only the head poison row from the throwaway store, re-run, and the blocked message
arrives. **Measured: rc 1 again**, `code=REJECT_S2_HDR_AUTH_FAIL` on the next frame — the blocked
message was **already unreadable**, because the run's own `hs2` had **replaced** alice's session while
the mailbox was wedged.

| same frame class | when | result |
|---|---|---|
| bob → alice user message | receive ran **before** `hs2` | `qsp_unpack ok=true`, `recv_item size=15`, `recv_commit count=1` |
| bob → alice user message | re-attempted **after** `hs2` replaced the session | `REJECT_S2_HDR_AUTH_FAIL` |

⇒ ⚠⚠ **a wedge that outlives a re-handshake DESTROYS the blocked traffic** — ENG-0142's *"delayed
until retention expires"* becomes *destroyed*. ⚠ **Stated as an inference from a measured A/B, NOT
source-proven**; ⚠ the clean demonstration — wedged before the receive, unwedged before any
re-handshake — was **NOT run**, and is marked unmeasured rather than filled in.
⚠ **Disclosed mutation:** one row (seq 2) deleted from NA-0738's **own throwaway loopback store**,
31 → 30, snapshotted both sides. **Deleting a relay row is an operator-side act no user can perform;
it demonstrates the blocking relation and is never presented as a remedy.**

---

## 5. NA-0738 STOP 003's SUBSTANCE

Carried from `STOP_NA0738_003_20260816T030116Z.md`, sha256 `d676c6b4…bd91`, 122 lines.

- ⚠⚠ **A false sentence of that seat's own, corrected in the open.** STOP 002's header said PR #1753
  was *"still OPEN, unmerged"*; it had merged at 02:50:20Z and the header was written at 02:57:12Z —
  **false by 6 min 52 s** — and its *"base `62752adf` verified UNMOVED"* was stale for the same
  reason. **Cause: a correctly-measured figure carried across seven minutes instead of re-measured at
  the moment of assertion (R324.2).** Caught only by a final verification pass that could have been
  skipped. Nothing downstream depended on it. **Recorded because the record should be right even when
  the error was harmless.**
- **The merge:** `64c60dc3ed5c6ca130567ab3048243bd214b86e7`, 2026-08-16T02:50:20Z, 6 files, **614
  insertions / 2 deletions**. ⚠ **Verified on `merged_at`, never on `merge_commit_sha`** — the latter
  is populated for closed-**unmerged** PRs too. Positive control that the records actually landed:
  `ENG-0195` measured **0** files at the old base and **5** at `64c60dc3`.
- **The sweep hazard**, recorded there and **filed here as WF-0087** (§7).

---

## 6. `docs_only` MEASURED BY EXECUTING THE CLASSIFIER, WITH DISCRIMINATING CONTROLS

`scripts/ci/classify_ci_scope.sh`, run against path sets:

| input | `docs_only` | `workflow_security` | `runtime_critical` | `scope_class` |
|---|---|---|---|---|
| **this lane's exact six-path edit set** | **true** | false | false | **`docs_only`** |
| control A — the same set **+ one product source file** | false | false | **true** | `runtime_critical` |
| control B — the same set **+ one `scripts/ci/` path** | false | **true** | false | `workflow_security` |
| control C — the gitignored evidence doc **alone** | true | false | false | `docs_only` |

⇒ **the instrument discriminates three ways, so `docs_only=true` is a measurement and not an
assumption**, and control C confirms `is_docs_path`'s `docs/*` case does cover
`docs/governance/evidence/…`. **SR-15's "not triggered" premise is corroborated by the tree's own
instrument.**

---

## 7. WF-0087 — THE SWEEP HAZARD, FILED

*Recording a freeness sweep plants the swept ids in the tree, so the next lane's sweep measures the
previous lane's record of its own sweep.* Filed as **pre-existing and general — NOT NA-0738's
creation**; NA-0738 added two instances and was the first lane to measure it.

Instances: `WF-0087` and `SR-25` free at 2 files / 0 declaring; ⛳ `SR-24` **free the whole time while
measuring 6 files** (proposed by directive 654, **REFUSED at R305** per WF-0078, so it accumulated six
files of discussion and never a declaration); `WF-0085` genuinely taken at 4 / 1. **The positive and
negative cases separate cleanly under declaring forms and not at all under a token count.**

**The cure, in two parts** — the second measured by NA-0739, not carried from NA-0738:
1. **Sweep DECLARING forms, not token counts**, building the form from the tree's own bytes (four
   `D-` record forms exist; a form-specific needle is right only by luck).
2. ⚠⚠ **Name the search tool: a gitignore-honouring recursive search cannot see 514 tracked files
   here** (§2.2). Without this, part 1 is insufficient — sweeping declaring forms with a blind tool is
   still blind.

⚠ **What the tree already recorded, and what is new.** The **write** half is recorded in many places
(`git add -f` past `**/evidence/`). The **read** half was recorded nowhere: measured across all **100**
`gitignore` mentions in **24** tracked files, **34** carry read-side vocabulary within ±350 characters
and **none** states the property; two independent routes return **exactly three** matches tree-wide and
all three are WF-0087's own new sentences — a positive control by construction.

⚠ **Executable consumer: plausible, and deliberately NOT built** (D-2 / R305). The id-derivation step
is already scripted, so a gate could re-derive a PR's claimed ids against declaring forms via
`git grep` and fail closed on a disagreement. **Two honest obstacles**, so the plausibility is not
overstated: the declaring forms are conventions rather than schema and therefore drift (the four `D-`
forms; the undocumented inline `Class` forms of NA-0725), and the gate must read the PR's prose to
know which ids were claimed. **Nothing was built.**

⚠ **The entry is itself an instance, and that is the point:** it raises the mention count of `SR-25`,
`SR-24` and `WF-0085` while **declaring only `WF-0087`**. Under the cure that is harmless and
legible; under a token count it would poison three id spaces at once.

---

## 8. CLAIM BOUNDARY — CARRIED VERBATIM FROM NA-0738, NOT SMOOTHED

The amendment's own boundary sentence, as landed: *"loopback plain HTTP, `qsl-server` rev
`37ec8207`, **not CI**, not through TLS; the wedge demonstration is n=3 aborts on n=1 wedged mailbox;
scenario `happy-path` seed 1, **`drop-reorder` not run**."*

NA-0738 STOP 001 §11's boundary, which governs the experiment the amendment rests on:

- **Loopback, NOT CI.** Plain **HTTP**, no TLS, `RELAY_CA_PEM` not involved. `qsl-server` rev
  `37ec8207`, locally generated bearer, **ZERO secrets read**, `MAX_BODY_BYTES` matched to AWS at
  65536. **This says nothing about the CI transport path.**
- **n = 2** for the relocated arrangement (byte-identical), **n = 1** for the control, **n = 1** for
  the failed first run. Scenario **`happy-path`, seed 1**. ⚠ **`drop-reorder` was NOT run.**
- **The instrument is a HARNESS COPY, never the committed script**, which was re-verified
  byte-identical after all four runs.
- **The relocated arrangement reached X4 only with the relay's visibility timeout raised.** A repair
  lane implementing (d) at the default timeout must clear that blocker first.
- ⚠ **The AWS relay's rev and its `PULL_LEASE_SECS` were NOT measured by that seat** — both need the
  bearer. The rev is carried from NA-0737 with attribution; **the AWS lease value is UNKNOWN.**

⚠⚠ **A GREEN PR IS NOT A GREEN SUITE.** `remote-handshake-tests` is a **scheduled** workflow, not a
PR check. **Issue #1745 stays OPEN** and nothing in this lane turns it green.

---

## 9. THE NEXT MEASUREMENT — NAMED, NOT PERFORMED

⚠⚠ **The highest-value unmeasured question this finding opens is whether `invite finish`'s residue is
specifically UNDECODABLE.** Undecodability is measured only for `handshake poll`. **If `invite
finish`'s residue is undecodable, the invite flow wedges the user's ordinary mailbox on first use** —
under a command the user is required to run — a materially worse statement than anything measured so
far. **Named as the successor measurement. NOT run in this lane, and the record does not guess its
answer.**

---

## 10. BOUNDS OBSERVED

Edit set, and nothing else: `docs/ops/IMPROVEMENT_LEDGER.md` · `DECISIONS.md` · `NEXT_ACTIONS.md` ·
`TRACEABILITY.md` · `docs/ops/PREDICTION_LEDGER.md` · `docs/governance/evidence/NA-0739_as_built.md`
(gitignored — **force-added**, presence confirmed in `git diff --cached --name-only`).

**Zero product source bytes.** No script, no test, no workflow, no `.github/**`, no dependency and no
lock change. **No test weakened, skipped or deleted. No standing rule minted.** No fenced ruling
edited; `## D-1373` not rewritten; **ENG-0142's pre-existing text not rewritten** — the amendment and
the coordinate note sit **beside** it, as NA-0736's 2026-08-15 note already does.

**ENG-0142 NOT repaired** (its repair triggers SR-15 and that lane is not promoted). **ENG-0191 NOT
ruled** — options (a)–(e) are all live and the choice is the operator's. **ENG-0194's inert assertion
NOT repaired. ENG-0193 NOT built. WF-0086's gate NOT built. `invite finish` NOT measured. Issue #1745
NOT closed.** No secret read. No sudo. No `qwork`/`qstart`/`qresume`/`qnext`.
**Nothing merged: the operator merges, the seat does not.**
