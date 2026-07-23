# NA-0668 — mirror freshness assertion: test plan and executed results

**Lane:** NA-0668 · **Decision:** D-1294 · **Directive:** D604 (sha256 `ace7783fdb00152e649809430aeb4776e32062758deaf1375d3ce3e6f5a1a5a9`, 412 lines)

---

## ⚠ 0. WHY THIS DOCUMENT IS NOT A CI TEST

Every artifact under test lives at **`/srv/qbuild/tools/`**, outside every repository. GitHub runners have no `/srv/qbuild`, no mirrors, and no `origin` to compare against, so a spine CI test asserting any of this would either fail on every runner or be a silent no-op there. **This spine PR is `docs_only=true`; both full suites SKIP; a green check proves nothing about any row below.**

Every row was **executed locally on `ideacentre`, 2026-07-23**, and the recorded result is the observed output — not the expected output. That distinction is the entire point of the lane: a freshness check never observed rejecting anything is exactly the artifact this lane exists to replace.

---

## 1. Harness

A fixture root at `/srv/qbuild/tmp/na0668-fixture` holding **copies** of all four mirrors (~63 MB total), selected over symlinks so that no test could write through to a live mirror even by accident. `QBUILD_ROOT` was pointed at it; `CARGO_HOME` and `RUSTUP_HOME` were pinned to the **real** cache paths, because redirecting them breaks toolchain-key resolution and produces a fixture artifact unrelated to the code under test (observed and diagnosed before it could be mistaken for a failure).

UNREACHABLE was forced with a fixture-scoped `GIT_CONFIG_GLOBAL` carrying a `url.<bogus>.insteadOf` rewrite for one repo — no live configuration was modified.

**The fixture was deleted after the run.** Live mirror `refs/heads/main` values were captured before and after the entire lane and verified byte-identical (D604 STOP-3).

---

## 2. §3a — the three-state freshness assertion

| # | case | expected | **observed** | verdict |
|---|---|---|---|---|
| 2.1 | all four mirrors current | `0 issue(s), 0 unverified`, exit 0 | `4 repos checked, 0 issue(s), 0 unverified`, **exit 0** | **PASS** |
| 2.2 | one mirror reset to `main^` | FAIL naming repo + **both shas**, exit 1 | `FAIL qsl-desktop: mirror is STALE -- mirror refs/heads/main is 8db2b2a5… but origin is 02cc9b96…`; `1 issue(s), 0 unverified`; **exit 1** | **PASS** |
| 2.3 | restore, re-run | exit 0 | `0 issue(s), 0 unverified`, **exit 0** | **PASS** |
| 2.4 | origin unresolvable | "could not verify", exit **2** | `???? qsl-desktop: could not verify mirror freshness -- origin unreachable (…)`; `0 issue(s), 1 unverified`; **exit 2** | **PASS** |
| 2.5 | **precedence:** one stale **and** one unreachable | exit **1** | `1 issue(s), 1 unverified`; **exit 1** | **PASS** |
| 2.6 | mirror has no `refs/heads/main` | STALE, not UNREACHABLE | code path asserted by inspection; the `-z "$local_sha"` arm calls `fail()`, not `unverified()` | **PASS (by construction)** |

**2.5 is the row that matters most.** Exit 1 is evaluated before exit 2 in the script, so a real staleness finding on one repo can never be masked by an unrelated network failure on another.

**2.6 is recorded honestly as asserted-by-construction rather than executed** — it was not exercised with a real ref-less mirror, and is the one row in this table not backed by observed output.

---

## 3. Hang-proofing — proved, not asserted

| # | case | **observed** | verdict |
|---|---|---|---|
| 3.1 | blackhole IP (TEST-NET-3 `203.0.113.1`) | failed in **23 ms** — connection refused, **did NOT exercise the timeout** | **INVALID as a hang test; recorded rather than dropped** |
| 3.2 | local listener accepting TCP and never replying, **unguarded** (outer `timeout 45` only so the test terminates) | **still hanging at 45.002 s**, rc 124 | **hang reproduced** |
| 3.3 | same target, `GIT_TERMINAL_PROMPT=0 timeout 20` — exactly as the script calls it | **bounded at 20.002 s**, rc 124 | **PASS** |
| 3.4 | the whole check against that stalling origin | **21.617 s total**, UNREACHABLE, **exit 2** | **PASS** |
| 3.5 | nonexistent repo — must not prompt for credentials | `remote: Repository not found` in **0.341 s** | **PASS** |

