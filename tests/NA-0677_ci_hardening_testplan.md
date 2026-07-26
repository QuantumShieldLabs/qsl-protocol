# NA-0677 — CI hardening — test plan (D613, D-1308 + D-1309)

The lane's deliverable is a **gate**, and a gate that is subtly wrong looks exactly like a gate that
is right — it stays green either way. So the acceptance is the **controls**, not the check marks.

## A. The four required controls (D613 §5)

| # | control | expected | result |
|---|---|---|---|
| A1 | §5.1 Tier-1 literal seeded in a **tracked** file | scan FAILS | ✅ RED → green |
| A2 | §5.1 Tier-2b literal on an **added** line | scan FAILS | ✅ RED → green |
| A3 | **§5.1a EMBEDDED literal** (`SOME_<name>_THING`) | scan FAILS | ✅ RED → green |
| A4 | §5.1b staged-set scan (pre-commit call site) | refuses the commit | ✅ RED → green |
| A5 | §5.2 identity assertion vs a wrong-identity seat | assertion FAILS | ✅ FAIL → pass |
| A6 | §5.3 target-dir assertion vs a foreign target dir | assertion FAILS | ✅ FAIL → pass |

**A5/A6 were called DIRECTLY** against a deliberately broken state (operator-approved OBS-O method),
so **the executor never ran `qwork`** and the control tests the assertion rather than the whole path.

## B. Re-run per repository — the proof was not inherited

| repo | embedded control | tree scan at landing |
|---|---|---|
| `qsl-desktop` | ✅ RED → green | clean; 42 files, 14,317 lines |
| `qsl-server` | ✅ RED → green | clean; 74 files, 13,896 lines |
| `qsl-attachments` | ✅ RED → green | clean; 39 files, 13,391 lines |
| `qsl-protocol` | ✅ RED → green **+ the private-address control** | clean; **2,283 files, 600,140 lines, ~12 s** |

**Why re-run at all, given the scanner is `cmp`-proven byte-identical:** a gate's behaviour depends on
the tree it scans, the workflow that invokes it and the checkout depth it gets — none of which the
script carries.

## C. ⚠ Three controls rejected what they were testing

| # | what happened | consequence |
|---|---|---|
| C1 | the embedded control fired **and** returned 11 false positives from one repo's own UI code (a 7-char host name inside `setServerBusy`, `commitServerSettings`) | **the matcher was redesigned** — substring → token-wise, splitting on non-alphanumerics *and* camelCase transitions |
| C2 | the Tier-1 control came back GREEN where RED was written down | **the test was invalid** — an unstaged seed is not tracked, so `--mode tree` correctly did not see it. Caught only because the expectation was recorded first |
| C3 | the gate **failed on its own implementation** — the scanner's docstring used a real host name as an example | fifth occurrence in four days of a record naming what it redacts; **the first caught by a machine** |

## D. ⚠ The vacuous-pass defect (found by reading the log, not by a control)

| # | check | result |
|---|---|---|
| D1 | did the first green prove anything was examined? | **NO** — `clean (diff)` printed identically for 438 lines or 0 |
| D2 | scan now states scope | ✅ `clean (tree; 2283 files, 600140 lines examined)` |
| D3 | empty input in `tree`/`diff` mode | ✅ **exit 2, refuses to report a pass** |
| D4 | empty input in `staged` mode | ✅ **passes** — a deletion-only commit legitimately has no added lines |

**D4 is the deliberate exception.** Refusing there would block honest commits from the pre-commit hook.

## E. The advisories gate

| # | check | result |
|---|---|---|
| E1 | `cargo audit --deny warnings`, qsl-desktop, with all 17 IDs waived | ✅ exit 0 |
| E2 | same, with **one ID removed** from the waiver | ✅ **exit 1 — still fails on an unwaived advisory** |
| E3 | `RUSTSEC-2024-0429` dispositioned **separately as UNSOUND**, not as unmaintained | ✅ in its own block, stating what is accepted |
| E4 | qsl-server, qsl-attachments — no waiver needed | ✅ exit 0 both |
| E5 | waiver location | ⚠ **must be `.cargo/audit.toml`** — a root-level file is silently ignored |

## F. Clippy `--all-targets`

| # | repo | before | after |
|---|---|---|---|
| F1 | qsl-desktop | 5 findings, all `field_reassign_with_default` in **test code** | ✅ fixed, clean |
| F2 | qsl-server | already clean (measured) | ✅ flag added defensively |
| F3 | qsl-attachments | already enforced | ✅ unchanged |
| F4 | production code changed | — | **none, in any repo** |

## G. Collision and protection safety

| # | check | result |
|---|---|---|
| G1 | spine required contexts read **by API** before naming the job | ✅ 14 contexts listed |
| G2 | collision between new job id and required contexts | ✅ **NONE** |
| G3 | `public-ci.yml` | ✅ **byte-untouched, zero diff** |
| G4 | satellite `rust` context id | ✅ unchanged in all three |
| G5 | branch-protection API calls for required sets | ✅ **none made** — operator's act |
| G6 | `enforce_admins` | ✅ **true on all four**, operator-run, **executor-verified by API** |

## H. Boundaries held

| # | boundary | result |
|---|---|---|
| H1 | no baseline / allowlist / suppression file to make a tier go green | ✅ none |
| H2 | no word-boundary anchoring added to quiet a pattern | ✅ token-wise instead |
| H3 | pattern set not forked across repos | ✅ `cmp`-proven identical ×4 |
| H4 | no dependency or lockfile change | ✅ |
| H5 | spine `public_safety_gate.py` logic | ✅ untouched — new script, own file |
| H6 | `~/.gitconfig` | ✅ untouched |
| H7 | private literals added to any public tree | ✅ **zero** — names are salted digests |

## I. What this plan does NOT establish

- **That the gate has ever caught a real leak.** It has not, **by construction** — NA-0676 cleaned
  every tree first, so every Tier-1 scan is green on arrival. The value is **prospective**.
- **That the new checks block anything.** They are **advisory** until added to the required sets.
- Anything about the classes deliberately left unscanned (Tier 2a).
- ⚠ The closeout PR is `docs_only`: the behavioural suites correctly **SKIP**, and **its green proves
  the governance edit is well-formed and nothing else.** #1655's full-suite run and these controls
  are the evidence.
