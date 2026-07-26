# NA-0678 testplan — relay invite slots, capability gate, durability (messaging epic Slice 1)

Directive: QSL-DIR-2026-07-26-614 (D614), APPROVED 2026-07-26.
Repo under test: `qsl-server` @ base `6ad078c2d857…`, merged as `131d63f4865544…`.
Result class asserted: **`RELAY_INVITE_SLOTS_PASS`**.

**Every expectation in this plan was written before its check ran** (standing rule).
The pre-registered expectations are preserved at
`/srv/qbuild/operator/relay/NA0678_EXPECTED_BEFORE_RUNNING.md` (census) and
`PHASE0_EXPECTED.md` (Phase 0), and the results are recorded against them in
`docs/governance/evidence/NA-0678_as_built.md`.

## 0. Method note — why two of these checks exist at all

Two of this lane's obligations could not be discharged by the tests the repository
already had, and the reason is the same in both cases: **the existing instrument could
not have returned a negative result for the property it was cited for.**

- `tests/na0642_durability_restart.rs` is the project's stated proof that "a 200 means
  fsynced". It SIGKILLs the relay and restarts it. SIGKILL destroys a **process**, not the
  OS page cache, so writes that reached the kernel survive it and `synchronous=FULL` is
  indistinguishable from `synchronous=OFF`. **Measured: that suite passes 3/3 with the
  pragma OFF.**
- The store's schema-version marker was written with `INSERT OR IGNORE`, a no-op on an
  existing key. **Measured: a `SCHEMA_VERSION=2` binary opened a v1 store, created its new
  table, and left `meta.schema_version = '1'`** — so D-0011's fail-closed downgrade guard
  had been inert since it was written.

Both are now discharged by instruments that move in both directions.

## 1. Invite lifecycle

| check | expected | result |
|---|---|---|
| create → redeem returns bundle + sig + ticket | 200, blobs byte-identical, ticket ≥32 chars | PASS |
| second redeem | **`ERR_INVITE_ALREADY_USED` (409), NOT `ERR_INVITE_NOT_FOUND`** | PASS |
| unknown invite_id | `ERR_INVITE_NOT_FOUND` (404) | PASS — **the negative half; without it the row above is vacuous** |
| expired invite | `ERR_INVITE_EXPIRED` (410), no bundle in the body | PASS |
| revoke, twice | 200 both times (idempotent), then redeem → `ERR_INVITE_REVOKED` | PASS |

The tombstone is a **contract requirement**: a deleted slot would report "never existed"
when the truth is "someone got here first", collapsing the interception signal the invite
design exists to surface.

## 2. Capability and revoke credential

| check | expected | result |
|---|---|---|
| **same-length** wrong capability | `ERR_INVITE_CAP_INVALID` (403), **and the slot still redeems afterwards** | PASS |
| **same-length** wrong revoke token | `ERR_INVITE_REVOKE_INVALID` (403), no mutation | PASS |

Same-length is the point: the pre-existing NA-0670 wrong-token test used a
different-length value, which `==` rejected on length before comparing a byte — it passed
against the buggy code. A different-length case proves nothing about the fold.

⚠ **Scope of the claim.** These prove the comparison returns the right **answer**. The
constant-time property is **structural and read-verified** (`ct_eq_secret` folds over a
fixed 32-byte digest with no data-dependent early return). **No timing measurement was run
and none is claimed.**

## 3. Atomicity

12 concurrent redemptions of one slot on a multi-thread runtime → **exactly 1 OK, 11
`ERR_INVITE_ALREADY_USED`**. PASS. The compare-and-set re-asserts ACTIVE in the WHERE
clause, so a loser updates zero rows; the single mutex-wrapped connection makes this exact
rather than probable.

## 4. Durability (the O2 obligation) — both arms

| arm | expected | result |
|---|---|---|
| shipped config, 0 creates | 0 create-attributable fsyncs | **0** |
| shipped config, 5 creates | ≥5 fsyncs | **5** |
| ordering | fsync completes before the `HTTP/1.1 200 OK` write | **observed** |
| `synchronous=OFF` control (built outside the repo) | **0** fsyncs for 5 creates | **0** |

The 0-create arm is what makes the 5-create arm meaningful — the count is attributable to
the creates, not to startup. The negative arm is what makes the whole instrument
non-vacuous.

**Skip discipline:** where `strace` is unavailable the test prints a SKIP naming the tool,
the property not examined, and where the coverage is discharged instead. A silent skip is
a vacuous pass.

`tests/na0642_durability_restart.rs` is **kept unchanged** — it is valid for process-crash
durability. Only its header comment was corrected.

## 5. Schema-version guard (positive AND negative)

| check | expected | result |
|---|---|---|
| fresh store | marker reads `2` | PASS |
| store built to look pre-NA-0678 | **advances to `2`**, gains `invites` | PASS |
| store marked `'99'` | **refused** with `ERR_STORE_VERSION` | PASS |

The third is the control: without it the second could pass against an implementation that
simply stopped checking.

## 6. Non-regression — the C3 guarantee, the most important row here

| check | expected | result |
|---|---|---|
| full pre-existing suite | unchanged and green | **29 suites / 129 tests, exit 0** |
| `qsl_attachments_integration_contract` | green | PASS (1) |
| `na0642_backward_compat` | green | PASS (4) |
| push/pull to a route that is not a slot | no ticket required, behaviour identical | PASS |

Gating the existing mailbox would have broken the shipped qsc client, the spine's pinned
in-process e2e, the attachments contract and the live relay **before Slice 2 exists to
replace them** — inverting the epic's own dependency-chain safety property. The capability
gates only the stranger-ingress.

## 7. Auth, opacity, advertisement, abuse controls, route set

| check | expected | result |
|---|---|---|
| all three routes on a bearer relay, unauthenticated | 401 **plain `ERR_UNAUTHORIZED`**, not the server-info probe body | PASS |
| opacity | 256 descending bytes + a 4-byte non-UTF-8 sig round-trip byte-identical; absent from logs; `channel_id=` present | PASS |
| server-info | `invite_v1` appended, pre-existing entries unmoved, `invite` block present; **both EXACT guards moved in the same commit** | PASS |
| create-rate bucket exhausted | `ERR_RATE_LIMITED` **and no slot stored** | PASS |
| slot cap full | `ERR_INVITE_CAP_FULL` and **both earlier slots survive** | PASS |
| oversize bundle | `ERR_INVITE_TOO_LARGE` (413) | PASS |
| `/v1/invite/mint` and two other spellings | **404** | PASS |

The cap and the bucket are **not substitutes**: the cap bounds storage, the bucket bounds
denial. `ERR_INVITE_CAP_FULL` never evicts — an eviction path would let an attacker delete
other people's invites.

## 8. Gates

`rust` PASS · `public-safety` PASS · `advisories` PASS on `87cb7a23`.
Locally before push: clippy `--all-targets -D warnings` clean; `fmt --check` clean;
infra-literal tree clean (76 files, 15,254 lines examined) and diff clean (13 files, 2,258
lines examined); **`Cargo.toml`/`Cargo.lock` diff EMPTY**.

⚠ `RUST_TEST_THREADS=2` is required on a many-core host — `tests::logs_do_not_contain_raw_channel`
flakes at full parallelism (ENG-0065, pre-existing, reproduced again).

## 9. What this PASS does NOT assert

Nothing about client behaviour, the handshake crypto, invite parsing, or messaging end to
end — those are Slices 2 and 3. No timing claim. No public, production,
security-complete or bug-free claim.
