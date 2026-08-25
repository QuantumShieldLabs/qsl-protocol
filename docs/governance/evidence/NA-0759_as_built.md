# NA-0759 — AS BUILT — THE macOS FIXTURE RACE: A RED MAIN BLAMED ON THE REPAIR THAT HAD JUST MERGED

Spine decision **D-1400** · class **`MACOS_FIXTURE_RACE_NOT_A_PRODUCT_REGRESSION`**
Diagnosis order `ORDER_macos_na0742_regression_20260824.md` (sha256 `ae2544bf…`, 3967 B, 444)
Ruling `RULING_MACOSREG_STOP001_20260825.md` (sha256 `c125a6a5…`, 5917 B, 444), ruled from
`STOP_MACOSREG_001_20260825T011937Z.md` (sha256 `895a928b…`, 942 l / 66372 B, 444)
Base: qsl-protocol `0b9d6967948c2fcf799cb817aeee55d5095835aa`, re-derived bare and unpiped at the
NAMED github remote (`origin` in these seats is the local mirror at `/srv/qbuild/mirrors/` and was
never used as a source of truth).

## 1. What shipped

**One test fixture, three edits, 41 insertions / 3 deletions in a single file.** Zero product
bytes, zero `.github` bytes, zero desktop bytes, and no test weakened, skipped, deleted or
scope-reclassified.

`qsl/qsl-client/qsc/tests/na0742_invite_finish_scan_producer_acks.rs`:

- **F1** — `let _ = stream.set_nonblocking(false);` immediately after `listener.accept()`. The
  `set_nonblocking(true)` at `:741` is for the **listener**; the accepted socket may inherit that
  flag on BSD-derived kernels, and on a non-blocking socket the `set_read_timeout(20 s)` on the
  next line is **inert**.
- **F2** — `read_head` adopts the house pattern from `tests/common/mod.rs:898-925`: tolerate
  `Interrupted | WouldBlock | TimedOut` and bound the wait with a deadline. The old body answered
  **every** `Err` with `None`, and the caller answers `None` by dropping the connection with **no
  response written at all**.
- **F4 (FILE-SCOPED by ruling)** — `QSC_RELAY_PULL_DIAGNOSTIC=redacted` in this file's own `qsc()`
  builder, so a future opaque pull failure names its own reason.

**F3 is SUBSUMED** by F2's deadline behaviour and was not separately taken. **F5** (do nothing) was
refused as a terminal state. **F6** (manifest exclusion / scope reclassification) was refused by the
seat before it reached the chair, and the refusal was ratified.

## 2. The finding, and why it is structural rather than probabilistic

The macOS push suite went red at `0b9d6967` on the **pre-existing** target
`na0742_invite_finish_scan_producer_acks` — `t5p_the_poll_tolerates_a_redelivered_already_processed_frame`,
failing its antecedent `proxy.faulted() > 0` (`:1044`) with
`event=handshake_recv code=relay_inbox_pull_failed ok=false` — one merge after `D-1399`'s ENG-0239
repair. The diagnosis order's S0 named that repair as the cause. **It is not the cause.**

| fact | measurement |
|---|---|
| the poll's exit | `relay_inbox_pull` at `handshake/mod.rs:1722`; `return Err(code)` at **`:1727`**, unconditional |
| the two NA-0757-edited sites | `qsp_session_store` at **`:1928`** and **`:2167`** — **201 and 440 lines below the return** |
| the test file across the merge | **byte-identical**, sha256 `5b136eee05f455dc0065814eeb3642b47a24481b49fde8bd881c625e3708c2f2` at both `f98af5cc` and `0b9d6967` |
| shard membership | macOS shard **1** at both shas; NA-0757's new target went to shard **0**; **35** targets in both runs |
| the same target on Linux at the same sha | **ok, 12/12, 513.96 s** |
| the same target on macOS at the previous main | **ok, 12/12, 660.53 s** |
| the marker that fired | `relay_inbox_pull_failed` — **not** the augmentation's `handshake_session_store_failed store_code=<wire name>` |

⇒ The failing execution **never reached** either edited line. `D-1399` is exonerated.

⚠ **The order's S0 was a one-sample inference** — "previous main green, this main red" — read as
causation against what measures as a **1-in-12** race. Recorded as a Director false premise at the
ruling, in that chair's own words.

