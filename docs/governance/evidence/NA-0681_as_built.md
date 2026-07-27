# NA-0681 — AS-BUILT EVIDENCE (MESSAGING EPIC SLICE 2)

Directive **QSL-DIR-2026-07-27-616 (D616, amended)**, sha256
`7c1dd5754393c129ced67df616692f5812f44160dabbbb1509232f3c4b8bccd4`, 798 lines.
Seat `/srv/qbuild/work/NA-0681/qsl-protocol`, base `71004202…` (== `origin/main` at Phase 0,
verified). Code PR **#1666**, merged `2155f45981b1410bcf42e5d5855792b0e5784fd5`, three commits
(`0184209a` implementation · `4d1cec28` the eighth §2m file · `8dd6fe2a` D-1315 +
TRACEABILITY). Identity GH007 author **and** committer, trailers empty, verified on each object.

⚠ **`docs/governance/evidence/` is gitignored** — this file needs `git add -f`, and the commit
must be re-checked after every amend.

---

## Phase 0 — nine checks, expectations written first

| | expected | actual |
|---|---|---|
| promotion landed, `READY_COUNT == 1` via the queue helper (WF-0026, not a grep) | 1, and it is NA-0681 | ✅ 1, NA-0681 |
| seat HEAD == `origin/main`, worktree/index/untracked clean | exact, clean | ✅ `71004202`, clean |
| `D-1315`/`D-1316` absent, `D-1314` present once | 0 / 0 / 1 | ✅ |
| identity | GH007, author and committer | ✅ |
| **the §8.4 anchor stop** | fires, but its premise void | ⚠ see below |
| §4a doc corrections still present | 2 in DESIGN, 1 in LANE_INTENT | ✅ |
| pin + `git diff Cargo.toml Cargo.lock` | `131d63f4…`, 0 lines | ✅ |
| `ContactRecord` lacks `deny_unknown_fields` | absent | ✅ 0 occurrences |
| baseline suite | exit 0 | ❌ **stalled — see below** |

### ⚠ The anchor stop fired, and its premise was disproved by measurement rather than argument

