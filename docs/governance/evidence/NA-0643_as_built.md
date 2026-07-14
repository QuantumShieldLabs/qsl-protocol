# NA-0643 As-Built — ENG-0041 Pin Bump + NA-0640 E2E Re-Run (D579, D-1266)

Goals: G4
Status: Supporting evidence (lane closeout)
Owner: QSL governance
Last-Updated: 2026-07-13

## 1. What this lane shipped, and where

LITE lane per QSL-DIR-2026-07-13-579 (D579), seated by promotion PR #1565,
base main `5153e7c1`. Pays ENG-0041 (filed by NA-0642 at D-1265 as an OWED
follow-up). Exactly two non-governance files changed:

- `qsl/qsl-client/qsc/Cargo.toml` — the ONE dev-dependency line (~line 38):
  qsl-server rev `19b9b02dbe1f2ae9bc246ff3a16890e56c073c3e` (pre-durability)
  → `8e4ea27877db46a2b660b46c36ba60f3db73b38c` (the NA-0642 durability merge,
  qsl-server PR #61).
- `Cargo.lock` — mechanical regeneration via a SCOPED `cargo update -p
  qsl-server`: the qsl-server source-rev swap plus its NEW transitive
  dependencies on the dev edge (rusqlite 0.32.1, libsqlite3-sys 0.30.1,
  hashlink 0.9.1, fallible-iterator 0.3.0, fallible-streaming-iterator 0.1.9,
  ahash 0.8.12, hashbrown 0.14.5, vcpkg 0.2.15 — the SQLite stack NA-0642
  added to qsl-server). 149 dependencies untouched. +79/−4 lines total across
  the two files.

Forbidden-surface proof: NO test-file change (`NA_0640_full_stack_e2e.rs` and
`tests/common/mod.rs` untouched — verified in the diff); NO qsc/protocol/
qsl-server source; NO `formal/`, vectors, canonical, `.github`; NO ack-client
work (ENG-0040 remains a separate owed lane).

## 2. Phase 0 (CONFIRM-LIVE, all green)

- qwork proof: `HEAD == origin/main == 5153e7c1`, clean tree; `ready_count=1`;
  `queue_top_ready=NA-0643`.
- D-number: highest DECISIONS entry D-1265 → this lane is **D-1266**.
- Target rev confirmed CURRENT: fresh `git ls-remote
  https://github.com/QuantumShieldLabs/qsl-server.git main` (GitHub direct,
  NOT the stale qbuild mirror) → `8e4ea27877db46a2b660b46c36ba60f3db73b38c`.
- ENG-0041 `Status: open` at ledger line 1533 before this lane.

## 3. The proof (THE deliverable): the NA-0640 e2e green UNCHANGED

Run LOCALLY (the e2e does not run on PRs — the local run is the only gate),
first invocation after the bump, zero retries, zero test edits:

```
$ cargo test -p qsc --test NA_0640_full_stack_e2e
   Compiling rusqlite v0.32.1
   Compiling qsl-server v0.1.0 (https://github.com/QuantumShieldLabs/qsl-server.git?rev=8e4ea27877db46a2b660b46c36ba60f3db73b38c#8e4ea278)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 32.13s
     Running tests/NA_0640_full_stack_e2e.rs

running 2 tests
test full_stack_message_round_trip_token_auth_relay ... ok
test full_stack_message_and_attachment_round_trip_open_relay ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 115.57s
```

Both scenarios pass against the DURABLE server: message round-trip with
plaintext byte-match + receipts, the >4 MiB (6 MiB+321 B) attachment
byte-verified through the real qsl-attachments service and real relay, open
AND bearer-token auth modes, and the wrong-bearer NEGATIVE still rejected.
This is the NA-0642 backward-compat guarantee (legacy pull byte-identical)
holding END-TO-END, converted from analysis into an artifact. The ENG-0041
analysis held exactly as filed: the in-process `start_qsl_server` harness
uses the library constructor, which defaults to `:memory:`, so the new
binary-only STORE_PATH requirement never applied — no harness change needed.

Full standard merge gate:

```
$ cargo test -p qsc
aggregate: 603 passed / 0 failed / 3 ignored across all 149 test-result sets
(the unit tests, the 146 integration-test binaries — including the NA-0640
e2e re-run and the NA-0639 crash-window harness — and the doc-tests);
exit code 0; zero FAILED lines, zero panics in the full log.
```

The 3 ignored are pre-existing `#[ignore]` tests, unchanged by this lane (the
diff contains no test file).

## 4. Dev-edge-only proof (the NA-0640 discipline)

`cargo tree -p qsc -e normal` (the PRODUCTION dependency graph) captured
BEFORE and AFTER the bump:

```
sha256(before) = 3b0e88967eeae7bd0e4b6b79890deebe081f9fad6c5335a94b513768f55fcc83  (483 lines)
sha256(after)  = 3b0e88967eeae7bd0e4b6b79890deebe081f9fad6c5335a94b513768f55fcc83  (483 lines)
diff           = empty (byte-identical)
```

The shipped-binary graph is unchanged; every lock delta rides the dev edge
(`qsl_server` is a `[dev-dependencies]` entry used only by the e2e).

## 5. Validation

- Scope guard: `git status` delta = exactly `qsl/qsl-client/qsc/Cargo.toml` +
  `Cargo.lock` + the governance files listed in D579's allowed paths.
- No test-file change: the diff contains no `tests/**/*.rs` hunk.
- goal-lint: run locally against the lane PR (synthesized event payload).
- ENG-0041 closed on the ledger citing this green run + the new pin rev.

## 6. Limits (stated, per D579's classification rule)

A PASS asserts the CURRENT e2e scenarios pass against the durable server at
`8e4ea278`. It does NOT exercise the new durability/ack features (`?ack=lease`,
`/v1/pull/ack`, retention, restart-durability from the client side) — that
coverage arrives with ENG-0040 (the qsc ack-client lane, still OWED, which now
has the durable server available in the dev-dep). Claim boundary UNCHANGED.