## 3. The defect

`start_ack_fault_proxy` is a hand-rolled, **single-connection-serial** TCP proxy:

```
741    listener.set_nonblocking(true).expect("proxy nonblocking");     // the LISTENER
753    let mut stream = match listener.accept() { … };                  // flag never cleared
762    let Some((head, _)) = read_head(&mut stream) else { continue; }; // silent drop: writes NOTHING
```

and inside `read_head` (`:720-737` at base):

```
727            Err(_) => return None,      // WouldBlock lands HERE, undistinguished
```

A dropped connection reaches the client as a bare transport error, which
`transport/mod.rs:2229-2240` collapses to `relay_inbox_pull_failed` for **every** non-TLS send
failure — and `faulted()` is never incremented, because the request was never classified as an ack.
**Both observed symptoms, from one cause.**

## 4. The blast radius, measured from the consumer

`git grep set_nonblocking -- qsl/` returns exactly five listener fixtures:

| site | per-conn thread? | read tolerates `WouldBlock`? | writes a response on read failure? | exposure |
|---|---|---|---|---|
| `tests/common/mod.rs:595` — the shared relay harness | yes | **YES** (`:898-925`, deadline-bounded) | 400 after deadline | **immune** |
| `tests/relay_pull_diagnostics.rs:65` | yes | no (`Err(_) => break`) | yes, fixed status | low |
| `tests/relay_push_diagnostics.rs:45` | yes | no (`Err(_) => break`) | yes, fixed status | low |
| `tests/relay_auth_header.rs:329` | yes | no (`Err(_) => break`) | yes | low |
| **`tests/na0742…rs:741`** | **NO — serial** | **no (`Err(_) => return None`)** | **NO — writes nothing** | **the failure** |

⛳ The correct pattern was already in the tree, in the very harness this test uses for its relay.
`read_head` is file-local (defined `:720`, called once `:762`), so F2's blast radius is exactly this
file.

## 5. Evidence — both arms, and the leg that is NOT measured

**Probe 1 (Linux, `probe_accept_inherit.rs`).** Prediction written before the run: the accepted
socket is blocking.

```
listener  O_NONBLOCK = true
accepted  O_NONBLOCK = false
read -> Err(WouldBlock) after 306.190397ms  => read_head returns None
```

**HIT**, plus a bonus the prediction did not contain: a `SO_RCVTIMEO` expiry also surfaces as
`WouldBlock`, so the silent-drop path exists on both platforms.

**Probe 2 (`probe_mech.rs`), only the accepted socket's blocking mode varying:**

```
ARM A (Linux-as-is, control) : accepted_nonblocking=false client_got=56 bytes -> HTTP response
ARM B (accept inherits NB)   : accepted_nonblocking=true  client_got=0 bytes  -> NO RESPONSE
```

⚠ **CLAIM BOUNDARY.** That macOS's `accept()` inherits the flag is **NOT measured** — no macOS host
was reachable from the seat. Two other routes to the identical marker stay live and are
indistinguishable in the banked log: any non-TLS send error, and any unmapped HTTP status including
the **502 this proxy itself emits** on upstream failure. The repair narrows all three; it is not
claimed to have discriminated among them. And because 11 of the last 12 executions were green
**without** the repair, a green validation dispatch proves that the repair does not regress the
shard — **not** that it caused the green.

## 6. The denominator correction

`t5p` was introduced 2026-08-17 in `403432ce`. Of the **24** green macOS `push` runs since, only
**11** actually executed `macos-qsc-shard-1`; the other 13 skipped the shards on a docs-only
classification. **12 executions: 11 green, 1 red.** Both control arms were run and they differ — a
skipped matrix job reports the **unexpanded** name `macos-qsc-shard-${{ matrix.shard }}`, an
executed one reports `macos-qsc-shard-0…4` individually.

**A run list counts RUNS, not EXECUTIONS.**

## 7. `ENG-0244` — the fourth finding

On an ordinary pull request this workflow skips its shards **and its named rollup check**, and the
run still concludes **`success`**. Measured on NA-0757's own PR run **32777855461** (head
`db7ebac9`): both `macos-qsc-shard-${{ matrix.shard }}` and `macos-qsc-sharded-suite` **skipped**,
conclusion **`success`**, elapsed **30 seconds** against ~61 minutes for a real execution.
**NA-0757 merged under a macOS check that executed zero tests.** Filed OPEN; the cure touches
`.github/**` and is the operator's own lane. A companion measurement is owed and takes ten seconds:
whether `macos-qsc-sharded-suite` is branch-protection-REQUIRED today.

