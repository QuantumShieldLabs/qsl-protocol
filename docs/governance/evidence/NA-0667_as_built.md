# NA-0667 — as built

**Lane:** NA-0667 · **Decision:** D-1293 · **Directive:** QSL-DIR-2026-07-22-603 (D603), sha256 `40388993b91e853e5671727c2f19cc6ce9e2477c4a2e45ff8005030f3406f079`, 309 lines
**Base:** spine `7fdcde6cbe71cf57687a31e1edb4a4b2eae5ac0a` (PR #1621, the NA-0667 queue promotion)
**Result class:** `QBUILD_TOOLS_CUSTODY_AND_DESKTOP_REGISTRATION_PASS` — **partial on acceptance D, see §6**

---

## ⚠ 1. THE CI GREEN ON THIS PR IS NOT VALIDATION OF THIS LANE

This must be read before anything else in this document.

The spine diff is governance markdown only. `scripts/ci/classify_ci_scope.sh` returns `docs_only=true`, so **both full suites SKIP**. **A green check on this PR says nothing about suite health and nothing about whether the tooling change works.**

The tooling this lane changes lives at **`/srv/qbuild/tools/`, outside every repository.** No CI system can see it. There is no workflow, no runner, and no check anywhere that exercises it.

**The validating evidence is the before/after `qwork` pair in §3** — and the "after" half is **reserved for the operator**, because the executor does not run `qwork`. Until the operator runs it, this lane's central claim is *supported by construction and unverified by execution*, and should be described that way.

This is the same masking mechanism recorded against NA-0664 (*"closeout is docs_only so BOTH full suites SKIPPED on its merge — a green closeout is NOT ceiling evidence"*). It is named here in advance so this lane's own green cannot be mistaken for proof.

---

## 2. What landed, and where

**Nothing in this PR is the fix.** The fix is three commits in a git repository created by this lane at `/srv/qbuild/tools/`, which no PR can carry — that being the substance of WF-0031.

| commit | what |
|---|---|
| `8978a2f` | **Baseline** — `/srv/qbuild/tools/` imported byte-exactly, **before any edit** |
| `af3e426` | **WF-0030** — `qsl-desktop` registered at six sites + the mirror created |
| `dafea7f` | **WF-0035** — dispatch fix, dead definitions removed, `check_repo_registration.sh` added |

The baseline commit is load-bearing: it is what makes the two subsequent commits *reviewable diffs against the state that ran every lane through NA-0666*, rather than an undifferentiated import.

### WF-0031 — custody
21 tracked files + `.gitignore`; `backups/` excluded (a single rsync-era `.bak` this history supersedes). Repo-local identity set to the GH007 noreply address **deliberately**, so the new repo does not inherit the WF-0029 hazard. Secret scan over the full tree: **3 hits, all false positives** — detector regexes inside `na0607-proof-tools/added_line_publication_scan.py` and a type name in a readiness checker.

### WF-0030 — registration
`env_qbuild.sh` (`qbuild_all_repos`, `qbuild_require_known_repo`, `qbuild_repo_remote`) · `qwork.sh:68` · `qshell.sh:12` · `cache/targets/qsl-desktop` in `bootstrap_qbuild.sh` + `preflight_clean.sh` · **`/srv/qbuild/mirrors/qsl-desktop.git`** (296 KB, bare, `main` = `02cc9b96`), created **in the same change**.

The mirror is not separable: `preflight_clean.sh:57` reports an issue for any repo in `qbuild_all_repos()` without one, and `capture_evidence.sh:96` reads each mirror's remotes.

**Deliberately unchanged: `qwork.sh:142`, `qwork.sh:445`, `qshell.sh:104`.** These are the spine-specific branches and they already early-return for any repo that is not `qsl-protocol`. `qsl-server` and `qsl-attachments` were the live precedent throughout — both qwork-known, neither with a queue file or evidence helper. **The gap was a missing list entry, not missing machinery**, and the filing's implied difficulty was wrong even though its diagnosis was right.

### WF-0035 — the dispatch divergence
The direct-execution block sat at `:140-160`, above the NA-0543 redefinitions at `:188`/`:239`. Both dead flat definitions deleted; the block relocated to end-of-file with a comment stating it must stay there and why.

---

## 3. Acceptance A — the before/after pair (**the only real validation**)

### BEFORE (captured by this lane, pre-registration)

```
$ bash /srv/qbuild/tools/qwork.sh NA-0667 qsl-protocol qsl-desktop
startup_result=FAIL
reason=unknown-repo
lane=NA-0667
repo=qsl-desktop
path=/srv/qbuild/work/NA-0667/qsl-desktop
exit_code=2
```

**This is WF-0030 in its own words.** The gate at `qwork.sh:478-480` rejects every unknown repo argument *before* the lock is taken and *before any repo is seated*, so the command fails **totally, not partially** — `qsl-protocol` is not seated either.

**Verified inert:** no `work/NA-0667/qsl-desktop`, no `locks/qwork.NA-0667.lock`, no proof file. This also demonstrates **WF-0027** in passing: a failed `qwork` writes nothing to disk, so the terminal output above is the only record that could exist.

### AFTER — **RESERVED FOR THE OPERATOR, post-merge**

```
qwork NA-0667 qsl-protocol qsl-desktop
```

Expected: `startup_result=OK`; `repo=qsl-desktop` with `created_or_existing=created`; `head` = `02cc9b96` or newer; `head_equals_origin_main=yes`; `worktree_clean` / `index_clean` / `untracked_clean` = `yes`; `ready_count=1` and `queue_top_ready=NA-0667` from the spine; and

```
cargo_target_dir=/srv/qbuild/cache/targets/qsl-desktop/rustc-1.95.0-x86_64-unknown-linux-gnu/default
```

**A result that does not show that keyed shared path is NOT a pass.**

⚠ **Run the WF-0033 command in §6 first**, or `shared_target_ready` will read `no` and the shared-cache benefit — the point of the whole lane — will not actually be available.

---

## 4. Acceptance B — invocation parity ✅

| repo | `--print` vs `source` |
|---|---|
| qsl-protocol | MATCH `…/qsl-protocol/rustc-1.95.0-x86_64-unknown-linux-gnu/default` |
| qsl-server | MATCH `…/qsl-server/rustc-1.95.0-x86_64-unknown-linux-gnu/default` |
| qsl-attachments | MATCH `…/qsl-attachments/rustc-1.95.0-x86_64-unknown-linux-gnu/default` |
| qsl-desktop | MATCH `…/qsl-desktop/rustc-1.95.0-x86_64-unknown-linux-gnu/default` |

**4 match, 0 diverge.** Before the fix, every one of these returned the flat path from `--print` and the keyed path from `source`.

## 5. Acceptance C and E ✅

**C — no regression on the existing three.** The entire diff to `qwork.sh` since baseline is one line:

```
-    qsl-protocol|qsl-server|qsl-attachments) return 0 ;;
+    qsl-protocol|qsl-server|qsl-attachments|qsl-desktop) return 0 ;;
```

All three spine guards verified present and intact: `qwork.sh:142`, `qwork.sh:445`, `qshell.sh:104`.

**E — revertibility demonstrated.** `git revert --no-commit af3e426` restored the three-repo state (`qbuild_all_repos` → 3 repos; `qbuild_require_known_repo qsl-desktop` → `Unknown repo: qsl-desktop`); restoring returned the four-repo state with a clean worktree.

---

## 6. ⚠ Acceptance D — **NOT MET. One privileged command outstanding.**

```
$ bash /srv/qbuild/tools/preflight_clean.sh <directive>
Missing required qbuild path: /srv/qbuild/cache/targets/qsl-desktop
exit=1
```

`/srv/qbuild/cache/targets` is **`drwxr-sr-x root victor`** — group-readable, not group-writable. The three existing per-repo roots are `victor`-owned, i.e. created when someone held root. **The operating account cannot create a fourth**, and `bootstrap_qbuild.sh` — whose job is creating exactly these directories — would fail identically.

**The lane did not work around this and ran no privileged command.** Filed as **WF-0033**. Completing it:

```
sudo install -d -o victor -g victor -m 2775 /srv/qbuild/cache/targets/qsl-desktop
```

**Why this is quiet in one direction and loud in the other, stated plainly:** `qwork` *still succeeds* — `qbuild_select_cargo_target` computes and probes the path but never creates it, so a seat prints `startup_result=OK` with the correct keyed `cargo_target_dir` and merely reports `shared_target_ready=no`. **So the registration can look complete while the shared-cache benefit is absent.** A reader checking only the qwork proof would not notice. It was `check_repo_registration.sh` — added by this lane to close WF-0030's proof gap — that surfaced it, on its first real run.

---

## 7. Corrections of record

**WF-0031's "no backup" clause was FALSE when filed.** `/srv/qbuild/tools` is source #33 in `/usr/local/sbin/qsl-backup`, `DAILY_KEEP=30`, last run SUCCESS `2026-07-22 02:36:07`. Up to thirty recoverable prior versions existed throughout. **"No diff, no history, no review path, no revert-by-commit" was true and was the real gap.** Corrected, not erased — an item that overstates its own severity distorts every sequencing decision that reads it, and this one was filed as the blocker gating WF-0027, WF-0029 and WF-0030.

**ENG-0062's "blocked by WF-0030" was over-stated.** The easy half needs only a desktop PR — nine desktop lanes landed PRs from tmp directories. The hard half (the `measure.py:73-84` `fitCode` drift seam) is independent of WF-0030 in both directions. **And it now carries a hard deadline: the only copy is deletion-eligible on/after 2026-07-29 under `qbuild-ssd-maintenance --tmp-days 7`, and `/srv/qbuild/tmp` is backup-EXCLUDED.**

**A self-correction, recorded because the lane's own subject is silent wrongness.** The first draft of `check_repo_registration.sh` tested the known-repo predicates by grepping for `"|<repo>)"`. That pattern **silently misses whichever name is first in the case arm**, and the check reported 6 false failures against correctly-registered repos on its first run. It was rewritten to extract and execute each predicate rather than grep for it. **A census that produces confident wrong answers is worse than no census** — the same reasoning ENG-0062 applies to `measure.py`.

---

## 8. Not done, and not claimed

- **Nothing deleted.** The 17.2 GB of WF-0034 stands untouched (D603 STOP-8).
- **`measure.py` not moved.** Still in `/srv/qbuild/tmp/NA0665_gui_round4a_20260722T051031Z/`, still on its 2026-07-29 clock.
- **The guardrail hook neither wired nor removed;** `settings.json` not modified.
- **No privileged command run.**
- **No `qsl-desktop` commit.** The desktop repo was read only through the mirror.
- **WF-0027 and WF-0029 unblocked but not fixed.**
- **No claim about product behaviour, security, or suite health.** This lane changed host tooling and governance records.
