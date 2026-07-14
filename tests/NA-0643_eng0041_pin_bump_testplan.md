# NA-0643 Testplan — ENG-0041 Pin Bump + NA-0640 E2E Re-Run (D579, D-1266)

Goals: G4
Status: Supporting
Owner: QSL governance
Last-Updated: 2026-07-13

## Objective

LITE lane: prove the NA-0640 full-stack e2e passes UNCHANGED against the
durable qsl-server (the NA-0642 merge `8e4ea278`) after bumping the dev-dep
pin from `19b9b02d`. The GREEN LOCAL RUN is the deliverable — the e2e does not
run on PRs, so the local run is the only gate. NO test file was changed; NO
product source was changed; the lock delta is dev-edge-only, proven.

## Validation matrix (all executed locally at the lane checkout, main base
`5153e7c1`, after the bump)

| # | Check | Method | Result |
|---|-------|--------|--------|
| 1 | Target rev is qsl-server main HEAD | fresh `git ls-remote https://github.com/QuantumShieldLabs/qsl-server.git main` (NOT the stale qbuild mirror) | PASS — `8e4ea278…` exactly |
| 2 | THE DELIVERABLE: NA-0640 e2e green UNCHANGED | `cargo test -p qsc --test NA_0640_full_stack_e2e`, first run post-bump, zero edits | PASS — 2 passed / 0 failed (115.57s): message + >4 MiB attachment round-trips byte-verified, open + token auth, wrong-bearer negative rejected |
| 3 | Full standard merge gate | `cargo test -p qsc` (entire suite) | PASS — 603 passed / 0 failed / 3 ignored (pre-existing `#[ignore]`) across all 149 test-result sets, exit 0; zero panics |
| 4 | Dev-edge-only (production graph unchanged) | `cargo tree -p qsc -e normal` before vs after | PASS — byte-identical (sha256 `3b0e8896…` both sides; empty diff) |
| 5 | Lock delta mechanical + scoped | `cargo update -p qsl-server` only; inspect `Cargo.lock` diff | PASS — qsl-server rev swap + its new dev-edge transitives (rusqlite/libsqlite3-sys/hashlink/fallible-*/ahash/hashbrown/vcpkg); 149 deps unchanged |
| 6 | No test-file / source change | `git diff --name-only` scope guard | PASS — only `qsl/qsl-client/qsc/Cargo.toml`, `Cargo.lock`, and D579's allowed governance paths |

## Non-vacuity note

Check 2 is not vacuous by construction: the identical test binary previously
exercised the PRE-durability server (`19b9b02d`, NA-0640 baseline green), and
the NA-0642 server changed the pull pipeline underneath the legacy contract
(store-backed delete-on-deliver replacing the in-memory VecDeque). A
backward-compat break in the legacy pull would fail the byte-match assertions
here. The negative direction (a server that DOES break the contract) is
guarded server-side by qsl-server's `na0642_backward_compat.rs` exact-field-set
tests.

## What this plan does NOT cover (stated)

The new durability/ack surface (`?ack=lease`, `/v1/pull/ack`, retention TTL,
client-observed restart durability) — deliberately out of scope; arrives with
ENG-0040 (the qsc ack-client lane). No fault injection against the real relay
(unchanged NA-0640 limit).