D616 §8.4: *if spine main ≠ `7242b85d`, every line number is suspect — STOP and re-anchor.*
Main **was** `71004202` (this lane's own two governance merges), so the stop fired. The reason
it exists — that source line numbers become suspect — did not hold, but *knowing why main moved*
is exactly the confidence the condition overrides. Measured instead:

```
git diff --stat 7242b85d..HEAD  ->  NEXT_ACTIONS.md | 45 ++-   (1 file)
qsc files changed               ->  0
```

Then twelve anchors were spot-verified individually (`Cargo.toml:9`/`:45`, `lib.rs:3`/`:32`,
`identity:17`, `route.rs:21`/`:24`, `handshake:496`/`:769`, `store:198`, `protection:141`,
`contacts:890`) — all exact.

### ⚠ The baseline never completed, and the reason became a product finding

Attempt 1 stalled at 20/78 on `aws_file_medium_boundary_na0192a`: **17 minutes elapsed, 2
seconds of CPU**, `State: S`, `wchan: futex_do_wait`, with a spawned
`qsc receive --relay http://127.0.0.1:<port>` child whose **PID changed between inspections** —
a retry loop against a relay that never answers. Its binaries were built at **00:22:28** and
this lane's first source edit was at **00:25:50**, so the behaviour is **at base**, provably not
caused by Slice 2. Filed as **ENG-0079**.

**Consequence, recorded rather than smoothed over:** attempt 2 recompiled and therefore ran at
HEAD, so **this lane has no completed base baseline**. Attribution was done per-suite against a
stashed base instead (operator-accepted). The sequencing error — editing while the baseline ran
— is now a standing rule.

---

## The work, and what the census changed

**Every real risk in this slice is an ENCODING risk.** Three of the four were invisible until
the relay's *source* was read rather than its contract, and each fails as an attack or as
*success* rather than as an honest error. §2a defines each exactly once:

| encoding | the hazard it closes |
|---|---|
| `wire_id` — lowercase hex, 32 chars | `invite_id` **is** the relay route token; the relay finds a slot by hashing that header **string**. A mismatch makes the lookup miss, so the push is accepted into an unread route and the relay answers **200 OK**. Chosen by measurement: 32 chars sits inside `route_token_is_valid`'s 22–128 window, and hex is the shipped precedent. |
| `cap_hash_hex` — SHA-256 over the wire **string** | The relay stores the uploaded hash verbatim without validating it, so a case or rendering mismatch returns `ERR_INVITE_CAP_INVALID` — an encoding bug wearing the costume of a wrong capability. |
| `canonical_bundle_bytes` — an explicit layout | **Never** a re-serialization of the on-disk `IdentityPublicRecord`: `serde_json` offers no byte-stability contract, and the Phase-0 instrument proved drift can produce a **clean decode to the wrong bytes**. |
| the `QSLI-1-` payload codec | `URL_SAFE_NO_PAD`. The commitment is over **decoded bytes**; the relay accepts padded input and emits unpadded, so a string comparison passes every same-implementation test and breaks against a relay that re-encodes. |

**F1 — wrapping, not a protocol change.** `hs_encode_init` is fixed-length positional with no
route-token field, so DESIGN P2's "versioned TLV" described a state that did not exist — ruled a
Director error and corrected in both authorities. The invite handshake is a `QSLH-1` envelope
carrying A1 **verbatim**, plus a response envelope carrying B1 and the responder's route token.
Safe because `hs_transcript_mac` binds the A1 **bytes**. **Zero frame bytes changed**, verified
by diffing the handshake module for any touch of `hs_encode_*`/`hs_decode_*`/`hs_transcript_*`.

The response envelope is **load-bearing, not symmetry**: the initiator reached the responder
through a one-shot slot whose ticket its own push burned. The two-party test's decisive
assertion is that Bob's stored `route_token` for Alice is **her real inbox**, read from the
vault — no CLI surface prints route tokens.

**§2m — the auto-mint contact-add is retired.** It minted a token the peer had never seen, so
the contact was unreachable by construction — strictly worse than the out-of-band presumption it
resembled. The explicit `--route-token` form is **superseded-but-functional** as a named
residue: **73 test files and 111 call sites** depend on it, including `NA_0640_full_stack_e2e`
and `NA_0644_ack_client`, the instrument that proves this slice broke nothing.

**§2k — the clock is a parameter**, adopting the `_at` seam `vault/protection.rs` already
documents as "the test-visible clock seam". Reported from the tree first, as ordered: **no
shared injectable accessor exists**, but the seam does and is proven; the four duplicate private
now-helpers (`dedup:59`, `lib:1284`, `protection:112`, `attachments:474`) are a **named
follow-up micro-lane**, deliberately not re-plumbed here. **The privacy line struck `created_at`
from the canonical bundle** — those bytes are uploaded to the relay.

---

## Acceptance (D616 §5)

| requirement | evidence |
|---|---|
| substituted bundle → commitment | `a_substituted_bundle_fails_the_commitment`, and again over the real wire |
| tampered field → signature, **distinctly** | `a_tampered_code_field_fails_the_signature` (4 fields) + `the_two_security_failures_are_distinct_codes` + `commitment_is_checked_before_signature` |
| replay, **both arms independently** | relay arm `a_replayed_redemption_is_already_used_not_not_found` (with the never-existed negative half); client arm `a_second_redemption_is_refused_by_the_client_itself` |
| expired dies **pre-network** | `an_expired_invite_dies_before_any_network_attempt` — dead port, local code required, **plus the control** that a future expiry fails differently |
| interception race detected | ticketless push to a consumed slot → 403; loser gets `already_used` |
| **two-party handshake, real relay** | `two_strangers_become_a_session_through_one_invite` |
| **cross-impl codec gate** | `qsc_and_relay_codecs_agree_byte_for_byte` — 256 adversarial bytes **and** the real bundle, asserted on decoded bytes |
| **silent-200 guard, three parts** | `a_ticketless_push_is_refused_and_a_misrendered_token_is_not_success` + the slot proven READ by the contact existing on Alice's side |
| clamp tolerance | `an_over_long_expiry_is_clamped_not_rejected`, probe uninvolved |
| non-regression | full suite `EXIT=0`, 111/472/0/2 |

**Five negative controls run, five observed RED**, output recorded, sources restored
byte-identical. Details in the testplan §B.

## Gates

```
cargo test -p qsc --no-fail-fast (RUST_TEST_THREADS=2)  EXIT=0 · 111 targets · 472 passed · 0 failed · 2 ignored
cargo clippy -p qsc --all-targets -- -D warnings        26 sites at BASE, 26 at HEAD -> delta ZERO
rustfmt --check (this lane's four files)                0 diffs each
infra_literal_scan --mode staged                        clean (17 files, 3097 lines)
infra_literal_scan --mode tree                          clean (2294 files, 604530 lines)
git diff Cargo.toml Cargo.lock                          EMPTY
```

⚠ **Three `aws_file_*` suites EXCLUDED and named** (`…_na0192a`, `…_na0192b`, `…_na0186`) —
they hang at base on ENG-0079. Verified genuinely absent from the run (**0 occurrences** in the
log), not silently included. §5.9 holds **modulo those three**, per the operator ruling.

⚠ **Two of this directive's own gates were not gates**, both established by stashing the lane
and re-running at base: `fmt --all --check` is RED at base at **146 locations** (ENG-0050), and
`clippy -D warnings` is RED at base at **26 sites**. One new clippy finding *was* this lane's
(`large_enum_variant`, from the three additive `ServerInfoDoc` fields); resolving it surfaced a
second (`items after a test module`); both fixed, the enum `#[allow]`ed with the reasoning in
the code because boxing it would be a public API change with wider blast radius than the lint.

⚠ **A third specified gate was also dead: the two-PR split.** `goal-lint` refused a code-only
spine PR. D616 §3 copied Slice 1's arrangement without its property — Slice 1's code PR lived in
`qsl-server`, which has no `goal-lint`. D-1315 moved into the code PR; no bypass was used.

## Scope

Touched exactly D616 §7's MAY list: `qsc/src/invite/mod.rs` (new), `src/{lib,store/mod,
transport/mod,contacts/mod,handshake/mod,cmd/mod,main}.rs`, `qsc/tests/**`, and spine
governance. **No** `.github/**`, **no** `Cargo.*`, **no** pin movement, **no** qsl-server /
qsl-desktop / qsl-attachments file, **no** frame layout, **no** `formal/`, `specs/` or
conformance vector.

## Filed

**ENG-0079** — `qsc receive` has no overall timeout; measured at base.
**ENG-0080** — the Slice-4 landmine: a PENDING contact prints `state=PINNED` **and**
`device … state=TRUSTED`; the GUI must key on the contact state or it inverts I5.
**ENG-0081** — house defaults that differ from tool defaults (empty trailers, `pkill` self-match,
never `fmt --all`); operator-ruled to its own micro-lane because a `CLAUDE.md` edit fires both
full suites.
**`LANE_INTENT` §3b amended** with the operator's bidirectional rule.
