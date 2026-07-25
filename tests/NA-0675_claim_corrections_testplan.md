# NA-0675 — claim corrections — test plan (D611, D-1306)

The instrument for a claims lane is mostly `grep` and one `assert!`. What makes it evidence rather
than ceremony is that **the claim was measured before it was written**, and that **the new pin was
made to fail before it was trusted**.

## A. Phase 0 — the claim, proven before the sentence was written (D611 §7.4)

| # | check | expected | result |
|---|---|---|---|
| A1 | `grep -c 'invoke("relay_test"' ui/main.js` | `1` | ✅ `1` |
| A2 | that call site is inside the Test-connection handler | yes | ✅ `ui/main.js:1173`, in the Test handler |
| A3 | every other frontend `invoke` is local | yes | ✅ vault / settings / identity / protection only |
| A4 | `README.md:15-22` still byte-matches D611 §0 | yes | ✅ re-read, not pattern-matched |
| A5 | `profile/README.md:69-70` and `:73-74` byte-match D611 §0 | yes | ✅ |
| A6 | desktop `main` == D611 §0 (`b4ea47e7`) | yes | ✅ unmoved |

**A1–A3 are the lane's substantive measurement.** The published sentence asserts a property of the
code; the property was established first, and the sentence follows it.

## B. The A2 anti-regression pin — POSITIVE CONTROL (D611 §4.6)

The pin is `assert!(!repo_file("README.md").contains("makes no network connections at all"))`,
added to `claim_discipline_five_surfaces_swept` in `src-tauri/tests/server_pane.rs`.

| # | step | expected | result |
|---|---|---|---|
| B1 | run the pin against the **uncorrected** README | **FAIL** | ✅ `panicked at server_pane.rs:429:5: README status section still says makes no network connections at all` · `FAILED. 0 passed; 1 failed` |
| B2 | apply the README correction, re-run | **pass** | ✅ `ok. 1 passed; 0 failed` |
| B3 | the pin's path resolves to a real file | yes, or panic | ✅ `repo_file()` = `CARGO_MANIFEST_DIR/../<rel>` and **panics** on a failed read — a silently-empty read is not constructible |

Captured: `/srv/qbuild/evidence/NA-0675/a2_pin_positive_control.txt`.
**A pin that has never failed is not known to pin anything.**

## C. Acceptance greps (D611 §4.2–§4.3)

| # | check | expected | result |
|---|---|---|---|
| C1 | `grep -c "makes no network connections at all" README.md` | `0` | ✅ `0` |
| C2 | same, `profile/README.md` | `0` | ✅ `0` |
| C3 | `grep -c "cannot send a message" profile/README.md` | `1` (the true clause survives) | ✅ `1` |
| C4 | residual occurrences in `qsl-desktop`, all non-claims | guards + record only | ✅ 5 — three in `server_pane.rs` (2 needles + the failure message), two in `DECISIONS.md` quoting what was retired |
| C5 | the profile Repositories table (C1 census) | byte-unchanged | ✅ 0 table rows in the diff |

## D. Scope containment (D611 §4.4, §5)

| # | check | expected | result |
|---|---|---|---|
| D1 | `src-tauri/src/**` files changed | **0** | ✅ 0 |
| D2 | `settings.rs` autolock default | untouched, stays 60 | ✅ untouched (C4 — 60 supersedes 15 per D-0005; residue is WF-0024) |
| D3 | `Cargo.toml` / `Cargo.lock` | untouched | ✅ |
| D4 | qsl-desktop files changed | exactly `README.md`, `server_pane.rs`, `DECISIONS.md` | ✅ 3 |
| D5 | `.github` files changed | exactly `profile/README.md` | ✅ 1 |
| D6 | spine claim documents (`docs/public/**` etc.) | untouched (OBS-P) | ✅ untouched, nothing filed |
| D7 | `CLAUDE.md` | byte-untouched (WF-0032) | ✅ |

## E. Gates

| # | gate | result |
|---|---|---|
| E1 | qsl-desktop `cargo test -q` | ✅ **73 pass / 1 ignored** |
| E2 | `cargo fmt --all -- --check` | ✅ clean |
| E3 | `cargo clippy -q -- -D warnings` | ✅ clean |
| E4 | `git diff --check` (all three repos) | ✅ clean |
| E5 | qsl-desktop PR #12 required context `rust` | ✅ **SUCCESS**, `mergeStateStatus=CLEAN` |
| E6 | `.github` PR #5 | ⚠ **no checks exist — that repository has no workflows.** A property of the repo, not a skipped gate, and **not reportable as green** |
| E7 | infra-literal scan of every added line | ✅ zero addresses / host names / account names / personal addresses |
| E8 | GH007 identity on every commit, verified on the object | ✅ author and committer, trailers empty |

## F. §2a — the retirement, verified by reading (D611 §4.7)

| # | check | expected | result |
|---|---|---|---|
| F1 | §4a.4's original text still present and legible | yes | ✅ table, discrepancy and "the relay file did NOT supersede…" all intact |
| F2 | the retirement note is dated and carries the operator's rationale | yes | ✅ |
| F3 | five discharges named individually | NA-0664, NA-0665, D611, D612, D613 | ✅ all five, in a table |
| F4 | §1 template element marked retired | yes | ✅ |
| F5 | §3 path + 575 archived files kept | yes | ✅ nothing renamed or removed |
| F6 | §4a.1 marked as the sole required artifact | yes | ✅ |
| F7 | `CLAUDE.md` byte-untouched | yes | ✅ (WF-0032; its staleness has flipped from "incomplete" to "wrong", and the note says so) |

## G. What this plan does NOT establish

- Nothing about messaging, any security property, or release readiness.
- Nothing about the `.github` page beyond the operator's read — **that repository has no automated
  gate of any kind**.
- The pin guards **one sentence**. It does **not** guard the heading: the old
  *"Status: slice A — the serverless skeleton"* contains none of the pinned phrases and would have
  passed. Carried to Lane C's claim-gate thinking, not fixed here.
- ⚠ This closeout is `docs_only`, so the behavioural suites correctly **SKIP** on its merge.
  **Its green proves the governance edit is well-formed and nothing else.**
