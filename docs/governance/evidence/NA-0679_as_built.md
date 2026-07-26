# NA-0679 — AS-BUILT: the qsl-server dev-dependency pin bump (messaging epic, sequencing step 1.5)

Lane: **NA-0679**, spine decision **D-1312**, result class **`QSL_SERVER_PIN_BUMP_SLICE1_PASS`**.
Seat: `/srv/qbuild/work/NA-0678/qsl-protocol`. Work was prepared on base
`7def702c18a9abd0b93a3fa48409af8d7a8ca9a2` (spine main after the NA-0678 closeout) and
**rebased onto `de55b404`** after the parallel polish lane's enqueue PR #1659 merged, so the
branch carries #1659's `HIGHEST_NA=0680` rather than overwriting it.

⚠ **HIGHEST_NA is deliberately NOT lowered to 0679.** NA-#### is assigned in creation order and
never renumbered; 0679 was reserved by this lane before NA-0680 was enqueued, and #1659's own
prior-comment records that reservation from the other side. Only `HIGHEST_D` moves (1311 → 1312).
The rebase was verified not to disturb the adjacent `qsl-attachments` pin by diffing that line
against `origin/main` directly — a `grep -c` over the diff had counted it as changed when it was
only context.

Pin: `3cc551a8d9cfd8f8f53d51e0b98d10a5dc62c944` → **`131d63f4865544addd2784c305970b21ddbeb69c`**
(qsl-server main = the merged Slice-1 commit, verified live via the API, not from memory).

Expectations for every check below were written first, at
`/tmp/.../PIN_EXPECTED.md` and reproduced in §0.

---

## 0. Why this is its own lane

A pin bump is a deliberate, separate step (the ENG-0041 / NA-0640 discipline, restated by D578
and D588 which each forbade the bump in-lane). It matters more than usual this time: the spine's
`qsc` test suite is the **only** consumer of qsl-server outside the qsl-server repository, so this
bump is **the first external test of Slice 1's central promise** — that `/v1/push`, `/v1/pull` and
`/v1/pull/ack` are unchanged for every route the invite system did not create. The client that
proves it knows nothing about invites, which is exactly what makes the proof worth having.

## 1. The delta — exactly the rev advance

```
qsl/qsl-client/qsc/Cargo.toml   1 line  (rev = …)
Cargo.lock                      1 line  (source = git+…?rev=X#X)
4 changed lines total
```

## 2. ⚠ The resolver-drift control — reproduced, and larger than recorded

Memory (NA-0654) said a scoped `cargo update` also flips five Windows-only `windows-sys` edges.
**That was not taken on trust.** The scoped update was run on a **throwaway copy**, never on the
seat:

```
cargo update -p qsl-server
  Locking 1 package to latest compatible version          <- cargo's own summary
  Adding   qsl-server …#131d63f4
  Removing qsl-server …#3cc551a8

actual lock delta: 14 changed lines across SEVEN packages
  windows-sys 0.61.2 -> 0.52.0      (x3)
  windows-sys 0.61.2 -> 0.59.0
  windows-sys 0.60.2 -> 0.52.0
  getrandom   0.4.2  -> 0.3.4       <- NEW since NA-0654
  qsl-server  3cc551a8 -> 131d63f4  <- the only legitimate change
```

Two things worth recording:

1. **The drift has grown.** NA-0654 documented five `windows-sys` edges; there is now also a
   `getrandom` edge. A method ruled on July 17 was re-validated rather than assumed on July 26,
   and the re-validation found the situation had changed.
2. **⚠ Cargo's summary line said `Locking 1 package` while changing seven.** The delta was found
   by diffing the lockfile, not by reading the message. This is the same defect class as reading a
   check mark instead of the log.

**Method used, per the operator's NA-0654 ruling:** hand-apply the single lock `source` line.

## 3. Lock integrity and dev-edge proof

