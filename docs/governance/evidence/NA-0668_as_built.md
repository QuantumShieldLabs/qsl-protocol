# NA-0668 — as built

**Lane:** NA-0668 · **Decision:** D-1294 · **Directive:** QSL-DIR-2026-07-23-604 (D604), sha256 `ace7783fdb00152e649809430aeb4776e32062758deaf1375d3ce3e6f5a1a5a9`, 412 lines
**Base:** spine `8a05c1a3a99dd8311f14cd615ad2f54013b40fb2` (PR #1623, the NA-0668 queue promotion)
**Result class:** `MIRROR_FRESHNESS_AND_BACKUP_COVERAGE_PASS`

---

## ⚠ 1. THE CI GREEN ON THIS PR IS NOT VALIDATION OF THIS LANE

Read this before anything else.

The spine diff is governance markdown only. `scripts/ci/classify_ci_scope.sh` returns `docs_only=true`, so **both full suites SKIP.** A green check on this PR says nothing about suite health and nothing about whether the tooling change works.

The tooling this lane changes lives at **`/srv/qbuild/tools/`, outside every repository.** No CI system can see it. There is no workflow, no runner, and no check anywhere that exercises it. This is the NA-0667 property, recurring.

**The validating evidence is §4 — the freshness check OBSERVED FAILING, and observed distinguishing "stale" from "could not verify."** A freshness check that has never been seen to reject anything is exactly the check this lane exists to replace, so a lane that shipped one on the strength of a green PR would have reproduced its own subject.

---

## 2. What landed, and where

**Nothing in this PR is the fix.** The fix is two commits in `/srv/qbuild/tools/` plus one privileged operator action on `/usr/local/sbin/qsl-backup`, none of which a PR can carry.

| commit | what |
|---|---|
| `b8ec4e923f190f689ec146558d8b192819b73be8` | **§3a** — three-state mirror freshness assertion in `check_repo_registration.sh` |
| `e16b3f942be52eff864f3982b5490f62886e72a2` | **§3b** — refresh the mirror on worktree creation in `qshell.sh` + `qwork.sh` |

Both sit on top of `dafea7f` (NA-0667). Change surface, whole lane:

```
 check_repo_registration.sh | 78 ++++++++++++++++++++++++++++++++++++++++++++--
 qshell.sh                  | 33 +++++++++++++++++---
 qwork.sh                   | 16 ++++++++++
 3 files changed, 121 insertions(+), 6 deletions(-)
```

**Verified byte-unchanged:** `preflight_clean.sh`, `new_checkout.sh`, `env_qbuild.sh`, `refresh_mirrors.sh` (`git diff --quiet dafea7f HEAD -- …` returns clean). `preflight_clean.sh:54` is existence-only **by design** — it reports missing mirrors — and was deliberately not touched. `new_checkout.sh` is WF-0037, filed only (§6).

Both commits carry the GH007 noreply identity, **verified on the object** rather than merely requested, per the open WF-0029:

```
author=Tebbens4832 <238594419+Tebbens4832@users.noreply.github.com>
committer=Tebbens4832 <238594419+Tebbens4832@users.noreply.github.com>
```

No trailers on either (`git log -1 --format='%(trailers)'` → empty).

---

## 3. B0 — `/srv/qbuild/operator/` is now in the backup

**B0 was executed by the operator before this lane's implementation phase.** The executor prepared the package and stopped before `sudo`, per §6 STOP-1; the operator ran Step 5.

**Installed script sha256 is now `4230eccd6d9aa68179fb8595acee570498b79104239b1883fae6c1cd76f071c1`** (was `8d2dd5d1d48ef0e3103e42b55740cd8329e880c5dd071b977639a286532f1262`).

Both approved source lines are live in `daily_sources`, and nothing else changed (STOP-6 as amended):

```
39:  /srv/qbuild/operator
44:  /home/victor/work/qsl/codex
```

The redundant `codex/logs`, `codex/responses`, `codex/ops` entries remain in place deliberately — removing them would exceed the bounded amendment.

### Acceptance A — §2e output, verified not assumed

Checkpoint `checkpoint-20260723T083238-0500-after-operator-source-added`:

| check | result |
|---|---|
| `test -d $CP/srv/qbuild/operator` | **PRESENT** |
| `find $CP/srv/qbuild/operator -type f \| wc -l` | **807** |
| rsync files created across **all** newly-added sources | **839** (the operator's Step 7 figure — `operator/` **plus** the `codex/` delta; **807 and 839 measure different things and are not in tension**, recorded explicitly because conflating two counts is this lane's subject) |
| `relay/AUDIT_20260722_independent_full_audit.md` | **RECOVERABLE** |
| `diff -r --brief` live vs snapshot | **7 entries, all files written after the snapshot began** (this directive, the NA-0668 package, `relay/LATEST.md`, `RELAY_20260723T134521Z.md`) — no unexpected difference |
| codex top-level files, live vs snapshot | **10 / 10** |
| deletions on the second run | **0** (idempotent under the overlapping parent+child sources) |

**The negative half pre-dates the fix and is already recorded:** three consecutive dailies (`daily-20260720`, `-20260721`, `-20260722`) verified **absent** of `/srv/qbuild/operator`.

### The circularity is closed

`QSL_BACKUP_DIRECTOR_PACKET_20260517.md` — **the document describing the backup procedure** — was verified absent from the latest snapshot *while B0 was being prepared from that very packet.* It is now covered, byte-proof:

```
88e7d164b79da683825faa3b11d82e55f23ffec2dace0db05ff57a1953e3ec8d  <checkpoint copy>
88e7d164b79da683825faa3b11d82e55f23ffec2dace0db05ff57a1953e3ec8d  /home/victor/work/qsl/codex/QSL_BACKUP_DIRECTOR_PACKET_20260517.md
```

The packet was corrected in place before B0 ran: **504 → 539 lines, +35 purely additive, nothing deleted**, sha256 `555fd000…6c25b` → `88e7d164…3ec8d`. It gained a mandatory Step 0 re-sync (with the 21-line staleness measurement and the three `daily_sources` entries that following the old procedure would have dropped) and a warning that none of the three workspace files, nor the packet itself, was a backup source. **The correction had to be additive-only precisely because there was no snapshot: the pre-edit text existed nowhere but in its recorded sha256.**

### ENG-0062's loss risk is closed — and the deadline is DEAD

All four preserved harness files are in the snapshot and byte-identical to the live copies:

| file | sha256 (first 16) | snapshot |
|---|---|---|
| `measure.py` | `7f84199e1ba0f1f7` | MATCH |
| `rig.sh` | `18f59a8435ada4ad` | MATCH |
| `analyze.py` | `96b4771918d8b616` | MATCH |
| `analyze2.py` | `8d6dcc0088725de9` | MATCH |

**⚠ Stated so the next sequencing decision is not distorted: the 2026-07-29 date is no longer a loss deadline.** The originals remain at `/srv/qbuild/tmp/NA0665_gui_round4a_20260722T051031Z/`, still backup-excluded and still deletion-eligible on/after 2026-07-29 — but a byte-identical copy is now under daily backup. **The durable `qsl-desktop/tools/gui-measure/` home remains owed AS GOOD PRACTICE, NOT AS RISK MITIGATION.** Carrying the old urgency forward would overstate a deadline, which is the same failure mode the WF-0031 severity correction was made to prevent.

### Recovery-path asymmetry, stated explicitly

**B0 is NOT `git revert`-able.** `/usr/local/sbin/qsl-backup` is not in any repository. Its recovery path is the Step-1 checkpoint (`checkpoint-20260723T082129-0500-before-backup-change`) plus re-installing the prior script from a snapshot. The §3 tooling changes *are* git-revertible (§5 acceptance D). That asymmetry is why B0 was sequenced first and kept to two lines.

### Not yet observed

The **scheduled daily** has not yet run under the new script — `daily-20260723T023805-0500` was taken at 02:38 CDT, ~6 hours *before* the 08:32 install, and correctly does not carry `operator/`. **The first daily to carry it will be 2026-07-24 ~02:38 CDT.** The checkpoint already proves the mechanism; this is confirmation, not validation, and is recorded as outstanding rather than claimed.

---

## 4. §3a — the freshness assertion, AND THE EVIDENCE OF IT REJECTING THINGS

### What it asserts

`check_repo_registration.sh` made five assertions per repo and **the word "commit" appeared in none of them.** Assertion 2 tested that the mirror directory existed and was bare. It read as health and meant presence: a mirror four months stale satisfied it exactly as well as one refreshed a second ago.

Assertion 6 adds three states, not two:

| state | outcome | exit |
|---|---|---|
| **CURRENT** — mirror `refs/heads/main` == origin `refs/heads/main` | `ok` | 0 |
| **STALE** — the two differ, or the mirror has no `main` | **FAIL**, naming the repo and **both shas** | **1** |
| **UNREACHABLE** — origin cannot be queried | `????` **"could not verify"** | **2** |

The third state exists because *"a check that fails on a network blip trains people to ignore it — a gate that cannot pass teaches bypass."*

**Exit precedence is normative and implemented as such:** `[[ $issues -eq 0 ]] || exit 1` is evaluated **before** `[[ $unverified -eq 0 ]] || exit 2`, so one repo's network failure can never mask another's real staleness.

**Scope is stated rather than implied:** this compares `refs/heads/main` **only**. Mirrors also carry `refs/pull/*` (qsl-protocol 1621, qsl-server 63, qsl-attachments 40, qsl-desktop 6) and tags. **PR-ref and tag currency are explicitly NOT asserted and are not claimed anywhere in this document.**

### ⚠ 4.1 — THE CHECK CAUGHT A REAL STALE MIRROR ON ITS FIRST LIVE RUN

Before any fixture existed, the first live run against `/srv/qbuild/mirrors` returned:

```
  FAIL  qsl-protocol: mirror is STALE -- mirror refs/heads/main is 0b396fc85da0397182bce51ee08c35b0f445ee3a but origin is 8a05c1a3a99dd8311f14cd615ad2f54013b40fb2
        mirror: /srv/qbuild/mirrors/qsl-protocol.git
        (run refresh_mirrors.sh qsl-protocol)

4 repos checked, 1 issue(s), 0 unverified
exit 1
```

**This is not a synthetic finding.** The directive's §0 recorded all four mirrors CURRENT at `0b396fc8` after the operator's 02:13Z manual refresh. The mirror is exactly **two commits behind** — `11a0fa70` (the NA-0668 queue promotion) and `8a05c1a3` (its merge) — and `git merge-base --is-ancestor` confirms it is **strictly behind, not diverged.**

**The staleness was created by this lane's own seating, within hours of the directive that specified the check.** That is the clearest possible demonstration of the gap being real and continuous rather than historical.

**The live mirror was NOT refreshed.** §7 puts mirror refresh out of scope and §6 STOP-3 forbids touching `/srv/qbuild/mirrors/*`. It is left stale, reported here, and is an operator action.

**Consequence for acceptance §5.B.6 at implementation time — "4/4 currency against live origin" was NOT MET, and was not claimed:** live state was **3/4 current, 1 genuinely stale**, for the reason above. **It was closed at 4/4 by the operator after merge — see §10, which also records the pass half of the evidence pair.**

### 4.2 — PASS baseline (§5.B.1)

Run against a fixture root holding copies of all four mirrors, all current:

```
  ok    mirror CURRENT at 8a05c1a3a99dd8311f14cd615ad2f54013b40fb2
  ok    mirror CURRENT at b4f86a3c814ca79713d4f3d73fcac65762a50f9c
  ok    mirror CURRENT at dd5a2e6b7c291e7d5be28eca461198decc6e655c
  ok    mirror CURRENT at 02cc9b9651f43c8c3d26e73561dafbb7ca018628

4 repos checked, 0 issue(s), 0 unverified
exit 0
```

All five original assertions pass for all four repos in the same run (§5.E, no regression).

### 4.3 — ⚠ OBSERVED FAILING (§5.B.2) — THE DELIVERABLE

Fixture `qsl-desktop` staled to `main^`:

```
  FAIL  qsl-desktop: mirror is STALE -- mirror refs/heads/main is 8db2b2a5758324ec120166b668022265af2ed3a9 but origin is 02cc9b9651f43c8c3d26e73561dafbb7ca018628
        mirror: /srv/qbuild/tmp/na0668-fixture/mirrors/qsl-desktop.git
        (run refresh_mirrors.sh qsl-desktop)

4 repos checked, 1 issue(s), 0 unverified
exit 1
```

Repo named, **both shas named**, exit 1.

### 4.4 — restored, PASS re-proved (§5.B.3)

`update-ref` back to `02cc9b96` → `4 repos checked, 0 issue(s), 0 unverified`, **exit 0**. Fixture deleted afterwards.

### 4.5 — the third state, distinctly (§5.B.4)

`qsl-desktop`'s origin rewritten to an unresolvable host via a fixture-scoped `GIT_CONFIG_GLOBAL` `insteadOf`:

```
  ????  qsl-desktop: could not verify mirror freshness -- origin unreachable (https://github.com/QuantumShieldLabs/qsl-desktop.git)

4 repos checked, 0 issue(s), 1 unverified
exit 2
```

**Different text, different counter, different exit code.** Not conflated with stale.

### 4.6 — the precedence rule (§5.B.5)

`qsl-attachments` STALE **and** `qsl-desktop` UNREACHABLE in the same run:

```
  FAIL  qsl-attachments: mirror is STALE -- mirror refs/heads/main is a3ebad2fd19ae50b0f764fd44b7fc47fd5ca8723 but origin is dd5a2e6b7c291e7d5be28eca461198decc6e655c
  ????  qsl-desktop: could not verify mirror freshness -- origin unreachable (…)

4 repos checked, 1 issue(s), 1 unverified
exit 1
```

**Exit 1.** The staleness finding is not masked.

### 4.7 — hang-proofing PROVED, not asserted

The directive requires `GIT_TERMINAL_PROMPT=0` and `timeout 20`. A blackhole address was tried first and **failed fast (23 ms) rather than hanging, so it did not exercise the guard at all** — recorded because a test that cannot fail proves nothing, which is this lane's subject. A local listener that accepts TCP and never replies was used instead:

| invocation | result |
|---|---|
| `git ls-remote` unguarded, bounded by an outer `timeout 45` | **still hanging at 45.002 s** (rc 124) |
| `GIT_TERMINAL_PROMPT=0 timeout 20 git ls-remote …` — exactly as the script calls it | **bounded at 20.002 s** (rc 124) |
| the whole check against that stalling origin | **21.617 s total, UNREACHABLE, exit 2** |

Credential prompting is likewise suppressed: a nonexistent repo returns `Repository not found` in 0.341 s instead of prompting.

Measured cost of the assertion against live origin with all four repos reachable: **~1.7 s** added to the check (0.53 s → 2.27 s).

---

## 5. §3b — refresh on creation, which is the structurally better half

### The three sites

| site | code before | when reached |
|---|---|---|
| `check_repo_registration.sh:51` | `[[ -z "$mirror" \|\| ! -d "$mirror" ]]` | every run — **fixed by §3a** |
| `qshell.sh:167` | `if [[ -d "$mirror" ]]; then return 0; fi` | `qbuild__ensure_mirror`, only when the worktree does not yet exist |
| `qwork.sh:106` | `if [[ ! -d "$mirror" ]]; then …refresh… fi` | `qwork_ensure_checkout`, only when the checkout does not yet exist |

**All three fixed.** Fixing one and leaving another leaves the trap armed in the place nobody is looking, and `qwork.sh` is the more heavily used tool.

### Why this is the better half, and its honest limit

**§3a detects staleness; §3b makes normal use prevent it**, because every new lane now refreshes the mirror it clones from. A mirror can then only go stale *between* lanes rather than across months.

**⚠ HONEST LIMIT, so this is not over-valued: neither change repairs a live incident path.** `qwork.sh:372` already runs `fetch --prune origin main` and hard-asserts `local_head == origin_main`; `qshell.sh:139` does the same behind the NA0378 fast-forward guard. Both were read and confirmed in place. **These changes are defence-in-depth plus a structural staleness bound — not the repair of a demonstrated incident.**

### Proof the refresh actually fires

A fixture mirror was staled, then a checkout created through the changed `qwork_ensure_checkout` path:

```
mirror BEFORE create: 8db2b2a5758324ec120166b668022265af2ed3a9   (stale)
mirror AFTER  create: 02cc9b9651f43c8c3d26e73561dafbb7ca018628   (== live origin/main)
```

**The stale mirror self-healed on creation.** That is the mechanism §3b claims, observed rather than argued.

### Behaviour matrix, all four cases exercised

| mirror | refresh | before | after |
|---|---|---|---|
| present | succeeds | returned 0, no fetch | **rc 0, mirror refreshed** |
| present | fails (no network) | returned 0 | **rc 0 + warning — offline path preserved** |
| absent | succeeds | clones, rc 0 | unchanged: clones, rc 0 |
| absent | fails | fatal | unchanged: **fatal (rc 128)** |

`qwork_ensure_checkout` with an **existing** checkout returns `created_or_existing=existing` and never reaches the change — the common path is untouched.

**⚠ ONE JUDGMENT CALL, FLAGGED FOR THE OPERATOR — SINCE AFFIRMED (§10.2).** The directive says to refresh whenever a new worktree is created but does not say whether a *refresh failure* against an already-present mirror should be fatal. It is implemented as **non-fatal (warn and proceed)**, on §3a's own stated principle that a gate which cannot pass teaches bypass, and because the downstream origin assertions are the real gate. This makes the change strictly additive: **it cannot break anything that works today**, and it preserves the existing offline creation path exactly. An **absent** mirror keeps the old fatal behaviour, because there the refresh *is* the clone.

### Measured cost (§5.C — a number, not an assertion)

One `git remote update --prune` per newly created worktree, against an already-current mirror:

| repo | time |
|---|---|
| qsl-server | 0.35 s |
| qsl-attachments | 0.37 s |
| qsl-desktop | 0.54 s |
| **qsl-protocol** (61 MB, 1621 PR refs — worst case) | **0.53 s / 0.56 s** over two runs |

---

## 6. Acceptance summary

| # | acceptance | result |
|---|---|---|
| **A** | B0 coverage, §2e output | **MET** — 807 files, audit report recoverable, packet byte-proof, 10/10 codex files, 0 deletions |
| **B.1** | PASS baseline | **MET** — `0 issue(s), 0 unverified`, exit 0 |
| **B.2** | **observed FAILING, both shas, exit 1** | **MET** — and additionally on a **real** live mirror (§4.1) |
| **B.3** | restore, re-prove PASS | **MET** — exit 0, fixture deleted |
| **B.4** | UNREACHABLE distinct, exit 2 | **MET** |
| **B.5** | precedence: stale + unreachable → exit 1 | **MET** |
| **B.6** | 4/4 currency against live origin | **MET at closeout (§10).** Was 3/4 at implementation — `qsl-protocol` genuinely 2 commits stale, left unrefreshed per §7/STOP-3. The operator refreshed and re-ran: **`4 repos checked, 0 issue(s), 0 unverified`, exit 0.** |
| **C** | three sites as diffs, cost as a number | **MET** — 0.35–0.56 s |
| **D** | revertibility | **MET** — `git revert` clean on both commits, reverted script returns to `4 repos checked, 0 issue(s)`; reset back to `e16b3f9`. **B0's asymmetry recorded (§3)** |
| **E** | no regression | **MET** — five original assertions 4/4; `qwork` seats (existing + created paths exercised); `preflight_clean.sh` byte-identical |

### STOP conditions

| # | condition | status |
|---|---|---|
| 1 | STOP before `sudo` | **HELD** — operator ran Step 5; executor ran no privileged command |
| 3 | no live mirror modified | **HELD** — all four `refs/heads/main` byte-identical before and after the entire lane; the fixture was a copy under `/srv/qbuild/tmp`, since deleted |
| 4 | B1 not drafted/scoped/half-built | **HELD** — §8's five questions are the operator's, untouched |
| 5 | `new_checkout.sh` unchanged | **HELD** — byte-identical; filed as WF-0037 |
| 6 | backup script limited to two source lines | **HELD** — lines 39 and 44, nothing else |
| 7 | nothing deleted | **HELD** — WF-0034's 17.2 GB untouched; both `.incomplete-*` directories still present (2) |
| 8 | `PRIVATE_VALUES_DO_NOT_PASTE.md` contents not reproduced | **HELD** — not read, not quoted, not summarised beyond §2b's class-level classification |

---

## 7. Ledger filings

- **WF-0037** — `new_checkout.sh:63` resolves an explicit `ref` from the mirror before origin is fetched. **FILED ONLY**, with the four ambiguity axes and the reachability limit.
- **WF-0038** — the W-1 finding itself, carrying the **WHY-IT-SURVIVED** note and **costume 4 with all six instances** (§8 below).
- **WF-0039** — `DOC-OPS-002` is owed a v0.2.0: its §3 enumerates three repos and predates `qsl-desktop`.
- **WF-0040** — `prune_snapshots()` filters `! -name '.incomplete-*'` from both its keep list and its prune list, so failed runs accumulate permanently (two present).
- **WF-0016 amended** — its "two artifacts outside version control" census is extended: the directive corpus was also **outside the backup** until B0.
- **ENG-0062 corrected** — deadline framing, per §3.

---

## 8. ⚠ THE FINDING IS ONE CLASS IN SIX COSTUMES

**An artifact that reads as safe and isn't.** These are not six findings; they are one, and the entry says so.

1. **An existence-only check that reads as health and means presence** — `check_repo_registration.sh:51`, and the same reasoning at `qshell.sh:167` and `qwork.sh:106`.
2. **A workaround that always worked and therefore removed the pressure to fix.** The defect was seen by name **four** times — **2026-06-03**, 2026-07-11, 2026-07-12, 2026-07-13 — plus **46 journal lines** matching stale-`mirror/main` phrasing (**a floor, hand-inspected for phrasing, NOT a count of distinct lanes**). *"A recovery convention that always works removes pressure to fix"* (NA-0664), on a second surface. **The 2026-06-03 sighting is the most damning: `DIRECTOR_QWORK_STARTUP_RECOMMENDATION.md` is the document that produced `qwork`, and it names the stale mirror as a motivation — so the workaround was institutionalised in the tool built to route around it.** That is why `qwork.sh:106` still carried the existence-only check until this lane.
3. **A documented, correct-to-follow procedure that would have silently regressed the thing the lane exists to protect** — the packet's own change procedure pointed at a workspace copy 21 diff lines stale; following it literally would have dropped three `daily_sources` entries including `/home/victor/.claude`.
4. **⚠ A VERIFICATION INSTRUCTION THAT READS AS AUTHORITATIVE AND IS WRONG.** The Director's own B0 package shipped **four wrong `EXPECT` lines**, found only because each was executed against a scratch copy instead of asserted. The worst promised `7a8` / `>   /srv/qbuild/operator` where the truth is `39d38` / `<   /srv/qbuild/operator` — **wrong in both direction and line numbers, at the one step whose entire stated purpose is "anything else ⇒ STOP."** A wrong halt condition does not merely fail; **it trains the operator to distrust halt conditions.** Also wrong: `wc -l` vs `grep -c` for the staleness count, and a dry-run grep against a stream containing no file paths (the script runs rsync without `-v`).
5. **⚠ THE FIFTH INSTANCE ARRIVED AFTER THE REMEDY WAS ADOPTED.** Step 7.3 shipped `EXPECT: 582+` for the response count; the real run returned **576**, because the census counted with `ls dir/*` and the glob **expanded the `director_handoff/` subdirectory**, absorbing its entries plus `ls`'s header and blank line. Ground truth: **576 top-level entries = 575 response files + 1 subdirectory; 580 recursively.** The subtree size was likewise corrected **37 MB → 48 MB** (a `du` multi-argument deduplication artifact; true total 48,390,098 bytes across 800 files). **The wrong expectations survived because the standing method was applied only to commands whose output was a *diff*, not to those whose output was a *count*.**
6. **⚠ AND A SIXTH, INSIDE THIS LANE, BY THE EXECUTOR, WHILE WRITING UP THE OTHER FIVE.** Verifying B0's codex coverage, the executor referenced the checkpoint as `checkpoint-20260723T083238-0500` — **dropping the `-after-operator-source-added` label suffix.** The path did not exist. Every `test -f "$CP/…" && echo COVERED || echo MISSING` dutifully printed **MISSING**, and `find … 2>/dev/null | wc -l` printed **0**. This produced a confident, fully-formatted, **entirely false** report that B0's second source line had silently failed — which was nearly recorded as a material finding against the operator's completed work. **`test -f` cannot distinguish "file absent" from "parent directory absent," and `2>/dev/null` erased the one signal that would have exposed it.** Caught by cross-checking against an earlier run that used the full path and returned 807. **Re-verified with the correct path: 10/10 codex files covered, packet byte-identical.** Recorded because it is self-implicating, because it is the same class, and because a lane about checks that report success regardless of result must not quietly bury its own instance of a check that reported failure regardless of result.

### The standing method, narrowed by instances 5 and 6

> **EXECUTE EVERY OPERATOR-FACING EXPECTATION BEFORE SHIPPING IT — INCLUDING THE ARITHMETIC ONES.**
> **A shell glob is not an inventory, exactly as a grep is not a measurement** (NA-0664).
> **And a negative result is only evidence if the path it was measured against exists** — bare `test`/`find` on an unverified path, especially with stderr suppressed, reports absence identically whether the thing is missing or the question was malformed.

**The entry's first draft closed by predicting a fourth instance; it arrived within the hour, inside the package written to fix the first three. A fifth followed after the remedy was adopted, and a sixth inside the lane that wrote the remedy down.** The prediction is not offered again as a rhetorical device — it is offered as the reason the method must be mechanical rather than intentional.

---

## 9. What this lane did NOT do

- **Did not refresh any live mirror.** `qsl-protocol` remains 2 commits stale; that is an operator action (§4.1).
- **Did not run any privileged command.** B0's Step 5 was the operator's.
- **Did not touch B1.** The five archive questions in §8 of the directive are the operator's to rule in chat.
- **Did not change `new_checkout.sh`** (WF-0037), `preflight_clean.sh`, or the `.incomplete-*` prune bug (WF-0040).
- **Did not delete anything** except its own test fixture, and the ENG-0062 originals were not moved.
- **Did not observe the scheduled daily** carrying `operator/` — first opportunity 2026-07-24 ~02:38 CDT (§3).

---

## 10. CLOSEOUT ADDENDUM (2026-07-23, after PR #1624 merged as `565d480c`)

### 10.1 — ⚠ THE EVIDENCE PAIR IS NOW COMPLETE ON A REAL MIRROR

§4.1 recorded the fail half. The operator refreshed `qsl-protocol` and re-ran the check:

```
4 repos checked, 0 issue(s), 0 unverified          exit 0
```

with each repo printing `mirror CURRENT at <sha>`.

**So the lane holds a complete FAIL → refresh → PASS pair against PRODUCTION mirrors, not only against the throwaway fixture** — and the fail half was caught **unprompted, on the check's first live run**, against a staleness created by this lane's own promotion and merge. **That is precisely the acceptance this lane was defined by.** §5.B.6 is **CLOSED at 4/4**.

### 10.2 — ⚠ AND IT WENT STALE A THIRD TIME WITHIN THE HOUR. THAT IS DATA, NOT A DEFECT.

Merging PR #1624 advanced origin `8a05c1a3` → `565d480c`. The same check now reports `qsl-protocol` STALE again:

```
FAIL  qsl-protocol: mirror is STALE -- mirror refs/heads/main is 8a05c1a3… but origin is 565d480c…
4 repos checked, 1 issue(s), 0 unverified          exit 1
```

**Three staleness events in one day** — 02:13Z refresh → stale at #1623's merge → refreshed, 4/4 → stale at #1624's merge. **Every spine merge re-stales the spine mirror.**

**This needs no chasing and is deliberately not chased here.** §3b self-heals it at the next lane seat, and the downstream origin asserts mean it cannot produce a wrong checkout in any case. It is recorded because it is the **measured decay rate** behind **WF-0041**, and because it means *"is the mirror set 4/4 right now?"* is a question with a **shelf life of one merge** — which is the strongest available argument that a point-in-time 4/4 is not the same thing as an owned detector.

### 10.3 — the non-fatal refresh ruling: AFFIRMED, with the reasoning recorded

The §5 judgment call is affirmed. The reasoning belongs in the record because **fatal is the superficially safer-looking choice**:

> `qwork.sh:372` and `qshell.sh:137` already fetch origin and hard-assert `head_equals_origin_main` **after** the mirror seeds the clone, so **a stale mirror CANNOT produce a wrong checkout — the refresh is hygiene, not a correctness gate.** Fatal would block work on a network blip for **zero correctness gain**, which is *"a gate that cannot pass teaches bypass"* **arriving inside the fix for it.** Absent-mirror staying fatal preserves the real invariant.

### 10.4 — costumes 6 and 7 share one shape, and the narrowing is generalized

**In both cases the tool answered a question ADJACENT to the one intended, and the answer was shaped like an answer to the intended question.**

| costume | what was asked | **what the tool actually answered** | direction of failure |
|---|---|---|---|
| 6 | *did B0 back up these files?* | *does this nonexistent path exist?* → **no** | **false alarm** against completed work |
| 7 | *is this directory ignored?* | *is this **tracked** file ignored?* → **no, tracked files are exempt** | **false all-clear**, nearly shipped a PR with no as-built |

**Opposite directions, same defect: the instrument was pointed slightly off the question.** Neither was catchable by re-reading the output — both outputs were well-formed and internally consistent. Only the *question* was wrong.

**The standing method's final clause is therefore generalized, superseding the narrower first draft:**

> **A NEGATIVE RESULT IS ONLY EVIDENCE IF THE INSTRUMENT COULD HAVE RETURNED POSITIVE.**

The earlier wording — *"only evidence if the PATH it was measured against exists"* — covers costume 6 and **misses costume 7 entirely**, since `check-ignore`'s path existed and the instrument was still incapable of answering the question asked. **Discharge it with a positive control:** point the same instrument at a known-positive case and trust the negative only if the control returns positive.

### 10.5 — ⚠ THE DETECTOR HAS NO OWNER (answered, filed as WF-0041, deliberately not fixed)

**Question asked by the operator: does anything run `check_repo_registration.sh` automatically? ANSWER: NO — and nothing even tells anyone to run it.**

| candidate | result |
|---|---|
| systemd system timers | 14 present; the only two project timers run `qsl-backup daily` and `qbuild-ssd-maintenance`. **Neither invokes it.** |
| systemd user timers | 2, both OS/snap |
| `crontab -l` | `no crontab for victor` |
| `/etc/cron.d`, `/etc/cron.daily` | OS defaults only |
| Claude Code hooks | `settings.json` has **no `hooks` key at all** — the same gap as **WF-0036** |
| spine CI | no `.github/**` reference |
| all 15 tools scripts | **zero non-comment references.** The only two mentions in `/srv/qbuild/tools/` are **comments this lane wrote** at `qwork.sh:111` and `qshell.sh:177` |
| `CLAUDE.md` / `START_HERE.md` / `AGENTS.md` / `DIRECTOR_OPERATIONS.md` / DOC-OPS-003 | **zero mentions** — every spine reference is *narrative*, never a procedural step |

**⚠ The negative was validated with positive controls, per §10.4.** The identical greps returned **8** references for `refresh_mirrors.sh` and **3** for `new_checkout.sh`; the identical doc sweep returned **3** `qwork` mentions in `CLAUDE.md` and **2** in `START_HERE.md`. **The instrument was demonstrably capable of returning positive and did not.**

**So the detector inherits the exact shape of the artifact it replaced.** WF-0038's defect was a check that read as health and meant presence; its replacement reads as health **whenever nobody looks — and nobody looks until something is already wrong.** §3a made the assertion correct; **it did not give it an owner.**

**Honest bounding, so this is not inflated:** §3b substantially defuses the operational consequence — a stale mirror self-heals at the next lane seat and cannot produce a wrong checkout regardless. **The residual risk is narrower: a repo in which no lane is ever seated receives no §3b refresh at all** (`qsl-attachments` is the live candidate), so only §3a would ever notice its drift, and only if somebody runs it. **That is the gap WF-0041 names, with four unchosen options and a recommendation to decide it alongside WF-0036.**
