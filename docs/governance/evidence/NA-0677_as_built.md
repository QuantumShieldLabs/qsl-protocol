# NA-0677 — as-built (D613, D-1308 + D-1309) — the operator-infrastructure literal gate

Result class: **`CI_HARDENING_PASS`**. FULL two-PR ritual (workflows and the advisories gate are
DOC-OPS-006 §9 hard-boundary items). Seat `qwork NA-0677 qsl-protocol qsl-desktop qsl-server
qsl-attachments`, all four `startup_result=OK`, spine `ready_count=1 · queue_top_ready=NA-0677`.

## 1. What shipped

| repo | PR | merge | decision |
|---|---|---|---|
| `qsl-desktop` | #13 | `44237b23` | D-0014 |
| `qsl-server` | #65 | `6ad078c2` | D-0015 |
| `qsl-attachments` | #41 | `999f3293` | D-0012 |
| `qsl-protocol` | #1655 | `27134984` | D-1308 |
| `/srv/qbuild/tools` | — | `410221d`, `9273417` | **no PR — that repository has no remote** |

## 2. ⚠ The premise the lane was given was false

The intent said *"port the spine's public-safety job (the operator-infra literal patterns)"*.
`scripts/ci/public_safety_gate.py:32-40` is the whole detection surface: PEM keys, `AKIA`/`ASIA`,
`ghp_`, `glpat-`, `xox*`, `AIza`, bearer headers. **No address, path or host pattern. `git grep
private_ipv4` = 0.**

**That is why it ran green on every pull request that published a private LAN address across nine
tracked files of this very repository.** The failure was the pattern set, not the scan's scope. There
was nothing to port; the lane built the missing set.

## 3. ⚠ Every control ran RED first — and three rejected what they were testing

**(a) The embedded-literal control rejected the matcher design.** Raw substring matching caught
`SOME_<name>_THING` **and** fired on every identifier that merely *spans* a camelCase seam — a
7-character host name sits inside `setServerBusy` and `commitServerSettings`, burying the real hit
under **eleven false positives from qsl-desktop's own UI code**. Matching became **token-wise**
(splitting on non-alphanumerics *and* camelCase transitions).

> **This is not a word boundary reintroduced under another name.** `\b` fails on `HOST_<name>` — the
> case the gate exists for. Token splitting catches it and drops the seam-spanning ones. Residual,
> stated: a name with no delimiter and no case change is one token and will not match.

**(b) The Tier-1 control rejected itself as invalid.** The seed file was created but never staged;
`--mode tree` scans `git ls-files`, so an untracked seed is invisible and the run came back GREEN
where RED was expected. Correct gate behaviour, invalid test — **caught only because the expected
result was written down before the run.**

**(c) The gate caught its own implementation.** Run against qsl-desktop #13, Tier 1 failed on the
scanner's own docstring, which had used a real host name to illustrate the embedded case.

> **The fifth occurrence in four days of a record naming what it redacts — and the first caught by a
> machine rather than by a person re-reading.** In the file that implements the gate, on the commit
> that introduces it.

**(d) In the spine, the control that matters is the address one.** Nine tracked files here carried a
private LAN address and the required `public-safety` context passed every one of those PRs. Seeded
again now, the new gate fails. *The lane's whole finding, demonstrated on its origin.*

**Every control was re-run in every repository.** A gate is a property of the repo it runs in — the
tree it scans, the workflow that invokes it, the checkout depth it gets — none of which the script
carries. Copying a proven script proves nothing about the repo it lands in.

## 4. ⚠ The gate had a way to pass without looking

The first CI run went green and the log read `infra-literal-scan: clean (diff)` — **a line printed
identically whether the scan examined 438 added lines or zero.**

**A green that cannot be distinguished from a no-op is the exact defect this gate was built to
answer**, reproduced inside the fix for it. Found by reading the log rather than the check mark.

Fixed two ways: the scan now reports what it examined (`clean (tree; 2283 files, 600140 lines
examined)`), and **an empty input is a failure (exit 2), not a pass** — in `tree` and `diff` modes,
where empty means a broken checkout or an unfetched base ref. **Deliberately not in `staged` mode**,
where a deletion-only or rename-only commit legitimately has no added lines and refusing would block
honest commits from the pre-commit hook.

## 5. ⚠ A job-name collision, caught by checking rather than assuming

The spine already publishes **required** contexts named `public-safety` **and** `advisories`. Naming
the new job either would have put **two check runs with one name in front of branch protection**, on
the single repository where fourteen contexts are enforced.

The live required list was read by API before choosing:

```
required : ci-4a, ci-4b, ci-4c, ci-4d, ci-4d-dur, demo-cli-build, demo-cli-smoke,
           formal-scka-model, goal-lint, metadata-conformance-smoke, suite2-vectors,
           CodeQL, macos-qsc-qshield-build, public-safety
new      : infra-literal-scan          collision: NONE
```

