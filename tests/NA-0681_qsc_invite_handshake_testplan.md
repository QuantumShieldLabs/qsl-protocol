# NA-0681 TESTPLAN — MESSAGING EPIC SLICE 2: qsc invite create, redeem, and the two-party handshake

Lane NA-0681 · directive **QSL-DIR-2026-07-27-616 (D616, amended)**, sha256
`7c1dd5754393c129ced67df616692f5812f44160dabbbb1509232f3c4b8bccd4`, 798 lines ·
implementation evidence **D-1315** · closeout **D-1316** · result class
`QSC_INVITE_HANDSHAKE_PASS`.

Built against the merged Slice-1 relay at pin `131d63f4`.

---

## §A — AUTOMATED COVERAGE

### A.1 `tests/NA_0681_invite_encodings.rs` — 23 tests, the four canonical encodings and the verify order

The census found the cryptography settled and **every real risk to be an encoding risk**, each
with a failure mode that presents as an attack or as *success* rather than as an honest error.
This file covers the four definitions.

| property | tests |
|---|---|
| `wire_id` is 32 lowercase hex, round-trips, and satisfies the client's own route-token validator | 3 |
| `cap_hash_hex` is SHA-256 over the **wire string**, lowercase | 2 |
| canonical bundle: exact layout, round-trip, trailing bytes refused, empty keys refused, **carries no timestamp** | 3 |
| commitment: domain-separated known answer; changes on any key byte | 2 |
| invite code: round-trip, SMS-sized, unpadded URL-safe, parse causes distinct, `QSLI-2-` reads as newer, endpoint policy | 6 |
| verify order: honest verifies; substituted bundle → commitment; tampered field → signature (4 fields); the two causes are **distinct**; **commitment is checked FIRST** | 5 |
| `QSLH-1` envelope: round-trip carrying A1 verbatim; unknown tag refused; duplicate tag refused; newer version distinguished; unusable route token refused | 2 |

**The hash and commitment vectors are KNOWN ANSWERS computed in Python before the Rust
existed**, and the file pins the two **forbidden** values as well as the correct one — the
raw-bytes hash and the uppercase-hex hash, which are the two ways C3 breaks silently. A test
that recomputed the answer with the same code would pass while both were wrong.

### A.2 `tests/NA_0681_invite_relay_contract.rs` — 8 tests, against the REAL relay in process

`common::start_qsl_server` runs qsl-server's actual router. No mocks.

| requirement | test |
|---|---|
| **the cross-implementation codec gate** — 256 adversarial bytes **and** the real identity bundle survive `qsc encode → relay store → relay re-encode → qsc decode` byte-for-byte, asserted on **decoded bytes** | `qsc_and_relay_codecs_agree_byte_for_byte` |
| the padding asymmetry is a rule, not folklore: the relay **accepts** padded input and **emits** unpadded, so the strings differ while the bytes match | `relay_accepts_padded_upload_but_emits_unpadded` |
| **the silent-200 guard**: a ticketless push to a live slot is **403**, a mis-rendered token is **not** success, and the two must differ | `a_ticketless_push_is_refused_and_a_misrendered_token_is_not_success` |
| replay, relay arm: second redeem is `ALREADY_USED` **not** `NOT_FOUND`, with the never-existed case proved to answer differently | `a_replayed_redemption_is_already_used_not_not_found` |
| a **same-length** wrong capability is refused **with no mutation** (the D-0014 lesson) | `a_same_length_wrong_capability_is_refused_and_the_slot_survives` |
| substitution and tampering fail **distinctly**, on data that made the real round trip | `substitution_and_tampering_fail_distinctly_through_the_real_relay` |
| clamp tolerance: an over-long expiry is **clamped not rejected**; a past expiry still refused | `an_over_long_expiry_is_clamped_not_rejected` |
| `resolve_expiry` respects the ceiling and treats "not advertised" as **unknown**, never as zero | `expiry_resolution_respects_the_ceiling_and_treats_zero_as_unknown` |

### A.3 `tests/NA_0681_two_party_handshake.rs` — 3 tests, two vaults, one invite, the real relay

| requirement | test |
|---|---|
| **§5.6** — create → redeem → verify → handshake → both sides hold a PENDING contact carrying the other's identity and route token | `two_strangers_become_a_session_through_one_invite` |
| replay, **client** arm — the client's own record refuses a second redemption without consulting the relay (the arm that survives a hostile relay) | `a_second_redemption_is_refused_by_the_client_itself` |
| an expired invite dies **pre-network**, proved by pointing at a dead port, **with the control** that a future expiry fails differently | `an_expired_invite_dies_before_any_network_attempt` |

### A.4 Non-regression

Full suite `EXIT=0` — **111 targets, 472 passed, 0 failed, 2 ignored**, `RUST_TEST_THREADS=2`.
`clippy` delta vs base **ZERO**. `git diff Cargo.toml Cargo.lock` **EMPTY**. **Zero handshake
frame functions touched.**

---

## §B — NEGATIVE CONTROLS: FIVE RUN, FIVE OBSERVED RED

