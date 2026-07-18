# NA-0654 As-Built — ENG-0046 Pin Bump + NA-0640 E2E Re-Run (D590, D-1277)

Goals: G4
Status: Supporting evidence (lane closeout)
Owner: QSL governance
Last-Updated: 2026-07-17

## 1. What this lane shipped, and where

LITE lane per QSL-DIR-2026-07-17-590 (D590, approved 2026-07-17 with F1
resolved: LITE stands per the executed NA-0643 precedent and the merged
ENG-0046 ledger recommendation), seated by promotion PR #1589, base main
`e8bf93cc` (the seating merge; qwork-proven). Pays ENG-0046 (filed by NA-0652
at D-1275 as an OWED follow-up). Exactly two non-governance files changed:

- `qsl/qsl-client/qsc/Cargo.toml` — the ONE dev-dependency line (line 34):
  qsl-server rev `8e4ea27877db46a2b660b46c36ba60f3db73b38c` (pre-server-info)
  → `3cc551a8d9cfd8f8f53d51e0b98d10a5dc62c944` (the NA-0652 server-info merge,
  qsl-server PR #62).
- `Cargo.lock` — EXACTLY the qsl_server rev advance: the ONE `source =` line
  of the `[[package]] qsl-server` stanza (line 1982), both rev occurrences.
  numstat 1 insertion / 1 deletion per file (+2/−2 total across the two
  files) — TIGHTER than the NA-0643 precedent (which legitimately carried 8
  new SQLite dev-edge transitives). Zero transitive movement, exactly as the
  D-1275 additive census predicted (qsl-server's own Cargo.toml/Cargo.lock
  are byte-identical between the two revs).

Forbidden-surface proof: NO test-file change (`NA_0640_full_stack_e2e.rs` and
`tests/common/` untouched — the diff contains no `tests/**/*.rs` hunk); NO
qsc/protocol/qsl-server source; NO other dependency movement; NO GUI code; NO
server-info consumption or probing (the ledger's optional e2e probe DECLINED
per the operator scope line at approval — consumption is the GUI skeleton's,
DOC-PROG-004 step 5); NO `formal/`, vectors, canonical, `.github`.

## 2. Phase 0 (CONFIRM-LIVE, all green)

- qwork proof (operator-run, verified in-lane from
  `/srv/qbuild/work/NA-0654/.qwork/`): `startup_result=OK`, `HEAD ==
  origin/main == e8bf93cc`, worktree/index/untracked clean, `ready_count=1`,
  `queue_top_ready=NA-0654`.
- D-number: highest DECISIONS entry D-1276, D-1277 absent → this lane is
  **D-1277**.
- Target rev confirmed CURRENT at execution: fresh `git ls-remote
  https://github.com/QuantumShieldLabs/qsl-server.git main` (GitHub direct,
  NOT the stale qbuild mirror) → `3cc551a8d9cfd8f8f53d51e0b98d10a5dc62c944`
  — unmoved since drafting; the Phase-0 STOP condition did not trigger.
- ENG-0046 `Status: open` at ledger line 1568 before this lane.
- Live pin re-verified: Cargo.toml line 34 = `8e4ea278…`; lock stanza line
  1982 matches.
- Base `e8bf93cc` = the #1589 seating merge, NEXT_ACTIONS.md-only — no suite
  reshape since the NA-0649 baseline record.

## 3. The suite baseline, derived LIVE at base (pre-bump)

Because no per-set baseline artifact exists in repo truth (the NA-0649 record
is aggregate-only), the full suite was run ONCE AT BASE `e8bf93cc` BEFORE any
edit, both to calibrate the environment and to derive the per-set comparison
baseline (the NA-0651 method):

```
$ cargo test -p qsc -- --test-threads=3     # niced, base e8bf93cc, pre-bump
aggregate: 412 passed / 0 failed / 1 pre-existing-ignored across all 108
result sets; exit 0; zero FAILED lines, zero panics.
NA-0640 e2e within the run: 2 passed / 0 failed (115.97s) against 8e4ea278.
```

This equals the repo-truth NA-0649 baseline EXACTLY (412/0/1 × 108),
re-verified live. The normalized per-set baseline (108 lines, target ::
counts, timing-stripped) is preserved in the proof root.

## 4. The bump — and the resolver-drift finding (operator-ruled in-session)

THE RAIL EVENT: the drafted mechanical method — scoped `cargo update -p
qsl-server` — produced the correct rev swap PLUS five unrequested edits:
the `windows-sys` dependency edges of `errno`, `nu-ansi-term`, `quinn-udp`,
`rustix`, `tempfile` flipped 0.59.0 → 0.52.0/0.61.2 (Cargo.lock numstat 6/6).
The D590 zero-transitive rail fired and work STOPPED.

CONTROL (the decisive diagnostic, preserved in the proof root): with the
Cargo.toml REVERTED to the old rev, the same scoped command — reporting
"Locking 0 packages" — still rewrites the same five edges (5/5). The churn is
therefore PRE-EXISTING RESOLVER DRIFT between cargo 1.95.0 and the lock as
last written (NA-0643), fully independent of this bump. All five are
Windows-only `cfg(windows)` edges with no effect on any Linux build.

OPERATOR RULING (in-session, 2026-07-17, recorded verbatim in the response
file §6a): Option 1 — hand-apply the single qsl-server rev line in Cargo.lock
to match the toml; prove fail-closed with `cargo metadata --locked` (accepts
only a lock requiring zero re-resolution) plus the full head-side suite and
e2e. The lane delta is EXACTLY the rev advance (1/1 + 1/1), tighter than
drafted — the D590 zero-transitive rail holds UNAMENDED. The windows-sys
resolver drift stays OUT of the lane: surfaced as a finding for operator
disposition (a future hygiene micro-lane may take the churn deliberately);
deliberately NOT filed on the ledger (no ENG filing without the operator's
word).

Proof of the landed delta:

```
$ git diff --numstat
1  1  Cargo.lock
1  1  qsl/qsl-client/qsc/Cargo.toml
$ cargo metadata --locked --format-version=1   # exit 0, empty stderr
```

## 5. The proof (THE deliverable): the NA-0640 e2e green UNCHANGED

First invocation after the bump, zero retries, zero test edits:

```
$ cargo test -p qsc --test NA_0640_full_stack_e2e
   Compiling qsl-server v0.1.0 (…?rev=3cc551a8d9cfd8f8f53d51e0b98d10a5dc62c944#3cc551a8)
     Running tests/NA_0640_full_stack_e2e.rs

running 2 tests
test full_stack_message_and_attachment_round_trip_open_relay ... ok
test full_stack_message_round_trip_token_auth_relay ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 118.47s
```

Both scenarios pass against the server-info-bearing server: message
round-trip with plaintext byte-match + receipts, the >4 MiB attachment
byte-verified through the real qsl-attachments service and real relay, open
AND bearer-token auth, and the wrong-bearer NEGATIVE still rejected. The
compile line proves the new rev was built and exercised. The NA-0652 additive
guarantee (push/pull/ack byte-untouched) held END-TO-END, converted from
analysis into an artifact. The e2e does NOT probe `/v1/server-info` (declined
scope; the route's client consumption arrives with the GUI skeleton).

## 6. The head-side full suite + per-set multiset comparison

```
$ cargo test -p qsc -- --test-threads=3     # niced, post-bump
aggregate: 412 passed / 0 failed / 1 pre-existing-ignored across all 108 result sets; exit 0; zero FAILED lines, zero panics
NA-0640 e2e within the run: 2 passed / 0 failed (117.49s) against 3cc551a8
```

Per-set multiset comparison (timing-stripped, target :: counts, sorted;
the NA-0651 method): base (108 sets) vs head (108 sets) —
the normalized files are sha256-IDENTICAL (5ea8a2d2… both sides; empty diff) — all 108 sets identical in target, status, and counts.

## 7. Dev-edge-only proof (the NA-0640 discipline)

`cargo tree -p qsc -e normal` (the PRODUCTION dependency graph) captured
BEFORE and AFTER the bump:

```
sha256(before) = b8206499b4dd869ce9549bc15eaf115acff60ff3a6f67d138497868bb84666ff  (371 lines)
sha256(after)  = b8206499b4dd869ce9549bc15eaf115acff60ff3a6f67d138497868bb84666ff  (371 lines)
diff           = empty (byte-identical)
```

The shipped-binary graph is unchanged; the lock delta rides the dev edge
(`qsl_server` is a `[dev-dependencies]` entry used only by the e2e).

## 8. Validation

- Scope guard: `git status` delta = exactly `qsl/qsl-client/qsc/Cargo.toml` +
  `Cargo.lock` + the D590-allowed governance paths. Nothing else.
- No test-file change: the diff contains no `tests/**/*.rs` hunk.
- `git diff --check`: clean. `cargo metadata --locked`: exit 0.
- `cargo fmt --check`: the known 145 pre-existing diffs (the known pre-existing residue at base —
  recorded, not fixed; zero lane Rust).
- Audits: root cargo audit 386 crate dependencies / 0 vulnerabilities, exit 0; nested qsc fuzz cargo audit
  287 crate dependencies / 0 vulnerabilities, exit 0.
- goal-lint: run locally against the lane PR (synthesized event payload).
- ENG-0046 flipped DONE on the ledger citing the green e2e run + the new pin
  rev.

## 9. Limits (stated, per D590's classification rule)

A PASS asserts the CURRENT e2e scenarios pass against the server-info-bearing
server at `3cc551a8`. It does NOT exercise `/v1/server-info` itself — that
coverage arrives with the GUI skeleton (DOC-PROG-004 step 5, the route's
eventual real consumer). The surfaced windows-sys resolver drift is an
ENVIRONMENT observation, not a product claim. Claim boundary UNCHANGED.
