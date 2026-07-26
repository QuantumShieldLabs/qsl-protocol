# NA-0678 — AS-BUILT (messaging epic Slice 1, the relay's invite-slot subsystem)

Implementation: **qsl-server PR #66 MERGED as `131d63f4865544addd2784c305970b21ddbeb69c`**
from head `87cb7a239918b229a753e3619f1670e817c46d81`. Repo-local decision **D-0016**.
Directive **QSL-DIR-2026-07-26-614 (D614)**, APPROVED 2026-07-26, sha256
`383dc36646907c03f3979c56c211efdb8abac82216a52cd7c0455aa501016269`, 600 lines.

**CI on the exact reviewed commit: `rust` PASS · `public-safety` PASS · `advisories` PASS.**
The `rust` pass IS evidence here — unlike the docs_only PRs around it, #66 changed
`src/`, so the check compiled and ran the code.

Seat: `/srv/qbuild/work/NA-0678/qsl-server`, base `6ad078c2d857…` (== origin/main at
Phase 0, verified). Commit `87cb7a239918b229a753e3619f1670e817c46d81`.
Identity GH007 (`238594419+Tebbens4832@users.noreply.github.com`) author and committer,
trailers empty, verified on the object.

---

## Phase 0 — six checks, expectations written first

| | expected | actual |
|---|---|---|
| promotion landed | #1657 MERGED, `READY=NA-0678`, READY count 1 | ✅ merge `3efbea73`, count **1** |
| base unmoved | `6ad078c…` exactly | ✅ exact |
| §4a design corrected | zero LIVE relay-mint statements | ❌ at Phase 0 (in transit) → **✅ re-verified at 12:14 before the PR: 0** |
| anchors | six src anchors + two guards | ✅ verified by reading |
| baseline | exit 0, all suites ok | ✅ EXIT=0, 26/26 |
| seat | clean, identity pinned, own target dir | ✅ `head_equals_origin_main=yes`, `worktree_clean=yes` |

The §4a miss was reported rather than walked past: D614 §8.5 makes executing against
the uncorrected authority a STOP, and the judgement recorded at the time was that the
STOP's *premise* no longer held — the binding construction lives in D614 §4 F1 and §2b,
and neither stale line (§1 wire format, §5.1 create flow) governs a byte the relay
writes, since the relay never parses the invite code. The re-verification was owed and
was performed; it passed.

## F4 — the durability obligation, both arms

The claim: an accepted `POST /v1/invite/create` is fsynced **before** its 200 reaches
the socket.

**Why a restart test could not discharge it** (measured during the D614 census, and the
reason this instrument exists): `tests/na0642_durability_restart.rs` passes **3 passed /
0 failed with `PRAGMA synchronous=OFF`**. SIGKILL destroys a *process*, not the OS page
cache, so `FULL` and `OFF` are indistinguishable to any process-kill test.

**Positive arm** — shipped config, in-repo instrument (`tests/na0678_invite_durability.rs`):
```
na0678 durability: EXAMINED 0-create and 5-create runs under strace;
fsync delta 0 and 5; fsync-before-200 ordering observed.
test result: ok. 1 passed; 0 failed
```
The 0-create arm returning **0** is what makes the 5-create arm mean something: the count
is attributable to the creates, not to startup.

**Negative arm** — a `synchronous=OFF` control built OUTSIDE the repository (the tree
stays clean; a test binary cannot recompile the relay):
```
NEGATIVE ARM (synchronous=OFF): before=4 after=4 delta=0
```
Five accepted creates, **zero** fsyncs. The instrument moves in both directions, so its
green carries information.

**Skip discipline:** where `strace` is unavailable the test prints a SKIP naming the tool,
the property, and where the coverage is discharged instead. A silent skip would be a
vacuous pass — indistinguishable from a passing gate.

## F5 — the schema-version repair, with a negative control