A test is not evidence until it has been seen to fail. Each control broke the property, the
suite was run, the red was **recorded**, and the source was restored **byte-identical**.

| control | red produced |
|---|---|
| `cap_hash` computed over the RAW bytes | `left: be45cb26…` — **exactly the forbidden constant the test pins**, caught by both the known-answer test and the must-not-equal test |
| signature checked **before** commitment | `left: "invite_signature_invalid"` / `right: "invite_commitment_mismatch"` — the user would be told "the invite was edited" when the truth is "the relay swapped the keys" |
| trailing bytes accepted in the payload | parse-taxonomy test red |
| `created_at` restored to the canonical bundle | length **20 vs 12**; known answer `…b2b2b20000000000000000` — the privacy line is enforced structurally, not by documentation |
| standard-alphabet base64 instead of URL-safe | **`ERR_INVITE_BAD_BODY` from the relay** — the cross-impl codec gate demonstrably sees a real disagreement |

**What would make each control vacuous** is stated beside it in the test file. The general
guard: every negative assertion in §A.1 sits next to a positive one exercising the same
function, so "it rejected everything" cannot masquerade as "it rejected the right thing".

---

## §C — WHAT THIS PLAN CANNOT SEE

*(The pattern NA-0680 established: a testplan that only asserts coverage teaches the next
reader to trust it. This section is the measurement instead.)*

**C.1 — Three of this lane's own assertions could never have passed, and the suite could not
have told me.** A check that the `invite_id` "must not appear anywhere" in the contact record
would have failed against **every correct implementation**, because §2f stores it deliberately
as provenance. A `"pinned"` comparison failed on a `PINNED` output. A reused identifier made
the relay's correct refusal look like a codec failure. All three surfaced only because the
code was finished enough to run. **§3b as written names only the can't-fail direction; a test
that cannot PASS is the same defect pointing the other way**, and the epic's §3b has been
amended accordingly (operator ruling, 2026-07-27). In practice: **run the happy path first and
see it green, before writing any negative assertion against it.**

**C.2 — Three gates this directive specified were dead at base, and the census never ran
them.** `cargo fmt --all -- --check` is **RED at base at 146 locations** (ENG-0050). The full
`cargo test -p qsc` **hangs at base** (ENG-0079). The two-PR split was **forbidden by
`goal-lint`**. The census produced eleven measured corrections about the *code* and zero
measurements of the *process that would judge it*. **An acceptance item never executed at base
is a hypothesis, not a gate.**

**C.3 — The §2m collateral was found one full-suite run at a time, because the sweep asked the
wrong question.** Every sweep was **file-level** ("which files never pass `--route-token`"),
which structurally cannot see a file that passes it on *some* call sites and not others. Two
files surfaced that way, ~90 minutes apart. The **call-site** sweep that closed the set should
have been the first query. **A sweep whose unit is coarser than the unit of change
under-reports silently.**

**C.4 — What a PASS does NOT assert.**
- Nothing about **messaging or delivery states** — that is Slice 3.
- Nothing about the **GUI** — Slice 4. ⚠ See ENG-0080: a PENDING contact prints
  `state=PINNED` **and** `device … state=TRUSTED`; Slice 4 must key its badge on the contact
  state or it inverts I5.
- **No timing or constant-time claim of any kind.** None was measured.
- **Two vaults on ONE host** is the tested topology (epic §4 Q2). Nothing here says anything
  about NAT, real partitions, or two physical devices.
- Nothing about the **three excluded `aws_file_*` suites**, which did not run.
- Nothing about **relay-operator behaviour** beyond what the client refuses on its own.
- The `pinned_cert_fp` hook is **captured and dormant**; no pinning is implemented or claimed.

**C.5 — The interruption-safety path is implemented but not crash-tested.** The redemption
response is persisted before and immediately after the network call, so a drop cannot strand
the redeemer on a consumed invite. That ordering is asserted by construction and by reading —
**there is no test that kills the process in the window.** The `revoke_token` loss window
(a crash between the relay's 200 and the persist) is likewise reasoned, recorded in the code as
a stated limitation, and unexercised.

---

## §D — HOW TO RE-RUN

```
source /srv/qbuild/work/NA-0681/.qwork/cargo-target.qsl-protocol.env
cd <seat>/qsl-protocol
RUST_TEST_THREADS=2 cargo test -p qsc --test NA_0681_invite_encodings
RUST_TEST_THREADS=2 cargo test -p qsc --test NA_0681_invite_relay_contract
RUST_TEST_THREADS=2 cargo test -p qsc --test NA_0681_two_party_handshake
```

⚠ The full suite must **exclude** `aws_file_confirmation_replay_na0192b`,
`aws_file_medium_boundary_na0192a` and `aws_file_robustness_na0186` until ENG-0079 is fixed;
they hang at base. ⚠ `cargo fmt --all` must **not** be run (ENG-0050). ⚠ `qsc-linux-full-suite`
skips on pull requests, so CI green never covers this suite — the local run is the evidence.
