# NA-0667 test plan — qbuild tooling custody and `qsl-desktop` registration (D603 / D-1293)

## 0. What can and cannot be tested here

**No test in this repository exercises anything this lane changed.** The changed code is at `/srv/qbuild/tools/`, outside every repository, on a single host. GitHub runners have no `/srv/qbuild`, so a spine CI test asserting any of it would fail on every runner or be a silent no-op.

**Consequently: `docs_only=true` on this PR, both full suites SKIP, and a green check validates nothing about this lane.** The checks below are host-local and were run by the executor except where marked **OPERATOR**.

The durable regression check lives with the code it checks: `/srv/qbuild/tools/check_repo_registration.sh`.

---

## 1. Acceptance A — before/after `qwork` (**the lane's central claim**)

| # | check | method | result |
|---|---|---|---|
| A1 | pre-registration, the two-repo seat fails | `bash qwork.sh NA-0667 qsl-protocol qsl-desktop` | ✅ `startup_result=FAIL`, `reason=unknown-repo`, exit 2 |
| A2 | that failure is inert | no `work/NA-0667/qsl-desktop`, no lock, no proof | ✅ nothing created (also demonstrates WF-0027) |
| A3 | post-registration, the same command seats cleanly | `qwork NA-0667 qsl-protocol qsl-desktop` | **OPERATOR, post-merge** |
| A4 | the seat routes to the shared keyed cache | `cargo_target_dir=…/qsl-desktop/rustc-<ver>-<host>/default` | **OPERATOR** — a result without the keyed path is NOT a pass |
| A5 | the spine queue gate still runs on the two-repo form | `ready_count=1`, `queue_top_ready=NA-0667` | **OPERATOR** |

⚠ **A4 depends on WF-0033 (§5).** Without it `shared_target_ready=no` and the benefit is unavailable even though A3 passes.

## 2. Acceptance B — invocation parity (WF-0035 regression)

| # | check | result |
|---|---|---|
| B1 | `--print` == `source` for `qsl-protocol` | ✅ MATCH, keyed |
| B2 | `--print` == `source` for `qsl-server` | ✅ MATCH, keyed |
| B3 | `--print` == `source` for `qsl-attachments` | ✅ MATCH, keyed |
| B4 | `--print` == `source` for `qsl-desktop` | ✅ MATCH, keyed |
| B5 | parity is asserted durably, not just once | ✅ `check_repo_registration.sh` asserts it per repo |

**Pre-fix, B1–B4 all diverged** (flat from `--print`, keyed from `source`). B5 is what makes a recurrence loud.

## 3. Acceptance C — no regression on the existing three

| # | check | result |
|---|---|---|
| C1 | `qwork.sh` diff since baseline is one line | ✅ only the `qwork_known_repo` case arm |
| C2 | `qwork.sh:142` queue-verification guard intact | ✅ present, byte-unchanged |
| C3 | `qwork.sh:445` queue-proof-fields guard intact | ✅ present, byte-unchanged |
| C4 | `qshell.sh:104` fast-forward guard intact | ✅ present, byte-unchanged |
| C5 | all five edited files parse | ✅ `bash -n` clean ×5 |
| C6 | remote + mirror resolve for all four repos | ✅ |
| C7 | spine seat still carries its queue check | **OPERATOR** (A5 covers it) |

## 4. Acceptance E — revertibility

| # | check | result |
|---|---|---|
| E1 | baseline commit precedes every edit | ✅ `8978a2f` |
| E2 | reverting the registration restores the 3-repo state | ✅ `qbuild_all_repos` → 3; `qsl-desktop` → `Unknown repo` |
| E3 | restoring returns the 4-repo state, worktree clean | ✅ |

## 5. ⚠ Acceptance D — **NOT MET**

| # | check | result |
|---|---|---|
| D1 | `preflight_clean.sh` reports zero issues with four repos | ❌ **exit 1** — `Missing required qbuild path: /srv/qbuild/cache/targets/qsl-desktop` |

`/srv/qbuild/cache/targets` is `drwxr-sr-x root victor`; the operating account cannot create a fourth per-repo root. Filed **WF-0033**. Completion, **OPERATOR**, before A3/A4:

```
sudo install -d -o victor -g victor -m 2775 /srv/qbuild/cache/targets/qsl-desktop
```

**The lane ran no privileged command and did not work around the permission.**

## 6. Negative checks — what must NOT have happened

| # | check | result |
|---|---|---|
| N1 | nothing deleted under `cache/targets/` (STOP-8) | ✅ 17.2 GB untouched |
| N2 | `measure.py` and companions not moved or edited | ✅ untouched, still on the 2026-07-29 clock |
| N3 | `qsl_guardrails_hook.sh` not wired, not removed | ✅; `settings.json` not modified |
| N4 | no `qsl-desktop` commit | ✅ read-only via the mirror |
| N5 | no privileged command run | ✅ |
| N6 | executor ran no `qwork`/`qstart`/`qresume` **seat** | ✅ — the only invocation was the A1 capture, which provably seats nothing |
| N7 | spine diff is governance markdown only | ✅ `docs_only=true` |

## 7. Self-correction found in-lane

The first draft of `check_repo_registration.sh` grepped for `"|<repo>)"` to test the known-repo predicates. **That misses whichever name is first in the case arm**, and it produced **6 false failures** against correctly-registered repos on its first run. Rewritten to extract and execute each predicate. Recorded because a census that returns confident wrong answers is worse than no census — the same standard ENG-0062 sets for `measure.py`.