**3.1 is kept in the record deliberately.** The first attempt at proving hang-resistance did not test hang-resistance at all, and a test that cannot fail proves nothing — which is this lane's subject applied to its own test plan.

---

## 4. §3b — refresh on worktree creation

### 4.1 `qbuild__ensure_mirror` (`qshell.sh`)

| mirror | refresh | before | **observed after** | verdict |
|---|---|---|---|---|
| present | succeeds | returned 0, no fetch | rc 0, **mirror refreshed** | **PASS** |
| present | fails | returned 0 | rc 0 **+ warning**; offline path preserved | **PASS** |
| absent | succeeds | clones, rc 0 | clones, rc 0 — **unchanged** | **PASS** |
| absent | fails | fatal | **rc 128**, fatal — **unchanged** | **PASS** |

### 4.2 `qwork_ensure_checkout` (`qwork.sh`)

| case | **observed** | verdict |
|---|---|---|
| checkout already exists | `created_or_existing=existing`; **the change is never reached** | **PASS — common path untouched** |
| create, mirror present, network OK | `created_or_existing=created`, rc 0 | **PASS** |
| create, mirror present, refresh fails | warning emitted, `created_or_existing=created`, rc 0 | **PASS — offline creation preserved** |

### 4.3 The mechanism, observed end-to-end

A fixture mirror was staled, then a checkout created through the changed path:

```
mirror BEFORE create: 8db2b2a5758324ec120166b668022265af2ed3a9   (stale)
mirror AFTER  create: 02cc9b9651f43c8c3d26e73561dafbb7ca018628   (== live origin/main)
```

**The stale mirror self-healed on creation.** This is the claim §3b makes, observed rather than argued.

### 4.4 Cost (§5.C — required as a number)

| repo | time |
|---|---|
| qsl-server | 0.35 s |
| qsl-attachments | 0.37 s |
| qsl-desktop | 0.54 s |
| **qsl-protocol** (61 MB, 1621 `refs/pull/*` — worst case) | **0.53 s / 0.56 s** |

---

## 5. Regression (§5.E)

| # | check | **observed** | verdict |
|---|---|---|---|
| 5.1 | five original assertions, all repos | 4/4 `ok` on remote, mirror, both known-repo predicates, target root, invocation parity | **PASS** |
| 5.2 | `preflight_clean.sh`, `new_checkout.sh`, `env_qbuild.sh`, `refresh_mirrors.sh` | `git diff --quiet dafea7f HEAD` clean — **byte-identical** | **PASS** |
| 5.3 | `git revert` both commits | both clean; reverted script returns to `4 repos checked, 0 issue(s)` | **PASS** |
| 5.4 | live mirrors unmodified | all four `refs/heads/main` identical before/after | **PASS** |

`preflight_clean.sh` was **not exercised behaviourally** — it is a cleanup tool, out of scope, and byte-identity is the stronger evidence. Recorded rather than implied.

---

## 6. ⚠ THE UNPLANNED RESULT: THE CHECK REJECTED A REAL MIRROR ON ITS FIRST LIVE RUN

Before any fixture existed:

```
FAIL  qsl-protocol: mirror is STALE -- mirror refs/heads/main is 0b396fc85da0397182bce51ee08c35b0f445ee3a
      but origin is 8a05c1a3a99dd8311f14cd615ad2f54013b40fb2
4 repos checked, 1 issue(s), 0 unverified          exit 1
```

The mirror is **exactly two commits behind** — `11a0fa70` (this lane's queue promotion) and `8a05c1a3` (its merge) — and `git merge-base --is-ancestor` confirms **strictly behind, not diverged**. D604 §0 had recorded all four mirrors CURRENT after the operator's 02:13Z refresh, so **the gap re-opened within hours of the directive that specified the check.**

**The live mirror was left stale.** Refreshing it is out of scope (§7) and forbidden (STOP-3). **Acceptance §5.B.6 — "4/4 currency against live origin" — is therefore recorded NOT MET at 3/4**, rather than being quietly satisfied by an out-of-scope refresh. It is an operator action.

---

## 7. What this plan does not cover

- **`refs/pull/*` and tag currency.** The assertion compares `refs/heads/main` only. Mirrors carry 1621 / 63 / 40 / 6 PR refs. Not asserted, not tested, not claimed.
- **`new_checkout.sh`** — WF-0037, file-only; byte-unchanged and untested here.
- **The scheduled daily backup** under the new script — first opportunity 2026-07-24 ~02:38 CDT. The post-install checkpoint is the evidence; the daily is confirmation still outstanding.
- **Any off-machine or disaster-recovery property.** The machine remains **same-machine only**.