Measured defect: a `SCHEMA_VERSION=2` binary opened a v1 store, created its new table,
and left `meta.schema_version = '1'` (`INSERT OR IGNORE` is a no-op on an existing key).
D-0011's fail-closed downgrade guard had therefore been inert since it was written.

`tests/na0678_schema_version.rs`, 3 passed:
- a fresh store records `2`;
- a store built to look pre-NA-0678 (old tables, marker `'1'`) **advances to `2`** and
  gains the `invites` table;
- a store marked `'99'` is **refused** with `ERR_STORE_VERSION` — the negative control,
  without which the first test could pass against an implementation that simply stopped
  checking.

## Acceptance (D614 §5)

| requirement | evidence |
|---|---|
| lifecycle; 2nd redeem `ALREADY_USED` **not** `NOT_FOUND` | `lifecycle_…` + `unknown_invite_is_not_found_not_already_used` (the negative half — proves the two causes really differ) |
| expiry kills pre-bundle | `expired_invite_dies_and_returns_no_bundle` |
| revoke idempotent; wrong token no mutation | `revoke_kills_the_slot_and_is_idempotent`, `same_length_wrong_revoke_token_…` |
| **same-length** wrong capability rejects, no mutation | `same_length_wrong_capability_rejects_with_no_mutation` — the D-0014 lesson; a different-length value proves nothing about the fold |
| atomic consume | `concurrent_redemption_yields_exactly_one_winner` — 12 concurrent, 1 OK / 11 CONFLICT, multi-thread runtime |
| durability both arms + ordering | above |
| **C3 non-regression** | full suite green, incl. `qsl_attachments_integration_contract` (1) and `na0642_backward_compat` (4); plus `non_slot_routes_are_completely_unaffected` |
| both auth modes, plain `ERR_UNAUTHORIZED` | `every_invite_route_is_gated_on_a_bearer_relay` — asserts the new routes do **not** adopt the server-info probe body (DOC-SRV-006 rule 4) |
| server-info guards in lockstep | `tests/na0652_server_info.rs` both EXACT guards moved in the same commit; `server_info_advertises_invite_v1_additively` |
| opacity | `bundle_is_opaque_bytes_in_bytes_out_and_never_logged` — 256 descending bytes, not valid anything; byte-identical round trip; absent from logs |
| create-rate + cap, not substitutes | `create_rate_bucket_exhausts_and_creates_no_slot` (and no slot stored), `slot_cap_rejects_and_never_evicts` (both earlier slots survive) |
| no mint route | `there_is_no_mint_route` — asserts 404 on three plausible spellings |

**Constant-time is claimed as STRUCTURAL and read-verified only.** The tests prove the
comparison returns the right ANSWER; no timing measurement was run and none is claimed.

## Gates, run locally before pushing

```
cargo test -q (RUST_TEST_THREADS=2)      EXIT=0, 29 suites, 129 tests
cargo clippy --all-targets -- -D warnings EXIT=0
cargo fmt --all -- --check                EXIT=0
infra_literal_scan --mode tree            clean (76 files, 15254 lines examined)
infra_literal_scan --mode diff            clean (13 files, 2258 lines examined)
git diff Cargo.toml Cargo.lock            EMPTY (0 lines)
```
⚠ `RUST_TEST_THREADS=2` is required on a many-core host: `tests::logs_do_not_contain_raw_channel`
flakes at full parallelism (ENG-0065, pre-existing, reproduced again this lane).

## Scope

Touched exactly D614 §7's MAY list: `src/{lib,store,main}.rs`, `tests/**` (three new files
+ the two server-info guards + the na0642 header comment), `docs/server/DOC-SRV-007_*`
(new), `README.md`, `packaging/systemd/relay.env.example`, `DECISIONS.md` (D-0016 + the
OBS-BM D-0015 correction), `TRACEABILITY.md` (this lane's row only). No `.github/**`, no
`Cargo.*`, no pin, no qsc or desktop code, no branch protection.
