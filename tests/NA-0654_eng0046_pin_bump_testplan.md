# NA-0654 Testplan — ENG-0046 Pin Bump + NA-0640 E2E Re-Run (D590, D-1277)

Goals: G4
Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-17

## Objective

LITE lane: prove the NA-0640 full-stack e2e passes UNCHANGED against the
server-info-bearing qsl-server (the NA-0652 merge `3cc551a8`) after bumping
the dev-dep pin from `8e4ea278`. The GREEN LOCAL RUN is the deliverable — the
e2e does not run on PRs, so the local run is the only gate. NO test file was
changed; NO product source was changed; the lock delta is EXACTLY the
qsl_server rev advance (1/1), proven; the production graph is byte-identical,
proven.

## Validation matrix (all executed locally at the lane checkout, main base
`e8bf93cc`)

| # | Check | Method | Result |
|---|-------|--------|--------|
| 1 | Target rev is qsl-server main HEAD | fresh `git ls-remote https://github.com/QuantumShieldLabs/qsl-server.git main` (NOT the stale qbuild mirror) | PASS — `3cc551a8…` exactly; unmoved since drafting (Phase-0 STOP not triggered) |
| 2 | Suite baseline reproduced LIVE at base (pre-bump) | full `cargo test -p qsc` at `e8bf93cc` BEFORE any edit; per-set results normalized (timing-stripped) | PASS — 412 passed / 0 failed / 1 pre-existing-ignored across all 108 result sets, exit 0 = the repo-truth NA-0649 record EXACTLY; e2e within: 2/0 (115.97s) against the OLD rev |
| 3 | THE DELIVERABLE: NA-0640 e2e green UNCHANGED post-bump | `cargo test -p qsc --test NA_0640_full_stack_e2e`, FIRST run post-bump, zero edits | PASS — 2 passed / 0 failed (118.47s): message + >4 MiB attachment round-trips byte-verified, open + token auth, wrong-bearer negative rejected; compile line proves `3cc551a8` built |
| 4 | Full standard merge gate post-bump | `cargo test -p qsc` (entire suite) | PASS — 412 passed / 0 failed / 1 pre-existing-ignored across all 108 result sets, exit 0; zero panics; e2e within the run 2/0 (117.49s) |
| 5 | Per-set multiset comparison base vs head | normalized per-set results (target :: counts, sorted, timing-stripped) diffed | PASS — normalized per-set files sha256-IDENTICAL (`5ea8a2d2…` both sides; empty diff): all 108 sets identical in target, status, and counts |
| 6 | Lock delta EXACTLY the rev advance | `git diff --numstat` + changed-lines census; `cargo metadata --locked` | PASS — 1/1 (Cargo.lock) + 1/1 (Cargo.toml), only the two rev strings; metadata --locked exit 0 (zero re-resolution). The drafted scoped-update method produced +5 windows-sys edge flips PROVEN pre-existing resolver drift (control at zero rev change reproduces them); operator-ruled Option 1 in-session: hand-apply the rev line, rail holds unamended, drift surfaced-not-filed |
| 7 | Dev-edge-only (production graph unchanged) | `cargo tree -p qsc -e normal` before vs after | PASS — byte-identical (sha256 `b8206499…`, 371 lines, both sides; empty diff) |
| 8 | No test-file / source change | `git diff --name-only` scope guard | PASS — only `qsl/qsl-client/qsc/Cargo.toml`, `Cargo.lock`, and D590's allowed governance paths |

## Non-vacuity note

Check 3 is not vacuous by construction: the identical test binary previously
exercised the pre-server-info relay (`8e4ea278`, green at check 2 within the
base run), and the bump swaps the ONLY real server in the standard suite. A
regression in the NA-0652 delta's handling of push/pull/ack would fail the
byte-match assertions here. The additive guarantee is guarded server-side by
qsl-server's NA-0642 contract tests + the NA-0652 exact-field-set tests (108/0
at the pinned rev).

## What this plan does NOT cover (stated)

`/v1/server-info` itself — the e2e does not probe it (the ledger's optional
enrichment DECLINED per the operator scope line at approval); client-side
coverage arrives with the GUI skeleton (DOC-PROG-004 step 5). The ack/lease
default flip (ENG-0043) and the windows-sys resolver-drift hygiene (surfaced
for operator disposition, un-filed by instruction) are separate concerns. No
fault injection against the real relay (unchanged NA-0640 limit).