Entirely additive: new workflow file, new job id, **`public-ci.yml` byte-untouched (zero diff)**.
D613 §7 anticipated the adjacent risk — *do not edit the 2,599-line gate* — but not the name clash.

## 6. The design

**Tiers.** Tier 1 (network-identifying + personal identity) whole-tree, fail on any hit. Tier 2b
(low-frequency private names) added-lines only. **Tier 2a (build-root and home paths) not scanned** —
the citation convention puts them in ~60% of spine commits (17/30 and 10/30, D613 C8); they remain
NA-0676's published residue.

**Salted digests, and why.** These repositories are public. A pattern file naming the private hosts
would republish exactly what NA-0676 removed **and make the Tier-1 scan hit its own pattern file** —
verified against a scratch repository, not assumed. Operator-approved (D613 §2a-ter) with the
plaintext list **operator-held**; §2a's one-screen reviewability is preserved for the tier structure,
class labels and structural regexes and **deliberately traded away for the name values**.

> ⚠ **One part of that ruling is not achievable, and is recorded rather than fudged.** The **salt
> must stay in the public file**: the scanner hashes candidate tokens at scan time and cannot compare
> them against stored digests without it, so an operator-held salt means the gate cannot run in
> public CI at all. An env-var or CI-secret salt breaks fork PRs and local runs — a worse failure
> than a weak salt. The public file says plainly that the salt defeats only a *precomputed* lookup
> and is not a secret from anyone who already knows a name. **The protection that matters is the
> names' absence from the tree.**

**Scan cost:** the spine is the largest tree — **2,283 files, 600,140 lines, ~12 s.**

## 7. ENG-0074 and ENG-0064 — both closed, and both proof gaps closed with them

**ENG-0074** shipped as a **fail-closed assertion**, not the recorded line its filing proposed,
because the operator's instruction said *assert*: `qwork_assert_seat_identity()` on **both** seat
paths plus `new_checkout.sh`, reading the **effective** identity so an inherited-from-global value is
visible rather than masked by an empty local key. `seat_user_name` / `seat_user_email` also land in
the KV and both JSON proof writers.

> **The filing's framing now holds in both directions:** the pre-fix world was dangerous but visible;
> the post-fix world was safe but invisible; it is now **safe and observed**.

**ENG-0064** closed in the same commit (FLAG-C4). The root cause was one line: the guard could not
tell an operator-set `CARGO_TARGET_DIR` from the one qbuild exported moments earlier **in the same
invocation**. `QBUILD_CARGO_TARGET_SET_BY_QBUILD` fixes it, and **the operator-set override is
preserved** — that was the whole risk in the change and it got its own control.

⚠ **The baseline, captured before the fix, is worse than every prior report: on this lane's four-repo
seat, THREE of four repos were wrong.** The defect scales with the seat. Sighted four times
(NA-0670 filing, NA-0674, NA-0675, NA-0677) before it was fixed — each time caught by hand.

**Positive controls, by the operator-approved OBS-O method** — the assertions called **directly**
against a broken state, so the executor never ran `qwork`: identity FAILS on a wrong seat and PASSES
on a correct one; target-dir FAILS when a repo in shared mode points at another repo's tree.

## 8. `enforce_admins` — done and verified

The census observed `qsl-attachments` at `false` while the other three read `true`, and **reported it
without acting** — branch protection is the operator's act. The operator flipped it; the executor
**verified independently by API: `enforce_admins.enabled = true`, required contexts still `["rust"]`**,
confirming the change touched only what it was meant to. **All four repositories now read `true`.**

## 9. ⚠ Still owed, and it is the operator's

**The four new checks are ADVISORY, not blocking.** `public-safety` in the three satellites (each
requiring exactly one context, `rust`) and `infra-literal-scan` in the spine (whose fourteen required
contexts do not include it) **run and report on every PR but cannot block a merge until added to the
required sets.**

**Green is not the same as blocking.** No part of this lane made a branch-protection API call for the
required sets. Until that deliberate act, the gate is a very good early-warning system and not a gate.

## 10. Claim boundary

A PASS asserts that the gate **exists, runs, and has been proved capable of failing** — in four
repositories, on the classes it claims to cover, including the embedded form a word-boundary pattern
misses.

**It does NOT claim the gate has ever caught a real leak. It has not, by construction:** NA-0676
cleaned every tree first, which is why every Tier-1 scan is green on arrival (D613 §3's sequencing).
**The value is prospective, and the controls are what make that a reasonable bet rather than a hope.**

⚠ The closeout PR is `docs_only`, so the behavioural suites correctly **SKIP** on its merge — **its
green proves the governance edit is well-formed and nothing else.** #1655's full-suite run
(36 pass / 2 skip / 0 fail, the two skips keyed to `qsc/src` changes it did not make) and the
controls are the evidence. No security, production, crypto-complete or vulnerability-free claim.