```
cargo metadata --locked --format-version 1      rc=0
   (a hand-applied line that does not satisfy the manifest fails HERE)
lock delta after metadata ran                   still 4 lines — no re-resolution occurred

cargo tree -p qsc -e normal   before  sha256 37d638c07e6a358c81639197c05f7eb2c761c488a196458a67f74bc5babc8218
                              after   sha256 37d638c07e6a358c81639197c05f7eb2c761c488a196458a67f74bc5babc8218
                              BYTE-IDENTICAL — the bump did not reach the production graph
```

## 4. The coverage this bridge exists to protect

⚠ These suites are **push-only** (`qsc-linux-full-suite` skips on pull requests), so **CI green on
the bump PR proves nothing about them.** They were run locally; that run is the evidence.

Three test files consume `qsl_server` — not one, which corrects an earlier statement of mine that
referred only to "the e2e":

| suite | expected | result |
|---|---|---|
| `NA_0640_full_stack_e2e` | passes UNCHANGED | **2 passed / 0 failed, 180.23s** |
| `NA_0644_ack_client` | passes UNCHANGED | **6 passed / 0 failed, 236.96s** |
| `NA_0671_vault_kdf_cost` | compiles | **COMPILED; DID NOT RUN** — `#[ignore]`d by design, a release-only measurement harness |
| full `cargo test -p qsc` | green | **446 passed / 0 failed / 2 ignored, 112 sets, exit 0** |

The compile line proves which rev was actually built:
```
qsl-server v0.1.0 (https://github.com/QuantumShieldLabs/qsl-server.git?rev=131d63f4…#131d63f4
```

**On `NA_0671_vault_kdf_cost`:** it is counted here as a COMPILE proof, not a pass. What it
establishes is that the library constructors it calls still exist. Stating that rather than
folding it into a test count, because "3/3 suites pass" would be false.

**ZERO test-file and ZERO source edits.** The harness (`tests/common/mod.rs`) builds the relay
in-process through `AppState::new_with_auth_and_controls` and `new_with_auth_controls_and_store`,
both preserved by Slice 1 — verified by reading the harness before the bump, not after it failed.

## 5. Full suite

```
cargo test -p qsc   ->  EXIT 0
112 result sets · 446 passed / 0 failed / 2 ignored   (1h20m wall)
```

The two ignored are the pre-existing `#[ignore]`d harnesses, one of which is
`NA_0671_vault_kdf_cost` (§4).

**On the base-side run:** the NA-0654 precedent ran the full suite on BOTH sides of the bump, to
make a head-side failure attributable. It was not run here **because there was nothing to
attribute** — the head side is green, and the bump is proven dev-edge-only by an identical
`cargo tree -e normal`. Had the head side gone red, the base-side run would have been the next
step. Recording the reasoning so the omission is a decision rather than a gap.

## 5b. ⚠ Scope of the "external confirmation" claim, stated precisely

Only **three** of the 109 qsc test files consume `qsl_server`. The other ~106 drive
`start_inbox_server` — a stand-in relay implemented inside `tests/common/mod.rs` — and therefore
say **nothing** about the bumped dependency. The full-suite green above is a regression check on
qsc generally, **not** additional evidence for the bump.

So the claim "Slice 1's contract held, confirmed externally" rests on `NA_0640_full_stack_e2e` and
`NA_0644_ack_client`, and nothing wider. That is still a real proof — two isolated qsc clients, the
real relay, push/pull/ack including the lease contract and a bearer-token negative — but it is
narrower than "the qsc suite passes against the new relay", which would be the easy and wrong way
to summarise it.

⚠ Worth carrying to Slice 2: **there is no mock relay in this tree.** Everything is either the real
qsl-server or `start_inbox_server`, which is a **second implementation of the same wire contract**
that nothing currently checks for agreement with the first. Same shape of risk as the hand-rolled
base64 codec, and it will matter more once the invite routes have a client.

## 6. What this PASS does not assert

Nothing about the invite subsystem's client side — no qsc code calls `/v1/invite/*` and none was
added. That is Slice 2. No public, production, security-complete or bug-free claim.