**ADDENDUM (2026-08-25, the branch-update touch) — A SECOND INSTANCE, MEASURED ON THE PR THAT FILES
IT.** PR #1791's own macOS `pull_request` run **32813603284** skipped both `macos-qsc-shard-${{ matrix.shard }}`
and the named rollup `macos-qsc-sharded-suite` and concluded **`success` in 54 seconds**, against
**64 minutes** for the `workflow_dispatch` arm on the identical commit.

**AND THE COMPANION MEASUREMENT IS NO LONGER OWED — it is taken, and its answer is sharper than the
question.** `gh api repos/QuantumShieldLabs/qsl-protocol/branches/main/protection` returns
**`strict: true`** (branches must be up to date — this is the `NA-0759` STOP 001 §15(7) unknown, and
it is what refused the first merge attempt) and a **15-context** required list:
`ci-4a`, `ci-4b`, `ci-4c`, `ci-4d`, `ci-4d-dur`, `demo-cli-build`, `demo-cli-smoke`,
`formal-scka-model`, `goal-lint`, `metadata-conformance-smoke`, `suite2-vectors`, `CodeQL`,
`macos-qsc-qshield-build`, `infra-literal-scan`, `public-safety`.
⇒ **`macos-qsc-sharded-suite` is NOT a required context by name — and it gates `main` anyway**,
because **`public-safety` IS required** and `public-safety` polls the push suites and fails on their
failure (measured at STOP 001 §10: `CHECK macos-qsc-sharded-suite: status=completed conclusion=failure`).
**A check can gate the branch without appearing in the list that says what gates the branch** — which
is the same shape as `ENG-0244` itself: the thing that decides is not the thing the reader is shown.

## 8. Gates measured in this seat

| gate | result |
|---|---|
| na0742 **baseline**, before any edit (Linux, `--test-threads=1`) | **12 passed / 0 failed**, 750.24 s, exit 0 |
| na0742 **post-edit** | recorded at STOP 2 |
| `cargo fmt --all -- --check` | ⚠ **not a gate in this repo** — no workflow runs `cargo fmt` or `cargo clippy` (positive control: `cargo test` appears 5× in `.github/workflows/`, `cargo fmt` and `cargo clippy` 0×). Locally it flags **309** files at the **untouched base** and **309** with the edit, **0 of them mine** — pre-existing rustfmt-version noise, measured on both arms |
| `qsc_shard_check.py --verify-log` | unaffected: it asserts **which targets ran**, by name, and this lane changes no manifest membership |
| macOS validation | one `workflow_dispatch` on this branch, ratchet line quoted, at STOP 2 |

## 9. Ids, re-derived at the edit, every space with both controls

`NA-0759` **0** · `D-1400` **0** (positive control `D-1399` = **14**) · `ENG-0243` **0** ·
`ENG-0244` **0** (positive control `ENG-0242` = **5**). ⚠ `ENG-0299` returns **1** hit and it is a
**MENTION** — NA-0743's own negative control in prose — the WF-0087 plant hazard again.
protocol has **0 open PRs**; the desktop's #37 is merged.

⚠⚠ **`NA-0758` is deliberately skipped and unavailable**: it is a live lane (the public-docs audit)
that has landed **zero bytes**, so a repo-derived counter cannot see it and would have re-issued it.
`HIGHEST_NA` advances `0757 → 0759`, naming the newest **declared** block per `ENG-0230`'s property.

## 10. What this landing does NOT carry

The banked close-out separately owes **prediction rows 207-233**, the seal-heading sweep filing,
three WF candidates and the operator's stale-branch sweep. **None is in this landing** — the
ruling's edit set is closed and names only rows **217-231** for the prediction ledger.
⚠ Those drafted numbers are now **twice stale**: `207-216` were consumed by `D-1399`'s own merge and
`217-231` are consumed here, so `STOP_NA0756_006`'s `207-226` and `STOP_NA0756_008`'s `227-233` must
be **re-derived to `232-258`** when they land. `STOP_NA0756_008:118` already carries the governing
instruction — *"if they are not, re-derive."*
